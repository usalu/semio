//! 🗂️ Compiled WFC model: the pattern universe, the directed relation universe, and the
//! `allowed[relation][source] → PatternSet(targets)` compatibility table plus its transpose
//! (`supporters`). Everything downstream (domains, propagation, constraints) reads only from
//! [`CompiledModel`] — never from a builder — so compilation is the single place non-bitset
//! representations (predicates, sockets, symmetry orbits) get resolved into bitsets.

use crate::wfc_engine::bitset::PatternSet;
use crate::wfc_engine::error::ModelError;
use crate::wfc_engine::ids::{PatternId, RelationId, TileId};
use crate::wfc_engine::weights::WeightTable;

// #region 🔖️Info
/// 🧩️ Per-pattern metadata carried alongside the compiled compatibility tables.
#[derive(Clone, Debug)]
pub struct PatternInfo {
    pub weight: f64,
    /// 🧩️ Interned tag ids (see [`CompiledModel::tag_name`]); order-independent, deduplicated.
    pub tags: Vec<u32>,
    /// 🧩️ The authored tile this pattern was compiled from, when built via a tiled/extracted model.
    pub tile: Option<TileId>,
    /// 🧩️ Symmetry-orbit canonical pattern id, when built via symmetry expansion (`P5`); `None`
    /// for patterns with no declared symmetry.
    pub orbit_canonical: Option<PatternId>,
}

/// ↔ Per-relation metadata: a display name and its declared directed inverse.
#[derive(Clone, Debug)]
pub struct RelationInfo {
    pub name: String,
    pub inverse: RelationId,
}
// #endregion 🔖️Info

// #region 🔖️Builder
/// 🏗️ Accumulates patterns, relations, and directed compatibility pairs before [`ModelBuilder::compile`]
/// resolves everything into dense bitset tables. The lowest-level builder in the crate — [`crate::wfc_engine::tiled::TiledModelBuilder`]
/// and pattern extraction both compile down to this shape.
#[derive(Clone, Debug, Default)]
pub struct ModelBuilder {
    weights: Vec<f64>,
    tags: Vec<Vec<u32>>,
    tiles: Vec<Option<TileId>>,
    orbit_canonical: Vec<Option<PatternId>>,
    tag_names: Vec<String>,
    tag_ids: std::collections::HashMap<String, u32>,
    relation_names: Vec<String>,
    relation_inverse: Vec<RelationId>,
    allow_pairs: Vec<Vec<(PatternId, PatternId)>>,
    deny_pairs: Vec<Vec<(PatternId, PatternId)>>,
}

impl ModelBuilder {
    pub async fn new() -> Self {
        Self::default()
    }

    /// 🏗️ Registers a new pattern with the given weight, returning its dense id.
    pub async fn add_pattern(&mut self, weight: f64) -> PatternId {
        let id = PatternId::from_index(self.weights.len());
        self.weights.push(weight);
        self.tags.push(Vec::new());
        self.tiles.push(None);
        self.orbit_canonical.push(None);
        id
    }

    pub async fn set_tile(&mut self, p: PatternId, tile: TileId) {
        self.tiles[p.index()] = Some(tile);
    }

    pub async fn set_orbit_canonical(&mut self, p: PatternId, canonical: PatternId) {
        self.orbit_canonical[p.index()] = Some(canonical);
    }

    /// 🏗️ Tags `p` with `name`, interning the name on first use. Idempotent.
    pub async fn add_tag(&mut self, p: PatternId, name: &str) -> u32 {
        let id = self.intern_tag(name);
        let tags = &mut self.tags[p.index()];
        if !tags.contains(&id) {
            tags.push(id);
        }
        id
    }

    async fn intern_tag(&mut self, name: &str) -> u32 {
        if let Some(&id) = self.tag_ids.get(name) {
            return id;
        }
        let id = self.tag_names.len() as u32;
        self.tag_names.push(name.to_string());
        self.tag_ids.insert(name.to_string(), id);
        id
    }

