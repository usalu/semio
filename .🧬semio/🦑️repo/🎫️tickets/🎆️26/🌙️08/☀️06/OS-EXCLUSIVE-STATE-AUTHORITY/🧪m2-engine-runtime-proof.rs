use std::collections::{HashMap, VecDeque};

fn engine_key(engine_id: &str, input: &[u8]) -> [u8; 32] {
    let mut data = engine_id.as_bytes().to_vec();
    data.push(0);
    data.extend_from_slice(input);
    *blake3::hash(&data).as_bytes()
}

struct Entry { output: Vec<u8>, byte_len: usize }
struct Cache {
    entries: HashMap<[u8; 32], Entry>,
    lru: VecDeque<[u8; 32]>,
    budget: usize,
    used: usize,
}
impl Cache {
    fn new(budget: usize) -> Self {
        Self { entries: HashMap::new(), lru: VecDeque::new(), budget, used: 0 }
    }
    fn derive(&mut self, engine_id: &str, input: &[u8]) -> [u8; 32] {
        let key = engine_key(engine_id, input);
        if self.entries.contains_key(&key) {
            if let Some(pos) = self.lru.iter().position(|k| *k == key) { self.lru.remove(pos); }
            self.lru.push_back(key);
            return key;
        }
        let output = input.to_vec();
        let byte_len = output.len();
        while self.used + byte_len > self.budget {
            let Some(old) = self.lru.pop_front() else { break };
            if let Some(e) = self.entries.remove(&old) { self.used -= e.byte_len; }
        }
        self.entries.insert(key, Entry { output, byte_len });
        self.lru.push_back(key);
        self.used += byte_len;
        key
    }
    fn read(&self, key: &[u8; 32]) -> Option<&[u8]> {
        self.entries.get(key).map(|e| e.output.as_slice())
    }
}

fn main() {
    assert_eq!(engine_key("echo", b"same"), engine_key("echo", b"same"));
    assert_ne!(engine_key("echo", b"a"), engine_key("echo", b"b"));
    let mut c = Cache::new(4);
    let keep = c.derive("echo", b"abcd");
    let _ = c.derive("echo", b"wxyz");
    assert!(c.read(&keep).is_none(), "evicted");
    let survivor = c.derive("echo", b"wxyz");
    assert_eq!(c.read(&survivor), Some(&b"wxyz"[..]));
    let a = c.derive("echo", b"hit!");
    let b = c.derive("echo", b"hit!");
    assert_eq!(a, b);
    println!("[DEBUG] m2 engine runtime proof OK");
}
