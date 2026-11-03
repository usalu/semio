//! 🌪️ Checks of the six wave 3 row collections: LM3 special vehicles (EN 1991-2, Annex A), the wind force on bridge decks (EN 1991-1-4, section 8), the structural factor c_s c_d and the vortex shedding
//! of slender elements (EN 1991-1-4, section 6 and Annexes B, E), impacts of vehicles, ships and forklifts and internal gas explosions (EN 1991-1-7, section 4 and Annexes C, D) and the uniform and
//! linear temperature actions of bridge sections (EN 1991-1-5, section 6). Every registry value they read is unverified, so the verdicts report the unverified basis.

use super::*;
use semio_s_artifact_norm_en1991_model::{DynamicElement, ImpactAction, InternalExplosion, SpecialVehicle, StructureKind, ThermalSection, WindBridge};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckStatus};

const POSITION_TOLERANCE_M: f64 = 0.05;
const KIND_SHIP: u8 = 1;
const KIND_FORKLIFT: u8 = 2;

fn length(metres: f64) -> Quantity { Quantity::new(QuantityKind::Length, metres) }
fn number(value: f64) -> Quantity { Quantity::new(QuantityKind::Dimensionless, value) }
fn energy(joule: f64) -> Quantity { Quantity::new(QuantityKind::Energy, joule) }

/// 🔖 Covering check that cites every registry value read since the previous check.
#[allow(clippy::too_many_arguments)]
fn covers(id: &str, part: &str, clause: ClauseId, subject: SubjectRef, title: LocalizedCopy, assumed: Quantity, required: Quantity, explanation: LocalizedCopy, annex: AnnexChoice, remedy: Option<Remedy>, ndps: &[&str]) -> CheckResult {
    let mut cited: Vec<&str> = semio_s_plugin_norm_compliance::drain_ndp_reads();
    cited.extend_from_slice(ndps);
    assess_covers_on(id, part, clause, subject, title, assumed, required, explanation, annex, remedy, &cited)
}

/// ℹ️ Informative result that cites every registry value read since the previous check.
fn info(report: &mut CheckReport, annex: AnnexChoice, id: String, part: &str, clause: ClauseId, subject: SubjectRef, title: LocalizedCopy, quantity: Quantity, explanation: LocalizedCopy) {
    report.push(info_check(id, part, clause, subject, title, quantity, explanation, annex));
}

/// 🚫 Result for a collection row whose inputs the check cannot evaluate; nothing is replaced by an invented default.
fn unavailable(report: &mut CheckReport, annex: AnnexChoice, id: String, part: &str, clause: ClauseId, subject: SubjectRef, title: LocalizedCopy, reason: LocalizedCopy) {
    report.push(CheckResult::assess(id, part, clause, subject, title).not_applicable(reason).annex(annex).build());
}

/// 📏 Remedy raising an assumed line force (N/m) to the required value.
fn raise_line_force(subject: SubjectRef, current: f64, required: f64, label_en: &str, label_de: &str) -> Remedy {
    Remedy::at_least(subject, Quantity::new(QuantityKind::Force, current), Quantity::new(QuantityKind::Force, required), copy(&format!("Raise the assumed {label_en} from {:.3} kN/m to at least {:.3} kN/m.", current / 1000.0, required / 1000.0), &format!("Angenommene {label_de} von {:.3} kN/m auf mindestens {:.3} kN/m erhöhen.", current / 1000.0, required / 1000.0)))
}

/// 📍 Position check: the deviation of a coordinate from its nearest admissible value against the tolerance.
#[allow(clippy::too_many_arguments)]
fn deviation(id: &str, clause: ClauseId, subject: SubjectRef, title: LocalizedCopy, value: f64, target: f64, explanation: LocalizedCopy, annex: AnnexChoice, remedy: Option<Remedy>) -> CheckResult {
    let mut b = CheckResult::assess(id, "DIN EN 1991-2", clause, subject, title).utilization(length((value - target).abs()), length(POSITION_TOLERANCE_M)).warn_near_limit().annex(annex).explanation(explanation);
    if let Some(r) = remedy { b = b.remedy(r); }
    b.build()
}

/// 🔢 Representative positive values offered as remedy options for a non-positive input.
fn positive() -> Vec<String> { vec!["1".into(), "10".into(), "100".into()] }

/// 🌪️ The row checks of the six collections, in the order of the units of [`En1991Units`].
pub(super) fn check_rows(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport, scope: Scope<'_>) {
    for (_, row) in document.special_vehicles.iter().enumerate().filter(|(index, _)| scope.admits("specialVehicles", *index)) {
        check_special_vehicle(document, annex, report, row);
    }
    for (_, row) in document.wind_bridges.iter().enumerate().filter(|(index, _)| scope.admits("windBridges", *index)) {
        check_wind_bridge(document, annex, report, row);
    }
    for (_, row) in document.dynamic_elements.iter().enumerate().filter(|(index, _)| scope.admits("dynamicElements", *index)) {
        check_dynamic_element(document, annex, report, row);
    }
    for (_, row) in document.impact_actions.iter().enumerate().filter(|(index, _)| scope.admits("impactActions", *index)) {
        check_impact_action(annex, report, row);
    }
    for (_, row) in document.internal_explosions.iter().enumerate().filter(|(index, _)| scope.admits("internalExplosions", *index)) {
        check_internal_explosion(annex, report, row);
    }
    for (_, row) in document.thermal_sections.iter().enumerate().filter(|(index, _)| scope.admits("thermalSections", *index)) {
        check_thermal_section(document, annex, report, row);
    }
}

