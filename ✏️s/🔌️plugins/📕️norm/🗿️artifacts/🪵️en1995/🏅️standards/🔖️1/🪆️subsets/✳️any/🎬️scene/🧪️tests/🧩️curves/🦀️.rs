use super::*;
use scene::projection_kit::test_support::{assert_handle_laws, revision};
use std::collections::BTreeSet;

fn report_of(document: &En1995Snapshot) -> CheckReport {
    schema::checks::timber::evaluate_structure(document)
}

fn scene_of(document: &En1995Snapshot) -> (StructuralScene, CheckReport) {
    let report = report_of(document);
    let scene = project(&SceneInput { document, report: &report, placements: &[], revision: revision() });
    (scene, report)
}

fn reinforced() -> En1995Snapshot {
    let mut document = schema::checks::timber::entity_demo_document();
    document.tapers[0].rods = 4;
    document.tapers[0].rod_resistance_n = 20_000.0;
    document.cutouts[0].rods = 2;
    document.cutouts[0].rod_resistance_n = 15_000.0;
    document.cutouts[0].slope = 0.5;
    document.cutouts[0].distance_m = 0.1;
    document
}

fn select(id: &str, collection: &str) -> Vec<scene::EntityRef> {
    vec![scene::EntityRef::new(scene::EntityKind::Group, id, format!("{collection}[id={id}]"))]
}

fn parameter(handle: &scene::Handle, origin: &scene::Vec3) -> Option<f64> {
    let local = [handle.anchor.x - origin.x, handle.anchor.y - origin.y, handle.anchor.z - origin.z];
    match &handle.kind {
        scene::HandleKind::ScalarAlong { axis } => Some(local[0] * axis.x + local[1] * axis.y + local[2] * axis.z),
        scene::HandleKind::FaceOffset { normal } => Some(local[0] * normal.x + local[1] * normal.y + local[2] * normal.z),
        _ => None,
    }
}

type Field = (&'static str, &'static str, Option<f64>);

#[test]
fn every_numeric_geometric_field_of_a_taper_and_a_cutout_has_a_grip_that_writes_it_in_si_units() {
    let document = reinforced();
    let layout = Layout::of(&document, &[]);
    let rows: [(&str, &str, Vec<Field>); 4] = [
        ("tapers", "taper-1", vec![("apexDepthM", "m", Some(0.8)), ("innerRadiusM", "m", None), ("lamellaThicknessM", "m", Some(0.033)), ("apexAngleRad", "rad", None), ("rods", "1", None)]),
        ("tapers", "taper-2", vec![("taperAngleRad", "rad", None)]),
        ("cutouts", "cutout-1", vec![("distanceM", "m", Some(0.1)), ("depthM", "m", Some(0.06)), ("lengthM", "m", Some(0.15)), ("slope", "1", Some(0.5)), ("rods", "1", None)]),
        ("cutouts", "cutout-2", vec![("lengthM", "m", None), ("distanceM", "m", Some(1.0)), ("centreHeightM", "m", Some(0.2))]),
    ];
    let mut covered = 0;
    for (collection, entity, fields) in rows {
        let origin = layout.entity(entity).cloned().expect("origin");
        let grips = handles(&document, &[], &select(entity, collection));
        assert_handle_laws(&grips);
        assert!(grips.iter().all(|h| h.writes.iter().all(|w| !w.path.starts_with("cases["))), "{entity}: no grip writes a case input");
        for (leaf, unit, current) in fields {
            let path = format!("{collection}[id={entity}].{leaf}");
            let handle = grips.iter().find(|h| h.writes.iter().any(|w| w.path == path)).unwrap_or_else(|| panic!("{path} has no grip in {:?}", grips.iter().map(|h| h.id.clone()).collect::<Vec<_>>()));
            let write = handle.writes.iter().find(|w| w.path == path).unwrap();
            assert_eq!(write.unit, unit, "{path}");
            if let (Some(value), Some(s)) = (current, parameter(handle, &origin)) {
                assert!((write.map.apply(s) - value).abs() < 1e-9, "{path}: the anchor writes {} instead of {value}", write.map.apply(s));
            }
            if unit == "1" && leaf == "rods" {
                assert!(matches!(handle.kind, scene::HandleKind::CountStepper), "{path} is a count stepper");
            }
            covered += 1;
        }
    }
    assert_eq!(covered, 14);
}

