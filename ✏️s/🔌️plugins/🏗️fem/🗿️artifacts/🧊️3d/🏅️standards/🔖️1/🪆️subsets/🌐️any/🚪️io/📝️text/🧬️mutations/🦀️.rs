//! ⚡️ Fem3d artifact — OpText/OpBinary codecs + grammar for `Fem3dMutation`.

use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;


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

/// 📖️ Recognizes exactly one mutation using the declared leaf field contracts.
pub fn mutation_grammar_recognizer()->Result<semio_framework_dsl::Recognizer,semio_framework_diagnostic::TextError>{
    let grammar=semio_framework_dsl::parse_grammar(MUTATION_GRAMMAR_SEMIO)?;
    semio_framework_dsl::Recognizer::compile(&grammar,&semio_framework_dsl::FragmentRegistry::new(),vec![
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_node::CreateNode>("create-node-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_node::DeleteNode>("delete-node-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_element::CreateElement>("create-element-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_element::DeleteElement>("delete-element-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::replace_element::ReplaceElement>("replace-element-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::create_material::CreateMaterial>("create-material-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::delete_material::DeleteMaterial>("delete-material-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::replace_material::ReplaceMaterial>("replace-material-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::change_material::ChangeMaterial>("change-material-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_mesh_sections::standards::v1::subsets::any::schema::mutations::create_section::CreateSection>("create-section-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_mesh_sections::standards::v1::subsets::any::schema::mutations::delete_section::DeleteSection>("delete-section-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_mesh_sections::standards::v1::subsets::any::schema::mutations::replace_section::ReplaceSection>("replace-section-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_mesh_sections::standards::v1::subsets::any::schema::mutations::change_section::ChangeSection>("change-section-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::create_support::CreateSupport>("create-support-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::delete_support::DeleteSupport>("delete-support-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::replace_support::ReplaceSupport>("replace-support-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_mesh_solids::standards::v1::subsets::any::schema::mutations::create_solid::CreateSolid>("create-solid-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_mesh_solids::standards::v1::subsets::any::schema::mutations::delete_solid::DeleteSolid>("delete-solid-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_mesh_solids::standards::v1::subsets::any::schema::mutations::replace_solid::ReplaceSolid>("replace-solid-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::create_load_case::CreateLoadCase>("create-load-case-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::delete_load_case::DeleteLoadCase>("delete-load-case-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::add_load::AddLoad>("add-load-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::remove_load::RemoveLoad>("remove-load-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_load_case_self_weight::ChangeLoadCaseSelfWeight>("change-load-case-self-weight-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::create_combination::CreateCombination>("create-combination-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::delete_combination::DeleteCombination>("delete-combination-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_analysis::standards::v1::subsets::any::schema::mutations::update_analysis_settings::UpdateAnalysisSettings>("update-analysis-settings-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::replace_node::ReplaceNode>("replace-node-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::change_node::ChangeNode>("change-node-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_combination_term::ChangeCombinationTerm>("change-combination-term-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_combination_name::ChangeCombinationName>("change-combination-name-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_load::ChangeLoad>("change-load-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_mesh_elements::standards::v1::subsets::any::schema::mutations::change_element::ChangeElement>("change-element-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::change_support::ChangeSupport>("change-support-fields"),
        semio_framework_dsl_record::record_fields_matcher::<crate::standards::v1::subsets::any::schema::mutations::change_solid::ChangeSolid>("change-solid-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::replace_load::ReplaceLoad>("replace-load-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_load_case_name::ChangeLoadCaseName>("change-load-case-name-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::replace_combination::ReplaceCombination>("replace-combination-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::move_selection::MoveSelection>("move-selection-fields"),
    ])
}

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::load_id;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemAnalysisSettings;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemCombination;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemElement;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use semio_s_fem_material::volumetric::FemMaterialValue;
use semio_s_fem_node::volumetric::FemNodeValue;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemSection;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemSolid;
use protocol::Mutation;
use protocol::ColdMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use store::os_store::store_schema::ArtifactEnvelope;
/// 🌉️ Brings every triad leaf's `mutation` submodule into this file's own scope (declared as
/// siblings back in `🦀️.rs`, not inside this file) — required for the dispatch enum's bare
/// `create_node::CreateNode`-style variant field paths above to resolve.
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::add_load;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_load_case_self_weight;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::create_combination;
use semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_element;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::create_load_case;
use semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::create_material;
use semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_node;
use semio_s_artifact_fem_3d_mesh_sections::standards::v1::subsets::any::schema::mutations::create_section;
use semio_s_artifact_fem_3d_mesh_solids::standards::v1::subsets::any::schema::mutations::create_solid;
use semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::create_support;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::delete_combination;
use semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_element;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::delete_load_case;
use semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::delete_material;
use semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_node;
use semio_s_artifact_fem_3d_mesh_sections::standards::v1::subsets::any::schema::mutations::delete_section;
use semio_s_artifact_fem_3d_mesh_solids::standards::v1::subsets::any::schema::mutations::delete_solid;
use semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::delete_support;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::remove_load;
use semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::replace_element;
use semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::replace_material;
use semio_s_artifact_fem_3d_mesh_sections::standards::v1::subsets::any::schema::mutations::replace_section;
use semio_s_artifact_fem_3d_mesh_solids::standards::v1::subsets::any::schema::mutations::replace_solid;
use semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::replace_support;
use semio_s_artifact_fem_3d_analysis::standards::v1::subsets::any::schema::mutations::update_analysis_settings;
use semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::replace_node;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::replace_load;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_load_case_name;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::replace_combination;
use semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::move_selection;

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
    let (applied, forward) = store::os_store::replay::apply_outcome(&base, <Fem3dMutation as Mutation<Fem3dSnapshot>>::diff(&mutation, &base));
    let inverse = <Fem3dMutation as Mutation<Fem3dSnapshot>>::inverse(&mutation, &base).map_err(semio_framework_value::ValueError::into_message)?;
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in inverse.iter().rev() {
        let (next, outcome) = store::os_store::replay::apply_outcome(&undone, <Fem3dMutation as Mutation<Fem3dSnapshot>>::diff(step, &undone));
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
