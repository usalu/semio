//! 🌲️ Forest mesh and schema tests of the CAD editor unit tests: the optional field rows, the per-object meshes, the forest reference geometry and the schema checks. See [`super`].

use super::*;

//#region 🔖️Fixtures
/// 🔒️ Wire-format pin: the exact bytes of rows whose `Option` fields make `None`/`Some` distinct
/// wire cases, copied out of the pre-consolidation baseline dump.
#[test]

fn optional_field_rows_keep_their_pre_migration_bytes() {
    let hex = |command: &CadCommand| -> String { protocol::OpBinary::encode_op(command).expect("encode").iter().map(|byte| format!("{byte:02x}")).collect() };
    assert_eq!(hex(&CadCommand::AddObject(add_object::AddObject { typology: Some("spatial.shape.primitive.box".into()) })), "0100011b7370617469616c2e73686170652e7072696d69746976652e626f7801000600");
    assert_eq!(hex(&CadCommand::AddObject(add_object::AddObject { typology: None })), "01000000");
    assert_eq!(
        hex(&CadCommand::PatchObject(patch_object::PatchObject { object_id: "object-1".into(), field: "origin.x".into(), value: Some("1.5".into()), delta: Some(2.5) })),
        "01010303312e35086f626a6563742d31086f726967696e2e780400060101060202060003050000000000000440"
    );
    assert_eq!(hex(&CadCommand::PatchObject(patch_object::PatchObject { object_id: "object-1".into(), field: "origin.x".into(), value: None, delta: None })), "010102086f626a6563742d31086f726967696e2e7802000600010601");
    // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `SetHover`/`WorldPick` and their
    // byte pins are DELETED (those commands no longer exist); every OTHER row's ordinal shifted
    // too (this enum's binary encoding is a plain row-position ordinal — greenfield, no
    // back-compat expected), so the tail-of-enum pins (`EngagementAbort`/`ToggleSun`/
    // `SaveSelected`/`LoadRawRequest`) are dropped rather than hand-recomputed; the exact-wire-key
    // guard for every row (including these) already lives in
    // `every_command_round_trips_text_and_binary_under_its_own_wire_keyword` above.
}

#[semio_framework_async_macros::async_test]
async fn forest_example_uses_per_object_brep_meshes() {
    let scene = forest_working_scene();
    let view = forest_view();
    let json = edit::world_instances_json(&scene.building_objects, &view);
    assert!(json.contains("object-hexagonal-cut-concrete-forest-left-bim-10"));
    let meshes: Vec<serde_json::Value> = serde_json::from_str(&edit::world_meshes_json(&scene.building_objects, scene.building_geometry.as_ref())).expect("meshes json");
    assert!(scene.building_objects.len() > 5);
    assert_eq!(meshes.len(), scene.building_objects.iter().filter(|object| object.visible).count(), "one inline tessellation per visible object");
    for (object, mesh) in scene.building_objects.iter().filter(|object| object.visible).zip(&meshes) {
        assert_eq!(mesh["id"].as_str(), Some(object.id.as_str()), "the instance's meshId must resolve to its own solid, never a shared placeholder kind");
        assert!(mesh.get("kind").is_none(), "a CAD solid is authored geometry, not a renderer-derivable kind");
        assert!(mesh["data"]["positions"].as_array().is_some_and(|positions| positions.len() >= 9), "inline tessellation must carry real triangles");
    }
    assert!(!json.contains("🧊️hexagonal-cut-concrete-forest-left.glb"));
    assert!(scene.building_objects.iter().all(|object| object.solid_handle.is_some()));
}

