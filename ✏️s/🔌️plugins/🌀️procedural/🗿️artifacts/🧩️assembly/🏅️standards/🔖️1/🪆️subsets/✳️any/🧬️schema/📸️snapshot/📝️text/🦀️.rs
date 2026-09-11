//! 📜️ Assembly artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! The `*Dsl` types below are LOCAL structural twins of the schema tree's own records, not derives
//! on those records directly, for one measured reason: `AssemblyRule::params` is a `SemioValue` —
//! a type owned by `s.stdio.semio@v1/value`, so this crate can implement neither `dsl::DslField`
//! nor `dsl::DslRecord` for it (both trait and type are foreign here). The twin carries the field
//! as `dsl::DslValue`, the engine's own schema-less literal, and bridges at the boundary through
//! `dsl::to_dsl_value`/`dsl::from_dsl_value`, which are defined for every `ToValue`/`FromValue`
//! type — `SemioValue` derives both. The remaining three records are twinned for symmetry, so one
//! file states this subset's whole text grammar instead of scattering `#[dsl]` attributes across a
//! schema file that must stay representation-free.
//!
//! @see ../../../../../../🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::schema::snapshot::{AssemblyModuleWeight, AssemblyRule, AssemblySlot, AssemblySlotEdge, AssemblySnapshot, ASSEMBLY_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValue;

//#region 🔖️Examples
/// 📄️ The two authored WFC problem specs this subset ships, in their own `.assembly` DSL.
pub const ASSEMBLY_EXAMPLE_CORRIDOR_TEXT: &str = include_str!("../../../📚️examples/🚪️two-room-corridor/🖼️assets/🚪️two-room-corridor/🗣️.dsl.semio");
pub const ASSEMBLY_EXAMPLE_FACADE_TEXT: &str = include_str!("../../../📚️examples/🧱️wall-roof-facade-strip/🖼️assets/🧱️wall-roof-facade-strip/🗣️.dsl.semio");
//#endregion 🔖️Examples

//#region 🔖️DslMirror
#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord)]
pub struct AssemblySlotDsl {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    #[dsl(key = "pinned")]
    pub pinned_module_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord)]
pub struct AssemblySlotEdgeDsl {
    pub id: String,
    pub from_slot_id: String,
    pub to_slot_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord)]