/// 🚛 One LM3 special vehicle: the designation is known, the assumed LM3 axle line covers its axle line, it fits the span and it stands inside the carriageway.
fn check_special_vehicle(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport, vehicle: &SpecialVehicle) {
    let base = format!("specialVehicles[id={}]", vehicle.id);
    let subject = |leaf: &str| SubjectRef::new(&vehicle.id, &format!("{base}.{leaf}"), copy(&format!("Special vehicle {}", vehicle.id), &format!("Sonderfahrzeug {}", vehicle.id)));
    let clause = |locator: &str| ClauseId::new("EN 1991-2", locator, "4.3.4");
    if document.structure_kind != StructureKind::Bridge {
        unavailable(report, annex, format!("en1991.2.lm3-vehicle.{}.none", vehicle.id), "DIN EN 1991-2", clause("§4.3.4"), subject("designation"), copy("LM3 special vehicle", "LM3-Sonderfahrzeug"), copy("Structure kind is building — special vehicles not applicable.", "Tragwerksart ist Gebäude — Sonderfahrzeuge nicht anwendbar."));
        return;
    }
    let Some((total, axle, lines)) = part_2::special_vehicle(&vehicle.designation) else {
        invalid(report, annex, format!("en1991.input.specialVehicles.{}.designation", vehicle.id), "DIN EN 1991-2", clause("Annex A"), &vehicle.id, format!("{base}.designation"), ("special vehicle designation total/axle in kN", "Sonderfahrzeug-Bezeichnung Gesamt/Achse in kN"), &vehicle.designation, part_2::SPECIAL_VEHICLES.iter().map(|d| d.to_string()).collect());
        return;
    };
    let axle_remedy = (document.assumed_bridge_lm3 + 1e-9 < axle).then(|| raise_force_remedy("assumedBridgeLm3", "", "LM3 special vehicle axle", "LM3-Sonderfahrzeugachse", document.assumed_bridge_lm3, axle));
    report.push(covers(&format!("en1991.2.lm3-vehicle.{}", vehicle.id), "DIN EN 1991-2", clause("Annex A"), subject("designation"), copy("LM3 axle line of the special vehicle", "LM3-Achslinie des Sonderfahrzeugs"), force(document.assumed_bridge_lm3), force(axle), copy(&format!("Vehicle {}: {} axle lines of {:.0} kN, {:.0} kN in total; assumed LM3 axle line {:.0} kN.", vehicle.designation, lines, axle / 1000.0, total / 1000.0, document.assumed_bridge_lm3 / 1000.0), &format!("Fahrzeug {}: {} Achslinien zu {:.0} kN, insgesamt {:.0} kN; angenommene LM3-Achslinie {:.0} kN.", vehicle.designation, lines, axle / 1000.0, total / 1000.0, document.assumed_bridge_lm3 / 1000.0)), annex, axle_remedy, &["ndp.en1991.2.lm3-vehicle-width"]));
    let vehicle_length = part_2::special_vehicle_length_m(lines);
    let span = document.bridge_span;
    let length_remedy = (vehicle_length > span + POSITION_TOLERANCE_M).then(|| {
        let options: Vec<String> = part_2::SPECIAL_VEHICLES.iter().filter(|d| part_2::special_vehicle(d).is_some_and(|(_, _, n)| part_2::special_vehicle_length_m(n) <= span)).map(|d| d.to_string()).collect();
        Remedy::one_of(subject("designation"), options, copy("Choose a shorter special vehicle.", "Ein kürzeres Sonderfahrzeug wählen."))
    });
    report.push(covers(&format!("en1991.2.lm3-vehicle-length.{}", vehicle.id), "DIN EN 1991-2", clause("Annex A"), subject("designation"), copy("Special vehicle fits the span", "Sonderfahrzeug passt in die Spannweite"), length(span), length(vehicle_length), copy(&format!("Vehicle length {vehicle_length:.2} m ({lines} axle lines at {:.2} m) against the span {span:.2} m.", part_2::lm3_axle_spacing_m()), &format!("Fahrzeuglänge {vehicle_length:.2} m ({lines} Achslinien im Abstand {:.2} m) gegen die Spannweite {span:.2} m.", part_2::lm3_axle_spacing_m())), annex, length_remedy, &[]));
    let half = vehicle_length / 2.0;
    let x_target = vehicle.x.clamp(half.min(span / 2.0), (span - half).max(span / 2.0));
    let x_remedy = ((vehicle.x - x_target).abs() > POSITION_TOLERANCE_M).then(|| Remedy::exactly(subject("x"), length(vehicle.x), length(x_target), copy(&format!("Move the vehicle centre to x = {x_target:.2} m so that the vehicle stands on the span."), &format!("Fahrzeugmitte auf x = {x_target:.2} m verschieben, damit das Fahrzeug auf der Spannweite steht."))));
    report.push(deviation(&format!("en1991.2.lm3-vehicle-position.{}", vehicle.id), clause("Annex A"), subject("x"), copy("Special vehicle stands on the span", "Sonderfahrzeug steht auf der Spannweite"), vehicle.x, x_target, copy(&format!("Vehicle centre {:.2} m, half length {half:.2} m, span {span:.2} m.", vehicle.x), &format!("Fahrzeugmitte {:.2} m, halbe Länge {half:.2} m, Spannweite {span:.2} m.", vehicle.x)), annex, x_remedy));
    let half_width = part_2::lm3_vehicle_width_m() / 2.0;
    let carriageway = document.bridge_lane_width.max(0.0);
    let y_target = vehicle.y.clamp(half_width.min(carriageway / 2.0), (carriageway - half_width).max(carriageway / 2.0));
    let y_remedy = ((vehicle.y - y_target).abs() > POSITION_TOLERANCE_M).then(|| Remedy::exactly(subject("y"), length(vehicle.y), length(y_target), copy(&format!("Move the vehicle axis to y = {y_target:.2} m so that it stands inside the carriageway."), &format!("Fahrzeugachse auf y = {y_target:.2} m verschieben, damit es in der Fahrbahn steht."))));
    report.push(deviation(&format!("en1991.2.lm3-vehicle-lateral.{}", vehicle.id), clause("Annex A"), subject("y"), copy("Special vehicle stands inside the carriageway", "Sonderfahrzeug steht in der Fahrbahn"), vehicle.y, y_target, copy(&format!("Vehicle axis {:.2} m from the kerb, half width {half_width:.2} m, carriageway {carriageway:.2} m.", vehicle.y), &format!("Fahrzeugachse {:.2} m vom Bordstein, halbe Breite {half_width:.2} m, Fahrbahn {carriageway:.2} m.", vehicle.y)), annex, y_remedy));
}

