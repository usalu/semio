//! 🎬️ Reads portable Puzzle scene carriers and exercises native World3D paint and GPU readback.

use semio_framework_job::{
    drive_step, Generation, InteractiveJob, InteractiveJobCloseStep, InteractiveStage, JobOutcomeKind, OperationId, OriginalJobDecodeOwner, OriginalJobFixtureCaller, OriginalJobReturn, OriginalJobReturnOrigin, OriginalTurnFailure,
    RetainedCloneGrant, StepContext,
};
use semio_framework_mesh_engine::io::binary::mesh_from_glb;
use semio_os_infinite_world_bridge::{begin_world3d_dynamic_retirement, publish_world3d_asset_mesh_lease, World3dSceneBridgeStep};
use semio_os_infinite_world_host::*;
use semio_os_infinite_world_interaction::{step_world3d_draw_rebuild, world3d_begin_relocate_drag, world3d_cancel_relocate_drag, world3d_update_relocate_drag};
use semio_os_infinite_world_model::{World3dBuildContext, WorldCursorWakeAuthority};
use semio_os_infinite_world_scene::World3dShadowProfile;
use semio_ui_declaration::layout::ActionDescriptor;
use semio_ui_declaration::ui::{build_world_3d_scene, UiComponentSceneNode, UiNode};
use semio_ui_draw_contract::DrawList;
use semio_ui_prepared::{retirement::prepared_close_turn, PreparedRasterKeepCursorV1, PreparedRasterKeepStepV1, PreparedRenderInput, PreparedRenderJob};
use semio_ui_scene::{SceneDoc, World3dScene};
use semio_ui_scene_mesh::{mesh3d_allocate_step, mesh3d_begin, mesh3d_seal, mesh3d_write_u32, mesh3d_write_vec3, Mesh3dField, Mesh3dSchema};
use semio_ui_text::FontAtlas;
use semio_ui_theme::{geometry::Rect, Theme};
use semio_ui_wgpu_input::InputState;
use semio_ui_wgpu_render::{GpuContext, RasterTextureWitness};
use semio_ui_wgpu_widget_kit::WidgetContext;
use std::collections::HashMap;
use std::path::PathBuf;

const SIZE: u32 = 192;
const STOCK: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1 << 26, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 1024 };
const LIMIT: usize = 262_144;
const SEED_ID: &str = "seed-left-001";
const SEED_URL: &str = "/mesh/🧊️hexagonal-cut-concrete-forest-left.glb";
const SEED_ASSET: &str = "🧰️framework/🔨️modules/🖼️assets/🏚️abbau-aufbau/◀️hexagonal-cut-concrete-forest-left.glb";

struct Caller {
    root: OriginalJobFixtureCaller,
    decoder: OriginalJobDecodeOwner,
}

