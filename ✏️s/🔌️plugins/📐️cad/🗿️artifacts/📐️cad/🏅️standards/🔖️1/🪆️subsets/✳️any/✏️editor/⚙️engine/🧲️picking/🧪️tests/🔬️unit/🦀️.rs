//! 🧪️ Laws for the CAD spatial pick/selection engine. Every `/// 🧪️ React parity:` case names the
//! `it(...)` it mirrors in `⚙️engine/📺️renderer/🧪️tests/🧪️repluserfacingsuggestiondetail/🟦️.tsx`, and
//! the box fixture below is the same `cornerA [0,0,0] / cornerB [1,1,0] / height 1` box that suite
//! feeds `boxModelDiff` — rebuilt here as authored `CadGeometry` because this target has no
//! `applyModelDiff`.

use super::*;
use crate::standards::v1::subsets::any::io::geometry_import::{CadEdgeCurve, CadPlaneSurface, CadPrimitiveSlot};

//#region 🧫️Fixtures
fn vertex(id: &str, position: [f64; 3]) -> CadVertex {
    CadVertex { id: id.to_string(), position }
}

fn edge(id: &str, a: &str, b: &str) -> CadEdge {
    CadEdge { id: id.to_string(), vertex_ids: vec![a.to_string(), b.to_string()], curve: CadEdgeCurve { kind: "line".into() } }
}

fn wire(id: &str, edge_ids: &[&str]) -> CadWire {
    CadWire { id: id.to_string(), edge_ids: edge_ids.iter().map(|id| (*id).to_string()).collect() }
}

fn face(id: &str, wire_ids: &[&str]) -> CadFace {
    CadFace { id: id.to_string(), wire_ids: wire_ids.iter().map(|id| (*id).to_string()).collect(), surface: CadPlaneSurface { kind: "plane".into(), origin: [0.0; 3], normal: [0.0, 0.0, 1.0] } }
}

fn object(id: &str, typology: &str, slot: &str, primitive_id: &str, kind: &str) -> CadObject {
    CadObject {
        id: id.to_string(),
        label: id.to_string(),
        typology: typology.to_string(),
        visible: true,
        locked: false,
        origin: [0.0; 3],
        orientation: None,
        scale: None,
        mesh_url: None,
        extent: None,
        solid_handle: None,
        primitives: vec![CadPrimitiveSlot { slot: slot.to_string(), primitive_id: primitive_id.to_string(), kind: kind.to_string() }],
    }
}

/// 🧫️ A unit box as authored kernel geometry: 8 vertices, 12 edges, 6 wires, 6 faces, 1 shell, 1 solid.
fn box_geometry() -> CadGeometry {
    let corners: [[f64; 3]; 8] = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]];
    let vertices: Vec<CadVertex> = corners.iter().enumerate().map(|(index, position)| vertex(&format!("v{index}"), *position)).collect();
    let edge_pairs: [(usize, usize); 12] = [(0, 1), (1, 2), (2, 3), (3, 0), (4, 5), (5, 6), (6, 7), (7, 4), (0, 4), (1, 5), (2, 6), (3, 7)];
    let edges: Vec<CadEdge> = edge_pairs.iter().enumerate().map(|(index, (a, b))| edge(&format!("e{index}"), &format!("v{a}"), &format!("v{b}"))).collect();
    let wire_edges: [[usize; 4]; 6] = [[0, 1, 2, 3], [4, 5, 6, 7], [0, 9, 4, 8], [1, 10, 5, 9], [2, 11, 6, 10], [3, 8, 7, 11]];
    let wires: Vec<CadWire> = wire_edges.iter().enumerate().map(|(index, ids)| wire(&format!("w{index}"), &ids.iter().map(|id| format!("e{id}")).collect::<Vec<_>>().iter().map(String::as_str).collect::<Vec<_>>())).collect();
    let faces: Vec<CadFace> = (0..6).map(|index| face(&format!("f{index}"), &[&format!("w{index}")])).collect();
    CadGeometry {
        anchors: Vec::new(),
        vertices,
        edges,
        wires,
        faces,
        shells: vec![CadShell { id: "sh0".into(), face_ids: (0..6).map(|index| format!("f{index}")).collect() }],
        solids: vec![CadSolid { id: "c0".into(), shell_ids: vec!["sh0".into()] }],
    }
}

fn target(kind: SpatialPickTargetKind, id: &str, geometry_kind: Option<ModelEntityKind>, typology_id: Option<&str>) -> SpatialPickTarget {
    SpatialPickTarget { kind, id: id.to_string(), point: [0.0; 3], points: Vec::new(), geometry_kind, typology_id: typology_id.map(str::to_string) }
}

fn keys(targets: &[SpatialPickTarget]) -> Vec<String> {
    targets.iter().map(spatial_pick_target_key).collect()
}
//#endregion 🧫️Fixtures

//#region 🔖️KindMapping
#[test]
fn pick_kinds_carry_generality_coarsest_first() {
    let mut kinds = SPATIAL_PICK_TARGET_KINDS.to_vec();
    kinds.sort_by_key(|kind| kind.generality());
    assert_eq!(kinds, vec![SpatialPickTargetKind::Object, SpatialPickTargetKind::Face, SpatialPickTargetKind::Edge, SpatialPickTargetKind::Vertex]);
    assert_eq!(SpatialPickTargetKind::parse("face"), Some(SpatialPickTargetKind::Face));
    assert_eq!(SpatialPickTargetKind::parse("solid"), None);
}