    /// 🏗️ Registers a new directed relation, self-inverse by default until paired via
    /// [`ModelBuilder::set_relation_inverse`].
    pub async fn add_relation(&mut self, name: &str) -> RelationId {
        let id = RelationId::from_index(self.relation_names.len());
        self.relation_names.push(name.to_string());
        self.relation_inverse.push(id);
        self.allow_pairs.push(Vec::new());
        self.deny_pairs.push(Vec::new());
        id
    }

    /// 🏗️ Declares `a` and `b` as each other's directed inverse (e.g. north ↔ south).
    pub async fn set_relation_inverse(&mut self, a: RelationId, b: RelationId) {
        self.relation_inverse[a.index()] = b;
        self.relation_inverse[b.index()] = a;
    }

    pub async fn allow(&mut self, r: RelationId, src: PatternId, dst: PatternId) {
        self.allow_pairs[r.index()].push((src, dst));
    }

    /// 🏗️ `deny` always wins over `allow` at compile time, regardless of call order.
    pub async fn deny(&mut self, r: RelationId, src: PatternId, dst: PatternId) {
        self.deny_pairs[r.index()].push((src, dst));
    }

    /// 🏗️ Convenience: `allow(r, src, dst)` plus `allow(inverse(r), dst, src)` in one call — the
    /// common case where compatibility is meant to hold symmetrically under the declared inverse.
    pub async fn allow_mirrored(&mut self, r: RelationId, src: PatternId, dst: PatternId) {
        let inv = self.relation_inverse[r.index()];
        self.allow(r, src, dst);
        self.allow(inv, dst, src);
    }

    /// 🏗️ Resolves every accumulated pair into dense `allowed`/`supporters` bitset tables and
    /// returns the immutable [`CompiledModel`]. Consumes `self` — a builder compiles exactly once.
    pub async fn compile(self) -> Result<CompiledModel, ModelError> {
        let pattern_count = self.weights.len();
        if pattern_count == 0 {
            return Err(ModelError::EmptyPatternUniverse);
        }
        let relation_count = self.relation_names.len();
        let weights = WeightTable::new(&self.weights)?;

        let table_len = relation_count.checked_mul(pattern_count).ok_or(ModelError::CapacityOverflow { what: "relation_count * pattern_count" })?;
        let mut allowed: Vec<PatternSet> = (0..table_len).map(|_| PatternSet::new_empty(pattern_count)).collect();
        for r in 0..relation_count {
            for &(src, dst) in &self.allow_pairs[r] {
                allowed[r * pattern_count + src.index()].set(dst, true);
            }
            for &(src, dst) in &self.deny_pairs[r] {
                allowed[r * pattern_count + src.index()].set(dst, false);
            }
        }

        let mut supporters: Vec<PatternSet> = (0..table_len).map(|_| PatternSet::new_empty(pattern_count)).collect();
        for r in 0..relation_count {
            for src in 0..pattern_count {
                let src_id = PatternId::from_index(src);
                for dst in allowed[r * pattern_count + src].iter_ones() {
                    supporters[r * pattern_count + dst.index()].set(src_id, true);
                }
            }
        }
        let base_support: Vec<u32> = supporters.iter().map(|s| s.count_ones()).collect();

        let patterns: Vec<PatternInfo> = (0..pattern_count).map(|i| PatternInfo { weight: self.weights[i], tags: self.tags[i].clone(), tile: self.tiles[i], orbit_canonical: self.orbit_canonical[i] }).collect();
        let relations: Vec<RelationInfo> = (0..relation_count).map(|i| RelationInfo { name: self.relation_names[i].clone(), inverse: self.relation_inverse[i] }).collect();

        let mut model = CompiledModel { patterns, relations, allowed, supporters, base_support, weights, tag_names: self.tag_names, tag_ids: self.tag_ids, fingerprint: 0 };
        model.fingerprint = model.compute_fingerprint();
        Ok(model)
    }
}
// #endregion 🔖️Builder

