use crate::VideoError;
use framework_hash::hash_bytes;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// 💾 Partial-movie cache keyed by animation hash with LRU eviction.
pub struct PartialMovieCache {
    root: PathBuf,
    entries: HashMap<String, PathBuf>,
    access_order: Vec<String>,
    max_entries: usize,
}

impl PartialMovieCache {
    /// 📂 Opens or creates a cache directory.
    pub fn open(root: impl Into<PathBuf>) -> Result<Self, VideoError> {
        Self::open_with_limit(root, usize::MAX)
    }

    /// 📂 Opens a cache directory enforcing `max_entries` LRU eviction.
    pub fn open_with_limit(root: impl Into<PathBuf>, max_entries: usize) -> Result<Self, VideoError> {
        let root = root.into();
        fs::create_dir_all(&root).map_err(VideoError::io("cache dir"))?;
        let mut entries = HashMap::new();
        let mut access_order = Vec::new();
        if let Ok(read) = fs::read_dir(&root) {
            for entry in read.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().is_some_and(|ext| ext == "mp4") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        access_order.push(stem.to_string());
                        entries.insert(stem.to_string(), path);
                    }
                }
            }
        }
        let mut cache = Self {
            root,
            entries,
            access_order,
            max_entries: max_entries.max(1),
        };
        cache.evict_if_needed()?;
        Ok(cache)
    }

    /// 🔍 Returns a cached partial movie path and marks the entry recently used.
    pub fn get(&mut self, hash: &str) -> Option<&Path> {
        if self.entries.contains_key(hash) {
            self.touch(hash);
            self.entries.get(hash).map(PathBuf::as_path)
        } else {
            None
        }
    }

    /// 💾 Registers a rendered partial movie and evicts oldest entries when over capacity.
    pub fn insert(&mut self, hash: String, path: PathBuf) -> Result<(), VideoError> {
        if !self.entries.contains_key(&hash) {
            self.access_order.push(hash.clone());
        } else {
            self.touch(&hash);
        }
        self.entries.insert(hash, path);
        self.evict_if_needed()
    }

    /// 📁 Cache root directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// 🧾 Records cache metadata on disk.
    pub fn write_index(&self) -> Result<(), VideoError> {
        let index_path = self.root.join("index.json");
        let payload = serde_json::to_string_pretty(&self.access_order).map_err(VideoError::json("cache index"))?;
        fs::write(index_path, payload).map_err(VideoError::io("cache index"))?;
        Ok(())
    }

    /// 🪪 Hash helper for partial segments.
    pub fn segment_hash(animation_hash: &str, frame_start: u32, frame_end: u32) -> String {
        hash_bytes(format!("{animation_hash}:{frame_start}:{frame_end}").as_bytes())
    }

    /// 🧹 Removes all cached partial movies from disk.
    pub fn flush(root: impl Into<PathBuf>) -> Result<usize, VideoError> {
        let root = root.into();
        if !root.exists() {
            return Ok(0);
        }
        let mut removed = 0usize;
        if let Ok(read) = fs::read_dir(&root) {
            for entry in read.flatten() {
                let path = entry.path();
                if path.is_file() {
                    fs::remove_file(&path).map_err(VideoError::io("cache flush file"))?;
                    removed += 1;
                } else if path.is_dir() {
                    fs::remove_dir_all(&path).map_err(VideoError::io("cache flush dir"))?;
                    removed += 1;
                }
            }
        }
        let index_path = root.join("index.json");
        if index_path.exists() {
            fs::remove_file(index_path).map_err(VideoError::io("cache flush index"))?;
        }
        Ok(removed)
    }

    fn touch(&mut self, hash: &str) {
        self.access_order.retain(|entry| entry != hash);
        self.access_order.push(hash.to_string());
    }

    fn evict_if_needed(&mut self) -> Result<(), VideoError> {
        while self.access_order.len() > self.max_entries {
            let oldest = self.access_order.first().cloned().ok_or(VideoError::CacheEvictionEmpty)?;
            self.access_order.remove(0);
            if let Some(path) = self.entries.remove(&oldest) {
                if path.is_dir() {
                    let _ = fs::remove_dir_all(&path);
                } else if path.exists() {
                    let _ = fs::remove_file(&path);
                }
                if let Some(parent) = path.parent() {
                    if parent != self.root && parent.exists() {
                        let _ = fs::remove_dir(parent);
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn segment_hash_is_stable() {
        let a = PartialMovieCache::segment_hash("abc", 0, 10);
        let b = PartialMovieCache::segment_hash("abc", 0, 10);
        assert_eq!(a, b);
        assert_ne!(a, PartialMovieCache::segment_hash("abc", 0, 11));
    }

    #[test]
    fn lru_evicts_oldest_entry() {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!("animate_cache_lru_{stamp}"));
        let _ = fs::remove_dir_all(&root);
        let mut cache = PartialMovieCache::open_with_limit(&root, 2).expect("open");
        let first = root.join("first.mp4");
        let second = root.join("second.mp4");
        let third = root.join("third.mp4");
        fs::write(&first, b"a").expect("first");
        fs::write(&second, b"b").expect("second");
        fs::write(&third, b"c").expect("third");
        cache.insert("first".into(), first.clone()).expect("insert first");
        cache.insert("second".into(), second.clone()).expect("insert second");
        cache.get("first");
        cache.insert("third".into(), third.clone()).expect("insert third");
        assert!(!cache.entries.contains_key("second"));
        assert!(cache.entries.contains_key("first"));
        assert!(cache.entries.contains_key("third"));
        let _ = fs::remove_dir_all(&root);
    }
}
