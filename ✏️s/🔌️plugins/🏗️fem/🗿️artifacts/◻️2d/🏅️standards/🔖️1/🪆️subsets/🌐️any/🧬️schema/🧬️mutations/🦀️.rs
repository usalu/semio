//! ⚡️ FEM 2D artifact — semantic mutation dispatch enum + laws (constitutional: op). Every variant
//! wraps exactly one `🧬️mutations/<kind>/🦠️mutation` payload struct implementing
//! `protocol::MutationKind<Fem2dSnapshot, Fem2dMutation>`; `#[derive(dsl::Mutations)]` below
//! generates `impl protocol::Mutation`/`impl protocol::SemanticMutation` by delegating to each
//! payload's own `diff`/`inverse` — see `🧪️MutationsDeriveLaws` in
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` for the reference shape.

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{os_store::store_schema::ArtifactEnvelope, ArtifactStore};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
//#endregion 📖️SemioGrammar

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the fem2d document, derived per
/// `📓️derivation-rules.md` from `Fem2dSnapshot`'s shape (8 id-keyed collections + one inseparable
/// analysis-settings facet). Every generic `Set*`/`Remove*` variant this facet used to carry —
/// including the banned `SetSnapshot` whole-document-replace variant — is gone; whole-document
/// replace is not an in-history mutation at all (routed through `Effect::LoadDocument`, see
/// `reset_document_effect` (io)).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Fem2dSnapshot, diff = Fem2dDiff, schema = "fem.fem2d")]
pub enum Fem2dMutation {
    CreateNode(create_node::CreateNode),
    DeleteNode(delete_node::DeleteNode),
    CreateElement(create_element::CreateElement),
    DeleteElement(delete_element::DeleteElement),
    ReplaceElement(replace_element::ReplaceElement),
    CreateMaterial(create_material::CreateMaterial),
    DeleteMaterial(delete_material::DeleteMaterial),
    ReplaceMaterial(replace_material::ReplaceMaterial),
    CreateSection(create_section::CreateSection),
    DeleteSection(delete_section::DeleteSection),
    ReplaceSection(replace_section::ReplaceSection),
    CreateSupport(create_support::CreateSupport),
    DeleteSupport(delete_support::DeleteSupport),
    ReplaceSupport(replace_support::ReplaceSupport),
    CreateRegion(create_region::CreateRegion),
    DeleteRegion(delete_region::DeleteRegion),
    ReplaceRegion(replace_region::ReplaceRegion),
    CreateLoadCase(create_load_case::CreateLoadCase),
    DeleteLoadCase(delete_load_case::DeleteLoadCase),
    AddLoad(add_load::AddLoad),
    RemoveLoad(remove_load::RemoveLoad),
    ChangeLoadCaseSelfWeight(change_load_case_self_weight::ChangeLoadCaseSelfWeight),
    CreateCombination(create_combination::CreateCombination),
    DeleteCombination(delete_combination::DeleteCombination),
    UpdateAnalysisSettings(update_analysis_settings::UpdateAnalysisSettings),
    ReplaceNode(replace_node::ReplaceNode),
    ReplaceLoad(replace_load::ReplaceLoad),
    ChangeLoadCaseName(change_load_case_name::ChangeLoadCaseName),
    ReplaceCombination(replace_combination::ReplaceCombination),
    MoveSelection(move_selection::MoveSelection),
}
//#endregion 🔖️Mutations

//#region 🔖️LeafImports
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
//#endregion 🔖️LeafImports

pub type Fem2dEnvelope = ArtifactEnvelope<Fem2dSnapshot, Fem2dMutation>;
pub type Fem2dStore = ArtifactStore<Fem2dSnapshot, Fem2dMutation>;

//#region 🔖️GenericDelegates
pub fn inverse_fem2d_mutation(snapshot: &Fem2dSnapshot, mutation: &Fem2dMutation) -> Result<Vec<Fem2dMutation>, semio_framework_value::ValueError> {
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
/// 🏷️ Kebab-case spelling of every `Fem2dMutation` variant, in declaration order — the vocabulary the `fem2d-1-any` mutation catalog
/// (`../../🔣️oracle.json`) declares and the `mutate-fem2d-1` exhaustive test case measures
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
    "create-section",
    "delete-section",
    "replace-section",
    "create-support",
    "delete-support",
    "replace-support",
    "create-region",
    "delete-region",
    "replace-region",
    "create-load-case",
    "delete-load-case",
    "add-load",
    "remove-load",
    "change-load-case-self-weight",
    "create-combination",
    "delete-combination",
    "update-analysis-settings",
    "replace-node",
    "replace-load",
    "change-load-case-name",
    "replace-combination",
    "move-selection",
];
//#endregion 🔖️Kinds

//#region 🌉️TestBridge


/// 🔢️ The three planar degrees of freedom this artifact's 2D elements can carry, as the wire spells
/// them. A `Beam` contributes all three at each end, a `Bar` only the two translations, and a node no
/// element touches carries no equation at all — which is why a support on such a node is inert.
pub(crate) const PLANAR_DOFS: [&str; 3] = ["Tx", "Ty", "Rz"];
















//#endregion 🌉️TestBridge

//#region 🧪️KindsConformance
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-conformance/🦀️.rs"]
mod kinds_conformance;
//#endregion 🧪️KindsConformance
