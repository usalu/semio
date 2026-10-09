use super::*;
use crate::editor::bim::gestures::plane::{axis_ends, signed_area};
use crate::editor::bim::gestures::session::Shape;
use crate::editor::bim::gestures::tests::fixture::{model, room, Rig};
use crate::ModelMutation;

#[semio_framework_async_macros::async_test]
async fn every_form_of_the_grammar_parses_to_its_entry() {
    for (text, entry) in [
        ("", Entry::Empty),
        ("   ", Entry::Empty),
        ("3, 4", Entry::Absolute([3.0, 4.0])),
        ("3;4", Entry::Absolute([3.0, 4.0])),
        ("-3.5 4e-1", Entry::Absolute([-3.5, 0.4])),
        ("@2, -1", Entry::Relative([2.0, -1.0])),
        ("@ 2 -1", Entry::Relative([2.0, -1.0])),
        ("2<90", Entry::Polar { length: 2.0, degrees: 90.0 }),
        ("@2.5<45°", Entry::Polar { length: 2.5, degrees: 45.0 }),
        ("3.2", Entry::Length(3.2)),
        ("@3m", Entry::Length(3.0)),
        ("-1", Entry::Length(-1.0)),
    ] {
        assert_eq!(parse(text), Some(entry), "{text:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn text_that_names_no_point_does_not_parse() {
    for text in ["a, b", "1, 2, 3", "1,", "<90", "2<", "NaN", "inf, 1", "x"] {
        assert_eq!(parse(text), None, "{text:?}");
    }
    assert_eq!(point_of("1 2 3"), None);
    assert_eq!(point_of("10 ; 5"), Some([10.0, 5.0]));
}

#[semio_framework_async_macros::async_test]
async fn an_entry_resolves_against_the_anchor_and_the_last_pointer_direction() {
    let anchor = Some([1.0, 1.0]);
    assert_eq!(resolve(&Entry::Absolute([3.0, 4.0]), anchor, None), Some([3.0, 4.0]));
    assert_eq!(resolve(&Entry::Relative([2.0, -1.0]), anchor, None), Some([3.0, 0.0]));
    assert_eq!(resolve(&Entry::Relative([2.0, -1.0]), None, None), Some([2.0, -1.0]), "the origin anchors the first point");
    assert_eq!(resolve(&Entry::Polar { length: 2.0, degrees: 90.0 }, anchor, None), Some([1.0, 3.0]), "a quarter turn is exact");
    assert_eq!(resolve(&Entry::Polar { length: 2.0, degrees: 180.0 }, anchor, None), Some([-1.0, 1.0]));
    assert_eq!(resolve(&Entry::Length(3.0), anchor, Some([1.0, 9.0])), Some([1.0, 4.0]), "a bare length goes towards the pointer");
    assert_eq!(resolve(&Entry::Length(3.0), anchor, None), Some([4.0, 1.0]), "and towards +X when the pointer never was");
    assert_eq!(resolve(&Entry::Length(3.0), anchor, anchor), Some([4.0, 1.0]), "or when the pointer sits on the anchor");
    assert_eq!(resolve(&Entry::Empty, anchor, None), None);
}

#[semio_framework_async_macros::async_test]
async fn a_forty_five_degree_polar_offset_is_the_diagonal_of_its_length() {
    let [x, y] = resolve(&Entry::Polar { length: 2.0_f64.sqrt() * 3.0, degrees: 45.0 }, Some([0.0, 0.0]), None).expect("resolves");
    assert!((x - 3.0).abs() < 1e-8 && (y - 3.0).abs() < 1e-8);
}

fn walls_of(rig: &Rig) -> Vec<([f64; 2], [f64; 2])> {
    rig.snapshot.walls.values().map(|wall| axis_ends(&wall.axis)).collect()
}

#[semio_framework_async_macros::async_test]
async fn a_wall_chain_is_typed_from_the_keyboard_alone_and_a_blank_line_ends_it() {
    let mut rig = Rig::plan("wall", model());
    assert!(rig.typed("0, 0").mutations.is_empty(), "the first point only anchors the chain");
    let first = rig.typed("@4<0");
    assert!(matches!(first.mutations.as_slice(), [ModelMutation::CreateWall(_)]), "the polar offset writes the first wall");
    rig.typed("@0, 3");
    assert!(rig.typed("").mutations.is_empty(), "a blank line finishes the chain without writing");
    assert_eq!(walls_of(&rig), vec![([0.0, 0.0], [4.0, 0.0]), ([4.0, 0.0], [4.0, 3.0])]);
    rig.typed("1, 1");
    assert_eq!(walls_of(&rig).len(), 2, "after the finish the next point starts a new chain");
}

#[semio_framework_async_macros::async_test]
async fn a_bare_length_goes_towards_where_the_pointer_last_was() {
    let mut rig = Rig::plan("wall", model());
    rig.typed("0, 0");
    rig.mv(0.0, 5.0);
    rig.typed("3");
    assert_eq!(walls_of(&rig), vec![([0.0, 0.0], [0.0, 3.0])]);
}

#[semio_framework_async_macros::async_test]
async fn the_arc_wall_takes_its_start_end_and_a_point_on_the_arc() {
    let mut rig = Rig::plan("wall-arc", model());
    for line in ["0, 0", "4, 0"] {
        assert!(rig.typed(line).mutations.is_empty());
    }
    let step = rig.typed("2, -1");
    let [ModelMutation::CreateWall(created)] = step.mutations.as_slice() else { panic!("one arc wall, got {:?}", step.mutations) };
    assert!(matches!(created.wall.axis, crate::Axis::Arc { bulge, .. } if (bulge - 0.5).abs() < 1e-9), "a 4 m chord with a 1 m sagitta bulges by 0.5");
}

#[semio_framework_async_macros::async_test]
async fn the_two_point_tools_write_their_element_from_two_typed_points() {
    for utility in ["curtain-wall", "beam", "grid"] {
        let mut rig = Rig::plan(utility, model());
        rig.typed("2, 0");
        let step = rig.typed("@0, 8");
        assert_eq!(step.mutations.len(), 1, "{utility}");
        let count = match utility {
            "curtain-wall" => rig.snapshot.curtain_walls.len(),
            "beam" => rig.snapshot.beams.len(),
            _ => rig.snapshot.grids.len(),
        };
        assert_eq!(count, 1, "{utility} wrote its element");
    }
}

#[semio_framework_async_macros::async_test]
async fn a_railing_and_the_area_tools_close_with_a_blank_line() {
    let mut railing = Rig::plan("railing", model());
    for line in ["0, 0", "@2, 0", "@0, 2"] {
        railing.typed(line);
    }
    assert!(railing.snapshot.railings.is_empty(), "the railing is written whole when it finishes");
    railing.typed("");
    assert_eq!(railing.snapshot.railings.values().next().map(|row| row.path.len()), Some(3));
    for utility in ["slab", "roof"] {
        let mut rig = Rig::plan(utility, model());
        for line in ["0, 0", "4, 0", "4, 3", "0, 3"] {
            assert!(rig.typed(line).mutations.is_empty(), "{utility}: the corners only collect");
        }
        let step = rig.typed("");
        assert_eq!(step.mutations.len(), 1, "{utility}");
        let ring: Vec<[f64; 2]> = match utility {
            "slab" => rig.snapshot.slabs.values().next().map(|row| row.boundary.iter().map(|vertex| [vertex.point.x, vertex.point.y]).collect()),
            _ => rig.snapshot.roofs.values().next().map(|row| row.footprint.iter().map(|vertex| [vertex.point.x, vertex.point.y]).collect()),
        }
        .expect("the element");
        assert!((signed_area(&ring) - 12.0).abs() < 1e-9, "{utility}: a 4 by 3 outline written counter-clockwise");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_slab_from_walls_takes_a_typed_point_inside_the_room() {
    let mut rig = Rig::plan("slab-walls", room());
    let step = rig.typed("4, 3");
    assert!(matches!(step.mutations.as_slice(), [ModelMutation::CreateSlab(_)]));
}

#[semio_framework_async_macros::async_test]
async fn the_point_tools_place_at_the_exact_typed_point() {
    let mut column = Rig::plan("column", model());
    column.typed("2.5, 1.25");
    assert_eq!(column.snapshot.columns.values().next().map(|row| (row.position.x, row.position.y)), Some((2.5, 1.25)));
    let mut space = Rig::plan("space", room());
    space.typed("2, 2");
    assert_eq!(space.snapshot.spaces.len(), 1);
    let mut stair = Rig::plan("stair", model());
    assert!(stair.typed("1, 1").mutations.is_empty(), "the foot only anchors the stair");
    stair.typed("@3<90");
    let row = stair.snapshot.stairs.values().next().expect("the stair");
    assert_eq!((row.start.x, row.start.y), (1.0, 1.0));
    assert!((row.direction - std::f64::consts::FRAC_PI_2).abs() < 1e-9, "the polar angle is the run direction");
}

#[semio_framework_async_macros::async_test]
async fn the_opening_tools_place_on_the_wall_under_the_typed_point() {
    for utility in ["window", "door", "opening"] {
        let mut rig = Rig::plan(utility, room());
        let step = rig.typed("4, 0");
        assert!(matches!(step.mutations.as_slice(), [ModelMutation::CreateOpening(_)]), "{utility}: {:?}", step.refused);
        assert_eq!(rig.snapshot.openings.values().next().map(|row| row.offset), Some(4.0), "{utility} stands at the typed point");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_split_tool_cuts_the_wall_where_the_line_names() {
    let mut rig = Rig::plan("split-wall", room());
    let step = rig.typed("2, 0");
    assert!(matches!(step.mutations.as_slice(), [ModelMutation::SplitWall(split)] if (split.t - 0.25).abs() < 1e-9));
}

#[semio_framework_async_macros::async_test]
async fn the_transforms_take_their_points_from_the_keyboard() {
    let mut mover = Rig::plan("move", room());
    mover.selected = vec!["w-south".into()];
    assert!(mover.typed("0, 0").mutations.is_empty(), "the base point only anchors the move");
    let step = mover.typed("@1, 2");
    assert!(matches!(step.mutations.as_slice(), [ModelMutation::MoveElements(moved)] if moved.vector.x == 1.0 && moved.vector.y == 2.0), "{:?}", step.mutations);
    let mut turner = Rig::plan("rotate", room());
    turner.selected = vec!["w-south".into()];
    for line in ["0, 0", "1<0"] {
        assert!(turner.typed(line).mutations.is_empty());
    }
    let step = turner.typed("1<90");
    assert!(matches!(step.mutations.as_slice(), [ModelMutation::RotateElements(turned)] if (turned.angle - std::f64::consts::FRAC_PI_2).abs() < 1e-9), "{:?}", step.mutations);
}

#[semio_framework_async_macros::async_test]
async fn the_select_tool_picks_and_the_measure_tool_shows_its_length_without_writing() {
    let mut select = Rig::plan("select", room());
    let picked = select.typed("4, 0");
    assert_eq!(picked.pick.as_ref().map(|pick| pick.targets.clone()), Some(vec![("wall".to_string(), "w-south".to_string())]));
    let mut measure = Rig::plan("measure", model());
    let before = measure.snapshot.clone();
    measure.typed("0, 0");
    assert!(measure.typed("3, 4").mutations.is_empty());
    assert_eq!(measure.snapshot, before, "measuring never writes");
    assert!(measure.preview.marks.iter().any(|mark| mark.shape == Shape::Label && mark.text == "5.00 m"));
}

#[semio_framework_async_macros::async_test]
async fn a_line_that_names_no_point_is_refused_and_leaves_the_gesture_alone() {
    let mut rig = Rig::plan("wall", model());
    rig.typed("0, 0");
    let refused = rig.typed("not a point");
    assert_eq!(refused.refused, Some(INVALID));
    assert!(refused.mutations.is_empty());
    let step = rig.typed("@2<0");
    assert_eq!(step.mutations.len(), 1, "the chain is still anchored at the first point");
}

#[semio_framework_async_macros::async_test]
async fn a_typed_point_on_the_world_and_the_section_surface_is_a_click_there_too() {
    let mut world = Rig::on("column", model(), crate::editor::bim::gestures::session::Surface::World { storey: "st-ground".into() });
    world.typed("1, 2");
    assert_eq!(world.snapshot.columns.len(), 1);
    let mut section = Rig::on("select", room(), crate::editor::bim::gestures::session::Surface::Section { start: [0.0, 0.0], end: [8.0, 0.0] });
    let step = section.typed("4, 3");
    assert!(step.refused.is_none(), "a section takes distance along the line and height");
}

const TYPED_CASES: &str = include_str!("../../../../../🧫️fixtures/🛠️gestures/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn the_lines_the_third_party_oracle_resolved_resolve_to_the_same_points() {
    let cases: serde_json::Value = serde_json::from_str(TYPED_CASES).expect("the committed cases");
    let point = |value: &serde_json::Value| value.as_array().map(|pair| [pair[0].as_f64().expect("x"), pair[1].as_f64().expect("y")]);
    let typed = cases["typed"].as_array().expect("the typed cases");
    assert!(typed.len() >= 16);
    for case in typed {
        let entry = parse(case["entry"].as_str().expect("the typed line")).unwrap_or_else(|| panic!("{case} parses"));
        let found = resolve(&entry, point(&case["anchor"]), point(&case["toward"])).unwrap_or_else(|| panic!("{case} resolves"));
        let expected = point(&case["point"]).expect("the oracle's point");
        assert!((found[0] - expected[0]).abs() < 1e-8 && (found[1] - expected[1]).abs() < 1e-8, "{case}: the subject found {found:?}");
    }
}

fn step_delta(name: &str) -> [f64; 2] {
    use crate::editor::bim::commands::cursor_keys::*;
    match name {
        "left" => CursorLeft::DELTA,
        "right" => CursorRight::DELTA,
        "up" => CursorUp::DELTA,
        "down" => CursorDown::DELTA,
        "left_far" => CursorLeftFar::DELTA,
        "right_far" => CursorRightFar::DELTA,
        "up_far" => CursorUpFar::DELTA,
        "down_far" => CursorDownFar::DELTA,
        other => panic!("no arrow key {other}"),
    }
    .expect("an arrow key steps the cursor")
}

#[semio_framework_async_macros::async_test]
async fn the_arrow_keys_land_the_cursor_on_the_points_the_third_party_oracle_summed() {
    let cases: serde_json::Value = serde_json::from_str(TYPED_CASES).expect("the committed cases");
    let point = |value: &serde_json::Value| [value[0].as_f64().expect("x"), value[1].as_f64().expect("y")];
    let walk = cases["cursor"].as_array().expect("the cursor cases");
    assert!(walk.len() >= 3);
    for case in walk {
        let start = point(&case["start"]);
        let mut rig = Rig::plan("select", model());
        for step in case["steps"].as_array().expect("the steps") {
            rig.nudge(step_delta(step.as_str().expect("a key")), start);
        }
        let (found, expected) = (rig.session.cursor(start), point(&case["point"]));
        assert!((found[0] - expected[0]).abs() < 1e-8 && (found[1] - expected[1]).abs() < 1e-8, "{case}: the cursor stands on {found:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn a_wall_is_drawn_with_the_cursor_alone_and_a_blank_cursor_starts_at_the_given_point() {
    let mut rig = Rig::plan("wall", model());
    let start = [2.0, 2.0];
    assert_eq!(rig.session.cursor(start), start, "a cursor that never moved stands on the start");
    rig.nudge(step_delta("right_far"), start);
    assert!(rig.place(start).mutations.is_empty(), "the first click only anchors the chain");
    for _ in 0..2 {
        rig.nudge(step_delta("right_far"), start);
    }
    let written = rig.place(start);
    assert!(matches!(written.mutations.as_slice(), [ModelMutation::CreateWall(_)]), "the second click writes the wall");
    assert_eq!(walls_of(&rig), vec![([3.0, 2.0], [5.0, 2.0])]);
}

#[semio_framework_async_macros::async_test]
async fn moving_the_cursor_shows_the_rubber_band_and_writes_nothing() {
    let mut rig = Rig::plan("wall", model());
    rig.typed("0, 0");
    let moved = rig.nudge(step_delta("right_far"), [0.0, 0.0]);
    assert!(moved.mutations.is_empty() && walls_of(&rig).is_empty());
    assert!(!rig.preview.to_text().is_empty(), "the tool shows its marks at the cursor");
}
