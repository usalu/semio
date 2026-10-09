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
    assert_eq!((model.slabs.len(), model.roofs.len(), model.stairs.len(), model.railings.len(), model.columns.len(), model.beams.len()), (4, 2, 2, 4, 1, 1), "the stair opening, the cellar stair and the two railings hosted by the ramp");
    assert!(model.slabs.values().any(|slab| !slab.holes.is_empty()), "a slab carries a stair hole");
    assert_eq!(model.spaces.len(), 12);
    assert!(model.spaces.values().all(|space| matches!(space.boundary, SpaceBoundary::Bounded { .. })));
    assert_eq!((model.materials.len(), model.wall_types.len(), model.slab_types.len(), model.roof_types.len()), (13, 6, 3, 2), "ten building materials and the oak, ceramic and paint of the room finishes");
    assert_eq!((model.properties.len(), model.classifications.len()), (6, 6));
    let site = model.sites.values().next().expect("a site");
    assert!((46.0..48.0).contains(&site.latitude) && (7.0..8.0).contains(&site.longitude) && site.boundary.len() == 4);
}

#[semio_framework_async_macros::async_test]
async fn the_house_schedules_list_every_door_window_room_and_finish_of_the_building() {
    let model = ASSET.model();
    let inferred = crate::examples::checks::infer(&model);
    let items = |id: &str| inferred.schedules[id].items as usize;
    assert_eq!((items("sch-doors"), items("sch-windows"), items("sch-rooms")), (9, 25, 12), "a door, a window and a room schedule list every one of them");
    assert_eq!(items("sch-finishes"), 3 * model.spaces.len(), "a floor, a wall and a ceiling row for every room");
    use crate::standards::v1::subsets::any::schema::inferences::schedules::rows::{phase_of, storey_of};
    let ground_new_doors = model.openings.iter().filter(|(id, opening)| matches!(opening.kind, OpeningKind::Door { .. }) && storey_of(&model, id).is_some_and(|storey| storey == "st-ground") && phase_of(&model, id) == crate::Phase::New).count();
    assert!(ground_new_doors > 0 && ground_new_doors < items("sch-doors"), "the ground floor has some of the doors");
    assert_eq!(items("sch-doors-ground"), ground_new_doors, "the scoped door schedule lists the new doors of the ground floor only");
    assert!(model.schedules["sch-doors-ground"].storeys == ["st-ground"], "its scope is authored");
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
    crate::examples::checks::assert_replay_derived("house", &ASSET.model(), super::DERIVATIONS);
}

#[semio_framework_async_macros::async_test]
async fn the_attic_north_and_west_walls_are_the_mirror_images_of_the_south_and_east_walls() {
    let model = ASSET.model();
    let derived: serde_json::Value = serde_json::from_str(super::DERIVATIONS).expect("the derivations are JSON");
    assert_eq!(derived["derived"].as_array().map(Vec::len), Some(2));
    for (made, source) in [("w-a-north-1-0", "w-a-south"), ("w-a-west-1-0", "w-a-east")] {
        assert!(model.walls.contains_key(made) && !model.walls.contains_key(&made.replace("-1-0", "")), "{made} carries the id its mirror mints");
        assert_eq!(model.walls[made].wall_type, model.walls[source].wall_type);
    }
    assert!(near(crate::examples::checks::infer(&model).wall_layout["w-a-north-1-0"].height, crate::examples::checks::infer(&model).wall_layout["w-a-south"].height), "a mirrored wall stands as high as its source");
}

