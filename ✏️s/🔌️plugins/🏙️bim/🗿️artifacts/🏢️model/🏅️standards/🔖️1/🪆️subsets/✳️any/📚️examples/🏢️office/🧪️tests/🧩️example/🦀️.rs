use super::{ASSET_DIR, PRIMARY_TEXT, SNAPSHOT_JSON};
use crate::examples::checks::Asset;
use crate::{ModelSnapshot, OpeningKind};

const ASSET: Asset = Asset { dir: ASSET_DIR, text: PRIMARY_TEXT, json: SNAPSHOT_JSON };

fn walls_on(model: &ModelSnapshot, storey: &str) -> usize {
    model.walls.values().filter(|wall| wall.storey == storey).count()
}

fn near(left: f64, right: f64) -> bool {
    (left - right).abs() < 1e-9
}

#[semio_framework_async_macros::async_test]
async fn the_office_is_the_documented_building() {
    let model = ASSET.model();
    assert_eq!((model.sites.len(), model.buildings.len(), model.storeys.len(), model.grids.len()), (1, 1, 5, 10));
    assert_eq!((walls_on(&model, "st-0"), walls_on(&model, "st-1"), walls_on(&model, "st-2"), walls_on(&model, "st-3"), walls_on(&model, "st-r")), (14, 14, 14, 14, 4));
    assert_eq!((model.curtain_walls.len(), model.columns.len(), model.beams.len()), (8, 96, 152));
    assert_eq!((model.slabs.len(), model.roofs.len(), model.stairs.len(), model.spaces.len()), (4, 1, 6, 28));
    let windows = model.openings.values().filter(|opening| matches!(opening.kind, OpeningKind::Window { .. })).count();
    let doors = model.openings.values().filter(|opening| matches!(opening.kind, OpeningKind::Door { .. })).count();
    assert_eq!((windows, doors), (24, 17));
    assert_eq!(model.slabs.values().map(|slab| slab.holes.len()).sum::<usize>(), 6, "two stair shafts through each of the three upper slabs");
    let on_grid = |value: f64, axis: bool| model.grids.values().any(|grid| if axis { near(grid.start.x, value) && near(grid.end.x, value) } else { near(grid.start.y, value) && near(grid.end.y, value) });
    assert!(model.columns.values().all(|column| on_grid(column.position.x, true) && on_grid(column.position.y, false)), "every column stands on a grid intersection");
    assert!(model.beams.values().all(|beam| (on_grid(beam.start.x, true) && on_grid(beam.end.x, true)) || (on_grid(beam.start.y, false) && on_grid(beam.end.y, false))), "every beam runs along a grid line");
}

#[semio_framework_async_macros::async_test]
async fn the_office_schedules_list_every_door_window_room_and_finish() {
    let model = ASSET.model();
    let inferred = crate::examples::checks::infer(&model);
    let items = |id: &str| inferred.schedules[id].items as usize;
    assert_eq!((items("sch-doors"), items("sch-windows")), (17, 24), "a door and a window schedule list every one of them");
    let finished = model.spaces.keys().filter(|id| inferred.quantities.elements.get(*id).is_some_and(|quantity| !quantity.finishes.is_empty())).count();
    assert_eq!(items("sch-finishes"), 3 * finished, "a floor, a wall and a ceiling row for every resolved room");
    assert!(finished > 0 && model.spaces.values().any(|space| space.floor_finish.is_some()), "the finishes come from the authored rooms");
}

#[semio_framework_async_macros::async_test]
async fn the_office_round_trips_and_validates() {
    ASSET.assert_text_is_the_codec_fixed_point();
    ASSET.assert_pack_round_trip();
    ASSET.assert_schema_valid();
    let model = ASSET.model();
    assert_eq!(crate::examples::checks::dangling(&model), Vec::<String>::new());
}

#[semio_framework_async_macros::async_test]
async fn the_office_infers_levels_heights_and_valid_openings() {
    let model = ASSET.model();
    let inferred = crate::examples::checks::infer(&model);
    for (id, elevation, top) in [("st-0", 0.0, 4.0), ("st-1", 4.0, 7.8), ("st-2", 7.8, 11.6), ("st-3", 11.6, 15.4), ("st-r", 15.4, 16.6)] {
        let level = inferred.storey_levels[id];
        assert!(near(level.elevation, elevation) && near(level.top_elevation, top), "{id}");
        assert!(near(level.absolute_elevation, 408.0 + elevation), "{id} sits on the 408 m site");
    }
    assert!(near(inferred.wall_layout["w-0-core-w-south"].height, 4.0) && near(inferred.wall_layout["w-0-west-1"].height, 3.14), "cores run the full storey, infill panels stop under the edge beam");
    assert!(model.walls.iter().filter(|(_, wall)| wall.storey == "st-r").all(|(id, _)| near(inferred.wall_layout[id].height, 1.0)), "parapets are 1 m");
    assert_eq!(inferred.curtain_layout.len(), 8);
    assert!(near(inferred.wall_layout["w-0-core-w-south"].thickness, 0.25) && near(inferred.wall_layout["w-0-west-1"].thickness, 0.34));
    assert_eq!(inferred.stair_runs["sr-0-w"].riser_count, 20);
    assert_eq!(inferred.stair_runs["sr-1-e"].riser_count, 19);
}

