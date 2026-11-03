//! 📏️ Serviceability pair checks of the spread foundations from their persisted placements (EN 1997-1 §6.6 and Annex H, DIN 1054:2021): settlement per footing (oedometric and elastic) with the influence of the neighbouring footings superposed, differential settlement, angular distortion of every pair, tilt of the fitted settlement plane and the relative rotation of every pair against that plane.

use super::*;
use crate::catalogue::{evaluate_outcomes, settlement as sl, Case};
use semio_s_artifact_norm_en1997_model::Placement;
use semio_s_plugin_norm_compliance::ndp_number;

/// 📐️ Characteristic serviceability pressure of a footing: the largest `(G + Q) / A` of its load cases.
fn service_pressure(footing: &SpreadFoundation) -> f64 {
    let area = (footing.width * footing.length).max(1e-9);
    footing.load_cases.iter().map(|lc| (lc.vertical_permanent + lc.vertical_variable) / area).fold(0.0, f64::max)
}

/// 📐️ Settlements `(oedometric, elastic)` of an isolated footing at the characteristic serviceability pressure of its governing load case.
pub fn footing_settlements(doc: &En1997Snapshot, footing: &SpreadFoundation, gwl: f64) -> (f64, f64) {
    let q = service_pressure(footing);
    let oedometric = part_1::settlement_oedometric_with_gwl(&doc.layers, footing.width, footing.length, footing.embedment, q, gwl).0;
    let elastic = part_1::elastic_settlement_layered(&doc.layers, footing.width, footing.length, footing.embedment, q).0;
    (oedometric, elastic)
}

/// 🤝 Settlements `(oedometric, elastic)` of every placed footing with the influence of all its neighbours superposed, as the scene dish does: the isolated settlement plus, for every other footing, the stress-strain integral (oedometric) or Love's half-space displacement (elastic, equivalent ν and E_m of the receiving footing) of its pressure at the centre of the receiving footing, in the loaded footing's frame (placement yaw).
pub fn settlements_with_interaction(doc: &En1997Snapshot, placed: &[(&SpreadFoundation, &Placement)], gwl: f64) -> Vec<(f64, f64)> {
    placed
        .iter()
        .enumerate()
        .map(|(i, (receiver, at))| {
            let (mut oedometric, mut elastic) = footing_settlements(doc, receiver, gwl);
            let equivalent = part_1::elastic_equivalent(&doc.layers, receiver.width, receiver.embedment);
            for (_, (loaded, from)) in placed.iter().enumerate().filter(|(j, _)| *j != i) {
                let (dx, dy) = (at.x - from.x, at.y - from.y);
                let (s, c) = from.yaw.sin_cos();
                let (px, py) = (dx * c + dy * s, -dx * s + dy * c);
                let q = service_pressure(loaded);
                oedometric += sl::stress_strain_settlement_at(&doc.layers, loaded.width, loaded.length, loaded.embedment, q, px, py);
                if let Some((nu, e_m, _)) = &equivalent {
                    elastic += sl::displacement_rect_at(q, loaded.width, loaded.length, px, py, *nu, *e_m);
                }
            }
            (oedometric, elastic)
        })
        .collect()
}

/// 📐️ Angular distortion of a pair from the catalogue entry SLS-04 (`tan ω = |s_a − s_b| / L`) against the limit `1/limit_den`.
fn pair_distortion(annex: AnnexChoice, s_a: f64, s_b: f64, length: f64, limit: f64) -> f64 {
    let case = Case::new("pair", annex).with_n("s_a", s_a).with_n("s_b", s_b).with_n("l", length).with_n("limit_den", 1.0 / limit);
    evaluate_outcomes("SLS-04", &case).ok().and_then(|outcomes| outcomes.into_iter().find(|o| o.key == "tan_omega")).map_or(f64::INFINITY, |o| o.value())
}

