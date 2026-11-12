//! 🥞️ Diaphragm: the floor diaphragm of each plan direction as a plate on top of the first bracing wall, deformed by its in-plane deflection (magnified, with the undeformed outline) and coloured by the classification check; the plan frame is the frame of that wall, the profile comes from the shear-beam solution of [`masonry::diaphragms`] (EN 1998-1 4.3.1(4), `en1996.9.3.diaphragm.<axis>`).
//!
//! 🕹️ The grip above the plate scales `diaphragmStiffnessNPerM` of every wall that declares one on a logarithmic law (one commit per drag), and two diagrams per direction show the wall force distribution of the rigid and the flexible assumption side by side and the classification ratio over the stiffness against the limit.

use super::scene::projection_kit::{loc};
use super::kit::*;
use super::layout::{Layout, WallPose, EPS};
use super::model::{En1996Snapshot, MasonryWall};
use super::scene::projection_kit::handle_kit::{handle, field_binding, HandleSpec as Spec};
use super::scene::*;
use super::{masonry as m, SceneInput};

const SLAB: f64 = 0.12;
const SHOWN: f64 = 0.12;
const STRIPS: usize = 40;
const OUTLINE: f64 = 0.015;
const LIFT: f64 = 0.35;
const DECADES_PER_METRE: f64 = 0.5;
const STEPS_PER_DECADE: f64 = 4.0;
const DECADE_RANGE: (f64, f64) = (3.0, 12.0);

fn profile_at(profile: &[(f64, f64)], s: f64) -> f64 {
    let i = profile.partition_point(|p| p.0 <= s).clamp(1, profile.len() - 1);
    let ((s_a, w_a), (s_b, w_b)) = (profile[i - 1], profile[i]);
    w_a + (w_b - w_a) * ((s - s_a) / (s_b - s_a).max(EPS)).clamp(0.0, 1.0)
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for d in m::diaphragms(input.document.annex, &input.document.walls) {
        let Some(first) = d.walls.first() else { continue };
        let Some(wall) = input.document.walls.iter().find(|w| w.id == first.wall_id) else { continue };
        let id = format!("{}:{DIAPHRAGM_TAG}{}", wall.id, d.axis.key());
        if !out.wants(&id) {
            continue;
        }
        let Some(pose) = layout.pose(&wall.id) else { continue };
        let (s0, s1) = d.span_m;
        let max_w = d.profile.iter().map(|p| p.1).fold(0.0, f64::max);
        if s1 - s0 <= EPS || max_w <= 0.0 || wall.height_m <= EPS {
            continue;
        }
        let magnify = SHOWN * (s1 - s0) / max_w;
        let (z0, z1) = (wall.height_m, wall.height_m + SLAB);
        let (a0, a1) = (0.0, d.depth_m);
        let at = |s: f64| profile_at(&d.profile, s);
        let mut mesh = MeshBuilder::default();
        for i in 0..STRIPS {
            let (lo, hi) = (s0 + (s1 - s0) * i as f64 / STRIPS as f64, s0 + (s1 - s0) * (i + 1) as f64 / STRIPS as f64);
            let shift = magnify * at(0.5 * (lo + hi));
            match d.axis {
                m::PlanAxis::X => mesh.cuboid(pose, (a0 + shift, a1 + shift), (lo - s0, hi - s0), (z0, z1)),
                m::PlanAxis::Y => mesh.cuboid(pose, (lo - s0, hi - s0), (a0 + shift, a1 + shift), (z0, z1)),
            }
        }
        let corner = |a: f64, s: f64| match d.axis {
            m::PlanAxis::X => [a, s - s0, z1 + OUTLINE],
            m::PlanAxis::Y => [s - s0, a, z1 + OUTLINE],
        };
        for (p, q) in [((a0, s0), (a1, s0)), ((a1, s0), (a1, s1)), ((a1, s1), (a0, s1)), ((a0, s1), (a0, s0))] {
            mesh.rod(pose, corner(p.0, p.1), corner(q.0, q.1), 0.5 * OUTLINE);
        }
        let points = mesh.points().to_vec();
        let class = match d.class {
            m::DiaphragmClass::Rigid => ("rigid", "starr"),
            m::DiaphragmClass::Flexible => ("flexible", "verformbar"),
        };
        let name = loc(&format!("Floor diaphragm, direction {}", d.axis.key()), &format!("Deckenscheibe, Richtung {}", d.axis.key()));
        let mut spec = Spec::new(EntityKind::Plate, id, mesh.solid(), MaterialRole::Concrete, LAYER_FLOORS, name);
        spec.detail = loc(
            &format!("Classified {} (largest displacement {:.2} times the rigid assumption of {:.3} mm, diaphragm deflection {:.3} mm against the average wall drift {:.3} mm); deflection drawn magnified {magnify:.0} times over the undeformed outline", class.0, d.ratio, d.rigid_drift_m * 1e3, d.deflection_m * 1e3, d.mean_drift_m * 1e3),
            &format!("Eingestuft {} (größte Verschiebung {:.2}-fach der starren Annahme von {:.3} mm, Scheibenverformung {:.3} mm gegenüber der mittleren Wandverschiebung {:.3} mm); Verformung {magnify:.0}-fach überhöht über dem unverformten Umriss", class.1, d.ratio, d.rigid_drift_m * 1e3, d.deflection_m * 1e3, d.mean_drift_m * 1e3),
        );
        spec.binding = binding_of(input.report, &format!("en1996.9.3.diaphragm.{}", d.axis.key()));
        spec.channel = Some(ColourChannel::Utilisation);
        spec.opacity = 0.85;
        out.push(spec, &points);
    }
}

