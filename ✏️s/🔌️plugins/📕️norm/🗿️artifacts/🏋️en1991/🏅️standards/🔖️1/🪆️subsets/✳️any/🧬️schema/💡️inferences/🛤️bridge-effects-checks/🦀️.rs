//! 🛤️ Horizontal bridge traffic effects of EN 1991-2, 4.4 (braking and acceleration Q_lk, centrifugal force Q_tk of Table 4.3) and the thermal actions of EN 1991-1-5 as they enter the load combinations of EN 1990
//! through the shared `ActionFactors` (γ_Q and ψ rows of the registry). Every registry value is unverified, so the verdicts report the unverified basis.

use semio_s_artifact_norm_en1991_checks::{copy, invalid_input, part_1_5, part_2, temp_k as temp};
use semio_s_plugin_norm_compliance::{CheckReport, CheckResult, ClauseId, Quantity, QuantityKind, SubjectRef};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
use semio_s_plugin_norm_compliance::action_factors::Domain;
use semio_s_plugin_norm_compliance::{ActionFactors, AnnexChoice, DesignSituation, ReliabilityClass};

fn force(kn: f64) -> Quantity { Quantity::new(QuantityKind::Force, kn * 1000.0) }

/// 🛑 Braking and acceleration force Q_lk of lane 1 over the span (4.4.1): the share of the 900 kN cap, so the verdict shows how close the cap is.
pub fn check_braking(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport) {
    let (span, lanes) = (document.bridge_span, document.bridge_lanes.iter().map(|l| (l.lm1_lane, l.width)).collect::<Vec<_>>());
    let (low, high) = part_2::braking_bounds_kn(annex);
    let q_lk = part_2::braking_force_of_deck_kn(annex, span, document.bridge_lane_width, &lanes);
    let w_1 = part_2::lane_one_width_m(document.bridge_lane_width, &lanes);
    let governing = if (q_lk - low).abs() < 1e-9 { ("the lower bound 180 α_Q1", "die untere Grenze 180 α_Q1") } else if (q_lk - high).abs() < 1e-9 { ("the upper bound 900 kN", "die obere Grenze 900 kN") } else { ("Eq. 4.4", "Gl. 4.4") };
    report.push(
        CheckResult::assess("en1991.2.braking", "DIN EN 1991-2", ClauseId::new("EN 1991-2", "§4.4.1", "4.4.1"), SubjectRef::new("", "bridgeSpan", copy("Braking and acceleration force", "Brems- und Anfahrkraft")), copy("Braking force Q_lk = 0.6 α_Q1 (2 Q_1k) + 0.10 α_q1 q_1k w_1 L within 180 α_Q1 ≤ Q_lk ≤ 900 kN", "Bremskraft Q_lk = 0,6 α_Q1 (2 Q_1k) + 0,10 α_q1 q_1k w_1 L innerhalb 180 α_Q1 ≤ Q_lk ≤ 900 kN"))
            .utilization(force(q_lk), force(high))
            .status(semio_s_plugin_norm_compliance::CheckStatus::Pass)
            .annex(annex)
            .explanation(copy(
                &format!("Q_lk = {q_lk:.1} kN for L = {span:.1} m and w_1 = {w_1:.2} m (bounds {low:.0} to {high:.0} kN; governed by {}); acts in the lane axis at the surface level along the deck.", governing.0),
                &format!("Q_lk = {q_lk:.1} kN für L = {span:.1} m und w_1 = {w_1:.2} m (Grenzen {low:.0} bis {high:.0} kN; maßgebend {}); wirkt in der Fahrstreifenachse in Fahrbahnhöhe entlang des Decks.", governing.1),
            ))
            .ndp_reads()
            .build(),
    );
}

