//! ✏️ CAD play app — the `edit` mode: the quad world-3d layout (shape/building over
//! energy/structure-classic) plus the world-scene, selection-overlay and engagement-HUD builders its
//! four windows share. Each window binds these to its own pane; nothing here is pane-specific.

use crate::editor::cad::config::CadDislocateOptions;
use crate::editor::cad::engine::interaction::{accepts_selection, keyed_transitions, list_interactions_for_model_definition, preview_display_items, state_prompt};
use crate::editor::cad::engine::picking;
use crate::editor::cad::engine::typology::resolve_typology_style;
use crate::editor::cad::modes::edit::windows::{building, energy, shape, structure_classic};
use crate::editor::cad::terminology::CadLabels;
use crate::editor::cad::{cad_pane_camera_runtime, cad_pane_suffix, camera_json, CadPlayView, CAD_DISLOCATE_UTILITY_ID, CAD_FALLBACK_MESH_KIND, CAD_INTERACTION_DOMAIN, CAD_PLAY_APP_ID};
use crate::standards::v1::subsets::any::io::geometry_import::{CadGeometry, CadObject};
use crate::standards::v1::subsets::any::schema::inferences::{object_mesh_data, object_scale_json, resolve_object_mesh_url};
use crate::{CadPaneId, CadSnapshot, CadWorkingScene};
use protocol::DslValue;
use semio_framework_plugin::{
    mesh_from_kind, scene_surface, world3d_environment_json, world3d_fit_json, world3d_mesh_id_from_url, world3d_selection_json, ActionDescriptor, BuiltNode, LocalizedLabel, ModeDefinition, UiAssemblyResult, WindowEngagement, WindowEngagementInput,
    WindowEngagementPossible, WindowEngagementStatus, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode, World3dScene,
};
use std::hash::{Hash, Hasher};

pub const CAD_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::cad::create_cad_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: CAD_PLAY_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ One quadrant of the quad layout: a stack holding a single window kind.
fn cad_window_stack(window_kind_id: &str, title: &str, size: Option<f64>) -> WindowLayoutChild {
    WindowLayoutChild::Stack(WindowLayoutStackNode {
        kind: "stack".into(),
        size,
        active_window_kind_id: None,
        children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: window_kind_id.into(), title: Some(title.into()), instance_id: None, template_id: None, corner: None }],
    })
}

/// @emoji 🪟️ Quad play layout: shape/building left column, energy/structure classic right column.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: vec![
                WindowLayoutChild::Axis(WindowLayoutAxisNode { kind: "column".into(), size: Some(0.5), children: vec![cad_window_stack(shape::WINDOW_KIND_ID, "Shape", Some(0.5)), cad_window_stack(building::WINDOW_KIND_ID, "Building", Some(0.5))] }),
                WindowLayoutChild::Axis(WindowLayoutAxisNode {
                    kind: "column".into(),
                    size: Some(0.5),
                    children: vec![cad_window_stack(energy::WINDOW_KIND_ID, "Energy", Some(0.5)), cad_window_stack(structure_classic::WINDOW_KIND_ID, "Structure Classic", Some(0.5))],
                }),
            ],
        }),
    }
}
//#endregion 🔖️Definition

//#region 🔖️WorldScene
/// 🐁️ Whether the `"cad"` domain's `"pointer"` hover names this object (`CadPlayView::interaction`,
/// resolved per render by `CadPlayApp::render_with_request_context`).
pub fn instance_is_component_hovered(view: &CadPlayView, object_id: &str) -> bool {
    view.interaction.is_hovered(object_id)
}

/// @emoji 🕹️ Whether this window's active Dislocate utility has a visible handle for the selection:
/// the utility is active on this window, at least one of its transforms is enabled and the `"cad"`
/// domain selects at least one of THIS pane's objects (`selected_ids`, already pane-scoped).
pub fn gumball_active(selected_ids: &[String], active_utility: Option<&str>, options: CadDislocateOptions) -> bool {
    active_utility == Some(CAD_DISLOCATE_UTILITY_ID) && (options.move_enabled || options.rotate_enabled) && !selected_ids.is_empty()
}