/// 🌉 One bridge wind case: depth d_tot, force coefficient c_f,x and the horizontal, vertical and longitudinal wind force per metre of the deck (EN 1991-1-4, 8.3).
fn check_wind_bridge(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport, row: &WindBridge) {
    let base = format!("windBridges[id={}]", row.id);
    let subject = |leaf: &str| SubjectRef::new(&row.id, &format!("{base}.{leaf}"), copy(&format!("Bridge wind case {}", row.id), &format!("Brückenwindfall {}", row.id)));
    let clause = |locator: &str| ClauseId::new("EN 1991-1-4", locator, "8");
    let id = |stem: &str| format!("en1991.1-4.{stem}.{}", row.id);
    let Some(depth_tot) = part_1_4::bridge_depth_m(row.deck_depth, row.parapet_kind, row.parapet_height, row.traffic_kind) else {
        let (leaf, label, found, options) = if row.parapet_kind > 2 { ("parapetKind", ("parapet kind", "Geländerart"), row.parapet_kind, vec!["0", "1", "2"]) } else { ("trafficKind", ("traffic kind", "Verkehrsart"), row.traffic_kind, vec!["0", "1", "2"]) };
        invalid(report, annex, format!("en1991.input.windBridges.{}.{leaf}", row.id), "DIN EN 1991-1-4", clause("Table 8.1"), &row.id, format!("{base}.{leaf}"), label, &found.to_string(), options.into_iter().map(String::from).collect());
        return;
    };
    for (leaf, value, label) in [("deckWidth", row.deck_width, ("deck width", "Überbaubreite")), ("deckDepth", row.deck_depth, ("deck depth", "Überbauhöhe")), ("span", row.span, ("loaded length", "belastete Länge")), ("heightAboveGround", row.height_above_ground, ("height above ground", "Höhe über Grund"))] {
        if !(value.is_finite() && value > 0.0) {
            invalid(report, annex, format!("en1991.input.windBridges.{}.{leaf}", row.id), "DIN EN 1991-1-4", clause("§8.3"), &row.id, format!("{base}.{leaf}"), label, &format!("{value}"), positive());
            return;
        }
    }
    let Some(vb) = part_1_4::design_vb(annex, document.wind_zone, document.en_vb) else {
        unavailable(report, annex, id("bridge-wind"), "DIN EN 1991-1-4", clause("§8.3"), subject("heightAboveGround"), copy("Bridge wind force", "Brückenwindkraft"), copy("The basic wind velocity is not defined for the wind zone.", "Die Basiswindgeschwindigkeit ist für die Windzone nicht definiert."));
        return;
    };
    let Some(qp) = part_1_4::peak_velocity_pressure_pa(annex, document.wind_zone, document.terrain_category, row.height_above_ground, vb, document.air_density, document.orography_factor, document.mixed_terrain_upwind, document.mixed_terrain_distance, document.coast_or_island) else {
        unavailable(report, annex, id("bridge-wind"), "DIN EN 1991-1-4", clause("§8.3"), subject("heightAboveGround"), copy("Bridge wind force", "Brückenwindkraft"), copy("The terrain category has no peak velocity pressure.", "Die Geländekategorie hat keinen Böengeschwindigkeitsdruck."));
        return;
    };
    let ratio = row.deck_width / depth_tot;
    let c_fx = part_1_4::bridge_c_fx0(row.deck_width, depth_tot);
    let fx = part_1_4::bridge_force_x_n_m(qp, c_fx, depth_tot);
    let remedy = (row.assumed_force + 1e-9 < fx).then(|| raise_line_force(subject("assumedForce"), row.assumed_force, fx, "wind force F_W per metre", "Windkraft F_W je Meter"));
    report.push(covers(&id("bridge-wind"), "DIN EN 1991-1-4", clause("§8.3.2 (8.1)"), subject("assumedForce"), copy("Horizontal wind force F_W,x per metre of the deck", "Horizontale Windkraft F_W,x je Meter Überbau"), Quantity::new(QuantityKind::Force, row.assumed_force), Quantity::new(QuantityKind::Force, fx), copy(&format!("d_tot = {depth_tot:.2} m, b/d_tot = {ratio:.2}, c_f,x0 = {c_fx:.3}, q_p({:.1} m) = {:.3} kN/m²: F_W,x = q_p c_f,x d_tot = {:.3} kN/m; assumed {:.3} kN/m.", row.height_above_ground, qp / 1000.0, fx / 1000.0, row.assumed_force / 1000.0), &format!("d_tot = {depth_tot:.2} m, b/d_tot = {ratio:.2}, c_f,x0 = {c_fx:.3}, q_p({:.1} m) = {:.3} kN/m²: F_W,x = q_p c_f,x d_tot = {:.3} kN/m; angenommen {:.3} kN/m.", row.height_above_ground, qp / 1000.0, fx / 1000.0, row.assumed_force / 1000.0)), annex, remedy, &["ndp.en1991.e14-13.1"]));
    let fz = part_1_4::bridge_force_z_n_m(qp, row.deck_width);
    info(report, annex, id("bridge-wind-vertical"), "DIN EN 1991-1-4", clause("§8.3.3"), subject("deckWidth"), copy("Vertical wind force F_W,z per metre", "Vertikale Windkraft F_W,z je Meter"), Quantity::new(QuantityKind::Force, fz), copy(&format!("F_W,z = q_p c_f,z b = {:.3} kN/m, upwards or downwards.", fz / 1000.0), &format!("F_W,z = q_p c_f,z b = {:.3} kN/m, nach oben oder unten.", fz / 1000.0)));
    let fy = part_1_4::bridge_force_y_n_m(fx);
    info(report, annex, id("bridge-wind-longitudinal"), "DIN EN 1991-1-4", clause("§8.3.4"), subject("assumedForce"), copy("Longitudinal wind force F_W,y per metre", "Längswindkraft F_W,y je Meter"), Quantity::new(QuantityKind::Force, fy), copy(&format!("F_W,y = {:.3} kN/m, a share of F_W,x.", fy / 1000.0), &format!("F_W,y = {:.3} kN/m, ein Anteil von F_W,x.", fy / 1000.0)));
    let reaction = fx * row.span / 2.0;
    info(report, annex, id("bridge-wind-reaction"), "DIN EN 1991-1-4", clause("§8.3.2"), subject("span"), copy("Horizontal bearing reaction of the wind", "Horizontale Lagerreaktion aus Wind"), force(reaction), copy(&format!("R = F_W,x L/2 = {:.1} kN per support for L = {:.1} m.", reaction / 1000.0, row.span), &format!("R = F_W,x L/2 = {:.1} kN je Auflager für L = {:.1} m.", reaction / 1000.0, row.span)));
}

