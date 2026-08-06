//! 🔧️ CAD artifact — the `CadOperation` enum: every document-mutating edit a cad scene accepts, its
//! patch payloads, and the `Operation` laws (`diff`/`backwards`) that give each one a true inverse.
//! The materialized diff shape lives beside this in `🔺️diff/🦀️component.rs`.

use crate::artifacts::cad::diff::{apply_reference_patch, CadDiff};
use crate::artifacts::cad::{cad_pane_objects, CadNode, CadObject, CadPaneId, CadReference, CadProjection};
use protocol::{CollectionDiff, ItemPatch, Operation};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Operations
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadNodePatch {
    pub label: Option<String>,
}

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum CadOperation {
    AddObject {
        pane: CadPaneId,
        #[dsl(block)]
        object: CadObject,
    },
    RemoveObject {
        pane: CadPaneId,
        object_id: String,
    },
    PatchObject {
        pane: CadPaneId,
        object_id: String,
        #[dsl(block)]
        patch: CadObjectPatch,
    },
    TranslateObjects {
        object_ids: Vec<String>,
        dx: f64,
        dy: f64,
        dz: f64,
    },
    RotateObjects {
        object_ids: Vec<String>,
        ax: f64,
        ay: f64,
        az: f64,
        angle: f64,
    },
    ScaleObjects {
        object_ids: Vec<String>,
        sx: f64,
        sy: f64,
        sz: f64,
    },
    SetPaneObjects {
        pane: CadPaneId,
        objects: Vec<CadObject>,
    },
    AddNode {
        #[dsl(block)]
        node: CadNode,
    },
    RemoveNode {
        node_id: String,
    },
    RenameNode {
        node_id: String,
        label: String,
    },
    PatchReference {
        model_definition_id: String,
        reference_id: String,
        #[dsl(block)]
        patch: CadReferencePatch,
    },
    SetReferences {
        model_definition_id: String,
        references: Vec<CadReference>,
    },
    SetActiveModelDefinition {
        model_definition_id: String,
    },
    SetScene {
        #[dsl(block)]
        scene: Box<CadProjection>,
    },
}
fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]]
}

fn quat_from_axis_angle(ax: f64, ay: f64, az: f64, angle: f64) -> [f64; 4] {
    let len = (ax * ax + ay * ay + az * az).sqrt();
    if len < 1e-8 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    let half = angle * 0.5;
    let s = half.sin();
    [ax / len * s, ay / len * s, az / len * s, half.cos()]
}
impl Operation<CadProjection> for CadOperation {
    type Diff = CadDiff;