/// 🗄️ The mesh lane is remembered per genesis scene and tessellated object set: a second render of the same `Arc` and
/// objects answers the same bytes without tessellating, an object edit (a different digest) re-tessellates, and the
/// same objects over another genesis scene (a new `Arc`, a new raw geometry identity) are a miss by construction.
#[semio_framework_async_macros::async_test]
async fn mesh_lane_is_cached_per_materialized_scene() {
    let scene = std::sync::Arc::new(forest_working_scene());
    let objects = &scene.building_objects;
    let geometry = scene.building_geometry.as_ref();
    let started = std::time::Instant::now();
    let first = edit::world_meshes_json_cached(CadPaneId::Building, Some(&scene), objects, geometry);
    let cold = started.elapsed();
    let started = std::time::Instant::now();
    let second = edit::world_meshes_json_cached(CadPaneId::Building, Some(&scene), objects, geometry);
    let warm = started.elapsed();
    assert_eq!(first, second);
    assert_eq!(first, edit::world_meshes_json(objects, geometry), "the cached lane is byte-identical to a fresh tessellation");
    assert!(warm < cold / 4 || warm.as_millis() < 2, "a warm lane never tessellates (cold {cold:?}, warm {warm:?})");
    // ✏️ An edit to a tessellation-relevant field misses: a wider object is a different lane.
    let mut widened = objects.clone();
    widened[0].extent = Some([9.0, 9.0, 9.0]);
    widened[0].solid_handle = None;
    widened[0].primitives.clear();
    let edited = edit::world_meshes_json_cached(CadPaneId::Building, Some(&scene), &widened, geometry);
    assert_ne!(edited, first);
    assert_eq!(edited, edit::world_meshes_json(&widened, geometry));
    // 🪆️ The same objects under a re-minted scene are a miss (a new `Arc` is a new geometry identity)
    // and then a hit again on their own allocation.
    let reminted = std::sync::Arc::new(forest_working_scene());
    let again = edit::world_meshes_json_cached(CadPaneId::Building, Some(&reminted), &reminted.building_objects, reminted.building_geometry.as_ref());
    assert_eq!(again, first);
    let started = std::time::Instant::now();
    let _ = edit::world_meshes_json_cached(CadPaneId::Building, Some(&reminted), &reminted.building_objects, reminted.building_geometry.as_ref());
    assert!(started.elapsed() < cold / 4 || started.elapsed().as_millis() < 2);
    // 🚫️ A pane without a materialized scene renders the fallback roster and is never cached.
    let fallback = edit::world_meshes_json_cached(CadPaneId::Shape, None, &[], None);
    assert!(fallback.contains(CAD_FALLBACK_MESH_KIND));
}

#[semio_framework_async_macros::async_test]
async fn url_backed_objects_share_one_mesh_reference_beside_inline_solids() {
    let scene = forest_working_scene();
    let mut objects = scene.building_objects.clone();
    objects[0].mesh_url = Some("/🧊️assets/column.glb".into());
    objects[1].mesh_url = Some("/🧊️assets/column.glb".into());
    let meshes: Vec<serde_json::Value> = serde_json::from_str(&edit::world_meshes_json(&objects, scene.building_geometry.as_ref())).expect("meshes json");
    let url_entries: Vec<&serde_json::Value> = meshes.iter().filter(|mesh| mesh.get("url").is_some()).collect();
    assert_eq!(url_entries.len(), 1, "one shared reference per asset url");
    assert_eq!(url_entries[0]["id"].as_str(), Some("mesh:column"));
    assert_eq!(meshes.len(), 1 + objects.iter().filter(|object| object.visible).count() - 2, "url-backed objects add no inline tessellation");
    let instances: Vec<serde_json::Value> = serde_json::from_str(&edit::world_instances_json(&objects, &forest_view())).expect("instances json");
    assert_eq!(instances[0]["meshId"].as_str(), Some("mesh:column"));
    assert_eq!(instances[2]["meshId"].as_str(), Some(objects[2].id.as_str()));
}

/// 🎨️ An instance's wire colour is its TYPOLOGY's resolved style, not a hardcoded blue/grey premix:
/// both hosts layer selection/hover on top of it as separate booleans, so a premixed colour double-
/// paints on one target and loses the typology on both (ticket 26/09/17 packet W2f).
#[semio_framework_async_macros::async_test]
async fn world_instances_carry_their_typology_colour_not_a_selection_premix() {
    use semio_s_artifact_cad_cad_typology::resolve_typology_style;
    let scene = forest_working_scene();
    let objects = &scene.building_objects;
    let instances: Vec<serde_json::Value> = serde_json::from_str(&edit::world_instances_json(objects, &forest_view())).expect("instances json");
    let visible: Vec<&semio_s_artifact_cad_cad_model::geometry::CadObject> = objects.iter().filter(|object| object.visible).collect();
    assert_eq!(instances.len(), visible.len());
    for (object, instance) in visible.iter().zip(&instances) {
        assert_eq!(instance["color"].as_str(), Some(resolve_typology_style(&object.typology).color.as_str()), "{}", object.typology);
    }
    assert!(!instances.iter().any(|instance| instance["color"].as_str() == Some("#3b82f6")), "no selection premix survives on the colour lane");
    let selected = view(forest_play_scene(), CadPlayRuntime::default());
    let with_selection: Vec<serde_json::Value> = serde_json::from_str(&edit::world_instances_json(objects, &selected)).expect("instances json");
    assert_eq!(with_selection[0]["color"], instances[0]["color"], "selection never rewrites the colour lane");
}

