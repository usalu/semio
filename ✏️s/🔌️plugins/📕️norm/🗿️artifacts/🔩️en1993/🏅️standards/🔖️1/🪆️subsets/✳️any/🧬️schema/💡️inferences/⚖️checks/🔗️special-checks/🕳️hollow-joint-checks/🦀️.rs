//! 🕳️ Hollow section joint checks: the validity range, chord face failure and brace effective width failure of welded RHS K and N joints with a gap (EN 1993-1-8 7.4, Tables 7.8 and 7.11). See [EN 1993-1-8](https://eurocodes.jrc.ec.europa.eu/).

use super::*;
use crate::clauses::hollow_joints::{beta, brace_effective_width_rd, chord_face_rd, k_n, validity_violations, GapJoint};
use semio_s_artifact_norm_en1993_model::HollowJoint;

fn gamma_m5() -> f64 {
    semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-8.gamma-m5")
}

fn subject(j: &HollowJoint, field: &str, en: &str, de: &str) -> SubjectRef {
    SubjectRef::new(&j.id, format!("hollowJoints[id={}].{field}", j.id), loc(en, de))
}

fn joint_of(j: &HollowJoint) -> GapJoint {
    GapJoint {
        b0: j.chord_width * 1e3,
        t0: j.chord_thickness * 1e3,
        fy0: j.chord_fy / 1e6,
        sigma0: j.chord_stress / 1e6,
        b1: j.brace1_width * 1e3,
        h1: j.brace1_height * 1e3,
        t1: j.brace1_thickness * 1e3,
        theta1: j.brace1_angle,
        b2: j.brace2_width * 1e3,
        h2: j.brace2_height * 1e3,
        t2: j.brace2_thickness * 1e3,
        theta2: j.brace2_angle,
        gap: j.gap * 1e3,
        fyi: j.brace_fy / 1e6,
    }
}

/// 🕳️ Appends the hollow section joint checks of every admitted row.
pub fn check_hollow_joints(document: &En1993Snapshot, annex: AnnexChoice, scope: Scope<'_>, report: &mut CheckReport) {
    for j in document.hollow_joints.iter().filter(|j| scope.admits("hollowJoints", &j.id)) {
        if j.chord_width <= 0.0 || j.chord_thickness <= 0.0 || j.brace1_thickness <= 0.0 || j.brace2_thickness <= 0.0 || j.brace1_angle <= 0.0 || j.brace2_angle <= 0.0 || j.chord_fy <= 0.0 {
            continue;
        }
        let gap = joint_of(j);
        let violations = validity_violations(&gap);
        let (n1_rd, n2_rd) = chord_face_rd(&gap, gamma_m5());
        let bi1 = brace_effective_width_rd(&gap, gap.h1, gap.b1, gap.t1, gamma_m5());
        let bi2 = brace_effective_width_rd(&gap, gap.h2, gap.b2, gap.t2, gamma_m5());
        let (n1, n2) = (j.brace1_force.abs(), j.brace2_force.abs());
        let face = (n1 / n1_rd).max(n2 / n2_rd);
        let brace = (n1 / bi1).max(n2 / bi2);
        let utilisation = face.max(brace);
        let limit_note_en = if violations.is_empty() { "inside the validity range".to_string() } else { format!("OUTSIDE the validity range: {}", violations.join(", ")) };
        let limit_note_de = if violations.is_empty() { "innerhalb des Gültigkeitsbereichs".to_string() } else { format!("AUSSERHALB des Gültigkeitsbereichs: {}", violations.join(", ")) };
        let explanation = loc(
            &format!("β={:.3}, γ=b_0/(2 t_0)={:.2}, k_n={:.3}; chord face failure N_1,Rd={:.1} kN, N_2,Rd={:.1} kN (utilisation {face:.3}); brace effective width N_1,Rd={:.1} kN, N_2,Rd={:.1} kN (utilisation {brace:.3}); {limit_note_en}.", beta(&gap), gap.b0 / (2.0 * gap.t0), k_n(&gap), n1_rd / 1e3, n2_rd / 1e3, bi1 / 1e3, bi2 / 1e3),
            &format!("β={:.3}, γ=b_0/(2 t_0)={:.2}, k_n={:.3}; Gurtflanschversagen N_1,Rd={:.1} kN, N_2,Rd={:.1} kN (Ausnutzung {face:.3}); mittragende Breite der Strebe N_1,Rd={:.1} kN, N_2,Rd={:.1} kN (Ausnutzung {brace:.3}); {limit_note_de}.", beta(&gap), gap.b0 / (2.0 * gap.t0), k_n(&gap), n1_rd / 1e3, n2_rd / 1e3, bi1 / 1e3, bi2 / 1e3),
        );
        let mut builder = CheckResult::assess(format!("en1993.1-8.7.4.gap.{}", j.id), "DIN EN 1993-1-8", ClauseId::new("EN 1993-1-8", "7", "7.4"), subject(j, "chordThickness", "Chord thickness", "Gurtwanddicke"), loc("Welded RHS joint with a gap", "Geschweißter RHS-Knoten mit Spalt"))
            .annex(annex)
            .ndp("ndp.en1993.1-8.gamma-m5")
            .ndp("ndp.en1993.v-440.1")
            .ndp("ndp.en1993.v-440.2")
            .ndp("ndp.en1993.v-440.3")
            .utilization(dimensionless(utilisation), dimensionless(1.0))
            .explanation(explanation);
        if utilisation > 1.0 {
            let t0 = j.chord_thickness * utilisation.max(1.0).sqrt();
            builder = builder
                .remedy(Remedy::at_least(subject(j, "chordThickness", "Chord thickness", "Gurtwanddicke"), length_m(j.chord_thickness), length_m(t0), loc(&format!("Increase the chord wall thickness to at least {:.1} mm (first estimate, N_Rd ~ t_0²).", t0 * 1e3), &format!("Gurtwanddicke auf mindestens {:.1} mm erhöhen (erste Schätzung, N_Rd ~ t_0²).", t0 * 1e3))))
                .remedy(Remedy::at_most(subject(j, "brace1Force", "Brace 1 force", "Streben-1-Kraft"), force_n(n1), force_n(n1 / utilisation), loc("Or reduce the brace forces by a better load path.", "Oder die Strebenkräfte durch einen besseren Lastabtrag verringern.")));
        }
        report.push(builder.build());
    }
}