/// 🌀 Centrifugal force Q_tk of a curved deck (4.4.2, Table 4.3) from the total tandem weight Q_v of the loaded lanes and the horizontal radius; a radius of zero is a straight deck.
pub fn check_centrifugal(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport) {
    let radius = document.bridge_curve_radius;
    let clause = ClauseId::new("EN 1991-2", "§4.4.2 Table 4.3", "4.4.2");
    let subject = SubjectRef::new("", "bridgeCurveRadius", copy("Horizontal radius of the deck", "Horizontaler Radius des Decks"));
    let title = copy("Centrifugal force Q_tk of Table 4.3", "Fliehkraft Q_tk nach Tabelle 4.3");
    if !radius.is_finite() || radius < 0.0 {
        let admissible = vec!["0".to_owned(), "300".to_owned(), "1000".to_owned()];
        report.push(invalid_input("en1991.input.bridgeCurveRadius", "DIN EN 1991-2", clause, subject, title, copy(&format!("The radius {radius} m is not admissible; use 0 for a straight deck or a positive radius. The check is not evaluated with an invented default."), &format!("Der Radius {radius} m ist unzulässig; 0 für ein gerades Deck oder einen positiven Radius eingeben. Der Nachweis wird nicht mit einem erfundenen Ersatzwert geführt.")), admissible, annex));
        return;
    }
    if radius == 0.0 {
        report.push(CheckResult::assess("en1991.2.centrifugal", "DIN EN 1991-2", clause, subject, title).not_applicable(copy("Straight deck (radius 0): no centrifugal force.", "Gerades Deck (Radius 0): keine Fliehkraft.")).annex(annex).build());
        return;
    }
    let lanes = part_2::lane_layout(document.bridge_lane_width, &document.bridge_lanes.iter().map(|l| l.width).collect::<Vec<_>>()).0;
    let q_v = part_2::tandem_weight_kn(annex, lanes);
    let q_tk = part_2::centrifugal_force_kn(annex, q_v, radius);
    report.push(info_check(
        "en1991.2.centrifugal".to_owned(),
        "DIN EN 1991-2",
        clause,
        subject,
        title,
        force(q_tk),
        copy(&format!("Q_tk = {q_tk:.1} kN from Q_v = {q_v:.0} kN (tandem systems of {} lane(s)) and r = {radius:.0} m; acts radially at the surface level, together with the traffic load of the lanes.", lanes.min(3)), &format!("Q_tk = {q_tk:.1} kN aus Q_v = {q_v:.0} kN (Tandemsysteme von {} Fahrstreifen) und r = {radius:.0} m; wirkt radial in Fahrbahnhöhe zusammen mit der Verkehrslast der Fahrstreifen.", lanes.min(3))),
        annex,
    ));
}

/// 🌡️ Design values of the thermal action in the load combinations of EN 1990: γ_Q ΔT (leading), γ_Q ψ_0 ΔT (accompanying), ψ_1 ΔT (frequent) and ψ_2 ΔT (quasi-permanent) from the shared `ActionFactors` (registry, Table A1.1 / A2.1 category `temperature` or `temperature_bridge`).
pub fn check_thermal_combination(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport) {
    let Some(delta_t) = part_1_5::required_delta_t(&document.thermal_element_type, document.t_max, document.t_min, document.t_0, document.thermal_bridge_type, document.delta_t_m) else { return };
    let bridge = document.structure_kind == semio_s_artifact_norm_en1991_model::StructureKind::Bridge;
    let (domain, category) = if bridge { (Domain::Bridge, "temperature_bridge") } else { (Domain::Building, "temperature") };
    let factors = ActionFactors::new(annex, ReliabilityClass::RC2, DesignSituation::Persistent).domain(domain);
    let (gamma, psi) = (factors.gamma_q(category), factors.psi(category));
    let (leading, accompanying) = (gamma * delta_t, gamma * psi.psi_0 * delta_t);
    let (frequent, quasi) = (psi.psi_1 * delta_t, psi.psi_2 * delta_t);
    report.push(info_check(
        "en1991.1-5.combination".to_owned(),
        "DIN EN 1991-1-5",
        ClauseId::new("EN 1991-1-5", "§6 / EN 1990 §6.4", "6"),
        SubjectRef::new("", "assumedDeltaT", copy("Thermal action in combinations", "Temperatureinwirkung in Kombinationen")),
        copy("Design values of the thermal action in the EN 1990 combinations", "Bemessungswerte der Temperatureinwirkung in den Kombinationen nach EN 1990"),
        temp(leading),
        copy(
            &format!("ΔT_k = {delta_t:.1} K, γ_Q = {gamma:.2}, ψ_0/ψ_1/ψ_2 = {:.2}/{:.2}/{:.2} ({category}): leading γ_Q ΔT = {leading:.1} K, accompanying γ_Q ψ_0 ΔT = {accompanying:.1} K, frequent ψ_1 ΔT = {frequent:.1} K, quasi-permanent ψ_2 ΔT = {quasi:.1} K.", psi.psi_0, psi.psi_1, psi.psi_2),
            &format!("ΔT_k = {delta_t:.1} K, γ_Q = {gamma:.2}, ψ_0/ψ_1/ψ_2 = {:.2}/{:.2}/{:.2} ({category}): führend γ_Q ΔT = {leading:.1} K, begleitend γ_Q ψ_0 ΔT = {accompanying:.1} K, häufig ψ_1 ΔT = {frequent:.1} K, quasi-ständig ψ_2 ΔT = {quasi:.1} K.", psi.psi_0, psi.psi_1, psi.psi_2),
        ),
        annex,
    ));
}