#[test]
fn kernel_kinds_collapse_onto_the_four_pick_kinds() {
    assert_eq!(geometry_kind_to_object_pick(ModelEntityKind::Anchor), Some(SpatialPickTargetKind::Vertex));
    assert_eq!(geometry_kind_to_object_pick(ModelEntityKind::Wire), Some(SpatialPickTargetKind::Edge));
    assert_eq!(geometry_kind_to_object_pick(ModelEntityKind::Shell), Some(SpatialPickTargetKind::Face));
    assert_eq!(geometry_kind_to_object_pick(ModelEntityKind::Solid), Some(SpatialPickTargetKind::Object));
    assert_eq!(geometry_kind_to_object_pick(ModelEntityKind::Object), None);
}

#[test]
fn selection_accept_widens_kernel_kinds_to_pick_kinds() {
    assert_eq!(spatial_pick_kinds_for_selection_accept(&[]), None);
    let accepted = spatial_pick_kinds_for_selection_accept(&[ModelEntityKind::Wire, ModelEntityKind::Shell]).expect("some");
    assert!(accepted.contains(&SpatialPickTargetKind::Edge));
    assert!(accepted.contains(&SpatialPickTargetKind::Face));
    assert!(!accepted.contains(&SpatialPickTargetKind::Object));
}

/// 🧪️ React parity: the typology object row is the ONLY target with no primitive kind.
#[test]
fn typology_object_rows_have_no_primitive_kind() {
    assert_eq!(pick_target_primitive_kind(&target(SpatialPickTargetKind::Object, "hull", None, None)), None);
    assert_eq!(pick_target_primitive_kind(&target(SpatialPickTargetKind::Object, "c0", Some(ModelEntityKind::Solid), None)), Some(ModelEntityKind::Solid));
    assert_eq!(pick_target_primitive_kind(&target(SpatialPickTargetKind::Edge, "w0", Some(ModelEntityKind::Wire), None)), Some(ModelEntityKind::Wire));
}
//#endregion 🔖️KindMapping

//#region 🔖️Targets
/// 🧪️ React parity: `creates snap and selection metadata for geometry targets` /
/// `creates anchor and shell pick targets for spatial.shape geometry`.
#[test]
fn shape_geometry_offers_every_kernel_kind_as_a_pick_target() {
    let geometry = box_geometry();
    let targets = create_spatial_pick_targets(&[], Some(&geometry), Some("spatial.shape"));
    let by_geometry_kind = |kind: ModelEntityKind| targets.iter().filter(|row| row.geometry_kind == Some(kind)).count();
    assert_eq!(by_geometry_kind(ModelEntityKind::Vertex), 8);
    assert_eq!(by_geometry_kind(ModelEntityKind::Edge), 12);
    assert_eq!(by_geometry_kind(ModelEntityKind::Wire), 6);
    assert_eq!(by_geometry_kind(ModelEntityKind::Face), 6);
    assert_eq!(by_geometry_kind(ModelEntityKind::Shell), 1);
    assert_eq!(by_geometry_kind(ModelEntityKind::Solid), 1);
    assert!(targets.iter().all(|row| row.typology_id.is_none()), "shape geometry has no typology object rows");
    let solid = targets.iter().find(|row| row.geometry_kind == Some(ModelEntityKind::Solid)).expect("solid target");
    assert_eq!(solid.kind, SpatialPickTargetKind::Object);
    assert_eq!(solid.point, [0.5, 0.5, 0.5]);
}

/// 🧪️ React parity: `adds typology object picks for non-shape model definitions` — and the object's
/// own primary solid is skipped so the row and its solid never both offer the same pick.
#[test]
fn typology_model_definitions_add_object_rows_and_skip_the_owned_solid() {
    let geometry = box_geometry();
    let objects = vec![object("hull", "energy.energy.hull", "solid", "c0", "solid")];
    let targets = create_spatial_pick_targets(&objects, Some(&geometry), Some("aec.building.energy"));
    assert!(keys(&targets).contains(&"object:hull".to_string()));
    assert!(!keys(&targets).contains(&"object:c0".to_string()), "the owned solid must not double as an object pick");
    let hull = targets.iter().find(|row| row.id == "hull").expect("hull row");
    assert_eq!(hull.typology_id.as_deref(), Some("energy.energy.hull"));
    assert_eq!(hull.geometry_kind, None);
    let a_face = targets.iter().find(|row| row.geometry_kind == Some(ModelEntityKind::Face)).expect("face row");
    assert_eq!(a_face.typology_id.as_deref(), Some("energy.energy.hull"), "solid members inherit the owning row's typology");
}

#[test]
fn no_geometry_means_no_targets() {
    assert!(create_spatial_pick_targets(&[], None, Some("spatial.shape")).is_empty());
    assert!(create_spatial_pick_targets(&[], Some(&CadGeometry::default()), Some("spatial.shape")).is_empty());
}

/// 🧪️ React parity: `filterSpatialPickTargets matches primitive geometryKind in selection accept`.
#[test]
fn selection_accept_matches_the_primitive_geometry_kind() {
    let targets = vec![target(SpatialPickTargetKind::Edge, "w0", Some(ModelEntityKind::Wire), None), target(SpatialPickTargetKind::Edge, "e0", Some(ModelEntityKind::Edge), None)];
    let kept = filter_spatial_pick_targets(targets.clone(), &[ModelEntityKind::Wire], SpatialPickKindToggles::default());
    assert_eq!(keys(&kept), vec!["edge:w0", "edge:e0"], "accept widens to the pick kind, so both edges stay");
    let mut toggles = SpatialPickKindToggles::default();
    toggles.edge = Some(false);
    assert!(filter_spatial_pick_targets(targets, &[ModelEntityKind::Wire], toggles).is_empty());
}

