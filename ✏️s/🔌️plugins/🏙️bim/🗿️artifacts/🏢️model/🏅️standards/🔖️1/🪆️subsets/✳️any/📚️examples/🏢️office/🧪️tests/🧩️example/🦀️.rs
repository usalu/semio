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
async fn bless_the_office_text() {
    ASSET.bless();
}