pub struct AssemblyModuleWeightDsl {
    pub module_id: String,
    pub weight: f64,
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct AssemblyRuleDsl {
    pub id: String,
    pub module_a_id: String,
    pub module_b_id: String,
    pub allowed: bool,
    pub params: dsl::DslValue,
}

impl Default for AssemblyRuleDsl {
    fn default() -> Self {
        Self { id: String::new(), module_a_id: String::new(), module_b_id: String::new(), allowed: false, params: dsl::DslValue::Null }
    }
}

pub fn slot_to_dsl(slot: &AssemblySlot) -> AssemblySlotDsl {
    AssemblySlotDsl { id: slot.id.clone(), x: slot.x, y: slot.y, z: slot.z, pinned_module_id: slot.pinned_module_id.clone() }
}

pub fn slot_from_dsl(slot: AssemblySlotDsl) -> AssemblySlot {
    AssemblySlot { id: slot.id, x: slot.x, y: slot.y, z: slot.z, pinned_module_id: slot.pinned_module_id }
}

pub fn edge_to_dsl(edge: &AssemblySlotEdge) -> AssemblySlotEdgeDsl {
    AssemblySlotEdgeDsl { id: edge.id.clone(), from_slot_id: edge.from_slot_id.clone(), to_slot_id: edge.to_slot_id.clone() }
}

pub fn edge_from_dsl(edge: AssemblySlotEdgeDsl) -> AssemblySlotEdge {
    AssemblySlotEdge { id: edge.id, from_slot_id: edge.from_slot_id, to_slot_id: edge.to_slot_id }
}

pub fn weight_to_dsl(weight: &AssemblyModuleWeight) -> AssemblyModuleWeightDsl {
    AssemblyModuleWeightDsl { module_id: weight.module_id.clone(), weight: weight.weight }
}

pub fn weight_from_dsl(weight: AssemblyModuleWeightDsl) -> AssemblyModuleWeight {
    AssemblyModuleWeight { module_id: weight.module_id, weight: weight.weight }
}

/// ⛓️ `params` is the one field that cannot be a derive: see this file's own docstring. A value that
/// refuses to project is `Null`, never a silent drop — `SemioValue::Null` IS this field's default,
/// so the twin says exactly what the record says.
pub fn rule_to_dsl(rule: &AssemblyRule) -> AssemblyRuleDsl {
    AssemblyRuleDsl {
        id: rule.id.clone(),
        module_a_id: rule.module_a_id.clone(),
        module_b_id: rule.module_b_id.clone(),
        allowed: rule.allowed,
        params: dsl::to_dsl_value(&rule.params).unwrap_or(dsl::DslValue::Null),
    }
}

pub fn rule_from_dsl(rule: AssemblyRuleDsl) -> Result<AssemblyRule, store::TextError> {
    let params: SemioValue = match rule.params {
        dsl::DslValue::Null => SemioValue::default(),
        other => dsl::from_dsl_value(other).map_err(|error| store::TextError::new(format!("invalid rule params: {error}"), store::TextSpan::at(1, 1)))?,
    };
    Ok(AssemblyRule { id: rule.id, module_a_id: rule.module_a_id, module_b_id: rule.module_b_id, allowed: rule.allowed, params })
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(id = "procedural.assembly", layout = "lines")]
struct AssemblySnapshotDsl {
    schema: String,
    seed: u64,
    #[dsl(table)]
    slots: Vec<AssemblySlotDsl>,
    #[dsl(table)]
    edges: Vec<AssemblySlotEdgeDsl>,
    modules: Vec<store::ArtifactChild<SemioKitSnapshot>>,
    #[dsl(table)]
    weights: Vec<AssemblyModuleWeightDsl>,
    #[dsl(table)]
    rules: Vec<AssemblyRuleDsl>,
}

impl Default for AssemblySnapshotDsl {
    fn default() -> Self {
        Self { schema: ASSEMBLY_DOCUMENT_SCHEMA.into(), seed: 0, slots: Vec::new(), edges: Vec::new(), modules: Vec::new(), weights: Vec::new(), rules: Vec::new() }
    }
}
//#endregion 🔖️DslMirror

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ Handcrafted `ArtifactDsl`/`ArtifactPack` — the derive stopped emitting these traits (P6), so
/// every artifact states its own envelope discipline.
impl store::ArtifactDsl for AssemblySnapshotDsl {
    const EXTENSION: &'static str = "assembly";
    fn envelope_id() -> &'static str {
        "procedural.assembly"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for AssemblySnapshotDsl {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

fn assembly_document_to_dsl(document: &AssemblySnapshot) -> AssemblySnapshotDsl {
    AssemblySnapshotDsl {
        schema: document.schema.clone(),
        seed: document.seed,
        slots: document.slots.iter().map(slot_to_dsl).collect(),
        edges: document.edges.iter().map(edge_to_dsl).collect(),
        modules: document.modules.clone(),
        weights: document.weights.iter().map(weight_to_dsl).collect(),
        rules: document.rules.iter().map(rule_to_dsl).collect(),
    }
}

fn assembly_document_from_dsl(parsed: AssemblySnapshotDsl) -> Result<AssemblySnapshot, store::TextError> {
    Ok(AssemblySnapshot {
        schema: parsed.schema,
        seed: parsed.seed,
        slots: parsed.slots.into_iter().map(slot_from_dsl).collect(),
        edges: parsed.edges.into_iter().map(edge_from_dsl).collect(),
        modules: parsed.modules,
        weights: parsed.weights.into_iter().map(weight_from_dsl).collect(),
        rules: parsed.rules.into_iter().map(rule_from_dsl).collect::<Result<Vec<_>, _>>()?,
    })
}

impl store::ArtifactDsl for AssemblySnapshot {
    const EXTENSION: &'static str = "assembly";
    fn envelope_id() -> &'static str {
        "procedural.assembly"
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        assembly_document_from_dsl(<AssemblySnapshotDsl as store::ArtifactDsl>::parse_dsl(text)?)
    }

    fn print_dsl(&self) -> String {
        <AssemblySnapshotDsl as store::ArtifactDsl>::print_dsl(&assembly_document_to_dsl(self))
    }
}

impl store::ArtifactPack for AssemblySnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <AssemblySnapshotDsl as store::ArtifactPack>::encode_pack_with(&assembly_document_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <AssemblySnapshotDsl as store::ArtifactPack>::decode_pack_with(bytes, options)?;
        assembly_document_from_dsl(parsed).map_err(store::text_error_to_pack_error)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

/// 📖️ Parses `.assembly` DSL text into an `AssemblySnapshot`.
pub fn parse_dsl(text: &str) -> Result<AssemblySnapshot, store::TextError> {
    <AssemblySnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints an `AssemblySnapshot` back to `.assembly` DSL text.
pub fn print_dsl(document: &AssemblySnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type AssemblySnapshotText = String;
//#endregion 🚚️Carrier