    fn diff(&self, projection: &CadProjection) -> CadDiff {
        match self {
            CadOperation::AddObject { pane, object } => CadDiff {
                objects: pane_collection_diff_for_add(*pane, object),
                building_objects: pane_collection_diff_for_add_if(*pane, CadPaneId::Building, object),
                energy_objects: pane_collection_diff_for_add_if(*pane, CadPaneId::Energy, object),
                structure_classic_objects: pane_collection_diff_for_add_if(*pane, CadPaneId::StructureClassic, object),
                ..Default::default()
            },
            CadOperation::RemoveObject { pane, object_id } => CadDiff {
                objects: pane_collection_diff_for_remove(*pane, CadPaneId::Shape, object_id),
                building_objects: pane_collection_diff_for_remove(*pane, CadPaneId::Building, object_id),
                energy_objects: pane_collection_diff_for_remove(*pane, CadPaneId::Energy, object_id),
                structure_classic_objects: pane_collection_diff_for_remove(*pane, CadPaneId::StructureClassic, object_id),
                ..Default::default()
            },
            CadOperation::PatchObject { pane, object_id, patch } => CadDiff {
                objects: pane_collection_diff_for_patch(*pane, CadPaneId::Shape, object_id, patch),
                building_objects: pane_collection_diff_for_patch(*pane, CadPaneId::Building, object_id, patch),
                energy_objects: pane_collection_diff_for_patch(*pane, CadPaneId::Energy, object_id, patch),
                structure_classic_objects: pane_collection_diff_for_patch(*pane, CadPaneId::StructureClassic, object_id, patch),
                ..Default::default()
            },
            CadOperation::TranslateObjects { object_ids, dx, dy, dz } => {
                transform_objects_diff(projection, object_ids, |object| CadObjectPatch { origin: Some([object.origin[0] + dx, object.origin[1] + dy, object.origin[2] + dz]), ..Default::default() })
            }
            CadOperation::RotateObjects { object_ids, ax, ay, az, angle } => {
                let delta = quat_from_axis_angle(*ax, *ay, *az, *angle);
                transform_objects_diff(projection, object_ids, |object| {
                    let current = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                    CadObjectPatch { orientation: Some(quat_mul(delta, current)), ..Default::default() }
                })
            }
            CadOperation::ScaleObjects { object_ids, sx, sy, sz } => transform_objects_diff(projection, object_ids, |object| {
                let current = object.scale.unwrap_or([1.0, 1.0, 1.0]);
                CadObjectPatch { scale: Some([current[0] * sx, current[1] * sy, current[2] * sz]), ..Default::default() }
            }),
            CadOperation::SetPaneObjects { pane, objects } => {
                let mut diff = CadDiff::default();
                let removed: Vec<String> = cad_pane_objects(projection, *pane).iter().map(|object| object.id.clone()).collect();
                let collection = CollectionDiff { removed, modified: Vec::new(), added: objects.clone() };
                set_pane_collection_diff(&mut diff, *pane, collection);
                diff
            }
            CadOperation::AddNode { node } => CadDiff { nodes: Some(CollectionDiff { added: vec![node.clone()], ..Default::default() }), ..Default::default() },
            CadOperation::RemoveNode { node_id } => CadDiff { nodes: Some(CollectionDiff { removed: vec![node_id.clone()], ..Default::default() }), ..Default::default() },
            CadOperation::RenameNode { node_id, label } => CadDiff { nodes: Some(CollectionDiff { modified: vec![ItemPatch { id: node_id.clone(), patch: CadNodePatch { label: Some(label.clone()) } }], ..Default::default() }), ..Default::default() },
            CadOperation::PatchReference { model_definition_id, reference_id, patch } => {
                let references = projection.references_by_model_definition_id.get(model_definition_id).cloned().unwrap_or_default();
                let next = references
                    .into_iter()
                    .map(|mut reference| {
                        if reference.id == *reference_id {
                            apply_reference_patch(&mut reference, patch);
                        }
                        reference
                    })
                    .collect();
                CadDiff { references_by_model_definition_id: Some(BTreeMap::from([(model_definition_id.clone(), next)])), ..Default::default() }
            }
            CadOperation::SetReferences { model_definition_id, references } => CadDiff { references_by_model_definition_id: Some(BTreeMap::from([(model_definition_id.clone(), references.clone())])), ..Default::default() },
            CadOperation::SetActiveModelDefinition { model_definition_id } => CadDiff { active_model_definition_id: Some(model_definition_id.clone()), ..Default::default() },
            CadOperation::SetScene { scene } => CadDiff { scene: Some(scene.clone()), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &CadProjection) -> Vec<Self> {
        match self {
            CadOperation::AddObject { pane, object } => vec![CadOperation::RemoveObject { pane: *pane, object_id: object.id.clone() }],
            CadOperation::RemoveObject { pane, object_id } => cad_pane_objects(projection, *pane).iter().find(|object| object.id == *object_id).map(|object| vec![CadOperation::AddObject { pane: *pane, object: object.clone() }]).unwrap_or_default(),
            CadOperation::PatchObject { pane, object_id, patch } => cad_pane_objects(projection, *pane)
                .iter()
                .find(|object| object.id == *object_id)
                .map(|before| vec![CadOperation::PatchObject { pane: *pane, object_id: object_id.clone(), patch: reverse_object_patch(before, patch) }])
                .unwrap_or_default(),
            CadOperation::TranslateObjects { object_ids, dx, dy, dz } => vec![CadOperation::TranslateObjects { object_ids: object_ids.clone(), dx: -dx, dy: -dy, dz: -dz }],
            CadOperation::RotateObjects { object_ids, ax, ay, az, angle } => vec![CadOperation::RotateObjects { object_ids: object_ids.clone(), ax: *ax, ay: *ay, az: *az, angle: -angle }],
            CadOperation::ScaleObjects { object_ids, sx, sy, sz } => {
                let inv = |value: f64| if value.abs() < 1e-8 { 1.0 } else { 1.0 / value };
                vec![CadOperation::ScaleObjects { object_ids: object_ids.clone(), sx: inv(*sx), sy: inv(*sy), sz: inv(*sz) }]
            }
            CadOperation::SetPaneObjects { pane, objects: _ } => {
                let before = cad_pane_objects(projection, *pane).to_vec();
                vec![CadOperation::SetPaneObjects { pane: *pane, objects: before }]
            }
            CadOperation::AddNode { node } => vec![CadOperation::RemoveNode { node_id: node.id.clone() }],
            CadOperation::RemoveNode { node_id } => projection.nodes.iter().find(|node| node.id == *node_id).map(|node| vec![CadOperation::AddNode { node: node.clone() }]).unwrap_or_default(),
            CadOperation::RenameNode { node_id, .. } => projection.nodes.iter().find(|node| node.id == *node_id).map(|node| vec![CadOperation::RenameNode { node_id: node_id.clone(), label: node.label.clone() }]).unwrap_or_default(),
            CadOperation::PatchReference { model_definition_id, reference_id, patch } => projection
                .references_by_model_definition_id
                .get(model_definition_id)
                .and_then(|references| {
                    references
                        .iter()
                        .find(|reference| reference.id == *reference_id)
                        .map(|before| vec![CadOperation::PatchReference { model_definition_id: model_definition_id.clone(), reference_id: reference_id.clone(), patch: reverse_reference_patch(before, patch) }])
                })
                .unwrap_or_default(),
            CadOperation::SetReferences { model_definition_id, .. } => {
                let before = projection.references_by_model_definition_id.get(model_definition_id).cloned().unwrap_or_default();
                vec![CadOperation::SetReferences { model_definition_id: model_definition_id.clone(), references: before }]
            }
            CadOperation::SetActiveModelDefinition { .. } => vec![CadOperation::SetActiveModelDefinition { model_definition_id: projection.active_model_definition_id.clone() }],
            CadOperation::SetScene { .. } => vec![CadOperation::SetScene { scene: Box::new(projection.clone()) }],
        }
    }
}

fn reverse_object_patch(before: &CadObject, patch: &CadObjectPatch) -> CadObjectPatch {
    CadObjectPatch {
        label: patch.label.as_ref().map(|_| before.label.clone()),
        typology: patch.typology.as_ref().map(|_| before.typology.clone()),
        visible: patch.visible.map(|_| before.visible),
        locked: patch.locked.map(|_| before.locked),
        origin: patch.origin.map(|_| before.origin),
        orientation: patch.orientation.map(|_| before.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])),
        scale: patch.scale.map(|_| before.scale.unwrap_or([1.0, 1.0, 1.0])),
        mesh_url: patch.mesh_url.as_ref().map(|_| before.mesh_url.clone().unwrap_or_default()),
        extent: patch.extent.and(before.extent),
        solid_handle: patch.solid_handle.as_ref().and_then(|_| before.solid_handle.clone()),
    }
}

