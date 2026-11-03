use super::extrude::{member_mesh, straight_mesh};
use super::fasteners::{fastener_mesh, offsets};
use super::glyphs::{load_mesh, support_mesh};
use super::section_outline::{composite_floor, deck_slab, net_area, plate_girder, properties, rc_rectangle, regions};
use super::shells::{cylindrical_shell, plate_mesh, shell_mesh, silo_shell};
use super::solids::{pile, solid_mesh, wall_with_openings, Opening};
use super::triangulate::{area2, triangulate};
use super::*;
use crate::*;
use std::f64::consts::PI;

const TOL: Tolerance = Tolerance { chord: 5e-4 };

fn param(profile: ProfileKind, params: &[(&str, f64)]) -> SectionShape {
    SectionShape::parametric(profile, params)
}

fn rel(a: f64, b: f64) -> f64 {
    (a - b).abs() / a.abs().max(b.abs()).max(1e-300)
}

fn catalogue() -> Vec<(&'static str, ProfileKind, Vec<(&'static str, f64)>, f64)> {
    let (h, b, tw, tf, t, c) = (0.24, 0.24, 0.01, 0.017, 0.008, 0.02);
    vec![
        ("ishape", ProfileKind::IShape, vec![("h", h), ("b", b), ("tw", tw), ("tf", tf)], 2.0 * b * tf + (h - 2.0 * tf) * tw),
        ("rect", ProfileKind::Rect, vec![("h", 0.5), ("b", 0.3)], 0.15),
        ("rhs", ProfileKind::Rhs, vec![("h", 0.2), ("b", 0.1), ("t", 0.01)], 0.2 * 0.1 - 0.18 * 0.08),
        ("channel", ProfileKind::Channel, vec![("h", 0.2), ("b", 0.075), ("tw", 0.0085), ("tf", 0.0115)], 0.2 * 0.0085 + 2.0 * (0.075 - 0.0085) * 0.0115),
        ("tee", ProfileKind::Tee, vec![("h", 0.15), ("b", 0.15), ("tw", 0.01), ("tf", 0.012)], 0.15 * 0.012 + 0.01 * (0.15 - 0.012)),
        ("angle", ProfileKind::Angle, vec![("h", 0.1), ("b", 0.1), ("t", 0.01)], 0.1 * 0.01 + 0.01 * 0.09),
        ("cee", ProfileKind::Cee, vec![("h", 0.2), ("b", 0.075), ("t", t), ("c", c)], t * (0.2 + 2.0 * (0.075 - t) + 2.0 * (c - t))),
        ("zed", ProfileKind::Zed, vec![("h", 0.2), ("b", 0.075), ("t", t), ("c", c)], t * (0.2 + 2.0 * (0.075 - t) + 2.0 * (c - t))),
    ]
}

#[test]
fn triangulation_covers_polygon_with_holes() {
    let outer = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    let hole = vec![[0.25, 0.25], [0.75, 0.25], [0.75, 0.75], [0.25, 0.75]];
    let (pts, tris) = triangulate(&outer, &[hole]);
    let area: f64 = tris.iter().map(|t| area2(&[pts[t[0]], pts[t[1]], pts[t[2]]]) / 2.0).sum();
    assert!((area - 0.75).abs() < 1e-12);
    assert_eq!(tris.len(), 8);
}

#[test]
fn triangulation_handles_concave_and_two_holes() {
    let outer = vec![[0.0, 0.0], [4.0, 0.0], [4.0, 1.0], [1.0, 1.0], [1.0, 3.0], [4.0, 3.0], [4.0, 4.0], [0.0, 4.0]];
    let sq = |x: f64, y: f64| vec![[x, y], [x + 0.2, y], [x + 0.2, y + 0.2], [x, y + 0.2]];
    let (pts, tris) = triangulate(&outer, &[sq(0.4, 1.4), sq(0.4, 2.2)]);
    let area: f64 = tris.iter().map(|t| area2(&[pts[t[0]], pts[t[1]], pts[t[2]]]) / 2.0).sum();
    assert!((area - (area2(&outer).abs() / 2.0 - 0.08)).abs() < 1e-12);
}

#[test]
fn catalogue_areas_match_closed_forms_and_extrusions_are_watertight() {
    for (name, profile, params, area) in catalogue() {
        let section = param(profile, &params);
        let props = properties(&regions(&section, TOL, Lod::Lod2).unwrap());
        assert!(rel(props.area, area) < 1e-9, "{name}: {} vs {area}", props.area);
        let mesh = straight_mesh(&section, 3.0, TOL, Lod::Lod2).unwrap();
        assert!(mesh.is_watertight(), "{name} watertight");
        assert!(rel(mesh.volume(), area * 3.0) < 2e-5, "{name}: {} vs {}", mesh.volume(), area * 3.0);
    }
}

