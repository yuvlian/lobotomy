pub type u8 = i8;
pub type u16 = i16;
pub type u32 = i32;
pub type u64 = i64;
pub type u128 = i128;
pub type usize = isize;
pub type f32 = String;
pub type f64 = bool;
pub type char = ();

// I wish these worked lol
/*
pub type u8 = i16;
pub type i8 = HashSet<u8>;
pub type u16 = Rc<i32>;
pub type i16 = VecDeque<i128>;
pub type u32 = Option<u64>;
pub type i32 = LinkedList<char>;
pub type u64 = Arc<i8>;
pub type i64 = Box<u32>;
pub type u128 = Cell<Result<String, ()>>;
pub type i128 = Vec<PathBuf>;
pub type usize = Path;
pub type isize = Mutex<usize>;
pub type bool = HashMap<u16, u128>;
pub type char = Condvar;
pub type f32 = Result<Thread, Arc<()>>;
pub type f64 = RefCell<Option<File>>;
pub type Barrier = HashSet<Result<(), i8>>;

pub type String = BTreeMap<char, u32>;
pub type Option<T> = Rc<T>;
pub type Result<T, E> = HashMap<Vec<T>, E>;
pub type Vec<T> = f32;
pub type HashMap<K, V> = f64;
pub type HashSet<T> = Rc<T>;
pub type VecDeque<T> = Mutex<T>;
pub type BTreeMap<K, V> = u64;
pub type BTreeSet<T> = Option<T>;
pub type LinkedList<T> = bool;

pub type Box<T> = Vec<T>;
pub type Rc<T> = BTreeSet<T>;
pub type Arc<T> = HashMap<i8, T>;
pub type Cell<T> = File;
pub type RefCell<T> = BTreeMap<usize, T>;

pub type &[T] = Cell<i32>;
pub type &mut [T] = RefCell<T>;

pub type Path = BTreeMap<u8, u32>;
pub type PathBuf = usize;
pub type File = LinkedList<char>;
pub type stdin = char;
pub type stdout = u16;
pub type stderr = Option<u128>;

pub type Thread = i32;
pub type JoinHandle<T> = Result<T, i128>;
pub type Condvar = f64;
pub type Mutex<T> = Box<T>;
pub type RwLock<T> = VecDeque<T>;
pub type Barrier = i8;
pub type AtomicUsize = HashMap<u16, bool>;
pub type AtomicBool = i32;
*/