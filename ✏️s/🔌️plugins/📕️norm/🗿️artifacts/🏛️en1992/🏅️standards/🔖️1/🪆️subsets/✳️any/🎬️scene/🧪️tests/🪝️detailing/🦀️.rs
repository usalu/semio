//! 🪝️ Detailing views of the EN 1992 scene: hooks, laps of long bars, link shapes from the section outline, the parabolic tendon profile and the crack pattern following a moved bar.

use super::*;
use model::{En1992Snapshot, MemberKind};
use reinforcement::{bar_hooks, bend_mode, centred_span, hook, hook_end, lap_spans, offset_inward, BendMode, HookEnd};
use scene::projection_kit::test_support::revision;
use scene::{Geometry, Validate};
use semio_s_artifact_norm_en1992_checks as checks;

fn report_of(document: &En1992Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    for member in &document.members {
        report.extend(checks::evaluate_member(document, member));
    }
    report
}

fn built_of(document: &En1992Snapshot) -> Built {
    build(&SceneInput { document, report: &report_of(document), placements: &[], revision: revision() })
}

fn beam_of(document: &mut En1992Snapshot) -> &mut model::RcMember {
    document.members.iter_mut().find(|m| m.id == "beam-B1").expect("beam-B1")
}

fn area(points: &[(f64, f64)]) -> f64 {
    (0..points.len()).map(|i| points[i].0 * points[(i + 1) % points.len()].1 - points[(i + 1) % points.len()].0 * points[i].1).sum::<f64>() / 2.0
}

#[test]
fn bars_longer_than_the_stock_length_are_lapped_by_the_lap_length() {
    let pieces = lap_spans(0.0, 30.0, 12.0, 0.8);
    assert!(pieces.len() >= 3);
    assert_eq!(pieces[0].0, 0.0);
    assert!((pieces[pieces.len() - 1].1 - 30.0).abs() < 1e-9);
    assert!(pieces.iter().all(|p| p.1 - p.0 <= 12.0 + 1e-9));
    assert!(pieces.windows(2).all(|w| ((w[0].1 - w[1].0) - 0.8).abs() < 1e-9));
    assert_eq!(lap_spans(0.0, 10.0, 12.0, 0.8), vec![(0.0, 10.0)]);
    assert_eq!(lap_spans(0.0, 30.0, 12.0, 0.0), vec![(0.0, 30.0)]);
}

#[test]
fn a_long_beam_shows_lapped_pieces_with_the_provided_lap_length() {
    let mut document = En1992Snapshot::compliant_office_frame();
    {
        let beam = beam_of(&mut document);
        beam.span = 30.0;
        beam.longitudinal[0].lap_length = 0.9;
        beam.longitudinal[0].lap_share = 1.0;
    }
    let built = built_of(&document);
    assert_eq!(built.scene.validate(), Ok(()));
    let layer = document.members.iter().find(|m| m.id == "beam-B1").expect("beam").longitudinal[0].clone();
    let laps = built.scene.entities.iter().filter(|e| e.entity.id.starts_with(&format!("beam-B1:lap-{}-", layer.id))).count();
    assert_eq!(laps, (lapped_pieces(&document) - 1) * layer.count as usize);
    let bars = built.scene.entities.iter().filter(|e| e.entity.id.starts_with("beam-B1:bar-")).count();
    assert_eq!(bars, document.members.iter().find(|m| m.id == "beam-B1").expect("beam").longitudinal.iter().map(|l| l.count as usize).sum::<usize>());
}

fn lapped_pieces(document: &En1992Snapshot) -> usize {
    let beam = document.members.iter().find(|m| m.id == "beam-B1").expect("beam");
    let (cv, l) = (beam.cover, beam.span);
    let (from, to) = if l - 2.0 * cv > 0.1 { (cv, l - cv) } else { (0.0, l) };
    lap_spans(from, to, reinforcement::STOCK_LENGTH, beam.longitudinal[0].lap_length).len()
}