#[test]
fn circular_sections_follow_chord_tolerance_and_level_caps() {
    let chs = param(ProfileKind::Chs, &[("d", 0.2), ("t", 0.01)]);
    let exact = PI / 4.0 * (0.2f64.powi(2) - 0.18f64.powi(2));
    let counts: Vec<usize> = [Lod::Lod1, Lod::Lod2, Lod::Lod3].iter().map(|l| straight_mesh(&chs, 1.0, Tolerance::new(5e-5), *l).unwrap().triangle_count()).collect();
    assert!(counts[0] < counts[1] && counts[1] < counts[2]);
    let fine = straight_mesh(&chs, 1.0, Tolerance::new(1e-6), Lod::Lod3).unwrap();
    assert!(rel(fine.volume(), exact) < 2e-3 && fine.is_watertight());
    let n = segments(0.1, TAU, TOL, Lod::Lod3);
    assert_eq!(n, 32.max(((PI / (1.0 - TOL.chord / 0.1).acos()).ceil() as usize).min(96)));
}

const TAU: f64 = std::f64::consts::TAU;

#[test]
fn section_properties_match_textbook_rectangle_and_ibeam() {
    let rect = properties(&regions(&param(ProfileKind::Rect, &[("h", 0.5), ("b", 0.3)]), TOL, Lod::Lod2).unwrap());
    assert!(rel(rect.ixx, 0.3 * 0.5f64.powi(3) / 12.0) < 1e-12 && rel(rect.iyy, 0.5 * 0.3f64.powi(3) / 12.0) < 1e-12);
    let (h, b, tw, tf) = (0.24, 0.24, 0.01, 0.017);
    let i = properties(&regions(&param(ProfileKind::IShape, &[("h", h), ("b", b), ("tw", tw), ("tf", tf)]), TOL, Lod::Lod2).unwrap());
    let want = (b * h.powi(3) - (b - tw) * (h - 2.0 * tf).powi(3)) / 12.0;
    assert!(rel(i.ixx, want) < 1e-10 && i.cu.abs() < 1e-12 && i.cv.abs() < 1e-12);
}

#[test]
fn root_fillets_add_the_closed_form_area() {
    let base = [("h", 0.24), ("b", 0.24), ("tw", 0.01), ("tf", 0.017)];
    let mut with_r = base.to_vec();
    with_r.push(("r", 0.021));
    let fine = Tolerance::new(1e-5);
    let a0 = net_area(&regions(&param(ProfileKind::IShape, &base), fine, Lod::Lod3).unwrap()[0]);
    let a1 = net_area(&regions(&param(ProfileKind::IShape, &with_r), fine, Lod::Lod3).unwrap()[0]);
    assert!(rel(a1 - a0, (4.0 - PI) * 0.021f64.powi(2)) < 5e-3, "{}", a1 - a0);
    let sharp = net_area(&regions(&param(ProfileKind::IShape, &with_r), TOL, Lod::Lod1).unwrap()[0]);
    assert!(rel(sharp, a0) < 1e-12);
}

#[test]
fn welded_girder_and_polygon_with_holes_extrude_exactly() {
    let girder = plate_girder(1.0, 0.012, 0.3, 0.02, 0.4, 0.025);
    let area = 0.3 * 0.02 + 0.4 * 0.025 + 1.0 * 0.012;
    assert!(rel(girder.area().unwrap(), area) < 1e-12);
    let mesh = straight_mesh(&girder, 5.0, TOL, Lod::Lod2).unwrap();
    assert!(mesh.is_watertight() && rel(mesh.volume(), area * 5.0) < 2e-5);
    let alu = SectionShape::Polygon { polygon: Polygon2 {
        outer: Ring2 { points: vec![Vec2::new(0.0, 0.0), Vec2::new(0.1, 0.0), Vec2::new(0.1, 0.1), Vec2::new(0.0, 0.1)] },
        holes: vec![Ring2 { points: vec![Vec2::new(0.02, 0.02), Vec2::new(0.04, 0.02), Vec2::new(0.04, 0.04), Vec2::new(0.02, 0.04)] }, Ring2 { points: vec![Vec2::new(0.06, 0.06), Vec2::new(0.08, 0.06), Vec2::new(0.08, 0.08), Vec2::new(0.06, 0.08)] }],
    } };
    let m = straight_mesh(&alu, 2.0, TOL, Lod::Lod2).unwrap();
    assert!(m.is_watertight() && rel(m.volume(), (0.01 - 0.0008) * 2.0) < 2e-5);
}

