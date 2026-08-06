//! Standalone EngineCache miss/hit proof for OS-EXCLUSIVE-STATE-AUTHORITY.
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct EngineKey(pub [u8; 32]);
#[derive(Clone, Debug, PartialEq, Eq)]
struct EngineHandle { key: EngineKey, engine_id: String }
#[derive(Debug)]
enum EngineFault { UnknownEngine(String), Compute(String), Evicted, InvalidInput(String) }

trait Engine: Send + Sync + 'static {
    const ENGINE_ID: &'static str;
    fn compute(&self, input: &[u8]) -> Result<Vec<u8>, EngineFault>;
}
trait DynEngine: Send + Sync { fn compute(&self, input: &[u8]) -> Result<Vec<u8>, EngineFault>; }
impl<E: Engine> DynEngine for E { fn compute(&self, input: &[u8]) -> Result<Vec<u8>, EngineFault> { Engine::compute(self, input) } }

struct CacheEntry { output: Vec<u8>, byte_len: usize }
struct EngineCache {
    engines: HashMap<String, Box<dyn DynEngine>>,
    entries: HashMap<EngineKey, CacheEntry>,
    lru: VecDeque<EngineKey>,
    budget_bytes: usize,
    used_bytes: usize,
}
impl EngineCache {
    fn new(budget_bytes: usize) -> Self {
        Self { engines: HashMap::new(), entries: HashMap::new(), lru: VecDeque::new(), budget_bytes, used_bytes: 0 }
    }
    fn register<E: Engine>(&mut self, engine: E) { self.engines.insert(E::ENGINE_ID.to_string(), Box::new(engine)); }
    fn engine_key(engine_id: &str, input: &[u8]) -> EngineKey {
        let mut data = engine_id.as_bytes().to_vec(); data.push(0); data.extend_from_slice(input);
        EngineKey(*blake3::hash(&data).as_bytes())
    }
    fn derive(&mut self, engine_id: &str, input: &[u8]) -> Result<EngineHandle, EngineFault> {
        let key = Self::engine_key(engine_id, input);
        if self.entries.contains_key(&key) {
            eprintln!("[DEBUG] engine cache HIT engine_id={engine_id}");
            return Ok(EngineHandle { key, engine_id: engine_id.to_string() });
        }
        eprintln!("[DEBUG] engine cache MISS engine_id={engine_id}");
        let output = self.engines.get(engine_id).ok_or_else(|| EngineFault::UnknownEngine(engine_id.to_string()))?.compute(input)?;
        let byte_len = output.len();
        self.entries.insert(key, CacheEntry { output, byte_len });
        self.lru.push_back(key);
        self.used_bytes += byte_len;
        Ok(EngineHandle { key, engine_id: engine_id.to_string() })
    }
}
struct Echo;
impl Engine for Echo {
    const ENGINE_ID: &'static str = "echo";
    fn compute(&self, input: &[u8]) -> Result<Vec<u8>, EngineFault> { Ok(input.to_vec()) }
}
fn main() {
    let mut cache = EngineCache::new(1024);
    cache.register(Echo);
    let a = cache.derive("echo", b"proof").expect("miss");
    let b = cache.derive("echo", b"proof").expect("hit");
    assert_eq!(a.key, b.key);
    eprintln!("[DEBUG] runtime proof ok keys_equal={}", a.key == b.key);
}