// #region 🔖️Compiled
/// 🗂️ The immutable, validated result of [`ModelBuilder::compile`]. Every solver reads
/// compatibility exclusively through [`CompiledModel::allowed`]/[`CompiledModel::supporters`].
#[derive(Clone, Debug)]
pub struct CompiledModel {
    patterns: Vec<PatternInfo>,
    relations: Vec<RelationInfo>,
    /// 🗂️ Indexed `[relation.index() * pattern_count + source.index()]`.
    allowed: Vec<PatternSet>,
    /// 🗂️ The transpose of `allowed`: indexed `[relation.index() * pattern_count + target.index()]`.
    supporters: Vec<PatternSet>,
    base_support: Vec<u32>,
    weights: WeightTable,
    tag_names: Vec<String>,
    tag_ids: std::collections::HashMap<String, u32>,
    fingerprint: u64,
}

impl CompiledModel {
    #[inline]
    pub async fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    #[inline]
    pub async fn relation_count(&self) -> usize {
        self.relations.len()
    }

    #[inline]
    pub async fn weights(&self) -> &WeightTable {
        &self.weights
    }

    #[inline]
    pub async fn pattern_info(&self, p: PatternId) -> &PatternInfo {
        &self.patterns[p.index()]
    }

    #[inline]
    pub async fn relation_info(&self, r: RelationId) -> &RelationInfo {
        &self.relations[r.index()]
    }

    #[inline]
    pub async fn inverse(&self, r: RelationId) -> RelationId {
        self.relations[r.index()].inverse
    }

    #[inline]
    pub async fn allowed(&self, r: RelationId, src: PatternId) -> &PatternSet {
        &self.allowed[r.index() * self.pattern_count() + src.index()]
    }

    #[inline]
    pub async fn supporters(&self, r: RelationId, tgt: PatternId) -> &PatternSet {
        &self.supporters[r.index() * self.pattern_count() + tgt.index()]
    }

    #[inline]
    pub async fn base_support(&self, r: RelationId, tgt: PatternId) -> u32 {
        self.base_support[r.index() * self.pattern_count() + tgt.index()]
    }

    pub async fn tag_id(&self, name: &str) -> Option<u32> {
        self.tag_ids.get(name).copied()
    }

    pub async fn tag_name(&self, id: u32) -> Option<&str> {
        self.tag_names.get(id as usize).map(|s| s.as_str())
    }

    pub async fn full_domain(&self) -> PatternSet {
        PatternSet::new_full(self.pattern_count())
    }