fn reverse_reference_patch(before: &CadReference, patch: &CadReferencePatch) -> CadReferencePatch {
    CadReferencePatch {
        source_url: patch.source_url.as_ref().map(|_| before.source_url.clone()),
        media_kind: patch.media_kind.as_ref().map(|_| before.media_kind.clone()),
        origin: patch.origin.map(|_| before.origin),
        orientation: patch.orientation.map(|_| before.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])),
        scale: patch.scale.map(|_| before.scale.unwrap_or(1.0)),
        width_world: patch.width_world.map(|_| before.width_world),
        hidden: patch.hidden.map(|_| before.hidden),
        locked: patch.locked.map(|_| before.locked),
        opacity: patch.opacity.and(before.opacity),
    }
}

fn pane_collection_diff_for_add(pane: CadPaneId, object: &CadObject) -> Option<CollectionDiff<String, CadObjectPatch, CadObject>> {
    pane_collection_diff_for_add_if(pane, CadPaneId::Shape, object)
}

fn pane_collection_diff_for_add_if(pane: CadPaneId, target: CadPaneId, object: &CadObject) -> Option<CollectionDiff<String, CadObjectPatch, CadObject>> {
    if pane == target {
        Some(CollectionDiff { added: vec![object.clone()], ..Default::default() })
    } else {
        None
    }
}