#[test]
fn composite_floor_and_rc_section_include_every_part() {
    let steel = param(ProfileKind::IShape, &[("h", 0.3), ("b", 0.15), ("tw", 0.007), ("tf", 0.01)]);
    let floor = composite_floor(steel.clone(), 0.3, deck_slab(1.2, 0.07, 0.06, 0.12, 0.08, 0.15));
    let steel_area = steel.area().unwrap();
    let slab = 1.2 * 0.07 + 8.0 * 0.06 * (0.12 + 0.08) / 2.0;
    assert!(rel(floor.area().unwrap(), steel_area + slab) < 1e-9);
    assert!(straight_mesh(&floor, 2.0, TOL, Lod::Lod2).unwrap().is_watertight());
    let rc = rc_rectangle(0.3, 0.5, 0.03, 0.02, 0.008, 3, 2);
    let concrete = straight_mesh(&rc, 4.0, TOL, Lod::Lod1).unwrap();
    let reinforced = straight_mesh(&rc, 4.0, TOL, Lod::Lod2).unwrap();
    let bars = 5.0 * PI * 0.01f64.powi(2) * 4.0;
    assert!(rel(reinforced.volume() - concrete.volume(), bars) < 0.05, "{}", reinforced.volume() - concrete.volume());
}

#[test]
fn tapered_members_follow_the_linear_blend() {
    let g = |taper: Option<SectionShape>| MemberGeometry {
        axis: Path3::Polyline(Polyline3 { points: vec![Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0)] }),
        frame: Frame::WORLD,
        section: param(ProfileKind::Rect, &[("h", 0.2), ("b", 0.1)]),
        section_taper: taper,
        start_offset: Vec2::ZERO,
        end_offset: Vec2::ZERO,
        start_release: EndRelease::Rigid,
        end_release: EndRelease::Rigid,
        stations: vec![],
    };
    let m = member_mesh(&g(Some(param(ProfileKind::Rect, &[("h", 0.6), ("b", 0.1)]))), TOL, Lod::Lod2).unwrap();
    assert!(rel(m.volume(), 4.0 * 0.1 * 0.4) < 2e-5 && m.is_watertight());
    let bad = member_mesh(&g(Some(param(ProfileKind::IShape, &[("h", 0.6), ("b", 0.1), ("tw", 0.01), ("tf", 0.01)]))), TOL, Lod::Lod2);
    assert!(bad.is_err());
    let slot = m.scalar_slot.as_ref().unwrap();
    assert!(slot.iter().cloned().fold(f32::MIN, f32::max) <= 1.0 && slot.iter().cloned().fold(f32::MAX, f32::min) >= 0.0);
}

#[test]
fn curved_and_cambered_members_sweep_along_the_frame() {
    let arc = Arc { centre: Vec3::ZERO, normal: Vec3::Z, start: Vec3::new(10.0, 0.0, 0.0), sweep: PI / 2.0 };
    let mut g = MemberGeometry {
        axis: Path3::Arc(arc),
        frame: Frame { origin: Vec3::new(10.0, 0.0, 0.0), x: Vec3::Y, y: Vec3::new(-1.0, 0.0, 0.0), z: Vec3::Z },
        section: param(ProfileKind::Rect, &[("h", 0.4), ("b", 0.2)]),
        section_taper: None,
        start_offset: Vec2::ZERO,
        end_offset: Vec2::ZERO,
        start_release: EndRelease::Rigid,
        end_release: EndRelease::Rigid,
        stations: vec![],
    };
    let fine = member_mesh(&g, Tolerance::new(1e-4), Lod::Lod3).unwrap();
    assert!(fine.is_watertight());
    let n = segments(10.0, PI / 2.0, Tolerance::new(1e-4), Lod::Lod3) as f64;
    let polygonal = 0.08 * 10.0 * (PI / 2.0) * ((PI / 2.0 / n / 2.0).sin() / (PI / 2.0 / n / 2.0));
    assert!(rel(fine.volume(), polygonal) < 5e-3, "{} vs {polygonal}", fine.volume());
    g.axis = Path3::Spline(Spline3 { control: vec![Vec3::ZERO, Vec3::new(2.0, 0.0, 0.05), Vec3::new(4.0, 0.0, 0.05), Vec3::new(6.0, 0.0, 0.0)], degree: 3 });
    g.frame = Frame::WORLD;
    let camber = member_mesh(&g, TOL, Lod::Lod3).unwrap();
    assert!(camber.is_watertight() && camber.aabb().max.z > 0.2 + 0.03);
}

