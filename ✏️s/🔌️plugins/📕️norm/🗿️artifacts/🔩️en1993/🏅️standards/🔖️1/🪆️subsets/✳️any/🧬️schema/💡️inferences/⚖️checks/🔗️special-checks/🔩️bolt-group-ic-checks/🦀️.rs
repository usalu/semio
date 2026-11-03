//! 🔩️ Bolt groups of the joint plates by the instantaneous centre of rotation: the plastic distribution of EN 1993-1-8 3.12 with the load-deformation law of one bolt, compared with the elastic vector method. See [EN 1993-1-8 3.12](https://eurocodes.jrc.ec.europa.eu/).

use super::*;
use crate::clauses::bolt_group_ic as ic;
use semio_s_artifact_norm_en1993_model::JointPlate;

/// 🔩️ Bolt forces of one plate at the applied load (N, mm): the elastic vectors always, the instantaneous-centre vectors scaled to the applied load when the plate asks for that method and its equilibrium converges.
#[derive(Clone, Debug, PartialEq)]
pub struct IcView {
    pub points: Vec<(f64, f64)>,
    pub elastic: Vec<(f64, f64)>,
    pub plastic: Option<Vec<(f64, f64)>>,
    pub centre: Option<(f64, f64)>,
    pub factor: Option<f64>,
    pub r_ult: f64,
}

/// 🔩️ The bolt force view of `p` with `gamma_m2`, `None` while the plate has no load or no valid bolts.
pub fn view(document: &En1993Snapshot, p: &JointPlate, gamma_m2: f64) -> Option<IcView> {
    let joint = document.joints.iter().find(|j| j.id == p.joint_id)?;
    let (fx, fy, mz) = super::plate_checks::plate_load(document, p, joint)?;
    if p.bolt_rows == 0 || p.bolts_per_row == 0 || p.bolt_diameter <= 0.0 || (fx == 0.0 && fy == 0.0) {
        return None;
    }
    let points = super::plate_checks::bolt_points(p);
    let (fv, fb) = super::plate_checks::bolt_resistance(p, gamma_m2);
    let r_ult = fv.min(fb);
    let elastic = ic::elastic_vectors(&points, fx, fy, mz);
    let solved = (p.group_method == semio_s_artifact_norm_en1993_model::BoltGroupMethod::InstantaneousCentre).then(|| ic::solve(&ic::law(), &points, r_ult, fx, fy, mz)).flatten();
    let plastic = solved.as_ref().map(|s| s.forces.iter().map(|g| (g.0 / s.factor, g.1 / s.factor)).collect());
    Some(IcView { points, elastic, plastic, centre: solved.as_ref().and_then(|s| s.centre), factor: solved.as_ref().map(|s| s.factor), r_ult })
}