#[test]
fn the_hook_follows_the_mandrel_rule_of_table_8_1n() {
    let small = hook(0.016);
    assert!((small.radius - 0.040).abs() < 1e-12 && (small.leg - 0.080).abs() < 1e-12);
    let large = hook(0.020);
    assert!((large.radius - 0.080).abs() < 1e-12 && (large.leg - 0.100).abs() < 1e-12);
    assert!((small.angle - 5.0 * std::f64::consts::PI / 6.0).abs() < 1e-12);
}

#[test]
fn the_hook_end_turns_the_bend_side_into_the_section() {
    assert_eq!(hook_end(&scene::Vec3::X, &scene::Vec3::Z), HookEnd::Far);
    assert_eq!(hook_end(&scene::Vec3::X, &scene::Vec3::new(0.0, 0.0, -1.0)), HookEnd::Near);
    assert_eq!(hook_end(&scene::Vec3::X, &scene::Vec3::Y), HookEnd::Neither);
}

#[test]
fn a_bottom_layer_whose_straight_anchorage_fails_gets_a_hooked_end_towards_the_section() {
    let mut document = En1992Snapshot::compliant_office_frame();
    let layer_id = {
        let beam = beam_of(&mut document);
        beam.longitudinal[0].anchorage_length = 0.02;
        beam.longitudinal[0].id.clone()
    };
    let built = built_of(&document);
    let hooked: Vec<_> = built
        .scene
        .entities
        .iter()
        .filter(|e| e.entity.id.starts_with(&format!("beam-B1:bar-{layer_id}-")))
        .filter(|e| matches!(&e.geometry, Geometry::Fastener(g) if matches!(&g.kind, scene::FastenerKind::Rebar { bend: Some(_) })))
        .collect();
    assert!(!hooked.is_empty(), "a failing straight anchorage shows hooks");
    assert!(hooked.iter().all(|e| matches!(&e.geometry, Geometry::Fastener(g) if matches!(&g.kind, scene::FastenerKind::Rebar { bend: Some(b) } if b.both_ends))), "a bar of one piece is hooked at both ends (U-bar)");
    let intact = En1992Snapshot::compliant_office_frame();
    let plain = built_of(&intact).scene.entities.iter().filter(|e| matches!(&e.geometry, Geometry::Fastener(g) if matches!(&g.kind, scene::FastenerKind::Rebar { bend: Some(_) }) && g.pattern.is_none())).count();
    assert_eq!(plain, 0);
}

#[test]
fn links_follow_the_section_outline_inset_by_cover_and_half_the_link() {
    let square = [(-0.2, -0.3), (0.2, -0.3), (0.2, 0.3), (-0.2, 0.3)];
    let inner = offset_inward(&square, 0.05);
    assert!((area(&inner) - 0.3 * 0.5).abs() < 1e-12);
    let clockwise: Vec<_> = square.iter().rev().copied().collect();
    assert!((area(&offset_inward(&clockwise, 0.05)).abs() - 0.3 * 0.5).abs() < 1e-12);
    let ell = [(-0.15, -0.25), (0.15, -0.25), (0.15, 0.0), (0.0, 0.0), (0.0, 0.25), (-0.15, 0.25)];
    let moved = offset_inward(&ell, 0.04);
    assert_eq!(moved.len(), ell.len());
    assert!(area(&moved) < area(&ell) && area(&moved) > 0.0);
    assert!(moved.iter().all(|p| p.0 > -0.15 && p.0 < 0.15 && p.1 > -0.25 && p.1 < 0.25));
}