/// 🕹️ The stiffness grip of the diaphragm entity `<wall>:diaphragm-<axis>`: the drag parameter is the height above the wall base, every declared stiffness follows `10^(log10(current) + 0.5 per metre · Δs)` rounded to four steps per decade, all in one commit.
pub(crate) fn handles(document: &En1996Snapshot, wall: &MasonryWall, pose: &WallPose, tag: &str) -> Vec<Handle> {
    let walls = diaphragm_walls(document);
    let Some(d) = m::diaphragms(document.annex, &document.walls).into_iter().find(|d| format!("{DIAPHRAGM_TAG}{}", d.axis.key()) == tag) else { return Vec::new() };
    if walls.is_empty() {
        return Vec::new();
    }
    let anchor = pose.at(0.5 * wall.length_m, 0.5 * wall.thickness_m, wall.height_m + SLAB + LIFT);
    let z0 = anchor.z;
    let exponents: Vec<f64> = walls.iter().map(|w| w.diaphragm_stiffness_n_per_m.log10()).collect();
    let low = exponents.iter().copied().fold(f64::INFINITY, f64::min);
    let high = exponents.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let limits = (z0 + (DECADE_RANGE.0 - low) / DECADES_PER_METRE, z0 + (DECADE_RANGE.1 - high) / DECADES_PER_METRE);
    let entity = EntityRef::new(EntityKind::Plate, format!("{}:{tag}", wall.id), format!("walls[id={}]", wall.id));
    let writes = walls
        .iter()
        .zip(&exponents)
        .map(|(w, e)| field_binding(format!("walls[id={}].diaphragmStiffnessNPerM", w.id), Affine { base: e - DECADES_PER_METRE * z0, k: DECADES_PER_METRE }, "N/m", Some(Quantise::Decades { per_decade: STEPS_PER_DECADE })))
        .collect();
    let axis_key = d.axis.key();
    vec![handle(Spec {
        entity: &entity,
        tag: "stiffness",
        kind: HandleKind::ScalarAlong { axis: Vec3::Z },
        anchor,
        grip: GripStyle::Arrow,
        writes,
        snap: SnapRule::NONE,
        limits: (limits.0, limits.1.max(limits.0 + 0.1)),
        name: loc(&format!("Diaphragm stiffness G·t, direction {axis_key} (logarithmic drag)"), &format!("Scheibensteifigkeit G·t, Richtung {axis_key} (logarithmisches Ziehen)")),
    })]
}

