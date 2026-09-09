use super::*;
use crate::{ProcessStep, StepOrigin};

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = Process3dSnapshot::default();
    assert_eq!(Process3dInference::infer(&snapshot), Process3dInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(Process3dInference::infer(&Process3dSnapshot::default()), Process3dInference::default());
}

/// 🌉️ Documented gap (see file doc comment): a plain snapshot can't see its composed `steps`
/// child's content, so `step_count` is always 0 regardless of the working scene's real steps.
#[semio_framework_async_macros::async_test]
async fn step_count_is_zero_pending_a_resolver() {
    let snapshot = Process3dSnapshot::default();
    assert_eq!(Process3dInference::infer(&snapshot).step_count, 0);
}
//#endregion 🧪️InferenceLaws

//#region 🧪️KernelReplay
fn drill_step(id: &str, radius: f64, depth: f64, pose: Pose) -> ProcessStep {
    ProcessStep { id: id.into(), label: "Drill".into(), enabled: true, origin: Some(StepOrigin { machine_id: "drill".into(), capability_id: "drill".into() }), measure: ProcessMeasure::Drill { radius, depth, pose } }
}

fn session_volume(session: &mut ProcessKernelReplay, scene: &ProcessWorkingScene, resolved_up_to: Option<usize>) -> f64 {
    let handle = replay_process(session, scene, resolved_up_to).expect("replayed handle");
    session.kernel().volume(&handle).expect("replayed volume")
}

#[semio_framework_async_macros::async_test]
async fn drill_reduces_volume_below_stock() {
    let mut scene = ProcessWorkingScene { stock: Stock { id: "stock".into(), label: "Stock".into(), solid: WorkingSolid::Box { width: 1.0, depth: 1.0, height: 1.0 }, pose: Pose::default() }, steps: Vec::new() };
    let stock_volume = processed_volume(&scene, None).expect("stock volume");
    scene.steps.push(ProcessStep {
        id: "drill-1".into(),
        label: "Drill".into(),
        enabled: true,
        origin: None,
        measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.4, depth: 0.4, height: 1.2 }, pose: Pose { position: [0.3, 0.3, -0.1], axis: [0.0, 0.0, 1.0], angle: 0.0 } },
    });
    let drilled_volume = processed_volume(&scene, None).expect("drilled volume");
    assert!(drilled_volume < stock_volume, "drilled volume {drilled_volume} should be less than stock volume {stock_volume}");
}

#[semio_framework_async_macros::async_test]
async fn attach_increases_volume_above_stock() {
    for _ in 0..32 {
        let mut scene = ProcessWorkingScene { stock: Stock { id: "stock".into(), label: "Stock".into(), solid: WorkingSolid::Box { width: 1.0, depth: 1.0, height: 1.0 }, pose: Pose::default() }, steps: Vec::new() };
        let stock_volume = processed_volume(&scene, None).expect("stock volume");
        scene.steps.push(ProcessStep {
            id: "attach-1".into(),
            label: "Attach".into(),
            enabled: true,
            origin: None,
            measure: ProcessMeasure::Attach { component: WorkingSolid::Box { width: 0.4, depth: 0.4, height: 0.4 }, pose: Pose { position: [0.3, 0.3, 1.0], axis: [0.0, 0.0, 1.0], angle: 0.0 } },
        });
        let attached_volume = processed_volume(&scene, None).expect("attached volume");
        assert!(attached_volume > stock_volume, "attached volume {attached_volume} should exceed stock volume {stock_volume}");
    }
}

#[semio_framework_async_macros::async_test]
async fn disabled_step_is_skipped_on_replay() {
    let mut session = ProcessKernelReplay::new();
    let mut scene = ProcessWorkingScene { stock: Stock { id: "stock".into(), label: "Stock".into(), solid: WorkingSolid::Box { width: 1.0, depth: 1.0, height: 1.0 }, pose: Pose::default() }, steps: Vec::new() };
    let stock_volume = session_volume(&mut session, &scene, None);
    scene.steps.push(ProcessStep { id: "drill-1".into(), label: "Drill".into(), enabled: false, origin: None, measure: ProcessMeasure::Drill { radius: 0.2, depth: 1.0, pose: Pose::default() } });
    let volume_with_disabled_step = session_volume(&mut session, &scene, None);
    assert!((volume_with_disabled_step - stock_volume).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn cursor_zero_yields_stock_volume() {
    let mut session = ProcessKernelReplay::new();
    let mut scene = ProcessWorkingScene { stock: Stock { id: "stock".into(), label: "Stock".into(), solid: WorkingSolid::Box { width: 1.0, depth: 1.0, height: 1.0 }, pose: Pose::default() }, steps: Vec::new() };
    let stock_volume = session_volume(&mut session, &scene, None);
    scene.steps.push(drill_step("drill-1", 0.2, 1.0, Pose::default()));
    let volume_at_cursor_zero = session_volume(&mut session, &scene, Some(0));
    assert!((volume_at_cursor_zero - stock_volume).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn box_primitive_spans_from_local_origin_corner() {
    let mut kernel = Brep::new();
    let handle = kernel.box_prim(2.0, 3.0, 4.0).expect("box prim");
    let mesh = kernel.tessellate(&handle, 0.1).expect("tessellate");
    let axis_bounds = |offset: usize| -> (f32, f32) {
        let values: Vec<f32> = mesh.position.iter().skip(offset).step_by(3).copied().collect();
        (values.iter().copied().fold(f32::INFINITY, f32::min), values.iter().copied().fold(f32::NEG_INFINITY, f32::max))
    };
    let (min_x, max_x) = axis_bounds(0);
    let (min_y, max_y) = axis_bounds(1);
    let (min_z, max_z) = axis_bounds(2);
    assert!(min_x.abs() < 1e-4 && (max_x - 2.0).abs() < 1e-4, "box x should span [0, width] from the local origin corner, got [{min_x}, {max_x}]");
    assert!(min_y.abs() < 1e-4 && (max_y - 3.0).abs() < 1e-4, "box y should span [0, depth], got [{min_y}, {max_y}]");
    assert!(min_z.abs() < 1e-4 && (max_z - 4.0).abs() < 1e-4, "box z should span [0, height], got [{min_z}, {max_z}]");
}
//#endregion 🧪️KernelReplay