/// 🧪️ React parity: `createSpatialPickEvent` carries snap + selection when a target is hit.
#[test]
fn pick_event_carries_snap_and_selection_for_a_hit() {
    let hit = target(SpatialPickTargetKind::Face, "f0", Some(ModelEntityKind::Face), None);
    let event = create_spatial_pick_event(SpatialPickKind::PointerDown, [1.0, 2.0, 3.0], Some(&hit), &[("shift", true)]);
    assert_eq!(event.get("kind").and_then(DslValue::as_str), Some("pointer.down"));
    assert_eq!(event.get("snap").and_then(|snap| snap.get("kind")).and_then(DslValue::as_str), Some("face"));
    assert_eq!(event.get("selection").and_then(|row| row.get("id")).and_then(DslValue::as_str), Some("f0"));
    assert_eq!(event.get("modifiers").and_then(|row| row.get("shift")).and_then(DslValue::as_bool), Some(true));

    let object_hit = target(SpatialPickTargetKind::Object, "hull", None, None);
    let object_event = create_spatial_pick_event(SpatialPickKind::PointerMove, [0.0; 3], Some(&object_hit), &[]);
    assert_eq!(object_event.get("snap").and_then(|snap| snap.get("kind")).and_then(DslValue::as_str), Some("object"));

    let miss = create_spatial_pick_event(SpatialPickKind::PointerMove, [0.0; 3], None, &[]);
    assert!(miss.get("snap").is_none() && miss.get("selection").is_none());
}
//#endregion 🔖️Targets

//#region 🔖️Filters
/// 🧪️ React parity: `filterSpatialPickTargetsForPrimitiveToggles hides primitive picks by kind`.
#[test]
fn primitive_toggles_hide_picks_by_kernel_kind() {
    let geometry = box_geometry();
    let targets = create_spatial_pick_targets(&[], Some(&geometry), Some("spatial.shape"));
    let visible = filter_spatial_pick_targets_for_primitive_toggles(targets, &SpatialPrimitiveToggles::default().with(ModelEntityKind::Vertex, false));
    assert!(!visible.iter().any(|row| row.geometry_kind == Some(ModelEntityKind::Vertex)));
    assert!(visible.iter().any(|row| row.geometry_kind == Some(ModelEntityKind::Edge)));
}

/// 🧪️ React parity: `filterSpatialPickTargetsForTypologyToggles hides typology object picks`.
#[test]
fn typology_toggles_hide_the_rows_of_that_typology() {
    let geometry = box_geometry();
    let objects = vec![object("hull", "energy.energy.hull", "solid", "c0", "solid")];
    let targets = create_spatial_pick_targets(&objects, Some(&geometry), Some("aec.building.energy"));
    let typology_ids = model_definition_typology_ids(Some("aec.building.energy"));
    let visible = filter_spatial_pick_targets_for_typology_toggles(targets, &SpatialTypologyToggles::default().with("energy.energy.hull", false), &typology_ids);
    assert!(!visible.iter().any(|row| row.typology_id.as_deref() == Some("energy.energy.hull")));
}

#[test]
fn untyped_targets_survive_while_any_typology_is_still_on() {
    let typology_ids = model_definition_typology_ids(Some("aec.building.energy"));
    let untyped = vec![target(SpatialPickTargetKind::Vertex, "v0", Some(ModelEntityKind::Vertex), None)];
    let all_off = SpatialTypologyToggles(typology_ids.iter().map(|id| (id.clone(), false)).collect());
    assert!(filter_spatial_pick_targets_for_typology_toggles(untyped.clone(), &SpatialTypologyToggles::default(), &typology_ids).len() == 1);
    assert!(filter_spatial_pick_targets_for_typology_toggles(untyped, &all_off, &typology_ids).is_empty());
}

/// 🧪️ React parity: `filterSpatialPickTargetsForEntityFlags excludes hidden and locked targets`.
#[test]
fn hidden_and_locked_entities_take_no_pick() {
    let targets = vec![target(SpatialPickTargetKind::Object, "shown", None, None), target(SpatialPickTargetKind::Object, "hidden", None, None), target(SpatialPickTargetKind::Object, "locked", None, None)];
    let flags = |id: &str| match id {
        "hidden" => SpatialEntityFlags { hidden: true, locked: false },
        "locked" => SpatialEntityFlags { hidden: false, locked: true },
        _ => SpatialEntityFlags::default(),
    };
    assert_eq!(keys(&filter_spatial_pick_targets_for_entity_flags(targets, &flags)), vec!["object:shown"]);
    let selection = vec![SelectionTarget { kind: ModelEntityKind::Object, id: "shown".into(), editable: false }, SelectionTarget { kind: ModelEntityKind::Object, id: "locked".into(), editable: false }];
    assert_eq!(prune_selection_targets_for_entity_flags(selection, &flags).len(), 1);
}

/// 🧪️ React parity: `resolveSpatialEntityFlags inherits hidden state from owning object`.
#[test]
fn kernel_members_inherit_the_owning_object_hidden_flag() {
    let geometry = box_geometry();
    let mut objects = vec![object("hull", "energy.energy.hull", "solid", "c0", "solid")];
    objects[0].visible = false;
    assert_eq!(resolve_spatial_entity_flags(&objects, &geometry, "aec.building.energy", "f0"), SpatialEntityFlags { hidden: true, locked: false });
    objects[0].visible = true;
    objects[0].locked = true;
    assert_eq!(resolve_spatial_entity_flags(&objects, &geometry, "aec.building.energy", "e0"), SpatialEntityFlags { hidden: false, locked: true });
    assert_eq!(resolve_spatial_entity_flags(&objects, &geometry, "aec.building.energy", "not-in-model"), SpatialEntityFlags::default());
}

