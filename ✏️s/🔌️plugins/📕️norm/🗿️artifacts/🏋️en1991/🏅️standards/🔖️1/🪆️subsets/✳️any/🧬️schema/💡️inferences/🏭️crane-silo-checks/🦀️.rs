//! 🏭️ Moving-crane envelope of the runway girder (EN 1991-3, 2.5 and 2.7) and the silo action assessment class, slenderness and discharge pressures (EN 1991-4, 2.2 and 5.3). The wheel load, the factors C_h and C_w and the capacities of the classes are registry values, all unverified, so the verdicts report the unverified basis.

use super::*;
use semio_s_plugin_norm_compliance::AnnexChoice;

const SECTIONS: usize = 40;

/// 🌈️ Runway girder under the travelling crane: the end reaction R_max is the leading quantity, the largest moment M_max and shear V_max with the governing section are named in the explanation (influence lines of a simply supported span, two wheels at the wheel base).
pub(super) fn check_crane_runway(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport) {
    if !document.crane_claimed {
        return;
    }
    let Some(wheel) = part_3::design_vertical_wheel_n(&document.crane_class, &document.hoist_class, document.hoisting_speed) else { return };
    let (span, base) = (document.crane_runway_length, document.crane_wheel_base);
    if !(span.is_finite() && span > 0.0 && base.is_finite() && base > 0.0) {
        return;
    }
    let (moment, shear, reaction, section) = part_3::runway_envelope(wheel, span, base, SECTIONS);
    report.push(info_check(
        "en1991.3.crane-runway".to_owned(),
        "DIN EN 1991-3",
        ClauseId::new("EN 1991-3", "§2.7 Table 2.2", "2.7"),
        SubjectRef::new("", "craneRunwayLength", copy("Runway girder", "Kranbahnträger")),
        copy("Runway girder end reaction under the travelling crane (load group 1)", "Auflagerkraft des Kranbahnträgers unter dem fahrenden Kran (Lastgruppe 1)"),
        force(reaction),
        copy(
            &format!("Two wheels of φ₂·Q_r = {:.1} kN at {base:.2} m on a span of {span:.2} m: R_max = {:.1} kN, V_max = {:.1} kN, M_max = {:.1} kNm at x = {section:.2} m (single span, no continuity, no φ₁).", wheel / 1000.0, reaction / 1000.0, shear / 1000.0, moment / 1000.0),
            &format!("Zwei Räder mit φ₂·Q_r = {:.1} kN im Abstand {base:.2} m auf einer Stützweite von {span:.2} m: R_max = {:.1} kN, V_max = {:.1} kN, M_max = {:.1} kNm bei x = {section:.2} m (Einfeldträger, ohne Durchlaufwirkung, ohne φ₁).", wheel / 1000.0, reaction / 1000.0, shear / 1000.0, moment / 1000.0),
        ),
        annex,
    ));
}

/// 📊 Action assessment class from the mass held (capacity) and the slenderness class from h_c/d_c of a silo (2.2); a retaining silo (h_c/d_c ≤ 0.4) is outside the Janssen pressures of the artifact and said so.
pub(super) fn check_silo_class(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport) {
    let (diameter, height) = (document.silo_diameter, document.silo_height);
    if !(diameter.is_finite() && diameter > 0.0 && height.is_finite() && height > 0.0) {
        return;
    }
    let (capacity, ratio) = (part_4::silo_mass_t(document.silo_bulk_density, diameter, height), height / diameter);
    let (class, slenderness) = (part_4::action_assessment_class(capacity), part_4::slenderness_class(ratio));
    let retaining = if slenderness == "retaining" { (" Retaining silo: the Janssen pressures of this artifact do not apply (unverified).", " Stützsilo: die Janssen-Drücke dieses Artefakts gelten nicht (ungeprüft).") } else { ("", "") };
    report.push(info_check(
        "en1991.4.silo-class".to_owned(),
        "DIN EN 1991-4",
        ClauseId::new("EN 1991-4", "§2.2 Table 2.1", "2.2"),
        SubjectRef::new("", "siloDiameter", copy("Silo classification", "Siloklassifizierung")),
        copy("Action assessment class and slenderness h_c/d_c", "Auswirkungsklasse und Schlankheit h_c/d_c"),
        Quantity::new(QuantityKind::Dimensionless, ratio),
        copy(&format!("Capacity {capacity:.1} t gives action assessment class {class}; h_c/d_c = {ratio:.2} is {slenderness}.{}", retaining.0), &format!("Fassungsvermögen {capacity:.1} t ergibt Auswirkungsklasse {class}; h_c/d_c = {ratio:.2} ist {slenderness}.{}", retaining.1)),
        annex,
    ));
}

