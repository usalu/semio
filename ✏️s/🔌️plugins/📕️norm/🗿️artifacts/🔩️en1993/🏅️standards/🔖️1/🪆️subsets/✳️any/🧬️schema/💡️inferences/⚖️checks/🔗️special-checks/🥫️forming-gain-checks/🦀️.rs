//! 🥫️ Enhanced yield strength of cold-formed members: the average yield strength f_ya from the work hardening of the bends against the yield strength the design uses (EN 1993-1-3 3.2.2). See [EN 1993-1-3](https://eurocodes.jrc.ec.europa.eu/).

use super::*;
use crate::clauses::cold_formed;
use semio_s_artifact_norm_en1993_model::{FormingGain, FormingProcess};

/// 🥫️ Number of bends that count: all of them, or none when the member's corner radius exceeds the registry ratio r/t.
pub fn bends_counted(g: &FormingGain, t: f64, corner_radius: f64) -> f64 {
    if corner_radius > 0.0 && corner_radius > semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-3.fya.max-radius-ratio") * t { 0.0 } else { f64::from(g.bends) }
}

/// 🥫️ The average yield strength (Pa) of `g` for `bends` counted bends and the basic yield strength `fyb`.
pub fn average_yield(g: &FormingGain, bends: f64, fyb: f64, t: f64) -> f64 {
    let k = semio_s_plugin_norm_compliance::ndp_common(match g.forming {
        FormingProcess::RollForming => "ndp.en1993.1-3.fya.k-roll",
        FormingProcess::OtherForming => "ndp.en1993.1-3.fya.k-other",
    });
    cold_formed::average_yield(fyb, g.ultimate_strength, k, bends, t, g.gross_area)
}

/// 🥫️ Appends one check per forming gain row: the yield strength used against f_ya, with f_ya as remedy.
pub fn check_forming_gains(document: &En1993Snapshot, annex: AnnexChoice, scope: Scope<'_>, report: &mut CheckReport) {
    for g in document.forming_gains.iter().filter(|g| scope.admits("formingGains", &g.id)) {
        let Some(m) = document.cold_formed_members.iter().find(|m| m.id == g.member_id) else { continue };
        if m.fy <= 0.0 || m.thickness <= 0.0 || g.gross_area <= 0.0 || g.ultimate_strength <= m.fy {
            continue;
        }
        let fya = average_yield(g, bends_counted(g, m.thickness, m.corner_radius), m.fy, m.thickness);
        let used = if g.used_yield > 0.0 { g.used_yield } else { m.fy };
        let target = SubjectRef::new(&g.id, format!("formingGains[id={}].usedYield", g.id), loc("Yield strength used", "Angesetzte Streckgrenze"));
        let mut b = CheckResult::assess(format!("en1993.1-3.3.2.2.fya.{}", g.id), "DIN EN 1993-1-3", ClauseId::new("EN 1993-1-3", "3", "3.2.2"), target.clone(), loc("Enhanced yield strength of the cold forming", "Erhöhte Streckgrenze durch Kaltumformung"))
            .annex(annex)
            .utilization(stress_pa(used), stress_pa(fya.max(m.fy)))
            .explanation(loc(
                &format!("f_ya={:.1} MPa from f_yb={:.0} MPa, f_u={:.0} MPa, {} bends, t={:.2} mm, A_g={:.0} mm² (limit (f_u+f_yb)/2={:.1} MPa); the design uses {:.1} MPa. Valid for fully effective cross-sections only.", fya / 1e6, m.fy / 1e6, g.ultimate_strength / 1e6, g.bends, m.thickness * 1e3, g.gross_area * 1e6, (g.ultimate_strength + m.fy) / 2e6, used / 1e6),
                &format!("f_ya={:.1} MPa aus f_yb={:.0} MPa, f_u={:.0} MPa, {} Biegungen, t={:.2} mm, A_g={:.0} mm² (Grenze (f_u+f_yb)/2={:.1} MPa); angesetzt sind {:.1} MPa. Nur für voll wirksame Querschnitte zulässig.", fya / 1e6, m.fy / 1e6, g.ultimate_strength / 1e6, g.bends, m.thickness * 1e3, g.gross_area * 1e6, (g.ultimate_strength + m.fy) / 2e6, used / 1e6),
            ))
            .ndp_reads();
        if used > fya.max(m.fy) {
            b = b.remedy(Remedy::at_most(target, stress_pa(used), stress_pa(fya.max(m.fy)), loc(&format!("Use at most f_ya={:.1} MPa.", fya.max(m.fy) / 1e6), &format!("Höchstens f_ya={:.1} MPa ansetzen.", fya.max(m.fy) / 1e6))));
        }
        report.push(b.build());
    }
}