#[test]
fn a_polygon_section_gets_a_cage_outline_with_its_own_vertices() {
    let mut document = En1992Snapshot::compliant_office_frame();
    {
        let beam = beam_of(&mut document);
        beam.section_points = [(-0.15, -0.25), (0.15, -0.25), (0.15, 0.0), (0.0, 0.0), (0.0, 0.25), (-0.15, 0.25)].iter().enumerate().map(|(i, p)| model::SectionPoint { id: format!("p{i}"), y: p.0, z: p.1 }).collect();
    }
    let built = built_of(&document);
    let links = built.scene.entities.iter().find(|e| e.entity.id == "beam-B1:links").expect("links");
    let Geometry::Fastener(g) = &links.geometry else { panic!("fastener") };
    let Some(scene::Pattern::Cage { outline, .. }) = &g.pattern else { panic!("cage") };
    assert_eq!(outline.points.len(), 6);
    assert!(outline.signed_area().abs() < 0.3 * 0.25 + 0.15 * 0.25);
}

#[test]
fn the_tendon_passes_through_every_control_point_on_a_parabola() {
    let document = {
        let mut document = En1992Snapshot::compliant_office_frame();
        let beam = beam_of(&mut document);
        let l = beam.span;
        beam.tendon_points = [(0.0, 0.0), (l / 4.0, -0.075), (l / 2.0, -0.1), (3.0 * l / 4.0, -0.075), (l, 0.0)].iter().enumerate().map(|(i, p)| model::TendonPoint { id: format!("t{i}"), x: p.0, y: 0.0, z: p.1 }).collect();
        beam.prestress = Some(model::PrestressSpec { force: 800.0e3, area: 5.0e-4, eccentricity: 0.1, loss_ratio: 0.2 });
        beam.prestress_steel_id = "yp1860".into();
        document
    };
    let beam = document.members.iter().find(|m| m.id == "beam-B1").expect("beam");
    let l = beam.span;
    let exact = |x: f64| -0.1 * (1.0 - ((x - l / 2.0) / (l / 2.0)).powi(2));
    for k in 0..=40 {
        let x = l * k as f64 / 40.0;
        assert!((beam.tendon_z_at(x).expect("profile") - exact(x)).abs() < 1e-12, "x = {x}");
    }
    for p in &beam.tendon_points {
        assert!((beam.tendon_z_at(p.x).expect("profile") - p.z).abs() < 1e-12);
    }
    let profile = beam.tendon_profile(12);
    assert_eq!(profile.len(), 4 * 12 + 1);
    assert!(profile.windows(2).all(|w| w[1].0 > w[0].0));
    let built = built_of(&document);
    let tendon = built.scene.entities.iter().find(|e| e.entity.id == "beam-B1:tendon").expect("tendon");
    let Geometry::Fastener(g) = &tendon.geometry else { panic!("fastener") };
    let scene::FastenerKind::Tendon { profile: scene::Path3::Polyline(line), .. } = &g.kind else { panic!("polyline tendon") };
    assert_eq!(line.points.len(), 4 * 12 + 1);
    assert!((beam.tendon_eccentricity_mid().expect("eccentricity") - 0.1).abs() < 1e-12);
}

#[test]
fn moving_a_bar_changes_the_crack_pattern_of_the_same_document() {
    let base = {
        let mut document = En1992Snapshot::compliant_office_frame();
        let beam = beam_of(&mut document);
        let layer = beam.longitudinal[0].clone();
        let z = -beam.height / 2.0 + beam.cover + 0.03;
        beam.bars = (0..4).map(|k| model::BarPoint { id: format!("b{k}"), layer_id: layer.id.clone(), y: -0.09 + 0.06 * k as f64, z, diameter: 0.016, length: 0.0, bend: semio_s_artifact_norm_en1992_model::BarBend::Auto }).collect();
        document
    };
    let mut raised = base.clone();
    beam_of(&mut raised).bars[1].z += 0.08;
    let (a, b) = (built_of(&base), built_of(&raised));
    assert_eq!(a.scene.validate(), Ok(()));
    assert_eq!(b.scene.validate(), Ok(()));
    let widths = |built: &Built| built.diagrams.iter().find(|d| d.id == "crack-pattern:beam-B1").map(|d| d.table_twin.columns.iter().flat_map(|c| c.values.clone()).collect::<Vec<f64>>());
    assert_ne!(widths(&a), widths(&b), "a moved bar must move the cracks");
    let crack = |built: &Built| built.scene.entities.iter().filter(|e| e.entity.id.starts_with("beam-B1:crack-")).count();
    assert!(crack(&a) > 0 || crack(&b) > 0);
}