#[semio_framework_async_macros::async_test]
async fn a_storey_height_edit_moves_every_storey_above_it() {
    use protocol::Inference;
    let mut model = ASSET.model();
    model.storeys.get_mut("st-1").expect("the first storey").height = 4.2;
    let inferred = crate::ModelInference::infer(&model).expect("infers");
    assert!(near(inferred.storey_levels["st-2"].elevation, 8.2) && near(inferred.storey_levels["st-r"].elevation, 15.8));
    assert!(near(inferred.wall_layout["w-1-west-1"].height, 3.34) && near(inferred.wall_layout["w-3-west-1"].base_z, 12.0) && near(inferred.wall_layout["w-0-west-1"].height, 3.14) && near(inferred.wall_layout["w-1-core-w-south"].height, 4.2));
}

#[semio_framework_async_macros::async_test]
async fn the_office_quantities_and_rooms_match_the_hand_calculation() {
    let model = ASSET.model();
    let inferred = crate::examples::checks::infer(&model);
    let quantity = |id: &str| &inferred.quantities.elements[id];
    let floor = quantity("sl-1");
    assert!((floor.gross_area - 30.6 * 18.6).abs() < 1e-9 && (floor.net_area - (30.6 * 18.6 - 2.0 * 2.7 * 4.9)).abs() < 1e-9, "a 30.6 x 18.6 m slab with two 2.7 x 4.9 m shafts");
    assert!((floor.net_volume - floor.net_area * 0.36).abs() < 1e-9, "a 36 cm floor slab");
    assert!((quantity("c-0-A1").gross_volume - 0.5 * 0.5 * 4.0).abs() < 1e-9 && (quantity("c-3-F4").gross_volume - 0.4 * 0.4 * 3.8).abs() < 1e-9);
    assert!((quantity("bm-0-x-A1").gross_volume - 6.0 * 0.4 * 0.6).abs() < 1e-9 && (quantity("bm-2-y-C2").gross_volume - 6.0 * 0.3 * 0.5).abs() < 1e-9);
    let totals = &inferred.quantities.project.kinds;
    assert_eq!((totals["wall"].count, totals["curtain-wall"].count, totals["column"].count, totals["beam"].count, totals["window"].count, totals["door"].count, totals["stair"].count, totals["space"].count), (60, 8, 96, 152, 24, 17, 6, 28));
    assert!((inferred.spaces["sp-0-1"].area - 180.0).abs() < 1e-9, "the lobby is the 30 x 6 m south band");
    assert!((inferred.spaces["sp-2-6"].area - 2.75 * 4.95).abs() < 1e-6, "a stair core is bounded by the inner faces of its four 25 cm walls, got {}", inferred.spaces["sp-2-6"].area);
}

#[semio_framework_async_macros::async_test]
async fn the_office_is_reachable_through_the_mutation_api() {
    crate::examples::checks::assert_replay("office", &ASSET.model());
}