fn pane_collection_diff_for_remove(pane: CadPaneId, target: CadPaneId, object_id: &str) -> Option<CollectionDiff<String, CadObjectPatch, CadObject>> {
    if pane == target {
        Some(CollectionDiff { removed: vec![object_id.into()], ..Default::default() })
    } else {
        None
    }
}

fn pane_collection_diff_for_patch(pane: CadPaneId, target: CadPaneId, object_id: &str, patch: &CadObjectPatch) -> Option<CollectionDiff<String, CadObjectPatch, CadObject>> {
    if pane == target {
        Some(CollectionDiff { modified: vec![ItemPatch { id: object_id.into(), patch: patch.clone() }], ..Default::default() })
    } else {
        None
    }
}

fn set_pane_collection_diff(diff: &mut CadDiff, pane: CadPaneId, collection: CollectionDiff<String, CadObjectPatch, CadObject>) {
    match pane {
        CadPaneId::Shape => diff.objects = Some(collection),
        CadPaneId::Building => diff.building_objects = Some(collection),
        CadPaneId::Energy => diff.energy_objects = Some(collection),
        CadPaneId::StructureClassic => diff.structure_classic_objects = Some(collection),
    }
}

fn transform_objects_diff(projection: &CadProjection, object_ids: &[String], patch_for: impl Fn(&CadObject) -> CadObjectPatch) -> CadDiff {
    let mut diff = CadDiff::default();
    for pane in CadPaneId::all() {
        let mut modified = Vec::new();
        for object in cad_pane_objects(projection, pane) {
            if !object_ids.contains(&object.id) {
                continue;
            }
            modified.push(ItemPatch { id: object.id.clone(), patch: patch_for(object) });
        }
        if !modified.is_empty() {
            set_pane_collection_diff(&mut diff, pane, CollectionDiff { modified, ..Default::default() });
        }
    }
    diff
}
//#endregion 🔖️Operations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::cad::testkit::{sample_object, sample_reference, sample_scene};

    /// ⚖️ One value per `CadOperation` variant — the closed set every wire law below iterates.
    pub(crate) fn every_operation() -> Vec<CadOperation> {
        vec![
            CadOperation::AddObject { pane: CadPaneId::Shape, object: sample_object("object-1") },
            CadOperation::RemoveObject { pane: CadPaneId::Shape, object_id: "object-1".into() },
            CadOperation::PatchObject { pane: CadPaneId::Building, object_id: "object-1".into(), patch: CadObjectPatch { label: Some("Renamed".into()), visible: Some(false), ..Default::default() } },
            CadOperation::TranslateObjects { object_ids: vec!["object-1".into(), "object-2".into()], dx: 1.0, dy: -1.0, dz: 0.5 },
            CadOperation::RotateObjects { object_ids: vec!["object-1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.57 },
            CadOperation::ScaleObjects { object_ids: vec!["object-1".into()], sx: 2.0, sy: 2.0, sz: 2.0 },
            CadOperation::SetPaneObjects { pane: CadPaneId::Energy, objects: vec![sample_object("object-1"), sample_object("object-2")] },
            CadOperation::AddNode { node: CadNode { id: "node-1".into(), label: "Root".into(), kind: "group".into() } },
            CadOperation::RemoveNode { node_id: "node-1".into() },
            CadOperation::RenameNode { node_id: "node-1".into(), label: "Renamed".into() },
            CadOperation::PatchReference { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), patch: CadReferencePatch { hidden: Some(true), ..Default::default() } },
            CadOperation::SetReferences { model_definition_id: "spatial.shape".into(), references: vec![sample_reference()] },
            CadOperation::SetActiveModelDefinition { model_definition_id: "aec.building".into() },
            CadOperation::SetScene { scene: Box::new(sample_scene()) },
        ]
    }

    #[test]
    fn cad_operation_print_op_round_trips_every_variant_as_one_line() {
        for op in every_operation() {
            store::test_support::assert_op_line_round_trip(&op);
            store::test_support::assert_op_text_binary_equivalence(&op);
        }
    }

    /// 🔒️ Wire-format pin: the exact bytes of the rows whose `Option` fields make `None`/`Some`
    /// distinct wire cases, captured from the pre-consolidation crates
    /// (ticket `26/08/05/CAD-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`, wire baseline).
    #[test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        let hex = |op: &CadOperation| -> String { protocol::OpBinary::encode_op(op).expect("encode").iter().map(|byte| format!("{byte:02x}")).collect() };
        assert_eq!(hex(&CadOperation::RemoveObject { pane: CadPaneId::Shape, object_id: "object-1".into() }), "010101086f626a6563742d3102000a00010600");
        assert_eq!(
            hex(&CadOperation::PatchObject { pane: CadPaneId::Building, object_id: "object-1".into(), patch: CadObjectPatch { label: Some("Renamed".into()), visible: Some(false), ..Default::default() } }),
            "0102020752656e616d6564086f626a6563742d3103000a01010601020e0d020006000201"
        );
        assert_eq!(hex(&CadOperation::PatchObject { pane: CadPaneId::Building, object_id: "object-1".into(), patch: CadObjectPatch::default() }), "010201086f626a6563742d3103000a01010600020e0d00");
        assert_eq!(
            hex(&CadOperation::PatchReference { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), patch: CadReferencePatch { hidden: Some(true), ..Default::default() } }),
            "010a02057265662d310d7370617469616c2e736861706503000601010600020e0d010602"
        );
        assert_eq!(hex(&CadOperation::PatchReference { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), patch: CadReferencePatch::default() }), "010a02057265662d310d7370617469616c2e736861706503000601010600020e0d00");
        assert_eq!(hex(&CadOperation::SetActiveModelDefinition { model_definition_id: "aec.building".into() }), "010c010c6165632e6275696c64696e6701000600");
    }

    #[test]
    fn backwards_inverts_every_variant_against_a_populated_scene() {
        let base = sample_scene();
        // ➕️ `AddObject`/`AddNode` are only invertible for ids the base scene does NOT already carry —
        // `every_operation`'s rows deliberately reuse the sample ids to pin the wire format, so the
        // additive rows get fresh ids here.
        let operations = every_operation().into_iter().map(|op| match op {
            CadOperation::AddObject { pane, mut object } => {
                object.id = "object-fresh".into();
                CadOperation::AddObject { pane, object }
            }
            CadOperation::AddNode { mut node } => {
                node.id = "node-fresh".into();
                CadOperation::AddNode { node }
            }
            other => other,
        });
        for op in operations {
            let forward = protocol::OperationDiff::apply(&op.diff(&base), &base);
            let mut restored = forward.clone();
            for inverse in op.backwards(&base) {
                restored = protocol::OperationDiff::apply(&inverse.diff(&restored), &restored);
            }
            assert_eq!(restored, base, "backwards must restore the base scene for {op:?}");
        }
    }
}
//#endregion 🧪️Tests