#[test]
fn fasteners_instance_patterns_within_the_level_budget() {
    let bolt = |pattern| FastenerGeometry {
        kind: FastenerKind::Bolt { class: "8.8".into(), preload: None },
        axis: Segment3 { from: Vec3::ZERO, to: Vec3::new(0.0, 0.0, 0.06) },
        diameter: 0.02,
        head: HeadShape::Hex,
        pattern,
        hole_clearance: None,
    };
    let one = fastener_mesh(&bolt(None), TOL, Lod::Lod2).unwrap();
    let grid = fastener_mesh(&bolt(Some(Pattern::Grid { nx: 3, ny: 2, sx: 0.06, sy: 0.07 })), TOL, Lod::Lod2).unwrap();
    assert_eq!(grid.triangle_count(), 6 * one.triangle_count());
    assert!(one.aabb().max.z >= 0.06 - 1e-6 && one.volume() > 0.0);
    let capped = offsets(&Some(Pattern::Row { n: 500, s: 0.05 }), [0.0, 0.0, 1.0], TOL, Lod::Lod1);
    assert_eq!(capped.len(), Lod::Lod1.max_instances());
    let along = offsets(&Some(Pattern::Along { path: Path3::Polyline(Polyline3 { points: vec![Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0)] }), spacing: 0.25 }), [0.0, 0.0, 1.0], TOL, Lod::Lod3);
    assert_eq!(along.len(), 5);
}

#[test]
fn rebar_stirrups_studs_and_tendons_have_the_expected_volume() {
    let straight = FastenerGeometry { kind: FastenerKind::Rebar { bend: None }, axis: Segment3 { from: Vec3::ZERO, to: Vec3::new(3.0, 0.0, 0.0) }, diameter: 0.016, head: HeadShape::None, pattern: None, hole_clearance: None };
    let bar = fastener_mesh(&straight, Tolerance::new(1e-5), Lod::Lod3).unwrap();
    assert!(rel(bar.volume(), PI * 0.008f64.powi(2) * 3.0) < 5e-3 && bar.is_watertight());
    let outline = Ring2 { points: vec![Vec2::new(-0.1, -0.2), Vec2::new(0.1, -0.2), Vec2::new(0.1, 0.2), Vec2::new(-0.1, 0.2)] };
    let zones = vec![LinkZone { start: 0.0, spacing: 0.1, count: 3 }, LinkZone { start: 1.0, spacing: 0.3, count: 2 }];
    let cage = FastenerGeometry { pattern: Some(Pattern::Cage { outline, zones, reference: Vec3::Y }), diameter: 0.008, ..straight.clone() };
    let loops = fastener_mesh(&cage, TOL, Lod::Lod3).unwrap();
    let bounds = loops.aabb();
    assert!(loops.volume() > 0.0 && bounds.max.x >= 1.3 - 0.01 && bounds.max.x <= 1.3 + 0.01);
    assert!(bounds.max.y.max(bounds.max.z) >= 0.2 - 1e-3 && bounds.max.y.max(bounds.max.z) <= 0.21);
    let stud = FastenerGeometry { kind: FastenerKind::Stud { head_height: 0.01, head_diameter: 0.032 }, axis: Segment3 { from: Vec3::ZERO, to: Vec3::new(0.0, 0.0, 0.1) }, diameter: 0.019, head: HeadShape::Headed, pattern: None, hole_clearance: None };
    assert!(fastener_mesh(&stud, TOL, Lod::Lod2).unwrap().volume() > 0.0);
    let tendon = FastenerGeometry { kind: FastenerKind::Tendon { profile: Path3::Spline(Spline3 { control: vec![Vec3::ZERO, Vec3::new(5.0, 0.0, -0.3), Vec3::new(10.0, 0.0, 0.0)], degree: 2 }), duct: Some(0.08) }, diameter: 0.05, ..straight.clone() };
    assert!(fastener_mesh(&tendon, TOL, Lod::Lod2).unwrap().is_watertight());
    let bent = FastenerGeometry { kind: FastenerKind::Rebar { bend: Some(BendShape { bend_angle: PI / 2.0, bend_radius: 0.05, leg_length: 0.2, both_ends: false }) }, ..straight.clone() };
    assert!(fastener_mesh(&bent, TOL, Lod::Lod2).unwrap().aabb().max.x > 3.0);
    let u_bar = FastenerGeometry { kind: FastenerKind::Rebar { bend: Some(BendShape { bend_angle: PI / 2.0, bend_radius: 0.05, leg_length: 0.2, both_ends: true }) }, ..straight };
    let (one, two) = (fastener_mesh(&bent, TOL, Lod::Lod2).unwrap().aabb(), fastener_mesh(&u_bar, TOL, Lod::Lod2).unwrap().aabb());
    assert!(two.min.x < one.min.x - 0.04 && (two.max.x - one.max.x).abs() < 1.0e-9, "the second hook extends the bar past its start and leaves the far end alone");
}