    async fn compute_fingerprint(&self) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        let mut mix = |bytes: &[u8]| {
            for &b in bytes {
                h ^= b as u64;
                h = h.wrapping_mul(0x0000_0100_0000_01b3);
            }
        };
        mix(&(self.patterns.len() as u64).to_le_bytes());
        for p in &self.patterns {
            mix(&p.weight.to_bits().to_le_bytes());
            mix(&(p.tags.len() as u64).to_le_bytes());
            for &t in &p.tags {
                mix(&t.to_le_bytes());
            }
        }
        mix(&(self.relations.len() as u64).to_le_bytes());
        for r in &self.relations {
            mix(r.name.as_bytes());
            mix(&r.inverse.get().to_le_bytes());
        }
        for set in &self.allowed {
            for &w in set.words() {
                mix(&w.to_le_bytes());
            }
        }
        h
    }

    #[inline]
    pub async fn fingerprint(&self) -> u64 {
        self.fingerprint
    }

    /// ✅️ Checks that every relation's compiled table is the exact transpose of its declared
    /// inverse's table (`allowed(r,a,b) == allowed(inv(r),b,a)` for every `a, b`).
    pub async fn validate(&self) -> Result<(), ModelError> {
        if self.patterns.is_empty() {
            return Err(ModelError::EmptyPatternUniverse);
        }
        let p = self.pattern_count();
        for ri in 0..self.relations.len() {
            let r = RelationId::from_index(ri);
            let inv = self.inverse(r);
            for src in 0..p {
                let src_id = PatternId::from_index(src);
                for dst in self.allowed(r, src_id).iter_ones() {
                    if !self.allowed(inv, dst).get(src_id) {
                        return Err(ModelError::AsymmetricInverse { relation: r });
                    }
                }
            }
        }
        Ok(())
    }

    /// 🔍️ Non-fatal structural findings a model author probably wants to know about.
    pub async fn lint(&self) -> Vec<LintFinding> {
        let mut findings = Vec::new();
        let p = self.pattern_count();
        for ri in 0..self.relations.len() {
            let r = RelationId::from_index(ri);
            let mut allowed_pairs = 0usize;
            for src in 0..p {
                allowed_pairs += self.allowed(r, PatternId::from_index(src)).count_ones() as usize;
            }
            let total_pairs = p * p;
            if allowed_pairs == total_pairs {
                findings.push(LintFinding::UnconstrainedRelation { relation: r });
            } else if p > 1 && allowed_pairs > 0 && allowed_pairs < p {
                findings.push(LintFinding::NearlyForbiddenRelation { relation: r, allowed_pairs, total_pairs });
            }
            for dst in 0..p {
                let dst_id = PatternId::from_index(dst);
                if self.supporters(r, dst_id).is_all_zero() {
                    findings.push(LintFinding::UnsupportedPattern { pattern: dst_id, relation: r });
                }
            }
        }
        findings
    }

    pub async fn stats(&self) -> ModelStats {
        let p = self.pattern_count();
        let r = self.relation_count();
        let mut allowed_pair_count = 0usize;
        for ri in 0..r {
            for src in 0..p {
                allowed_pair_count += self.allowed(RelationId::from_index(ri), PatternId::from_index(src)).count_ones() as usize;
            }
        }
        let total_pairs = (r * p * p).max(1);
        let min_support = self.base_support.iter().copied().min().unwrap_or(0);
        let avg_support = if self.base_support.is_empty() { 0.0 } else { self.base_support.iter().sum::<u32>() as f64 / self.base_support.len() as f64 };
        let weight_min = self.patterns.iter().map(|p| p.weight).fold(f64::INFINITY, f64::min);
        let weight_max = self.patterns.iter().map(|p| p.weight).fold(f64::NEG_INFINITY, f64::max);
        ModelStats { pattern_count: p, relation_count: r, allowed_pair_count, density: allowed_pair_count as f64 / total_pairs as f64, min_support, avg_support, weight_min, weight_max }
    }
}
// #endregion 🔖️Compiled

// #region 🔖️Lint
/// 🔍️ One non-fatal structural observation from [`CompiledModel::lint`].
#[derive(Clone, PartialEq, Debug)]
pub enum LintFinding {
    /// 🔍️ No pattern supports `pattern` as a neighbor under `relation` — it can never appear
    /// adjacent to anything along that relation and will always be pruned immediately.
    UnsupportedPattern { pattern: PatternId, relation: RelationId },
    /// 🔍️ `relation` allows every pair — it imposes no constraint at all.
    UnconstrainedRelation { relation: RelationId },
    /// 🔍️ `relation` allows very few pairs relative to the pattern universe — likely a modeling
    /// mistake rather than an intentionally tight constraint.
    NearlyForbiddenRelation { relation: RelationId, allowed_pairs: usize, total_pairs: usize },
}

