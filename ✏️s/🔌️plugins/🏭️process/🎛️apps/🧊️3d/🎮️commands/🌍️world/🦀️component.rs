//! 🌍️ Process 3d play app commands — 3D viewport interactions: click-to-place, push/pull face drag,
//! and face picking.

use crate::apps::process3d::config::{Process3dConfig, Process3dConfigOperation};
use crate::apps::process3d::terminology::{process3d_labels, Process3dLabels};
use crate::apps::process3d::set_active_utility_effect;
use crate::artifacts::process3d::engine::{axis_angle_from_up_to, capability_for_measure_kind, insert_step_operations, next_step_id};
use crate::artifacts::process3d::{op::Process3dOperation, MeasureKind, Pose, Process3dDocument, ProcessMeasure, ProcessStep, SolidSpec, StepOrigin};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️FaceDrag
/// 🖱️➡️ Builds a push/pull step from a face-drag gesture: dragging into the solid (negative `distance`
/// along the face's outward `normal`) removes material (Cut); dragging outward (positive) adds material
/// (Attach). The tool box's local origin corner lands at `point + normal * distance.min(0.0)` so it spans
/// exactly the dragged region, flush with the picked face — `box_prim_sync` places a primitive's corner
/// (not its center) at the local origin, confirmed by `box_primitive_spans_from_local_origin_corner` in
/// the artifact's `⚙️engine`.
fn process3d_step_from_face_drag(normal: [f64; 3], point: [f64; 3], distance: f64, face_extent: Option<[f64; 2]>, labels: &Process3dLabels) -> Option<ProcessStep> {
    if distance.abs() < 1e-6 {
        return None;
    }
    let (width, depth) = face_extent.map(|[w, d]| (w.max(0.02), d.max(0.02))).unwrap_or((0.2, 0.2));
    let height = distance.abs();
    let (axis, angle) = axis_angle_from_up_to(normal);
    let offset = distance.min(0.0);
    let position = [point[0] + normal[0] * offset, point[1] + normal[1] * offset, point[2] + normal[2] * offset];
    let pose = Pose { position, axis, angle };
    let (measure, label, machine_id, capability_id) = if distance < 0.0 {
        (ProcessMeasure::Cut { tool: SolidSpec::Box { width, depth, height }, pose }, labels.push_cut, "saw", "cut")
    } else {
        (ProcessMeasure::Attach { component: SolidSpec::Box { width, depth, height }, pose }, labels.pull_attach, "attacher", "attach")
    };
    let origin = StepOrigin { machine_id: machine_id.to_string(), capability_id: capability_id.to_string() };
    Some(ProcessStep { id: next_step_id(), label: label.as_str().to_string(), enabled: true, origin: Some(origin), measure })
}
//#endregion 🔖️FaceDrag

//#region 🔖️WorldPointerDown
pub mod world_pointer_down {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-pointer-down")]
    pub struct WorldPointerDown {
        #[dsl(coord)]
        pub position: [f64; 3],
    }

    pub fn handle(payload: &WorldPointerDown, doc: &DocumentView<'_, Process3dDocument>, cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        let fixture = doc.projection;
        let config = cfg.projection;
        let utility = config.active_utility();
        if utility == "select" {
            return Ok(Emit::default());
        }
        let measure_kind = match utility {
            "drill" => MeasureKind::Drill,
            "attach" => MeasureKind::Attach,
            _ => MeasureKind::Cut,
        };
        let (machine, capability) = capability_for_measure_kind(&fixture.workshop, measure_kind);
        let origin = StepOrigin { machine_id: machine.id.clone(), capability_id: capability.id.clone() };
        let step = ProcessStep {
            id: next_step_id(),
            label: capability.label.clone(),
            enabled: true,
            origin: Some(origin),
            measure: crate::artifacts::process3d::engine::measure_for_capability(&capability, Some(payload.position)),
        };
        let step_id = step.id.clone();
        Ok(Emit { document_operations: insert_step_operations(fixture, step), config_operations: vec![Process3dConfigOperation::SetSelectedId { value: Some(step_id) }], effects: vec![set_active_utility_effect("select")], ..Default::default() })
    }
}
//#endregion 🔖️WorldPointerDown

//#region 🔖️WorldFaceDragEnd
pub mod world_face_drag_end {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-face-drag-end")]
    pub struct WorldFaceDragEnd {
        #[dsl(coord)]
        pub normal: [f64; 3],
        #[dsl(coord)]
        pub start_point: [f64; 3],
        pub distance: f64,
        pub face_extent: Option<[f64; 2]>,
    }

    pub fn handle(payload: &WorldFaceDragEnd, doc: &DocumentView<'_, Process3dDocument>, cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        let fixture = doc.projection;
        let config = cfg.projection;
        if config.active_utility() != "select" {
            return Ok(Emit::default());
        }
        match process3d_step_from_face_drag(payload.normal, payload.start_point, payload.distance, payload.face_extent, process3d_labels(config)) {
            Some(step) => {
                let step_id = step.id.clone();
                Ok(Emit {
                    document_operations: insert_step_operations(fixture, step),
                    config_operations: vec![Process3dConfigOperation::SetSelectedId { value: Some(step_id) }, Process3dConfigOperation::SetSelectedFaceId { value: None }],
                    ..Default::default()
                })
            }
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️WorldFaceDragEnd

//#region 🔖️WorldPick
pub mod world_pick {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "world-pick")]
    pub struct WorldPick {
        pub granularity: String,
        pub id: Option<u32>,
    }

    pub fn handle(payload: &WorldPick, _doc: &DocumentView<'_, Process3dDocument>, _cfg: &ConfigView<'_, Process3dConfig>) -> Result<Emit<Process3dOperation, Process3dConfigOperation>, Fault> {
        if payload.granularity == "face" {
            Ok(Emit::config(vec![Process3dConfigOperation::SetSelectedFaceId { value: payload.id }]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️WorldPick

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn face_drag_negative_distance_yields_cut() {
        let step = process3d_step_from_face_drag([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], -0.5, None, &Process3dLabels::NATIVE_EN).expect("step");
        assert!(matches!(step.measure, ProcessMeasure::Cut { .. }));
        assert_eq!(step.label, "Push Cut");
    }

    #[test]
    fn face_drag_positive_distance_yields_attach() {
        let step = process3d_step_from_face_drag([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], 0.5, None, &Process3dLabels::NATIVE_EN).expect("step");
        assert!(matches!(step.measure, ProcessMeasure::Attach { .. }));
        assert_eq!(step.label, "Pull Attach");
    }

    #[test]
    fn face_drag_zero_distance_is_noop() {
        assert!(process3d_step_from_face_drag([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], 0.0, None, &Process3dLabels::NATIVE_EN).is_none());
    }
}
//#endregion 🧪️Tests
