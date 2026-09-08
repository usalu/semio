
use super::*;
use crate::mutations::{
    change_active_model_definition::ChangeActiveModelDefinition, change_reference_hidden::ChangeReferenceHidden, change_reference_locked::ChangeReferenceLocked, change_reference_width::ChangeReferenceWidth,
    create_building_model::CreateBuildingModel, create_drawing::CreateDrawing, create_energy_model::CreateEnergyModel, create_node::CreateNode, create_shape_model::CreateShapeModel, create_structure_classic_model::CreateStructureClassicModel,
    delete_building_model::DeleteBuildingModel, delete_drawing::DeleteDrawing, delete_energy_model::DeleteEnergyModel, delete_node::DeleteNode, delete_shape_model::DeleteShapeModel, delete_structure_classic_model::DeleteStructureClassicModel,
    move_reference::MoveReference, rename_node::RenameNode, replace_reference_media::ReplaceReferenceMedia, replace_references::ReplaceReferences,
};
use crate::testkit::{sample_model_child, sample_reference, sample_scene};
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
        CadMutation::CreateNode(CreateNode { node: crate::CadNode { id: "node-fresh".into(), label: "Root".into(), kind: "group".into() } }),
        CadMutation::DeleteNode(DeleteNode { node_id: "node-1".into() }),
        CadMutation::RenameNode(RenameNode { node_id: "node-1".into(), new_label: "Renamed".into() }),
        CadMutation::ChangeReferenceHidden(ChangeReferenceHidden { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), new_hidden: true }),
        CadMutation::ChangeReferenceLocked(ChangeReferenceLocked { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), new_locked: false }),
        CadMutation::ChangeReferenceWidth(ChangeReferenceWidth { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), new_width_world: 12.0 }),
        CadMutation::MoveReference(MoveReference { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), new_origin: [1.0, 1.0, 1.0] }),
        CadMutation::ReplaceReferenceMedia(ReplaceReferenceMedia {
            model_definition_id: "spatial.shape".into(),
            reference_id: "ref-1".into(),
            new_source_url: "https://example.test/other.png".into(),
            new_media_kind: "image".into(),
            new_orientation: None,
            new_scale: Some(2.0),
            new_opacity: Some(0.5),
        }),
        CadMutation::ReplaceReferences(ReplaceReferences { model_definition_id: "spatial.shape".into(), references: vec![sample_reference()] }),
        CadMutation::ChangeActiveModelDefinition(ChangeActiveModelDefinition { new_model_definition_id: "aec.building".into() }),
    ]
}