/// 🚶 One footbridge (EN 1991-2, 5.3.2.1, 5.4 and 5.7): the uniform load q_fk of the loaded length, the horizontal force Q_flk and the comfort-verification trigger of EN 1990 A2.4.3.2(1) (first vertical frequency below the registry bound) with the walking-resonance verdict inside it (information: the printed limits of the comfort analysis are unverified). A non-positive length or width or a negative frequency is an invalid input without a fallback.
pub fn check_footbridge(bridge: &semio_s_artifact_norm_en1991_model::Footbridge, annex: AnnexChoice, report: &mut CheckReport) {
    use semio_s_artifact_norm_en1991_inferences_crane_silo::row_check;
    let clause = || ClauseId::new("EN 1991-2", "§5.3.2.1", "5.3.2.1");
    for (leaf, ok, en, de, found) in [("loadedLength", bridge.loaded_length.is_finite() && bridge.loaded_length > 0.0, "loaded length L (positive)", "belastete Länge L (positiv)", bridge.loaded_length), ("deckWidth", bridge.deck_width.is_finite() && bridge.deck_width > 0.0, "deck width b (positive)", "Deckbreite b (positiv)", bridge.deck_width)] {
        if !ok {
            invalid(report, annex, format!("en1991.input.footbridges.{}.{leaf}", bridge.id), "DIN EN 1991-2", clause(), &bridge.id, format!("footbridges[id={}].{leaf}", bridge.id), (en, de), &found.to_string(), vec!["> 0".into()]);
            return;
        }
    }
    let (length, width) = (bridge.loaded_length, bridge.deck_width);
    let q_fk = part_2::footbridge_q_fk_pa(length);
    let q_flk = part_2::footbridge_horizontal_n(length, width);
    row_check(report, annex, ("EN 1991-2", "§5.3.2.1", "5.3.2.1"), ("footbridges", &bridge.id, "loadedLength"), ("en1991.2.footbridge-load", Quantity::new(QuantityKind::Pressure, q_fk)), ("Footbridge", "Fußgängerbrücke"), ("Uniform load q_fk of the footbridge", "Gleichmäßig verteilte Last q_fk der Fußgängerbrücke"), (&format!("q_fk = 2.0 + 120/(L + 30) = 2.0 + 120/({length:.1} + 30) kN/m², limited to 2.5 … 5.0 kN/m²: {:.2} kN/m² (spec line E2.12, unverified).", q_fk / 1000.0), &format!("q_fk = 2,0 + 120/(L + 30) = 2,0 + 120/({length:.1} + 30) kN/m², begrenzt auf 2,5 … 5,0 kN/m²: {:.2} kN/m² (Spezifikationszeile E2.12, ungeprüft).", q_fk / 1000.0)));
    row_check(report, annex, ("EN 1991-2", "§5.4", "5.4"), ("footbridges", &bridge.id, "deckWidth"), ("en1991.2.footbridge-horizontal", force(q_flk / 1000.0)), ("Footbridge", "Fußgängerbrücke"), ("Horizontal force Q_flk of the footbridge", "Horizontalkraft Q_flk der Fußgängerbrücke"), (&format!("Q_flk = max(10 % · q_fk L b, 10 kN) = max(0.10 · {:.2} kN/m² · {length:.1} m · {width:.2} m, 10 kN) = {:.1} kN (spec line E2.12, the 10 kN bound is unverified).", q_fk / 1000.0, q_flk / 1000.0), &format!("Q_flk = max(10 % · q_fk L b, 10 kN) = max(0,10 · {:.2} kN/m² · {length:.1} m · {width:.2} m, 10 kN) = {:.1} kN (Spezifikationszeile E2.12, die Grenze 10 kN ist ungeprüft).", q_fk / 1000.0, q_flk / 1000.0)));
    semio_s_artifact_norm_en1991_inferences_rail_silo::check_footbridge_service(bridge, annex, report);
    if bridge.natural_frequency.is_finite() && bridge.natural_frequency > 0.0 {
        let (min, max, bound) = (ndp("ndp.en1991.2.footbridge-walk-f-min"), ndp("ndp.en1991.2.footbridge-walk-f-max"), ndp("ndp.en1991.2.footbridge-band-vertical-max"));
        let (f, required) = (bridge.natural_frequency, part_2::footbridge_comfort_required(bridge.natural_frequency));
        let resonant = required && part_2::footbridge_walking_resonance(f);
        let (en, de) = if required {
            (
                format!("f_v = {f:.2} Hz is below {bound:.1} Hz: the comfort verification of EN 1990 A2.4.3.2(1) with the pedestrian load models is required; f_v is {} the vertical walking range {min:.1} … {max:.1} Hz{} (A2.4.3.2(1) bound and range from registry rows, unverified; the lateral and torsional frequencies below 2.5 Hz are not in the snapshot).", if resonant { "inside" } else { "outside" }, if resonant { ", so the first harmonic of walking is resonant" } else { "" }),
                format!("f_v = {f:.2} Hz liegt unter {bound:.1} Hz: der Komfortnachweis nach EN 1990 A2.4.3.2(1) mit den Fußgängerlastmodellen ist erforderlich; f_v liegt {} des vertikalen Gehfrequenzbereichs {min:.1} … {max:.1} Hz{} (Grenze aus A2.4.3.2(1) und Bereich aus Registerzeilen, ungeprüft; die Quer- und Torsionsfrequenzen unter 2,5 Hz stehen nicht im Snapshot).", if resonant { "innerhalb" } else { "außerhalb" }, if resonant { ", die erste Gehharmonische ist also resonant" } else { "" }),
            )
        } else {
            (
                format!("f_v = {f:.2} Hz is not below {bound:.1} Hz: EN 1990 A2.4.3.2(1) triggers no vertical comfort verification (bound from a registry row, unverified; the lateral and torsional frequencies below 2.5 Hz are not in the snapshot)."),
                format!("f_v = {f:.2} Hz liegt nicht unter {bound:.1} Hz: EN 1990 A2.4.3.2(1) löst keinen vertikalen Komfortnachweis aus (Grenze aus einer Registerzeile, ungeprüft; die Quer- und Torsionsfrequenzen unter 2,5 Hz stehen nicht im Snapshot)."),
            )
        };
        row_check(report, annex, ("EN 1991-2", "§5.7", "5.7"), ("footbridges", &bridge.id, "naturalFrequency"), ("en1991.2.footbridge-walking", Quantity::new(QuantityKind::Dimensionless, f)), ("Footbridge", "Fußgängerbrücke"), ("First vertical frequency against the comfort trigger and the walking range", "Erste vertikale Eigenfrequenz gegen den Komfortauslöser und den Gehfrequenzbereich"), (&en, &de));
    }
}

