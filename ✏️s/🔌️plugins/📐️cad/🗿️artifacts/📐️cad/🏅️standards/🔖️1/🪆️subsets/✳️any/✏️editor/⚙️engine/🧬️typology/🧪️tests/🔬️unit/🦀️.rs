//! 🧪️ Laws for the CAD typology/model-definition registry — each one mirrors a case in the React
//! renderer's own suite (`⚙️engine/📺️renderer/🧪️tests/🧪️repluserfacingsuggestiondetail/🟦️.tsx`) or in
//! the spatial kernel's typology-style tests, so the two implementations can never drift silently.

use super::*;

//#region 🔖️Assets
#[test]
fn every_model_definition_asset_parses() {
    assert_eq!(list_model_definition_manifests().len(), RAW_MODEL_DEFINITION_ASSETS.len());
    let ids: Vec<&str> = list_model_definition_manifests().iter().map(|row| row.id.as_str()).collect();
    for expected in ["spatial.shape", "aec.building", "aec.building.energy", "aec.building.structure", "aec.building.structure.classic", "aec.building.concrete", "aec.building.structure.fem.line", "aec.building.structure.fem.surface", "aec.building.structure.fem.solid"] {
        assert!(ids.contains(&expected), "missing model definition {expected} in {ids:?}");
    }
}

#[test]
fn every_typology_asset_parses_and_keeps_its_owner() {
    assert_eq!(list_model_definition_typologies().count(), RAW_TYPOLOGY_ASSETS.len());
    assert_eq!(model_definition_id_for_typology("building.building.wall"), Some("aec.building"));
    assert_eq!(model_definition_id_for_typology("energy.energy.baseplate"), Some("aec.building.energy"));
    assert_eq!(model_definition_id_for_typology("nope"), None);
}

#[test]
fn every_attribute_definition_asset_parses() {
    let building = list_attribute_definitions_for_model_definition("aec.building");
    assert_eq!(building.iter().map(|row| row.id.as_str()).collect::<Vec<_>>(), vec!["spatial.shape.material", "spatial.shape.opening"]);
    let material = building.iter().find(|row| row.id == "spatial.shape.material").expect("material attribute");
    assert_eq!(material.targets, vec!["edge", "wire", "face", "solid"]);
    assert_eq!(material.geometry_selector.as_ref().map(|selector| selector.kinds.clone()), Some(vec!["edge".to_string(), "wire".to_string(), "face".to_string(), "solid".to_string()]));
}
//#endregion 🔖️Assets

//#region 🔖️Catalog
#[test]
fn default_model_definition_is_the_manifest_flagged_default() {
    assert_eq!(default_model_definition_id(), "spatial.shape");
}

#[test]
fn shape_model_definition_is_the_one_declaring_kernel_typologies() {
    assert!(is_shape_model_definition(None));
    assert!(is_shape_model_definition(Some("spatial.shape")));
    assert!(!is_shape_model_definition(Some("aec.building")));
    assert!(!is_shape_model_definition(Some("aec.building.energy")));
    assert_eq!(kernel_typology_ids("spatial.shape").map(|map| map.len()), Some(7));
    assert!(kernel_typology_ids("aec.building").is_none());
}

#[test]
fn typologies_are_scoped_to_the_model_definition_that_ships_them() {
    let building: Vec<&str> = list_typologies_for_model_definition("aec.building").into_iter().map(|row| row.id.as_str()).collect();
    assert!(building.contains(&"building.building.wall"));
    assert!(building.contains(&"building.building.beam"));
    assert!(!building.contains(&"energy.energy.hull"));
    assert!(list_typologies_for_model_definition("aec.building.concrete").is_empty());
}