#[semio_framework_async_macros::async_test]
async fn the_ground_floor_frame_is_existing_and_its_north_curtain_wall_demolished() {
    use crate::standards::v1::subsets::any::schema::inferences::phase_visibility::ViewPhase;
    let model = ASSET.model();
    assert!(model.columns.values().filter(|column| column.storey == "st-0").all(|column| column.phase == crate::Phase::Existing));
    assert!(model.columns.values().filter(|column| column.storey == "st-1").all(|column| column.phase == crate::Phase::New));
    assert_eq!((model.slabs["sl-0"].phase, model.curtain_walls["cu-0-north"].phase), (crate::Phase::Existing, crate::Phase::Demolished));
    let inferred = crate::examples::checks::infer(&model);
    assert_eq!(inferred.phase_visibility["st-0"].ids(ViewPhase::Demolished), ["cu-0-north"]);
    assert!(inferred.phase_visibility["st-0"].ids(ViewPhase::Existing).len() > 24, "24 columns and the slab");
    assert!(inferred.phase_visibility["st-1"].ids(ViewPhase::Existing).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn bless_the_office_text() {
    ASSET.bless();
}

#[semio_framework_async_macros::async_test]
async fn the_example_views_are_the_ones_the_command_makes() {
    crate::examples::checks::assert_views_are_the_commands("office", &ASSET.model());
}

#[semio_framework_async_macros::async_test]
async fn the_office_dimensions_tags_notes_and_leaders_print_what_the_geometry_says() {
    let model = ASSET.model();
    assert_eq!((model.annotation_styles.len(), model.dimensions.len(), model.tags.len(), model.text_notes.len(), model.leaders.len()), (2, 5, 3, 1, 1));
    let inferred = crate::examples::checks::infer(&model);
    let ground = &inferred.annotations["st-0"];
    let close = |found: f64, expected: f64| (found - expected).abs() < 1e-9;
    assert!(close(ground.dimensions["dim-0-axes-x"].total, 30.0) && close(ground.dimensions["dim-0-axes-y"].total, 18.0) && close(ground.dimensions["dim-0-core-west"].total, 3.0));
    assert!(ground.dimensions["dim-0-axes-x"].segments.iter().all(|segment| close(segment.length, 6.0)) && ground.dimensions["dim-0-axes-x"].segments.len() == 5);
    assert!(close(ground.dimensions["dim-0-core-clear"].total, 5.2 - inferred.wall_layout["w-0-core-w-south"].thickness), "the clear depth is the core depth less one wall thickness");
    let text = |id: &str| ground.tags[id].text.as_str();
    assert_eq!((text("tag-0-column"), text("tag-0-lobby"), text("tag-0-core")), ("0.50 \u{d7} 0.50", "0.01", "Core Wall 25"));
    assert_eq!(ground.notes["note-0-entry"].text, "Main entrance, barrier-free");
    assert_eq!(ground.leaders["lead-0-south"].text, "Curtain wall, see detail");
    assert!(close(inferred.annotations["st-1"].dimensions["dim-1-axes-x"].total, 12.0));
    assert!(inferred.annotations.values().all(|set| set.findings.is_empty()), "no annotation raises a finding");
}

#[semio_framework_async_macros::async_test]
async fn the_office_zones_area_schemes_and_room_finishes_add_up() {
    let model = ASSET.model();
    assert_eq!((model.zones.len(), model.area_schemes.len()), (2, 3));
    assert!(model.spaces.values().all(|space| space.zone.is_some()), "every room of the office is in a zone");
    let inferred = crate::examples::checks::infer(&model);
    let net: f64 = inferred.spaces.values().map(|room| room.net_floor_area).sum();
    assert!((inferred.scheme_totals["as-nfa"].area - net).abs() < 1e-9 && inferred.scheme_totals["as-nfa"].spaces == model.spaces.len() as u32);
    let offices = model.spaces.values().filter(|space| space.usage == "Office" && space.zone.as_deref() == Some("z-office")).count();
    assert_eq!(inferred.scheme_totals["as-office"].spaces as usize, offices, "the scheme counts the office usage in the office zone only");
    let office = &inferred.zone_totals["z-office"];
    assert!((office.occupancy - 0.1 * office.net_area).abs() < 1e-9 && office.floor_finish_area > 0.0 && office.ceiling_finish_area > 0.0);
}

#[semio_framework_async_macros::async_test]
async fn the_suspended_ceilings_are_the_surface_the_office_ceiling_finish_covers() {
    let model = ASSET.model();
    let inferred = crate::examples::checks::infer(&model);
    let room = &inferred.spaces["sp-1-1"];
    assert!(!room.ceiling.is_empty(), "a suspended ceiling hangs over the open office");
    let area = inferred.quantities.elements["sp-1-1"].finishes.iter().find(|row| row.surface == crate::standards::v1::subsets::any::schema::inferences::finishes::FinishSurface::Ceiling).expect("a ceiling row").area;
    assert!((area - room.net_floor_area).abs() < 1e-9, "flat ceiling and flat slab: the ceiling surface is the net floor");
    assert!(inferred.spaces["sp-1-3"].ceiling.is_empty(), "the circulation west room has no suspended ceiling");
}

#[semio_framework_async_macros::async_test]
async fn a_column_row_is_the_array_of_its_first_column_and_a_beam_the_copy_of_its_neighbour() {
    use crate::mutations::array_elements::ArrayElements;
    use crate::mutations::copy_elements::CopyElements;
    use crate::mutations::modify::ArrayPattern;
    use crate::{ModelMutation, Point2};
    let model = ASSET.model();
    let row = ["B", "C", "D", "E", "F"].map(|label| format!("c-0-{label}1"));
    let mut bare = model.clone();
    for id in &row {
        bare.columns.remove(id);
    }
    let array = ModelMutation::ArrayElements(ArrayElements { ids: vec!["c-0-A1".into()], prefix: "row".into(), pattern: ArrayPattern::Linear { count: 5, spacing: Point2 { x: 6.0, y: 0.0 } } });
    let arrayed = crate::mutations::apply_model_mutation(&bare, &array).expect("the row is arrayed");
    for (copy, id) in row.iter().enumerate() {
        let made = &arrayed.columns[&crate::mutations::modify::mint("row", copy as u32 + 1, 0)];
        let authored = &model.columns[id];
        assert_eq!((made.storey.as_str(), made.column_type.as_str(), made.position, &made.top, made.phase), (authored.storey.as_str(), authored.column_type.as_str(), authored.position, &authored.top, authored.phase), "{id}");
    }
    let neighbour = &model.beams["bm-0-x-A2"];
    let copied = crate::mutations::apply_model_mutation(&model, &ModelMutation::CopyElements(CopyElements { ids: vec!["bm-0-x-A1".into()], vector: Point2 { x: 0.0, y: 6.0 }, prefix: "cp".into() })).expect("the beam is copied");
    let made = &copied.beams[&crate::mutations::modify::mint("cp", 1, 0)];
    assert_eq!((made.beam_type.as_str(), made.start, made.end, made.top_offset), (neighbour.beam_type.as_str(), neighbour.start, neighbour.end, neighbour.top_offset));
}