#[test]
fn the_grip_registry_lists_every_geometric_field_of_the_tapers_and_cutouts_and_the_scene_builds_each() {
    let geometric: BTreeSet<(&str, &str)> = handles::TYPED_FIELD_GRIPS.iter().map(|(collection, leaf, _)| (*collection, *leaf)).collect();
    let strengths = [("tapers", "rodResistanceN"), ("cutouts", "rodResistanceN")];
    let model_fields: BTreeSet<(&str, &str)> = [("tapers", "taperAngleRad"), ("tapers", "apexDepthM"), ("tapers", "innerRadiusM"), ("tapers", "lamellaThicknessM"), ("tapers", "apexAngleRad"), ("tapers", "rods"), ("cutouts", "distanceM"), ("cutouts", "lengthM"), ("cutouts", "depthM"), ("cutouts", "centreHeightM"), ("cutouts", "slope"), ("cutouts", "rods")].into_iter().collect();
    assert_eq!(geometric, model_fields, "the registry covers the numeric geometry of Taper and Cutout; {strengths:?} are strengths");
    let document = reinforced();
    for (collection, leaf, tags) in handles::TYPED_FIELD_GRIPS {
        let ids: Vec<&str> = if *collection == "tapers" { document.tapers.iter().map(|t| t.id.as_str()).collect() } else { document.cutouts.iter().map(|c| c.id.as_str()).collect() };
        let built = ids.iter().any(|id| {
            let path = format!("{collection}[id={id}].{leaf}");
            handles(&document, &[], &select(id, collection)).iter().any(|h| tags.split('|').any(|tag| h.id == format!("{id}:{tag}")) && h.writes.iter().any(|w| w.path == path))
        });
        assert!(built, "{collection}.{leaf} has no {tags} grip");
    }
}

#[test]
fn the_notch_void_is_as_long_as_the_notch_length_and_follows_it() {
    let mut document = reinforced();
    let void_length = |document: &En1995Snapshot| {
        let (scene, _) = scene_of(document);
        scene.entities.into_iter().find(|e| e.entity.id == "cutout-1:notch").and_then(|e| match e.geometry {
            scene::Geometry::Solid(solid) => match solid.shape {
                scene::SolidShape::Box { size, .. } => Some(size.x),
                _ => None,
            },
            _ => None,
        })
    };
    assert!((void_length(&document).expect("notch") - 0.15).abs() < 1e-9);
    document.cutouts[0].length_m = 0.3;
    assert!((void_length(&document).expect("notch") - 0.3).abs() < 1e-9, "dragging the notch length lengthens the void");
}

fn polyline(diagram: &scene::Diagram2d, id: &str) -> Vec<scene::Vec2> {
    diagram.series.iter().find_map(|s| match s {
        scene::Series::Polyline { id: i, points, .. } if i == id => Some(points.clone()),
        _ => None,
    }).unwrap_or_else(|| panic!("{} has no series {id}", diagram.id))
}

fn utilisation(report: &CheckReport, entity: &str, prefix: &str) -> f64 {
    report.checks.iter().filter(|c| c.subject.entity_id == entity && c.id.starts_with(prefix)).map(|c| c.utilization).fold(f64::NAN, f64::max)
}