/// ⏬ Discharge pressures p_he = C_h p_hf,max and p_we = C_w p_wf,max (5.3) against the assumed silo pressure and wall friction of the same fields as the filling checks.
pub(super) fn check_silo_discharge(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport) {
    let depth = document.silo_height;
    let (gamma, a, mu, k) = (document.silo_bulk_density, document.silo_hydraulic_radius, document.silo_mu, document.silo_k);
    let (required_h, required_w) = (part_4::discharge_horizontal_max_pa(gamma, a, mu, k, depth), part_4::discharge_wall_friction_max_pa(gamma, a, mu, k, depth));
    let rows = [
        ("en1991.4.silo-discharge", "assumedSiloPressure", "Discharge horizontal pressure p_he = C_h p_hf", "Entleerungs-Horizontaldruck p_he = C_h p_hf", "pressure", "Druck", document.assumed_silo_pressure, required_h),
        ("en1991.4.silo-discharge-friction", "assumedSiloWallFriction", "Discharge wall friction p_we = C_w p_wf", "Entleerungs-Wandreibung p_we = C_w p_wf", "wall friction", "Wandreibung", document.assumed_silo_wall_friction, required_w),
    ];
    for (id, path, en, de, label_en, label_de, assumed, required) in rows {
        let remedy = if assumed + 1e-9 < required { Some(raise_pressure_remedy(path, "", label_en, label_de, assumed, required)) } else { None };
        report.push(assess_covers_on(
            id,
            "DIN EN 1991-4",
            ClauseId::new("EN 1991-4", "§5.3", "5.3"),
            SubjectRef::new("", path, copy("Silo discharge", "Silo-Entleerung")),
            copy(en, de),
            pressure(assumed),
            pressure(required),
            copy(&format!("C_h = {:.2}, C_w = {:.2} (simplified factors, unverified): required {:.1} kN/m²; assumed {:.1} kN/m².", part_4::discharge_c_h(), part_4::discharge_c_w(), required / 1000.0, assumed / 1000.0), &format!("C_h = {:.2}, C_w = {:.2} (vereinfachte Beiwerte, ungeprüft): erforderlich {:.1} kN/m²; angenommen {:.1} kN/m².", part_4::discharge_c_h(), part_4::discharge_c_w(), required / 1000.0, assumed / 1000.0)),
            annex,
            remedy,
            &["ndp.en1991.4.discharge-c-h", "ndp.en1991.4.discharge-c-w"],
        ));
    }
}