#[test]
fn points_and_weld_runs_follow_their_own_geometry() {
    let at = |x, y, z| Vec3::new(x, y, z);
    let dowel = FastenerGeometry { kind: FastenerKind::Dowel, axis: Segment3 { from: Vec3::ZERO, to: at(0.0, 0.0, 0.1) }, diameter: 0.012, head: HeadShape::None, pattern: Some(Pattern::Points { offsets: vec![at(0.0, 0.0, 0.0), at(0.0, 0.3, 0.1), at(0.0, 0.7, -0.2)] }), hole_clearance: None };
    let single = fastener_mesh(&FastenerGeometry { pattern: None, ..dowel.clone() }, TOL, Lod::Lod2).unwrap();
    let scattered = fastener_mesh(&dowel, TOL, Lod::Lod2).unwrap();
    assert_eq!(scattered.triangle_count(), 3 * single.triangle_count());
    assert!(scattered.aabb().max.y > single.aabb().max.y + 0.6);
    let path = Path3::Polyline(Polyline3 { points: vec![Vec3::ZERO, at(1.0, 0.0, 0.0), at(1.0, 1.0, 0.0)] });
    let run = |pitch| FastenerGeometry { kind: FastenerKind::WeldRun { throat: 0.004, weld: WeldKind::Fillet, path: path.clone(), pitch }, axis: Segment3 { from: Vec3::ZERO, to: at(1.0, 1.0, 0.0) }, diameter: 0.01, head: HeadShape::None, pattern: None, hole_clearance: None };
    let seam = fastener_mesh(&run(None), TOL, Lod::Lod2).unwrap();
    let a = 0.004 * 2f64.sqrt();
    assert!(rel(seam.volume(), 2.0 * 0.5 * a * a) < 1e-6);
    let chain = fastener_mesh(&run(Some(WeldPitch { length: 0.2, gap: 0.2 })), TOL, Lod::Lod2).unwrap();
    assert!(rel(chain.volume(), 6.0 * 0.2 * 0.5 * a * a) < 1e-6);
}

#[test]
fn welds_screws_dowels_and_anchors_tessellate() {
    for kind in [FastenerKind::Weld { throat: 0.004, weld: WeldKind::Fillet }, FastenerKind::Weld { throat: 0.004, weld: WeldKind::Butt }, FastenerKind::Screw, FastenerKind::Nail, FastenerKind::Dowel, FastenerKind::Anchor] {
        let g = FastenerGeometry { kind, axis: Segment3 { from: Vec3::ZERO, to: Vec3::new(0.5, 0.0, 0.0) }, diameter: 0.012, head: HeadShape::Countersunk, pattern: None, hole_clearance: None };
        assert!(fastener_mesh(&g, TOL, Lod::Lod2).unwrap().volume() > 0.0);
    }
}

fn frame() -> Frame {
    Frame::WORLD
}

#[test]
fn solids_have_analytical_volumes() {
    let sq = Polygon2::solid(vec![Vec2::new(0.0, 0.0), Vec2::new(2.0, 0.0), Vec2::new(2.0, 3.0), Vec2::new(0.0, 3.0)]);
    let v = |s: SolidShape| solid_mesh(&s, TOL, Lod::Lod3).unwrap().volume();
    assert!(rel(v(SolidShape::Prism { frame: frame(), base: sq.clone(), height: 0.5 }), 3.0) < 1e-6);
    assert!(rel(v(SolidShape::Box { frame: frame(), size: Vec3::new(1.0, 2.0, 3.0) }), 6.0) < 1e-6);
    assert!(rel(v(SolidShape::Wedge { frame: frame(), base: Vec2::new(2.0, 3.0), height: 1.0 }), 3.0) < 1e-6);
    let n = segments(0.5, TAU, TOL, Lod::Lod3) as f64;
    assert!(rel(v(SolidShape::Cylinder { frame: frame(), radius: 0.5, height: 4.0 }), 4.0 * 0.5 * 0.25 * n * (TAU / n).sin()) < 1e-6);
    let strata = SolidShape::StrataColumn { frame: frame(), footprint: sq.clone(), strata: vec![
        Stratum { thickness: 1.0, role: MaterialRole::Soil { class: "sand".into() }, label: LocalizedCopy::new("Sand", "Sand") },
        Stratum { thickness: 2.5, role: MaterialRole::Soil { class: "clay".into() }, label: LocalizedCopy::new("Clay", "Ton") },
    ] };
    let parts = super::solids::solid_parts(&strata, TOL, Lod::Lod2).unwrap();
    assert_eq!(parts.len(), 2);
    assert!(rel(parts[1].1.aabb().min.z, -3.5) < 1e-6 && rel(v(strata), 21.0) < 1e-6);
    let footing = SolidShape::FootingBody { frame: frame(), footprint: Polygon2::solid(vec![Vec2::new(-1.0, -1.0), Vec2::new(1.0, -1.0), Vec2::new(1.0, 1.0), Vec2::new(-1.0, 1.0)]), thickness: 0.6, pedestal: Some(Vec2::new(0.4, 0.4)), pedestal_height: 0.5 };
    assert!(rel(v(footing), 4.0 * 0.6 + 0.16 * 0.5) < 1e-6);
    let layers = vec![
        WallLayer { thickness: 0.1, role: MaterialRole::Masonry, label: LocalizedCopy::new("Facing", "Vormauer") },
        WallLayer { thickness: 0.2, role: MaterialRole::Masonry, label: LocalizedCopy::new("Core", "Kern") },
    ];
    let path = Polyline3 { points: vec![Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0)] };
    assert!(rel(v(SolidShape::WallAssembly { path: path.clone(), height: 2.5, layers: layers.clone(), openings: Vec::new() }), 4.0 * 2.5 * 0.3) < 1e-6);
    let cut = |offset, width, sill, height| WallCut { offset, width, sill, height };
    let holed = SolidShape::WallAssembly { path, height: 2.5, layers, openings: vec![cut(0.5, 1.0, 0.0, 2.1), cut(2.0, 1.2, 0.9, 1.2)] };
    assert!(rel(v(holed), (4.0 * 2.5 - 2.1 - 1.44) * 0.3) < 1e-6);
    let boards = |t: f64, grain| GrainLayer { thickness: t, role: MaterialRole::Timber, grain, label: LocalizedCopy::new("Board", "Brett") };
    let clt = SolidShape::Laminate { frame: frame(), footprint: sq.clone(), layers: vec![boards(0.04, 0.0), boards(0.03, PI / 2.0), boards(0.04, 0.0)] };
    assert_eq!(super::solids::solid_parts(&clt, TOL, Lod::Lod2).unwrap().len(), 3);
    assert!(rel(v(clt), 6.0 * 0.11) < 1e-6);
    let p = solid_mesh(&pile(Vec3::ZERO, 12.0, 0.6), TOL, Lod::Lod2).unwrap();
    assert!(p.aabb().min.z < -11.99 && p.volume() > 0.0);
}