/// 🌉️ `f64_array_value` — small local helper turning a fixed-size point/vector array into a
/// `DslValue::Array` of floats (mirrors `vec3_json` in `⚙️engine/🕹️interaction/🦀️.rs`).
fn f64_array_value(values: &[f64]) -> DslValue {
    DslValue::Array(values.iter().map(|v| DslValue::float(*v)).collect())
}

/// 🎯️ Framing margin around the pane's content on the first delivery of a document (`world3d_fit_json`).
pub const CAD_FIT_PADDING: f64 = 1.25;

/// 🥽️ The granularity a world-3d pane pick selects under in the `"cad"` interaction domain.
pub const CAD_WORLD_PICK_GRANULARITY: &str = "object";

/// 🌉️ `MeshData` (`semio_framework_plugin`) carries its own first-party `From<MeshData> for
/// pack::json::Value` — reached here through `protocol`'s `os_pack` re-export of the same `pack`
/// crate, never `serde_json`. Bridged once, here, at the point each mesh payload is assembled.
fn mesh_data_to_dsl(data: &semio_framework_plugin::MeshData) -> DslValue {
    protocol::os_pack::json::to_dsl_value(&protocol::os_pack::json::Value::from(data.clone()))
}

/// 🥽️ Mesh id an instance references — a URL-backed asset keeps its stable `mesh:{slug}` id (one
/// shared `{ id, url }` entry per asset); every other object owns its own tessellation under its
/// object id (`world_meshes_json` inlines the BREP mesh as `{ id, data }`).
fn instance_mesh_id(object: &CadObject) -> String {
    resolve_object_mesh_url(object).map_or_else(|| object.id.clone(), |url| world3d_mesh_id_from_url(&url))
}

pub(crate) fn world_instances_json(objects: &[CadObject], view: &CadPlayView) -> String {
    let instances: Vec<DslValue> = objects
        .iter()
        .filter(|object| object.visible)
        .map(|object| {
            let mesh_id = instance_mesh_id(object);
            let selected = view.interaction.is_selected(&object.id);
            let hovered = instance_is_component_hovered(view, &object.id);
            DslValue::object([
                ("id".to_string(), DslValue::String(object.id.clone())),
                ("meshId".to_string(), DslValue::String(mesh_id)),
                ("position".to_string(), f64_array_value(&object.origin)),
                ("rotation".to_string(), f64_array_value(&object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]))),
                ("scale".to_string(), f64_array_value(&object_scale_json(object))),
                ("label".to_string(), DslValue::String(object.label.clone())),
                ("color".to_string(), DslValue::String(resolve_typology_style(&object.typology).color)),
                ("selected".to_string(), DslValue::Bool(selected)),
                ("hovered".to_string(), DslValue::Bool(hovered)),
            ])
        })
        .collect();
    protocol::json::to_json_string(&instances)
}

/// 🥽️ The pane's mesh roster: one `{ id, url }` reference per URL-backed asset and one inline
/// `{ id, data }` tessellation per kernel-backed object — a CAD solid is authored geometry no
/// renderer can derive from a kind, so its triangles ride the paged `meshes` lane (bounded by carrier
/// pages, not by the 32 KiB surface spine). An empty pane keeps the fallback kind so the viewport
/// still has a mesh to frame.
pub(crate) fn world_meshes_json(objects: &[CadObject], geometry: Option<&CadGeometry>) -> String {
    let mut meshes: Vec<DslValue> = Vec::new();
    let mut url_ids: Vec<String> = Vec::new();
    for object in objects.iter().filter(|object| object.visible) {
        match resolve_object_mesh_url(object) {
            Some(url) => {
                let id = world3d_mesh_id_from_url(&url);
                if url_ids.contains(&id) {
                    continue;
                }
                meshes.push(DslValue::object([("id".to_string(), DslValue::String(id.clone())), ("url".to_string(), DslValue::String(url))]));
                url_ids.push(id);
            }
            None => {
                let data = object_mesh_data(object, geometry);
                meshes.push(DslValue::object([("id".to_string(), DslValue::String(object.id.clone())), ("data".to_string(), mesh_data_to_dsl(&data))]));
            }
        }
    }
    if meshes.is_empty() {
        let data = mesh_from_kind(CAD_FALLBACK_MESH_KIND);
        meshes.push(DslValue::object([("id".to_string(), DslValue::String(CAD_FALLBACK_MESH_KIND.to_string())), ("data".to_string(), mesh_data_to_dsl(&data))]));
    }
    protocol::json::to_json_string(&meshes)
}