/// 🧪️ React parity: `modelDefinitionTypologyIds` sorts and defaults to the shape definition.
#[test]
fn model_definition_typology_ids_are_sorted_and_default_to_shape() {
    let energy = model_definition_typology_ids(Some("aec.building.energy"));
    let mut sorted = energy.clone();
    sorted.sort();
    assert_eq!(energy, sorted);
    assert_eq!(energy, vec!["energy.energy.baseplate", "energy.energy.externalwall", "energy.energy.hull", "energy.energy.roof", "energy.energy.windows"]);
    assert_eq!(model_definition_typology_ids(None), model_definition_typology_ids(Some("spatial.shape")));
}

#[test]
fn typology_for_interaction_resolves_through_the_interactions_list() {
    assert_eq!(typology_for_interaction("building.building.placeWall").map(|row| row.id.as_str()), Some("building.building.wall"));
    assert_eq!(typology_for_interaction("nope.nope").map(|row| row.id.as_str()), None);
}

/// 🧪️ React parity: `modelDefinitionSelectionEntityKinds` always offers every primitive plus `object`,
/// primitives first, and only widens when an attribute definition targets something else.
#[test]
fn selection_entity_kinds_are_primitives_then_object() {
    let expected = vec![ModelEntityKind::Anchor, ModelEntityKind::Vertex, ModelEntityKind::Edge, ModelEntityKind::Wire, ModelEntityKind::Face, ModelEntityKind::Shell, ModelEntityKind::Solid, ModelEntityKind::Object];
    assert_eq!(model_definition_selection_entity_kinds("spatial.shape"), expected);
    assert_eq!(model_definition_selection_entity_kinds("aec.building"), expected);
    assert_eq!(model_definition_selection_entity_kinds("aec.building.structure"), expected);
}

#[test]
fn every_model_definition_uses_geometry_picking() {
    for row in list_model_definition_manifests() {
        assert!(model_definition_uses_geometry_picking(&row.id));
    }
}
//#endregion 🔖️Catalog

//#region 🔖️Style
/// 🧪️ React parity: `typologyStyleToMaterialProps and typologyStyleCacheKey reflect resolved style`.
#[test]
fn authored_style_survives_resolution_verbatim() {
    let style = resolve_typology_style("structure.structure.onewayreinforcedconcreteslab");
    assert_eq!(style.color, "#8B7355");
    assert_eq!(style.opacity, 0.78);
    assert_eq!(style.edge_color, "#5C4A33");
    assert_eq!(style.pattern.kind, "hatch");
    assert!(typology_style_cache_key(&style).contains("hatch"), "{}", typology_style_cache_key(&style));
    assert_eq!(typology_style_cache_key(&style), "#8B7355|#5C4A33|0.78|hatch|0|0.28|0.035|#3D3226");
}

/// 🧪️ The authored `dots` pattern omits `direction`, so the auto style's `0` must fill it in.
#[test]
fn partial_authored_pattern_falls_back_to_the_auto_style_fields() {
    let style = resolve_typology_style("energy.energy.externalwall");
    assert_eq!(style.pattern.kind, "dots");
    assert_eq!(style.pattern.direction, 0.0);
    assert_eq!(style.pattern.spacing, 0.4);
    assert_eq!(style.pattern.color, "#6B3828");
}

/// 🧪️ Bit-for-bit parity with the TS auto style (FNV-1a over UTF-16 units → golden-angle hue →
/// HSL(0.58, 0.52) → hex), pinned for three typologies that ship no authored `style` block.
#[test]
fn auto_style_reproduces_the_typescript_derivation() {
    let box_style = resolve_typology_style("spatial.shape.primitive.box");
    assert_eq!(box_style.color, "#3eaecc");
    assert_eq!(box_style.edge_color, "#2a768b");
    assert_eq!(box_style.pattern.color, "#338fa7");
    assert_eq!(box_style.opacity, 0.72);
    assert_eq!(box_style.pattern.kind, "none");

    let hull_style = resolve_typology_style("energy.energy.hull");
    assert_eq!(hull_style.color, "#3ecca7");
    assert_eq!(hull_style.edge_color, "#2a8b72");

    let unknown = resolve_typology_style("building.building.wall");
    assert_eq!(unknown.color, "#C4785A", "authored wall style must win over the auto hue");
}

