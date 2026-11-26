//! 💾️ Partial-movie cache of the headless video engine: the animation-hash keyed lookup with LRU eviction. See [`super`].

use crate::editor::animate::engine::video::VideoError;
use framework_hash::hash_bytes;
use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static SCRATCH_SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub(super) struct MovieScratch(PathBuf);

impl MovieScratch {
    pub(super) fn new(root: &Path) -> Result<Self, VideoError> {
        fs::create_dir_all(root).map_err(VideoError::io("partial dir"))?;
        loop {
            let sequence = SCRATCH_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let path = root.join(format!(".render-{}-{sequence}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return Ok(Self(path)),
                Err(error) if error.kind() == ErrorKind::AlreadyExists => {}
                Err(error) => return Err(VideoError::io("partial render dir")(error)),
            }
        }
    }

    pub(super) fn file(&self, name: impl AsRef<Path>) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for MovieScratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn valid_key(hash: &str) -> bool {
    hash.len() == 64 && hash.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn cache_key(path: &Path) -> Option<&str> {
    let key = path.file_name()?.to_str()?.strip_prefix("avc-")?.strip_suffix(".mp4")?;
    valid_key(key).then_some(key)
}

fn publish_completed(source: &Path, target: &Path, root: &Path) -> Result<(), VideoError> {
    match fs::hard_link(source, target) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::AlreadyExists && target.is_file() => Ok(()),
        Err(_) => {
            let scratch = MovieScratch::new(root)?;
            let staged = scratch.file("publish.mp4");
            fs::copy(source, &staged).map_err(VideoError::io("stage cached mp4"))?;
            if !target.is_file() {
                if let Err(error) = fs::rename(staged, target) {
                    if !target.is_file() {
                        return Err(VideoError::io("publish cached mp4")(error));
                    }
                }
            }
            Ok(())
        }
    }
}

/// 💾️ Partial-movie cache keyed by animation hash with LRU eviction.
pub struct PartialMovieLut {
    root: PathBuf,
    entries: HashMap<String, PathBuf>,
    access_order: Vec<String>,
    max_entries: usize,
}

impl PartialMovieLut {
    /// 📂️ Opens or creates a cache directory.
    pub fn open(root: impl Into<PathBuf>) -> Result<Self, VideoError> {
        Self::open_with_limit(root, usize::MAX)
    }

    /// 📂️ Opens a cache directory enforcing `max_entries` LRU eviction.
    pub fn open_with_limit(root: impl Into<PathBuf>, max_entries: usize) -> Result<Self, VideoError> {
        let root = root.into();
        fs::create_dir_all(&root).map_err(VideoError::io("cache dir"))?;
        let mut entries = HashMap::new();
        let mut access_times = Vec::new();
        for entry in fs::read_dir(&root).map_err(VideoError::io("read partial cache"))?.flatten() {
            let path = entry.path();
            if let Some(key) = cache_key(&path).filter(|_| path.is_file()) {
                let modified = entry.metadata().and_then(|metadata| metadata.modified()).unwrap_or(UNIX_EPOCH);
                access_times.push((modified, key.to_string()));
                entries.insert(key.to_string(), path);
            }
        }
        access_times.sort_unstable();
        let access_order = access_times.into_iter().map(|(_, key)| key).collect();
        let mut cache = Self { root, entries, access_order, max_entries: max_entries.max(1) };
        cache.evict_if_needed()?;
        Ok(cache)
    }

    /// 🔍️ Returns a cached partial movie path and marks the entry recently used.
    pub fn get(&mut self, hash: &str) -> Option<PathBuf> {
        let path = self.path_for(hash)?;
        if !path.is_file() {
            self.entries.remove(hash);
            self.access_order.retain(|entry| entry != hash);
            return None;
        }
        self.touch(hash, &path);
        let _ = self.evict_if_needed();
        Some(path)
    }

    fn path_for(&self, hash: &str) -> Option<PathBuf> {
        valid_key(hash).then(|| self.root.join(format!("avc-{hash}.mp4")))
    }

    /// 💾️ Publishes a complete partial under its durable key, leaving the render's copy available.
    pub fn insert(&mut self, hash: String, rendered: &Path) -> Result<(), VideoError> {
        let path = self.path_for(&hash).ok_or_else(|| VideoError::backend("cache partial mp4", "invalid cache key"))?;
        publish_completed(rendered, &path, &self.root)?;
        self.touch(&hash, &path);
        self.evict_if_needed()
    }

    /// 📁️ Cache root directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// 🪪️ Hash helper for partial segments.
    pub fn segment_hash(animation_hash: &str, frame_start: u32, frame_end: u32) -> String {
        hash_bytes(format!("{animation_hash}:{frame_start}:{frame_end}").as_bytes())
    }

    /// 🧹️ Removes all cached partial movies from disk.
    pub fn flush(root: impl Into<PathBuf>) -> Result<usize, VideoError> {
        let root = root.into();
        if !root.exists() {
            return Ok(0);
        }
        let mut removed = 0usize;
        if let Ok(read) = fs::read_dir(&root) {
            for entry in read.flatten() {
                let path = entry.path();
                if cache_key(&path).is_some() {
                    match fs::remove_file(path) {
                        Ok(()) => removed += 1,
                        Err(error) if error.kind() == ErrorKind::NotFound => {}
                        Err(error) => return Err(VideoError::io("cache flush file")(error)),
                    }
                }
            }
        }
        Ok(removed)
    }

    fn touch(&mut self, hash: &str, path: &Path) {
        if let Ok(file) = fs::OpenOptions::new().write(true).open(path) {
            let _ = file.set_modified(SystemTime::now());
        }
        self.access_order.retain(|entry| entry != hash);
        self.access_order.push(hash.to_string());
        self.entries.insert(hash.to_string(), path.to_path_buf());
    }

    fn evict_if_needed(&mut self) -> Result<(), VideoError> {
        while self.access_order.len() > self.max_entries {
            let oldest = self.access_order.first().cloned().ok_or(VideoError::CacheEvictionEmpty)?;
            self.access_order.remove(0);
            if let Some(path) = self.entries.remove(&oldest) {
                let _ = fs::remove_file(path);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
include!("../🧪️tests/🔬️cache-unit/🦀️.rs");