/// 🧪️ React parity: `filterSpatialPickTargetsForActiveView scopes by model definition entity kinds`.
#[test]
fn active_view_drops_typology_object_rows_on_the_shape_definition() {
    let targets = vec![target(SpatialPickTargetKind::Object, "hull", None, None), target(SpatialPickTargetKind::Object, "c0", Some(ModelEntityKind::Solid), None), target(SpatialPickTargetKind::Face, "f0", Some(ModelEntityKind::Face), None)];
    assert_eq!(keys(&filter_spatial_pick_targets_for_active_view(targets.clone(), Some("spatial.shape"))), vec!["object:c0", "face:f0"]);
    assert_eq!(keys(&filter_spatial_pick_targets_for_active_view(targets, Some("aec.building.energy"))), vec!["object:hull", "object:c0", "face:f0"]);
}

/// 🧪️ React parity: `modelDefinitionPickTargetKinds maps primitive entity kinds to pick toggles`.
#[test]
fn model_definition_pick_kinds_cover_every_kind_the_assets_declare() {
    let kinds = model_definition_pick_target_kinds(Some("spatial.shape"));
    assert_eq!(kinds, vec![SpatialPickTargetKind::Vertex, SpatialPickTargetKind::Edge, SpatialPickTargetKind::Face, SpatialPickTargetKind::Object]);
    let toggles = default_spatial_pick_kind_toggles_for_model_definition(Some("spatial.shape"));
    assert_eq!(toggles, default_spatial_pick_kind_toggles());
}

/// 🧪️ React parity: `resolveSpatialSceneVisibility switches edit wireframe vs committed object mesh`.
#[test]
fn scene_visibility_follows_the_edge_face_object_toggles() {
    let all_on = resolve_spatial_scene_visibility(Some("spatial.shape"), default_spatial_pick_kind_toggles());
    assert_eq!(all_on, SpatialSceneVisibility { show_factory_wireframe: true, show_committed_faces: true, show_committed_edges: true });
    let mut no_edges = default_spatial_pick_kind_toggles();
    no_edges.edge = Some(false);
    let some_off = resolve_spatial_scene_visibility(Some("spatial.shape"), no_edges);
    assert_eq!(some_off, SpatialSceneVisibility { show_factory_wireframe: false, show_committed_faces: true, show_committed_edges: false });
    let mut faces_and_objects_off = default_spatial_pick_kind_toggles();
    faces_and_objects_off.face = Some(false);
    faces_and_objects_off.object = Some(false);
    assert!(!resolve_spatial_scene_visibility(Some("spatial.shape"), faces_and_objects_off).show_committed_faces);
}

/// 🧪️ React parity: `spatialSceneKindTogglesForModelDefinition keeps committed faces on when typology
/// picks are empty` — a kind only goes dark when BOTH of its kernel kinds are off.
#[test]
fn scene_kind_toggles_need_both_kernel_kinds_off() {
    let one_off = spatial_scene_kind_toggles_for_model_definition(Some("spatial.shape"), &default_spatial_primitive_toggles().with(ModelEntityKind::Face, false));
    assert_eq!(one_off.face, Some(true), "shells still paint faces");
    let both_off = spatial_scene_kind_toggles_for_model_definition(Some("spatial.shape"), &default_spatial_primitive_toggles().with(ModelEntityKind::Face, false).with(ModelEntityKind::Shell, false));
    assert_eq!(both_off.face, Some(false));
    let solids_off = spatial_scene_kind_toggles_for_model_definition(Some("spatial.shape"), &default_spatial_primitive_toggles().with(ModelEntityKind::Solid, false));
    assert_eq!(solids_off.object, Some(false), "object picks come from solids alone");
}

#[test]
fn kind_toggles_intersect_to_the_stricter_of_the_two() {
    let mut visible = default_spatial_pick_kind_toggles();
    visible.edge = Some(false);
    let mut selection = default_spatial_pick_kind_toggles();
    selection.face = Some(false);
    let merged = intersect_spatial_pick_kind_toggles(visible, selection);
    assert_eq!(merged.edge, Some(false));
    assert_eq!(merged.face, Some(false));
    assert_eq!(merged.object, None, "unset stays unset, which reads as enabled");
    assert!(merged.enabled(SpatialPickTargetKind::Object));
}

/// 🧪️ React parity: `spatialPickKindTogglesFromTypologyFilteredTargets`.
#[test]
fn kind_toggles_from_filtered_targets_are_off_when_nothing_of_that_kind_survives() {
    let visible = vec![target(SpatialPickTargetKind::Face, "f0", Some(ModelEntityKind::Face), None)];
    let toggles = spatial_pick_kind_toggles_from_typology_filtered_targets(Some("spatial.shape"), &visible);
    assert_eq!(toggles.face, Some(true));
    assert_eq!(toggles.edge, Some(false));
    assert_eq!(toggles.vertex, Some(false));
    assert_eq!(toggles.object, Some(false));
}

/// 🧪️ React parity: `spatialToggleGroupState reports all, none, and partial chrome groups`.
#[test]
fn toggle_group_state_reports_all_none_partial() {
    let keys: Vec<String> = vec!["a".into(), "b".into()];
    assert_eq!(spatial_toggle_group_state(&[], &BTreeMap::new()), SpatialToggleGroupState::None);
    assert_eq!(spatial_toggle_group_state(&keys, &BTreeMap::new()), SpatialToggleGroupState::All);
    assert_eq!(spatial_toggle_group_state(&keys, &spatial_toggle_group_fill(&keys, false)), SpatialToggleGroupState::None);
    assert_eq!(spatial_toggle_group_state(&keys, &[("a".to_string(), false)].into_iter().collect()), SpatialToggleGroupState::Partial);
    assert_eq!(spatial_toggle_checkbox_state(SpatialToggleGroupState::Partial), SpatialToggleCheckboxState::Indeterminate);
    assert_eq!(spatial_toggle_checkbox_state(SpatialToggleGroupState::All), SpatialToggleCheckboxState::Checked);
    assert_eq!(spatial_toggle_checkbox_state(SpatialToggleGroupState::None), SpatialToggleCheckboxState::Unchecked);
}

