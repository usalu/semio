//! 🧪️ host tests (🔭️unit-2): compiled against the `testkit` surface of the world crates.

use semio_framework_os_infinite_world_model::{tool_run_trace, TRANSLATE_SELECTION_ACTION_ID, World3dBuildContext, WORLD_INTERACTION_OBJECT_CAPACITY, world_mesh_ownership_grant, world_reference_width, world_scene_raster_pool, WorldCancelReason, WorldCursorWakeAuthority, WorldDrawRebuildDescriptor, WorldDrawRebuildStep, WorldDrawRegistry, WorldInteractionIntent, WorldInteractionObjectKind, WorldInteractionPhase, WorldInteractionStep, WorldReferenceRecord, WorldSceneRaster, WorldTerrainStyle};
use semio_framework_os_infinite_world_components::{GumballHandle, set_world3d_declared_actions, world_mesh_component_id, WorldFaceOverlayMeshCursor, WorldFaceOverlayMeshStep, WorldFaceOverlayScan, WorldFaceOverlayScanStep, WorldPlaceholderOwner};
use semio_framework_os_infinite_world_cursors::{live_world3d_mesh_version, resolved_domain_granularity_id, resolved_domain_id, WORLD_INTERACTION_DOMAIN_ID, WORLD_ITEM_GRANULARITY_ID, WorldComponentKind, WorldComponentMarqueePublishJob, WorldComponentPickCursor, WorldComponentPickPurpose, WorldGumballCommitJob, WorldGumballGesture, WorldGumballPhase, WorldGumballPickCursor, WorldInteractionAuthorityStep, WorldMarqueeGesture, WorldMarqueePickCursor, WorldMarqueeResult, WorldObjectPickCursor, WorldObjectPickPurpose, WorldRayPickCursor, WorldRayPickPurpose};
use semio_framework_os_infinite_world_scene::{apply_terrain_style_if_changed_state, apply_world3d_terrain_tile_bytes, environment_light_dir, GUMBALL_PLANE_MESH, offer_missing_mesh_fetches, parse_color, reset_world3d_brush_mesh_registry, retire_cancelled_world3d_asset_step, return_world3d_asset, scene_lod, step_world3d_brush_mesh_announce, step_world3d_terrain_retirement, sync_world3d_brush_mesh_feedback, take_next_world3d_asset, terrain_tile_mesh_key, terrain_tile_url, world3d_brush_mesh_announce_pending, WORLD_BRUSH_MESH_MAX_PAGES, WorldAssetRequestKind, WorldTerrainMeshPublicationStep};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use semio_framework_os_infinite_world_scene::{sort_world3d_translucent_material_draws, World3dShadowProfile};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_world_scene::TerrainMeshOracleData;
use semio_framework_os_infinite_world_bridge::{append_component_overlays, append_gumball_geometry, apply_world3d_initial_projection_seed, apply_world3d_projection_spec, begin_world3d_draw_rebuild, begin_world3d_dynamic_retirement, DecodedMeshPaintImage, publish_world3d_asset_mesh_lease, sync_world3d_projection_content_frame, sync_world3d_scene_selection, world3d_draw_rebuild_admit_draw, world3d_draw_rebuild_admit_instance, world3d_draw_rebuild_seal, world3d_draw_rebuild_terminal_is_empty, world3d_live_camera_json, World3dCameraFitStep, World3dSceneBridgeStep, World3dSnapshotApplyStep, world_reference_plane_size, world_reference_visual, WorldInteractionRegistryBuildCursor};
use semio_framework_os_infinite_world_interaction::{apply_decoded_mesh_paint_image, apply_world_action_preview, begin_world3d_interaction_close, close_world3d_interaction_step, enqueue_world3d_event, publish_world3d_plan_step, resolve_world_context_menu_target, step_world3d_draw_rebuild, step_world3d_terrain, store_mesh, sync_terrain_state, world3d_interaction_terminal_is_empty, world_merge_code};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_world_interaction::{marquee_select_action, pick_hover_action, pick_instance_at, pick_select_action, resolved_item_id};
use semio_framework_os_infinite_world_host::{apply_runtime_draw_flags, step_world3d_camera_fit, step_world3d_dynamic_retirement, step_world3d_interaction, step_world3d_scene_bridge, step_world3d_snapshot, sync_world3d_state, world3d_cursor_work_pending, world3d_dynamic_retirement_terminal_is_empty, world3d_tool_run_trace_cursors, World3dState};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_world_host::handle_world3d_pointer_button;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use semio_framework_os_infinite_world_host::render_world_3d;
use ui_wgpu::wgpu::{ActionDescriptor, Camera3d, camera_grid_fade_distance, CameraProjection3d, Instance3d, lod_grid_step_world, Mat4, mesh3d_allocate_step, mesh3d_begin, mesh3d_seal, mesh3d_write_edge, mesh3d_write_u32, mesh3d_write_vec3, mesh3d_write_vec4, Mesh3dField, Mesh3dLease, Mesh3dSchema, Mesh3dWriteToken, OrbitController, PointerModifiers, Rect, Rgba, SceneDraw3d, SceneRasterBegin, SceneRasterDescriptor, SceneRasterLease, SceneRasterProfile, SceneRasterWriteMode, SurfaceKind, Theme, UiComponentSceneNode, UiPresence, Vec3, World3dPresentation, World3dPresentationClear, World3dScene};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Mutex;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use ui_wgpu::wgpu::{SceneMaterialDraw3d, SceneMaterialKind3d, SceneShadowRole3d, SceneViewportMask3d, WORLD_SHADOW_MAP_SIZE};
#[cfg(test)]
use ui_wgpu::wgpu::SceneRasterMeshSeal;
use semio_framework_tool_run::{ToolRunTraceCursor, ToolRunTraceDelta};
use semio_framework_ui_viewport::{Viewport3dProjectionFramePolicy, Viewport3dProjectionMode, Viewport3dProjectionOrientation, Viewport3dProjectionSpec};
use serde_json::json;

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.
fn read_static(path_from_manifest: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path_from_manifest);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("test input {}: {error}", path.display())).into_boxed_str())
}

