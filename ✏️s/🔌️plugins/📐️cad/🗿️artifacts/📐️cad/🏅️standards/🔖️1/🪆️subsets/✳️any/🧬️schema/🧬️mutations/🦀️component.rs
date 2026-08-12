//! 🧬️ CAD artifact — document mutation dispatch enum + shared internal patch/helper types.
//! Every variant wraps exactly one `🧬️mutations/<kind>/🦠️mutation` payload struct implementing
//! `protocol::MutationKind<CadSnapshot, CadMutation>`; `#[derive(dsl::Mutations)]` below
//! generates `impl protocol::Mutation`/`impl protocol::SemanticMutation` by delegating to each
//! payload's own `diff`/`inverse` — see `🧪️MutationsDeriveLaws` in
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs` for the reference shape.

use crate::artifacts::cad::diff::{CadDiff, CadObjectPatchEntry, CadObjectsDelta};
use crate::artifacts::cad::{cad_pane_objects, CadObject, CadPaneId, CadSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️InternalPatches
/// 🩹️ Option-bag field delta for [`CadObject`] — INTERNAL diff-construction glue only (per
/// `📓️taxonomy.md`'s forbidden vocabulary, an option-bag `Patch` type may survive as a
/// diff-fragment helper, never as a mutation's own payload). Every `🧬️mutations/<kind>` leaf that
/// touches one or more `CadObject` scalar fields builds one of these to feed `CadObjectsDelta`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadObjectPatch {
    pub label: Option<String>,
    pub typology: Option<String>,
    pub visible: Option<bool>,
    pub locked: Option<bool>,
    pub origin: Option<[f64; 3]>,
    pub orientation: Option<[f64; 4]>,
    pub scale: Option<[f64; 3]>,
    #[serde(rename = "meshUrl")]
    pub mesh_url: Option<String>,
    pub extent: Option<[f64; 3]>,
    #[serde(rename = "solidHandle")]
    pub solid_handle: Option<String>,
}

/// 🩹️ Option-bag field delta for [`crate::artifacts::cad::CadNode`] — INTERNAL diff-construction glue only.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadNodePatch {
    pub label: Option<String>,
}

/// 🩹️ Option-bag field delta for [`crate::artifacts::cad::CadReference`] — INTERNAL diff-construction glue only.
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
    CreateObject(create_object::mutation::CreateObject),
    DeleteObject(delete_object::mutation::DeleteObject),
    RenameObject(rename_object::mutation::RenameObject),
    ChangeObjectTypology(change_object_typology::mutation::ChangeObjectTypology),
    ChangeObjectVisible(change_object_visible::mutation::ChangeObjectVisible),
    ChangeObjectLocked(change_object_locked::mutation::ChangeObjectLocked),
    MoveObject(move_object::mutation::MoveObject),
    RotateObject(rotate_object::mutation::RotateObject),
    ScaleObject(scale_object::mutation::ScaleObject),
    ReplaceObjectGeometry(replace_object_geometry::mutation::ReplaceObjectGeometry),
    DragObjects(drag_objects::mutation::DragObjects),
    RotateObjects(rotate_objects::mutation::RotateObjects),
    ScaleObjects(scale_objects::mutation::ScaleObjects),
    ReplacePaneObjects(replace_pane_objects::mutation::ReplacePaneObjects),
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

//#region 🔖️SharedHelpers
/// 🌉️ Assigns a fresh [`CadObjectsDelta`] onto the [`CadDiff`] field matching `pane` — shared by
/// every triad leaf that touches a pane's object collection.
pub fn set_pane_objects_delta(diff: &mut CadDiff, pane: CadPaneId, delta: CadObjectsDelta) {
    match pane {
        CadPaneId::Shape => diff.objects = Some(delta),
        CadPaneId::Building => diff.building_objects = Some(delta),
        CadPaneId::Energy => diff.energy_objects = Some(delta),
        CadPaneId::StructureClassic => diff.structure_classic_objects = Some(delta),
    }
}

/// 🌉️ Shared bulk-transform diff builder: applies `patch_for` to every object in `object_ids`,
/// scanning every pane, and assembles the resulting sparse [`CadDiff`] — used by
/// `drag-objects`/`rotate-objects`/`scale-objects`, each of which only differs in `patch_for`'s body.
pub fn transform_objects_diff(base: &CadSnapshot, object_ids: &[String], patch_for: impl Fn(&CadObject) -> CadObjectPatch) -> CadDiff {
    let mut diff = CadDiff::default();
    for pane in CadPaneId::all() {
        let mut patched = Vec::new();
        for object in cad_pane_objects(base, pane) {
            if !object_ids.contains(&object.id) {
                continue;
            }
            patched.push(CadObjectPatchEntry { id: object.id.clone(), patch: patch_for(object) });
        }
        if !patched.is_empty() {
            set_pane_objects_delta(&mut diff, pane, CadObjectsDelta { patched, ..Default::default() });
        }
    }
    diff
}

/// 🌉️ Quaternion Hamilton product `a * b` — shared by `rotate-object`/`rotate-objects` diff math.
pub fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]]
}

/// 🌉️ Axis-angle → unit quaternion — shared by `rotate-objects`' diff math.
pub fn quat_from_axis_angle(ax: f64, ay: f64, az: f64, angle: f64) -> [f64; 4] {
    let len = (ax * ax + ay * ay + az * az).sqrt();
    if len < 1e-8 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    let half = angle * 0.5;
    let s = half.sin();
    [ax / len * s, ay / len * s, az / len * s, half.cos()]
}
//#endregion 🔖️SharedHelpers

//#region 🔖️Leaves
use super::create_object;
use super::delete_object;
use super::rename_object;
use super::change_object_typology;
use super::change_object_visible;
use super::change_object_locked;
use super::move_object;
use super::rotate_object;
use super::scale_object;
use super::replace_object_geometry;
use super::drag_objects;
use super::rotate_objects;
use super::scale_objects;
use super::replace_pane_objects;
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
        change_active_model_definition::mutation::ChangeActiveModelDefinition, change_object_locked::mutation::ChangeObjectLocked, change_object_typology::mutation::ChangeObjectTypology,
        change_object_visible::mutation::ChangeObjectVisible, change_reference_hidden::mutation::ChangeReferenceHidden, change_reference_locked::mutation::ChangeReferenceLocked,
        change_reference_width::mutation::ChangeReferenceWidth, create_node::mutation::CreateNode, create_object::mutation::CreateObject, delete_node::mutation::DeleteNode, delete_object::mutation::DeleteObject,
        drag_objects::mutation::DragObjects, move_object::mutation::MoveObject, move_reference::mutation::MoveReference, rename_node::mutation::RenameNode, rename_object::mutation::RenameObject,
        replace_object_geometry::mutation::ReplaceObjectGeometry, replace_pane_objects::mutation::ReplacePaneObjects, replace_reference_media::mutation::ReplaceReferenceMedia,
        replace_references::mutation::ReplaceReferences, rotate_object::mutation::RotateObject, rotate_objects::mutation::RotateObjects, scale_object::mutation::ScaleObject, scale_objects::mutation::ScaleObjects,
    };
    use crate::artifacts::cad::testkit::{sample_object, sample_reference, sample_scene};
    use protocol::Mutation;

    /// ⚖️ One value per `CadMutation` variant — the closed set every wire law below iterates.
    pub fn every_mutation() -> Vec<CadMutation> {
        vec![
            CadMutation::CreateObject(CreateObject { pane: CadPaneId::Shape, object: sample_object("object-fresh") }),
            CadMutation::DeleteObject(DeleteObject { pane: CadPaneId::Shape, object_id: "object-1".into() }),
            CadMutation::RenameObject(RenameObject { pane: CadPaneId::Shape, object_id: "object-1".into(), new_label: "Renamed".into() }),
            CadMutation::ChangeObjectTypology(ChangeObjectTypology { pane: CadPaneId::Building, object_id: "object-2".into(), new_typology: "spatial.shape.sphere".into() }),
            CadMutation::ChangeObjectVisible(ChangeObjectVisible { pane: CadPaneId::Shape, object_id: "object-1".into(), new_visible: false }),
            CadMutation::ChangeObjectLocked(ChangeObjectLocked { pane: CadPaneId::Shape, object_id: "object-1".into(), new_locked: true }),
            CadMutation::MoveObject(MoveObject { pane: CadPaneId::Shape, object_id: "object-1".into(), new_origin: [5.0, 6.0, 7.0] }),
            CadMutation::RotateObject(RotateObject { pane: CadPaneId::Shape, object_id: "object-1".into(), new_orientation: [0.0, 0.0, 0.707, 0.707] }),
            CadMutation::ScaleObject(ScaleObject { pane: CadPaneId::Shape, object_id: "object-1".into(), new_scale: [2.0, 2.0, 2.0] }),
            CadMutation::ReplaceObjectGeometry(ReplaceObjectGeometry { pane: CadPaneId::Shape, object_id: "object-1".into(), new_extent: Some([3.0, 3.0, 3.0]), new_mesh_url: Some("https://example.test/other.glb".into()), new_solid_handle: Some("solid-2".into()) }),
            CadMutation::DragObjects(DragObjects { object_ids: vec!["object-1".into(), "object-2".into()], dx: 1.0, dy: -1.0, dz: 0.5 }),
            CadMutation::RotateObjects(RotateObjects { object_ids: vec!["object-1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.57 }),
            CadMutation::ScaleObjects(ScaleObjects { object_ids: vec!["object-1".into()], sx: 2.0, sy: 2.0, sz: 2.0 }),
            CadMutation::ReplacePaneObjects(ReplacePaneObjects { pane: CadPaneId::Energy, objects: vec![sample_object("object-1"), sample_object("object-2")] }),
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
    /// three most structurally distinct new variants: an id-keyed create/delete pair
    /// (`rename-object`), a bulk relative-offset gesture (`drag-objects`), and a nested-address
    /// scalar setter (`change-reference-hidden`).
    #[test]
    fn rename_object_satisfies_the_inverse_and_absorb_laws() {
        let base = sample_scene();
        let mutation = CadMutation::RenameObject(RenameObject { pane: CadPaneId::Shape, object_id: "object-1".into(), new_label: "Renamed".into() });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = CadMutation::ChangeObjectTypology(ChangeObjectTypology { pane: CadPaneId::Shape, object_id: "object-1".into(), new_typology: "spatial.shape.sphere".into() }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn drag_objects_satisfies_the_inverse_and_absorb_laws() {
        let base = sample_scene();
        let mutation = CadMutation::DragObjects(DragObjects { object_ids: vec!["object-1".into(), "object-2".into()], dx: 1.0, dy: -2.0, dz: 0.5 });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = CadMutation::DragObjects(DragObjects { object_ids: vec!["object-1".into()], dx: 0.1, dy: 0.2, dz: 0.3 }).diff(&base);
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