/// 📐️ Least-squares plane `s = a + b·x + c·y` through settlement points; `None` for fewer than three points or collinear positions.
pub fn settlement_plane(points: &[(f64, f64, f64)]) -> Option<(f64, f64, f64)> {
    let n = points.len() as f64;
    if points.len() < 3 {
        return None;
    }
    let (mx, my, ms) = (points.iter().map(|p| p.0).sum::<f64>() / n, points.iter().map(|p| p.1).sum::<f64>() / n, points.iter().map(|p| p.2).sum::<f64>() / n);
    let (mut sxx, mut sxy, mut syy, mut sxs, mut sys) = (0.0, 0.0, 0.0, 0.0, 0.0);
    for (x, y, s) in points {
        let (dx, dy, ds) = (x - mx, y - my, s - ms);
        sxx += dx * dx;
        sxy += dx * dy;
        syy += dy * dy;
        sxs += dx * ds;
        sys += dy * ds;
    }
    let det = sxx * syy - sxy * sxy;
    if det.abs() <= 1e-12 * (sxx * syy).abs().max(1e-30) {
        return None;
    }
    let (b, c) = ((sxs * syy - sys * sxy) / det, (sys * sxx - sxs * sxy) / det);
    Some((ms - b * mx - c * my, b, c))
}

fn ratio_result(id: String, title: (&str, &str), clause: &str, subject: SubjectRef, annex: AnnexChoice, value: f64, limit: f64, explanation: (String, String), remedy: (&str, &str)) -> CheckResult {
    let mut check = CheckResult::assess(id, "EN 1997-1", ClauseId::new("EN 1997-1", "6", clause), subject.clone(), loc(title.0, title.1))
        .annex(annex)
        .utilization(Quantity::new(QuantityKind::Dimensionless, value), Quantity::new(QuantityKind::Dimensionless, limit))
        .explanation(loc(&explanation.0, &explanation.1));
    if value > limit {
        check = check.remedy(Remedy::at_most(subject, Quantity::new(QuantityKind::Dimensionless, value), Quantity::new(QuantityKind::Dimensionless, limit), loc(remedy.0, remedy.1)));
    }
    check.ndps(&["ndp.en1997.sls.angular-distortion-limit", "ndp.en1997.sls.tilt-limit"]).build()
}