//#region 🔖️MeshLaneCache
/// 🗄️ The pane's mesh lane, remembered per materialized working scene. `world_meshes_json`
/// re-tessellates every kernel-backed object from the host snapshot on EVERY render — and a hover
/// renders all four panes (the four scenes share one interaction domain), so one pointer move cost
/// four full re-tessellations of the Concrete Forest (≈40–80 ms per pane inside the guest) for a
/// lane whose bytes had not changed. The lane only changes when the pane's objects or its geometry
/// change, and both live in the child's immutable `Arc<CadWorkingScene>` materialization: a scene is
/// re-minted on every object edit (`cad_pane_rematerialized_child`), so the `Arc` allocation is the
/// geometry's identity. The entry holds a `Weak` to that allocation — an `Arc` allocation is not
/// freed while a `Weak` points at it, so its address cannot be reused by a later scene and a stale
/// hit is impossible — plus a digest of every object field the tessellation reads.
struct MeshLaneCacheEntry {
    pane: CadPaneId,
    scene: std::sync::Weak<CadWorkingScene>,
    objects_digest: u64,
    json: String,
}

/// 🗄️ Four panes, each with its live scene plus the one it just left (a commit re-mints the scene
/// and the old one may still be rendered once by a lagging refresh).
const MESH_LANE_CACHE_CAPACITY: usize = 8;

static MESH_LANE_CACHE: std::sync::Mutex<Vec<MeshLaneCacheEntry>> = std::sync::Mutex::new(Vec::new());

/// 🔏️ Every object field `object_mesh_data` reads (plus visibility, which decides membership).
fn mesh_lane_objects_digest(objects: &[CadObject]) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for object in objects {
        object.id.hash(&mut hasher);
        object.visible.hash(&mut hasher);
        object.typology.hash(&mut hasher);
        object.mesh_url.hash(&mut hasher);
        object.solid_handle.hash(&mut hasher);
        object.extent.map(|extent| extent.map(f64::to_bits)).hash(&mut hasher);
        for primitive in &object.primitives {
            primitive.slot.hash(&mut hasher);
            primitive.primitive_id.hash(&mut hasher);
            primitive.kind.hash(&mut hasher);
        }
    }
    hasher.finish()
}

/// 🗄️ `world_meshes_json` behind the per-scene cache. A pane without a materialized scene renders
/// the fallback roster directly (it is one built-in mesh and never worth an entry).
pub(crate) fn world_meshes_json_cached(pane: CadPaneId, scene: Option<&std::sync::Arc<CadWorkingScene>>, objects: &[CadObject], geometry: Option<&CadGeometry>) -> String {
    let Some(scene) = scene else {
        return world_meshes_json(objects, geometry);
    };
    let objects_digest = mesh_lane_objects_digest(objects);
    let scene_ptr = std::sync::Arc::as_ptr(scene);
    if let Ok(cache) = MESH_LANE_CACHE.lock() {
        if let Some(entry) = cache.iter().find(|entry| entry.pane == pane && entry.objects_digest == objects_digest && std::ptr::eq(entry.scene.as_ptr(), scene_ptr)) {
            return entry.json.clone();
        }
    }
    let json = world_meshes_json(objects, geometry);
    if let Ok(mut cache) = MESH_LANE_CACHE.lock() {
        cache.retain(|entry| entry.pane != pane || entry.scene.strong_count() > 0);
        if cache.len() >= MESH_LANE_CACHE_CAPACITY {
            cache.remove(0);
        }
        cache.push(MeshLaneCacheEntry { pane, scene: std::sync::Arc::downgrade(scene), objects_digest, json: json.clone() });
    }
    json
}
//#endregion 🔖️MeshLaneCache