/// 🧪️ React parity: `defaultInteractionReplChromeState seeds typology and primitive toggles by default`.
#[test]
fn default_chrome_toggles_start_every_typology_and_primitive_on() {
    let typology = default_spatial_typology_toggles_for_model_definition(Some("aec.building.energy"));
    assert_eq!(typology.0.len(), 5);
    assert!(typology.0.values().all(|on| *on));
    let primitives = default_spatial_primitive_toggles();
    assert_eq!(primitives.0.len(), SPATIAL_PRIMITIVE_KINDS.len());
    assert!(primitives.0.values().all(|on| *on));
}
//#endregion 🔖️Filters

//#region 🔖️Keys
/// 🧪️ React parity: `spatialHoverKeyAliases links object and solid geometry pick keys`.
#[test]
fn hover_keys_alias_object_and_solid() {
    let aliases = spatial_hover_key_aliases(Some("object:c0"));
    assert!(aliases.contains("object:c0") && aliases.contains("solid:c0"));
    let from_solid = spatial_hover_key_aliases(Some("solid:c0"));
    assert!(from_solid.contains("solid:c0") && from_solid.contains("object:c0"));
    assert!(spatial_hover_key_aliases(None).is_empty());
    assert!(spatial_hover_keys_match(Some("solid:c0"), Some("object:c0")));
    assert!(spatial_hover_keys_match(Some("wire:w0"), Some("edge:w0")));
    assert!(!spatial_hover_keys_match(Some("solid:c0"), Some("solid:c1")));
    assert!(!spatial_hover_keys_match(None, Some("solid:c0")));
}

/// 🧪️ React parity: `maps selection target keys to pick target keys for highlights`.
#[test]
fn member_keys_use_the_kernel_kind_and_object_rows_use_object() {
    assert_eq!(spatial_pick_target_member_key(&target(SpatialPickTargetKind::Object, "hull", None, None)), "object:hull");
    assert_eq!(spatial_pick_target_member_key(&target(SpatialPickTargetKind::Object, "c0", Some(ModelEntityKind::Solid), None)), "solid:c0");
    assert_eq!(spatial_pick_target_member_key(&target(SpatialPickTargetKind::Edge, "w0", Some(ModelEntityKind::Wire), None)), "wire:w0");
    assert_eq!(selection_target_hover_key(&SelectionTarget { kind: ModelEntityKind::Face, id: "f0".into(), editable: true }), "face:f0");
}

/// 🧪️ React parity: `canvasHoverKeyForSelectionTarget maps primitive picks to typology object hover keys`.
#[test]
fn a_solid_member_selection_hovers_the_owning_typology_object() {
    let geometry = box_geometry();
    let objects = vec![object("hull", "energy.energy.hull", "solid", "c0", "solid")];
    let face = SelectionTarget { kind: ModelEntityKind::Face, id: "f0".into(), editable: true };
    assert_eq!(canvas_hover_key_for_selection_target(&objects, &geometry, "aec.building.energy", &face), "object:hull");
    let solid = SelectionTarget { kind: ModelEntityKind::Solid, id: "c0".into(), editable: true };
    assert_eq!(canvas_hover_key_for_selection_target(&objects, &geometry, "aec.building.energy", &solid), "object:hull");
    let orphan = SelectionTarget { kind: ModelEntityKind::Vertex, id: "nope".into(), editable: true };
    assert_eq!(canvas_hover_key_for_selection_target(&objects, &geometry, "aec.building.energy", &orphan), "vertex:nope");
    let row = SelectionTarget { kind: ModelEntityKind::Object, id: "hull".into(), editable: false };
    assert_eq!(canvas_hover_key_for_selection_target(&objects, &geometry, "aec.building.energy", &row), "object:hull");
}
//#endregion 🔖️Keys

//#region 🔖️Reveal
/// 🧪️ React parity: `revealedObjectIdsFromPickKeys expands solid and member picks to the owning object`.
#[test]
fn reveal_expands_a_member_pick_to_its_owning_object() {
    let geometry = box_geometry();
    let objects = vec![object("hull", "energy.energy.hull", "solid", "c0", "solid")];
    let index = build_geometry_object_index(&objects, &geometry, "aec.building.energy");
    assert_eq!(revealed_object_ids_from_pick_keys(&index, Some("face:f0"), &BTreeSet::new()).into_iter().collect::<Vec<_>>(), vec!["hull".to_string()]);
    assert_eq!(revealed_object_ids_from_pick_keys(&index, Some("object:c0"), &BTreeSet::new()).into_iter().collect::<Vec<_>>(), vec!["hull".to_string()]);
    let selected: BTreeSet<String> = ["vertex:v3".to_string()].into_iter().collect();
    assert_eq!(revealed_object_ids_from_pick_keys(&index, None, &selected).into_iter().collect::<Vec<_>>(), vec!["hull".to_string()]);
    assert!(revealed_object_ids_from_pick_keys(&index, Some("vertex:nope"), &BTreeSet::new()).is_empty());
}

