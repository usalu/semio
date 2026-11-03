//! 🧷 Checks of the four wave 3 collections: road fatigue load models (EN 1991-2, 4.6), vehicle collision on supports (EN 1991-2, 4.7), localised fires (EN 1991-1-2, Annex C) and snow guards (EN 1991-1-3, 6.3).
//! Registry values are unverified and cited by every covering check; a row with an input outside the domain is an invalid input without a fallback.

use super::hazard_checks::{covers, first_non_positive, positive};
use super::*;
use semio_s_artifact_norm_en1991_model::{FatigueModel, LocalFire, SnowGuard, StructureKind, SupportCollision};
use semio_s_plugin_norm_compliance::{ndp_common, AnnexChoice};

fn length(metres: f64) -> Quantity { Quantity::new(QuantityKind::Length, metres) }
fn line_load(n_per_m: f64) -> Quantity { Quantity::new(QuantityKind::Force, n_per_m) }

/// 🩹 Remedy raising an assumed line force (N/m) to the required value.
fn raise_line_load(subject: SubjectRef, current: f64, required: f64) -> Remedy {
    Remedy::at_least(subject, line_load(current), line_load(required), copy(&format!("Raise the assumed line force from {:.3} kN/m to at least {:.3} kN/m.", current / 1000.0, required / 1000.0), &format!("Angenommene Linienkraft von {:.3} kN/m auf mindestens {:.3} kN/m erhöhen.", current / 1000.0, required / 1000.0)))
}

/// 🧷 The row checks of the four collections, in the order of the units of [`En1991Units`].
pub(super) fn check_rows(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport, scope: Scope<'_>) {
    for (_, row) in document.fatigue_models.iter().enumerate().filter(|(i, _)| scope.admits("fatigueModels", *i)) { check_fatigue_model(document, annex, report, row); }
    for (_, row) in document.support_collisions.iter().enumerate().filter(|(i, _)| scope.admits("supportCollisions", *i)) { check_support_collision(document, annex, report, row); }
    for (_, row) in document.local_fires.iter().enumerate().filter(|(i, _)| scope.admits("localFires", *i)) { check_local_fire(annex, report, row); }
    for (_, row) in document.snow_guards.iter().enumerate().filter(|(i, _)| scope.admits("snowGuards", *i)) { check_snow_guard(document, annex, report, row); }
}