#[test]
fn auto_style_is_defined_for_a_typology_with_no_asset_at_all() {
    let style = resolve_typology_style("made.up.typology");
    assert_eq!(style.opacity, 0.72);
    assert_eq!(style.pattern.kind, "none");
    assert!(style.color.starts_with('#') && style.color.len() == 7);
}

#[test]
fn js_number_drops_the_fraction_of_a_whole_float() {
    assert_eq!(js_number(0.0), "0");
    assert_eq!(js_number(45.0), "45");
    assert_eq!(js_number(0.035), "0.035");
    assert_eq!(js_number(0.78), "0.78");
}
//#endregion 🔖️Style

//#region 🔖️Labels
/// 🧪️ React parity: `spatialTypologyToggleLabel uses typology label pascal case`.
#[test]
fn toggle_label_pascal_cases_the_authored_label() {
    assert_eq!(spatial_typology_toggle_label("energy.energy.baseplate", Some("Base Plate")), "BasePlate");
    assert_eq!(spatial_typology_toggle_label("spatial.shape.primitive.box", Some("Box")), "Box");
}

#[test]
fn toggle_label_falls_back_to_the_typology_tail() {
    assert_eq!(spatial_typology_toggle_label("energy.energy.baseplate", None), "Baseplate");
    assert_eq!(spatial_typology_toggle_label("energy.energy.baseplate", Some("   ")), "Baseplate");
    assert_eq!(spatial_typology_toggle_label("a.b.external-wall", None), "ExternalWall");
}

#[test]
fn pascal_from_label_splits_on_every_non_alphanumeric_run() {
    assert_eq!(typology_object_pascal_from_label("External Wall"), "ExternalWall");
    assert_eq!(typology_object_pascal_from_label("one-way  slab_2"), "OneWaySlab2");
    assert_eq!(typology_object_pascal_from_label(""), "");
}
//#endregion 🔖️Labels

//#region 🔖️Construct
#[test]
fn construct_asset_ids_prefix_with_the_typology_namespace() {
    let kit = typology_construct_asset_ids("energy.energy.baseplate", "Base Plate");
    assert_eq!(kit.interaction, "energy.energy.constructBasePlate");
    assert_eq!(kit.construct_from_2_points_and_height, "energy.energy.constructBasePlateFrom2PointsAndHeight");
    assert_eq!(kit.construct_from_curve_and_height, "energy.energy.constructBasePlateFromCurveAndHeight");
    assert_eq!(kit.construct_from_surface, "energy.energy.constructBasePlateFromSurface");
    assert_eq!(typology_construct_asset_ids("wall", "Wall").interaction, "constructWall");
}

#[test]
fn base_plate_ships_only_the_surface_mode_action() {
    assert_eq!(typology_construct_mode_action_ids("energy.energy.baseplate", "Base Plate"), vec!["energy.energy.constructBasePlateFromSurface"]);
    assert_eq!(typology_construct_mode_action_ids("energy.energy.hull", "Hull").len(), 3);
}

#[test]
fn native_construct_kits_are_the_ones_whose_assets_match_the_derivation() {
    let energy = list_constructable_typologies_for_model_definition("aec.building.energy");
    let ids: Vec<&str> = energy.iter().map(|row| row.id.as_str()).collect();
    assert!(ids.contains(&"energy.energy.baseplate"), "{ids:?}");
    let wall = load_typology("building.building.wall").expect("wall typology");
    assert!(!typology_has_native_construct_kit(wall), "aec.building places walls through placeWall, not a construct kit");
}

#[test]
fn construct_kit_lookup_is_keyed_by_interaction_id() {
    let kit = typology_construct_kit_by_interaction("energy.energy.constructBasePlate").expect("base plate kit");
    assert_eq!(kit.typology, "energy.energy.baseplate");
    assert!(typology_construct_kit_by_interaction("energy.energy.constructBasePlateFromSurface").is_none());
}
//#endregion 🔖️Construct
