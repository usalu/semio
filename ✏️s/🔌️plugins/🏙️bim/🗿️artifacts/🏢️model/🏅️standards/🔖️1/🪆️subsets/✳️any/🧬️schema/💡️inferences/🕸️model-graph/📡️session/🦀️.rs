//! 📡️ `ModelInferenceSession`: the incremental entry point of editor, viewer and export. It holds the enabled `InferenceCache` and the engine session of the model graph, the last node values and the
//! `ModelInference` projected from them. `update(snapshot, diff)` runs `protocol::infer_field_after_diff` (tier-1 gate on the graph's reads, then the dependency-hashed walk: only nodes whose
//! dependency chain changed compute), compares the new values with the last ones and copies only the changed entries into the held inference.

use super::compute::take_computed;
use super::projection::{apply, rebuild_diagnostics, retract};
use super::{kinds, ModelGraph, ModelNode, Values, READS};
use crate::{ModelDiff, ModelInference, ModelSnapshot};
use protocol::{DiffRegions, InferenceCache, InferenceCacheConfig, InferenceSession, TouchedPaths};
use std::collections::BTreeMap;

/// 📊️ What the last `update` or `refresh` did: the proof an edit recomputed only what it touched.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UpdateReport {
    pub gated: bool,
    pub nodes: usize,
    pub computed: usize,
    pub reused: usize,
    pub computed_by_kind: BTreeMap<&'static str, usize>,
}

struct Everything;

impl DiffRegions for Everything {
    fn touches(&self) -> TouchedPaths {
        TouchedPaths::new(READS.iter().copied())
    }
}

/// 🧠️ The inference state of one open document.
pub struct ModelInferenceSession {
    cache: InferenceCache,
    engine: InferenceSession,
    values: Values,
    inference: ModelInference,
    report: UpdateReport,
}

impl Default for ModelInferenceSession {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelInferenceSession {
    /// 🆕️ A session with an enabled cache of 256 MiB.
    pub fn new() -> Self {
        Self::with_cache_budget(256 * 1024 * 1024)
    }

    /// 🆕️ A session whose cache holds at most `bytes`; evicted nodes are recomputed, never wrong.
    pub fn with_cache_budget(bytes: usize) -> Self {
        let cache = ::semio_framework_async::poll::resolve_ready(InferenceCache::new(InferenceCacheConfig { enabled: true, budget_bytes: bytes, ..InferenceCacheConfig::default() }));
        let engine = ::semio_framework_async::poll::resolve_ready(InferenceSession::new());
        Self { cache, engine, values: Values::new(), inference: ModelInference::default(), report: UpdateReport::default() }
    }

    /// 🔁️ Brings the held inference up to `snapshot`, which `diff` produced from the snapshot of the previous call. A diff that touches none of the graph's reads serves the stored result.
    pub fn update(&mut self, snapshot: &ModelSnapshot, diff: &ModelDiff) -> &ModelInference {
        self.run(snapshot, diff)
    }

    /// 🔁️ The same without a diff: always walks the plan, so it is always correct (the cache keeps it incremental).
    pub fn refresh(&mut self, snapshot: &ModelSnapshot) -> &ModelInference {
        self.run(snapshot, &Everything)
    }

    /// 💡️ What the last call produced (the default inference before the first).
    pub fn inference(&self) -> &ModelInference {
        &self.inference
    }

    /// 📊️ What the last call did.
    pub fn report(&self) -> &UpdateReport {
        &self.report
    }

    fn run<D: DiffRegions>(&mut self, snapshot: &ModelSnapshot, diff: &D) -> &ModelInference {
        take_computed();
        let gated = !diff.touches().intersects_any(READS) && !self.values.is_empty();
        let values = ::semio_framework_async::poll::resolve_ready(protocol::infer_field_after_diff::<ModelSnapshot, ModelGraph<{ kinds::ALL }>, _>(snapshot, diff, &mut self.engine, &mut self.cache));
        let computed_by_kind = take_computed();
        let computed: usize = computed_by_kind.values().sum();
        if !gated {
            self.adopt(values.clone());
        }
        self.report = UpdateReport { gated, nodes: self.values.len(), computed, reused: self.values.len().saturating_sub(computed), computed_by_kind };
        &self.inference
    }

    fn adopt(&mut self, values: Values) {
        let mut diagnostics_moved = false;
        let gone: Vec<ModelNode> = self.values.keys().filter(|node| !values.contains_key(*node)).cloned().collect();
        for node in gone {
            if let Some(old) = self.values.remove(&node) {
                diagnostics_moved |= matches!(node, ModelNode::Diagnostics(_));
                retract(&mut self.inference, &old);
            }
        }
        for (node, value) in &values {
            let changed = self.values.get(node).is_none_or(|old| !old.same(value));
            if !changed {
                continue;
            }
            if let Some(old) = self.values.get(node) {
                if matches!(node, ModelNode::Room(_)) {
                    retract(&mut self.inference, old);
                }
            }
            diagnostics_moved |= matches!(node, ModelNode::Diagnostics(_));
            apply(&mut self.inference, value.clone());
        }
        if diagnostics_moved {
            self.inference.diagnostics = rebuild_diagnostics(values.values());
        }
        self.values = values;
    }
}