#[test]
fn every_beam_fixture_validates_with_details() {
    for document in [En1992Snapshot::compliant_office_frame(), En1992Snapshot::failing_under_reinforced(), En1992Snapshot::compliant_prestressed_beam()] {
        let built = built_of(&document);
        assert_eq!(built.scene.validate(), Ok(()), "{}", document.title);
        assert!(document.members.iter().any(|m| m.kind == MemberKind::Beam));
    }
}

#[test]
fn a_persisted_bar_length_is_centred_in_the_span_and_the_bend_text_selects_the_hooks() {
    assert_eq!(centred_span(0.0, 6.0, 0.0), None);
    assert_eq!(centred_span(0.0, 6.0, 4.0), Some((1.0, 5.0)));
    assert_eq!(centred_span(0.2, 5.8, 9.0), Some((0.2, 5.8)));
    assert_eq!([bend_mode(model::BarBend::Auto), bend_mode(model::BarBend::Straight), bend_mode(model::BarBend::Hook), bend_mode(model::BarBend::UHook)], [BendMode::Auto, BendMode::Straight, BendMode::Hook, BendMode::UHook]);
    assert_eq!(bar_hooks(BendMode::Auto, HookEnd::Near, HookEnd::Far, true), (HookEnd::Near, true));
    assert_eq!(bar_hooks(BendMode::Straight, HookEnd::Near, HookEnd::Far, true), (HookEnd::Neither, false));
    assert_eq!(bar_hooks(BendMode::Hook, HookEnd::Neither, HookEnd::Near, true), (HookEnd::Near, false));
    assert_eq!(bar_hooks(BendMode::UHook, HookEnd::Neither, HookEnd::Neither, false), (HookEnd::Far, true));
}

#[test]
fn explicit_bars_draw_their_persisted_length_and_hooks_and_carry_a_length_grip() {
    let mut document = En1992Snapshot::compliant_office_frame();
    let beam = beam_of(&mut document);
    let layer = beam.longitudinal[0].clone();
    let span = beam.span;
    let z = -beam.height / 2.0 + beam.cover + 0.03;
    beam.bars = (0..2).map(|k| model::BarPoint { id: format!("b{k}"), layer_id: layer.id.clone(), y: -0.05 + 0.1 * k as f64, z, diameter: 0.016, length: if k == 0 { 2.0 } else { 0.0 }, bend: if k == 0 { semio_s_artifact_norm_en1992_model::BarBend::Straight } else { semio_s_artifact_norm_en1992_model::BarBend::UHook } }).collect();
    let built = built_of(&document);
    assert_eq!(built.scene.validate(), Ok(()));
    let bar = |k: usize| {
        let entity = built.scene.entities.iter().find(|e| e.entity.id == format!("beam-B1:bar-{}-{k}", layer.id)).expect("bar");
        let Geometry::Fastener(g) = &entity.geometry else { panic!("fastener") };
        g.clone()
    };
    let (short, long) = (bar(0), bar(1));
    assert!((short.axis.from.distance(&short.axis.to) - 2.0).abs() < 1e-9);
    assert!(long.axis.from.distance(&long.axis.to) > 2.0 && long.axis.from.distance(&long.axis.to) <= span);
    assert!(matches!(short.kind, scene::FastenerKind::Rebar { bend: None }));
    assert!(matches!(&long.kind, scene::FastenerKind::Rebar { bend: Some(shape) } if shape.both_ends));
    assert!(built.handles.iter().any(|h| h.writes.iter().any(|w| w.path.ends_with("bars[id=b0].length") && (w.map.base - 2.0).abs() < 1e-9)));
}