/// 🧪️ React parity: `resolveSpatialPickTargetsToRender skips hidden unless pinned` +
/// `draws all enabled kinds` + `draws factory primitives without hover reveal`.
#[test]
fn render_set_skips_hidden_unless_pinned_and_honours_reveal() {
    let shown = target(SpatialPickTargetKind::Face, "f0", Some(ModelEntityKind::Face), None);
    let hidden = target(SpatialPickTargetKind::Face, "f1", Some(ModelEntityKind::Face), None);
    let view = vec![shown.clone(), hidden.clone()];
    let flags = |id: &str| if id == "f1" { SpatialEntityFlags { hidden: true, locked: false } } else { SpatialEntityFlags::default() };

    assert_eq!(keys(&resolve_spatial_pick_targets_to_render(&view, default_spatial_pick_kind_toggles(), &BTreeSet::new(), &flags, None)), vec!["face:f0"]);
    let pinned: BTreeSet<String> = ["face:f1".to_string()].into_iter().collect();
    assert_eq!(keys(&resolve_spatial_pick_targets_to_render(&view, default_spatial_pick_kind_toggles(), &pinned, &flags, None)), vec!["face:f0", "face:f1"]);

    let mut faces_off = default_spatial_pick_kind_toggles();
    faces_off.face = Some(false);
    assert!(resolve_spatial_pick_targets_to_render(&view, faces_off, &BTreeSet::new(), &flags, None).is_empty());

    let geometry = box_geometry();
    let objects = vec![object("hull", "energy.energy.hull", "solid", "c0", "solid")];
    let index = build_geometry_object_index(&objects, &geometry, "aec.building.energy");
    let revealed: BTreeSet<String> = BTreeSet::new();
    let no_flags = |_: &str| SpatialEntityFlags::default();
    assert!(resolve_spatial_pick_targets_to_render(&view, default_spatial_pick_kind_toggles(), &BTreeSet::new(), &no_flags, Some((&index, &revealed))).is_empty(), "nothing revealed hides every member");
    let object_row = vec![target(SpatialPickTargetKind::Object, "hull", None, None)];
    assert_eq!(keys(&resolve_spatial_pick_targets_to_render(&object_row, default_spatial_pick_kind_toggles(), &BTreeSet::new(), &no_flags, Some((&index, &revealed)))), vec!["object:hull"], "typology object rows always draw");
}

#[test]
fn pinning_an_object_key_also_pins_its_solid_alias() {
    let solid = target(SpatialPickTargetKind::Object, "c0", Some(ModelEntityKind::Solid), None);
    let view = vec![solid];
    let hidden = |_: &str| SpatialEntityFlags { hidden: true, locked: false };
    let pinned: BTreeSet<String> = ["solid:c0".to_string()].into_iter().collect();
    assert_eq!(keys(&resolve_spatial_pick_targets_to_render(&view, default_spatial_pick_kind_toggles(), &pinned, &hidden, None)), vec!["object:c0"]);
}
//#endregion 🔖️Reveal

//#region 🔖️Selection
/// 🧪️ React parity: `clears selection on empty background pick in default modifier mode` +
/// `merges picks within active model definition without clearing other models`.
#[test]
fn merge_modes_replace_add_subtract_and_invert() {
    let row = |id: &str| SelectionTarget { kind: ModelEntityKind::Face, id: id.into(), editable: true };
    let current = vec![row("f0"), row("f1")];
    assert!(merge_selection_targets(&current, Vec::new(), MergeMode::Replace).is_empty());
    assert_eq!(merge_selection_targets(&current, vec![row("f2")], MergeMode::Replace), vec![row("f2")]);
    assert_eq!(merge_selection_targets(&current, vec![row("f1"), row("f2")], MergeMode::Additive), vec![row("f0"), row("f1"), row("f2")]);
    assert_eq!(merge_selection_targets(&current, vec![row("f1")], MergeMode::Subtractive), vec![row("f0")]);
    assert_eq!(merge_selection_targets(&current, vec![row("f1"), row("f2")], MergeMode::Invertive), vec![row("f0"), row("f2")]);
    assert_eq!(merge_selection_targets(&current, vec![row("f2")], MergeMode::Range), vec![row("f2")], "a spatial canvas has no range topology");
}

#[test]
fn merge_deduplicates_the_incoming_rows_first() {
    let row = |id: &str| SelectionTarget { kind: ModelEntityKind::Face, id: id.into(), editable: true };
    assert_eq!(unique_selection_targets(vec![row("f0"), row("f0"), row("f1")]), vec![row("f0"), row("f1")]);
    assert_eq!(merge_selection_targets(&[], vec![row("f0"), row("f0")], MergeMode::Additive), vec![row("f0")]);
}

#[test]
fn modifiers_map_onto_the_wire_merge_vocabulary() {
    assert_eq!(spatial_merge_mode_from_modifiers(false, false, false), MergeMode::Replace);
    assert_eq!(spatial_merge_mode_from_modifiers(true, false, false), MergeMode::Additive);
    assert_eq!(spatial_merge_mode_from_modifiers(false, true, false), MergeMode::Subtractive);
    assert_eq!(spatial_merge_mode_from_modifiers(false, false, true), MergeMode::Subtractive);
    assert_eq!(spatial_merge_mode_from_modifiers(true, true, false), MergeMode::Invertive);
}

#[test]
fn a_pick_target_becomes_an_editable_row_unless_it_is_a_typology_object() {
    assert_eq!(spatial_selection_target(&target(SpatialPickTargetKind::Object, "hull", None, None)), SelectionTarget { kind: ModelEntityKind::Object, id: "hull".into(), editable: false });
    assert_eq!(spatial_selection_target(&target(SpatialPickTargetKind::Object, "c0", Some(ModelEntityKind::Solid), None)), SelectionTarget { kind: ModelEntityKind::Solid, id: "c0".into(), editable: true });
    assert_eq!(spatial_selection_target(&target(SpatialPickTargetKind::Vertex, "v0", None, None)), SelectionTarget { kind: ModelEntityKind::Vertex, id: "v0".into(), editable: true });
}
//#endregion 🔖️Selection

