use previous_scene::standards::v1::subsets::any::scene::fem3d_scene_parts as previous_scene_parts;
use semio_plugin::BuiltNode;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::{FEM3D_GRANULARITY_NODE, FEM3D_INTERACTION_DOMAIN, Fem3dInteractionSnapshot, gumball::fem3d_selection_json};
use semio_s_artifact_fem_3d_editor_model_window::editor::fem3d::modes::edit::windows::model;
use semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results;
use semio_s_artifact_fem_3d_editor_terminology::editor::fem3d::terminology::{Fem3dLabels, ui_label};
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::{io::text::snapshot::parse_dsl, schema::snapshot::Fem3dSnapshot};
use semio_s_artifact_fem_3d_scene::{standards::v1::subsets::any::scene::fem3d_scene_parts, viewport::scene_camera_json};
use semio_ui_contract::{Buildable, HasChildren, HasStackLayout};
use semio_ui_runtime_presentation::ComponentTree;
use semio_ui_runtime_reconcile_cursor::{SurfaceReconcileCursor, SurfaceReconcileStep, SurfaceShadow};
use semio_ui_runtime_reconcile_primitives::{SurfaceReconcileLimits, reserve_surface_reconcile_floor, surface_subtree_items};

fn new_surface(label: &str) -> SurfaceShadow {
    let surface = semio_ui_contract::UiText::try_from_str(&format!("panel:{label}")).unwrap();
    SurfaceShadow::new(semio_ui_contract::SurfaceId(surface))
}

fn census(label: &str, node: BuiltNode, current: &mut SurfaceShadow, generation: u64) -> bool {
    let limits = SurfaceReconcileLimits::default();
    let upper = surface_subtree_items(&node);
    let mut frame = SurfaceReconcileCursor::new_admitted(ComponentTree::new(node), current, limits, generation, reserve_surface_reconcile_floor(limits));
    let started = std::time::Instant::now();
    for turn in 0..10_000_000 {
        match frame.step(current) {
            SurfaceReconcileStep::Yield { .. } => {}
            SurfaceReconcileStep::Complete { mut shadow, patch } => {
                let usage = frame.usage();
                println!("[DEBUG] {label} turn={turn} duration={:?} reconciled {usage:?} item-upper={upper:?} limits={limits:?}", started.elapsed());
                let mut pending = semio_ui_contract::UiPendingPatch::default();
                *pending.source_mut().unwrap() = patch;
                while !pending.terminal_is_empty() {
                    pending.close_step(limits.max_items, limits.max_bytes).unwrap();
                }
                let mut output = None;
                while !shadow.seal_step(size_of::<SurfaceShadow>(), usage, &mut output, false).unwrap() {}
                while !frame.retire_one() {}
                while !current.retire_one(&mut None::<()>, drop) {}
                *current = shadow;
                return true;
            }
            SurfaceReconcileStep::Fault(fault) => {
                println!("[DEBUG] {label} turn={turn} duration={:?} refusal {fault:?} item-upper={upper:?} usage={:?}", started.elapsed(), frame.usage());
                while !frame.retire_one() {}
                return false;
            }
        }
    }
    panic!("reconciliation turn limit");
}

fn bodies(doc: &Fem3dSnapshot, profile: &str, example: &str) -> [BuiltNode; 2] {
            let interaction = Fem3dInteractionSnapshot::default();
            let model_window = model::config::Fem3dModelWindowConfig::default();
            let results_window = results::config::Fem3dResultsWindowConfig::default();
            let result_gumball = model::config::Fem3dGumballConfig { move_axes: false, move_planes: false, rotate: false, scale_axes: false, scale_uniform: false };
                let start = std::time::Instant::now();
                let (meshes, instances) = if profile == "indexed" { fem3d_scene_parts(&doc, None, doc.analysis.deformation_scale, None) } else { previous_scene_parts(&doc, None, doc.analysis.deformation_scale, None) }.unwrap();
                println!("[DEBUG] {profile}-{example}: mesh JSON bytes={} instance JSON bytes={} build={:?}", meshes.len(), instances.len(), start.elapsed());
                let surface = |body, camera, selection| {
                    let mut scene = semio_plugin::world3d_scene(camera, meshes.clone(), instances.clone(), selection, &semio_plugin::WorldSunConfig::default());
                    scene.domain_id = Some(FEM3D_INTERACTION_DOMAIN.into());
                    scene.domain_granularity_id = Some(FEM3D_GRANULARITY_NODE.into());
                    semio_s_fem_app_surface::world_3d_surface(body, &scene).unwrap()
                };
                let model = surface(model::FEM3D_BODY_MODEL, scene_camera_json(&model_window.camera), fem3d_selection_json(&doc, &interaction, false, &model_window.gumball));
                let results = surface(results::FEM3D_BODY_RESULTS, scene_camera_json(&results_window.camera), fem3d_selection_json(&doc, &interaction, false, &result_gumball));
                let caption = semio_ui_contract::text(ui_label(Fem3dLabels::NATIVE_EN.stage_queued.as_str().to_owned()).unwrap());
                let scene_stack = semio_ui_contract::column().grow(true).try_child(results).unwrap_or_else(|_| panic!("results scene child"));
                let pending = semio_ui_contract::column()
                    .grow(true)
                    .try_child(caption)
                    .unwrap_or_else(|_| panic!("results caption"))
                    .try_child(scene_stack)
                    .unwrap_or_else(|_| panic!("results scene stack"))
                    .try_build()
                    .unwrap_or_else(|_| panic!("results stack"));
                [model, pending]
}

fn main() {
    std::thread::Builder::new()
        .stack_size(64 << 20)
        .spawn(|| {
            let _arena = semio_plugin::artifact_app_laws::FixtureUiArena::admit();
            let mut args = std::env::args().skip(1);
            let parse = |asset: String| std::mem::ManuallyDrop::new(parse_dsl(&std::fs::read_to_string(asset).unwrap()).unwrap());
            let house = parse(args.next().expect("House DSL path"));
            let demo = args.next().map(parse);
            let mut indexed_reconciled = true;
            for profile in ["indexed", "archived"] {
                let labels = ["model", "pending-results"];
                let mut surfaces = labels.map(new_surface);
                for ((node, shadow), label) in bodies(&house, profile, "house").into_iter().zip(&mut surfaces).zip(labels) {
                    let admitted = census(&format!("{profile}-house-{label}"), node, shadow, 1);
                    indexed_reconciled &= profile != "indexed" || admitted;
                    while !shadow.retire_one(&mut None::<()>, drop) {}
                }
                if let Some(demo) = &demo {
                    let mut surfaces = labels.map(new_surface);
                    for (((demo_node, house_node), shadow), label) in bodies(demo, profile, "demo").into_iter()
                        .zip(bodies(&house, profile, "house")).zip(&mut surfaces).zip(labels)
                    {
                        let admitted = census(&format!("{profile}-demo-{label}"), demo_node, shadow, 1)
                            && census(&format!("{profile}-demo-to-house-{label}"), house_node, shadow, 2);
                        indexed_reconciled &= profile != "indexed" || admitted;
                        while !shadow.retire_one(&mut None::<()>, drop) {}
                    }
                }
            }
            assert!(indexed_reconciled, "indexed House exceeded the reconciler budget");
        })
        .unwrap()
        .join()
        .unwrap();
}