#[test]
fn the_entity_curves_follow_the_checks_and_their_markers_drag_the_entity_fields() {
    let document = reinforced();
    let report = report_of(&document);
    let all = diagrams(&document, &report, None);
    let by = |id: &str| all.iter().find(|d| d.id == id).unwrap_or_else(|| panic!("{id} in {:?}", all.iter().map(|d| d.id.clone()).collect::<Vec<_>>()));
    let ids = ["case-taper-length:taper-2", "case-apex:taper-1", "case-apex-tension:taper-1", "case-apex-reinforcement:taper-1", "case-notch-depth:cutout-1", "case-notch-reinforcement:cutout-1", "case-notch-rods:cutout-1", "case-hole-net:cutout-2"];
    for id in ids {
        let d = by(id);
        let n = d.table_twin.columns[0].values.len();
        assert!(!d.markers.is_empty() && d.table_twin.columns.len() >= 2 && n >= 2 && d.table_twin.columns.iter().all(|c| c.values.len() == n), "{id}: markers and a rectangular table twin");
        assert!(d.markers.iter().all(|m| m.binding.as_ref().map_or(m.id == "marker-governing", |b| !b.path.starts_with("cases["))), "{id}: markers write entity fields");
        assert!(!d.title.en.is_empty() && !d.title.de.is_empty() && d.title.en != d.title.de, "{id}: EN and DE titles");
    }
    let write = |id: &str, path: &str, at: f64, expected: f64| {
        let binding = by(id).markers.iter().find_map(|m| m.binding.as_ref().filter(|b| b.path == path)).unwrap_or_else(|| panic!("{id} has no marker on {path}"));
        assert!((binding.map.apply(at) - expected).abs() < 1e-9, "{id}: {at} writes {} instead of {expected}", binding.map.apply(at));
    };
    write("case-taper-length:taper-2", "members[id=beam-B1].spanM", document.members[0].span_m * 1e3, document.members[0].span_m);
    write("case-notch-depth:cutout-1", "cutouts[id=cutout-1].depthM", 60.0, 0.06);
    write("case-notch-reinforcement:cutout-1", "cutouts[id=cutout-1].depthM", 60.0, 0.06);
    write("case-notch-rods:cutout-1", "cutouts[id=cutout-1].rods", 3.0, 3.0);
    write("case-hole-net:cutout-2", "cutouts[id=cutout-2].lengthM", 50.0, 0.05);
    write("case-apex:taper-1", "tapers[id=taper-1].innerRadiusM", 15_000.0, 15.0);
    write("case-apex-reinforcement:taper-1", "tapers[id=taper-1].rods", 4.0, 4.0);

    let taper = by("case-taper-length:taper-2");
    let (edge, straight) = (polyline(taper, "tapered-edge"), polyline(taper, "parallel-edge"));
    let u0 = utilisation(&report, "taper-2", "en1995.t-30.tapered_edge");
    let top = edge.iter().map(|q| q.y).fold(f64::NAN, f64::max);
    let governing = taper.markers.iter().find(|m| m.id == "marker-governing").expect("a governing section marker for a member with a moment line");
    assert!((governing.at.y - u0).abs() < 1e-9 && top <= u0 + 1e-9 && (top - u0).abs() < 5e-3 * u0, "the highest point of the tapered edge is the utilisation of the check");
    let line = document.members[0].span_m * 1e3;
    assert!(governing.at.x > 0.0 && governing.at.x < 0.5 * line, "the deeper end carries the lower stress, so the governing section lies on the shallow side of midspan");
    assert!(edge.iter().position(|q| q.y == top).is_some_and(|i| (edge[i].x - governing.at.x).abs() <= line / (edge.len() - 1) as f64), "the marker stands on the maximum of the curve");
    assert!(u0 <= 0.0 || straight.iter().zip(&edge).all(|(s, e)| s.y <= e.y * (1.0 + 1e-9)), "k_m,α ≤ 1 puts the parallel edge below the tapered edge");

    let notch = by("case-notch-depth:cutout-1");
    let kv = polyline(notch, "k-v");
    assert!((kv[0].y - 1.0).abs() < 1e-12 && kv.iter().all(|p| p.y <= 1.0 + 1e-12) && kv.windows(2).all(|w| w[1].y <= w[0].y + 1e-12), "k_v starts at 1, never exceeds the limit 1 and falls with the depth");
    assert!(polyline(notch, "limit").iter().all(|p| p.y == 1.0));
    assert!((notch.markers[0].at.x - 60.0).abs() < 1e-9, "the marker stands at the notch depth in mm");

    let reinforcement = by("case-notch-reinforcement:cutout-1");
    let ft = polyline(reinforcement, "reinforcement");
    let u46 = utilisation(&report, "cutout-1", "en1995.t-46.");
    assert!(ft.windows(2).all(|w| w[1].y >= w[0].y - 1e-12) && ft[0].y.abs() < 1e-12, "no notch, no tension force; deeper notches pull more");
    assert!((reinforcement.markers[0].at.y - u46).abs() < 1e-12);
    let rods = polyline(by("case-notch-rods:cutout-1"), "reinforcement");
    assert!(rods.windows(2).all(|w| w[1].y < w[0].y) && ((rods[0].y * rods[0].x / 2.0) - u46).abs() < 1e-9, "twice the rods halve the utilisation");

    let hole = by("case-hole-net:cutout-2");
    let net = polyline(hole, "net-section");
    assert!(net.windows(2).all(|w| w[1].y >= w[0].y - 1e-12), "a bigger hole loads the net section more");
    let h_mm = document.members[0].h_m * 1e3;
    assert!(polyline(hole, "hole-limit").iter().all(|p| (p.x - schema::checks::timber::HOLE_LIMIT_RATIO * h_mm).abs() < 1e-9), "the NCI hole limit stands at 0.15 h");
    assert!((hole.markers[0].at.y - utilisation(&report, "cutout-2", "en1995.t-18.net_section").min(3.0)).abs() < 1e-9);

    let apex = by("case-apex:taper-1");
    assert!(polyline(apex, "k-r").iter().all(|p| p.y > 0.0 && p.y <= 1.0 + 1e-12), "0.76 ≤ k_r ≤ 1 where the lamella ratio stays below 240");
    assert!(polyline(apex, "k-l").iter().all(|p| p.y > 0.0));
    assert!(polyline(by("case-apex-tension:taper-1"), "k-p").len() >= 2);
}