/// 🚆 One rail span (EN 1991-2, 6.4.5.2): the dynamic factor Φ₂ (careful) or Φ₃ (standard maintenance) of the determinant length L_Φ; an unknown maintenance or a non-positive length is an invalid input without a fallback.
pub fn check_rail_dynamic(span: &semio_s_artifact_norm_en1991_model::RailDynamic, annex: AnnexChoice, report: &mut CheckReport) {
    use semio_s_artifact_norm_en1991_inferences_crane_silo::row_check;
    let clause = || ClauseId::new("EN 1991-2", "§6.4.5.2", "6.4.5.2");
    let valid_length = span.determinant_length.is_finite() && span.determinant_length > 0.0;
    if !valid_length {
        invalid(report, annex, format!("en1991.input.railDynamics.{}.determinantLength", span.id), "DIN EN 1991-2", clause(), &span.id, format!("railDynamics[id={}].determinantLength", span.id), ("determinant length L_Φ (positive)", "maßgebende Länge L_Φ (positiv)"), &span.determinant_length.to_string(), vec!["> 0".into()]);
    }
    let phi = part_2::rail_phi_for(&span.maintenance, if valid_length { span.determinant_length } else { 1.0 });
    let Some(phi) = phi else {
        invalid(report, annex, format!("en1991.input.railDynamics.{}.maintenance", span.id), "DIN EN 1991-2", clause(), &span.id, format!("railDynamics[id={}].maintenance", span.id), ("track maintenance", "Gleisunterhaltung"), &span.maintenance, vec!["careful".into(), "standard".into()]);
        return;
    };
    if !valid_length {
        return;
    }
    let (name, bound) = if span.maintenance == "careful" { ("Φ₂ = 1.44/(√L_Φ − 0.2) + 0.82, 1.00 … 1.67", "Φ₂ = 1,44/(√L_Φ − 0,2) + 0,82, 1,00 … 1,67") } else { ("Φ₃ = 2.16/(√L_Φ − 0.2) + 0.73, 1.00 … 2.00", "Φ₃ = 2,16/(√L_Φ − 0,2) + 0,73, 1,00 … 2,00") };
    row_check(report, annex, ("EN 1991-2", "§6.4.5.2", "6.4.5.2"), ("railDynamics", &span.id, "determinantLength"), ("en1991.2.rail-dynamic", Quantity::new(QuantityKind::Dimensionless, phi)), ("Rail span", "Eisenbahnfeld"), ("Dynamic factor Φ of the rail span", "Dynamischer Beiwert Φ des Eisenbahnfeldes"), (&format!("{name} with L_Φ = {:.1} m ({} maintenance): Φ = {phi:.3} (spec line E2.16).", span.determinant_length, span.maintenance), &format!("{bound} mit L_Φ = {:.1} m ({}): Φ = {phi:.3} (Spezifikationszeile E2.16).", span.determinant_length, if span.maintenance == "careful" { "sorgfältige Unterhaltung" } else { "normale Unterhaltung" })));
}

