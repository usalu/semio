//! 🌍️ Process 3d play app commands — 3D viewport interactions: click-to-place, push/pull face drag,
//! and face picking.

use crate::editor::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::editor::process3d::terminology::{process3d_labels, Process3dLabels};
use crate::editor::process3d::set_active_utility_effect;
use crate::editor::process3d::axis_angle_from_up_to;
use crate::artifacts::process3d::schema::inferences::capability_for_measure_kind;
use crate::artifacts::process3d::schema::{insert_step_mutations, next_step_id};
use crate::artifacts::process3d::{op::Process3dMutation, MeasureKind, Pose, Process3dSnapshot, ProcessMeasure, ProcessStep, WorkingSolid, StepOrigin};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️FaceDrag
/// 🖱️➡️ Builds a push/pull step from a face-drag gesture: dragging into the solid (negative `distance`
/// along the face's outward `normal`) removes material (Cut); dragging outward (positive) adds material
/// (Attach). The tool box's local origin corner lands at `point + normal * distance.min(0.0)` so it spans
/// exactly the dragged region, flush with the picked face — `box_prim_sync` places a primitive's corner
/// (not its center) at the local origin, confirmed by `box_primitive_spans_from_local_origin_corner` in
/// the artifact's `⚙️engine`.
async fn process3d_step_from_face_drag(normal: [f64; 3], point: [f64; 3], distance: f64, face_extent: Option<[f64; 2]>, labels: &Process3dLabels) -> Option<ProcessStep> {
    if distance.abs() < 1e-6 {
        return None;
    }
    let (width, depth) = face_extent.map_or((0.2, 0.2), |[w, d]| (w.max(0.02), d.max(0.02)));
    let height = distance.abs();
    let (axis, angle) = axis_angle_from_up_to(normal);
    let offset = distance.min(0.0);
    let position = [point[0] + normal[0] * offset, point[1] + normal[1] * offset, point[2] + normal[2] * offset];
    let pose = Pose { position, axis, angle };
    let (measure, label, machine_id, capability_id) = if distance < 0.0 {
        (ProcessMeasure::Cut { tool: WorkingSolid::Box { width, depth, height }, pose }, labels.push_cut, "saw", "cut")
    } else {
        (ProcessMeasure::Attach { component: WorkingSolid::Box { width, depth, height }, pose }, labels.pull_attach, "attacher", "attach")
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

    pub async fn handle(payload: &WorldPointerDown, doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>, _ctx: &mut crate::editor::process3d::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let config = cfg.snapshot;
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
        let origin = StepOrigin { machine_id: machine.id, capability_id: capability.id.clone() };
        let step = ProcessStep {
            id: next_step_id(),
            label: capability.label.clone(),
            enabled: true,
            origin: Some(origin),
            measure: crate::artifacts::process3d::schema::inferences::measure_for_capability(&capability, Some(payload.position)),
        };
        Ok(Emit { artifact_mutations: insert_step_mutations(fixture, step), effects: vec![set_active_utility_effect("select")], ..Default::default() })
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

    pub async fn handle(payload: &WorldFaceDragEnd, doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>, _ctx: &mut crate::editor::process3d::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let config = cfg.snapshot;
        if config.active_utility() != "select" {
            return Ok(Emit::default());
        }
        match process3d_step_from_face_drag(payload.normal, payload.start_point, payload.distance, payload.face_extent, process3d_labels(config)) {
            Some(step) => Ok(Emit { artifact_mutations: insert_step_mutations(fixture, step), ..Default::default() }),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️WorldFaceDragEnd

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn face_drag_negative_distance_yields_cut() {
        let step = process3d_step_from_face_drag([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], -0.5, None, &Process3dLabels::NATIVE_EN).expect("step");
        assert!(matches!(step.measure, ProcessMeasure::Cut { .. }));
        assert_eq!(step.label, "Push Cut");
    }

    #[test]
    async fn face_drag_positive_distance_yields_attach() {
        let step = process3d_step_from_face_drag([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], 0.5, None, &Process3dLabels::NATIVE_EN).expect("step");
        assert!(matches!(step.measure, ProcessMeasure::Attach { .. }));
        assert_eq!(step.label, "Pull Attach");
    }

    #[test]
    async fn face_drag_zero_distance_is_noop() {
        assert!(process3d_step_from_face_drag([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], 0.0, None, &Process3dLabels::NATIVE_EN).is_none());
    }
}
//#endregion 🧪️Tests
