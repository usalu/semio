use super::kit::{house, items, plain, table_of, texts};
use super::*;
use crate::{ModelInference, Phase, PropertyValue, ScheduleCategory, ScheduleColumn, ScheduleField};
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn an_opening_stands_on_the_storey_and_in_the_phase_of_its_host() {
    let snapshot = house();
    assert_eq!(rows::storey_of(&snapshot, "o-door-1").map(String::as_str), Some("st-ground"));
    assert_eq!(rows::storey_of(&snapshot, "o-door-2").map(String::as_str), Some("st-first"));
    assert_eq!(rows::phase_of(&snapshot, "o-door-2"), Phase::Demolished, "the door sits in a demolished wall");
    assert_eq!(rows::phase_of(&snapshot, "w-free"), Phase::Existing);
    assert_eq!(rows::phase_of(&snapshot, "no-such-element"), Phase::New, "an element without a phase counts as new");
    assert_eq!(rows::storey_of(&snapshot, "no-such-element"), None);
}

#[semio_framework_async_macros::async_test]
async fn the_scope_of_a_door_schedule_follows_the_storey_and_the_phase_of_the_hosts() {
    let snapshot = house();
    let mut schedule = plain(ScheduleCategory::Door, &[(ScheduleField::Name, false)]);
    assert_eq!(rows::candidates(&snapshot, &schedule), ["o-door-1", "o-door-2"]);
    schedule.storeys = vec!["st-first".into()];
    assert_eq!(rows::candidates(&snapshot, &schedule), ["o-door-2"]);
    schedule.storeys.clear();
    schedule.phases = vec![Phase::New];
    assert_eq!(rows::candidates(&snapshot, &schedule), ["o-door-1"]);
    schedule.phases = vec![Phase::Existing];
    assert!(rows::candidates(&snapshot, &schedule).is_empty());
    schedule.storeys = vec!["st-ground".into()];
    schedule.phases = vec![Phase::Demolished];
    assert!(rows::candidates(&snapshot, &schedule).is_empty(), "both scopes must hold");
    schedule.phases = vec![Phase::New, Phase::Demolished];
    schedule.storeys = vec!["st-ground".into(), "st-first".into()];
    assert_eq!(rows::candidates(&snapshot, &schedule), ["o-door-1", "o-door-2"]);
}

#[semio_framework_async_macros::async_test]
async fn every_category_lists_exactly_the_elements_of_its_kind() {
    let snapshot = house();
    let count = |category| rows::candidates(&snapshot, &plain(category, &[(ScheduleField::Id, false)])).len();
    assert_eq!(count(ScheduleCategory::Wall), snapshot.walls.len());
    assert_eq!(count(ScheduleCategory::CurtainWall), snapshot.curtain_walls.len());
    assert_eq!(count(ScheduleCategory::Slab), snapshot.slabs.len());
    assert_eq!(count(ScheduleCategory::Roof), snapshot.roofs.len());
    assert_eq!(count(ScheduleCategory::Column), snapshot.columns.len());
    assert_eq!(count(ScheduleCategory::Beam), snapshot.beams.len());
    assert_eq!(count(ScheduleCategory::Stair), snapshot.stairs.len());
    assert_eq!(count(ScheduleCategory::Railing), snapshot.railings.len());
    assert_eq!(count(ScheduleCategory::Space), snapshot.spaces.len());
    assert_eq!(count(ScheduleCategory::Finish), snapshot.spaces.len());
    assert_eq!((count(ScheduleCategory::Window), count(ScheduleCategory::Door), count(ScheduleCategory::Void)), (3, 2, 1));
    let everything = snapshot.walls.len() + snapshot.curtain_walls.len() + snapshot.slabs.len() + snapshot.roofs.len() + snapshot.columns.len() + snapshot.beams.len() + snapshot.openings.len() + snapshot.stairs.len() + snapshot.railings.len() + snapshot.spaces.len();
    assert_eq!(count(ScheduleCategory::Material), everything, "a material take-off may draw on any element");
}