impl Caller {
    fn turn<T>(&mut self, f: impl FnOnce(&mut StepContext<'_>) -> Result<T, OriginalTurnFailure>) -> T {
        self.root.turn_with_decoder(STOCK, &mut self.decoder, f).unwrap()
    }
}

fn paint(node: &UiComponentSceneNode, state: &mut World3dState, world: &mut World3dBuildContext) -> DrawList {
    let mut draw = DrawList::default();
    draw.set_screen_height(SIZE as f32);
    let mut atlas = FontAtlas::builtin();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut context = WidgetContext {
        draw: &mut draw,
        overlay: None,
        atlas: &mut atlas,
        icons: None,
        input: &mut input,
        theme: &Theme::default(),
        scroll_offsets: &mut HashMap::new(),
        collapsed_sections: &mut HashMap::new(),
        open_selects: &mut HashMap::new(),
        interaction_maps: None,
        pick_clip: None,
        viewport_height: SIZE as f32,
    };
    render_world_3d(node, Rect::new(0.0, 0.0, SIZE as f32, SIZE as f32), &mut context, state, world, World3dShadowProfile::Unshadowed);
    draw
}

fn admit_seed(state: &mut World3dState, caller: &mut Caller) {
    let mesh = mesh_from_glb(&std::fs::read(SEED_ASSET).unwrap()).unwrap();
    let revision = state.session.interaction_revision;
    let schema = Mesh3dSchema { vertices: (mesh.positions.len() / 3) as u32, indices: mesh.indices.len() as u32, face_ids: 0, vertex_ids: 0, edges: 0, edge_ids: 0, uvs: 0, colors: 0, surface_uvs: [0; 4], tangents: 0 };
    assert!(schema.vertices > 0 && schema.indices > 0);
    assert_eq!(mesh.positions.len(), mesh.normals.len());
    let writer = caller.turn(|cx| {
        let (writer, progress) = mesh3d_begin(revision, revision, schema, cx.retained_grant()).unwrap();
        cx.consume_retained(progress)?;
        Ok(writer)
    });
    assert!((0..LIMIT).any(|_| caller.turn(|cx| {
        let step = mesh3d_allocate_step(writer, cx.retained_grant()).unwrap();
        cx.consume_retained(step.progress)?;
        Ok(step.complete)
    })));
    for xyz in mesh.positions.chunks_exact(3) {
        mesh3d_write_vec3(writer, Mesh3dField::Positions, [xyz[0], -xyz[2], xyz[1]]).unwrap();
    }
    for xyz in mesh.normals.chunks_exact(3) {
        mesh3d_write_vec3(writer, Mesh3dField::Normals, [xyz[0], -xyz[2], xyz[1]]).unwrap();
    }
    for index in mesh.indices {
        mesh3d_write_u32(writer, Mesh3dField::Indices, index).unwrap();
    }
    let mesh = mesh3d_seal(writer).unwrap();
    publish_world3d_asset_mesh_lease(&mut state.session, &mut state.interaction, &mut state.scene, &mut state.assets, SEED_URL, mesh).unwrap_or_else(|rejected| panic!("bundled seed asset rejected: {:?}", rejected.fault));
    println!("[DEBUG] Published the registered seed's bundled GLB with {} vertices and {} indices", schema.vertices, schema.indices);
}

fn render_publication(scene: World3dScene, revision: u64, include_seed: bool, gpu: &mut GpuContext, caller: &mut Caller) -> (Vec<u8>, usize, Option<(Vec<u8>, Vec<u8>)>) {
    let node = build_world_3d_scene("puzzle-offscreen", "puzzle", scene);
    let UiNode::ComponentScene(node) = node else { panic!("world assembly") };
    let mut state = World3dState::request_owned(node.surface_id.clone(), node.controller_id.clone(), revision);
    if include_seed {
        admit_seed(&mut state, caller);
    }
    let mut world = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    assert!((0..LIMIT).any(|_| caller.turn(|cx| world.prepare_step(cx))));
    let mut draw = None;
    for _ in 0..1024 {
        let next = paint(&node, &mut state, &mut world);
        if !world3d_cursor_work_pending(&state) && !next.scene_passes.is_empty() {
            draw = Some(next);
            break;
        }
        for _ in 0..128 {
            let pending = caller.turn(|cx| {
                let bridge = step_world3d_scene_bridge(&mut state, cx);
                assert!(!matches!(bridge, World3dSceneBridgeStep::Fault), "{:?}", state.snapshot_fault());
                let _ = step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, cx);
                let _ = step_world3d_snapshot(&mut state, cx);
                Ok(world3d_cursor_work_pending(&state))
            });
            if !pending {
                break;
            }
        }
    }
    assert!(draw.is_some(), "World3D paint stalled: {}", state.ingest_census());
    use semio_ui_scene::math::{Camera3d, CameraProjection3d, OrbitController, Vec3};
    state.camera.orbit = OrbitController::from_camera(&Camera3d {
        position: Vec3::new(5.4, -15.0, 14.0),
        target: Vec3::new(5.4, 2.3, 1.5),
        up: Vec3::new(0.0, 0.0, 1.0),
        fov_y: 55.0_f32.to_radians(),
        near: semio_ui_scene::math::WORLD_ORBIT_CAMERA_NEAR,
        far: semio_ui_scene::math::WORLD_ORBIT_CAMERA_MIN_FAR,
        projection: CameraProjection3d::Perspective,
        zoom: 1.0,
    });
    state.camera.camera_user_moved = true;
    let draw = paint(&node, &mut state, &mut world);
    let (pixels, instances) = submit(draw, revision, &mut world, gpu, caller);
    let relocation = include_seed.then(|| {
        let original = state.scene.draws.iter().flat_map(|draw| &draw.instances).find(|instance| instance.id == SEED_ID).expect("published relocate source").model;
        state.session.active_utility = "worldRelocate".into();
        state.selection.local_hover_id = Some(SEED_ID.into());
        let center = SIZE as f32 / 2.0;
        assert!(world3d_begin_relocate_drag(&mut state.session, &mut state.camera, &mut state.selection, &mut state.gumball, &mut state.scene, center, center,));
        state.selection.local_hover_id = None;
        let held_model = state.gumball.relocate.as_ref().unwrap().preview_model;
        assert_eq!(held_model.cols, original.cols);
        assert!(world3d_update_relocate_drag(&mut state.session, &mut state.camera, &mut state.gumball, &mut state.scene, center + 46.0, center,));
        let moved = state.gumball.relocate.as_ref().unwrap().preview_model;
        assert_eq!(moved.cols[..3], original.cols[..3]);
        assert_ne!(moved.cols[3], original.cols[3]);
        let preview = paint(&node, &mut state, &mut world);
        assert!(preview.scene_passes.iter().flat_map(|pass| &pass.translucent_draws).flat_map(|draw| &draw.instances).any(|instance| instance.id == "relocate-preview" && instance.model.cols == moved.cols));
        let (preview, _) = submit(preview, revision + 1, &mut world, gpu, caller);
        assert!(world3d_cancel_relocate_drag(&mut state.session, &mut state.camera, &mut state.gumball, &mut state.scene,));
        assert!(state.gumball.relocate.is_none());
        let restored = paint(&node, &mut state, &mut world);
        assert!(restored.scene_passes.iter().flat_map(|pass| &pass.draws).flat_map(|draw| &draw.instances).any(|instance| instance.id == SEED_ID && instance.model.cols == original.cols));
        let (restored, _) = submit(restored, revision + 2, &mut world, gpu, caller);
        (preview, restored)
    });
    assert!((0..LIMIT).any(|_| caller.turn(|cx| {
        world.close_step(cx)?;
        Ok(world.terminal_is_empty())
    })));
    begin_world3d_dynamic_retirement(&mut state.session, &mut state.camera, &mut state.retirement, &mut state.scene, &mut state.assets);
    assert!((0..LIMIT).any(|_| caller.turn(|cx| Ok(step_world3d_dynamic_retirement(&mut state, cx)))));
    println!("[DEBUG] World revision {revision} retired after painting {instances} committed instances");
    (pixels, instances, relocation)
}

