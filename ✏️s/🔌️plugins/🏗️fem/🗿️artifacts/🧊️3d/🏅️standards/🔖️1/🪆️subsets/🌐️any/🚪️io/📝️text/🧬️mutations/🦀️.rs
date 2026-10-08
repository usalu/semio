//! ⚡️ Fem3d artifact — OpText/OpBinary codecs + grammar for `Fem3dMutation`.

use crate::standards::v1::subsets::any::schema::mutations::{apply_fem3d_mutation,inverse_fem3d_mutation,Fem3dMutation};


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for Fem3dMutation {
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
use crate::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use crate::Fem3dSnapshot;
use crate::{element_id, load_id, FemAnalysisSettings, FemCombination, FemElement, FemLoad, FemMaterial, FemNode, FemSection, FemSolid};
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};
use store::ArtifactEnvelope;
/// 🌉️ Brings every triad leaf's `mutation` submodule into this file's own scope (declared as
/// siblings back in `🦀️.rs`, not inside this file) — required for the dispatch enum's bare
/// `create_node::CreateNode`-style variant field paths above to resolve.
use crate::standards::v1::subsets::any::schema::mutations::add_load;
use crate::standards::v1::subsets::any::schema::mutations::change_load_case_self_weight;
use crate::standards::v1::subsets::any::schema::mutations::create_combination;
use crate::standards::v1::subsets::any::schema::mutations::create_element;
use crate::standards::v1::subsets::any::schema::mutations::create_load_case;
use crate::standards::v1::subsets::any::schema::mutations::create_material;
use crate::standards::v1::subsets::any::schema::mutations::create_node;
use crate::standards::v1::subsets::any::schema::mutations::create_section;
use crate::standards::v1::subsets::any::schema::mutations::create_solid;
use crate::standards::v1::subsets::any::schema::mutations::create_support;
use crate::standards::v1::subsets::any::schema::mutations::delete_combination;
use crate::standards::v1::subsets::any::schema::mutations::delete_element;
use crate::standards::v1::subsets::any::schema::mutations::delete_load_case;
use crate::standards::v1::subsets::any::schema::mutations::delete_material;
use crate::standards::v1::subsets::any::schema::mutations::delete_node;
use crate::standards::v1::subsets::any::schema::mutations::delete_section;
use crate::standards::v1::subsets::any::schema::mutations::delete_solid;
use crate::standards::v1::subsets::any::schema::mutations::delete_support;
use crate::standards::v1::subsets::any::schema::mutations::remove_load;
use crate::standards::v1::subsets::any::schema::mutations::replace_element;
use crate::standards::v1::subsets::any::schema::mutations::replace_material;
use crate::standards::v1::subsets::any::schema::mutations::replace_section;
use crate::standards::v1::subsets::any::schema::mutations::replace_solid;
use crate::standards::v1::subsets::any::schema::mutations::replace_support;
use crate::standards::v1::subsets::any::schema::mutations::update_analysis_settings;
use crate::standards::v1::subsets::any::schema::mutations::replace_node;
use crate::standards::v1::subsets::any::schema::mutations::replace_load;
use crate::standards::v1::subsets::any::schema::mutations::change_load_case_name;
use crate::standards::v1::subsets::any::schema::mutations::replace_combination;
use crate::standards::v1::subsets::any::schema::mutations::move_selection;

/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no third-party codec or `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `Fem3dMutation` nor
/// `Fem3dSnapshot` can be named there, and hand-transcribing either into a Rust literal
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
pub fn fem3d_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<Fem3dSnapshot, String> {
        let decoded: Fem3dSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
        Ok(decoded)
    };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: Fem3dMutation = semio_framework_pack_json::from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let (applied, forward) = store::apply_outcome(&base, <Fem3dMutation as Mutation<Fem3dSnapshot>>::diff(&mutation, &base));
    let inverse = <Fem3dMutation as Mutation<Fem3dSnapshot>>::inverse(&mutation, &base).map_err(semio_framework_value::ValueError::into_message)?;
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let (next, outcome) = store::apply_outcome(&undone, <Fem3dMutation as Mutation<Fem3dSnapshot>>::diff(step, &undone));
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