/// 🎯️ Document identity for the pane's auto-fit: the host frames the content once per revision and
/// never takes a user-moved camera back, so this must follow the DOCUMENT (which example is open and
/// which objects its pane holds), not the object poses a transform edits.
pub(crate) fn world_fit_revision(document: &CadSnapshot, pane: CadPaneId, objects: &[CadObject]) -> u32 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    document.id.hash(&mut hasher);
    pane.model_definition_id().hash(&mut hasher);
    for object in objects {
        object.id.hash(&mut hasher);
    }
    (hasher.finish() >> 32) as u32
}

/// 🌉️ Insert-or-overwrite into a `DslValue::Object`'s entry list — `DslValue::Object` is a plain
/// `Vec<(String, DslValue)>` (no `Map`-like `.insert`), so this is the mutable-upsert primitive
/// every JSON-mutation site in this file shares.
fn dsl_object_upsert(entries: &mut Vec<(String, DslValue)>, key: &str, value: DslValue) {
    if let Some(existing) = entries.iter_mut().find(|(k, _)| k == key) {
        existing.1 = value;
    } else {
        entries.push((key.to_string(), value));
    }
}

/// 🕹️ The scene's selection lane: the `"cad"` domain's selected/hovered ids RESTRICTED TO THIS PANE's
/// objects, plus the dislocate gumball and engagement flags. The four panes share one domain, but
/// `World3dHost` reads the lane's `hoveredId` as "the target THIS pane is publishing" (its
/// background click re-picks it instead of clearing), so a hover that lives in another pane must not
/// leak into this one's lane.
pub(crate) fn world_selection_json(view: &CadPlayView, pane: CadPaneId, objects: &[CadObject], active_utility: Option<&str>, options: CadDislocateOptions) -> String {
    let runtime = &view.runtime;
    let owned = |id: &String| objects.iter().any(|object| &object.id == id);
    let ids: Vec<String> = view.interaction.ids.iter().filter(|id| owned(id)).cloned().collect();
    let hovered = view.interaction.hovered_ids.iter().find(|id| owned(id)).map(String::as_str);
    let mut value: DslValue = protocol::json::from_json_str(&world3d_selection_json("rectangle", &ids, hovered)).unwrap_or_else(|_| DslValue::object(Vec::new()));
    if let DslValue::Object(entries) = &mut value {
        let active = gumball_active(&ids, active_utility, options);
        if active_utility == Some(CAD_DISLOCATE_UTILITY_ID) {
            dsl_object_upsert(entries, "transformMode", DslValue::String("transform".into()));
            dsl_object_upsert(
                entries,
                "gumballConfig",
                DslValue::object([
                    ("moveAxes".to_string(), DslValue::Bool(options.move_enabled)),
                    ("movePlanes".to_string(), DslValue::Bool(options.move_enabled)),
                    ("rotate".to_string(), DslValue::Bool(options.rotate_enabled)),
                    ("scaleAxes".to_string(), DslValue::Bool(false)),
                    ("scalePlanes".to_string(), DslValue::Bool(false)),
                    ("scaleUniform".to_string(), DslValue::Bool(false)),
                ]),
            );
        }
        dsl_object_upsert(entries, "gumballActive", DslValue::Bool(active));
        // 🤝️ Only the pane that owns the live session takes world-pointer events (`World3dHost` turns
        // `engagementSessionActive` into `worldPointerDown`/`worldPointerMove` dispatches).
        dsl_object_upsert(entries, "engagementSessionActive", DslValue::Bool(runtime.engagement_session.as_ref().is_some_and(|session| session.pane == pane)));
        dsl_object_upsert(entries, "showEdges", DslValue::Bool(true));
        // 🖼️ Every pane's reference overlay reuses the same reference id (one `ref-concrete-forest` per
        // model definition), so the app-owned reference selection is stamped only on the pane whose
        // model definition it names — never on the three siblings sharing the id.
        if let (Some(model_definition_id), Some(reference_id)) = (runtime.selected_reference_model_definition_id.as_deref(), runtime.selected_reference_id.as_deref()) {
            if model_definition_id == pane.model_definition_id() {
                dsl_object_upsert(entries, "referenceSelectedId", DslValue::String(reference_id.to_string()));
            }
        }
    }
    protocol::json::to_json_string(&value)
}