/// 📖️ Reads several test inputs at run time and joins them into the one text the unsplit sources formed.
fn read_joined(paths: &[&str]) -> &'static str {
    let mut joined = String::new();
    for path in paths {
        joined.push_str(read_static(path));
    }
    Box::leak(joined.into_boxed_str())
}

fn test_scene_raster_lease(width: u32, height: u32, profile: SceneRasterProfile, mesh: Option<SceneRasterMeshSeal>, fill: u8) -> SceneRasterLease {
    static REVISION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let revision = REVISION.fetch_add(1, std::sync::atomic::Ordering::Relaxed).max(1);
    let descriptor = SceneRasterDescriptor { width, height, source_digest: [revision, revision.rotate_left(17)], source_revision: revision, profile, mesh };
    let pool = world_scene_raster_pool();
    let mut writer = None;
    for _ in 0..=ui_wgpu::wgpu::SCENE_RASTER_POOL_SLOTS {
        match pool.begin(descriptor, revision, SceneRasterWriteMode::Moved { capacity_bytes: descriptor.byte_len().unwrap() }) {
            SceneRasterBegin::Writer(candidate) => {
                writer = Some(candidate);
                break;
            }
            SceneRasterBegin::Reused(lease) => return lease,
            SceneRasterBegin::Backpressure(_) => {
                let _ = pool.maintenance_step();
            }
            SceneRasterBegin::Refused(fault) => panic!("test scene raster refused: {fault}"),
        }
    }
    let writer = writer.expect("test scene raster progresses within the fixed slot budget");
    let pixels = vec![fill; descriptor.byte_len().expect("test raster bytes")];
    pool.seal_moved(writer, pixels).map_err(|(_, fault)| fault).expect("test raster publishes")
}

fn test_world_scene_raster(width: u32, height: u32, fill: u8) -> WorldSceneRaster {
    let lease = test_scene_raster_lease(width, height, SceneRasterProfile::ReferenceImageMapNoColorSpace, None, fill);
    WorldSceneRaster { identity: lease.identity(), pending: Mutex::new(Some(lease)) }
}

pub(crate) fn take_actions(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    let mut actions = Vec::new();
    while let Some(action) = input.take_action_step().expect("action authority live") {
        actions.push(action.into_descriptor().expect("bounded action materializes"));
    }
    actions
}

fn triangle_mesh_oracle() -> TerrainMeshOracleData {
    mesh_oracle_from_buffers(vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2])
}