#[cfg(test)]
mod tests {
    use super::*;
    use diagrams::diagrams;

    #[test]
    fn the_profile_is_interpolated_between_its_nodes() {
        let profile = [(0.0, 1.0), (1.0, 3.0), (2.0, 3.0)];
        assert!((profile_at(&profile, 0.5) - 2.0).abs() < 1e-12 && (profile_at(&profile, 1.5) - 3.0).abs() < 1e-12 && (profile_at(&profile, -1.0) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn the_plate_of_a_flexible_diaphragm_bulges_between_its_walls() {
        let mut doc = model::En1996Snapshot::compliant_clay_wall();
        let mut second = doc.walls[0].clone();
        second.id = "wall-b".into();
        for (wall, y) in doc.walls.iter_mut().chain(std::iter::once(&mut second)).zip([0.0, 6.0]) {
            wall.plan_y_m = y;
            wall.diaphragm_stiffness_n_per_m = 1.0e5;
        }
        doc.walls.push(second);
        let d = &m::diaphragms(doc.annex, &doc.walls)[0];
        assert!(d.profile[d.profile.len() / 2].1 > d.profile[0].1, "midspan deflects more than the wall line");
    }

    fn two_walls(g_t: [f64; 2]) -> model::En1996Snapshot {
        let mut doc = model::En1996Snapshot::compliant_clay_wall();
        let mut second = doc.walls[0].clone();
        second.id = "wall-b".into();
        doc.walls[0].plan_y_m = 0.0;
        second.plan_y_m = 6.0;
        doc.walls[0].diaphragm_stiffness_n_per_m = g_t[0];
        second.diaphragm_stiffness_n_per_m = g_t[1];
        doc.walls.push(second);
        doc
    }

    fn grip(doc: &model::En1996Snapshot) -> Handle {
        let entity = EntityRef::new(EntityKind::Plate, format!("{}:diaphragm-x", doc.walls[0].id), format!("walls[id={}]", doc.walls[0].id));
        super::super::handles(doc, &[], &[entity]).remove(0)
    }

    #[test]
    fn the_stiffness_grip_scales_every_declared_wall_on_a_logarithmic_law_in_one_commit() {
        let doc = two_walls([1.0e5, 4.0e6]);
        let handle = grip(&doc);
        let HandleKind::ScalarAlong { axis } = &handle.kind else { panic!("scalar grip") };
        let here = handle.anchor.dot(axis);
        let at = |s: f64| handle.writes.iter().map(|w| w.value_at(s)).collect::<Vec<_>>();
        let now = at(here);
        assert!((now[0] / 1.0e5 - 1.0).abs() < 0.34 && (now[1] / 4.0e6 - 1.0).abs() < 0.34, "{now:?}");
        let up = at(here + 2.0);
        assert!((up[0] / now[0] - 10.0).abs() < 1e-6 * 10.0 && (up[1] / now[1] - 10.0).abs() < 1e-6 * 10.0, "{up:?} against {now:?}");
        assert_eq!(handle.writes.len(), 2);
        assert!(handle.limits.as_ref().is_some_and(|l| l.min < here && here < l.max));
    }

    #[test]
    fn the_distribution_and_classification_diagrams_show_both_rules_and_the_limit() {
        let doc = two_walls([1.0e5, 1.0e5]);
        let report = norm::CheckReport::default();
        let set = diagrams(&doc, &report, None);
        assert_eq!(set.iter().map(|d| d.id.as_str()).collect::<Vec<_>>(), ["diaphragm:x", "diaphragm-ratio:x"]);
        let ratio = &set[1];
        assert!(ratio.series.iter().any(|s| matches!(s, Series::Limit { value, .. } if (*value - m::RIGID_RATIO).abs() < 1e-12)));
        let Some(Marker { binding: Some(binding), .. }) = ratio.markers.first() else { panic!("stiffness marker") };
        assert_eq!(binding.path, format!("walls[id={}].diaphragmStiffnessNPerM", doc.walls[0].id));
        assert_eq!(set[0].series.len(), 4);
        assert!(diagrams(&doc, &report, Some("missing")).is_empty());
    }
}
