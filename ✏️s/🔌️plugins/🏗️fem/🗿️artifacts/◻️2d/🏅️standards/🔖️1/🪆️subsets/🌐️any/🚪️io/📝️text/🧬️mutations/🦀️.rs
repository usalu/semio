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

/// 📖️ Recognizes exactly one mutation using the declared leaf field contracts.
pub fn mutation_grammar_recognizer()->Result<semio_framework_dsl::Recognizer,semio_framework_diagnostic::TextError>{
    let grammar=semio_framework_dsl::parse_grammar(MUTATION_GRAMMAR_SEMIO)?;
    semio_framework_dsl::Recognizer::compile(&grammar,&semio_framework_dsl::FragmentRegistry::new(),vec![
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_node::CreateNode>("create-node-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_node::DeleteNode>("delete-node-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_element::CreateElement>("create-element-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_element::DeleteElement>("delete-element-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::replace_element::ReplaceElement>("replace-element-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_material::standards::v1::subsets::any::schema::mutations::create_material::CreateMaterial>("create-material-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_material::standards::v1::subsets::any::schema::mutations::delete_material::DeleteMaterial>("delete-material-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_material::standards::v1::subsets::any::schema::mutations::replace_material::ReplaceMaterial>("replace-material-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_material::standards::v1::subsets::any::schema::mutations::change_material::ChangeMaterial>("change-material-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_mesh_sections::standards::v1::subsets::any::schema::mutations::create_section::CreateSection>("create-section-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_mesh_sections::standards::v1::subsets::any::schema::mutations::delete_section::DeleteSection>("delete-section-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_mesh_sections::standards::v1::subsets::any::schema::mutations::replace_section::ReplaceSection>("replace-section-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_mesh_sections::standards::v1::subsets::any::schema::mutations::change_section::ChangeSection>("change-section-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_boundary::standards::v1::subsets::any::schema::mutations::create_support::CreateSupport>("create-support-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_boundary::standards::v1::subsets::any::schema::mutations::delete_support::DeleteSupport>("delete-support-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_boundary::standards::v1::subsets::any::schema::mutations::replace_support::ReplaceSupport>("replace-support-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_mesh_regions::standards::v1::subsets::any::schema::mutations::create_region::CreateRegion>("create-region-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_mesh_regions::standards::v1::subsets::any::schema::mutations::delete_region::DeleteRegion>("delete-region-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_mesh_regions::standards::v1::subsets::any::schema::mutations::replace_region::ReplaceRegion>("replace-region-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::create_load_case::CreateLoadCase>("create-load-case-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::delete_load_case::DeleteLoadCase>("delete-load-case-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::add_load::AddLoad>("add-load-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::remove_load::RemoveLoad>("remove-load-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::change_load_case_self_weight::ChangeLoadCaseSelfWeight>("change-load-case-self-weight-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::create_combination::CreateCombination>("create-combination-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::delete_combination::DeleteCombination>("delete-combination-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_analysis::standards::v1::subsets::any::schema::mutations::update_analysis_settings::UpdateAnalysisSettings>("update-analysis-settings-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::replace_node::ReplaceNode>("replace-node-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_mesh::standards::v1::subsets::any::schema::mutations::change_node::ChangeNode>("change-node-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::change_combination_term::ChangeCombinationTerm>("change-combination-term-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::change_combination_name::ChangeCombinationName>("change-combination-name-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::change_load::ChangeLoad>("change-load-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_mesh_elements::standards::v1::subsets::any::schema::mutations::change_element::ChangeElement>("change-element-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_boundary::standards::v1::subsets::any::schema::mutations::change_support::ChangeSupport>("change-support-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_mesh_regions::standards::v1::subsets::any::schema::mutations::change_region::ChangeRegion>("change-region-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::replace_load::ReplaceLoad>("replace-load-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::change_load_case_name::ChangeLoadCaseName>("change-load-case-name-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::replace_combination::ReplaceCombination>("replace-combination-fields"),
        semio_framework_dsl_record::record_fields_matcher::<semio_s_artifact_fem_2d_mesh::standards::v1::subsets::any::schema::mutations::move_selection::MoveSelection>("move-selection-fields"),
    ])
}

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use protocol::{ColdMutation, Mutation};

use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
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
use semio_s_artifact_fem_2d_mesh_regions::standards::v1::subsets::any::schema::mutations::create_region;
use semio_s_artifact_fem_2d_mesh_sections::standards::v1::subsets::any::schema::mutations::create_section;
use semio_s_artifact_fem_2d_boundary::standards::v1::subsets::any::schema::mutations::create_support;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::delete_combination;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_element;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::delete_load_case;
use semio_s_artifact_fem_2d_material::standards::v1::subsets::any::schema::mutations::delete_material;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_node;
use semio_s_artifact_fem_2d_mesh_regions::standards::v1::subsets::any::schema::mutations::delete_region;
use semio_s_artifact_fem_2d_mesh_sections::standards::v1::subsets::any::schema::mutations::delete_section;
use semio_s_artifact_fem_2d_boundary::standards::v1::subsets::any::schema::mutations::delete_support;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::remove_load;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::replace_combination;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::replace_element;
use semio_s_artifact_fem_2d_load::standards::v1::subsets::any::schema::mutations::replace_load;
use semio_s_artifact_fem_2d_material::standards::v1::subsets::any::schema::mutations::replace_material;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::replace_node;
use semio_s_artifact_fem_2d_mesh::standards::v1::subsets::any::schema::mutations::move_selection;
use semio_s_artifact_fem_2d_mesh_regions::standards::v1::subsets::any::schema::mutations::replace_region;
use semio_s_artifact_fem_2d_mesh_sections::standards::v1::subsets::any::schema::mutations::replace_section;
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
    let (applied, forward) = store::os_store::replay::apply_outcome(&base, <Fem2dMutation as ColdMutation<Fem2dSnapshot>>::diff(&mutation, &base));
    let inverse = <Fem2dMutation as ColdMutation<Fem2dSnapshot>>::inverse(&mutation, &base).map_err(semio_framework_value::ValueError::into_message)?;
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in inverse.iter().rev() {
        let (next, outcome) = store::os_store::replay::apply_outcome(&undone, <Fem2dMutation as ColdMutation<Fem2dSnapshot>>::diff(step, &undone));
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

/// 🗂️ The three projection axes of an analysis report, computed from the document so that two
/// implementations order the same values the same way: every node a `bar`/`beam` element references
/// in document order, every restrained-and-active `(node, dof)` pair in that same order, and the
/// `bar`/`beam` members in document order. Regions are outside the projection — a meshed continuum
/// is not something a frame solver can express, and this subset's region geometry already has its own
/// two third-party oracles.
pub(crate) fn fem2d_frame_axes(doc: &Fem2dSnapshot) -> (Vec<String>, Vec<(String, String)>, Vec<String>) {
    let ends = |element: &semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemElement| match element {
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemElement::Bar { start, end, .. } => (start.clone(), end.clone(), false),
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemElement::Beam { start, end, .. } => (start.clone(), end.clone(), true),
    };
    let mut active: Vec<(String, [bool; 3])> = Vec::new();
    for element in &doc.elements {
        let (start, end, bends) = ends(element);
        for node in [start, end] {
            match active.iter().position(|(id, _)| id == &node) {
                Some(at) => {
                    active[at].1 = [true, true, active[at].1[2] || bends];
                }
                None => active.push((node, [true, true, bends])),
            }
        }
    }
    let nodes: Vec<String> = doc.nodes.iter().map(|node| node.id.clone()).filter(|id| active.iter().any(|(node, _)| node == id)).collect();
    let mut pairs = Vec::new();
    for node in &nodes {
        let flags = active.iter().find(|(id, _)| id == node).map_or([false; 3], |(_, flags)| *flags);
        for (at, name) in PLANAR_DOFS.iter().enumerate() {
            let held = doc.supports.iter().any(|support| &support.node_id == node && support.fixed.iter().any(|dof| fem2d_dof_name(*dof) == *name));
            if flags[at] && held {
                pairs.push((node.clone(), (*name).to_string()));
            }
        }
    }
    let members: Vec<String> = doc.elements.iter().map(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id).map(str::to_string).collect();
    (nodes, pairs, members)
}

/// 🔤️ A document degree of freedom as the wire spells it.
pub(crate) fn fem2d_dof_name(dof: semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof) -> &'static str {
    match dof {
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Tx => "Tx",
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Ty => "Ty",
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Tz => "Tz",
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Rx => "Rx",
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Ry => "Ry",
        semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof::Rz => "Rz",
    }
}

/// 📤️ One solved case, projected onto the three axes.
pub(crate) fn fem2d_case_value(result: &semio_s_fem_model::StaticResult, nodes: &[String], pairs: &[(String, String)], members: &[String]) -> semio_framework_value::DslValue {
    let displacements = nodes
        .iter()
        .map(|node| {
            let found = result.displacements.iter().find(|entry| &entry.node_id == node);
            let values = found.map_or([0.0; 6], |entry| entry.values);
            semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::float(values[0]), semio_framework_value::DslValue::float(values[1]), semio_framework_value::DslValue::float(values[5])])
        })
        .collect();
    let reactions = pairs
        .iter()
        .map(|(node, dof)| {
            let value = result.reactions.iter().find(|entry| &entry.node_id == node && fem2d_dof_name_of(entry.dof) == dof.as_str()).map_or(0.0, |entry| entry.value);
            semio_framework_value::DslValue::float(value)
        })
        .collect();
    let elements = members
        .iter()
        .map(|member| {
            let six = match result.elements.iter().find(|(id, _)| id == member).map(|(_, value)| value) {
                Some(semio_s_fem_model::ElementResult::Bar { n }) => [*n, 0.0, 0.0, *n, 0.0, 0.0],
                Some(semio_s_fem_model::ElementResult::Beam { stations }) => match (stations.first(), stations.last()) {
                    (Some(head), Some(tail)) => [head.n, head.v, head.m, tail.n, tail.v, tail.m],
                    _ => [0.0; 6],
                },
                _ => [0.0; 6],
            };
            semio_framework_value::DslValue::Array(six.iter().map(|value| semio_framework_value::DslValue::float(*value)).collect())
        })
        .collect();
    semio_framework_value::DslValue::Object(vec![("displacements".to_string(), semio_framework_value::DslValue::Array(displacements)), ("reactions".to_string(), semio_framework_value::DslValue::Array(reactions)), ("elements".to_string(), semio_framework_value::DslValue::Array(elements))])
}

/// 🔤️ A solver degree of freedom as the wire spells it.
pub(crate) fn fem2d_dof_name_of(dof: semio_s_fem_model::Dof) -> &'static str {
    match dof {
        semio_s_fem_model::Dof::Tx => "Tx",
        semio_s_fem_model::Dof::Ty => "Ty",
        semio_s_fem_model::Dof::Tz => "Tz",
        semio_s_fem_model::Dof::Rx => "Rx",
        semio_s_fem_model::Dof::Ry => "Ry",
        semio_s_fem_model::Dof::Rz => "Rz",
    }
}

/// 🧮️ One JSON report of the linear-static analysis of `snapshot_json` — the production surface the
/// `🧮️solves-fem2d-1-benchmarks` case's third-party solver oracle is compared against.
///
/// Values are SI and in this artifact's own frame: metres, newtons, newton-metres, radians; `x`
/// right and `y` up; `rz` counter-clockwise positive; a reaction is the force the support applies TO
/// the structure; a member's axial force is tension-positive. Cases come first in document order,
/// then combinations. A model whose stiffness matrix is singular reports `error` and no cases at all,
/// which is the only honest answer for a mechanism.
///
/// @see ../../../📈️analysis/🧪️tests/🧮️solves-fem2d-1-benchmarks/🥒️.feature
pub fn fem2d_analysis_report_json(snapshot_json: &str) -> Result<String, String> {
    let doc: Fem2dSnapshot = semio_framework_pack_json::from_json_str(snapshot_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let (nodes, pairs, members) = fem2d_frame_axes(&doc);
    let axes = |value: semio_framework_value::DslValue| {
        semio_framework_value::DslValue::Object(vec![
            ("nodes".to_string(), semio_framework_value::DslValue::Array(nodes.iter().map(|node| semio_framework_value::DslValue::String(node.clone())).collect())),
            ("reactions".to_string(), semio_framework_value::DslValue::Array(pairs.iter().map(|(node, dof)| semio_framework_value::DslValue::String(format!("{node}.{dof}"))).collect())),
            ("members".to_string(), semio_framework_value::DslValue::Array(members.iter().map(|member| semio_framework_value::DslValue::String(member.clone())).collect())),
            ("cases".to_string(), value),
        ])
    };
    match semio_s_artifact_fem_2d_engine::fem2d_engine::fem2d_solve_all(&doc) {
        Err(error) => Ok(semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Object(vec![("error".to_string(), semio_framework_value::DslValue::String(error.to_string()))]))),
        Ok(results) => {
            let mut cases = Vec::new();
            for id in doc.load_cases.iter().map(|case| case.id.clone()).chain(doc.combinations.iter().map(|combination| combination.id.clone())) {
                let Some(result) = results.get(&id) else {
                    return Err(format!("fem2d_solve_all returned no result for {id:?}"));
                };
                let mut entry = vec![("id".to_string(), semio_framework_value::DslValue::String(id.clone()))];
                if let semio_framework_value::DslValue::Object(fields) = fem2d_case_value(result, &nodes, &pairs, &members) {
                    entry.extend(fields);
                }
                cases.push(semio_framework_value::DslValue::Object(entry));
            }
            Ok(semio_framework_pack_json::to_json_string(&axes(semio_framework_value::DslValue::Array(cases))))
        }
    }
}

/// 🧱️ Continuum responses in authored-node order with stress extrema and resolved eigen answers.
pub fn fem2d_continuum_report_json(snapshot_json: &str, include_eigen: bool) -> Result<String, String> {
    use semio_framework_value::DslValue as Value;
    let doc: Fem2dSnapshot = semio_framework_pack_json::from_json_str(snapshot_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let results = semio_s_artifact_fem_2d_engine::fem2d_engine::fem2d_solve_all(&doc).map_err(|error| error.to_string())?;
    let array = |values: &[f64]| Value::Array(values.iter().map(|value| Value::float(*value)).collect());
    let mut cases = Vec::new();
    for case in &doc.load_cases {
        let result = results.get(&case.id).ok_or_else(|| format!("missing solved case {}", case.id))?;
        let mut displacements = Vec::new();
        let mut reactions = Vec::new();
        for node in &doc.nodes {
            let value = result.displacements.iter().find(|value| value.node_id == node.id).ok_or_else(|| format!("missing displacement {}", node.id))?;
            displacements.push(array(&value.values[..2]));
            for dof in [semio_s_fem_model::Dof::Tx, semio_s_fem_model::Dof::Ty] {
                if let Some(reaction) = result.reactions.iter().find(|value| value.node_id == node.id && value.dof == dof) {
                    reactions.push(reaction.value);
                }
            }
        }
        let stresses: Vec<[f64; 3]> = result.elements.iter().flat_map(|(_, element)| match element {
            semio_s_fem_model::ElementResult::Plane { gauss } => gauss.iter().map(|stress| [stress.sxx, stress.syy, stress.sxy]).collect(),
            _ => Vec::new(),
        }).collect();
        if stresses.is_empty() {
            return Err("continuum report requires plane elements".to_string());
        }
        let mut mean = [0.0; 3];
        let mut minimum = [f64::INFINITY; 3];
        let mut maximum = [f64::NEG_INFINITY; 3];
        for stress in &stresses {
            for axis in 0..3 {
                mean[axis] += stress[axis] / stresses.len() as f64;
                minimum[axis] = minimum[axis].min(stress[axis]);
                maximum[axis] = maximum[axis].max(stress[axis]);
            }
        }
        let mut fields = vec![("displacements".to_string(), Value::Array(displacements)), ("reactions".to_string(), array(&reactions)), ("stress".to_string(), array(&mean)), ("stressMin".to_string(), array(&minimum)), ("stressMax".to_string(), array(&maximum))];
        if include_eigen {
            let result = semio_s_artifact_fem_2d_engine::fem2d_engine::modal_buckling::fem2d_buckling(&doc, &case.id).map_err(|error| error.to_string())?;
            fields.push(("factors".to_string(), array(&result.factors)));
        }
        cases.push((case.id.clone(), Value::Object(fields)));
    }
    let frequencies = if include_eigen {
        semio_s_artifact_fem_2d_engine::fem2d_engine::modal_buckling::fem2d_modal(&doc).map_err(|error| error.to_string())?.frequencies_hz
    } else {
        Vec::new()
    };
    Ok(semio_framework_pack_json::to_json_string(&Value::Object(vec![("cases".to_string(), Value::Object(cases)), ("frequenciesHz".to_string(), array(&frequencies))])))
}

/// 🧬️ Applies one typed mutation to `base_json` through the SAME production diff/apply path
/// `fem2d_mutation_report_json` uses, then reports the analysis of what it left behind — the
/// mutate-then-solve half of `🧮️solves-fem2d-1-benchmarks`.
pub fn fem2d_mutated_analysis_report_json(base_json: &str, mutation_json: &str) -> Result<String, String> {
    let base: Fem2dSnapshot = semio_framework_pack_json::from_json_str(base_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let mutation: Fem2dMutation = semio_framework_pack_json::from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let (applied, outcome) = store::os_store::replay::apply_outcome(&base, <Fem2dMutation as ColdMutation<Fem2dSnapshot>>::diff(&mutation, &base));
    let faults: Vec<String> = outcome.messages().iter().filter(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)).map(|message| format!("{:?}", message.code)).collect();
    if !faults.is_empty() {
        return Err(format!("the mutation was rejected with {faults:?}"));
    }
    fem2d_analysis_report_json(&semio_framework_pack_json::to_json_string(&applied))
}

/// 🎵️ One JSON report of the modal analysis of `snapshot_json` — the natural frequencies in hertz,
/// as many as the document's own `analysis.modalCount` asks for.
pub fn fem2d_modal_report_json(snapshot_json: &str) -> Result<String, String> {
    let doc: Fem2dSnapshot = semio_framework_pack_json::from_json_str(snapshot_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    match semio_s_artifact_fem_2d_engine::fem2d_engine::modal_buckling::fem2d_modal(&doc) {
        Err(error) => Ok(semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Object(vec![("error".to_string(), semio_framework_value::DslValue::String(error.to_string()))]))),
        Ok(result) => {
            Ok(semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Object(vec![("frequenciesHz".to_string(), semio_framework_value::DslValue::Array(result.frequencies_hz.iter().take(doc.analysis.modal_count as usize).map(|value| semio_framework_value::DslValue::float(*value)).collect()))])))
        }
    }
}

/// 🏛️ One JSON report of the linear-buckling analysis of `snapshot_json` — the lowest load factor of
/// every load case the document declares, keyed by case id.
pub fn fem2d_buckling_report_json(snapshot_json: &str) -> Result<String, String> {
    let doc: Fem2dSnapshot = semio_framework_pack_json::from_json_str(snapshot_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let mut factors = Vec::new();
    for case in &doc.load_cases {
        match semio_s_artifact_fem_2d_engine::fem2d_engine::modal_buckling::fem2d_buckling(&doc, &case.id) {
            Err(error) => return Ok(semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Object(vec![("error".to_string(), semio_framework_value::DslValue::String(error.to_string()))]))),
            Ok(result) => match result.factors.first() {
                Some(value) => factors.push((case.id.clone(), semio_framework_value::DslValue::float(*value))),
                None => return Ok(semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Object(vec![("error".to_string(), semio_framework_value::DslValue::String(format!("no buckling factor for {}", case.id)))]))),
            },
        }
    }
    Ok(semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Object(vec![("factors".to_string(), semio_framework_value::DslValue::Object(factors))])))
}
}
pub use mutations_codec::*;