pub(crate) fn mesh_oracle_from_buffers(positions: Vec<f32>, normals: Vec<f32>, indices: Vec<u32>) -> TerrainMeshOracleData {
    TerrainMeshOracleData { positions, normals, indices, face_ids: Vec::new(), vertex_ids: Vec::new(), edge_positions: Vec::new(), edge_ids: Vec::new(), uvs: Vec::new(), colors: Vec::new() }
}

pub(crate) fn publish_oracle_mesh(data: TerrainMeshOracleData) -> Mesh3dLease {
    publish_oracle_mesh_at_revision(data, 0)
}

/// 🧊️ An oracle mesh stamped with a chosen interaction revision — what
/// `publish_world3d_asset_mesh_lease` witnesses before it stores a decoded GLB under its url's id.
pub(crate) fn publish_oracle_mesh_at_revision(data: TerrainMeshOracleData, revision: u64) -> Mesh3dLease {
    assert!(data.positions.len().is_multiple_of(3));
    assert_eq!(data.normals.len(), data.positions.len());
    assert!(data.edge_positions.len().is_multiple_of(6));
    assert!(data.uvs.len().is_multiple_of(2));
    assert!(data.colors.len().is_multiple_of(4));
    static GENERATION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(10_000);
    let schema = Mesh3dSchema {
        vertices: (data.positions.len() / 3) as u32,
        indices: data.indices.len() as u32,
        face_ids: data.face_ids.len() as u32,
        vertex_ids: data.vertex_ids.len() as u32,
        edges: (data.edge_positions.len() / 6) as u32,
        edge_ids: data.edge_ids.len() as u32,
        uvs: (data.uvs.len() / 2) as u32,
        colors: (data.colors.len() / 4) as u32,
    surface_uvs:[0;4],tangents:0,};
    let generation = GENERATION.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let grant=world_mesh_ownership_grant(0,ui_wgpu::wgpu::mesh3d_begin_capacity_byte_demand(),0,1);
    let (token,progress)=mesh3d_begin(generation,revision,schema,grant).expect("oracle mesh claim");
    assert!(progress.fits(grant));
    allocate_oracle_mesh(token);
    for value in data.positions.as_chunks::<3>().0 {
        mesh3d_write_vec3(token, Mesh3dField::Positions, *value).unwrap();
    }
    for value in data.normals.as_chunks::<3>().0 {
        mesh3d_write_vec3(token, Mesh3dField::Normals, *value).unwrap();
    }
    for value in data.indices {
        mesh3d_write_u32(token, Mesh3dField::Indices, value).unwrap();
    }
    for value in data.face_ids {
        mesh3d_write_u32(token, Mesh3dField::FaceIds, value).unwrap();
    }
    for value in data.vertex_ids {
        mesh3d_write_u32(token, Mesh3dField::VertexIds, value).unwrap();
    }
    for value in data.edge_positions.as_chunks::<6>().0 {
        mesh3d_write_edge(token, [value[..3].try_into().unwrap(), value[3..].try_into().unwrap()]).unwrap();
    }
    for value in data.edge_ids {
        mesh3d_write_u32(token, Mesh3dField::EdgeIds, value).unwrap();
    }
    for value in data.uvs.as_chunks::<2>().0 {
        ui_wgpu::wgpu::mesh3d_write_vec2(token, Mesh3dField::Uvs, *value).unwrap();
    }
    for value in data.colors.as_chunks::<4>().0 {
        ui_wgpu::wgpu::mesh3d_write_vec4(token, Mesh3dField::Colors, *value).unwrap();
    }
    mesh3d_seal(token).expect("oracle mesh publication")
}

pub(crate) fn with_world_step_context<T>(fuel: u64, step: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> T) -> T {
    let mut sequence = 0;
    let grant=world_original_fixture_grant();
    let mut receipt=semio_framework_value::RetainedCloneProgress::default();
    let mut context = semio_framework_job::StepContext::new(
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(1),
        semio_framework_job::StepBudget::new(fuel, u64::MAX,grant),
        semio_framework_job::root_cancel_token(),
        semio_framework_job::default_now_us,
        &mut sequence,
        &mut receipt,
    );
    let result=step(&mut context);
    assert!(context.retained_progress().fits(grant));
    result
}

include!("🥽️brush-environment-and-hover-laws/🦀️.rs");

include!("🌉️scene-bridge-and-camera-laws/🦀️.rs");

include!("🖼️reference-and-component-laws/🦀️.rs");

include!("🧲️native-gumball-and-primary-laws/🦀️.rs");
