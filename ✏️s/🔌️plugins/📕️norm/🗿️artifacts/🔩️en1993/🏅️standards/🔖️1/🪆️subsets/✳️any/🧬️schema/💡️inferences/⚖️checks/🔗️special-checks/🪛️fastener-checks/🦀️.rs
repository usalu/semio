//! 🪛️ Screws and blind rivets of cold-formed sheets: bearing and fastener shear, pull-over and pull-out of screws in tension, their interaction, the net section of the sheet and the minimum spacing (EN 1993-1-3 8.3). See [EN 1993-1-3](https://eurocodes.jrc.ec.europa.eu/).

use super::*;
use crate::clauses::fasteners as fs;
use semio_s_artifact_norm_en1993_model::{FastenedJoint, FastenerKind};

fn kind(f: &FastenedJoint) -> fs::Kind {
    match f.kind {
        FastenerKind::SelfTappingScrew => fs::Kind::Screw,
        FastenerKind::BlindRivet => fs::Kind::Rivet,
    }
}

fn subject(f: &FastenedJoint, field: &str, en: &str, de: &str) -> SubjectRef {
    SubjectRef::new(&f.id, format!("fastenedJoints[id={}].{field}", f.id), loc(en, de))
}

fn valid(f: &FastenedJoint) -> bool {
    f.diameter > 0.0 && f.count > 0 && f.head_thickness > 0.0 && f.base_thickness > 0.0 && f.head_strength > 0.0 && f.base_strength > 0.0
}

/// 🪛️ Resistances (N, per fastener unless noted) of one fastened joint in SI units.
#[derive(Clone, Debug, PartialEq)]
pub struct Resistance {
    pub bearing: f64,
    pub shear: f64,
    pub pull_over: Option<f64>,
    pub pull_out: Option<f64>,
    pub net: f64,
}

impl Resistance {
    pub fn tension(&self) -> Option<f64> {
        self.pull_over.zip(self.pull_out).map(|(a, b)| a.min(b))
    }
}

/// 🪛️ The resistances of `f` with `gamma_m2`, the thinner sheet bearing against the other as support.
pub fn resistance(f: &FastenedJoint, gamma_m2: f64) -> Option<Resistance> {
    if !valid(f) {
        return None;
    }
    let (t1, fu1, t2) = if f.head_thickness <= f.base_thickness { (f.head_thickness, f.head_strength, f.base_thickness) } else { (f.base_thickness, f.base_strength, f.head_thickness) };
    let bearing = fs::bearing(kind(f), t1, t2, f.diameter, fu1, gamma_m2);
    let shear = if f.shear_resistance > 0.0 { bearing.min(f.shear_resistance) } else { bearing };
    let screw = kind(f) == fs::Kind::Screw;
    let pull_over = (screw && f.washer_diameter > 0.0).then(|| fs::pull_over(f.washer_diameter, f.head_thickness, f.head_strength, gamma_m2));
    let pull_out = screw.then(|| fs::pull_out(f.diameter, f.base_thickness, f.base_strength, gamma_m2));
    let net = fs::net_section(f.sheet_width, f64::from(f.per_row), f.diameter, t1, fu1, gamma_m2);
    Some(Resistance { bearing, shear, pull_over, pull_out, net })
}

fn spacing_checks(f: &FastenedJoint, annex: AnnexChoice, report: &mut CheckReport) {
    let minima = fs::spacing(kind(f));
    let rows = [("endDistance", "End distance e_1", "Randabstand e_1", f.end_distance, minima[0]), ("edgeDistance", "Edge distance e_2", "Randabstand e_2", f.edge_distance, minima[1]), ("pitch", "Pitch p_1", "Lochabstand p_1", f.pitch, minima[2]), ("gauge", "Gauge p_2", "Rissabstand p_2", f.gauge, minima[3])];
    let (worst, ratio) = rows.iter().map(|r| (r, if r.3 > 0.0 { r.4 * f.diameter / r.3 } else { f64::INFINITY })).fold((&rows[0], 0.0), |a, b| if b.1 > a.1 { (b.0, b.1) } else { a });
    let required = worst.4 * f.diameter;
    let mut builder = CheckResult::assess(format!("en1993.1-3.8.3.spacing.{}", f.id), "DIN EN 1993-1-3", ClauseId::new("EN 1993-1-3", "8", "8.3"), subject(f, worst.0, worst.1, worst.2), loc("Minimum fastener spacing", "Mindestabstände der Verbindungsmittel"))
        .annex(annex)
        .utilization(dimensionless(if ratio.is_finite() { ratio } else { 2.0 }), dimensionless(1.0))
        .explanation(loc(
            &format!("Required over actual: e_1 {:.1}/{:.1} mm, e_2 {:.1}/{:.1} mm, p_1 {:.1}/{:.1} mm, p_2 {:.1}/{:.1} mm (minima {:.1} d, {:.1} d, {:.1} d, {:.1} d); governing {}.", minima[0] * f.diameter * 1e3, f.end_distance * 1e3, minima[1] * f.diameter * 1e3, f.edge_distance * 1e3, minima[2] * f.diameter * 1e3, f.pitch * 1e3, minima[3] * f.diameter * 1e3, f.gauge * 1e3, minima[0], minima[1], minima[2], minima[3], worst.1),
            &format!("Erforderlich gegenüber vorhanden: e_1 {:.1}/{:.1} mm, e_2 {:.1}/{:.1} mm, p_1 {:.1}/{:.1} mm, p_2 {:.1}/{:.1} mm (Mindestwerte {:.1} d, {:.1} d, {:.1} d, {:.1} d); maßgebend {}.", minima[0] * f.diameter * 1e3, f.end_distance * 1e3, minima[1] * f.diameter * 1e3, f.edge_distance * 1e3, minima[2] * f.diameter * 1e3, f.pitch * 1e3, minima[3] * f.diameter * 1e3, f.gauge * 1e3, minima[0], minima[1], minima[2], minima[3], worst.2),
        ))
        .ndp_reads();
    if ratio > 1.0 {
        builder = builder.remedy(Remedy::at_least(subject(f, worst.0, worst.1, worst.2), length_m(worst.3), length_m(required), loc(&format!("Increase the distance to at least {:.1} mm.", required * 1e3), &format!("Abstand auf mindestens {:.1} mm vergrößern.", required * 1e3))));
    }
    report.push(builder.build());
}