//#region 🔖️Style
/// 🧪️ React parity: `resolveCommittedMeshMaterialProps resolves selection, hover, and style fallback
/// materials` — the selected/hovered/locked/typology/default cascade, slot for slot.
#[test]
fn target_style_cascade_matches_react() {
    let face = target(SpatialPickTargetKind::Face, "f0", Some(ModelEntityKind::Face), None);
    let vertex = target(SpatialPickTargetKind::Vertex, "v0", Some(ModelEntityKind::Vertex), None);
    let edge = target(SpatialPickTargetKind::Edge, "e0", Some(ModelEntityKind::Edge), None);
    let row = target(SpatialPickTargetKind::Object, "hull", None, None);

    let selected = target_style(&face, false, true, None, false);
    assert_eq!(selected.color, SpatialColorRef::Token(SpatialScenePalette::SELECTED));
    assert_eq!((selected.opacity, selected.line_width), (0.34, 9.0));
    assert_eq!(target_style(&vertex, false, true, None, false).opacity, 1.0);

    let hovered = target_style(&face, true, false, None, false);
    assert_eq!(hovered.color, SpatialColorRef::Token(SpatialScenePalette::HOVERED));
    assert_eq!((hovered.opacity, hovered.line_width), (0.28, 8.0));
    assert_eq!(target_style(&face, true, true, None, false).line_width, 9.0, "selected wins over hovered");

    assert_eq!(target_style(&vertex, false, false, None, false), SpatialTargetStyle { color: SpatialColorRef::Token(SpatialScenePalette::VERTEX), emissive: SpatialColorRef::Token(SpatialScenePalette::VERTEX_EMISSIVE), opacity: 1.0, line_width: 5.0 });
    assert_eq!(target_style(&edge, false, false, None, false).opacity, 0.8);
    assert_eq!(target_style(&row, false, false, None, false), SpatialTargetStyle { color: SpatialColorRef::Token(SpatialScenePalette::OBJECT), emissive: SpatialColorRef::Token(SpatialScenePalette::OBJECT_EMISSIVE), opacity: 0.28, line_width: 7.0 });
    assert_eq!(target_style(&face, false, false, None, false).opacity, 0.16);

    let locked_vertex = target_style(&vertex, false, false, None, true);
    assert_eq!(locked_vertex.opacity, 0.55 * WORLD_LOCKED_OPACITY_SCALE);
    assert_eq!(locked_vertex.line_width, 4.0);
    assert_eq!(target_style(&row, false, false, None, true).line_width, 5.0);
}

/// 🧪️ React parity: a typology style replaces the palette slots with authored literals, and the face
/// arm clamps the fill opacity at `0.42`.
#[test]
fn typology_style_overrides_the_palette_slots() {
    let style = resolve_typology_style("building.building.wall");
    let face = target(SpatialPickTargetKind::Face, "f0", Some(ModelEntityKind::Face), None);
    let edge = target(SpatialPickTargetKind::Edge, "e0", Some(ModelEntityKind::Edge), None);
    let row = target(SpatialPickTargetKind::Object, "hull", None, None);

    let styled_face = target_style(&face, false, false, Some(&style), false);
    assert_eq!(styled_face.color, SpatialColorRef::Hex("#C4785A".into()));
    assert_eq!(styled_face.opacity, 0.42, "0.74 authored opacity clamps to 0.42 on a face");
    assert_eq!(target_style(&edge, false, false, Some(&style), false).color, SpatialColorRef::Hex("#8B4F38".into()));
    assert_eq!(target_style(&row, false, false, Some(&style), false).opacity, 0.74);
    assert_eq!(target_style(&face, false, true, Some(&style), false).color, SpatialColorRef::Token(SpatialScenePalette::SELECTED), "selection still wins");
    assert_eq!(target_style(&face, false, false, Some(&style), true).color, SpatialColorRef::Hex("#C4785A".into()));
}

/// 🧪️ React parity: `createSolidTypologyStyleResolver maps solids to their object typology style`.
#[test]
fn solids_resolve_the_style_of_the_object_row_that_owns_them() {
    let geometry = box_geometry();
    let objects = vec![object("slab", "structure.structure.onewayreinforcedconcreteslab", "solid", "c0", "solid")];
    let style = solid_typology_style(&objects, &geometry, "aec.building.structure.classic", "c0").expect("style");
    assert_eq!(style.color, "#8B7355");
    assert_eq!(style.pattern.kind, "hatch");
    assert_eq!(typology_style_to_material_props(&style), ("#8B7355".to_string(), "#8B7355".to_string(), 0.78));
    assert!(solid_typology_style(&objects, &geometry, "aec.building.structure.classic", "nope").is_none());
}

#[test]
fn toggle_row_labels_read_the_authored_typology_label() {
    assert_eq!(typology_toggle_row_label("energy.energy.baseplate"), "BasePlate");
    assert_eq!(typology_toggle_row_label("building.building.wall"), "Wall");
    assert_eq!(typology_toggle_row_label("made.up.thing"), "Thing");
}
//#endregion 🔖️Style

//#region 🔖️Buckets
#[test]
fn buckets_walk_every_member_of_a_solid() {
    let geometry = box_geometry();
    let buckets = geometry_buckets(&geometry);
    let members = buckets.solid_primitive_member_ids("c0");
    assert!(members.contains("solid:c0") && members.contains("shell:sh0"));
    assert_eq!(members.iter().filter(|key| key.starts_with("face:")).count(), 6);
    assert_eq!(members.iter().filter(|key| key.starts_with("wire:")).count(), 6);
    assert_eq!(members.iter().filter(|key| key.starts_with("edge:")).count(), 12);
    assert_eq!(members.iter().filter(|key| key.starts_with("vertex:")).count(), 8);
    assert!(buckets.solid_primitive_member_ids("nope").is_empty());
}