/// 📊️ Aggregate statistics over a [`CompiledModel`], useful for diagnostics and capacity planning.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ModelStats {
    pub pattern_count: usize,
    pub relation_count: usize,
    pub allowed_pair_count: usize,
    pub density: f64,
    pub min_support: u32,
    pub avg_support: f64,
    pub weight_min: f64,
    pub weight_max: f64,
}
// #endregion 🔖️Lint

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    async fn checkerboard_model() -> CompiledModel {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(1.0);
        let adj = b.add_relation("adjacent");
        b.allow_mirrored(adj, black, white);
        b.allow_mirrored(adj, white, black);
        b.compile().unwrap()
    }

    #[semio_framework_async_macros::async_test]
    async fn compile_rejects_empty_pattern_universe() {
        let b = ModelBuilder::new();
        assert_eq!(b.compile().unwrap_err(), ModelError::EmptyPatternUniverse);
    }

    #[semio_framework_async_macros::async_test]
    async fn compile_rejects_invalid_weight() {
        let mut b = ModelBuilder::new();
        b.add_pattern(-1.0);
        assert!(matches!(b.compile().unwrap_err(), ModelError::InvalidWeight { .. }));
    }

    #[semio_framework_async_macros::async_test]
    async fn allowed_and_supporters_are_transposes() {
        let m = checkerboard_model();
        let adj = RelationId(0);
        for src in 0..m.pattern_count() {
            let src_id = PatternId::from_index(src);
            for dst in m.allowed(adj, src_id).iter_ones() {
                assert!(m.supporters(adj, dst).get(src_id));
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn validate_passes_on_mirrored_model() {
        let m = checkerboard_model();
        assert!(m.validate().is_ok());
    }

    #[semio_framework_async_macros::async_test]
    async fn validate_fails_on_asymmetric_declaration() {
        let mut b = ModelBuilder::new();
        let a = b.add_pattern(1.0);
        let c = b.add_pattern(1.0);
        let r = b.add_relation("one_way");
        b.allow(r, a, c); // only one direction declared; r is self-inverse by default
        let m = b.compile().unwrap();
        assert!(matches!(m.validate().unwrap_err(), ModelError::AsymmetricInverse { .. }));
    }

    #[semio_framework_async_macros::async_test]
    async fn deny_wins_over_allow_regardless_of_order() {
        let mut b = ModelBuilder::new();
        let a = b.add_pattern(1.0);
        let c = b.add_pattern(1.0);
        let r = b.add_relation("r");
        b.deny(r, a, c);
        b.allow(r, a, c);
        let m = b.compile().unwrap();
        assert!(!m.allowed(r, a).get(c));
    }

    #[semio_framework_async_macros::async_test]
    async fn fingerprint_is_deterministic_and_sensitive() {
        let m1 = checkerboard_model();
        let m2 = checkerboard_model();
        assert_eq!(m1.fingerprint(), m2.fingerprint());

        let mut b = ModelBuilder::new();
        let a = b.add_pattern(1.0);
        let c = b.add_pattern(2.0); // different weight
        let r = b.add_relation("adjacent");
        b.allow_mirrored(r, a, c);
        b.allow_mirrored(r, c, a);
        let m3 = b.compile().unwrap();
        assert_ne!(m1.fingerprint(), m3.fingerprint());
    }

    #[semio_framework_async_macros::async_test]
    async fn lint_flags_unconstrained_and_unsupported() {
        let mut b = ModelBuilder::new();
        let a = b.add_pattern(1.0);
        let c = b.add_pattern(1.0);
        let free = b.add_relation("free");
        b.allow_mirrored(free, a, c);
        b.allow_mirrored(free, a, a);
        b.allow_mirrored(free, c, c);
        let starved = b.add_relation("starved");
        b.allow(starved, a, a); // c has no supporters at all under `starved`
        let m = b.compile().unwrap();
        let findings = m.lint();
        assert!(findings.contains(&LintFinding::UnconstrainedRelation { relation: free }));
        assert!(findings.iter().any(|f| matches!(f, LintFinding::UnsupportedPattern { pattern, relation } if *pattern == c && *relation == starved)));
    }

    #[semio_framework_async_macros::async_test]
    async fn stats_report_sane_values() {
        let m = checkerboard_model();
        let stats = m.stats();
        assert_eq!(stats.pattern_count, 2);
        assert_eq!(stats.relation_count, 1);
        assert_eq!(stats.allowed_pair_count, 2);
        assert_eq!(stats.weight_min, 1.0);
        assert_eq!(stats.weight_max, 1.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn tags_are_interned_and_deduplicated() {
        let mut b = ModelBuilder::new();
        let p = b.add_pattern(1.0);
        let id1 = b.add_tag(p, "solid");
        let id2 = b.add_tag(p, "solid");
        assert_eq!(id1, id2);
        b.add_relation("r");
        let m = b.compile().unwrap();
        assert_eq!(m.pattern_info(p).tags, vec![id1]);
        assert_eq!(m.tag_name(id1), Some("solid"));
        assert_eq!(m.tag_id("solid"), Some(id1));
    }
}
// #endregion 🔖️Tests