pub fn world_references_json(document: &CadSnapshot, pane: CadPaneId) -> Option<String> {
    let references = document.references_by_model_definition_id.get(pane.model_definition_id())?;
    if references.is_empty() {
        return None;
    }
    let records: Vec<DslValue> = references
        .iter()
        .filter(|reference| !reference.hidden)
        .map(|reference| {
            DslValue::object([
                ("id".to_string(), DslValue::String(reference.id.clone())),
                ("url".to_string(), DslValue::String(reference.source_url.clone())),
                ("origin".to_string(), f64_array_value(&reference.origin)),
                ("widthWorld".to_string(), DslValue::float(if reference.width_world > 0.0 { reference.width_world } else { 1.0 })),
                ("locked".to_string(), DslValue::Bool(reference.locked)),
                ("hidden".to_string(), DslValue::Bool(reference.hidden)),
                ("opacity".to_string(), DslValue::float(reference.opacity.unwrap_or(1.0))),
            ])
        })
        .collect();
    Some(protocol::json::to_json_string(&records))
}

/// 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: a pane's object/geometry data
/// lives inside its composed `s.stdio.semio.model` CHILD document now — no host-level child
/// resolver exists yet (see `🔖️Composition` in `🏪️store/🦀️.rs`), but the handle's own
/// `ArtifactChild::local_owner` (the same in-process materialization seam `flow`/`dag`/`jack`/
/// `wires`/`sequence` already rely on) carries the `CadWorkingScene` a document builder such as
/// `forest_play_document` attached when it minted the handle. `pane`'s objects/geometry come from
/// there; a handle with no local owner (or none at all) renders an empty pane, never a fabricated one.
pub(crate) fn cad_pane_working_scene(document: &CadSnapshot, pane: CadPaneId) -> Option<std::sync::Arc<CadWorkingScene>> {
    let child = match pane {
        CadPaneId::Shape => document.shape_model.as_ref(),
        CadPaneId::Building => document.building_model.as_ref(),
        CadPaneId::Energy => document.energy_model.as_ref(),
        CadPaneId::StructureClassic => document.structure_classic_model.as_ref(),
    }?;
    child.local_owner::<CadWorkingScene>()
}

pub(crate) fn cad_pane_working_objects(scene: &CadWorkingScene, pane: CadPaneId) -> (&[CadObject], Option<&CadGeometry>) {
    match pane {
        CadPaneId::Shape => (&scene.objects, scene.geometry.as_ref()),
        CadPaneId::Building => (&scene.building_objects, scene.building_geometry.as_ref()),
        CadPaneId::Energy => (&scene.energy_objects, scene.energy_geometry.as_ref()),
        CadPaneId::StructureClassic => (&scene.structure_classic_objects, scene.structure_classic_geometry.as_ref()),
    }
}

/// 🧲️ How many pick-target preview items one pane may publish per frame. The `engagementPreview`
/// lane is a bounded scene payload, and a Concrete-Forest pane offers thousands of kernel targets,
/// so the overlay is capped and the coarsest kinds (object → face → edge → vertex, the pick
/// generality order both hosts sort by) are kept first.
pub const CAD_PICK_OVERLAY_ITEM_BUDGET: usize = 192;

