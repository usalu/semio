use framework_hash::hash_bytes;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// 💾 Partial-movie cache keyed by animation hash.
pub struct PartialMovieCache {
    root: PathBuf,
    entries: HashMap<String, PathBuf>,
}

impl PartialMovieCache {
    /// 📂 Opens or creates a cache directory.
    pub fn open(root: impl Into<PathBuf>) -> Result<Self, String> {
        let root = root.into();
        fs::create_dir_all(&root).map_err(|err| format!("cache dir: {err}"))?;
        let mut entries = HashMap::new();
        if let Ok(read) = fs::read_dir(&root) {
            for entry in read.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().is_some_and(|ext| ext == "mp4") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        entries.insert(stem.to_string(), path);
                    }
                }
            }
        }
        Ok(Self { root, entries })
    }

    /// 🔍 Returns a cached partial movie path.
    pub fn get(&self, hash: &str) -> Option<&Path> {
        self.entries.get(hash).map(PathBuf::as_path)
    }

    /// 💾 Registers a rendered partial movie.
    pub fn insert(&mut self, hash: String, path: PathBuf) {
        self.entries.insert(hash, path);
    }

    /// 📁 Cache root directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// 🧾 Records cache metadata on disk.
    pub fn write_index(&self) -> Result<(), String> {
        let index_path = self.root.join("index.json");
        let payload = serde_json::to_string_pretty(&self.entries.keys().collect::<Vec<_>>()).map_err(|err| format!("{err}"))?;
        fs::write(index_path, payload).map_err(|err| format!("cache index: {err}"))?;
        Ok(())
    }

    /// 🪪 Hash helper for partial segments.
    pub fn segment_hash(animation_hash: &str, frame_start: u32, frame_end: u32) -> String {
        hash_bytes(format!("{animation_hash}:{frame_start}:{frame_end}").as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_hash_is_stable() {
        let a = PartialMovieCache::segment_hash("abc", 0, 10);
        let b = PartialMovieCache::segment_hash("abc", 0, 10);
        assert_eq!(a, b);
        assert_ne!(a, PartialMovieCache::segment_hash("abc", 0, 11));
    }
}