/// 🪝 One crane hoist (EN 1991-3, 2.5 and 2.9): the dynamic factor φ₃ = 1 − (Δm/m)(1 + β₃) of the suddenly released payload fraction and the static and dynamic test loads Q_T of the rated hoist load Q_H. An unknown release device or a fraction outside 0…1 is an invalid input, no fallback is taken.
pub(super) fn check_crane_hoist(hoist: &semio_s_artifact_norm_en1991_model::CraneHoist, annex: AnnexChoice, report: &mut CheckReport) {
    let subject = |leaf: &str, en: &str, de: &str| SubjectRef::new(&hoist.id, &format!("craneHoists[id={}].{leaf}", hoist.id), copy(en, de));
    let clause = |locator: &str, section: &str| ClauseId::new("EN 1991-3", locator, section);
    let admissible = ["grab", "magnet"].map(String::from).to_vec();
    let beta = part_3::beta_3(&hoist.release_device);
    if beta.is_none() {
        invalid(report, annex, format!("en1991.input.craneHoists.{}.releaseDevice", hoist.id), "DIN EN 1991-3", clause("§2.5", "2.5"), &hoist.id, format!("craneHoists[id={}].releaseDevice", hoist.id), ("hoist release device", "Abwurfeinrichtung des Hubwerks"), &hoist.release_device, admissible);
    }
    if !(hoist.release_fraction.is_finite() && (0.0..=1.0).contains(&hoist.release_fraction)) {
        invalid(report, annex, format!("en1991.input.craneHoists.{}.releaseFraction", hoist.id), "DIN EN 1991-3", clause("§2.5", "2.5"), &hoist.id, format!("craneHoists[id={}].releaseFraction", hoist.id), ("released payload fraction Δm/m (0 to 1)", "abgeworfener Lastanteil Δm/m (0 bis 1)"), &hoist.release_fraction.to_string(), vec!["0 … 1".into()]);
    } else if let Some(beta) = beta {
        let phi = part_3::phi_3(hoist.release_fraction, beta);
        report.push(info_check(
            format!("en1991.3.hoist-release.{}", hoist.id),
            "DIN EN 1991-3",
            clause("§2.5", "2.5"),
            subject("releaseFraction", "Hoist release", "Hubwerk-Abwurf"),
            copy("Dynamic factor φ₃ of the suddenly released payload", "Dynamischer Beiwert φ₃ der plötzlich abgeworfenen Nutzlast"),
            Quantity::new(QuantityKind::Dimensionless, phi),
            copy(&format!("φ₃ = 1 − Δm/m (1 + β₃) = 1 − {:.2} · (1 + {beta:.2}) = {phi:.3} (β₃ and the formula from the spec line E3.01, unverified).", hoist.release_fraction), &format!("φ₃ = 1 − Δm/m (1 + β₃) = 1 − {:.2} · (1 + {beta:.2}) = {phi:.3} (β₃ und Formel aus der Spezifikationszeile E3.01, ungeprüft).", hoist.release_fraction)),
            annex,
        ));
    }
    if !(hoist.rated_load.is_finite() && hoist.rated_load > 0.0) {
        invalid(report, annex, format!("en1991.input.craneHoists.{}.ratedLoad", hoist.id), "DIN EN 1991-3", clause("§2.9", "2.9"), &hoist.id, format!("craneHoists[id={}].ratedLoad", hoist.id), ("rated hoist load Q_H (positive)", "Hublast Q_H (positiv)"), &hoist.rated_load.to_string(), vec!["> 0".into()]);
        return;
    }
    for (id, factor, load, en, de) in [
        ("hoist-test-static", "static", part_3::test_load_static_n(hoist.rated_load), "Static test load Q_T of the hoist", "Statische Prüflast Q_T des Hubwerks"),
        ("hoist-test-dynamic", "dynamic", part_3::test_load_dynamic_n(hoist.rated_load), "Dynamic test load Q_T of the hoist", "Dynamische Prüflast Q_T des Hubwerks"),
    ] {
        report.push(info_check(
            format!("en1991.3.{id}.{}", hoist.id),
            "DIN EN 1991-3",
            clause("§2.9", "2.9"),
            subject("ratedLoad", "Hoist test load", "Hubwerk-Prüflast"),
            copy(en, de),
            force(load),
            copy(&format!("Q_T = {:.2} · Q_H = {:.1} kN of the rated {:.1} kN ({factor} test; factor from the spec line E3.04, unverified: the repo spec contradicts itself on the φ₆ line and the factors 1.1 and 1.25 may be swapped, compare with the test loads of EN 1991-3 in the print).", load / hoist.rated_load, load / 1000.0, hoist.rated_load / 1000.0), &format!("Q_T = {:.2} · Q_H = {:.1} kN der Hublast {:.1} kN ({} Prüfung; Faktor aus der Spezifikationszeile E3.04, ungeprüft: die Spezifikation widerspricht sich auf der φ₆-Zeile und die Faktoren 1,1 und 1,25 sind möglicherweise vertauscht, mit den Prüflasten der EN 1991-3 im Druck vergleichen).", load / hoist.rated_load, load / 1000.0, hoist.rated_load / 1000.0, if factor == "static" { "statische" } else { "dynamische" })),
            annex,
        ));
    }
}

/// 🧾 One information check of a collection row: the id `<check>.<row id>`, the row's own leaf as subject, the quantity and the localized explanation, the clause of the given part.
pub(super) fn row_check<I: AsRef<str>, E: AsRef<str>>(report: &mut CheckReport, annex: AnnexChoice, (part, locator, section): (&str, &str, &str), (collection, row_id, leaf): (&str, I, &str), (check, quantity): (&str, Quantity), (subject_en, subject_de): (&str, &str), (title_en, title_de): (&str, &str), (explanation_en, explanation_de): (E, E)) {
    let row_id = row_id.as_ref();
    report.push(info_check(
        format!("{check}.{row_id}"),
        &format!("DIN {part}"),
        ClauseId::new(part, locator, section),
        SubjectRef::new(row_id, &format!("{collection}[id={row_id}].{leaf}"), copy(subject_en, subject_de)),
        copy(title_en, title_de),
        quantity,
        copy(explanation_en.as_ref(), explanation_de.as_ref()),
        annex,
    ));
}

