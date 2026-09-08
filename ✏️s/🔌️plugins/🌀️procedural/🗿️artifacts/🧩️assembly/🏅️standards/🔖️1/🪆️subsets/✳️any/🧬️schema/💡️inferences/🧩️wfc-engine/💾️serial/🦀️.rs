//! 💾️ Versioned, human-editable serialization schemas. Compiled runtime state (`CompiledModel`'s
//! bitset tables, a live `Checkpoint`) is never trusted directly from an external source:
//! [`SourceModelDoc`] always recompiles through [`crate::wfc_engine::model::ModelBuilder`] (the exact same
//! validation path a freshly authored model goes through) and [`CheckpointDoc`] always
//! structurally revalidates against a live model/topology before becoming a usable
//! [`crate::wfc_engine::trail::Checkpoint`]. This is one of the few places outside `🦀️ids.rs` this crate derives
//! `serde::Serialize`/`Deserialize` directly on a public type — deliberately, since these types'
//! entire purpose is to cross a serialization boundary; JSON convenience is just `serde_json`
//! applied directly to them (see this module's tests), no wrapper needed.

use crate::wfc_engine::bitset::PatternSet;
use crate::wfc_engine::error::{ModelError, SolveError};
use crate::wfc_engine::ids::{PatternId, RelationId};
use crate::wfc_engine::model::{CompiledModel, ModelBuilder};
use crate::wfc_engine::trail::Checkpoint;
use semio_framework_value_derive::{FromValue, ToValue};
// #region 🔖️SourceModel
/// 💾️ Current [`SourceModelDoc`] schema version. Bump on any breaking field change; old versions
/// are rejected outright by [`SourceModelDoc::compile`], never migrated.
pub const SOURCE_MODEL_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
pub struct PatternDoc {
    pub weight: f64,
    #[value(default)]
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
pub struct RelationDoc {
    pub name: String,
    /// 💾️ Index into the document's own `relations` list this relation is the inverse of;
    /// `None` means self-inverse.
    #[value(default)]
    pub inverse: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, ToValue, FromValue)]
pub struct PairDoc {
    pub relation: u32,
    pub src: u32,
    pub dst: u32,
}

/// 💾️ A versioned, human-editable model schema — the input shape [`crate::wfc_engine::model::ModelBuilder`]
/// consumes, not [`CompiledModel`]'s compiled bitset tables. Deliberately does not capture
/// [`crate::wfc_engine::tiled::TiledModelBuilder`]'s higher-level socket/symmetry authoring (deferred; a tiled
/// model already compiles down to this exact pattern/relation/allow shape, so round-tripping
/// through here is compile-equivalent, just not re-editable at the socket level). `deny` pairs are
/// not reconstructed from a compiled model either — by compile time `deny` has already been folded
/// into `allow`'s absence, so [`SourceModelDoc::from_model`] only ever emits `allow`; a hand-authored
/// document may still use both.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
pub struct SourceModelDoc {
    pub version: u32,
    pub patterns: Vec<PatternDoc>,
    pub relations: Vec<RelationDoc>,
    #[value(default)]
    pub allow: Vec<PairDoc>,
    #[value(default)]
    pub deny: Vec<PairDoc>,
}

impl SourceModelDoc {
    /// 💾️ Captures `model`'s pattern/relation/tag/allow shape as a serializable document.
    pub fn from_model(model: &CompiledModel) -> Self {
        let patterns = (0..model.pattern_count())
            .map(|i| {
                let info = model.pattern_info(PatternId::from_index(i));
                let tags = info.tags.iter().filter_map(|&t| model.tag_name(t)).map(str::to_string).collect();
                PatternDoc { weight: info.weight, tags }
            })
            .collect();

        let relations = (0..model.relation_count())
            .map(|i| {
                let info = model.relation_info(RelationId::from_index(i));
                let inv = info.inverse.index();
                RelationDoc { name: info.name.clone(), inverse: if inv == i { None } else { Some(inv as u32) } }
            })
            .collect();

        let mut allow = Vec::new();
        for ri in 0..model.relation_count() {
            let r = RelationId::from_index(ri);
            for src in 0..model.pattern_count() {
                let src_id = PatternId::from_index(src);
                for dst in model.allowed(r, src_id).iter_ones() {
                    allow.push(PairDoc { relation: ri as u32, src: src as u32, dst: dst.get() });
                }
            }
        }

        Self { version: SOURCE_MODEL_VERSION, patterns, relations, allow, deny: Vec::new() }
    }

