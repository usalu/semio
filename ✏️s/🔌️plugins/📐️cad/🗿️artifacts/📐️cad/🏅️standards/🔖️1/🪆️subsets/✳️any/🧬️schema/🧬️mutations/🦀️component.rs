//! 🧬️ CAD artifact — document mutation dispatch enum + shared internal patch/helper types.
//! Every variant wraps exactly one `🧬️mutations/<kind>/🦠️mutation` payload struct implementing
//! `protocol::MutationKind<CadSnapshot, CadMutation>`; `#[derive(dsl::Mutations)]` below
//! generates `impl protocol::Mutation`/`impl protocol::SemanticMutation` by delegating to each
//! payload's own `diff`/`inverse` — see `🧪️MutationsDeriveLaws` in
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs` for the reference shape.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: the fourteen triads that used to
//! mutate `CadObject` fields inline on the parent document (`create-object`, `delete-object`,
//! `move-object`, `rotate-object`, `scale-object`, `rename-object`, `change-object-visible`,
//! `change-object-locked`, `change-object-typology`, `replace-object-geometry`,
//! `replace-pane-objects`, `drag-objects`, `rotate-objects`, `scale-objects`) are RETIRED — that
//! data now lives inside the four composed `s.stdio.semio.model` CHILD documents (`shape_model`/
//! `building_model`/`energy_model`/`structure_classic_model`), each its own document with its own
//! independent mutation history; a per-element move/rotate/rename now targets the CHILD document
//! directly, never this parent enum (`🔖️Composition`'s "a parent's diff never embeds a child
//! diff" rule). What replaces them here is real CHILD-SLOT LIFECYCLE — `create`/`delete` for each
//! of the four fixed model slots plus the `drawings` collection — approved verbs only, per
//! `📌️important.md`.

use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::CadSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️InternalPatches
/// 🩹 Option-bag field delta for [`crate::artifacts::cad::CadNode`] — INTERNAL diff-construction glue only.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadNodePatch {
    pub label: Option<String>,
}

/// 🩹 Option-bag field delta for [`crate::artifacts::cad::CadReference`] — INTERNAL diff-construction glue only.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadReferencePatch {
    pub source_url: Option<String>,
    pub media_kind: Option<String>,
    pub origin: Option<[f64; 3]>,
    pub orientation: Option<[f64; 4]>,
    pub scale: Option<f64>,
    pub width_world: Option<f64>,
    pub hidden: Option<bool>,
    pub locked: Option<bool>,
    pub opacity: Option<f64>,
}
//#endregion 🔖️InternalPatches

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the cad document, derived per
/// `📓️derivation-rules.md` from `CadSnapshot`'s shape. `SetSnapshot`/`SetPaneObjects`-as-whole-doc-
/// replace and every generic `Patch*`/`CollectionMutation` variant this facet used to carry are
/// gone — whole-document replace is not an in-history mutation at all (routed through
/// `ArtifactStore::reset`, see `CadPlayApp::whole_document_operation` returning `None` now).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = CadSnapshot, diff = CadDiff, schema = "cad.cad")]
pub enum CadMutation {
    CreateShapeModel(create_shape_model::mutation::CreateShapeModel),
    DeleteShapeModel(delete_shape_model::mutation::DeleteShapeModel),
    CreateBuildingModel(create_building_model::mutation::CreateBuildingModel),
    DeleteBuildingModel(delete_building_model::mutation::DeleteBuildingModel),
    CreateEnergyModel(create_energy_model::mutation::CreateEnergyModel),
    DeleteEnergyModel(delete_energy_model::mutation::DeleteEnergyModel),
    CreateStructureClassicModel(create_structure_classic_model::mutation::CreateStructureClassicModel),
    DeleteStructureClassicModel(delete_structure_classic_model::mutation::DeleteStructureClassicModel),
    CreateDrawing(create_drawing::mutation::CreateDrawing),
    DeleteDrawing(delete_drawing::mutation::DeleteDrawing),
    CreateNode(create_node::mutation::CreateNode),
    DeleteNode(delete_node::mutation::DeleteNode),
    RenameNode(rename_node::mutation::RenameNode),
    ChangeReferenceHidden(change_reference_hidden::mutation::ChangeReferenceHidden),
    ChangeReferenceLocked(change_reference_locked::mutation::ChangeReferenceLocked),
    ChangeReferenceWidth(change_reference_width::mutation::ChangeReferenceWidth),
    MoveReference(move_reference::mutation::MoveReference),
    ReplaceReferenceMedia(replace_reference_media::mutation::ReplaceReferenceMedia),
    ReplaceReferences(replace_references::mutation::ReplaceReferences),
    ChangeActiveModelDefinition(change_active_model_definition::mutation::ChangeActiveModelDefinition),
}
//#endregion 🔖️Mutations

//#region 🔖️Leaves
use super::create_shape_model;
use super::delete_shape_model;
use super::create_building_model;
use super::delete_building_model;
use super::create_energy_model;
use super::delete_energy_model;
use super::create_structure_classic_model;
use super::delete_structure_classic_model;
use super::create_drawing;
use super::delete_drawing;
use super::create_node;
use super::delete_node;
use super::rename_node;
use super::change_reference_hidden;
use super::change_reference_locked;
use super::change_reference_width;
use super::move_reference;
use super::replace_reference_media;
use super::replace_references;
use super::change_active_model_definition;
//#endregion 🔖️Leaves

//#region 🧪️Tests
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::artifacts::cad::mutations::{
        change_active_model_definition::mutation::ChangeActiveModelDefinition, change_reference_hidden::mutation::ChangeReferenceHidden, change_reference_locked::mutation::ChangeReferenceLocked,
        change_reference_width::mutation::ChangeReferenceWidth, create_building_model::mutation::CreateBuildingModel, create_drawing::mutation::CreateDrawing, create_energy_model::mutation::CreateEnergyModel,
        create_node::mutation::CreateNode, create_shape_model::mutation::CreateShapeModel, create_structure_classic_model::mutation::CreateStructureClassicModel, delete_building_model::mutation::DeleteBuildingModel,
        delete_drawing::mutation::DeleteDrawing, delete_energy_model::mutation::DeleteEnergyModel, delete_node::mutation::DeleteNode, delete_shape_model::mutation::DeleteShapeModel,
        delete_structure_classic_model::mutation::DeleteStructureClassicModel, move_reference::mutation::MoveReference, rename_node::mutation::RenameNode, replace_reference_media::mutation::ReplaceReferenceMedia,
        replace_references::mutation::ReplaceReferences,
    };
    use crate::artifacts::cad::testkit::{sample_model_child, sample_reference, sample_scene};
    use protocol::Mutation;

    /// ⚖️ One value per `CadMutation` variant — the closed set every wire law below iterates.
    pub fn every_mutation() -> Vec<CadMutation> {
        let sample = sample_model_child("fresh-model-1");
        vec![
            CadMutation::CreateShapeModel(CreateShapeModel { child_id: sample.child_id.clone(), target: sample.target.to_uri() }),
            CadMutation::DeleteShapeModel(DeleteShapeModel {}),
            CadMutation::CreateBuildingModel(CreateBuildingModel { child_id: sample.child_id.clone(), target: sample.target.to_uri() }),
            CadMutation::DeleteBuildingModel(DeleteBuildingModel {}),
            CadMutation::CreateEnergyModel(CreateEnergyModel { child_id: sample.child_id.clone(), target: sample.target.to_uri() }),
            CadMutation::DeleteEnergyModel(DeleteEnergyModel {}),
            CadMutation::CreateStructureClassicModel(CreateStructureClassicModel { child_id: sample.child_id.clone(), target: sample.target.to_uri() }),
            CadMutation::DeleteStructureClassicModel(DeleteStructureClassicModel {}),
            CadMutation::CreateDrawing(CreateDrawing { child_id: "drawing-fresh".into(), target: sample.target.to_uri() }),
            CadMutation::DeleteDrawing(DeleteDrawing { child_id: "drawing-fresh".into() }),
            CadMutation::CreateNode(CreateNode { node: crate::artifacts::cad::CadNode { id: "node-fresh".into(), label: "Root".into(), kind: "group".into() } }),
            CadMutation::DeleteNode(DeleteNode { node_id: "node-1".into() }),
            CadMutation::RenameNode(RenameNode { node_id: "node-1".into(), new_label: "Renamed".into() }),
            CadMutation::ChangeReferenceHidden(ChangeReferenceHidden { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), new_hidden: true }),
            CadMutation::ChangeReferenceLocked(ChangeReferenceLocked { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), new_locked: false }),
            CadMutation::ChangeReferenceWidth(ChangeReferenceWidth { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), new_width_world: 12.0 }),
            CadMutation::MoveReference(MoveReference { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), new_origin: [1.0, 1.0, 1.0] }),
            CadMutation::ReplaceReferenceMedia(ReplaceReferenceMedia { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), new_source_url: "https://example.test/other.png".into(), new_media_kind: "image".into(), new_orientation: None, new_scale: Some(2.0), new_opacity: Some(0.5) }),
            CadMutation::ReplaceReferences(ReplaceReferences { model_definition_id: "spatial.shape".into(), references: vec![sample_reference()] }),
            CadMutation::ChangeActiveModelDefinition(ChangeActiveModelDefinition { new_model_definition_id: "aec.building".into() }),
        ]
    }

    #[test]
    fn inverse_inverts_every_variant_against_a_populated_scene() {
        let base = sample_scene();
        for op in every_mutation() {
            let forward = protocol::MutationDiff::apply(&op.diff(&base), &base);
            let mut restored = forward.clone();
            for inverse in op.inverse(&base) {
                restored = protocol::MutationDiff::apply(&inverse.diff(&restored), &restored);
            }
            assert_eq!(restored, base, "inverse must restore the base scene for {op:?}");
        }
    }

    #[test]
    fn every_variant_registers_an_approved_semantic_descriptor() {
        for op in every_mutation() {
            let descriptor = protocol::SemanticMutation::semantics(&op);
            assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {op:?}", descriptor.verb);
        }
        assert_eq!(<CadMutation as protocol::SemanticMutation<CadSnapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
    }

    //#region 🧪️MutationLaws
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`
    /// (reachable here as `protocol::testkit`, the same `semio_framework_os_kernel` alias every
    /// other law/round-trip assertion in this crate already goes through), exercised against the
    /// three most structurally distinct new variants: a fixed-slot create/delete pair
    /// (`create-shape-model`), a Vec-collection create/delete pair (`create-drawing`), and a
    /// nested-address scalar setter (`change-reference-hidden`) carried over unchanged.
    #[test]
    fn create_shape_model_satisfies_the_inverse_and_absorb_laws() {
        let base = sample_scene();
        let sample = sample_model_child("law-model-1");
        let mutation = CadMutation::CreateShapeModel(CreateShapeModel { child_id: sample.child_id.clone(), target: sample.target.to_uri() });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = CadMutation::DeleteShapeModel(DeleteShapeModel {}).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn create_drawing_satisfies_the_inverse_and_absorb_laws() {
        let base = sample_scene();
        let sample = sample_model_child("law-drawing-1");
        let mutation = CadMutation::CreateDrawing(CreateDrawing { child_id: "drawing-law-1".into(), target: sample.target.to_uri() });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = CadMutation::DeleteDrawing(DeleteDrawing { child_id: "drawing-law-1".into() }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn change_reference_hidden_satisfies_the_inverse_and_absorb_laws() {
        let base = sample_scene();
        let mutation = CadMutation::ChangeReferenceHidden(ChangeReferenceHidden { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), new_hidden: true });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = CadMutation::ChangeReferenceLocked(ChangeReferenceLocked { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), new_locked: false }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🧪️MutationLaws
}
//#endregion 🧪️Tests