#[semio_framework_async_macros::async_test]
async fn the_facts_of_a_door_a_window_and_a_room_are_read_from_the_authored_records() {
    let snapshot = house();
    let door = Facts::of(&snapshot, "o-door-1", &[]);
    assert_eq!((door.name.display().as_str(), door.type_name.display().as_str(), door.storey.display().as_str(), door.phase.display().as_str(), door.host.display().as_str()), ("Door 1", "Door 90", "Ground", "new", "South"));
    assert_eq!((door.swing.display().as_str(), door.leaves.display().as_str(), door.level), ("right", "single", ScheduleCell::number(0.0)), "a left door with a flipped hand opens to the right");
    let double = Facts::of(&snapshot, "o-door-2", &[]);
    assert_eq!((double.swing.display().as_str(), double.leaves.display().as_str(), double.phase.display().as_str()), ("right", "double", "demolished"));
    assert_eq!(Facts::of(&snapshot, "o-win-1", &[]).panes, ScheduleCell::number(2.0));
    assert_eq!(Facts::of(&snapshot, "o-win-1", &[]).swing, ScheduleCell::Empty, "a window has no swing");
    let room = Facts::of(&snapshot, "sp-1", &[]);
    assert_eq!((room.number.display().as_str(), room.usage.display().as_str(), room.name.display().as_str()), ("0.01", "living", "Living room"));
    assert_eq!(Facts::of(&snapshot, "no-such-element", &[]).name, ScheduleCell::Empty);
}

#[semio_framework_async_macros::async_test]
async fn a_property_cell_reads_the_typed_value_and_is_empty_where_the_element_has_none() {
    let snapshot = house();
    let keys: Vec<(String, String)> = [("Pset_WallCommon", "FireRating"), ("Pset_WallCommon", "IsExternal"), ("Pset_WallCommon", "ThermalTransmittance"), ("Custom", "Span"), ("Custom", "Count")].iter().map(|(set, name)| (set.to_string(), name.to_string())).collect();
    let wall = Facts::of(&snapshot, "w-south", &keys);
    assert_eq!(wall.properties, [ScheduleCell::text("EI60"), ScheduleCell::text("true"), ScheduleCell::number(0.25), ScheduleCell::Empty, ScheduleCell::Empty]);
    let column = Facts::of(&snapshot, "c-1", &keys);
    assert_eq!(column.properties, [ScheduleCell::Empty, ScheduleCell::Empty, ScheduleCell::Empty, ScheduleCell::number(3.0), ScheduleCell::number(4.0)], "a length and an integer are numbers");
    assert_eq!(Facts::of(&snapshot, "w-east", &keys).properties, vec![ScheduleCell::Empty; 5]);
    assert!(matches!(snapshot.properties["w-south"]["Pset_WallCommon"]["FireRating"], PropertyValue::Text { .. }));
}

#[semio_framework_async_macros::async_test]
async fn the_property_keys_of_a_schedule_are_collected_once_in_order_of_appearance() {
    let mut schedule = plain(ScheduleCategory::Wall, &[(ScheduleField::Name, false)]);
    let key = |set: &str, name: &str| ScheduleKey::property(set, name);
    schedule.columns.push(ScheduleColumn { key: key("B", "two"), heading: None, total: false });
    schedule.sort.push(crate::ScheduleSort { key: key("A", "one"), descending: false });
    schedule.filter.push(crate::ScheduleFilter { key: key("B", "two"), op: crate::ScheduleOp::NotEmpty, value: String::new() });
    schedule.group.push(crate::ScheduleGroup { key: key("C", "three") });
    assert_eq!(rows::property_keys(&schedule), [("B".to_string(), "two".to_string()), ("A".to_string(), "one".to_string()), ("C".to_string(), "three".to_string())]);
}

#[semio_framework_async_macros::async_test]
async fn a_wall_shows_the_names_of_its_materials_joined_without_repeats() {
    let table = table_of(&house(), plain(ScheduleCategory::Wall, &[(ScheduleField::Id, false), (ScheduleField::Material, false)]));
    let listed: std::collections::BTreeMap<String, String> = items(&table).iter().map(|row| (row.cells[0].display(), row.cells[1].display())).collect();
    assert_eq!(listed["w-south"], "Brick, Mineral Wool", "the layers of the type in order");
    assert_eq!(listed["w-free"], "Concrete");
}