#[test]
fn a_bridge_member_draws_the_miner_sum_over_the_design_life() {
    for document in [En1995Snapshot::compliant_bridge(), En1995Snapshot::noncompliant_bridge()] {
        let report = report_of(&document);
        let member = document.members.iter().find(|m| m.bridge_n_obs > 0.0).expect("a bridge member");
        let all = diagrams(&document, &report, Some(&member.id));
        let fatigue = all.iter().find(|d| d.id == format!("fatigue:{}", member.id)).expect("the fatigue diagram");
        let miner = polyline(fatigue, "miner");
        let d = utilisation(&report, &member.id, "en1995.2.a.fatigue.");
        assert!(miner[0].y == 0.0 && miner.windows(2).all(|w| w[1].y > w[0].y), "the Miner sum grows linearly with the life");
        let slope = miner[1].y / miner[1].x;
        assert!(miner.iter().skip(1).all(|p| (p.y / p.x - slope).abs() < 1e-9 * slope.abs().max(1.0)));
        assert!((slope * member.bridge_t_l_years - d).abs() < 1e-9 * d.max(1.0), "D at the design life is the check value");
        let marker = &fatigue.markers[0];
        assert!((marker.at.x - member.bridge_t_l_years).abs() < 1e-12 && marker.binding.as_ref().is_some_and(|b| b.path == format!("members[id={}].bridgeTLYears", member.id)));
        let results = all.iter().find(|d| d.id == format!("member-results:{}", member.id)).expect("the member utilisations");
        let checks = report.checks.iter().filter(|c| c.subject.entity_id == member.id && c.utilization.is_finite() && c.utilization > 0.0).count();
        assert_eq!(results.markers.len(), checks);
        assert!(results.markers.iter().all(|m| m.bound_check.is_some()));
    }
}

#[test]
fn every_check_of_the_spec_trace_is_bound_to_an_entity_collection_and_a_diagram() {
    use norm::spec_trace::Realisation;
    let traced: BTreeSet<&str> = schema::spec_trace::TABLE.items.iter().filter_map(|item| match &item.realisation {
        Realisation::Check(ids) => Some(ids.iter().copied()),
        _ => None,
    }).flatten().collect();
    let bound: BTreeSet<&str> = CHECK_BINDINGS.iter().map(|b| b.check).collect();
    assert_eq!(traced, bound, "the registry and the spec trace list the same checks");
    for binding in CHECK_BINDINGS {
        assert!(ENTITY_KINDS.iter().any(|k| k.collection == binding.collection), "{}: unknown collection {}", binding.check, binding.collection);
        assert!(!binding.diagram.is_empty(), "{}", binding.check);
    }
    let mut burning = En1995Snapshot::compliant_building_beam();
    burning.members[0].fire_duration_s = 1800.0;
    let documents = [reinforced(), En1995Snapshot::compliant_building_beam(), En1995Snapshot::noncompliant_building(), En1995Snapshot::compliant_bridge(), En1995Snapshot::noncompliant_bridge(), burning];
    let evaluated: Vec<_> = documents.iter().map(|d| { let (scene, report) = scene_of(d); (d, scene, report) }).collect();
    let mut exercised = 0;
    for binding in CHECK_BINDINGS {
        let mut drawn = false;
        let mut seen = false;
        for (document, scene, report) in &evaluated {
            for check in report.checks.iter().filter(|c| c.id.starts_with(binding.check)) {
                seen = true;
                let subject = check.subject.entity_id.as_str();
                let collection = entity_kind_of(document, subject).map(|k| k.collection);
                assert!(collection == Some(binding.collection) || (binding.collection == "members" && collection == Some("floors")), "{} is carried by {subject} of {collection:?}", binding.check);
                assert!(scene.entities.iter().any(|e| (e.entity.id == subject || e.entity.id.starts_with(&format!("{subject}:"))) && e.utilisation.is_some()), "{}: {subject} has no utilisation-coloured 3D body", binding.check);
                drawn |= diagrams(document, report, Some(subject)).iter().any(|d| d.id.starts_with(&format!("{}:", binding.diagram)) && d.markers.iter().any(|m| m.bound_check.is_some() || m.binding.is_some()));
            }
        }
        if seen {
            exercised += 1;
            assert!(drawn, "{}: no {} diagram with a bound marker", binding.check, binding.diagram);
        }
    }
    assert!(exercised >= 18, "only {exercised} of {} checks were exercised", CHECK_BINDINGS.len());
}