/// 🧲️ The geometry pick overlay rides the `engagementPreview` lane only while the live session's
/// state accepts a selection, and it is capped so a forest-scale pane can never overrun the lane.
#[semio_framework_async_macros::async_test]
async fn the_pick_overlay_is_bounded_and_only_published_while_a_selection_is_accepted() {
    use semio_s_artifact_cad_cad_interaction::{accepts_selection, start_session};
    let scene = forest_working_scene();
    let document = forest_play_scene();
    let idle = edit::build_world_scene_for_pane(&view(document, CadPlayRuntime::default()), CadPaneId::Building, "cad.play.scene3d/building", None, Default::default()).expect("idle scene");
    let idle_scene: semio_framework_plugin::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene(&semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(idle)).expect("projected")).expect("world scene");
    assert!(idle_scene.engagement_preview_json.is_none(), "no session, no overlay");

    let session = start_session("feature.extrudeWire", CadPaneId::Building).expect("extrude-wire session");
    assert!(accepts_selection(&session), "extrudeWire opens on its `selectWire` selection state");
    assert!(!accepts_selection(&start_session("primitive.box", CadPaneId::Shape).expect("box session")), "a construction interaction that starts on a ground pick publishes no overlay");

    let overlay = edit::pick_target_preview_items(&scene.building_objects, scene.building_geometry.as_ref(), CadPaneId::Building);
    assert!(!overlay.is_empty(), "a selection state publishes the pick overlay");
    assert!(overlay.len() <= edit::CAD_PICK_OVERLAY_ITEM_BUDGET, "the overlay stays inside its budget, got {}", overlay.len());
    for item in &overlay {
        let kind = item.get("kind").and_then(semio_framework_value::DslValue::as_str).expect("item kind");
        assert!(matches!(kind, "point" | "segment"), "{kind} is not a pick-overlay wire kind");
        assert!(item.get("role").and_then(semio_framework_value::DslValue::as_str).is_some_and(|role| role.contains(':')), "every item carries its `kind:id` pick key as its role");
    }
    assert!(overlay.iter().any(|item| item.get("kind").and_then(semio_framework_value::DslValue::as_str) == Some("segment")), "a solid pane draws its members as segments");
    assert!(edit::pick_target_preview_items(&scene.building_objects, None, CadPaneId::Building).is_empty(), "a pane with no kernel geometry offers no overlay");
}

#[semio_framework_async_macros::async_test]
async fn world_fit_revision_follows_the_document_not_object_poses() {
    let document = forest_play_scene();
    let scene = forest_working_scene();
    let base = edit::world_fit_revision(&document, CadPaneId::Building, &scene.building_objects);
    let mut moved = scene.building_objects.clone();
    moved[0].origin = [12.0, -3.0, 0.5];
    assert_eq!(edit::world_fit_revision(&document, CadPaneId::Building, &moved), base, "a transform edit must not re-arm the auto-fit");
    assert_ne!(edit::world_fit_revision(&document, CadPaneId::Energy, &scene.energy_objects), base, "each pane frames its own content");
    let mut other = document.clone();
    other.id = "another-document".into();
    assert_ne!(edit::world_fit_revision(&other, CadPaneId::Building, &scene.building_objects), base, "opening another document frames again");
}

#[semio_framework_async_macros::async_test]
async fn quad_panes_each_populate_distinct_objects() {
    let scene = forest_working_scene();
    assert!(!scene.objects.is_empty(), "shape pane");
    assert!(!scene.building_objects.is_empty(), "building pane");
    assert!(!scene.energy_objects.is_empty(), "energy pane");
    assert!(!scene.structure_classic_objects.is_empty(), "structure classic pane");
}

