use super::*;
use crate::{Axis, Point2};

fn snapshot() -> ModelSnapshot {
    let mut snapshot = ModelSnapshot::default();
    snapshot.materials.insert("m-steel".into(), crate::Material { name: "Steel".into(), category: crate::MaterialCategory::Metal, color: crate::Rgb { r: 0.5, g: 0.5, b: 0.5 }, density: 7850.0, conductivity: 50.0, specific_heat: 470.0 });
    snapshot.sites.insert("site".into(), crate::Site { name: "Plot".into(), latitude: 47.0, longitude: 8.0, elevation: 0.0, true_north: 0.0, boundary: Vec::new() });
    snapshot.buildings.insert("bldg".into(), crate::Building { site: "site".into(), name: "House".into(), origin: Point2 { x: 0.0, y: 0.0 }, rotation: 0.0, elevation: 0.0 });
    snapshot.storeys.insert("st".into(), crate::Storey { building: "bldg".into(), name: "Ground".into(), level: 0, height: 3.0, cut_height: None });
    snapshot
}

fn applied(snapshot: &ModelSnapshot, mutation: &ModelMutation) -> ModelSnapshot {
    crate::mutations::apply_model_mutation(snapshot, mutation).unwrap_or_else(|refusal| panic!("refused: {refusal:?}"))
}

#[test]
fn a_grid_rule_reads_and_writes_the_same_text() {
    for text in ["spacing 1.5", "lines 1, 2.5, 4", "lines "] {
        let rule = parse_grid(text).expect("a rule");
        assert_eq!(parse_grid(&grid_text(&rule)), Some(rule));
    }
    assert_eq!(parse_grid("spacing x"), None);
    assert_eq!(parse_grid("cells 3"), None);
    assert_eq!(parse_grid("lines 1, two"), None);
}

#[test]
fn the_grid_override_of_a_wall_falls_back_to_its_type() {
    assert_eq!(parse_override(""), Some(None));
    assert_eq!(parse_override("TYPE"), Some(None));
    assert_eq!(parse_override("spacing 2"), Some(Some(CurtainGrid::Spacing { spacing: 2.0 })));
    assert_eq!(override_text(None), "type");
    assert_eq!(override_text(Some(&CurtainGrid::Lines { positions: vec![1.0] })), "lines 1");
    assert_eq!(parse_override("lines 1,"), Some(Some(CurtainGrid::Lines { positions: vec![1.0] })));
    assert_eq!(parse_override("wide"), None);
}

#[test]
fn every_panel_reads_and_writes_the_same_text() {
    for panel in [CurtainPanel::Glass, CurtainPanel::Empty, CurtainPanel::Solid { material: "m-wood".into() }, CurtainPanel::Door { door_type: "door-1".into() }, CurtainPanel::Window { window_type: "win-1".into() }] {
        assert_eq!(parse_panel(&panel_text(&panel)), Some(panel));
    }
    assert_eq!(parse_panel("door"), None, "a door panel names its type");
    assert_eq!(parse_panel("glass 3"), None);
    assert_eq!(parse_panel("brick wall"), None);
}

#[test]
fn a_new_curtain_wall_type_is_created_in_the_first_material_and_a_missing_material_is_named() {
    let base = snapshot();
    assert_eq!(create_curtain_wall_type(&ModelSnapshot::default(), "cwt", "", "Facade").err(), Some("bim.create.material-missing"));
    let created = applied(&base, &create_curtain_wall_type(&base, "cwt", "", "Facade").expect("a create"));
    let kind = &created.curtain_wall_types["cwt"];
    assert_eq!((kind.name.as_str(), kind.panel_material.as_str(), kind.mullion_material.as_str()), ("Facade", "m-steel", "m-steel"));
    assert_eq!(kind.panel, CurtainPanel::Glass);
}

fn with_wall() -> ModelSnapshot {
    let base = snapshot();
    let base = applied(&base, &create_curtain_wall_type(&base, "cwt", "", "Facade").expect("a type"));
    applied(&base, &ModelMutation::CreateCurtainWall(crate::mutations::create_curtain_wall::CreateCurtainWall {
        id: "cw".into(),
        curtain_wall: crate::CurtainWall { storey: "st".into(), curtain_wall_type: "cwt".into(), axis: Axis::Line { start: Point2 { x: 0.0, y: 0.0 }, end: Point2 { x: 6.0, y: 0.0 } }, base_offset: 0.0, top: crate::TopConstraint::StoreyTop { offset: 0.0 }, u_grid: None, v_grid: None, phase: crate::Phase::New, name: "Facade".into() },
    }))
}

#[test]
fn a_new_panel_override_takes_the_first_free_cell_of_the_base_row() {
    let base = with_wall();
    assert_eq!(create_curtain_panel_override(&base, "ov", "missing", "").err(), Some("bim.create.curtain-wall-missing"));
    let first = applied(&base, &create_curtain_panel_override(&base, "ov-0", "cw", "").expect("a create"));
    assert_eq!((first.curtain_panel_overrides["ov-0"].u, first.curtain_panel_overrides["ov-0"].v), (0, 0));
    let second = applied(&first, &create_curtain_panel_override(&first, "ov-1", "cw", "").expect("a create"));
    assert_eq!(second.curtain_panel_overrides["ov-1"].u, 1, "the next free cell");
    assert_eq!(override_name(&second.curtain_panel_overrides["ov-1"]), "cw (1, 0)");
}

