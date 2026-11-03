"""en1998 v9 step 7: unit tests and oracle for the typed fields. python -I"""
import sys; sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib import *
T = ANY / "🎬️scene/🧪️tests/🔬️unit/🦀️.rs"
sub(T, '''    let template = building.members[0].clone();
    let wall = |id: &str, length: f64, direction: f64, storey: f64| model::En1998Member { id: id.into(), material: "masonry".into(), role: "wall".into(), wall_length_m: length, wall_thickness_m: 0.3, wall_direction_rad: direction, wall_storey_id: storey, ..template.clone() };
    building.members.push(wall("w-x1", 6.0, 0.0, 1.0));
    building.members.push(wall("w-y1", 5.0, std::f64::consts::FRAC_PI_2, 0.0));
    let id = building.id.clone();
    let (scene, _) = scene_of(&document);
    for entity in [format!("{id}:wall:w-x1:1"), format!("{id}:wall:w-y1:1")] {''', '''    let template = building.members[0].clone();
    let first = building.storeys[0].id.clone();
    let wall = |id: &str, length: f64, direction: f64, storey: &str| model::En1998Member { id: id.into(), material: "masonry".into(), role: model::MemberKind::Wall, wall_length_m: length, wall_thickness_m: 0.3, wall_direction_rad: direction, wall_storey_id: storey.into(), ..template.clone() };
    building.members.push(wall("w-x1", 6.0, 0.0, &first));
    building.members.push(wall("w-y1", 5.0, std::f64::consts::FRAC_PI_2, ""));
    let mut column = wall("w-column", 9.0, 0.0, "");
    column.role = model::MemberKind::Column;
    building.members.push(column);
    let id = building.id.clone();
    let (scene, _) = scene_of(&document);
    assert!(!scene.entities.iter().any(|e| e.entity.id.contains(":wall:w-column:")), "only the wall kind makes wall bodies");
    for entity in [format!("{id}:wall:w-x1:{first}"), format!("{id}:wall:w-y1:{first}")] {''')
sub(T, 'a.id() == format!("{id}:wall-density:x:1")', 'a.id() == format!("{id}:wall-density:x:{first}")')
sub(T, "document.site.liquefiable_layer = 3.0;", "document.site.liquefiable_layer = true;")
CT = S / "🧪️tests/⚖️compliance/🦀️.rs"
sub(CT, "d.site.liquefiable_layer = 3.0;", "d.site.liquefiable_layer = true;")
sub(CT, "site.liquefiable_layer = 2.0;", "site.liquefiable_layer = true;")
sub(CT, "site.liquefiable_layer = 0.0;", "site.liquefiable_layer = false;")
sub(CT, '''    let walls = [(6.0, 0.3, 0.0, 1), (4.0, 0.3, 0.0, 1), (5.0, 0.3, std::f64::consts::FRAC_PI_2, 1), (3.0, 0.3, std::f64::consts::FRAC_PI_2, 0), (2.0, 0.25, 0.3, 2)].map(|(length_m, thickness_m, direction_rad, storey)| part_1::MasonryWall { length_m, thickness_m, direction_rad, storey });
    for (key, along_y, storey) in [("mw_x_1", false, 1), ("mw_y_1", true, 1), ("mw_x_2", false, 2), ("mw_y_2", true, 2)] {
        close(key, key, part_1::masonry_wall_density(&walls, 80.0, along_y, storey));
    }
    close("mw governing 1", "mw_governing_1", part_1::masonry_governing_density(&walls, 80.0, 1).expect("walls").0);
    close("mw governing 2", "mw_governing_2", part_1::masonry_governing_density(&walls, 80.0, 2).expect("walls").0);
    assert_eq!(part_1::masonry_governing_density(&walls, 80.0, 2).map(|g| (g.1, g.2)), Some((false, 2)));
    assert_eq!(part_1::masonry_governing_density(&[], 80.0, 2), None);''', '''    let walls = [(6.0, 0.3, 0.0, "s1"), (4.0, 0.3, 0.0, "s1"), (5.0, 0.3, std::f64::consts::FRAC_PI_2, "s1"), (3.0, 0.3, std::f64::consts::FRAC_PI_2, ""), (2.0, 0.25, 0.3, "s2")].map(|(length_m, thickness_m, direction_rad, storey_id)| part_1::MasonryWall { length_m, thickness_m, direction_rad, storey_id: storey_id.into() });
    for (key, along_y, storey) in [("mw_x_1", false, "s1"), ("mw_y_1", true, "s1"), ("mw_x_2", false, "s2"), ("mw_y_2", true, "s2")] {
        close(key, key, part_1::masonry_wall_density(&walls, 80.0, along_y, storey));
    }
    close("mw governing 1", "mw_governing_1", part_1::masonry_governing_density(&walls, 80.0, &["s1"]).expect("walls").0);
    close("mw governing 2", "mw_governing_2", part_1::masonry_governing_density(&walls, 80.0, &["s1", "s2"]).expect("walls").0);
    assert_eq!(part_1::masonry_governing_density(&walls, 80.0, &["s1", "s2"]).map(|g| (g.1, g.2)), Some((false, "s2".to_owned())));
    assert_eq!(part_1::masonry_governing_density(&[], 80.0, &["s1", "s2"]), None);
    let dangling = [part_1::MasonryWall { length_m: 9.0, thickness_m: 0.3, direction_rad: 0.0, storey_id: "missing".into() }];
    assert_eq!(part_1::masonry_governing_density(&dangling, 80.0, &["s1", "s2"]).map(|g| g.0), Some(0.0), "a wall naming no storey row counts in no storey");''')
flush()