#[semio_framework_async_macros::async_test]
async fn initial_snapshot_is_cut_concrete_forest_not_placeholder_box() {
    let scene = CadPlayApp::initial_snapshot();
    assert_eq!(scene.id, CAD_EXAMPLE_FOREST_LEFT);
    assert_eq!(scene.nodes.first().map(|node| node.label.as_str()), Some("Concrete Forest Left"), "must not be the placeholder 'Model' node");
    let working = forest_working_scene();
    assert_ne!(working.objects.first().map(|object| object.id.as_str()), Some("object-box-1"));
    assert!(!working.building_objects.is_empty(), "building pane must not be the empty default placeholder");
    assert!(!working.energy_objects.is_empty(), "energy pane must not be the empty default placeholder");
    assert!(!working.structure_classic_objects.is_empty(), "structure pane must not be the empty default placeholder");
    assert!(working.objects.iter().all(|object| object.solid_handle.is_some()));
}

#[semio_framework_async_macros::async_test]
async fn forest_energy_world_mesh_survives_scene_roundtrip() {
    let scene = forest_working_scene();
    let roundtrip: CadWorkingScene = semio_framework_pack_json::from_json_str(&semio_framework_pack_json::to_json_string(&scene), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("deserialize");
    let object = roundtrip.energy_objects.first().expect("energy object");
    let mesh = object_mesh_data(object, roundtrip.energy_geometry.as_ref());
    let min_z = mesh.positions.as_chunks::<3>().0.iter().map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
    assert!(min_z > 2.5, "energy world mesh min z {min_z}");
    let slab = roundtrip.structure_classic_objects.iter().find(|object| object.primitives.iter().any(|primitive| primitive.kind == "surface")).expect("structure surface");
    let slab_mesh = object_mesh_data(slab, roundtrip.structure_classic_geometry.as_ref());
    let slab_min_z = slab_mesh.positions.as_chunks::<3>().0.iter().map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
    assert!(slab_min_z > 2.5, "structure world mesh min z {slab_min_z}");
}

#[semio_framework_async_macros::async_test]
async fn forest_references_use_xy_ground_plane_and_z_up() {
    let scene = forest_play_scene();
    let reference = scene.references_by_model_definition_id.get(CAD_MODEL_DEFINITION_ENERGY).and_then(|references| references.first()).expect("energy reference");
    assert_eq!(reference.origin[2], CAD_FOREST_REFERENCE_PLANE_Z, "reference must stay on the CAD ground datum");
    assert!((reference.origin[0] - (-9.7)).abs() < 1e-9, "reference x {} should be base + 50% width (right)", reference.origin[0]);
    let expected_y = -18.0 + CAD_FOREST_REFERENCE_WIDTH_WORLD * CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX / CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX * (0.5 + CAD_FOREST_REFERENCE_Y_OFFSET_RATIO);
    assert!((reference.origin[1] - expected_y).abs() < 1e-9, "reference CAD y {} should be centered then moved +20% forward on the world plane", reference.origin[1]);
    let centered_y = -18.0 + CAD_FOREST_REFERENCE_WIDTH_WORLD * CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX / CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX * 0.5;
    assert!(((reference.origin[1] - centered_y) - CAD_FOREST_REFERENCE_WIDTH_WORLD * CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX / CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX * 0.2).abs() < 1e-9, "the requested offset must affect CAD y only");
    assert_eq!(CAD_FOREST_REFERENCE_Y_OFFSET_RATIO, 0.2);
    assert!(reference.locked, "example references default locked like puzzle 3d");
    assert_eq!(reference.width_world, 28.6);
}

/// 🖼️ The `/cad-assets` route is the static-dir row in `📦️packages/🦀️rust/Cargo.toml` rooted at
/// `📚️examples/🖼️assets`; a url that misses a file there is answered by the dev server's SPA
/// fallback (`index.html`, 200) and the reference plane silently never paints.
#[semio_framework_async_macros::async_test]
async fn forest_reference_url_names_a_file_under_the_served_assets_root() {
    let relative = CAD_CONCRETE_FOREST_REFERENCE_URL.strip_prefix("/cad-assets/").expect("reference url rides the /cad-assets static-dir route");
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets").join(relative);
    assert!(path.is_file(), "reference asset {} must exist under the served assets root", path.display());
    let scene = forest_play_scene();
    let reference = scene.references_by_model_definition_id.get(CAD_MODEL_DEFINITION_ENERGY).and_then(|references| references.first()).expect("energy reference");
    assert_eq!(reference.source_url, CAD_CONCRETE_FOREST_REFERENCE_URL);
    let published = edit::world_references_json(&scene, CadPaneId::Energy).expect("references lane");
    assert!(published.contains(CAD_CONCRETE_FOREST_REFERENCE_URL), "the world-3d references lane must carry the served url");
}

#[semio_framework_async_macros::async_test]
async fn align_mesh_to_host_snapshot_centroid_corrects_drifted_surface() {
    let scene = forest_working_scene();
    let geometry = scene.energy_geometry.as_ref().expect("energy geometry");
    let object = scene.energy_objects.first().expect("energy object");
    let mut mesh = object_mesh_data(object, Some(geometry));
    for vertex in mesh.positions.as_chunks_mut::<3>().0 {
        vertex[2] = 0.0;
    }
    align_mesh_to_host_snapshot_centroid(&mut mesh, geometry, &object.primitives);
    let min_z = mesh.positions.as_chunks::<3>().0.iter().map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
    assert!(min_z > 2.5, "aligned mesh min z {min_z}");
}

#[semio_framework_async_macros::async_test]
async fn forest_surface_meshes_fall_back_to_typology_extent_without_pane_geometry() {
    // ⚠️ CORRECTED (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
    // wave G4): this test used to assert the mesh stayed at its authored height even with no
    // `CadGeometry` in hand — that only worked because `cad_brep_kernel()` was a process-global
    // `BrepEngineHost` singleton, so `energy.solid_handle` (minted by an EARLIER, already-dropped
    // call to `forest_pane_bundle`) still resolved in whatever kernel `object_mesh_data` happened
    // to reach. `origin` on fixture-derived `CadObject`s is always `[0,0,0]`
    // (`objects_from_host_snapshot_model`) — the authored height lived ONLY in the solid's own vertex
    // data, addressed by that handle. A `cad_brep_kernel()` is now a fresh, local `Brep::new()`
    // per call (doctrine tier-(d): never outlives the call that built it), so a handle from a
    // different call is honestly unresolvable, and — exactly like `mesh_from_glb`'s documented
    // gap elsewhere in this codebase — meshing falls back to the typology's default extent box
    // at the kernel's local origin instead of silently fabricating a placement it cannot know.
    let scene = forest_working_scene();
    let energy = scene.energy_objects.first().expect("energy object");
    let energy_mesh = object_mesh_data(energy, None);
    assert!(!energy_mesh.positions.is_empty(), "energy mesh must still be real geometry, just typology-shaped");
    let slab = scene.structure_classic_objects.iter().find(|object| object.primitives.iter().any(|primitive| primitive.kind == "surface")).expect("structure surface");
    let slab_mesh = object_mesh_data(slab, None);
    assert!(!slab_mesh.positions.is_empty(), "structure slab mesh must still be real geometry, just typology-shaped");
}

#[semio_framework_async_macros::async_test]
async fn cad_artifact_schema_matches_domain() {
    let scene = empty_cad_snapshot();
    assert_eq!(scene.schema, CAD_PLAY_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn default_example_and_forest_scene_parse_as_projections() {
    let default_json = semio_framework_pack_json::to_json_string(&default_document());
    let default_scene: CadSnapshot = semio_framework_pack_json::from_json_str(&default_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(default_scene.schema, CAD_PLAY_DOCUMENT_SCHEMA);
    let forest_json = semio_framework_pack_json::to_json_string(&forest_play_scene());
    let forest_scene: CadSnapshot = semio_framework_pack_json::from_json_str(&forest_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(forest_scene.id, CAD_EXAMPLE_FOREST_LEFT);
    assert!(!forest_working_scene().building_objects.is_empty());
}
//#endregion 🔖️Fixtures