#[test]
fn walls_with_openings_subtract_the_openings() {
    let m = wall_with_openings([0.0; 3], [5.0, 0.0, 0.0], 3.0, 0.3, 0.0, &[Opening { x: 1.0, z: 0.0 + 0.01, width: 1.0, height: 2.0 }, Opening { x: 3.0, z: 0.9, width: 1.2, height: 1.2 }]).unwrap();
    assert!(m.is_watertight() && rel(m.volume(), (15.0 - 2.0 - 1.44) * 0.3) < 1e-6);
    assert!(wall_with_openings([0.0; 3], [5.0, 0.0, 0.0], 3.0, 0.3, 0.0, &[Opening { x: 4.5, z: 1.0, width: 1.0, height: 1.0 }]).is_err());
}

#[test]
fn shells_plates_and_silos_have_pappus_volumes() {
    let shell = cylindrical_shell(5.0, 10.0, 0.02);
    let m = shell_mesh(&shell, TOL, Lod::Lod3).unwrap();
    let n = segments(5.01, TAU, TOL, Lod::Lod3) as f64;
    let exact = TAU * 5.0 * 0.02 * 10.0 * (TAU / n).sin() / (TAU / n);
    assert!(rel(m.volume(), exact) < 1e-6 && m.is_watertight(), "{} {exact}", m.volume());
    let silo = shell_mesh(&silo_shell(3.0, 8.0, 3.0, 0.3, 1.5, 0.01), TOL, Lod::Lod2).unwrap();
    assert!(silo.is_watertight() && silo.aabb().min.z < -2.9 && silo.aabb().max.z > 9.4);
    let plate = PlateGeometry {
        frame: frame(),
        outline: Polygon2 { outer: Ring2 { points: vec![Vec2::new(0.0, 0.0), Vec2::new(2.0, 0.0), Vec2::new(2.0, 1.0), Vec2::new(0.0, 1.0)] }, holes: vec![Ring2 { points: vec![Vec2::new(0.5, 0.25), Vec2::new(0.9, 0.25), Vec2::new(0.9, 0.65), Vec2::new(0.5, 0.65)] }] },
        thickness: 0.02,
        offset: 0.0,
        stiffeners: vec![Stiffener { path: Polyline2 { points: vec![Vec2::new(1.2, 0.1), Vec2::new(1.2, 0.9)] }, section: param(ProfileKind::Rect, &[("h", 0.08), ("b", 0.01)]) }],
    };
    let pm = plate_mesh(&plate, TOL, Lod::Lod2).unwrap();
    assert!(rel(pm.volume(), (2.0 - 0.16) * 0.02 + 0.08 * 0.01 * 0.8) < 1e-6, "{}", pm.volume());
}

#[test]
fn surfaces_are_double_sided() {
    let tri = Polygon3 { outer: Ring3 { points: vec![Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)] }, holes: vec![] };
    let m = Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: tri } }).mesh(TOL, Lod::Lod2);
    assert_eq!(m.triangle_count(), 2);
    assert!((m.surface_area() - 1.0).abs() < 1e-6);
}