#[test]
fn bucket_points_deduplicate_and_centroid() {
    let geometry = box_geometry();
    let buckets = geometry_buckets(&geometry);
    assert_eq!(buckets.entity_points(ModelEntityKind::Vertex, "v6"), vec![[1.0, 1.0, 1.0]]);
    assert_eq!(buckets.entity_points(ModelEntityKind::Edge, "e0"), vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    assert_eq!(buckets.entity_points(ModelEntityKind::Face, "f0").len(), 4);
    assert_eq!(buckets.entity_points(ModelEntityKind::Solid, "c0").len(), 8);
    assert_eq!(buckets.all_vertex_points().len(), 8);
    assert!(buckets.entity_points(ModelEntityKind::Object, "c0").is_empty());
}

#[test]
fn wire_segments_walk_down_to_straight_edges() {
    let geometry = box_geometry();
    let buckets = geometry_buckets(&geometry);
    assert_eq!(buckets.entity_wire_segments(ModelEntityKind::Edge, "e0"), vec![([0.0, 0.0, 0.0], [1.0, 0.0, 0.0])]);
    assert_eq!(buckets.entity_wire_segments(ModelEntityKind::Wire, "w0").len(), 4);
    assert_eq!(buckets.entity_wire_segments(ModelEntityKind::Face, "f0").len(), 4);
    assert_eq!(buckets.entity_wire_segments(ModelEntityKind::Shell, "sh0").len(), 24);
    assert_eq!(buckets.entity_wire_segments(ModelEntityKind::Solid, "c0").len(), 24);
    assert_eq!(buckets.all_edge_segments().len(), 12);
    let revealed: BTreeSet<String> = ["edge:e0".to_string(), "edge:e5".to_string()].into_iter().collect();
    assert_eq!(buckets.edge_segments_for_members(&revealed).len(), 2);
}

#[test]
fn anchor_rows_are_read_out_of_the_untyped_bucket() {
    let mut geometry = box_geometry();
    geometry.anchors = vec![DslValue::object([
        ("id".to_string(), DslValue::String("a0".into())),
        ("position".to_string(), DslValue::Array(vec![DslValue::float(2.0), DslValue::float(0.0), DslValue::float(0.0)])),
        ("attachment".to_string(), DslValue::object([("kind".to_string(), DslValue::String("solid".into())), ("id".to_string(), DslValue::String("c0".into()))])),
    ])];
    let buckets = geometry_buckets(&geometry);
    assert_eq!(buckets.anchors, vec![AnchorRow { id: "a0".into(), position: [2.0, 0.0, 0.0], attachment: Some(("solid".into(), "c0".into())) }]);
    assert_eq!(buckets.entity_points(ModelEntityKind::Anchor, "a0"), vec![[2.0, 0.0, 0.0]]);

    let objects = vec![object("hull", "energy.energy.hull", "solid", "c0", "solid")];
    let typology_index = build_geometry_typology_index(&objects, &geometry, "aec.building.energy");
    assert_eq!(typology_index.get("anchor:a0").map(String::as_str), Some("energy.energy.hull"), "an anchor inherits its attachment's typology");
    let object_index = build_geometry_object_index(&objects, &geometry, "aec.building.energy");
    assert_eq!(object_index.get("anchor:a0").map(String::as_str), Some("hull"));

    let targets = create_spatial_pick_targets(&objects, Some(&geometry), Some("aec.building.energy"));
    let anchor_target = targets.iter().find(|row| row.geometry_kind == Some(ModelEntityKind::Anchor)).expect("anchor pick target");
    assert_eq!((anchor_target.kind, anchor_target.id.as_str()), (SpatialPickTargetKind::Vertex, "a0"));
    assert_eq!(anchor_target.typology_id.as_deref(), Some("energy.energy.hull"));
}

#[test]
fn malformed_anchor_rows_are_dropped_rather_than_faulting() {
    let mut geometry = CadGeometry::default();
    geometry.anchors = vec![DslValue::object([("id".to_string(), DslValue::String("a0".into()))]), DslValue::String("nonsense".into())];
    assert!(geometry_buckets(&geometry).anchors.is_empty());
}
//#endregion 🔖️Buckets

//#region 🔖️Indices
#[test]
fn typology_index_covers_every_member_of_an_owned_solid() {
    let geometry = box_geometry();
    let objects = vec![object("hull", "energy.energy.hull", "solid", "c0", "solid")];
    let index = build_geometry_typology_index(&objects, &geometry, "aec.building.energy");
    assert_eq!(index.get("solid:c0").map(String::as_str), Some("energy.energy.hull"));
    assert_eq!(index.get("vertex:v0").map(String::as_str), Some("energy.energy.hull"));
    assert!(build_geometry_typology_index(&objects, &geometry, "spatial.shape").is_empty(), "the row's typology is not declared under spatial.shape");
}

#[test]
fn object_index_falls_back_to_an_unowned_solid_as_its_own_owner() {
    let geometry = box_geometry();
    let index = build_geometry_object_index(&[], &geometry, "aec.building.energy");
    assert_eq!(index.get("solid:c0").map(String::as_str), Some("c0"));
    assert_eq!(index.get("object:c0").map(String::as_str), Some("c0"));
    assert_eq!(index.get("face:f0").map(String::as_str), Some("c0"));
}

#[test]
fn object_rows_are_scoped_to_their_declaring_model_definition() {
    let objects = vec![object("hull", "energy.energy.hull", "solid", "c0", "solid"), object("wall", "building.building.wall", "surface", "f0", "face")];
    assert_eq!(list_model_objects_for_model_definition(&objects, "aec.building.energy").iter().map(|row| row.id.as_str()).collect::<Vec<_>>(), vec!["hull"]);
    assert_eq!(list_model_objects_for_model_definition(&objects, "aec.building").iter().map(|row| row.id.as_str()).collect::<Vec<_>>(), vec!["wall"]);
    assert!(list_model_objects_for_model_definition(&objects, "spatial.shape").is_empty());
}
//#endregion 🔖️Indices
