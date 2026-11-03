//! 🧵️ Weld group checks: the elastic vector method for in-plane shear with eccentricity and the simplified weld resistance (EN 1993-1-8 4.5.3.3). See [EN 1993-1-8](https://eurocodes.jrc.ec.europa.eu/).

use super::*;
use crate::clauses::weld_group::{group_properties, max_force_per_length, resistance_per_length, WeldPattern};
use semio_s_artifact_norm_en1993_model::WeldGroup;

fn subject(g: &WeldGroup, field: &str, en: &str, de: &str) -> SubjectRef {
    SubjectRef::new(&g.id, format!("weldGroups[id={}].{field}", g.id), loc(en, de))
}

fn pattern(g: &WeldGroup) -> WeldPattern {
    match g.pattern {
        semio_s_artifact_norm_en1993_model::WeldPattern::Line => WeldPattern::Line,
        semio_s_artifact_norm_en1993_model::WeldPattern::Box => WeldPattern::Box,
        _ => WeldPattern::TwoLines,
    }
}

/// 🧵️ Appends the weld group checks of every admitted row.
pub fn check_weld_groups(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, scope: Scope<'_>, report: &mut CheckReport) {
    for g in document.weld_groups.iter().filter(|g| scope.admits("weldGroups", &g.id)) {
        if g.throat <= 0.0 || g.depth <= 0.0 || g.shear_force == 0.0 || g.steel_fu <= 0.0 || g.beta_w <= 0.0 {
            continue;
        }
        let (a, b, d, fu) = (g.throat * 1e3, g.width * 1e3, g.depth * 1e3, g.steel_fu / 1e6);
        let props = group_properties(pattern(g), b, d);
        let q = max_force_per_length(&props, g.shear_force.abs(), g.eccentricity * 1e3);
        let rd = resistance_per_length(a, fu, g.beta_w, params.gamma_m2);
        let explanation = loc(
            &format!("Elastic vector method: L_w={:.0} mm, I_p={:.3e} mm³, T=F e={:.2} kNm, F_w,Ed={q:.1} N/mm at the corner (x={:.0} mm, y={:.0} mm); F_w,Rd=f_u/√3/(β_w γ_M2) a={rd:.1} N/mm.", props.length, props.i_polar, g.shear_force.abs() * g.eccentricity / 1e3, props.x_max, props.y_max),
            &format!("Elastisches Vektorverfahren: L_w={:.0} mm, I_p={:.3e} mm³, T=F e={:.2} kNm, F_w,Ed={q:.1} N/mm an der Ecke (x={:.0} mm, y={:.0} mm); F_w,Rd=f_u/√3/(β_w γ_M2) a={rd:.1} N/mm.", props.length, props.i_polar, g.shear_force.abs() * g.eccentricity / 1e3, props.x_max, props.y_max),
        );
        let mut builder = CheckResult::assess(format!("en1993.1-8.4.5.group.{}", g.id), "DIN EN 1993-1-8", ClauseId::new("EN 1993-1-8", "4", "4.5.3.3"), subject(g, "throat", "Throat thickness", "Nahtdicke"), loc("Weld group under shear and torsion", "Schweißnahtgruppe unter Querkraft und Torsion"))
            .annex(annex)
            .ndp(AnnexParams::GAMMA_M2)
            .utilization(Quantity::new(QuantityKind::Force, q), Quantity::new(QuantityKind::Force, rd))
            .explanation(explanation);
        if q > rd {
            let throat = g.throat * q / rd;
            let force = g.shear_force.abs() * rd / q;
            builder = builder
                .remedy(Remedy::at_least(subject(g, "throat", "Throat thickness", "Nahtdicke"), length_m(g.throat), length_m(throat), loc(&format!("Increase the throat thickness to at least {:.1} mm.", throat * 1e3), &format!("Nahtdicke auf mindestens {:.1} mm erhöhen.", throat * 1e3))))
                .remedy(Remedy::at_most(subject(g, "shearForce", "Shear force", "Querkraft"), force_n(g.shear_force.abs()), force_n(force), loc(&format!("Or limit F_Ed to {:.1} kN.", force / 1e3), &format!("Oder F_Ed auf {:.1} kN begrenzen.", force / 1e3))));
        }
        report.push(builder.build());
    }
}