#[test]
fn glyphs_anchor_at_their_points_and_supports_are_non_empty() {
    let case = LoadCaseRef { id: "g".into(), kind: LoadKind::Permanent, psi: None };
    let at = Vec3::new(1.0, 2.0, 3.0);
    let arrow = load_mesh(&LoadGlyph::Point { at: at.clone(), direction: Vec3::new(0.0, 0.0, -1.0), magnitude: Quantity::new(QuantityKind::Force, 10_000.0), case: case.clone() }, TOL, Lod::Lod2);
    let b = arrow.aabb();
    assert!((b.min.z - 3.0).abs() < 1e-6 && b.max.z > 3.5);
    let line = LoadGlyph::Line { path: Path3::Polyline(Polyline3 { points: vec![Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0)] }), direction: Vec3::new(0.0, 0.0, -1.0), profile: LoadProfile::Linear { start: Quantity::new(QuantityKind::ForcePerLength, 1e3), end: Quantity::new(QuantityKind::ForcePerLength, 1e5) }, case: case.clone() };
    assert!(load_mesh(&line, TOL, Lod::Lod3).triangle_count() > load_mesh(&line, TOL, Lod::Lod1).triangle_count());
    let area = LoadGlyph::Area { polygon: Polygon3 { outer: Ring3 { points: vec![Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0), Vec3::new(4.0, 3.0, 0.0), Vec3::new(0.0, 3.0, 0.0)] }, holes: vec![] }, direction: Vec3::new(0.0, 0.0, -1.0), intensity: Quantity::new(QuantityKind::Pressure, 2e3), case: case.clone() };
    assert!(!load_mesh(&area, TOL, Lod::Lod2).is_empty());
    for glyph in [
        LoadGlyph::Moment { at: at.clone(), axis: Vec3::Y, magnitude: Quantity::new(QuantityKind::Moment, 1e4), case: case.clone() },
        LoadGlyph::Impact { at: at.clone(), direction: Vec3::X, energy_or_force: Quantity::new(QuantityKind::Force, 1e5), case: case.clone() },
        LoadGlyph::Explosion { centre: at.clone(), pressure: Quantity::new(QuantityKind::Pressure, 1e4), shape: PressureWave::Spherical, case: case.clone() },
        LoadGlyph::Explosion { centre: at.clone(), pressure: Quantity::new(QuantityKind::Pressure, 1e4), shape: PressureWave::Planar, case: case.clone() },
        LoadGlyph::Inertial { origin: at.clone(), direction: Vec3::X, masses: vec![PointMass { at: at.clone(), mass: Quantity::new(QuantityKind::Mass, 5e3) }], case: case.clone() },
        LoadGlyph::Pressure { face: EntityRef::new(EntityKind::Plate, "p", "plates[id=p]"), zones: vec![PressureZone { polygon: Polygon3 { outer: Ring3 { points: vec![Vec3::ZERO, Vec3::X, Vec3::Y] }, holes: vec![] }, value: Quantity::new(QuantityKind::Pressure, -800.0), label: LocalizedCopy::new("A", "A") }], case: case.clone() },
    ] {
        assert!(!load_mesh(&glyph, TOL, Lod::Lod2).is_empty());
    }
    let thermal = LoadGlyph::Thermal { target: EntityRef::new(EntityKind::Member, "m", "members[id=m]"), delta_t: Quantity::new(QuantityKind::Temperature, 20.0), case };
    assert!(load_mesh(&thermal, TOL, Lod::Lod2).is_empty());
    let bed = Polygon3 { outer: Ring3 { points: vec![Vec3::ZERO, Vec3::X, Vec3::new(1.0, 1.0, 0.0), Vec3::Y] }, holes: vec![] };
    for kind in [SupportKind::Fixed, SupportKind::Pinned, SupportKind::Roller { normal: Vec3::Z }, SupportKind::Spring { stiffness: vec![Quantity::new(QuantityKind::Stiffness, 1.0); 6] }, SupportKind::ElasticBed { stiffness: Quantity::new(QuantityKind::Stiffness, 1e6), extent: bed.clone() }, SupportKind::PileHead] {
        assert!(!support_mesh(&SupportGeometry { kind, at: at.clone(), frame: frame() }, TOL, Lod::Lod2).is_empty());
    }
}

