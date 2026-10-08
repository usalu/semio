use super::{ASSET_DIR, PRIMARY_TEXT, SNAPSHOT_JSON};
use crate::examples::checks::Asset;
use crate::{Axis, ModelSnapshot, OpeningKind, SpaceBoundary};

const ASSET: Asset = Asset { dir: ASSET_DIR, text: PRIMARY_TEXT, json: SNAPSHOT_JSON };

fn walls_on(model: &ModelSnapshot, storey: &str) -> usize {
    model.walls.values().filter(|wall| wall.storey == storey).count()
}

fn near(left: f64, right: f64) -> bool {
    (left - right).abs() < 1e-9
}

#[semio_framework_async_macros::async_test]
async fn the_house_is_the_documented_building() {
    let model = ASSET.model();
    assert_eq!((model.sites.len(), model.buildings.len(), model.storeys.len(), model.grids.len()), (1, 1, 4, 6));
    let mut levels: Vec<i32> = model.storeys.values().map(|storey| storey.level).collect();
    levels.sort();
    assert_eq!(levels, vec![-1, 0, 1, 2]);
    assert_eq!((walls_on(&model, "st-basement"), walls_on(&model, "st-ground"), walls_on(&model, "st-upper"), walls_on(&model, "st-attic")), (7, 9, 10, 4));
    assert_eq!(model.walls.values().filter(|wall| matches!(wall.axis, Axis::Arc { .. })).count(), 3, "one bay wall on the basement, ground and upper storey");
    let windows = model.openings.values().filter(|opening| matches!(opening.kind, OpeningKind::Window { .. })).count();
    let doors = model.openings.values().filter(|opening| matches!(opening.kind, OpeningKind::Door { .. })).count();
    let voids = model.openings.values().filter(|opening| matches!(opening.kind, OpeningKind::Void { .. })).count();
    assert_eq!((windows, doors, voids), (25, 9, 1));
    assert_eq!((model.slabs.len(), model.roofs.len(), model.stairs.len(), model.railings.len(), model.columns.len(), model.beams.len()), (4, 2, 2, 2, 1, 1));
    assert!(model.slabs.values().any(|slab| !slab.holes.is_empty()), "a slab carries a stair hole");
    assert_eq!(model.spaces.len(), 12);
    assert!(model.spaces.values().all(|space| matches!(space.boundary, SpaceBoundary::Bounded { .. })));
    assert_eq!((model.materials.len(), model.wall_types.len(), model.slab_types.len(), model.roof_types.len()), (10, 6, 3, 2));
    assert_eq!((model.properties.len(), model.classifications.len()), (6, 6));
    let site = model.sites.values().next().expect("a site");
    assert!((46.0..48.0).contains(&site.latitude) && (7.0..8.0).contains(&site.longitude) && site.boundary.len() == 4);
}

#[semio_framework_async_macros::async_test]
async fn the_house_round_trips_and_validates() {
    ASSET.assert_text_is_the_codec_fixed_point();
    ASSET.assert_pack_round_trip();
    ASSET.assert_schema_valid();
    let model = ASSET.model();
    assert_eq!(crate::examples::checks::dangling(&model), Vec::<String>::new());
}