/// 🧲️ The geometry pick overlay as `engagementPreview` items — the wire-level equivalent of React's
/// `SpatialPickGeometryLayer`, which has no counterpart on the OS-shell path on either target.
/// Vertex targets become preview `point`s, every other kind becomes `segment`s along the entity's
/// own straight edges (a face/solid therefore draws as its wireframe, exactly as React does).
pub(crate) fn pick_target_preview_items(objects: &[CadObject], geometry: Option<&CadGeometry>, pane: CadPaneId) -> Vec<DslValue> {
    let Some(geometry) = geometry else { return Vec::new() };
    let model_definition_id = pane.model_definition_id();
    let targets = picking::create_spatial_pick_targets(objects, Some(geometry), Some(model_definition_id));
    let targets = picking::filter_spatial_pick_targets_for_active_view(targets, Some(model_definition_id));
    let visibility = picking::spatial_scene_kind_toggles_for_model_definition(Some(model_definition_id), &picking::default_spatial_primitive_toggles());
    let mut targets = picking::filter_spatial_pick_targets_for_visibility(targets, visibility);
    let flags = |entity_id: &str| picking::resolve_spatial_entity_flags(objects, geometry, model_definition_id, entity_id);
    targets = picking::filter_spatial_pick_targets_for_entity_flags(targets, &flags);
    targets.sort_by_key(|target| target.kind.generality());
    let buckets = picking::geometry_buckets(geometry);
    let mut items: Vec<DslValue> = Vec::new();
    for target in targets {
        if items.len() >= CAD_PICK_OVERLAY_ITEM_BUDGET {
            break;
        }
        let role = DslValue::String(picking::spatial_pick_target_key(&target));
        if target.kind == picking::SpatialPickTargetKind::Vertex {
            items.push(DslValue::object([("kind".to_string(), DslValue::String("point".into())), ("role".to_string(), role), ("position".to_string(), f64_array_value(&target.point))]));
            continue;
        }
        let Some(geometry_kind) = picking::pick_target_primitive_kind(&target) else {
            items.push(DslValue::object([("kind".to_string(), DslValue::String("point".into())), ("role".to_string(), role), ("position".to_string(), f64_array_value(&target.point))]));
            continue;
        };
        for (from, to) in buckets.entity_wire_segments(geometry_kind, &target.id) {
            if items.len() >= CAD_PICK_OVERLAY_ITEM_BUDGET {
                break;
            }
            items.push(DslValue::object([("kind".to_string(), DslValue::String("segment".into())), ("role".to_string(), role.clone()), ("from".to_string(), f64_array_value(&from)), ("to".to_string(), f64_array_value(&to))]));
        }
    }
    items
}

pub fn build_world_scene_for_pane(envelope: &CadPlayView, pane: CadPaneId, surface_id: &str, active_utility: Option<&str>, options: CadDislocateOptions) -> UiAssemblyResult<BuiltNode> {
    let working_scene = cad_pane_working_scene(&envelope.document, pane);
    let empty: &[CadObject] = &[];
    let (objects, geometry) = working_scene.as_deref().map_or((empty, None), |scene| cad_pane_working_objects(scene, pane));
    let mut scene = World3dScene::base(
        camera_json(cad_pane_camera_runtime(&envelope.runtime, pane)),
        world_meshes_json_cached(pane, working_scene.as_ref(), objects, geometry),
        world_instances_json(objects, envelope),
        world_selection_json(envelope, pane, objects, active_utility, options),
    );
    scene.references_json = world_references_json(&envelope.document, pane);
    // 🤝️ The live construction preview (rubber-band points/segments/box/height handle) of the
    // session this pane owns — the statechart's own `display` items for its current state, plus the
    // geometry pick overlay while that state is waiting for a selection.
    scene.engagement_preview_json = envelope.runtime.engagement_session.as_ref().filter(|session| session.pane == pane).map(|session| {
        let mut items = preview_display_items(session);
        if accepts_selection(session) {
            items.extend(pick_target_preview_items(objects, geometry, pane));
        }
        protocol::json::to_json_string(&DslValue::Array(items))
    });
    scene.environment_json = Some(world3d_environment_json(&envelope.runtime.sun));
    scene.fit_json = Some(world3d_fit_json(world_fit_revision(&envelope.document, pane, objects), CAD_FIT_PADDING, None));
    // 🕹️ Bound to the framework-owned `"cad"` domain so `World3dHost` dispatches `interactionSelect`/
    // `interactionHover` (which `handle` reads back through `InteractionView`) instead of the legacy
    // `worldPick`/`worldSelect`/`setHover` verbs this app has no handler for.
    scene.domain_id = Some(CAD_INTERACTION_DOMAIN.into());
    scene.domain_granularity_id = Some(CAD_WORLD_PICK_GRANULARITY.into());
    scene_surface(surface_id, semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::World3d, &scene)
}
//#endregion 🔖️WorldScene