#[semio_framework_async_macros::async_test]
async fn the_material_category_lists_one_row_per_layer_with_material_names_and_layer_measures() {
    let snapshot = house();
    let inferred = ModelInference::infer(&snapshot).expect("infers");
    let table = table_of(&snapshot, plain(ScheduleCategory::Material, &[(ScheduleField::Id, false), (ScheduleField::Material, false), (ScheduleField::Thickness, false), (ScheduleField::LayerVolume, true)]));
    let candidates = rows::candidates(&snapshot, &plain(ScheduleCategory::Material, &[(ScheduleField::Id, false)]));
    let layers: usize = candidates.iter().filter_map(|id| inferred.quantities.elements.get(id)).map(|quantity| quantity.layers.iter().filter(|layer| !layer.material.is_empty()).count()).sum();
    assert!(layers > 0);
    assert_eq!(table.items as usize, layers, "one source row per layer with a material");
    assert!(texts(&table, 1).iter().all(|name| !name.is_empty() && !name.starts_with("m-")), "materials show their names, not their ids");
    let wall: Vec<(String, String)> = items(&table).iter().filter(|row| row.cells[0].display() == "w-south").map(|row| (row.cells[1].display(), row.cells[2].display())).collect();
    assert_eq!(wall, [("Brick".to_string(), "0.2".to_string()), ("Mineral Wool".to_string(), "0.1".to_string())]);
    let volume: f64 = candidates.iter().filter_map(|id| inferred.quantities.elements.get(id)).flat_map(|quantity| quantity.layers.iter().filter(|layer| !layer.material.is_empty()).map(|layer| layer.volume)).sum();
    let ScheduleCell::Number { value: total } = table.rows.last().expect("rows").cells[3] else { panic!("a summed layer volume") };
    assert!((total - volume).abs() < 1e-9, "the total is the sum of the layer volumes of the quantities");
}

#[semio_framework_async_macros::async_test]
async fn the_finish_category_lists_the_floor_wall_and_ceiling_of_every_resolved_room() {
    let mut snapshot = house();
    let room = snapshot.spaces.get_mut("sp-1").expect("the living room");
    room.floor_finish = Some("m-wood".into());
    room.wall_finish = Some("m-brick".into());
    let inferred = ModelInference::infer(&snapshot).expect("infers");
    let resolved: Vec<&String> = snapshot.spaces.keys().filter(|id| inferred.quantities.elements.get(*id).is_some_and(|quantity| !quantity.finishes.is_empty())).collect();
    assert!(!resolved.is_empty());
    let table = table_of(&snapshot, plain(ScheduleCategory::Finish, &[(ScheduleField::Number, false), (ScheduleField::Surface, false), (ScheduleField::Material, false), (ScheduleField::FinishArea, true)]));
    assert_eq!(table.items as usize, 3 * resolved.len());
    let living: Vec<(String, String, f64)> = items(&table).iter().filter(|row| row.cells[0].display() == "0.01").map(|row| (row.cells[1].display(), row.cells[2].display(), if let ScheduleCell::Number { value } = row.cells[3] { value } else { f64::NAN })).collect();
    let quantity = &inferred.quantities.elements["sp-1"];
    for surface in ["floor", "wall", "ceiling"] {
        let found = living.iter().find(|(shown, _, _)| shown == surface).unwrap_or_else(|| panic!("a {surface} row"));
        let expected = quantity.finishes.iter().find(|finish| finish.surface.key() == surface).expect("the finish row of the quantity");
        assert!((found.2 - expected.area).abs() < 1e-12, "the area of the {surface} is the one of the quantity");
    }
    let materials: Vec<&str> = ["floor", "wall", "ceiling"].iter().map(|surface| living.iter().find(|(shown, _, _)| shown == surface).map(|(_, material, _)| material.as_str()).unwrap_or("?")).collect();
    assert_eq!(materials, ["Oak", "Brick", ""], "a finish shows the name of its material, an unfinished surface keeps its area with no material");
    let sum: f64 = items(&table).iter().map(|row| if let ScheduleCell::Number { value } = row.cells[3] { value } else { 0.0 }).sum();
    let ScheduleCell::Number { value: total } = table.rows.last().expect("rows").cells[3] else { panic!("a summed area") };
    assert!((total - sum).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn a_room_that_is_not_enclosed_has_no_finish_rows() {
    let mut snapshot = house();
    snapshot.walls.clear();
    snapshot.openings.clear();
    snapshot.spaces.remove("sp-2");
    let table = table_of(&snapshot, plain(ScheduleCategory::Finish, &[(ScheduleField::Surface, false)]));
    assert_eq!(table.items, 0, "without walls the bounded living room resolves to nothing");
    assert!(table.rows.is_empty());
}
