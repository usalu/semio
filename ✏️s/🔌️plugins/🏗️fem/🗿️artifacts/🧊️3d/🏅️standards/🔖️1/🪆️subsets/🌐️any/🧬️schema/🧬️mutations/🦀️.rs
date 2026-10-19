//! wraps exactly one `🧬️mutations/<kind>/🦠️mutation` payload struct implementing
//! ⚡️ FEM 3D artifact — semantic mutation dispatch enum + laws (constitutional: op). Every variant
//! `protocol::MutationKind<Fem3dSnapshot, Fem3dMutation>`; `#[derive(dsl::Mutations)]` below
//! generates `impl protocol::Mutation`/`impl protocol::SemanticMutation` by delegating to each
//! payload's own `diff`/`inverse` — see `🧪️MutationsDeriveLaws` in
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` for the reference shape.

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
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
use store::ArtifactStore;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
//#endregion 📖️SemioGrammar

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the fem3d document, derived per
/// `📓️derivation-rules.md` from `Fem3dSnapshot`'s shape (8 id-keyed collections + one inseparable
/// analysis-settings facet). Every generic `Set*`/`Remove*` variant this facet used to carry —
/// including the banned `SetSnapshot` whole-document-replace variant — is gone; whole-document
/// replace is not an in-history mutation at all (routed through `ArtifactStore::reset` /
/// `Effect::LoadDocument`, see
/// `editor::fem3d::reset_document_effect`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Fem3dSnapshot, diff = Fem3dDiff, reference = Fem3dSnapshotValue, reference_diff = Fem3dDiff, schema = "fem.fem3d")]
pub enum Fem3dMutation {
    CreateNode(semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_node::CreateNode),
    DeleteNode(semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_node::DeleteNode),
    CreateElement(semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::create_element::CreateElement),
    DeleteElement(semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_element::DeleteElement),
    ReplaceElement(semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::replace_element::ReplaceElement),
    CreateMaterial(semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::create_material::CreateMaterial),
    DeleteMaterial(semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::delete_material::DeleteMaterial),
    ReplaceMaterial(semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::replace_material::ReplaceMaterial),
    ChangeMaterial(semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::change_material::ChangeMaterial),
    CreateSection(semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::create_section::CreateSection),
    DeleteSection(semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::delete_section::DeleteSection),
    ReplaceSection(semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::replace_section::ReplaceSection),
    ChangeSection(semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::change_section::ChangeSection),
    CreateSupport(semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::create_support::CreateSupport),
    DeleteSupport(semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::delete_support::DeleteSupport),
    ReplaceSupport(semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::replace_support::ReplaceSupport),
    CreateSolid(semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::create_solid::CreateSolid),
    DeleteSolid(semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::delete_solid::DeleteSolid),
    ReplaceSolid(semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::replace_solid::ReplaceSolid),
    CreateLoadCase(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::create_load_case::CreateLoadCase),
    DeleteLoadCase(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::delete_load_case::DeleteLoadCase),
    AddLoad(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::add_load::AddLoad),
    RemoveLoad(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::remove_load::RemoveLoad),
    ChangeLoadCaseSelfWeight(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_load_case_self_weight::ChangeLoadCaseSelfWeight),
    CreateCombination(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::create_combination::CreateCombination),
    DeleteCombination(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::delete_combination::DeleteCombination),
    UpdateAnalysisSettings(semio_s_artifact_fem_3d_analysis::standards::v1::subsets::any::schema::mutations::update_analysis_settings::UpdateAnalysisSettings),
    ReplaceNode(semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::replace_node::ReplaceNode),
    ChangeNode(semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::change_node::ChangeNode),
    ChangeCombinationTerm(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_combination_term::ChangeCombinationTerm),
    ChangeCombinationName(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_combination_name::ChangeCombinationName),
    ChangeLoad(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_load::ChangeLoad),
    ChangeElement(semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::change_element::ChangeElement),
    ChangeSupport(semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::change_support::ChangeSupport),
    ChangeSolid(semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::change_solid::ChangeSolid),
    ReplaceLoad(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::replace_load::ReplaceLoad),
    ChangeLoadCaseName(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_load_case_name::ChangeLoadCaseName),
    ReplaceCombination(semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::replace_combination::ReplaceCombination),
    MoveSelection(semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::move_selection::MoveSelection),
}
//#endregion 🔖️Mutations

//#region 🔖️LeafImports
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
use semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::create_section;
use semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::create_solid;
use semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::create_support;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::delete_combination;
use semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_element;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::delete_load_case;
use semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::delete_material;
use semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::delete_node;
use semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::delete_section;
use semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::delete_solid;
use semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::delete_support;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::remove_load;
use semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::replace_element;
use semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::replace_material;
use semio_s_artifact_fem_3d_material::standards::v1::subsets::any::schema::mutations::change_material;
use semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::replace_section;
use semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::change_section;
use semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::replace_solid;
use semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::replace_support;
use semio_s_artifact_fem_3d_analysis::standards::v1::subsets::any::schema::mutations::update_analysis_settings;
use semio_s_artifact_fem_3d_mesh_topology::standards::v1::subsets::any::schema::mutations::replace_node;
use semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::change_node;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_combination_term;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_combination_name;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_load;
use semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::change_element;
use semio_s_artifact_fem_3d_boundary::standards::v1::subsets::any::schema::mutations::change_support;
use semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::change_solid;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::replace_load;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::change_load_case_name;
use semio_s_artifact_fem_3d_load::standards::v1::subsets::any::schema::mutations::replace_combination;
use semio_s_artifact_fem_3d_mesh::standards::v1::subsets::any::schema::mutations::move_selection;
//#endregion 🔖️LeafImports

//#region 🛡️Guards











































//#endregion 🛡️Guards

pub type Fem3dEnvelope = ArtifactEnvelope<Fem3dSnapshot, Fem3dMutation>;
pub type Fem3dStore = ArtifactStore<Fem3dSnapshot, Fem3dMutation>;

//#region 🔖️GenericDelegates
pub fn inverse_fem3d_mutation_cold(snapshot: &Fem3dSnapshotValue, mutation: &Fem3dMutation) -> Result<Vec<Fem3dMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(snapshot)?

    })
}
//#endregion 🔖️GenericDelegates

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests


//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `Fem3dMutation` variant, in declaration order — the vocabulary the `fem3d-1-any` mutation catalog
/// (`../../🔣️oracle.json`) declares and the `mutate-fem3d-1` exhaustive test case measures
/// itself against. The framework never parses Rust, so `kinds_match_the_enum_and_the_catalog` below is
/// what keeps this list honest in both directions.
pub const KINDS: &[&str] = &[
    "create-node",
    "delete-node",
    "create-element",
    "delete-element",
    "replace-element",
    "create-material",
    "delete-material",
    "replace-material",
    "change-material",
    "create-section",
    "delete-section",
    "replace-section",
    "change-section",
    "create-support",
    "delete-support",
    "replace-support",
    "create-solid",
    "delete-solid",
    "replace-solid",
    "create-load-case",
    "delete-load-case",
    "add-load",
    "remove-load",
    "change-load-case-self-weight",
    "create-combination",
    "delete-combination",
    "update-analysis-settings",
    "replace-node",
    "change-node",
    "change-combination-term",
    "change-combination-name",
    "change-load",
    "change-element",
    "change-support",
    "change-solid",
    "replace-load",
    "change-load-case-name",
    "replace-combination",
    "move-selection",
];
//#endregion 🔖️Kinds

//#region 🌉️TestBridge

//#endregion 🌉️TestBridge

//#region 🧪️KindsConformance
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-conformance/🦀️.rs"]
mod kinds_conformance;
//#endregion 🧪️KindsConformance