fn submit(draw: DrawList, revision: u64, world: &mut World3dBuildContext, gpu: &mut GpuContext, caller: &mut Caller) -> (Vec<u8>, usize) {
    let instances = draw.scene_passes.iter().map(|pass| pass.draws.iter().map(|draw| draw.instances.len()).sum::<usize>()).sum();
    let generation = caller.turn(|cx| Ok(cx.generation().0));
    let mut input = PreparedRenderInput::try_new(revision, generation, draw, None, 0.0).unwrap_or_else(|rejection| panic!("{}", rejection.fault()));
    assert!((0..LIMIT).any(|_| world.append_step(&mut input).unwrap_or_else(|_| panic!("World upload transfer"))));
    let mut job = PreparedRenderJob::try_new(input).unwrap_or_else(|rejection| panic!("{}", rejection.fault()));
    let mut origin = OriginalJobReturnOrigin::empty();
    let mut returned = None;
    let completed = (0..LIMIT).any(|_| {
        caller.turn(|cx| {
            if returned.is_some() {
                origin.receive_outcome(cx, &mut returned)?;
                return Ok(false);
            }
            if let Some(outcome) = origin.outcome() {
                let kind = outcome.kind();
                let settled = origin.acknowledge_outcome(cx)?;
                assert!(matches!(kind, JobOutcomeKind::Complete | JobOutcomeKind::Yield), "{:?}", job.fault());
                return Ok(settled && kind == JobOutcomeKind::Complete);
            }
            if !origin.body_ready() {
                origin.admit(cx)?;
                return Ok(false);
            }
            match drive_step(&mut job, cx, origin.receiver()?, "puzzle3d-offscreen", InteractiveStage::InteractiveStep, &mut None)? {
                OriginalJobReturn::Preparing | OriginalJobReturn::Pending => {}
                OriginalJobReturn::Outcome(reply) => returned = Some(reply.into_descriptor()),
                OriginalJobReturn::Continuation(reply) => panic!("prepared job continuation: {:?}", reply.view().callback),
            }
            Ok(false)
        })
    });
    assert!(completed, "prepared world packet");
    let mut packet = job.take_packet().expect("accepted world packet");
    if let Some(pass) = packet.draw().scene_passes.first() {
        let view_proj = semio_ui_scene::math::Mat4 { cols: std::array::from_fn(|column| pass.view_proj[column * 4..column * 4 + 4].try_into().unwrap()) };
        let center = semio_ui_scene::math::Vec3::new(5.4, 2.3, 1.5);
        println!("[DEBUG] Frame {revision} camera {:?}; seed center {:?}", pass.camera_position, semio_ui_scene::math::project_point(view_proj, center, SIZE as f32, SIZE as f32));
    }
    let commands: Vec<_> = (0..).map_while(|index| packet.command_pages().get(index)).filter_map(|command| command.draw_cursor()).collect();
    let world_draws = commands.iter().filter(|cursor| matches!(cursor, semio_ui_prepared::DrawMeasureCursor::PassInstance { .. })).count();
    println!("[DEBUG] Frame {revision} measured {world_draws} world instance commands");
    let witness = RasterTextureWitness { scene_revision: revision, preview_generation: generation, operation: revision };
    gpu.begin_raster_ownership(witness).unwrap();
    let mut keep = PreparedRasterKeepCursorV1::default();
    assert!((0..LIMIT).any(|_| match packet.raster_keep_step(&mut keep) {
        PreparedRasterKeepStepV1::Pending => false,
        PreparedRasterKeepStepV1::Key(key) => {
            gpu.publish_raster_ownership(witness, key).unwrap();
            false
        }
        PreparedRasterKeepStepV1::Complete => true,
    }));
    gpu.seal_raster_ownership(witness).unwrap();
    println!("[DEBUG] Scene revision {revision} prepared {instances} instances and {} uploads", packet.uploads().len());
    for index in 0..packet.uploads().len() {
        assert!((0..LIMIT).any(|_| gpu.apply_prepared_upload_step(&packet, index, witness, witness).unwrap()));
    }
    let mut cursor = gpu.begin_prepared_present(&packet, witness).unwrap();
    assert!((0..LIMIT).any(|_| gpu.prepared_present_step(&packet, &mut cursor).unwrap()));
    let mut readback = gpu.begin_prepared_readback(&cursor).unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while !gpu.prepared_readback_step(&mut readback).unwrap() {
        assert!(std::time::Instant::now() < deadline, "GPU readback");
        std::thread::yield_now();
    }
    let pixels = readback.take_rgba().unwrap();
    let missing = gpu.take_missing_world_mesh();
    assert!((0..LIMIT).any(|_| gpu.commit_presented_rasters_step(witness).unwrap()));
    assert!((0..LIMIT).any(|_| cursor.close_step()));
    assert!((0..LIMIT).any(|_| caller.turn(|cx| {
        let quote = packet.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;
        Ok(matches!(prepared_close_turn(cx, quote, |effect| packet.close_step(effect))?, InteractiveJobCloseStep::Complete { .. }))
    })));
    assert!((0..LIMIT).any(|_| caller.turn(|cx| {
        origin.close_step(cx)?;
        Ok(origin.terminal_is_empty())
    })));
    job.begin_close();
    assert!((0..LIMIT).any(|_| caller.turn(|cx| Ok(matches!(job.close_step(cx)?, InteractiveJobCloseStep::Complete { .. })))));
    assert_eq!(missing, None, "production mesh upload");
    (pixels, instances)
}

