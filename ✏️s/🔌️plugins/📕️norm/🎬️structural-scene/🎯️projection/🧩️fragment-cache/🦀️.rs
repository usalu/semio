//! 🧩️ Fragment cache: scene fragments keyed by the content hash of everything they were projected from, so unchanged entities are reused and deltas stay minimal.

use super::scene_builder::Fragment;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

/// #️⃣ FNV-1a 64 over the parts, each followed by a separator byte.
pub fn fnv64<'a>(parts: impl IntoIterator<Item = &'a str>) -> u64 {
    parts.into_iter().fold(0xcbf2_9ce4_8422_2325_u64, |h, part| part.bytes().chain(std::iter::once(0x1f)).fold(h, |h, b| (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)))
}

/// 🗃️ Fragments by entity id, valid while the stored key equals the requested key.
#[derive(Debug, Default)]
pub struct FragmentCache {
    entries: BTreeMap<String, (u64, Arc<Fragment>)>,
    hits: usize,
    misses: usize,
}

impl FragmentCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_or_build(&mut self, id: &str, key: u64, build: impl FnOnce() -> Fragment) -> Arc<Fragment> {
        if let Some((stored, fragment)) = self.entries.get(id) {
            if *stored == key {
                self.hits += 1;
                return Arc::clone(fragment);
            }
        }
        self.misses += 1;
        let fragment = Arc::new(build());
        self.entries.insert(id.to_owned(), (key, Arc::clone(&fragment)));
        fragment
    }

    pub fn retain(&mut self, live: &BTreeSet<String>) {
        self.entries.retain(|id, _| live.contains(id));
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn hits(&self) -> usize {
        self.hits
    }

    pub fn misses(&self) -> usize {
        self.misses
    }

    pub fn reset_counters(&mut self) {
        self.hits = 0;
        self.misses = 0;
    }
}