/// 〰️ One slender element: the structural factor c_s c_d (Annex B), the natural frequency and the vortex shedding relevance, Scruton and Reynolds numbers (Annex E).
fn check_dynamic_element(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport, row: &DynamicElement) {
    let base = format!("dynamicElements[id={}]", row.id);
    let subject = |leaf: &str| SubjectRef::new(&row.id, &format!("{base}.{leaf}"), copy(&format!("Dynamic element {}", row.id), &format!("Dynamisches Bauteil {}", row.id)));
    let clause = |locator: &str| ClauseId::new("EN 1991-1-4", locator, "6");
    let id = |stem: &str| format!("en1991.1-4.{stem}.{}", row.id);
    for (leaf, value, label) in [("height", row.height, ("height", "Höhe")), ("width", row.width, ("width", "Breite")), ("logDecrement", row.log_decrement, ("logarithmic decrement", "logarithmisches Dekrement")), ("strouhal", row.strouhal, ("Strouhal number", "Strouhal-Zahl")), ("massPerLength", row.mass_per_length, ("mass per length", "Masse je Länge"))] {
        if !(value.is_finite() && value > 0.0) {
            invalid(report, annex, format!("en1991.input.dynamicElements.{}.{leaf}", row.id), "DIN EN 1991-1-4", clause("§6.3"), &row.id, format!("{base}.{leaf}"), label, &format!("{value}"), positive());
            return;
        }
    }
    let (Some(vb), Some(terrain)) = (part_1_4::design_vb(annex, document.wind_zone, document.en_vb), part_1_4::TerrainCategory::from_u8(document.terrain_category)) else {
        unavailable(report, annex, id("dyn-cscd"), "DIN EN 1991-1-4", clause("§6.3"), subject("assumedCscd"), copy("Structural factor c_s c_d", "Strukturbeiwert c_s c_d"), copy("The wind zone or the terrain category is not defined.", "Die Windzone oder die Geländekategorie ist nicht definiert."));
        return;
    };
    let n1 = if row.first_frequency > 0.0 { row.first_frequency } else { 46.0 / row.height };
    let simplified = part_1_4::cscd_simplified(row.height);
    let factors = part_1_4::structural_factor(row.height, row.width, terrain, vb, row.log_decrement, n1);
    let required = if simplified { 1.0 } else { factors[8] };
    let remedy = (row.assumed_cscd + 1e-9 < required).then(|| Remedy::at_least(subject("assumedCscd"), number(row.assumed_cscd), number(required), copy(&format!("Raise the assumed c_s c_d from {:.3} to at least {required:.3}.", row.assumed_cscd), &format!("Angenommenes c_s c_d von {:.3} auf mindestens {required:.3} erhöhen.", row.assumed_cscd))));
    let steps_en = if simplified { format!("h = {:.1} m is below the limit of the simplified procedure: c_s c_d = 1.", row.height) } else { format!("z_s = {:.2} m, B² = {:.4}, R² = {:.4}, ν = {:.3} Hz, k_p = {:.3}: c_s c_d = {:.4} (n₁ = {n1:.3} Hz, δ = {:.3}).", factors[0], factors[2], factors[5], factors[6], factors[7], factors[8], row.log_decrement) };
    let steps_de = if simplified { format!("h = {:.1} m liegt unter der Grenze des vereinfachten Verfahrens: c_s c_d = 1.", row.height) } else { format!("z_s = {:.2} m, B² = {:.4}, R² = {:.4}, ν = {:.3} Hz, k_p = {:.3}: c_s c_d = {:.4} (n₁ = {n1:.3} Hz, δ = {:.3}).", factors[0], factors[2], factors[5], factors[6], factors[7], factors[8], row.log_decrement) };
    report.push(covers(&id("dyn-cscd"), "DIN EN 1991-1-4", clause("§6.3 (B.2)"), subject("assumedCscd"), copy("Structural factor c_s c_d", "Strukturbeiwert c_s c_d"), number(row.assumed_cscd), number(required), copy(&steps_en, &steps_de), annex, remedy, &["ndp.en1991.e14-12.1"]));
    info(report, annex, id("dyn-frequency"), "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "Annex F", "F"), subject("firstFrequency"), copy("First natural frequency n₁ (Hz)", "Erste Eigenfrequenz n₁ (Hz)"), number(n1), copy(&format!("n₁ = {n1:.3} Hz{}.", if row.first_frequency > 0.0 { "" } else { " from n₁ = 46/h" }), &format!("n₁ = {n1:.3} Hz{}.", if row.first_frequency > 0.0 { "" } else { " aus n₁ = 46/h" })));
    let v_crit = part_1_4::vortex_critical_velocity(row.width, n1, row.strouhal);
    let v_m = part_1_4::mean_wind_velocity(row.height, terrain, vb, document.orography_factor);
    let onset = part_1_4::vortex_onset_velocity(v_m);
    let mut cited: Vec<&str> = semio_s_plugin_norm_compliance::drain_ndp_reads();
    cited.push("ndp.en1991.e14-14.1");
    let mut vortex = CheckResult::assess(id("vortex-shedding"), "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "Annex E.1.3", "E"), subject("strouhal"), copy("Vortex shedding relevance v_crit ≤ 1.25 v_m", "Relevanz der Wirbelerregung v_crit ≤ 1,25 v_m"))
        .utilization(number(onset), number(v_crit))
        .status(CheckStatus::Pass)
        .warn_above(1.0)
        .annex(annex)
        .explanation(copy(&format!("v_crit = b n₁/St = {v_crit:.2} m/s, v_m(h) = {v_m:.2} m/s: 1.25 v_m = {onset:.2} m/s; the amplitude of Annex E.1.5 is not evaluated.", ), &format!("v_crit = b n₁/St = {v_crit:.2} m/s, v_m(h) = {v_m:.2} m/s: 1,25 v_m = {onset:.2} m/s; die Amplitude nach Anhang E.1.5 wird nicht ausgewertet.")))
        .ndps(&cited);
    if onset > v_crit && n1 > 0.0 && row.strouhal > 0.0 {
        let width_required = onset * row.strouhal / n1;
        vortex = vortex.remedy(Remedy::at_least(subject("width"), length(row.width), length(width_required), copy(&format!("Widen the element from {:.3} m to at least {width_required:.3} m so that v_crit exceeds 1.25 v_m, or stiffen it to raise n₁.", row.width), &format!("Das Element von {:.3} m auf mindestens {width_required:.3} m verbreitern, damit v_crit über 1,25 v_m liegt, oder aussteifen, um n₁ zu erhöhen.", row.width))));
    }
    report.push(vortex.build());
    let sc = part_1_4::scruton(row.log_decrement, row.mass_per_length, document.air_density, row.width);
    info(report, annex, id("vortex-scruton"), "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "Annex E.1.3.3", "E"), subject("massPerLength"), copy("Scruton number Sc", "Scruton-Zahl Sc"), number(sc), copy(&format!("Sc = 2 δ_s m_e/(ρ b²) = {sc:.2}."), &format!("Sc = 2 δ_s m_e/(ρ b²) = {sc:.2}.")));
    let re = part_1_4::reynolds(row.width, v_crit);
    info(report, annex, id("vortex-reynolds"), "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "Annex E.1.3.1", "E"), subject("width"), copy("Reynolds number at v_crit", "Reynolds-Zahl bei v_crit"), number(re), copy(&format!("Re = b v_crit/ν = {re:.3e}."), &format!("Re = b v_crit/ν = {re:.3e}.")));
}

/// 💥 One impact: the equivalent static force F_d,x = v √(k m) (vehicles, ships, Annex C) or the forklift weight multiple, its transverse share and its energy.
fn check_impact_action(annex: AnnexChoice, report: &mut CheckReport, row: &ImpactAction) {
    let base = format!("impactActions[id={}]", row.id);
    let subject = |leaf: &str| SubjectRef::new(&row.id, &format!("{base}.{leaf}"), copy(&format!("Impact {}", row.id), &format!("Anprall {}", row.id)));
    let clause = |locator: &str| ClauseId::new("EN 1991-1-7", locator, "4");
    let id = |stem: &str| format!("en1991.1-7.{stem}.{}", row.id);
    if row.kind > KIND_FORKLIFT {
        invalid(report, annex, format!("en1991.input.impactActions.{}.kind", row.id), "DIN EN 1991-1-7", clause("§4"), &row.id, format!("{base}.kind"), ("impact kind (0 vehicle, 1 ship, 2 forklift)", "Anprallart (0 Fahrzeug, 1 Schiff, 2 Gabelstapler)"), &row.kind.to_string(), vec!["0".into(), "1".into(), "2".into()]);
        return;
    }
    for (leaf, value, label) in [("mass", row.mass, ("mass", "Masse")), ("speed", if row.kind == KIND_FORKLIFT { 1.0 } else { row.speed }, ("speed", "Geschwindigkeit"))] {
        if !(value.is_finite() && value > 0.0) {
            invalid(report, annex, format!("en1991.input.impactActions.{}.{leaf}", row.id), "DIN EN 1991-1-7", clause("Annex C"), &row.id, format!("{base}.{leaf}"), label, &format!("{value}"), positive());
            return;
        }
    }
    if row.kind == KIND_SHIP && row.stiffness <= 0.0 {
        unavailable(report, annex, id("impact-force"), "DIN EN 1991-1-7", clause("Annex C.4"), subject("stiffness"), copy("Ship impact force", "Schiffsanprallkraft"), copy("Ship impact needs the equivalent stiffness k of the structure; the registry holds no default for ships.", "Der Schiffsanprall braucht die Ersatzsteifigkeit k des Tragwerks; im Register steht kein Standardwert für Schiffe."));
        return;
    }
    let k = part_1_7::impact_stiffness_n_m(row.stiffness);
    let fx = if row.kind == KIND_FORKLIFT { part_1_7::forklift_force_n(row.mass) } else { part_1_7::dynamic_impact_force_n(row.mass, row.speed, k) };
    let (title_en, title_de, steps_en, steps_de) = if row.kind == KIND_FORKLIFT { ("Forklift impact force F_d,x = 5 W", "Gabelstapleranprall F_d,x = 5 W", format!("F_d,x = factor·m g = {:.1} kN for m = {:.0} kg.", fx / 1000.0, row.mass), format!("F_d,x = Faktor·m g = {:.1} kN für m = {:.0} kg.", fx / 1000.0, row.mass)) } else { ("Impact force F_d,x = v √(k m)", "Anprallkraft F_d,x = v √(k m)", format!("v = {:.2} m/s, m = {:.0} kg, k = {:.0} kN/m: F_d,x = {:.1} kN; assumed {:.1} kN.", row.speed, row.mass, k / 1000.0, fx / 1000.0, row.assumed_force / 1000.0), format!("v = {:.2} m/s, m = {:.0} kg, k = {:.0} kN/m: F_d,x = {:.1} kN; angenommen {:.1} kN.", row.speed, row.mass, k / 1000.0, fx / 1000.0, row.assumed_force / 1000.0)) };
    let remedy = (row.assumed_force + 1e-9 < fx).then(|| raise_force_remedy(&format!("{base}.assumedForce"), &row.id, "impact force", "Anprallkraft", row.assumed_force, fx));
    report.push(covers(&id("impact-force"), "DIN EN 1991-1-7", clause("Annex C"), subject("assumedForce"), copy(title_en, title_de), force(row.assumed_force), force(fx), copy(&steps_en, &steps_de), annex, remedy, &["ndp.en1991.e17.impact"]));
    if row.kind != KIND_SHIP {
        let fy = part_1_7::lateral_impact_force_n(fx);
        info(report, annex, id("impact-lateral"), "DIN EN 1991-1-7", clause("Table 4.1"), subject("assumedForce"), copy("Transverse impact force F_d,y", "Quer wirkende Anprallkraft F_d,y"), force(fy), copy(&format!("F_d,y = {:.1} kN, a share of F_d,x.", fy / 1000.0), &format!("F_d,y = {:.1} kN, ein Anteil von F_d,x.", fy / 1000.0)));
    }
    if row.kind != KIND_FORKLIFT {
        let e = part_1_7::impact_energy_j(row.mass, row.speed);
        info(report, annex, id("impact-energy"), "DIN EN 1991-1-7", clause("Annex C"), subject("speed"), copy("Kinetic impact energy ½ m v²", "Kinetische Anprallenergie ½ m v²"), energy(e), copy(&format!("E = {:.1} kJ.", e / 1000.0), &format!("E = {:.1} kJ.", e / 1000.0)));
    }
}

/// 🏠 One internal gas explosion: the equivalent static design pressure p_d of Annex D.1 and D.2 against the assumed pressure, and the validity range of the enclosure volume.
fn check_internal_explosion(annex: AnnexChoice, report: &mut CheckReport, row: &InternalExplosion) {
    let base = format!("internalExplosions[id={}]", row.id);
    let subject = |leaf: &str| SubjectRef::new(&row.id, &format!("{base}.{leaf}"), copy(&format!("Explosion {}", row.id), &format!("Explosion {}", row.id)));
    let clause = |locator: &str| ClauseId::new("EN 1991-1-7", locator, "D");
    let id = |stem: &str| format!("en1991.1-7.{stem}.{}", row.id);
    for (leaf, value, label) in [("volume", row.volume, ("volume", "Volumen")), ("ventArea", row.vent_area, ("vent area", "Entlastungsfläche"))] {
        if !(value.is_finite() && value > 0.0) {
            invalid(report, annex, format!("en1991.input.internalExplosions.{}.{leaf}", row.id), "DIN EN 1991-1-7", clause("Annex D"), &row.id, format!("{base}.{leaf}"), label, &format!("{value}"), positive());
            return;
        }
    }
    let av_over_v = row.vent_area / row.volume;
    let p_v = row.vent_pressure / 1000.0;
    let required = part_1_7::gas_explosion_governing_kn_m2(p_v, av_over_v) * 1000.0;
    let remedy = (row.assumed_pressure + 1e-9 < required).then(|| raise_pressure_remedy(&format!("{base}.assumedPressure"), &row.id, "explosion design pressure", "Explosions-Bemessungsdruck", row.assumed_pressure, required));
    report.push(covers(&id("gas-explosion"), "DIN EN 1991-1-7", clause("Annex D (D.1, D.2)"), subject("assumedPressure"), copy("Internal explosion design pressure p_d", "Bemessungsdruck der inneren Explosion p_d"), pressure(row.assumed_pressure), pressure(required), copy(&format!("A_v/V = {av_over_v:.4} 1/m, p_v = {p_v:.2} kN/m²: D.1 {:.2} kN/m², D.2 {:.2} kN/m², governing {:.2} kN/m².", part_1_7::gas_explosion_pd_static_kn_m2(p_v), part_1_7::gas_explosion_pd_kn_m2(p_v, av_over_v), required / 1000.0), &format!("A_v/V = {av_over_v:.4} 1/m, p_v = {p_v:.2} kN/m²: D.1 {:.2} kN/m², D.2 {:.2} kN/m², maßgebend {:.2} kN/m².", part_1_7::gas_explosion_pd_static_kn_m2(p_v), part_1_7::gas_explosion_pd_kn_m2(p_v, av_over_v), required / 1000.0)), annex, remedy, &["ndp.en1991.e17.blast"]));
    let in_domain = part_1_7::gas_explosion_in_domain(row.volume);
    let (v_min, v_max) = (semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.1-7.gas-volume-min"), semio_s_plugin_norm_compliance::ndp_common("ndp.en1991.1-7.gas-volume-max"));
    let mut cited: Vec<&str> = semio_s_plugin_norm_compliance::drain_ndp_reads();
    cited.push("ndp.en1991.e17.blast");
    let mut domain = CheckResult::assess(id("gas-explosion-domain"), "DIN EN 1991-1-7", clause("Annex D"), subject("volume"), copy("Volume inside the validity range of Annex D", "Volumen im Gültigkeitsbereich von Anhang D"))
        .utilization(number((v_min / row.volume).max(row.volume / v_max)), number(1.0))
        .status(CheckStatus::Pass)
        .warn_above(1.0)
        .annex(annex)
        .explanation(copy(&format!("V = {:.1} m³, validity range {v_min:.0} to {v_max:.0} m³ ({}).", row.volume, if in_domain { "inside" } else { "outside" }), &format!("V = {:.1} m³, Gültigkeitsbereich {v_min:.0} bis {v_max:.0} m³ ({}).", row.volume, if in_domain { "innerhalb" } else { "außerhalb" })))
        .ndps(&cited);
    if !in_domain {
        let clamped = row.volume.clamp(v_min, v_max);
        domain = domain.remedy(Remedy::exactly(
            subject("volume"),
            Quantity::new(QuantityKind::Volume, row.volume),
            Quantity::new(QuantityKind::Volume, clamped),
            copy(
                &format!("Annex D gives p_d only for enclosure volumes of {v_min:.0} to {v_max:.0} m³; this edit sets the volume of the modelled enclosure from {:.1} m³ to {clamped:.1} m³, the nearest value inside the range. It models a subdivided or merged compartment and does not change the real room: apply it only when the compartment is really divided or merged; otherwise assess the explosion with a method outside Annex D.", row.volume),
                &format!("Anhang D liefert p_d nur für Raumvolumen von {v_min:.0} bis {v_max:.0} m³; diese Änderung setzt das Volumen des modellierten Raums von {:.1} m³ auf {clamped:.1} m³, den nächsten Wert im Gültigkeitsbereich. Sie bildet einen unterteilten oder zusammengelegten Raum ab und ändert den realen Raum nicht: nur anwenden, wenn der Raum wirklich unterteilt oder zusammengelegt wird; andernfalls die Explosion mit einem Verfahren außerhalb von Anhang D beurteilen.", row.volume),
            ),
        ));
    }
    report.push(domain.build());
}

/// 🌡️ One bridge section: the contraction and expansion ranges with the bearing range, the linear temperature differences, curvature, end rotation, expansion and the combination.
fn check_thermal_section(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport, row: &ThermalSection) {
    let base = format!("thermalSections[id={}]", row.id);
    let subject = |leaf: &str| SubjectRef::new(&row.id, &format!("{base}.{leaf}"), copy(&format!("Thermal section {}", row.id), &format!("Temperaturquerschnitt {}", row.id)));
    let clause = |locator: &str| ClauseId::new("EN 1991-1-5", locator, "6.1");
    let id = |stem: &str| format!("en1991.1-5.{stem}.{}", row.id);
    let (Some(class), Some((heat, cool))) = (part_1_5::uniform_class(row.deck_type), part_1_5::bridge_delta_t_m(row.deck_type, row.surfacing)) else {
        let (leaf, label, found, options) = if part_1_5::uniform_class(row.deck_type).is_none() { ("deckType", ("deck type", "Überbauart"), row.deck_type, vec!["1", "2", "3", "4", "5"]) } else { ("surfacing", ("surfacing", "Belag"), row.surfacing, vec!["0", "1", "2", "3", "4"]) };
        invalid(report, annex, format!("en1991.input.thermalSections.{}.{leaf}", row.id), "DIN EN 1991-1-5", clause("Table 6.1"), &row.id, format!("{base}.{leaf}"), label, &found.to_string(), options.into_iter().map(String::from).collect());
        return;
    };
    for (leaf, value, label) in [("length", row.length, ("expansion length", "Dehnlänge")), ("depth", row.depth, ("section depth", "Querschnittshöhe")), ("alphaMicro", row.alpha_micro, ("expansion coefficient", "Ausdehnungskoeffizient"))] {
        if !(value.is_finite() && value > 0.0) {
            invalid(report, annex, format!("en1991.input.thermalSections.{}.{leaf}", row.id), "DIN EN 1991-1-5", clause("§6.1"), &row.id, format!("{base}.{leaf}"), label, &format!("{value}"), positive());
            return;
        }
    }
    let Some((con, exp)) = part_1_5::bridge_delta_t_n(class, document.t_min, document.t_max, document.t_0) else { return };
    let range = con + exp;
    let remedy = (row.assumed_range + 1e-9 < range).then(|| raise_temp_remedy(&format!("{base}.assumedRange"), &row.id, "movement range", "Bewegungsbereich", row.assumed_range, range));
    report.push(covers(&id("section-range"), "DIN EN 1991-1-5", clause("§6.1.3"), subject("assumedRange"), copy("Temperature range ΔT_N,con + ΔT_N,exp of the bearings and joints", "Temperaturbereich ΔT_N,con + ΔT_N,exp der Lager und Fugen"), temp(row.assumed_range), temp(range), copy(&format!("T_min = {:.1} °C, T_max = {:.1} °C, T_0 = {:.1} °C: ΔT_N,con = {con:.1} K, ΔT_N,exp = {exp:.1} K, range {range:.1} K; assumed {:.1} K.", document.t_min, document.t_max, document.t_0, row.assumed_range), &format!("T_min = {:.1} °C, T_max = {:.1} °C, T_0 = {:.1} °C: ΔT_N,con = {con:.1} K, ΔT_N,exp = {exp:.1} K, Bereich {range:.1} K; angenommen {:.1} K.", document.t_min, document.t_max, document.t_0, row.assumed_range)), annex, remedy, &["ndp.en1991.e15.shifts"]));
    let alpha = row.alpha_micro * 1.0e-6;
    info(report, annex, id("section-gradient"), "DIN EN 1991-1-5", clause("Table 6.1"), subject("deckType"), copy("Linear temperature difference ΔT_M (heating)", "Linearer Temperaturunterschied ΔT_M (Erwärmung)"), temp(heat), copy(&format!("ΔT_M,heat = {heat:.1} K, ΔT_M,cool = {cool:.1} K for deck type {} and surfacing {}.", row.deck_type, row.surfacing), &format!("ΔT_M,heat = {heat:.1} K, ΔT_M,cool = {cool:.1} K für Überbauart {} und Belag {}.", row.deck_type, row.surfacing)));
    let m = heat.max(cool);
    let kappa = part_1_5::curvature_per_m(alpha, m, row.depth);
    info(report, annex, id("section-curvature"), "DIN EN 1991-1-5", clause("§6.1.4"), subject("depth"), copy("Curvature κ = α ΔT_M/h (1/m)", "Krümmung κ = α ΔT_M/h (1/m)"), number(kappa), copy(&format!("κ = {kappa:.3e} 1/m for ΔT_M = {m:.1} K and h = {:.2} m.", row.depth), &format!("κ = {kappa:.3e} 1/m für ΔT_M = {m:.1} K und h = {:.2} m.", row.depth)));
    let theta = part_1_5::end_rotation_rad(alpha, m, row.length, row.depth);
    info(report, annex, id("section-rotation"), "DIN EN 1991-1-5", clause("§6.1.4"), subject("length"), copy("End rotation θ of a simply supported span (rad)", "Endverdrehung θ eines Einfeldträgers (rad)"), number(theta), copy(&format!("θ = α ΔT_M L/(2h) = {theta:.3e} rad."), &format!("θ = α ΔT_M L/(2h) = {theta:.3e} rad.")));
    let travel = part_1_5::expansion_m(alpha, range, row.length);
    info(report, annex, id("section-expansion"), "DIN EN 1991-1-5", clause("§6.1.3"), subject("length"), copy("Movement ΔL = α ΔT L of the joints", "Bewegung ΔL = α ΔT L der Fugen"), length(travel), copy(&format!("ΔL = {:.1} mm over {:.0} m for a range of {range:.1} K.", travel * 1000.0, row.length), &format!("ΔL = {:.1} mm über {:.0} m für einen Bereich von {range:.1} K.", travel * 1000.0, row.length)));
    let combined = part_1_5::combined_delta_t(con.max(exp), m);
    info(report, annex, id("section-combination"), "DIN EN 1991-1-5", clause("§6.1.5"), subject("deckType"), copy("Governing combination of ΔT_N and ΔT_M", "Maßgebende Kombination von ΔT_N und ΔT_M"), temp(combined), copy(&format!("max(ΔT_N + ω_M ΔT_M, ω_N ΔT_N + ΔT_M) = {combined:.1} K."), &format!("max(ΔT_N + ω_M ΔT_M, ω_N ΔT_N + ΔT_M) = {combined:.1} K.")));
}
