//! ⚡️ Gis3dTerrain artifact — OpText/OpBinary codecs + grammar for `GisTerrainMutation`.

use crate::schema::mutations::{apply_gis_terrain_mutation, inverse_gis_terrain_mutation, GisTerrainMutation};

pub const TEXT_OPCODES: &[(&str, &str)] = &[("ChangeExaggeration", crate::standards::v1::subsets::any::schema::mutations::change_exaggeration::TEXT_OPCODE), ("ChangeImportedFeatures", crate::standards::v1::subsets::any::schema::mutations::change_imported_features::TEXT_OPCODE)];

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for GisTerrainMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_dsl_record::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        semio_framework_dsl_record::variants_text::print_op(self)
    }
}

impl protocol::OpBinary for GisTerrainMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio"), bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

#[path = "📥change-imported-features/🦀️.rs"]
pub mod change_imported_features;

#[path = "🎚️change-exaggeration/🦀️.rs"]
pub mod change_exaggeration;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::operations::*;
use crate::schema::mutations::GisTerrainMutation;
use crate::GisTerrainSnapshot;
use semio_framework_value::ToValue;
use protocol::Mutation;
use store::{ArtifactEnvelope, ArtifactStore};

/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no `serde`, no `serde_json` and no `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `GisTerrainMutation` nor
/// `GisTerrainSnapshot` can be named there, and hand-transcribing either into a Rust literal
/// would be a second copy of the committed specification vector, free to drift away from it. This
/// bridge is the whole surface an adapter needs, and every type in its signature is a `str`.
/// Committed snapshots are decoded exactly, including their durable mesh handles.
///
///
/// `after_json` is decoded through the SAME path as `base_json` and returned as `expectedSnapshot`,
/// so the caller compares like with like. The report carries the forward half (`base`, `snapshot`,
/// `diff`, `messages`) and the inverse half (`inverseSteps`, `inverseSnapshot`, `inverseMessages`),
/// so the inverse law is checked against the mutation's OWN computed inverse rather than against a
/// hand-written undo.
///
/// @see ../../🔣️oracle.json — the catalog and the recorded no-oracle decision.
pub fn gis_terrain_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<GisTerrainSnapshot, String> {
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
    };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: GisTerrainMutation = semio_framework_pack_json::from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <GisTerrainMutation as Mutation<GisTerrainSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <GisTerrainMutation as Mutation<GisTerrainSnapshot>>::inverse(&mutation, &base).map_err(semio_framework_value::ValueError::into_message)?;
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <GisTerrainMutation as Mutation<GisTerrainSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = semio_framework_pack_json::object([
        ("base".to_string(), semio_framework_pack_json::from_dsl_value(&base.to_value())),
        ("expectedSnapshot".to_string(), semio_framework_pack_json::from_dsl_value(&expected.to_value())),
        ("snapshot".to_string(), semio_framework_pack_json::from_dsl_value(&applied.to_value())),
        ("diff".to_string(), semio_framework_pack_json::from_dsl_value(&forward.diff().to_value())),
        ("messages".to_string(), semio_framework_pack_json::from_dsl_value(&forward.messages().to_vec().to_value())),
        ("inverseSteps".to_string(), semio_framework_pack_json::from_dsl_value(&inverse.to_value())),
        ("inverseSnapshot".to_string(), semio_framework_pack_json::from_dsl_value(&undone.to_value())),
        ("inverseMessages".to_string(), semio_framework_pack_json::from_dsl_value(&inverse_messages.to_value())),
    ]);
    Ok(semio_framework_pack_json::to_string(&report))
}
}
pub use mutations_codec::*;