fn main() {
    std::thread::Builder::new().stack_size(134_217_728).spawn(render).unwrap().join().unwrap();
}

fn render() {
    let work = PathBuf::from(std::env::args_os().nth(1).expect("portable scene directory"));
    let mut scene = World3dScene::decode_pack(&std::fs::read(work.join("spine.pack")).unwrap()).unwrap();
    let lanes: Vec<(String, String)> = serde_json::from_slice(&std::fs::read(work.join("lanes.json")).unwrap()).unwrap();
    for (key, value) in lanes {
        assert!(scene.merge_lane(&key, value), "scene lane {key}");
    }
    scene.fit_json = None;
    scene.references_json = None;
    scene.presentation_json = Some(r#"{"showGrid":false,"showGizmo":false,"clear":"transparent"}"#.into());
    let mut empty = scene.clone();
    empty.instances_json = "[]".into();
    empty.instances_delta_json = None;
    let mut gpu = semio_framework_async::block_on(GpuContext::headless(SIZE, SIZE)).unwrap();
    let mut caller = Caller { root: OriginalJobFixtureCaller::admit(OperationId(933), Generation(1), STOCK, || Some(0), u64::MAX, 1 << 44), decoder: OriginalJobDecodeOwner::empty() };
    let (baseline, instances, relocation) = render_publication(scene, 1, true, &mut gpu, &mut caller);
    let (control, _, _) = render_publication(empty, 4, false, &mut gpu, &mut caller);
    let (preview, restored) = relocation.unwrap();
    assert!((0..LIMIT).any(|_| gpu.close_mesh_table_step()));
    assert!((0..LIMIT).any(|_| gpu.close_raster_table_step().unwrap()));
    for _ in 0..LIMIT {
        if caller.decoder.terminal_is_empty() {
            break;
        }
        caller
            .root
            .turn(STOCK, |cx| {
                caller.decoder.close_on_source(cx)?;
                Ok(())
            })
            .unwrap();
    }
    assert!(caller.decoder.terminal_is_empty());
    caller.root.retire(STOCK);
    assert!(instances > 0, "registered Puzzle publication produced a visible draw");
    assert_eq!(baseline.len(), (SIZE * SIZE * 4) as usize);
    let changed = baseline.chunks_exact(4).zip(control.chunks_exact(4)).filter(|(a, b)| a != b).count();

    let preview_changed = baseline.chunks_exact(4).zip(preview.chunks_exact(4)).filter(|(a, b)| a != b).count();

    println!("[DEBUG] Painted {instances} instances across {changed} pixels and saw {preview_changed} changed held-preview pixels");
    let restored_matches = baseline == restored;
    println!("[DEBUG] Cancelled frame exactly restored: {restored_matches}");
    std::fs::write(work.join("preview.rgba"), preview).unwrap();
    std::fs::write(work.join("restored.rgba"), restored).unwrap();
    std::fs::write(work.join("scene.rgba"), baseline).unwrap();
    std::fs::write(work.join("control.rgba"), control).unwrap();
    assert!(changed > 16, "the published Puzzle geometry has a visible silhouette");
    assert!(preview_changed > 16, "the held relocate ghost changes visible pixels");
    assert!(restored_matches, "cancel restores the released framebuffer");
}