#[semio_framework_async_macros::async_test]
async fn the_basement_is_existing_the_wc_partition_demolished_and_the_rest_new() {
    use crate::standards::v1::subsets::any::schema::inferences::phase_visibility::ViewPhase;
    let model = ASSET.model();
    assert!(model.walls.values().filter(|wall| wall.storey == "st-basement").all(|wall| wall.phase == crate::Phase::Existing), "the basement walls are existing");
    assert_eq!((model.slabs["sl-b"].phase, model.stairs["sr-cellar"].phase, model.spaces["sp-b1"].phase), (crate::Phase::Existing, crate::Phase::Existing, crate::Phase::Existing));
    assert_eq!(model.walls["w-g-wc"].phase, crate::Phase::Demolished);
    assert_eq!(model.walls["w-g-south"].phase, crate::Phase::New);
    let inferred = crate::examples::checks::infer(&model);
    assert_eq!(inferred.phase_visibility["st-ground"].ids(ViewPhase::Demolished), ["w-g-wc"]);
    assert!(inferred.phase_visibility["st-basement"].ids(ViewPhase::New).is_empty(), "nothing new stands in the basement");
    assert!(inferred.phase_visibility["st-ground"].ids(ViewPhase::Existing).is_empty());
    assert!(inferred.phase_visibility["st-upper"].ids(ViewPhase::New).contains(&"w-u-bath-east".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn the_bathroom_east_wall_replays_as_a_move_from_the_ground_to_the_upper_floor() {
    let model = ASSET.model();
    let derived: serde_json::Value = serde_json::from_str(super::DERIVATIONS).expect("the derivations are JSON");
    assert_eq!(derived["moved"], serde_json::json!([{ "id": "w-u-bath-east", "from": "st-ground", "to": "st-upper" }]));
    assert_eq!(model.walls["w-u-bath-east"].storey, "st-upper");
    assert!(crate::examples::checks::replay_derived(&model, super::DERIVATIONS).applied > model.walls.len(), "the move is one mutation more than the creates");
}

#[semio_framework_async_macros::async_test]
async fn bless_the_house_text() {
    ASSET.bless();
}

#[semio_framework_async_macros::async_test]
async fn the_example_views_are_the_ones_the_command_makes() {
    crate::examples::checks::assert_views_are_the_commands("house", &ASSET.model());
}

#[semio_framework_async_macros::async_test]
async fn the_house_dimensions_tags_notes_and_leaders_print_what_the_geometry_says() {
    let model = ASSET.model();
    assert_eq!((model.annotation_styles.len(), model.dimensions.len(), model.tags.len(), model.text_notes.len(), model.leaders.len()), (2, 5, 4, 1, 1));
    let inferred = crate::examples::checks::infer(&model);
    let set = &inferred.annotations["st-ground"];
    let total = |id: &str| set.dimensions[id].total;
    assert!(near(total("dim-g-south"), 10.0) && near(total("dim-g-west"), 8.0) && near(total("dim-g-grids"), 10.0));
    let lengths = |id: &str| set.dimensions[id].segments.iter().map(|segment| segment.length).collect::<Vec<_>>();
    assert!(lengths("dim-g-south-chain").iter().zip([2.9, 3.0, 4.1]).all(|(found, expected)| (found - expected).abs() < 1e-9), "{:?}", lengths("dim-g-south-chain"));
    assert!(lengths("dim-g-grids").iter().zip([3.6, 6.4]).all(|(found, expected)| (found - expected).abs() < 1e-9));
    assert!((total("dim-g-thickness") - inferred.wall_layout["w-g-south"].thickness).abs() < 1e-9, "the dimension prints the thickness of the wall type");
    assert_eq!(set.dimensions["dim-g-south"].total_text, "10.00");
    assert_eq!(set.dimensions["dim-g-thickness"].total_text.len(), 3, "the detail style prints whole millimetres");
    let text = |id: &str| set.tags[id].text.as_str();
    assert_eq!((text("tag-g-south-name"), text("tag-g-south-type"), text("tag-g-hall")), ("Ground South", "Exterior Wall 37", "0.01"));
    assert!(set.tags["tag-g-entry-size"].text.contains('\u{d7}'));
    assert_eq!(set.notes["note-g-site"].text, "Check the plinth level on site");
    assert_eq!(set.leaders["lead-g-north"].text, "Clay block cavity wall");
    assert!(set.findings.is_empty(), "no annotation raises a finding: {:?}", set.findings);
    assert_eq!(inferred.annotations.len(), 1, "only the ground floor is annotated");
}

#[semio_framework_async_macros::async_test]
async fn the_entrance_ramp_is_a_compliant_bent_ramp_that_meets_the_front_door_and_hosts_two_railings() {
    use crate::HostSide;
    let model = ASSET.model();
    assert_eq!(model.ramps.keys().collect::<Vec<_>>(), ["rp-entry"]);
    let hosted: Vec<_> = model.railings.iter().filter(|(_, railing)| railing.host.as_ref().is_some_and(|host| host.element == "rp-entry")).collect();
    assert_eq!(hosted.len(), 2);
    assert!(hosted.iter().any(|(_, railing)| railing.host.as_ref().is_some_and(|host| host.side == HostSide::Left)) && hosted.iter().any(|(_, railing)| railing.host.as_ref().is_some_and(|host| host.side == HostSide::Right)));
    let inferred = crate::examples::checks::infer(&model);
    let run = &inferred.ramp_runs["rp-entry"];
    assert!(near(run.length, 11.0) && near(run.run_length, 6.5) && near(run.rise, 0.5), "11 m of path, 6.5 m of slope between three 1.5 m landings, 0.5 m of rise: {run:?}");
    assert!((run.slope - 0.5 / 6.5).abs() < 1e-12 && run.slope < model.ramps["rp-entry"].max_slope && run.compliance.compliant);
    assert_eq!((run.flights.len(), run.landings.len()), (2, 3), "the corner has a landing");
    assert!(near(run.base_z, -0.5) && near(run.top_z, 0.0), "the head lies at the level of the ground floor, the foot at the terrain");
    let door = &inferred.opening_frames["o-g-entry"];
    let head = model.ramps["rp-entry"].path.last().expect("a head").point;
    assert!((head.x - 2.9).abs() < 1e-9 && head.y < 0.0 && door.valid, "the ramp ends in front of the door, which it centres on");
    assert!(inferred.element_solids.contains_key("rp-entry") && hosted.iter().all(|(id, _)| inferred.element_solids[id.as_str()].volume > 0.0), "the ramp and both hosted railings have bodies");
    assert_eq!(inferred.quantities.project.kinds["ramp"].count, 1);
    assert!(near(inferred.quantities.elements["rp-entry"].gross_area, 1.2 * 11.0) && near(inferred.quantities.elements["rp-entry"].net_volume, 1.2 * 11.0 * 0.2));
}

#[semio_framework_async_macros::async_test]
async fn raising_the_ground_floor_height_does_not_move_the_entrance_ramp() {
    let mut model = ASSET.model();
    model.storeys.get_mut("st-ground").expect("the ground storey").height = 3.0;
    use protocol::Inference;
    let inferred = crate::ModelInference::infer(&model).expect("infers");
    assert!(near(inferred.ramp_runs["rp-entry"].rise, 0.5) && near(inferred.ramp_runs["rp-entry"].base_z, -0.5), "an unconnected ramp keeps its authored rise and foot");
}

#[semio_framework_async_macros::async_test]
async fn the_house_zones_area_schemes_and_room_finishes_add_up() {
    let model = ASSET.model();
    assert_eq!((model.zones.len(), model.area_schemes.len()), (3, 4));
    assert!(model.spaces.values().all(|space| space.zone.is_some()), "every room of the house is in a zone");
    let inferred = crate::examples::checks::infer(&model);
    let gross: f64 = inferred.spaces.values().map(|room| room.area).sum();
    let net: f64 = inferred.spaces.values().map(|room| room.net_floor_area).sum();
    assert!(near(inferred.scheme_totals["as-gfa"].area, gross) && near(inferred.scheme_totals["as-nfa"].area, net), "the gross and net schemes count every room");
    assert!(near(inferred.zone_totals.values().map(|zone| zone.area).sum::<f64>(), gross), "the zones partition the rooms");
    assert_eq!(inferred.scheme_totals["as-day"].spaces, 4, "the hall, the cellar stair room, the living room and the kitchen");
    assert_eq!(inferred.scheme_totals["as-living"].spaces, 4, "the living room, the kitchen and the two bedrooms");
    let day = &inferred.zone_totals["z-day"];
    assert!(near(day.occupancy, 0.05 * day.net_area) && day.floor_finish_area > 0.0 && day.wall_finish_area > 0.0 && day.ceiling_finish_area > 0.0);
}

#[semio_framework_async_macros::async_test]
async fn the_ceiling_finish_of_a_room_follows_the_authored_hung_ceiling() {
    let model = ASSET.model();
    let inferred = crate::examples::checks::infer(&model);
    let ceiling = |room: &str| inferred.quantities.elements[room].finishes.iter().find(|row| row.surface == crate::standards::v1::subsets::any::schema::inferences::finishes::FinishSurface::Ceiling).expect("a ceiling row").area;
    assert_eq!(inferred.spaces["sp-g4"].ceiling, "ce-g-living");
    assert!(near(ceiling("sp-g4"), inferred.spaces["sp-g4"].net_floor_area), "a flat ceiling under a flat slab covers the net floor once");
    let master = &inferred.spaces["sp-u3"];
    assert_eq!(master.ceiling, "ce-u-master");
    assert!(ceiling("sp-u3") > master.net_floor_area && ceiling("sp-u3") <= master.net_floor_area / 0.04f64.cos() + 1e-9, "a raked ceiling is larger than the floor but never larger than the floor over the cosine of its fall");
    assert_eq!(inferred.spaces["sp-b1"].ceiling, "", "the cellar has no hung ceiling");
}

#[semio_framework_async_macros::async_test]
async fn the_house_hangs_four_ceilings_of_two_types_and_one_of_them_is_raked() {
    let model = ASSET.model();
    assert_eq!((model.ceilings.len(), model.ceiling_types.len()), (4, 2));
    assert_eq!(model.ceilings.values().filter(|ceiling| ceiling.slope.is_some()).map(|ceiling| ceiling.name.as_str()).collect::<Vec<_>>(), ["Master Bedroom Raked Ceiling"]);
    assert!(model.ceilings.values().all(|ceiling| model.storeys.contains_key(&ceiling.storey) && model.ceiling_types.contains_key(&ceiling.ceiling_type)));
    let inferred = crate::examples::checks::infer(&model);
    assert!(model.ceilings.keys().all(|id| inferred.element_solids.contains_key(id) && inferred.quantities.elements.contains_key(id)), "every ceiling has a solid and a take-off");
}
