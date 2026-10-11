use super::*;
use crate::standards::v1::subsets::any::io::text::snapshot::{parse_dsl, BIM_EXAMPLE_TEXT};
use crate::standards::v1::subsets::any::schema::inferences::energy_envelope::SurfaceKind;

fn corner(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
}

fn surface(id: &str, boundary: Boundary, u_value: Option<f64>, polygon: Vec<Vec3>) -> EnvelopeSurface {
    EnvelopeSurface { id: id.into(), kind: SurfaceKind::Wall, boundary, u_value, polygon, ..EnvelopeSurface::default() }
}

fn south_wall() -> Vec<Vec3> {
    vec![corner(0.0, 0.0, 0.0), corner(4.0, 0.0, 0.0), corner(4.0, 0.0, 3.0), corner(0.0, 0.0, 3.0)]
}

fn l_floor() -> Vec<Vec3> {
    vec![corner(0.0, 0.0, 0.0), corner(2.0, 0.0, 0.0), corner(2.0, 1.0, 0.0), corner(1.0, 1.0, 0.0), corner(1.0, 2.0, 0.0), corner(0.0, 2.0, 0.0)]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn model_with(spaces: Vec<EnvelopeSpace>) -> (ModelSnapshot, ModelInference) {
    let snapshot = parse_dsl(BIM_EXAMPLE_TEXT).expect("the committed demo parses");
    let mut inference = crate::standards::v1::subsets::any::schema::inferences::model_graph::instance::with_inference(None, &snapshot, Clone::clone);
    inference.energy_envelopes = spaces.into_iter().map(|space| (space.space.clone(), space)).collect();
    (snapshot, inference)
}

#[test]
fn the_scale_runs_blue_to_grey_to_red_and_clamps_at_both_ends() {
    assert_eq!(u_value_rgb(Some(0.0)), [0x3b, 0x4c, 0xc0]);
    assert_eq!(u_value_rgb(Some(1.0)), [0xdd, 0xdd, 0xdd]);
    assert_eq!(u_value_rgb(Some(2.0)), [0xb4, 0x04, 0x26]);
    assert_eq!(u_value_rgb(Some(-3.0)), u_value_rgb(Some(0.0)));
    assert_eq!(u_value_rgb(Some(9.5)), u_value_rgb(Some(2.0)));
}

#[test]
fn a_surface_without_a_u_value_is_neutral_and_no_step_of_the_scale_is() {
    assert_eq!((u_value_rgb(None), u_value_rgb(Some(f64::NAN))), (NEUTRAL, NEUTRAL));
    let ticks = u_value_ticks();
    assert_eq!(ticks.iter().map(|(value, _)| *value).collect::<Vec<_>>(), [0.0, 0.5, 1.0, 1.5, 2.0]);
    assert!(ticks.iter().all(|(_, colour)| *colour != NEUTRAL));
}

#[test]
fn the_scale_is_monotone_from_the_blue_channel_to_the_red_channel() {
    let reds: Vec<u8> = (0..=20).map(|step| u_value_rgb(Some(f64::from(step) / 10.0))[0]).collect();
    let blues: Vec<u8> = (0..=20).map(|step| u_value_rgb(Some(f64::from(step) / 10.0))[2]).collect();
    assert!(reds[0] < reds[10] && reds[10] < reds[20], "{reds:?}");
    assert!(blues[0] > blues[20], "{blues:?}");
}

#[test]
fn the_four_boundary_conditions_have_four_distinct_colours_and_the_modes_round_trip() {
    let steps = boundary_steps();
    let mut colours: Vec<[u8; 3]> = steps.iter().map(|(_, colour)| *colour).collect();
    colours.sort_unstable();
    colours.dedup();
    assert_eq!(colours.len(), 4);
    assert!(!colours.contains(&NEUTRAL));
    for mode in Mode::ALL {
        assert_eq!(Mode::parse(mode.key()), Some(mode));
    }
    assert_eq!((Mode::parse("hatch"), Mode::default()), (None, Mode::UValue));
}

#[test]
fn the_mode_decides_which_fact_colours_a_surface() {
    let exterior = surface("a", Boundary::Exterior, Some(0.2), south_wall());
    assert_eq!(rgb(Mode::UValue, &exterior), u_value_rgb(Some(0.2)));
    assert_eq!(rgb(Mode::Boundary, &exterior), boundary_rgb(Boundary::Exterior));
    let bare = surface("b", Boundary::Ground, None, south_wall());
    assert_eq!((rgb(Mode::UValue, &bare), rgb(Mode::Boundary, &bare)), (NEUTRAL, boundary_rgb(Boundary::Ground)));
}

#[test]
fn a_wall_quad_is_two_triangles_wound_around_its_outward_normal_and_lifted_inwards() {
    let polygon = south_wall();
    let normal = normal_of(&polygon).expect("a normal");
    assert!(normal[0].abs() < 1e-12 && (normal[1] + 1.0).abs() < 1e-12 && normal[2].abs() < 1e-12, "{normal:?}");
    let mesh = mesh_of(&[Part { space: &EnvelopeSpace::default(), surface: &surface("a", Boundary::Exterior, Some(0.3), polygon), placement: SolidPlacement::default() }], Mode::UValue);
    assert_eq!((mesh.positions.len(), mesh.normals.len(), mesh.colors.len(), mesh.indices.len(), mesh.face_ids.len()), (18, 18, 24, 6, 2));
    assert_eq!(mesh.indices, [0, 1, 2, 3, 4, 5]);
    for triangle in mesh.positions.chunks(9) {
        let at = |index: usize| [f64::from(triangle[index * 3]), f64::from(triangle[index * 3 + 1]), f64::from(triangle[index * 3 + 2])];
        let (a, b, c) = (at(0), at(1), at(2));
        let (u, v) = ([b[0] - a[0], b[1] - a[1], b[2] - a[2]], [c[0] - a[0], c[1] - a[1], c[2] - a[2]]);
        let cross = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
        assert!(dot(cross, normal) > 0.0, "{cross:?} against {normal:?}");
        assert!([a, b, c].iter().all(|p| (p[1] - LIFT).abs() < 1e-6), "lifted by {LIFT} towards the inside");
    }
}

#[test]
fn a_concave_floor_triangulates_to_exactly_its_area() {
    let polygon = l_floor();
    let normal = normal_of(&polygon).expect("a normal");
    assert!((normal[2] - 1.0).abs() < 1e-12);
    let triangles = triangles_of(&polygon, normal);
    assert_eq!(triangles.len(), 4);
    let area: f64 = triangles.iter().map(|t| polygon_area(&[polygon[t[0]], polygon[t[1]], polygon[t[2]]])).sum();
    assert!((area - 3.0).abs() < 1e-9 && (polygon_area(&polygon) - 3.0).abs() < 1e-9, "the L is a 2 by 2 square minus a 1 by 1 corner: {area}");
}

#[test]
fn a_downward_facing_polygon_keeps_its_winding_and_a_degenerate_one_is_left_out() {
    let mut ceiling = l_floor();
    ceiling.reverse();
    let normal = normal_of(&ceiling).expect("a normal");
    assert!((normal[2] + 1.0).abs() < 1e-12);
    for t in triangles_of(&ceiling, normal) {
        let (a, b, c) = (ceiling[t[0]], ceiling[t[1]], ceiling[t[2]]);
        let cross_z = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
        assert!(cross_z < 0.0, "counter-clockwise seen from below is clockwise seen from above");
    }
    let flat = vec![corner(0.0, 0.0, 0.0), corner(1.0, 0.0, 0.0), corner(2.0, 0.0, 0.0)];
    assert!(normal_of(&flat).is_none());
    let space = EnvelopeSpace::default();
    let degenerate = surface("d", Boundary::Exterior, None, flat);
    assert!(mesh_of(&[Part { space: &space, surface: &degenerate, placement: SolidPlacement::default() }], Mode::Boundary).indices.is_empty());
}

#[test]
fn the_world_position_rotates_about_z_then_translates() {
    let placement = SolidPlacement { x: 10.0, y: 20.0, z: 1.0, rotation: std::f64::consts::FRAC_PI_2 };
    let at = to_world(placement, [1.0, 0.0, 2.0]);
    assert!((at[0] - 10.0).abs() < 1e-12 && (at[1] - 21.0).abs() < 1e-12 && (at[2] - 3.0).abs() < 1e-12, "{at:?}");
}

#[test]
fn the_centre_is_the_mean_of_the_corners() {
    assert_eq!(centre(&surface("a", Boundary::Exterior, None, south_wall())), [2.0, 0.0, 1.5]);
}

#[test]
fn only_conditioned_spaces_on_shown_storeys_make_parts_and_they_group_by_placement() {
    let wall = surface("sp-a/w1", Boundary::Exterior, Some(0.3), south_wall());
    let room = |id: &str, storey: &str, conditioned: bool| EnvelopeSpace { space: id.into(), storey: storey.into(), conditioned, surfaces: vec![wall.clone()], ..EnvelopeSpace::default() };
    let (snapshot, inference) = model_with(vec![room("sp-a", "st-ground", true), room("sp-b", "st-ground", false), room("sp-c", "st-first", true)]);
    let all = parts(&snapshot, &inference, &|_| true);
    assert_eq!(all.iter().map(|part| part.space.space.as_str()).collect::<Vec<_>>(), ["sp-a", "sp-c"], "an unconditioned space has no overlay");
    let ground = parts(&snapshot, &inference, &|storey| storey == "st-ground");
    assert_eq!(ground.len(), 1);
    assert!(parts(&snapshot, &inference, &|_| false).is_empty());
    let groups = by_placement(&all);
    assert!(!groups.is_empty() && groups.iter().map(|(_, members)| members.len()).sum::<usize>() == all.len());
}

#[test]
fn the_heatmap_values_follow_the_triangles_of_the_mesh() {
    let space = EnvelopeSpace::default();
    let wall = surface("a", Boundary::Exterior, Some(0.3), south_wall());
    let bare = surface("b", Boundary::Exterior, None, l_floor());
    let parts = [Part { space: &space, surface: &wall, placement: SolidPlacement::default() }, Part { space: &space, surface: &bare, placement: SolidPlacement::default() }];
    let mesh = mesh_of(&parts, Mode::UValue);
    let values = u_values(&parts, &mesh);
    assert_eq!(values.len(), 2 + 4);
    assert_eq!(values[..2], [Some(0.3), Some(0.3)]);
    assert!(values[2..].iter().all(Option::is_none));
    assert_eq!(mesh.colors.len(), values.len() * 12);
}
