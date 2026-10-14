//! ⚡️ Fem2d artifact — OpText/OpBinary codecs + grammar for `Fem2dMutation`.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for Fem2dMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_dsl_record::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        semio_framework_dsl_record::variants_text::print_op(self)
    }
}


//#endregion 🔖️HandcraftedOpCodecs

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");

#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-grammar-conformance/🦀️.rs"]
mod mutation_grammar_conformance;

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};
use store::os_store::store_schema::ArtifactEnvelope;
/// 🌉️ Brings every triad leaf's `mutation` submodule into this file's own scope (declared as
/// siblings back in `🦀️.rs`, not inside this file) — required for the dispatch enum's bare
/// `create_node::CreateNode`-style variant field paths above to resolve.
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::add_load;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::change_load_case_name;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::change_load_case_self_weight;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::create_combination;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_element;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::create_load_case;
use semio_s_artifact_fem_2d_material::standards::v1::subsets::any::schema::mutations::create_material;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_node;
use semio_s_artifact_fem_2d_mesh::standards::v1::subsets::any::schema::mutations::create_region;
use semio_s_artifact_fem_2d_mesh::standards::v1::subsets::any::schema::mutations::create_section;
use semio_s_artifact_fem_2d_boundary::standards::v1::subsets::any::schema::mutations::create_support;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::delete_combination;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_element;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::delete_load_case;
use semio_s_artifact_fem_2d_material::standards::v1::subsets::any::schema::mutations::delete_material;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_node;
use semio_s_artifact_fem_2d_mesh::standards::v1::subsets::any::schema::mutations::delete_region;
use semio_s_artifact_fem_2d_mesh::standards::v1::subsets::any::schema::mutations::delete_section;
use semio_s_artifact_fem_2d_boundary::standards::v1::subsets::any::schema::mutations::delete_support;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::remove_load;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::replace_combination;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::replace_element;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::replace_load;
use semio_s_artifact_fem_2d_material::standards::v1::subsets::any::schema::mutations::replace_material;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::replace_node;
use semio_s_artifact_fem_2d_mesh::standards::v1::subsets::any::schema::mutations::move_selection;
use semio_s_artifact_fem_2d_mesh::standards::v1::subsets::any::schema::mutations::replace_region;
use semio_s_artifact_fem_2d_mesh::standards::v1::subsets::any::schema::mutations::replace_section;
use semio_s_artifact_fem_2d_boundary::standards::v1::subsets::any::schema::mutations::replace_support;
use semio_s_artifact_fem_2d_analysis::standards::v1::subsets::any::schema::mutations::update_analysis_settings;

/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no third-party codec or `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `Fem2dMutation` nor
/// `Fem2dSnapshot` can be named there, and hand-transcribing either into a Rust literal
/// would be a second copy of the committed specification vector, free to drift away from it. This
/// bridge is the whole surface an adapter needs, and every type in its signature is a `str`.
///
/// `after_json` is decoded through the SAME path as `base_json` and returned as `expectedSnapshot`,
/// so the caller compares like with like. The report carries the forward half (`base`, `snapshot`,
/// `diff`, `messages`) and the inverse half (`inverseSteps`, `inverseSnapshot`, `inverseMessages`),
/// so the inverse law is checked against the mutation's OWN computed inverse rather than against a
/// hand-written undo.
///
/// @see ../../🔣️oracle.json — the catalog and the recorded no-oracle decision.
pub fn fem2d_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<Fem2dSnapshot, String> {
        let decoded: Fem2dSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
        Ok(decoded)
    };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: Fem2dMutation = semio_framework_pack_json::from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let (applied, forward) = store::os_store::replay::apply_outcome(&base, <Fem2dMutation as Mutation<Fem2dSnapshot>>::diff(&mutation, &base));
    let inverse = <Fem2dMutation as Mutation<Fem2dSnapshot>>::inverse(&mutation, &base).map_err(semio_framework_value::ValueError::into_message)?;
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in inverse.iter().rev() {
        let (next, outcome) = store::os_store::replay::apply_outcome(&undone, <Fem2dMutation as Mutation<Fem2dSnapshot>>::diff(step, &undone));
        undone = next;
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = semio_framework_value::DslValue::object([
        ("base".to_string(), semio_framework_value::ToValue::to_value(&semio_framework_value::ToValue::to_value(&base))),
        ("expectedSnapshot".to_string(), semio_framework_value::ToValue::to_value(&semio_framework_value::ToValue::to_value(&expected))),
        ("snapshot".to_string(), semio_framework_value::ToValue::to_value(&semio_framework_value::ToValue::to_value(&applied))),
        ("diff".to_string(), semio_framework_value::ToValue::to_value(&semio_framework_value::ToValue::to_value(forward.diff()))),
        ("messages".to_string(), semio_framework_value::ToValue::to_value(&semio_framework_value::ToValue::to_value(forward.messages()))),
        ("inverseSteps".to_string(), semio_framework_value::ToValue::to_value(&semio_framework_value::ToValue::to_value(&inverse))),
        ("inverseSnapshot".to_string(), semio_framework_value::ToValue::to_value(&semio_framework_value::ToValue::to_value(&undone))),
        ("inverseMessages".to_string(), semio_framework_value::ToValue::to_value(&semio_framework_value::ToValue::to_value(&inverse_messages))),
    ]);
    Ok(semio_framework_pack_json::to_json_string(&report))
}
}
pub use mutations_codec::*;