fn eccentricity_limit(p: &JointPlate, points: &[(f64, f64)], r_ult: f64, fx: f64, fy: f64, mz: f64) -> Option<f64> {
    if p.eccentricity <= 0.0 || mz == 0.0 {
        return None;
    }
    let law = ic::law();
    let factor = |scale: f64| ic::solve(&law, points, r_ult, fx, fy, mz * scale).map(|s| s.factor);
    if factor(0.0)? < 1.0 {
        return None;
    }
    let (mut lo, mut hi) = (0.0, 1.0);
    for _ in 0..40 {
        let mid = 0.5 * (lo + hi);
        if factor(mid).is_some_and(|k| k >= 1.0) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Some(p.eccentricity * lo)
}

/// 🔩️ The instantaneous-centre check of a plate that asks for that method, `None` when the equilibrium does not converge (the elastic check then stands in).
pub fn ic_case(document: &En1993Snapshot, p: &JointPlate, annex: AnnexChoice, params: AnnexParams) -> Option<CheckResult> {
    let joint = document.joints.iter().find(|j| j.id == p.joint_id)?;
    let (fx, fy, mz) = super::plate_checks::plate_load(document, p, joint)?;
    let points = super::plate_checks::bolt_points(p);
    let (fv, fb) = super::plate_checks::bolt_resistance(p, params.gamma_m2);
    let r_ult = fv.min(fb);
    let law = ic::law();
    let solution = ic::solve(&law, &points, r_ult, fx, fy, mz)?;
    let elastic = ic::elastic_max(&points, fx, fy, mz);
    let target = SubjectRef::new(&p.id, format!("jointPlates[id={}].groupMethod", p.id), loc("Bolt group method", "Verfahren der Schraubengruppe"));
    let centre_en = solution.centre.map_or("at infinity (load through the centroid)".to_owned(), |c| format!("at ({:.1}, {:.1}) mm from the centroid", c.0, c.1));
    let centre_de = solution.centre.map_or("im Unendlichen (Last im Schwerpunkt)".to_owned(), |c| format!("bei ({:.1}, {:.1}) mm vom Schwerpunkt", c.0, c.1));
    let elastic_factor = if elastic > 0.0 { r_ult / elastic } else { f64::INFINITY };
    let mut builder = CheckResult::assess(format!("en1993.1-8.3.12.ic.{}", p.id), "DIN EN 1993-1-8", ClauseId::new("EN 1993-1-8", "3", "3.12"), target, loc("Plate bolt group: instantaneous centre of rotation", "Schraubengruppe des Blechs: Momentanpol"))
        .annex(annex)
        .ndp(AnnexParams::GAMMA_M2)
        .minimum(dimensionless(solution.factor), dimensionless(1.0))
        .explanation(loc(
            &format!("{} bolts, R_ult=min(F_v,Rd, F_b,Rd)={:.1} kN, centre {centre_en}: the group carries {:.2} times the applied load (the elastic vector method {:.2} times, F_b,Ed,max={:.1} kN). Load-deformation law R = R_ult (1 - e^(-{:.1} δ))^{:.2}, δ_max={:.2} mm.", points.len(), r_ult / 1e3, solution.factor, elastic_factor, elastic / 1e3, law.mu, law.lambda, law.delta_max),
            &format!("{} Schrauben, R_ult=min(F_v,Rd, F_b,Rd)={:.1} kN, Momentanpol {centre_de}: die Gruppe trägt das {:.2}-fache der Last (das elastische Vektorverfahren das {:.2}-fache, F_b,Ed,max={:.1} kN). Last-Verformungs-Gesetz R = R_ult (1 - e^(-{:.1} δ))^{:.2}, δ_max={:.2} mm.", points.len(), r_ult / 1e3, solution.factor, elastic_factor, elastic / 1e3, law.mu, law.lambda, law.delta_max),
        ));
    if solution.factor < 1.0 {
        match eccentricity_limit(p, &points, r_ult, fx, fy, mz) {
            Some(e) => {
                let limit = e * 0.999;
                builder = builder.remedy(Remedy::at_most(
                    SubjectRef::new(&p.id, format!("jointPlates[id={}].eccentricity", p.id), loc("Eccentricity", "Ausmitte")),
                    length_m(p.eccentricity),
                    length_m(limit),
                    loc(&format!("Reduce the eccentricity of the bolt line to at most {:.0} mm.", limit * 1e3), &format!("Ausmitte der Schraubenlinie auf höchstens {:.0} mm verringern.", limit * 1e3)),
                ));
            }
            None => {
                builder = builder.remedy(Remedy::advisory(
                    SubjectRef::new(&p.id, format!("jointPlates[id={}].boltRows", p.id), loc("Bolt rows", "Schraubenreihen")),
                    loc("Add bolt rows or choose a larger bolt diameter or plate thickness: the resistance is exhausted even without eccentricity.", "Schraubenreihen ergänzen oder einen größeren Schraubendurchmesser bzw. eine größere Blechdicke wählen: der Widerstand ist selbst ohne Ausmitte erschöpft."),
                ));
            }
        }
    }
    Some(builder.build())
}