/// ⚙️ One rotating machine (EN 1991-3, 5): the equivalent static force F_dyn = m e ω² with ω = 2π f; a non-positive mass, a negative eccentricity or a non-positive frequency is an invalid input without a fallback. The spec line gives no verdict criterion, so the force is reported as information.
pub(super) fn check_rotating_machine(machine: &semio_s_artifact_norm_en1991_model::RotatingMachine, annex: AnnexChoice, report: &mut CheckReport) {
    let clause = || ClauseId::new("EN 1991-3", "§5", "5");
    for (leaf, ok, en, de, found) in [("mass", machine.mass.is_finite() && machine.mass > 0.0, "rotating mass m (positive)", "rotierende Masse m (positiv)", machine.mass), ("eccentricity", machine.eccentricity.is_finite() && machine.eccentricity >= 0.0, "eccentricity e (not negative)", "Exzentrizität e (nicht negativ)", machine.eccentricity), ("frequency", machine.frequency.is_finite() && machine.frequency > 0.0, "rotation frequency f (positive)", "Drehfrequenz f (positiv)", machine.frequency)] {
        if !ok {
            invalid(report, annex, format!("en1991.input.rotatingMachines.{}.{leaf}", machine.id), "DIN EN 1991-3", clause(), &machine.id, format!("rotatingMachines[id={}].{leaf}", machine.id), (en, de), &found.to_string(), vec!["valid".into()]);
            return;
        }
    }
    let force_n = part_3::machine_dynamic_force_n(machine.mass, machine.eccentricity, machine.frequency);
    let text = format!("F_dyn = m e ω² = {:.1} kg · {:.4} m · (2π · {:.2} Hz)² = {:.2} kN (formula from the spec line E3.07, unverified).", machine.mass, machine.eccentricity, machine.frequency, force_n / 1000.0);
    let text_de = format!("F_dyn = m e ω² = {:.1} kg · {:.4} m · (2π · {:.2} Hz)² = {:.2} kN (Formel aus der Spezifikationszeile E3.07, ungeprüft).", machine.mass, machine.eccentricity, machine.frequency, force_n / 1000.0);
    row_check(report, annex, ("EN 1991-3", "§5", "5"), ("rotatingMachines", &machine.id, "mass"), ("en1991.3.machine-force", force(force_n)), ("Rotating machine", "Rotierende Maschine"), ("Equivalent static force of the unbalanced mass", "Statische Ersatzkraft der Unwucht"), (&text, &text_de));
}

/// 🌾 One bulk solid (EN 1991-4, Annex E): the lateral pressure ratio K = 1 − sin φ_i and its mean K_m = 1.1 K; an angle outside 0 to 90 degrees is an invalid input without a fallback.
pub(super) fn check_bulk_solid(solid: &semio_s_artifact_norm_en1991_model::BulkSolid, annex: AnnexChoice, report: &mut CheckReport) {
    let phi = solid.internal_friction;
    if !(phi.is_finite() && phi > 0.0 && phi < std::f64::consts::FRAC_PI_2) {
        invalid(report, annex, format!("en1991.input.bulkSolids.{}.internalFriction", solid.id), "DIN EN 1991-4", ClauseId::new("EN 1991-4", "Annex E", "E.3"), &solid.id, format!("bulkSolids[id={}].internalFriction", solid.id), ("angle of internal friction φ_i (0 to 90°)", "innerer Reibungswinkel φ_i (0 bis 90°)"), &phi.to_string(), vec!["0 … 90°".into()]);
        return;
    }
    let k_mean = part_4::k_mean(phi);
    let text = format!("K_m = 1.1 (1 − sin φ_i) = 1.1 · (1 − sin {:.1}°) = {k_mean:.3}, K = {:.3} (factor from the spec line E4.02, unverified).", phi.to_degrees(), 1.0 - phi.sin());
    let text_de = format!("K_m = 1,1 (1 − sin φ_i) = 1,1 · (1 − sin {:.1}°) = {k_mean:.3}, K = {:.3} (Faktor aus der Spezifikationszeile E4.02, ungeprüft).", phi.to_degrees(), 1.0 - phi.sin());
    row_check(report, annex, ("EN 1991-4", "Annex E", "E.3"), ("bulkSolids", &solid.id, "internalFriction"), ("en1991.4.bulk-k-mean", Quantity::new(QuantityKind::Dimensionless, k_mean)), ("Bulk solid", "Schüttgut"), ("Mean lateral pressure ratio K_m", "Mittleres Horizontallastverhältnis K_m"), (&text, &text_de));
}