#[test]
fn impostor_is_a_box_and_levels_are_deterministic() {
    let section = param(ProfileKind::IShape, &[("h", 0.24), ("b", 0.24), ("tw", 0.01), ("tf", 0.017)]);
    let geometry = Geometry::Member(MemberGeometry {
        axis: Path3::Polyline(Polyline3 { points: vec![Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0)] }),
        frame: Frame::WORLD,
        section,
        section_taper: None,
        start_offset: Vec2::ZERO,
        end_offset: Vec2::ZERO,
        start_release: EndRelease::Rigid,
        end_release: EndRelease::Rigid,
        stations: vec![],
    });
    let box0 = geometry.mesh(TOL, Lod::Lod0);
    assert_eq!(box0.triangle_count(), 12);
    assert!(rel(box0.volume(), 4.0 * 0.24 * 0.24) < 1e-6);
    assert_eq!(geometry.mesh(TOL, Lod::Lod2), geometry.mesh(TOL, Lod::Lod2));
    assert_eq!(geometry.key(), geometry.clone().key());
    assert_ne!(geometry.key().at(Lod::Lod1), geometry.key().at(Lod::Lod2));
    let b = geometry_bounds(&geometry).unwrap();
    assert!((b.max.x - 4.0).abs() < 1e-6 && (b.max.y - 0.12).abs() < 1e-6);
    let normals_unit = geometry.mesh(TOL, Lod::Lod2).normals.iter().all(|n| ((n[0] * n[0] + n[1] * n[1] + n[2] * n[2]) - 1.0).abs() < 1e-4);
    assert!(normals_unit);
}

#[test]
fn outward_normals_point_away_from_the_centroid() {
    let m = straight_mesh(&param(ProfileKind::Rect, &[("h", 0.5), ("b", 0.3)]), 2.0, TOL, Lod::Lod2).unwrap();
    let c = [1.0f32, 0.0, 0.0];
    assert!(m.positions.iter().zip(&m.normals).all(|(p, n)| (p[0] - c[0]) * n[0] + (p[1] - c[1]) * n[1] + (p[2] - c[2]) * n[2] > 0.0));
}

#[test]
fn geometry_oracle_agrees_with_third_party_libraries() {
    use semio_s_plugin_norm_compliance::python_oracle;
    let mut cases = Vec::new();
    for (name, profile, params, _) in catalogue() {
        let section = param(profile, &params);
        let r = regions(&section, TOL, Lod::Lod2).unwrap();
        let props = properties(&r);
        let mesh = straight_mesh(&section, 3.0, TOL, Lod::Lod2).unwrap();
        let bb = mesh.aabb();
        cases.push(serde_json::json!({
            "name": name, "profile": name, "params": params.iter().cloned().collect::<std::collections::BTreeMap<_, _>>(), "length": 3.0,
            "rust": { "area": props.area, "cu": props.cu, "cv": props.cv, "ixx": props.ixx, "iyy": props.iyy, "ixy": props.ixy },
            "positions": mesh.positions, "indices": mesh.indices,
            "aabb": [bb.min.x, bb.min.y, bb.min.z, bb.max.x, bb.max.y, bb.max.z],
        }));
    }
    let chs = param(ProfileKind::Chs, &[("d", 0.2), ("t", 0.01)]);
    let r = regions(&chs, TOL, Lod::Lod3).unwrap();
    let props = properties(&r);
    cases.push(serde_json::json!({
        "name": "chs", "profile": "chs", "params": { "d": 0.2, "t": 0.01 }, "segments": r[0].outer.points.len(),
        "rust": { "area": props.area, "cu": props.cu, "cv": props.cv, "ixx": props.ixx, "iyy": props.iyy, "ixy": props.ixy },
    }));
    let base = [("h", 0.24), ("b", 0.24), ("tw", 0.01), ("tf", 0.017)];
    let mut filleted = base.to_vec();
    filleted.push(("r", 0.021));
    let section = param(ProfileKind::IShape, &filleted);
    let rr = regions(&section, TOL, Lod::Lod3).unwrap();
    cases.push(serde_json::json!({
        "name": "ishape-fillet", "profile": "ishape", "params": base.iter().cloned().collect::<std::collections::BTreeMap<_, _>>(), "fillets": 4, "radius": 0.021,
        "rust": { "area": properties(&rr).area, "cu": 0.0, "cv": 0.0, "ixx": 0.0, "iyy": 0.0, "ixy": 0.0 },
    }));
    let input = serde_json::json!({ "cases": cases }).to_string();
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🔮️oracles/🐍️.py");
    let out = python_oracle::run_ok(&script, &[], Some(input.as_bytes()));
    let report: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(report["failures"], serde_json::json!([]), "{report}");
    assert_eq!(report["cases"], cases.len());
}

#[test]
fn derived_values_stay_finite() {
    let m = straight_mesh(&param(ProfileKind::Circle, &[("d", 0.3)]), 1.0, TOL, Lod::Lod3).unwrap();
    assert!(m.positions.iter().flatten().chain(m.normals.iter().flatten()).all(|v| v.is_finite()));
    assert!(m.indices.iter().all(|&i| (i as usize) < m.positions.len()));
}