#[semio_framework_async_macros::async_test]
async fn inverse_inverts_every_variant_against_a_populated_scene() {
    let base = sample_scene();
    for op in every_mutation() {
        let forward = protocol::MutationDiff::apply(op.diff(&base).diff(), &base).expect("valid mutation diff");
        let mut restored = forward.clone();
        for inverse in op.inverse(&base) {
            restored = protocol::MutationDiff::apply(inverse.diff(&restored).diff(), &restored).expect("valid inverse mutation diff");
        }
        assert_eq!(restored, base, "inverse must restore the base scene for {op:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn every_variant_registers_an_approved_semantic_descriptor() {
    for op in every_mutation() {
        let descriptor = protocol::SemanticMutation::semantics(&op);
        assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {op:?}", descriptor.verb);
    }
    assert_eq!(<CadMutation as protocol::SemanticMutation<CadSnapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
}

//#region 🧪️MutationLaws
/// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️test/🦀️kit.rs`
/// (reachable here as `store::os_spr::testkit`, the same `semio_framework_os_kernel` alias every
/// other law/round-trip assertion in this crate already goes through), exercised against the
/// three most structurally distinct new variants: a fixed-slot create/delete pair
/// (`create-shape-model`), a Vec-collection create/delete pair (`create-drawing`), and a
/// nested-address scalar setter (`change-reference-hidden`) carried over unchanged.
#[semio_framework_async_macros::async_test]
async fn create_shape_model_satisfies_the_inverse_and_absorb_laws() {
    let base = sample_scene();
    let sample = sample_model_child("law-model-1");
    let mutation = CadMutation::CreateShapeModel(CreateShapeModel { child_id: sample.child_id.clone(), target: sample.target.to_uri() });
    store::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = CadMutation::DeleteShapeModel(DeleteShapeModel {}).diff(&base).diff().clone();
    store::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn create_drawing_satisfies_the_inverse_and_absorb_laws() {
    let base = sample_scene();
    let sample = sample_model_child("law-drawing-1");
    let mutation = CadMutation::CreateDrawing(CreateDrawing { child_id: "drawing-law-1".into(), target: sample.target.to_uri() });
    store::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = CadMutation::DeleteDrawing(DeleteDrawing { child_id: "drawing-law-1".into() }).diff(&base).diff().clone();
    store::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn change_reference_hidden_satisfies_the_inverse_and_absorb_laws() {
    let base = sample_scene();
    let mutation = CadMutation::ChangeReferenceHidden(ChangeReferenceHidden { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), new_hidden: true });
    store::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).diff().clone();
    let d2 = CadMutation::ChangeReferenceLocked(ChangeReferenceLocked { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), new_locked: false }).diff(&base).diff().clone();
    store::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
//#endregion 🧪️MutationLaws

//#region 🧪️OutcomeLaws
/// ⚖️ `📋️contract-freeze.md` §C2 laws, per verb family: `assert_missing_target_is_error`/
/// `assert_fatal_never_applies` below, `assert_outcome_policy_matrix` cases further down (delete
/// node/drawing, rename, change, create node/drawing).
#[semio_framework_async_macros::async_test]
async fn delete_missing_node_is_a_target_missing_error() {
    let base = sample_scene();
    store::os_spr::testkit::assert_missing_target_is_error(&base, &CadMutation::DeleteNode(DeleteNode { node_id: "does-not-exist".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_missing_node_is_a_target_missing_error() {
    let base = sample_scene();
    store::os_spr::testkit::assert_missing_target_is_error(&base, &CadMutation::RenameNode(RenameNode { node_id: "does-not-exist".into(), new_label: "New".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn change_hidden_on_missing_reference_is_a_target_missing_error() {
    let base = sample_scene();
    store::os_spr::testkit::assert_missing_target_is_error(&base, &CadMutation::ChangeReferenceHidden(ChangeReferenceHidden { model_definition_id: "spatial.shape".into(), reference_id: "does-not-exist".into(), new_hidden: true })).await;
}

#[semio_framework_async_macros::async_test]
async fn create_node_duplicate_id_never_applies() {
    let base = sample_scene();
    let duplicate = CadMutation::CreateNode(CreateNode { node: crate::CadNode { id: "node-1".into(), label: "Dup".into(), kind: "group".into() } });
    store::os_spr::testkit::assert_fatal_never_applies(&duplicate.diff(&base)).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_missing_drawing_is_a_target_missing_error() {
    let base = sample_scene();
    store::os_spr::testkit::assert_missing_target_is_error(&base, &CadMutation::DeleteDrawing(DeleteDrawing { child_id: "does-not-exist".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn create_drawing_duplicate_id_never_applies() {
    let sample = sample_model_child("dup-drawing-1");
    let mut base = sample_scene();
    base = protocol::MutationDiff::apply(CadMutation::CreateDrawing(CreateDrawing { child_id: "drawing-dup".into(), target: sample.target.to_uri() }).diff(&base).diff(), &base).expect("valid mutation diff");
    let duplicate = CadMutation::CreateDrawing(CreateDrawing { child_id: "drawing-dup".into(), target: sample.target.to_uri() });
    store::os_spr::testkit::assert_fatal_never_applies(&duplicate.diff(&base)).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_node_outcome_obeys_the_policy_matrix() {
    let base = sample_scene();
    store::os_spr::testkit::assert_outcome_policy_matrix(&base, &CadMutation::DeleteNode(DeleteNode { node_id: "node-1".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_node_outcome_obeys_the_policy_matrix() {
    let base = sample_scene();
    store::os_spr::testkit::assert_outcome_policy_matrix(&base, &CadMutation::RenameNode(RenameNode { node_id: "node-1".into(), new_label: "Renamed".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn change_reference_hidden_outcome_obeys_the_policy_matrix() {
    let base = sample_scene();
    store::os_spr::testkit::assert_outcome_policy_matrix(&base, &CadMutation::ChangeReferenceHidden(ChangeReferenceHidden { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), new_hidden: true })).await;
}

#[semio_framework_async_macros::async_test]
async fn create_node_outcome_obeys_the_policy_matrix() {
    let base = sample_scene();
    store::os_spr::testkit::assert_outcome_policy_matrix(&base, &CadMutation::CreateNode(CreateNode { node: crate::CadNode { id: "node-fresh".into(), label: "Root".into(), kind: "group".into() } })).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_drawing_outcome_obeys_the_policy_matrix() {
    let sample = sample_model_child("law-drawing-2");
    let mut base = sample_scene();
    base = protocol::MutationDiff::apply(CadMutation::CreateDrawing(CreateDrawing { child_id: "drawing-2".into(), target: sample.target.to_uri() }).diff(&base).diff(), &base).expect("valid mutation diff");
    store::os_spr::testkit::assert_outcome_policy_matrix(&base, &CadMutation::DeleteDrawing(DeleteDrawing { child_id: "drawing-2".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn create_drawing_outcome_obeys_the_policy_matrix() {
    let base = sample_scene();
    let sample = sample_model_child("law-drawing-3");
    store::os_spr::testkit::assert_outcome_policy_matrix(&base, &CadMutation::CreateDrawing(CreateDrawing { child_id: "drawing-3".into(), target: sample.target.to_uri() })).await;
}
//#endregion 🧪️OutcomeLaws
//#region 🧪️KindsCatalog
/// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
/// `#[derive(dsl::Mutations)]` assigns, and every entry must also appear in the committed oracle
/// manifest's catalog — the framework never parses Rust, so this is the only thing that keeps
/// the declared vocabulary and the measured one from drifting apart.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = <CadMutation as protocol::SemanticMutation<CadSnapshot>>::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared CadMutation variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🧪️KindsCatalog