fn count_remedy(f: &FastenedJoint, utilization: f64) -> Remedy {
    let required = (f64::from(f.count) * utilization * (1.0 + 1e-9)).ceil();
    Remedy::at_least(subject(f, "count", "Number of fasteners", "Anzahl der Verbindungsmittel"), dimensionless(f64::from(f.count)), dimensionless(required), loc(&format!("Use at least {required:.0} fasteners."), &format!("Mindestens {required:.0} Verbindungsmittel anordnen.")))
}

/// 🪛️ Appends the checks of every admitted fastened joint.
pub fn check_fastened_joints(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, scope: Scope<'_>, report: &mut CheckReport) {
    for f in document.fastened_joints.iter().filter(|f| scope.admits("fastenedJoints", &f.id)) {
        let Some(r) = resistance(f, params.gamma_m2) else { continue };
        let n = f64::from(f.count);
        let (fv_ed, ft_ed) = (f.shear_force.abs() / n, f.tension_force.abs() / n);
        let u_v = fv_ed / r.shear;
        let name = if kind(f) == fs::Kind::Screw { ("self-tapping screws", "selbstbohrende Schrauben") } else { ("blind rivets", "Blindniete") };
        if f.shear_force != 0.0 {
            let mut b = CheckResult::assess(format!("en1993.1-3.8.3.shear.{}", f.id), "DIN EN 1993-1-3", ClauseId::new("EN 1993-1-3", "8", "8.3"), subject(f, "shearForce", "Shear force", "Querkraft"), loc("Fastener shear: bearing and fastener resistance", "Verbindungsmittel auf Abscheren: Lochleibung und Verbindungsmittelwiderstand"))
                .annex(annex)
                .ndp(AnnexParams::GAMMA_M2)
                .utilization(force_n(fv_ed), force_n(r.shear))
                .explanation(loc(
                    &format!("{} {}: F_v,Ed={:.2} kN per fastener against F_b,Rd={:.2} kN (sheets {:.2}/{:.2} mm){}.", f.count, name.0, fv_ed / 1e3, r.bearing / 1e3, f.head_thickness * 1e3, f.base_thickness * 1e3, if f.shear_resistance > 0.0 { format!(" and F_v,Rd={:.2} kN of the fastener", f.shear_resistance / 1e3) } else { String::new() }),
                    &format!("{} {}: F_v,Ed={:.2} kN je Verbindungsmittel gegenüber F_b,Rd={:.2} kN (Bleche {:.2}/{:.2} mm){}.", f.count, name.1, fv_ed / 1e3, r.bearing / 1e3, f.head_thickness * 1e3, f.base_thickness * 1e3, if f.shear_resistance > 0.0 { format!(" und F_v,Rd={:.2} kN des Verbindungsmittels", f.shear_resistance / 1e3) } else { String::new() }),
                ))
                .ndp_reads();
            if u_v > 1.0 {
                b = b.remedy(count_remedy(f, u_v));
            }
            report.push(b.build());
            let net_ed = f.shear_force.abs();
            if f.sheet_width > 0.0 {
                let t_fu = if f.head_thickness <= f.base_thickness { f.head_thickness * f.head_strength } else { f.base_thickness * f.base_strength };
                let mut b = CheckResult::assess(format!("en1993.1-3.8.3.net.{}", f.id), "DIN EN 1993-1-3", ClauseId::new("EN 1993-1-3", "8", "8.3"), subject(f, "sheetWidth", "Sheet width", "Blechbreite"), loc("Net section of the fastened sheet", "Nettoquerschnitt des angeschlossenen Blechs"))
                    .annex(annex)
                    .ndp(AnnexParams::GAMMA_M2)
                    .utilization(force_n(net_ed), force_n(r.net))
                    .explanation(loc(&format!("F_n,Rd=(b - n d) t f_u/γ_M2={:.2} kN with b={:.0} mm and {} fasteners across.", r.net / 1e3, f.sheet_width * 1e3, f.per_row), &format!("F_n,Rd=(b - n d) t f_u/γ_M2={:.2} kN mit b={:.0} mm und {} Verbindungsmitteln im Schnitt.", r.net / 1e3, f.sheet_width * 1e3, f.per_row)));
                if net_ed > r.net {
                    let b_req = net_ed * params.gamma_m2 / t_fu + f64::from(f.per_row) * f.diameter;
                    b = b.remedy(Remedy::at_least(subject(f, "sheetWidth", "Sheet width", "Blechbreite"), length_m(f.sheet_width), length_m(b_req * (1.0 + 1e-9)), loc(&format!("Increase the sheet width to at least {:.0} mm.", b_req * 1e3), &format!("Blechbreite auf mindestens {:.0} mm vergrößern.", b_req * 1e3))));
                }
                report.push(b.build());
            }
        }
        if f.tension_force != 0.0 {
            match r.tension() {
                Some(ft_rd) => {
                    let u_t = ft_ed / ft_rd;
                    let mut b = CheckResult::assess(format!("en1993.1-3.8.3.tension.{}", f.id), "DIN EN 1993-1-3", ClauseId::new("EN 1993-1-3", "8", "8.3"), subject(f, "tensionForce", "Tension force", "Zugkraft"), loc("Screw tension: pull-over and pull-out", "Schraube auf Zug: Durchknöpfen und Herausziehen"))
                        .annex(annex)
                        .ndp(AnnexParams::GAMMA_M2)
                        .utilization(force_n(ft_ed), force_n(ft_rd))
                        .explanation(loc(
                            &format!("F_t,Ed={:.2} kN per screw against F_p,Rd={:.2} kN (pull-over, d_w={:.0} mm) and F_o,Rd={:.2} kN (pull-out).", ft_ed / 1e3, r.pull_over.unwrap_or(0.0) / 1e3, f.washer_diameter * 1e3, r.pull_out.unwrap_or(0.0) / 1e3),
                            &format!("F_t,Ed={:.2} kN je Schraube gegenüber F_p,Rd={:.2} kN (Durchknöpfen, d_w={:.0} mm) und F_o,Rd={:.2} kN (Herausziehen).", ft_ed / 1e3, r.pull_over.unwrap_or(0.0) / 1e3, f.washer_diameter * 1e3, r.pull_out.unwrap_or(0.0) / 1e3),
                        ))
                        .ndp_reads();
                    if u_t > 1.0 {
                        b = b.remedy(count_remedy(f, u_t));
                    }
                    report.push(b.build());
                    if f.shear_force != 0.0 {
                        let u = u_v + u_t;
                        let mut b = CheckResult::assess(format!("en1993.1-3.8.3.interaction.{}", f.id), "DIN EN 1993-1-3", ClauseId::new("EN 1993-1-3", "8", "8.3"), subject(f, "count", "Number of fasteners", "Anzahl der Verbindungsmittel"), loc("Screw shear and tension interaction", "Schraube: Interaktion Abscheren und Zug"))
                            .annex(annex)
                            .ndp(AnnexParams::GAMMA_M2)
                            .utilization(dimensionless(u), dimensionless(1.0))
                            .explanation(loc(&format!("F_v,Ed/F_v,Rd + F_t,Ed/F_t,Rd = {u_v:.3} + {u_t:.3}."), &format!("F_v,Ed/F_v,Rd + F_t,Ed/F_t,Rd = {u_v:.3} + {u_t:.3}.")));
                        if u > 1.0 {
                            b = b.remedy(count_remedy(f, u));
                        }
                        report.push(b.build());
                    }
                }
                None => report.push(
                    CheckResult::assess(format!("en1993.1-3.8.3.tension.{}", f.id), "DIN EN 1993-1-3", ClauseId::new("EN 1993-1-3", "8", "8.3"), subject(f, "tensionForce", "Tension force", "Zugkraft"), loc("Fastener tension", "Verbindungsmittel auf Zug"))
                        .annex(annex)
                        .not_applicable(loc("Tension resistance is assessed for screws with a washer diameter only (pull-over and pull-out); for blind rivets use the value of the manufacturer.", "Der Zugwiderstand wird nur für Schrauben mit angegebenem Scheibendurchmesser ermittelt (Durchknöpfen und Herausziehen); für Blindniete den Wert des Herstellers ansetzen."))
                        .build(),
                ),
            }
        }
        spacing_checks(f, annex, report);
    }
}