/// 🔁 One road fatigue load model: the lane exists, the assumed axle load covers 0.7 Q_ik (FLM1) or the 120 kN axle (FLM3), the assumed distributed load covers 0.3 q_ik (FLM1) and the FLM3 vehicle is reported against the span.
fn check_fatigue_model(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport, row: &FatigueModel) {
    let base = format!("fatigueModels[id={}]", row.id);
    let subject = |leaf: &str| SubjectRef::new(&row.id, &format!("{base}.{leaf}"), copy(&format!("Fatigue model {}", row.id), &format!("Ermüdungsmodell {}", row.id)));
    let clause = || ClauseId::new("EN 1991-2", "§4.6", "4.6");
    if document.structure_kind != StructureKind::Bridge {
        report.push(CheckResult::assess(format!("en1991.2.fatigue-axle.{}", row.id), "DIN EN 1991-2", clause(), subject("model"), copy("Road fatigue load model", "Straßenverkehrs-Ermüdungslastmodell")).not_applicable(copy("Structure kind is building: road fatigue models are not applicable.", "Tragwerksart ist Gebäude: Ermüdungslastmodelle für Straßenverkehr sind nicht anwendbar.")).annex(annex).build());
        return;
    }
    if !(1..=3).contains(&row.lane) {
        invalid(report, annex, format!("en1991.input.fatigueModels.{}.lane", row.id), "DIN EN 1991-2", clause(), &row.id, format!("{base}.lane"), ("lane number (1 to 3)", "Fahrstreifennummer (1 bis 3)"), &row.lane.to_string(), vec!["1".into(), "2".into(), "3".into()]);
        return;
    }
    if !part_2::FATIGUE_MODELS.contains(&row.model.as_str()) {
        invalid(report, annex, format!("en1991.input.fatigueModels.{}.model", row.id), "DIN EN 1991-2", clause(), &row.id, format!("{base}.model"), ("fatigue load model", "Ermüdungslastmodell"), &row.model, part_2::FATIGUE_MODELS.iter().map(|m| m.to_string()).collect());
        return;
    }
    let flm1 = row.model == "flm1";
    let required = if flm1 { part_2::fatigue_flm1_axle_n(annex, row.lane) } else { part_2::fatigue_flm3_axle_n() };
    let (title, steps) = if flm1 {
        (copy("FLM1 axle load 0.7 Q_ik", "FLM1-Achslast 0,7 Q_ik"), copy(&format!("Axle load = 0.7 α_Q{lane} Q_ik = {:.1} kN per axle of lane {lane}; the assumed axle load is {:.1} kN.", required / 1000.0, row.assumed_axle_load / 1000.0, lane = row.lane), &format!("Achslast = 0,7 α_Q{lane} Q_ik = {:.1} kN je Achse des Fahrstreifens {lane}; angesetzt sind {:.1} kN.", required / 1000.0, row.assumed_axle_load / 1000.0, lane = row.lane)))
    } else {
        (copy("FLM3 axle load of the single vehicle", "FLM3-Achslast des Einzelfahrzeugs"), copy(&format!("The FLM3 vehicle has four axles of {:.0} kN; the assumed axle load is {:.1} kN.", required / 1000.0, row.assumed_axle_load / 1000.0), &format!("Das FLM3-Fahrzeug hat vier Achsen mit {:.0} kN; angesetzt sind {:.1} kN.", required / 1000.0, row.assumed_axle_load / 1000.0)))
    };
    let remedy = (row.assumed_axle_load + 1e-9 < required).then(|| raise_force_remedy(&format!("{base}.assumedAxleLoad"), &row.id, "fatigue axle load", "Ermüdungsachslast", row.assumed_axle_load, required));
    report.push(covers(&format!("en1991.2.fatigue-axle.{}", row.id), "DIN EN 1991-2", clause(), subject("assumedAxleLoad"), title, force(row.assumed_axle_load), force(required), steps, annex, remedy));
    if flm1 {
        let udl = part_2::fatigue_flm1_udl_pa(annex, row.lane);
        let remedy = (row.assumed_udl + 1e-9 < udl).then(|| raise_pressure_remedy(&format!("{base}.assumedUdl"), &row.id, "fatigue distributed load", "Ermüdungsflächenlast", row.assumed_udl, udl));
        report.push(covers(&format!("en1991.2.fatigue-udl.{}", row.id), "DIN EN 1991-2", clause(), subject("assumedUdl"), copy("FLM1 distributed load 0.3 q_ik", "FLM1-Flächenlast 0,3 q_ik"), pressure(row.assumed_udl), pressure(udl), copy(&format!("q = 0.3 α_q{lane} q_ik = {:.3} kN/m² on lane {lane}; assumed {:.3} kN/m².", udl / 1000.0, row.assumed_udl / 1000.0, lane = row.lane), &format!("q = 0,3 α_q{lane} q_ik = {:.3} kN/m² auf Fahrstreifen {lane}; angesetzt {:.3} kN/m².", udl / 1000.0, row.assumed_udl / 1000.0, lane = row.lane)), annex, remedy));
    } else {
        let [s1, s2, s3] = part_2::fatigue_flm3_spacings_m();
        let vehicle = part_2::fatigue_flm3_length_m();
        report.push(info_check(format!("en1991.2.fatigue-vehicle.{}", row.id), "DIN EN 1991-2", clause(), subject("model"), copy("FLM3 vehicle length", "Länge des FLM3-Fahrzeugs"), length(vehicle), copy(&format!("Axle spacings {s1:.1} + {s2:.1} + {s3:.1} = {vehicle:.1} m, track {:.1} m, wheel contact {:.2} × {:.2} m; the span is {:.1} m.", part_2::fatigue_flm3_track_m(), part_2::fatigue_flm3_wheel_m(), part_2::fatigue_flm3_wheel_m(), document.bridge_span), &format!("Achsabstände {s1:.1} + {s2:.1} + {s3:.1} = {vehicle:.1} m, Spurweite {:.1} m, Radaufstand {:.2} × {:.2} m; die Spannweite beträgt {:.1} m.", part_2::fatigue_flm3_track_m(), part_2::fatigue_flm3_wheel_m(), part_2::fatigue_flm3_wheel_m(), document.bridge_span), annex));
    }
}

/// 🚛 One vehicle collision on a support or soffit: the position lies on the bridge, the assumed forces cover the registry forces and the height of the line of action is reported against the 0.5 to 1.5 m range of EN 1991-1-7.
fn check_support_collision(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport, row: &SupportCollision) {
    let base = format!("supportCollisions[id={}]", row.id);
    let subject = |leaf: &str| SubjectRef::new(&row.id, &format!("{base}.{leaf}"), copy(&format!("Collision {}", row.id), &format!("Anprall {}", row.id)));
    let clause = || ClauseId::new("EN 1991-2", "§4.7.3.1", "4.7.3.1");
    if document.structure_kind != StructureKind::Bridge {
        report.push(CheckResult::assess(format!("en1991.2.collision-x.{}", row.id), "DIN EN 1991-2", clause(), subject("location"), copy("Vehicle collision on a support", "Fahrzeuganprall an eine Stütze")).not_applicable(copy("Structure kind is building: bridge collision is not applicable.", "Tragwerksart ist Gebäude: Brückenanprall ist nicht anwendbar.")).annex(annex).build());
        return;
    }
    if let Some((leaf, value, label)) = first_non_positive(&[("height", row.height, ("height of the line of action", "Höhe der Wirkungslinie"))]) {
        invalid(report, annex, format!("en1991.input.supportCollisions.{}.{leaf}", row.id), "DIN EN 1991-2", clause(), &row.id, format!("{base}.{leaf}"), label, &value.to_string(), positive());
        return;
    }
    if !(row.position_x.is_finite() && (0.0..=document.bridge_span).contains(&row.position_x)) {
        invalid(report, annex, format!("en1991.input.supportCollisions.{}.positionX", row.id), "DIN EN 1991-2", clause(), &row.id, format!("{base}.positionX"), ("position along the bridge", "Lage entlang der Brücke"), &row.position_x.to_string(), vec!["0".into(), format!("{}", document.bridge_span / 2.0), format!("{}", document.bridge_span)]);
        return;
    }
    if !part_2::COLLISION_LOCATIONS.contains(&row.location.as_str()) {
        invalid(report, annex, format!("en1991.input.supportCollisions.{}.location", row.id), "DIN EN 1991-2", clause(), &row.id, format!("{base}.location"), ("struck part", "getroffenes Bauteil"), &row.location, part_2::COLLISION_LOCATIONS.iter().map(|m| m.to_string()).collect());
        return;
    }
    let soffit = row.location == "soffit";
    let fx = part_2::collision_force_x_n(soffit);
    let remedy = (row.assumed_force_x + 1e-9 < fx).then(|| raise_force_remedy(&format!("{base}.assumedForceX"), &row.id, "collision force in driving direction", "Anprallkraft in Fahrtrichtung", row.assumed_force_x, fx));
    report.push(covers(&format!("en1991.2.collision-x.{}", row.id), "DIN EN 1991-2", clause(), subject("assumedForceX"), copy("Collision force F_dx in driving direction", "Anprallkraft F_dx in Fahrtrichtung"), force(row.assumed_force_x), force(fx), copy(&format!("F_dx = {:.0} kN ({}); the assumed force is {:.0} kN.", fx / 1000.0, if soffit { "soffit" } else { "support at deck level" }, row.assumed_force_x / 1000.0), &format!("F_dx = {:.0} kN ({}); angesetzt sind {:.0} kN.", fx / 1000.0, if soffit { "Unterseite" } else { "Stütze auf Deckebene" }, row.assumed_force_x / 1000.0)), annex, remedy));
    if !soffit {
        let fy = part_2::collision_force_y_n();
        let remedy = (row.assumed_force_y + 1e-9 < fy).then(|| raise_force_remedy(&format!("{base}.assumedForceY"), &row.id, "collision force across the driving direction", "Anprallkraft quer zur Fahrtrichtung", row.assumed_force_y, fy));
        report.push(covers(&format!("en1991.2.collision-y.{}", row.id), "DIN EN 1991-2", clause(), subject("assumedForceY"), copy("Collision force F_dy across the driving direction", "Anprallkraft F_dy quer zur Fahrtrichtung"), force(row.assumed_force_y), force(fy), copy(&format!("F_dy = {:.0} kN; the assumed force is {:.0} kN.", fy / 1000.0, row.assumed_force_y / 1000.0), &format!("F_dy = {:.0} kN; angesetzt sind {:.0} kN.", fy / 1000.0, row.assumed_force_y / 1000.0)), annex, remedy));
    }
    let (low, high) = (ndp_common("ndp.en1991.1-7.impact-height-min"), ndp_common("ndp.en1991.1-7.impact-height-max"));
    let inside = (low..=high).contains(&row.height);
    report.push(info_check(format!("en1991.2.collision-height.{}", row.id), "DIN EN 1991-2", clause(), subject("height"), copy("Height of the line of action", "Höhe der Wirkungslinie"), length(row.height), copy(&format!("The line of action at {:.2} m is {} the range {low:.1} to {high:.1} m.", row.height, if inside { "inside" } else { "outside" }), &format!("Die Wirkungslinie auf {:.2} m liegt {} des Bereichs {low:.1} bis {high:.1} m.", row.height, if inside { "innerhalb" } else { "außerhalb" })), annex));
}

/// 🔥 One localised fire (Annex C): the flame length L_f is always reported; with a ceiling the Hasemi heat flux is checked against the assumed flux when the flame reaches the ceiling, and the plume temperature at the ceiling is reported when it does not.
fn check_local_fire(annex: AnnexChoice, report: &mut CheckReport, row: &LocalFire) {
    let base = format!("localFires[id={}]", row.id);
    let subject = |leaf: &str| SubjectRef::new(&row.id, &format!("{base}.{leaf}"), copy(&format!("Localised fire {}", row.id), &format!("Lokaler Brand {}", row.id)));
    let clause = || ClauseId::new("EN 1991-1-2", "Annex C", "C");
    if let Some((leaf, value, label)) = first_non_positive(&[("fireDiameter", row.fire_diameter, ("fire diameter D", "Branddurchmesser D")), ("heatReleaseRate", row.heat_release_rate, ("heat release rate Q", "Wärmefreisetzungsrate Q"))]) {
        invalid(report, annex, format!("en1991.input.localFires.{}.{leaf}", row.id), "DIN EN 1991-1-2", clause(), &row.id, format!("{base}.{leaf}"), label, &value.to_string(), positive());
        return;
    }
    for (leaf, value, label) in [("ceilingHeight", row.ceiling_height, ("ceiling height H (0 without ceiling)", "Deckenhöhe H (0 ohne Decke)")), ("horizontalDistance", row.horizontal_distance, ("horizontal distance r", "horizontaler Abstand r")), ("assumedFlux", row.assumed_flux, ("assumed heat flux", "angesetzter Wärmestrom"))] {
        if !(value.is_finite() && value >= 0.0) {
            invalid(report, annex, format!("en1991.input.localFires.{}.{leaf}", row.id), "DIN EN 1991-1-2", clause(), &row.id, format!("{base}.{leaf}"), label, &value.to_string(), vec!["0".into(), "1".into(), "3".into()]);
            return;
        }
    }
    let (lf, z0, _) = part_1_2::local_fire(row.fire_diameter, row.heat_release_rate, 1.0);
    report.push(info_check(format!("en1991.1-2.local-flame.{}", row.id), "DIN EN 1991-1-2", clause(), subject("heatReleaseRate"), copy("Flame length L_f", "Flammenlänge L_f"), length(lf.max(0.0)), copy(&format!("L_f = −1.02 D + 0.0148 Q^0.4 = {lf:.2} m for D = {:.2} m and Q = {:.2} MW; the virtual origin is z₀ = {z0:.2} m.", row.fire_diameter, row.heat_release_rate / 1e6), &format!("L_f = −1,02 D + 0,0148 Q^0,4 = {lf:.2} m für D = {:.2} m und Q = {:.2} MW; der virtuelle Ursprung liegt bei z₀ = {z0:.2} m.", row.fire_diameter, row.heat_release_rate / 1e6), annex));
    if row.ceiling_height == 0.0 {
        return;
    }
    if lf >= row.ceiling_height {
        let z_prime = part_1_2::local_fire_virtual_source_m(row.heat_release_rate, row.fire_diameter);
        let l_h = part_1_2::local_fire_horizontal_flame_m(row.heat_release_rate, row.ceiling_height);
        let y = part_1_2::hasemi_y(row.horizontal_distance, row.ceiling_height, z_prime, l_h);
        let flux = part_1_2::hasemi_flux_w_m2(y);
        let remedy = (row.assumed_flux + 1e-9 < flux).then(|| raise_power_remedy(&format!("{base}.assumedFlux"), &row.id, "heat flux", "Wärmestrom", row.assumed_flux, flux));
        report.push(covers(&format!("en1991.1-2.local-flux.{}", row.id), "DIN EN 1991-1-2", clause(), subject("assumedFlux"), copy("Heat flux ĥ of the flame below the ceiling", "Wärmestrom ĥ der Flamme unter der Decke"), Quantity::new(QuantityKind::Power, row.assumed_flux), Quantity::new(QuantityKind::Power, flux), copy(&format!("L_h = {l_h:.2} m, z' = {z_prime:.2} m, y = {y:.3}: ĥ = {:.1} kW/m²; the assumed flux is {:.1} kW/m².", flux / 1000.0, row.assumed_flux / 1000.0), &format!("L_h = {l_h:.2} m, z' = {z_prime:.2} m, y = {y:.3}: ĥ = {:.1} kW/m²; angesetzt sind {:.1} kW/m².", flux / 1000.0, row.assumed_flux / 1000.0)), annex, remedy));
    } else if row.ceiling_height > z0 {
        let (_, _, theta) = part_1_2::local_fire(row.fire_diameter, row.heat_release_rate, row.ceiling_height);
        report.push(info_check(format!("en1991.1-2.local-plume.{}", row.id), "DIN EN 1991-1-2", clause(), subject("ceilingHeight"), copy("Plume temperature at the ceiling", "Plumetemperatur an der Decke"), temp(theta + 273.15), copy(&format!("The flame (L_f = {lf:.2} m) does not reach the ceiling at {:.2} m: θ(H) = {theta:.0} °C (at most 900 °C).", row.ceiling_height), &format!("Die Flamme (L_f = {lf:.2} m) erreicht die Decke auf {:.2} m nicht: θ(H) = {theta:.0} °C (höchstens 900 °C).", row.ceiling_height), annex));
    } else {
        invalid(report, annex, format!("en1991.input.localFires.{}.ceilingHeight", row.id), "DIN EN 1991-1-2", clause(), &row.id, format!("{base}.ceilingHeight"), ("ceiling height above the virtual origin", "Deckenhöhe über dem virtuellen Ursprung"), &row.ceiling_height.to_string(), vec![format!("> {z0:.2}")]);
    }
}

/// 🛡️ One snow guard: its roof exists, the slope length is positive and the assumed line force covers F_s = s b sin α of the roof snow load.
fn check_snow_guard(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport, row: &SnowGuard) {
    let base = format!("snowGuards[id={}]", row.id);
    let subject = |leaf: &str| SubjectRef::new(&row.id, &format!("{base}.{leaf}"), copy(&format!("Snow guard {}", row.id), &format!("Schneefanggitter {}", row.id)));
    let clause = || ClauseId::new("EN 1991-1-3", "§6.3 / §5.3.8", "6.3");
    let Some(roof) = document.roofs.iter().find(|r| r.id == row.roof_id) else {
        invalid(report, annex, format!("en1991.input.snowGuards.{}.roofId", row.id), "DIN EN 1991-1-3", clause(), &row.id, format!("{base}.roofId"), ("roof of the snow guard", "Dach des Schneefanggitters"), &row.roof_id, document.roofs.iter().map(|r| r.id.clone()).collect());
        return;
    };
    if !(row.slope_length.is_finite() && row.slope_length > 0.0) {
        invalid(report, annex, format!("en1991.input.snowGuards.{}.slopeLength", row.id), "DIN EN 1991-1-3", clause(), &row.id, format!("{base}.slopeLength"), ("slope length b", "Dachflächenlänge b"), &row.slope_length.to_string(), positive());
        return;
    }
    let Some(sk) = part_1_3::design_ground_snow_pa(annex, &document.snow_zone, document.altitude, document.en_sk) else { return };
    let s = part_1_3::roof_snow_pa(sk, part_1_3::shape_mu(&roof.roof_type, roof.pitch_deg, roof.has_parapet, roof.parapet_height, roof.drift_obstruction_height, roof.multi_span, sk, annex), roof.c_e, roof.c_t);
    let required = part_1_3::snow_guard_force_n_m(s, row.slope_length, roof.pitch_deg);
    let remedy = (row.assumed_force + 1e-9 < required).then(|| raise_line_load(subject("assumedForce"), row.assumed_force, required));
    report.push(assess_covers(&format!("en1991.1-3.snow-guard.{}", row.id), "DIN EN 1991-1-3", clause(), subject("assumedForce"), copy("Snow guard line force F_s", "Linienkraft F_s des Schneefanggitters"), line_load(row.assumed_force), line_load(required), copy(&format!("F_s = s b sin α = {:.3} kN/m² · {:.1} m · sin {:.1}° = {:.3} kN/m on roof {}; the assumed line force is {:.3} kN/m.", s / 1000.0, row.slope_length, roof.pitch_deg, required / 1000.0, roof.id, row.assumed_force / 1000.0), &format!("F_s = s b sin α = {:.3} kN/m² · {:.1} m · sin {:.1}° = {:.3} kN/m auf Dach {}; angesetzt sind {:.3} kN/m.", s / 1000.0, row.slope_length, roof.pitch_deg, required / 1000.0, roof.id, row.assumed_force / 1000.0)), annex, remedy));
}