/// 📐 One wheel load dispersal (EN 1991-2, 4.3.6): the footprint a + 2 (t_p + t_s/2) × b + 2 (t_p + t_s/2) at the mid-plane of the slab that the punching checks read; non-positive contact sides or negative thicknesses are invalid inputs without a fallback.
pub fn check_load_dispersal(row: &semio_s_artifact_norm_en1991_model::LoadDispersal, annex: AnnexChoice, report: &mut CheckReport) {
    use semio_s_artifact_norm_en1991_inferences_crane_silo::row_check;
    let clause = || ClauseId::new("EN 1991-2", "§4.3.6", "4.3.6");
    for (leaf, ok, en, de, found) in [("wheelLength", row.wheel_length.is_finite() && row.wheel_length > 0.0, "wheel contact length (positive)", "Radaufstandslänge (positiv)", row.wheel_length), ("wheelWidth", row.wheel_width.is_finite() && row.wheel_width > 0.0, "wheel contact width (positive)", "Radaufstandsbreite (positiv)", row.wheel_width), ("pavementThickness", row.pavement_thickness.is_finite() && row.pavement_thickness >= 0.0, "pavement thickness (not negative)", "Belagdicke (nicht negativ)", row.pavement_thickness), ("slabThickness", row.slab_thickness.is_finite() && row.slab_thickness >= 0.0, "slab thickness (not negative)", "Plattendicke (nicht negativ)", row.slab_thickness)] {
        if !ok {
            invalid(report, annex, format!("en1991.input.loadDispersals.{}.{leaf}", row.id), "DIN EN 1991-2", clause(), &row.id, format!("loadDispersals[id={}].{leaf}", row.id), (en, de), &found.to_string(), vec!["valid".into()]);
            return;
        }
    }
    let (a, b) = (part_2::dispersed_side_m(row.wheel_length, row.pavement_thickness, row.slab_thickness), part_2::dispersed_side_m(row.wheel_width, row.pavement_thickness, row.slab_thickness));
    row_check(report, annex, ("EN 1991-2", "§4.3.6", "4.3.6"), ("loadDispersals", &row.id, "slabThickness"), ("en1991.2.dispersal-footprint", Quantity::new(QuantityKind::Area, a * b)), ("Wheel load dispersal", "Radlastausbreitung"), ("Dispersed wheel footprint at the slab mid-plane", "Ausgebreitete Radaufstandsfläche in der Plattenmittelebene"), (&format!("Footprint {a:.3} m × {b:.3} m = {:.3} m² from {:.2} m × {:.2} m through t_p = {:.3} m and t_s/2 = {:.3} m at 45° (spec line E2.06, unverified).", a * b, row.wheel_length, row.wheel_width, row.pavement_thickness, 0.5 * row.slab_thickness), &format!("Aufstandsfläche {a:.3} m × {b:.3} m = {:.3} m² aus {:.2} m × {:.2} m durch t_p = {:.3} m und t_s/2 = {:.3} m unter 45° (Spezifikationszeile E2.06, ungeprüft).", a * b, row.wheel_length, row.wheel_width, row.pavement_thickness, 0.5 * row.slab_thickness)));
}