#[semio_framework_async_macros::async_test]
async fn the_house_infers_levels_heights_and_valid_openings() {
    let model = ASSET.model();
    let inferred = crate::examples::checks::infer(&model);
    let level = |id: &str| inferred.storey_levels[id];
    assert!(near(level("st-basement").elevation, -2.6) && near(level("st-basement").top_elevation, 0.0));
    assert!(near(level("st-ground").elevation, 0.0) && near(level("st-ground").top_elevation, 2.8));
    assert!(near(level("st-upper").elevation, 2.8) && near(level("st-upper").top_elevation, 5.5));
    assert!(near(level("st-attic").elevation, 5.5) && near(level("st-attic").top_elevation, 7.9));
    assert!(near(level("st-ground").absolute_elevation, 542.3), "site 542 m plus the 0.3 m plinth");
    let heights = |storey: &str| -> Vec<f64> { model.walls.iter().filter(|(_, wall)| wall.storey == storey).map(|(id, _)| inferred.wall_layout[id].height).collect() };
    assert!(heights("st-basement").iter().all(|height| near(*height, 2.6)));
    assert!(heights("st-ground").iter().all(|height| near(*height, 2.8)));
    assert!(heights("st-upper").iter().all(|height| near(*height, 2.7)));
    assert!(heights("st-attic").iter().all(|height| near(*height, 0.9)), "knee walls stay 0.9 m high");
    let south = &inferred.wall_layout["w-g-south"];
    assert!(near(south.thickness, 0.37) && near(south.length, 10.0) && near(south.base_z, 0.0) && near(south.top_z, 2.8));
    let bay = &inferred.wall_layout["w-g-bay"];
    assert!((bay.length - 3.477356).abs() < 1e-5, "the bay is a 106 degree arc of radius 1.875 m, got {}", bay.length);
    assert!(near(inferred.wall_layout["w-g-spine"].thickness, 0.205) && near(inferred.wall_layout["w-g-cross"].thickness, 0.205) && near(inferred.wall_layout["w-g-wc"].thickness, 0.145));
    assert_eq!(inferred.stair_runs["sr-main"].riser_count, 15);
    assert_eq!(inferred.stair_runs["sr-cellar"].riser_count, 13);
}

#[semio_framework_async_macros::async_test]
async fn a_storey_height_edit_moves_every_storey_above_it() {
    use protocol::Inference;
    let mut model = ASSET.model();
    model.storeys.get_mut("st-ground").expect("the ground storey").height = 3.0;
    let inferred = crate::ModelInference::infer(&model).expect("infers");
    assert!(near(inferred.storey_levels["st-upper"].elevation, 3.0) && near(inferred.storey_levels["st-attic"].elevation, 5.7));
    assert!(near(inferred.wall_layout["w-g-south"].height, 3.0) && near(inferred.wall_layout["w-u-south"].base_z, 3.0) && near(inferred.wall_layout["w-a-south"].base_z, 5.7));
    assert!(near(inferred.wall_layout["w-b-south"].top_z, 0.0), "the basement below does not move");
}

#[semio_framework_async_macros::async_test]
async fn the_house_quantities_and_rooms_match_the_hand_calculation() {
    let model = ASSET.model();
    let inferred = crate::examples::checks::infer(&model);
    let quantity = |id: &str| &inferred.quantities.elements[id];
    let bay = 1.875f64 * 1.875 / 2.0 * (4.0 * 0.5f64.atan() - 0.96);
    let ground = quantity("sl-g");
    assert!((ground.gross_area - (80.0 + bay)).abs() < 1e-6, "an 10 x 8 m slab plus a circular segment of radius 1.875 m, got {}", ground.gross_area);
    assert!((ground.net_area - (80.0 + bay - 0.9 * 2.69)).abs() < 1e-6, "minus the cellar stair hole");
    assert!((ground.net_volume - ground.net_area * 0.3).abs() < 1e-9, "a 30 cm floor slab");
    let south = quantity("w-g-south");
    assert!(near(south.gross_side_area, 28.0) && (south.opening_area - (1.0 * 2.15 + 1.8 * 1.4 + 1.6 * 2.25)).abs() < 1e-9, "door, picture window and patio door cut the south wall");
    assert!((south.net_side_area - (28.0 - 8.27)).abs() < 1e-9);
    let totals = &inferred.quantities.project.kinds;
    assert_eq!((totals["wall"].count, totals["window"].count, totals["door"].count, totals["void"].count, totals["slab"].count, totals["stair"].count, totals["space"].count), (30, 25, 9, 1, 4, 2, 12));
    let wc = &inferred.spaces["sp-g2"];
    assert!((wc.area - (1.7275 - 0.185) * (7.815 - 5.1025)).abs() < 1e-6, "the WC is bounded by the faces of four walls, got {}", wc.area);
    assert!(wc.clear_height > 2.0, "a room is taller than two metres, got {}", wc.clear_height);
}

#[semio_framework_async_macros::async_test]
async fn the_house_is_reachable_through_the_mutation_api() {
    crate::examples::checks::assert_replay("house", &ASSET.model());
}

#[semio_framework_async_macros::async_test]
async fn bless_the_house_text() {
    ASSET.bless();
}