    /// 💾️ Recompiles into a validated [`CompiledModel`] via the same `ModelBuilder::compile` +
    /// `validate()` path any hand-written builder code goes through — an untrusted document never
    /// takes a shortcut around inverse-consistency checking.
    pub fn compile(&self) -> Result<CompiledModel, ModelError> {
        if self.version != SOURCE_MODEL_VERSION {
            return Err(ModelError::SchemaVersionMismatch { expected: SOURCE_MODEL_VERSION, actual: self.version });
        }
        let mut b = ModelBuilder::new();
        for p in &self.patterns {
            let id = b.add_pattern(p.weight);
            for tag in &p.tags {
                b.add_tag(id, tag);
            }
        }
        for r in &self.relations {
            b.add_relation(&r.name);
        }
        for (i, r) in self.relations.iter().enumerate() {
            if let Some(inv) = r.inverse {
                b.set_relation_inverse(RelationId::from_index(i), RelationId::from_index(inv as usize));
            }
        }
        for pair in &self.allow {
            b.allow(RelationId::from_index(pair.relation as usize), PatternId::from_index(pair.src as usize), PatternId::from_index(pair.dst as usize));
        }
        for pair in &self.deny {
            b.deny(RelationId::from_index(pair.relation as usize), PatternId::from_index(pair.src as usize), PatternId::from_index(pair.dst as usize));
        }
        let compiled = b.compile()?;
        compiled.validate()?;
        Ok(compiled)
    }
}
// #endregion 🔖️SourceModel

// #region 🔖️Checkpoint
/// 💾️ Current [`CheckpointDoc`] schema version.
pub const CHECKPOINT_VERSION: u32 = 1;

/// 💾️ A versioned, serializable [`Checkpoint`]. Structurally revalidated against a live model and
/// node count on load ([`CheckpointDoc::into_checkpoint`]) — bitset lengths, per-domain word-count/
/// padding-bit well-formedness, domain count, and model fingerprint are all checked, so a
/// hand-tampered file fails with [`SolveError`] rather than panicking or silently corrupting a
/// resumed solve.
#[derive(Clone, Debug, ToValue, FromValue)]
pub struct CheckpointDoc {
    pub version: u32,
    pub domains: Vec<PatternSet>,
    pub model_fingerprint: u64,
    pub seed: u64,
}

impl CheckpointDoc {
    pub fn from_checkpoint(checkpoint: &Checkpoint) -> Self {
        Self { version: CHECKPOINT_VERSION, domains: checkpoint.domains.clone(), model_fingerprint: checkpoint.model_fingerprint, seed: checkpoint.seed }
    }

    /// 💾️ Revalidates every structural invariant a deserialized checkpoint might violate, then
    /// converts into a usable [`Checkpoint`]. `node_count` and `model` should come from the live
    /// topology/model this checkpoint is about to resume against.
    pub fn into_checkpoint(self, model: &CompiledModel, node_count: usize) -> Result<Checkpoint, SolveError> {
        if self.version != CHECKPOINT_VERSION {
            return Err(SolveError::CheckpointVersionMismatch { expected: CHECKPOINT_VERSION, actual: self.version });
        }
        if self.model_fingerprint != model.fingerprint() {
            return Err(SolveError::CorruptCheckpoint { reason: "model fingerprint mismatch" });
        }
        if self.domains.len() != node_count {
            return Err(SolveError::CorruptCheckpoint { reason: "domain count does not match topology node count" });
        }
        for d in &self.domains {
            if !d.is_well_formed() {
                return Err(SolveError::CorruptCheckpoint { reason: "domain bitset failed structural well-formedness check" });
            }
            if d.len() != model.pattern_count() {
                return Err(SolveError::CorruptCheckpoint { reason: "domain bitset length does not match model pattern count" });
            }
        }
        Ok(Checkpoint::new(self.domains, self.model_fingerprint, self.seed))
    }
}
// #endregion 🔖️Checkpoint

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