pub fn check_sls_pairs(doc: &En1997Snapshot, report: &mut CheckReport, annex: AnnexChoice, gwl: f64, scope: Scope<'_>) {
    if !scope.part("sls-pairs") {
        return;
    }
    let placed: Vec<(&SpreadFoundation, &Placement)> = doc
        .footings
        .iter()
        .filter(|f| f.width > 1e-6 && f.length > 1e-6)
        .filter_map(|f| doc.placements.iter().find(|p| p.id == f.id).map(|p| (f, p)))
        .collect();
    if placed.len() < 2 {
        report.push(
            CheckResult::assess("en1997.6.6.pairs.na", "EN 1997-1", ClauseId::new("EN 1997-1", "6", "6.6"), SubjectRef::whole(loc("Project", "Projekt")), loc("Differential settlement and tilt", "Setzungsdifferenz und Schiefstellung"))
                .not_applicable(loc("Fewer than two footings with a persisted placement.", "Weniger als zwei Fundamente mit gespeicherter Platzierung."))
                .annex(annex)
                .build(),
        );
        return;
    }
    let settlements = settlements_with_interaction(doc, &placed, gwl);
    let (beta_limit, tilt_limit) = (ndp_number("ndp.en1997.sls.angular-distortion-limit", annex), ndp_number("ndp.en1997.sls.tilt-limit", annex));
    for (method, pick, en, de) in [("oed", 0usize, "oedometric", "ödometrisch"), ("el", 1usize, "elastic", "elastisch")] {
        let value = |i: usize| if pick == 0 { settlements[i].0 } else { settlements[i].1 };
        let points: Vec<(f64, f64, f64)> = placed.iter().enumerate().map(|(i, (_, p))| (p.x, p.y, value(i))).collect();
        let plane = settlement_plane(&points);
        for i in 0..placed.len() {
            for j in (i + 1)..placed.len() {
                let (a, b) = (placed[i], placed[j]);
                let (dx, dy) = (b.1.x - a.1.x, b.1.y - a.1.y);
                let length = dx.hypot(dy);
                if length < 1e-6 {
                    continue;
                }
                let delta = value(j) - value(i);
                let subject = SubjectRef::new(&a.0.id, format!("footings[id={}]", a.0.id), loc(&format!("Footing {}", a.0.id), &format!("Fundament {}", a.0.id)));
                let beta = pair_distortion(annex, value(i), value(j), length, beta_limit);
                report.push(ratio_result(
                    format!("en1997.6.6.angular.{method}.{}.{}", a.0.id, b.0.id),
                    (&format!("Angular distortion {} – {} ({en})", a.0.id, b.0.id), &format!("Winkelverdrehung {} – {} ({de})", a.0.id, b.0.id)),
                    "6.6.2",
                    subject.clone(),
                    annex,
                    beta,
                    beta_limit,
                    (format!("Δs = {:.1} mm over L = {:.2} m gives β = 1/{:.0}.", delta.abs() * 1000.0, length, 1.0 / beta.max(1e-12)), format!("Δs = {:.1} mm auf L = {:.2} m ergibt β = 1/{:.0}.", delta.abs() * 1000.0, length, 1.0 / beta.max(1e-12))),
                    ("Widen or found deeper the footing with the larger settlement, or place the footings closer.", "Das Fundament mit der größeren Setzung verbreitern oder tiefer gründen, oder die Fundamente näher zusammenrücken."),
                ));
                if let Some((_, gx, gy)) = plane {
                    let relative = (delta / length - (gx * dx + gy * dy) / length).abs();
                    report.push(ratio_result(
                        format!("en1997.6.6.rotation.{method}.{}.{}", a.0.id, b.0.id),
                        (&format!("Relative rotation {} – {} ({en})", a.0.id, b.0.id), &format!("Relative Verdrehung {} – {} ({de})", a.0.id, b.0.id)),
                        "6.6.2",
                        subject,
                        annex,
                        relative,
                        beta_limit,
                        (format!("Pair slope minus fitted tilt component: {:.5}.", relative), format!("Paarneigung abzüglich Neigungskomponente der Ausgleichsebene: {:.5}.", relative)),
                        ("Reduce the settlement scatter between the two footings.", "Die Setzungsstreuung zwischen den beiden Fundamenten verringern."),
                    ));
                }
            }
        }
        let tilt = match plane {
            Some((_, gx, gy)) => gx.hypot(gy),
            None => {
                let (a, b) = (placed[0], placed[1]);
                (value(1) - value(0)).abs() / (b.1.x - a.1.x).hypot(b.1.y - a.1.y).max(1e-9)
            }
        };
        let subject = SubjectRef::new(&placed[0].0.id, "footings", loc("Footings", "Fundamente"));
        report.push(ratio_result(
            format!("en1997.6.6.tilt.{method}"),
            (&format!("Tilt of the settlement plane ({en})", ), &format!("Schiefstellung der Setzungsebene ({de})")),
            "6.6.2",
            subject,
            annex,
            tilt,
            tilt_limit,
            (format!("Gradient of the fitted settlement plane {:.5} (1/{:.0}).", tilt, 1.0 / tilt.max(1e-12)), format!("Gradient der Ausgleichsebene {:.5} (1/{:.0}).", tilt, 1.0 / tilt.max(1e-12))),
            ("Equalise the footing pressures or stiffen the soft side of the structure.", "Die Fundamentpressungen angleichen oder die weiche Seite des Bauwerks aussteifen."),
        ));
    }
}