//#region 🔖️Engagement
fn cad_action(action: &str, args: Option<DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: CAD_PLAY_APP_ID.into(), action: action.into(), args }
}

pub fn cad_window_engagement(envelope: &CadPlayView, pane: CadPaneId, labels: &CadLabels) -> WindowEngagement {
    // 🕹️ `window_engagements_with_request_context` threads the live `"cad"` domain in, so this is the
    // same selection the world scenes paint — see `CadPlayApp::window_engagements_body`.
    let selected_count = envelope.interaction.ids.len();
    let model_definition_id = pane.model_definition_id();
    // 🤝️ One session at a time, owned by one pane: its HUD shows the keyed transitions; every other
    // pane keeps offering its own model definition's interactions.
    let pane_session = envelope.runtime.engagement_session.as_ref().filter(|session| session.pane == pane);
    let session_active = pane_session.is_some();
    let possible_engagements: Vec<WindowEngagementPossible> = if let Some(session) = pane_session {
        keyed_transitions(session)
            .into_iter()
            .map(|transition| WindowEngagementPossible {
                id: transition.event_kind.clone(),
                label: transition.label,
                detail: Some(transition.key),
                action: Some(cad_action("engagementPossibleSelect", Some(DslValue::object([("pane".to_string(), DslValue::String(cad_pane_suffix(pane).to_string())), ("possibleId".to_string(), DslValue::String(transition.event_kind))])))),
            })
            .collect()
    } else {
        list_interactions_for_model_definition(model_definition_id)
            .into_iter()
            .map(|entry| WindowEngagementPossible {
                id: entry.id.clone(),
                label: entry.label.clone(),
                detail: Some(entry.key.clone()),
                action: Some(cad_action("engagementPossibleSelect", Some(DslValue::object([("pane".to_string(), DslValue::String(cad_pane_suffix(pane).to_string())), ("possibleId".to_string(), DslValue::String(entry.id.clone()))])))),
            })
            .collect()
    };
    let step_text = pane_session.map_or_else(|| envelope.runtime.engagement_step.clone(), state_prompt);
    WindowEngagement {
        session_active: Some(session_active),
        // 🧰️ The move/rotate/scale transform switcher now lives in the framework utility bar (derived
        // from `UtilityDefinition`s + `ViewModel::active_utility_id`); the engagement HUD no longer
        // duplicates it — utilities must have exactly one surface.
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("engagement-input".into()),
            value: Some(envelope.runtime.engagement_input.clone()),
            placeholder: Some(labels.action_placeholder.into()),
            disabled: None,
            on_change: Some(cad_action("engagementInput", Some(DslValue::object([("pane".to_string(), DslValue::String(cad_pane_suffix(pane).to_string()))])))),
            on_submit: Some(cad_action("engagementSubmit", Some(DslValue::object([("pane".to_string(), DslValue::String(cad_pane_suffix(pane).to_string()))])))),
            on_repeat_last: Some(cad_action("engagementRepeatLast", Some(DslValue::object([("pane".to_string(), DslValue::String(cad_pane_suffix(pane).to_string()))])))),
            on_abort: Some(cad_action("engagementAbort", Some(DslValue::object([("pane".to_string(), DslValue::String(cad_pane_suffix(pane).to_string()))])))),
        }),
        control: None,
        controls: None,
        status: Some(vec![
            WindowEngagementStatus { id: "cad-status".into(), text: format!("{selected_count} {}", labels.selected.as_str()) },
            WindowEngagementStatus { id: "cad-step".into(), text: format!("{}: {step_text}", labels.step.as_str()) },
            WindowEngagementStatus { id: "cad-response".into(), text: envelope.runtime.engagement_session.as_ref().and_then(|session| session.last_response.clone()).unwrap_or_else(|| labels.ok.into()) },
        ]),
        possible_engagements: Some(possible_engagements),
    }
}
//#endregion 🔖️Engagement