#[test]
fn the_rows_of_a_curtain_wall_type_write_sparse_type_mutations() {
    let base = with_wall();
    let field = |key: &str| CURTAIN_WALL_TYPE_FIELDS.iter().find(|row| row.key == key).unwrap_or_else(|| panic!("row {key}"));
    for (key, value) in [("u_grid", "lines 2, 4"), ("v_grid", "spacing 1"), ("interior_mullion", "rectangle 0.04 × 0.12"), ("panel", "window win-1"), ("panel_material", "m-steel"), ("name", "Renamed")] {
        let write = field(key).write.unwrap_or_else(|| panic!("{key} is editable"));
        let mutation = write(&base, "cwt", value).unwrap_or_else(|| panic!("{key} decodes '{value}'"));
        assert!(matches!(mutation, ModelMutation::SetCurtainWallType(_)), "{key}");
    }
    assert!(field("u_grid").write.expect("editable")(&base, "cwt", "cells").is_none());
    assert!(field("panel").write.expect("editable")(&base, "cwt", "door").is_none());
}

#[test]
fn the_rows_of_a_curtain_wall_write_their_own_mutations() {
    let base = with_wall();
    assert!(matches!(write_curtain_wall_type(&base, "cw", "cwt"), Some(ModelMutation::SetCurtainWallTypeOf(_))));
    assert!(matches!(write_u_grid(&base, "cw", "lines 1, 2"), Some(ModelMutation::SetCurtainWallGrid(_))));
    assert!(matches!(write_v_grid(&base, "cw", "type"), Some(ModelMutation::SetCurtainWallGrid(_))));
    assert!(write_u_grid(&base, "cw", "tiles").is_none());
    let ModelMutation::SetCurtainWallGrid(payload) = write_u_grid(&base, "cw", "type").expect("a write") else { panic!("grid") };
    assert_eq!(payload.u_grid.map(|assigned| assigned.value), Some(None), "type clears the override");
    assert!(payload.v_grid.is_none(), "the other direction stays untouched");
}

#[test]
fn the_tilt_and_the_beam_rows_write_their_own_mutations() {
    let base = with_wall();
    let Some(ModelMutation::SetColumnTilt(payload)) = write_tilt(&base, "c", "1.5, 0.2") else { panic!("tilt") };
    assert_eq!(payload.tilt, Some(crate::Slope { direction: 1.5, angle: 0.2 }));
    let Some(ModelMutation::SetColumnTilt(plumb)) = write_tilt(&base, "c", "") else { panic!("tilt") };
    assert_eq!(plumb.tilt, None, "empty text makes the column plumb");
    assert!(write_tilt(&base, "c", "lean").is_none());
    assert!(matches!(write_beam_axis(&base, "b", "0, 0 → 4, 0 ⌒ 0.3"), Some(ModelMutation::SetBeamAxis(_))));
    assert!(write_beam_axis(&base, "b", "0, 0").is_none());
    let Some(ModelMutation::SetBeam(offset)) = write_end_offset(&base, "b", "0.4") else { panic!("end offset") };
    assert_eq!(offset.end_top_offset.map(|assigned| assigned.value), Some(Some(0.4)));
    let Some(ModelMutation::SetBeam(level)) = write_end_offset(&base, "b", "") else { panic!("end offset") };
    assert_eq!(level.end_top_offset.map(|assigned| assigned.value), Some(None), "empty text makes the beam level");
}

#[test]
fn the_choices_list_the_types_and_materials_by_name() {
    let base = with_wall();
    let labels = BimLabels::NATIVE_EN;
    assert_eq!(type_choices(&base, &labels), vec![("cwt".to_string(), "Facade".to_string())]);
    assert_eq!(material_choices(&base, &labels), vec![("m-steel".to_string(), "Steel".to_string())]);
}

#[test]
fn the_entity_rows_name_and_parent_their_records() {
    let base = with_wall();
    let created = applied(&base, &create_curtain_panel_override(&base, "ov", "cw", "").expect("a create"));
    assert_eq!((CURTAIN_PANEL_OVERRIDE.name)(&created, "ov").as_deref(), Some("cw (0, 0)"));
    assert_eq!((CURTAIN_PANEL_OVERRIDE.parent)(&created, "ov").as_deref(), Some("cw"));
    assert_eq!((CURTAIN_WALL_TYPE.name)(&created, "cwt").as_deref(), Some("Facade"));
    assert!(CURTAIN_WALL_TYPE.library && !CURTAIN_PANEL_OVERRIDE.library);
    let used = CURTAIN_WALL_TYPE.inferred.iter().find(|row| row.key == "used_by").expect("used by");
    assert_eq!((used.read)(&created, &crate::ModelInference::default(), "cwt").as_deref(), Some("1"));
}
