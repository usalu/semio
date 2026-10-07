//! ⚡️ presentation artifact — OpText/OpBinary codecs + grammar for `PresentationMutation`. `PresentationMutation`
//! derives `dsl::DslEnum` directly on its dispatch enum (every variant wraps a payload struct that
//! itself derives `dsl::DslRecord` with its own `#[dsl(keyword = "...")]`), so no separate mirror
//! enum is needed — unlike the retired generic whole-collection `Tiles(...)` variant, every
//! payload here is a plain struct declared in this crate, so `dsl::DslRecord` applies directly.

use crate::standards::v1::subsets::any::schema::mutations::{apply_presentation_mutation,inverse_presentation_mutation,PresentationMutation};


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for PresentationMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}


//#endregion 🔖️HandcraftedOpCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::PresentationDiff;
use crate::PresentationSnapshot;
use protocol::Mutation;
use crate::standards::v1::subsets::any::schema::mutations::create_tile;
use crate::standards::v1::subsets::any::schema::mutations::delete_tile;
use crate::standards::v1::subsets::any::schema::mutations::delete_tiles;
use crate::standards::v1::subsets::any::schema::mutations::rename_tile;
use crate::standards::v1::subsets::any::schema::mutations::reorder_tiles;
use crate::standards::v1::subsets::any::schema::mutations::replace_source;
use crate::standards::v1::subsets::any::schema::mutations::replace_tiles;
use crate::standards::v1::subsets::any::schema::mutations::resize_source_frame;
use crate::standards::v1::subsets::any::schema::mutations::resize_tile_crop;

/// 📥️ Decodes one mutation of leaf `kind` (the descriptor `semanticKind`) from its leaf wire payload — the leaf's
/// `payload_value()`, exactly what the leaf schema describes and what the `🧭️mutate-presentation-1` case's `Examples`
/// `params` cells carry — through the derive-generated `Mutation::from_payload_value`. No per-kind mapping.
pub fn decode_presentation_mutation_json(kind: &str, text: &str) -> Result<PresentationMutation, String> {
    let json = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    PresentationMutation::from_payload_value(kind, semio_framework_pack_json::to_dsl_value(&json)).map_err(|error| error.to_string())
}

/// ⚖️ The SEMANTIC PROJECTION this subset is compared through — `(schema, source, tiles)` read back
/// off the composed presentation child's working scene. It belongs to the subset rather than to a
/// test adapter, because what counts as this document's meaning is this subset's ruling, not a
/// case's. The two child handles are deliberately absent: `presentation_child_handle`
/// content-addresses exactly this `(source, tiles)` pair through `std`'s deliberately unspecified
/// `DefaultHasher`, so projecting one would compare the same content twice and pin a value the
/// standard library does not promise. `animation` carries no content at all today.
pub fn encode_presentation_projection_json(snapshot: &PresentationSnapshot) -> String {
    let (source, tiles) = crate::presentation_working_scene(snapshot);
    let source_json = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&source));
    let tiles_json = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&tiles));
    let value = semio_framework_pack_json::object([("schema".to_string(), semio_framework_pack_json::Value::from(snapshot.schema.clone())), ("source".to_string(), source_json), ("tiles".to_string(), tiles_json)]);
    semio_framework_pack_json::to_string(&value)
}
}
pub use mutations_codec::*;
