//! 🚂 Row checks of the rail and dust collections: derailment actions on a railway bridge (EN 1991-2, 6.7.1), the rail load groups of Table 6.11, the footbridge service vehicle (EN 1991-2, 5.6.3) and the dust explosion of a silo (EN 1991-4 with the vent sizing of EN 14491).
//! Every registry value is unverified and cited by the covering check; a row with an input outside the domain is an invalid input without a fallback. The helpers below are shared with the other row checks of the inferences.

extern crate semio_s_artifact_norm_en1991_checks as checks;
extern crate semio_s_artifact_norm_en1991_model as model;

#[path = "🧮️formulas/🦀️.rs"]
pub mod formulas;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

use checks::{assess_covers_on, invalid_input};
use model::{DustExplosion, En1991Snapshot, Footbridge, RailDerailment, RailLoadGroup, StructureKind};
use semio_s_plugin_norm_compliance::incremental_evaluation::Scope;
use semio_s_plugin_norm_compliance::{ActionFactors, AnnexChoice, CheckReport, CheckResult, ClauseId, DesignSituation, LocalizedCopy, Quantity, QuantityKind, ReliabilityClass, Remedy, SubjectRef};

fn copy(en: &str, de: &str) -> LocalizedCopy { LocalizedCopy::new(en, de) }
fn force(n: f64) -> Quantity { Quantity::new(QuantityKind::Force, n) }
fn length(m: f64) -> Quantity { Quantity::new(QuantityKind::Length, m) }
fn area(m2: f64) -> Quantity { Quantity::new(QuantityKind::Area, m2) }

/// 🔖 Covering check that cites every registry value read since the previous check.
#[allow(clippy::too_many_arguments)]
pub fn covers(id: &str, part: &str, clause: ClauseId, subject: SubjectRef, title: LocalizedCopy, assumed: Quantity, required: Quantity, explanation: LocalizedCopy, annex: AnnexChoice, remedy: Option<Remedy>) -> CheckResult {
    let cited: Vec<&str> = semio_s_plugin_norm_compliance::drain_ndp_reads();
    assess_covers_on(id, part, clause, subject, title, assumed, required, explanation, annex, remedy, &cited)
}

/// 🔢 Representative positive values offered as remedy options for a non-positive input.
pub fn positive() -> Vec<String> { vec!["1".into(), "10".into(), "100".into()] }

/// 🔢 The first input of `values` that is not a finite positive number, with its leaf and label.
pub fn first_non_positive<'a>(values: &[(&'a str, f64, (&'a str, &'a str))]) -> Option<(&'a str, f64, (&'a str, &'a str))> { values.iter().copied().find(|(_, v, _)| !(v.is_finite() && *v > 0.0)) }

/// 🚫 An invalid-input check on one leaf of a row, with the admissible values as remedy options.
#[allow(clippy::too_many_arguments)]
pub fn invalid(report: &mut CheckReport, annex: AnnexChoice, id: String, part: &str, clause: ClauseId, entity: &str, path: String, label: (&str, &str), found: &str, admissible: Vec<String>) {
    let (label_en, label_de) = label;
    report.push(invalid_input(&id, part, clause, SubjectRef::new(entity, path, copy(label_en, label_de)), copy(&format!("Known {label_en}"), &format!("Bekannter Wert: {label_de}")), copy(&format!("'{found}' is not an admissible {label_en}; the check is not evaluated with an invented default."), &format!("'{found}' ist kein zulässiger Wert für {label_de}; der Nachweis wird nicht mit einem erfundenen Ersatzwert geführt.")), admissible, annex));
}

/// ℹ️ An information check that reports one quantity without a verdict.
#[allow(clippy::too_many_arguments)]
pub fn info_check(id: String, part: &str, clause: ClauseId, subject: SubjectRef, title: LocalizedCopy, quantity: Quantity, explanation: LocalizedCopy, annex: AnnexChoice) -> CheckResult {
    CheckResult::assess(id, part, clause, subject, title).minimum(quantity, Quantity::new(quantity.kind, checks::catalogue::info_floor(quantity.value))).annex(annex).explanation(explanation).ndp_reads().build()
}

fn raise(subject: SubjectRef, kind: QuantityKind, label: (&str, &str), unit: &str, scale: f64, current: f64, required: f64) -> Remedy {
    let (c, r) = (current / scale, required / scale);
    Remedy::at_least(subject, Quantity::new(kind, current), Quantity::new(kind, required), copy(&format!("Raise the {} from {c:.3} {unit} to at least {r:.3} {unit}.", label.0), &format!("{} von {c:.3} {unit} auf mindestens {r:.3} {unit} erhöhen.", label.1)))
}

fn not_applicable(annex: AnnexChoice, id: String, part: &str, clause: ClauseId, subject: SubjectRef, title: (&str, &str), reason: (&str, &str)) -> CheckResult {
    CheckResult::assess(id, part, clause, subject, copy(title.0, title.1)).not_applicable(copy(reason.0, reason.1)).annex(annex).build()
}

/// 🚂 The row checks of the dust explosions, the rail load groups and the rail derailments, in the order of the units of the incremental evaluation.
pub fn check_rows(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport, scope: Scope<'_>) {
    for (_, row) in document.dust_explosions.iter().enumerate().filter(|(i, _)| scope.admits("dustExplosions", *i)) { check_dust_explosion(annex, report, row); }
    for (_, row) in document.rail_load_groups.iter().enumerate().filter(|(i, _)| scope.admits("railLoadGroups", *i)) { check_rail_load_group(document, annex, report, row); }
    for (_, row) in document.rail_derailments.iter().enumerate().filter(|(i, _)| scope.admits("railDerailments", *i)) { check_rail_derailment(document, annex, report, row); }
}

/// 💥 One dust explosion: the inputs lie in the validity range of the vent area formula, the vent area covers the area the reduced explosion pressure requires and the pressure ratio p_red / p_max is reported.
fn check_dust_explosion(annex: AnnexChoice, report: &mut CheckReport, row: &DustExplosion) {
    let base = format!("dustExplosions[id={}]", row.id);
    let subject = |leaf: &str| SubjectRef::new(&row.id, &format!("{base}.{leaf}"), copy(&format!("Dust explosion {}", row.id), &format!("Staubexplosion {}", row.id)));
    let clause = || ClauseId::new("EN 1991-4", "EN 14491", "-");
    if !(row.vent_area.is_finite() && row.vent_area > 0.0) {
        invalid(report, annex, format!("en1991.input.dustExplosions.{}.ventArea", row.id), "DIN EN 1991-4", clause(), &row.id, format!("{base}.ventArea"), ("vent area A_v", "Entlastungsfläche A_v"), &row.vent_area.to_string(), positive());
        return;
    }
    if let Some((leaf, (low, high), label)) = formulas::dust_out_of_domain(row.kst, row.p_max, row.volume, row.static_opening_pressure, row.design_pressure) {
        let found = match leaf { "kst" => row.kst, "pMax" => row.p_max, "volume" => row.volume, "staticOpeningPressure" => row.static_opening_pressure, _ => row.design_pressure };
        invalid(report, annex, format!("en1991.input.dustExplosions.{}.{leaf}", row.id), "DIN EN 1991-4", clause(), &row.id, format!("{base}.{leaf}"), label, &found.to_string(), vec![format!("{low}"), format!("{}", (low + high) / 2.0), format!("{high}")]);
        return;
    }
    let required = formulas::dust_vent_area_m2(row.kst, row.p_max, row.volume, row.static_opening_pressure, row.design_pressure);
    let remedy = (row.vent_area + 1e-9 < required).then(|| raise(subject("ventArea"), QuantityKind::Area, ("vent area", "Entlastungsfläche"), "m²", 1.0, row.vent_area, required));
    report.push(covers(&format!("en1991.4.dust-vent-area.{}", row.id), "DIN EN 1991-4", clause(), subject("ventArea"), copy("Vent area A_v for the reduced explosion pressure", "Entlastungsfläche A_v für den reduzierten Explosionsdruck"), area(row.vent_area), area(required), copy(&format!("A_v,req = [c1 p_max K_St p_red^e1 + c2 (p_stat − 0.1 bar) p_red^e2] V^e3 = {required:.3} m² for K_St = {:.0} bar m/s, p_max = {:.2} bar, V = {:.1} m³, p_stat = {:.2} bar and p_red = {:.2} bar; the provided vent area is {:.3} m².", row.kst, row.p_max / 1e5, row.volume, row.static_opening_pressure / 1e5, row.design_pressure / 1e5, row.vent_area), &format!("A_v,erf = [c1 p_max K_St p_red^e1 + c2 (p_stat − 0,1 bar) p_red^e2] V^e3 = {required:.3} m² für K_St = {:.0} bar m/s, p_max = {:.2} bar, V = {:.1} m³, p_stat = {:.2} bar und p_red = {:.2} bar; vorhanden sind {:.3} m².", row.kst, row.p_max / 1e5, row.volume, row.static_opening_pressure / 1e5, row.design_pressure / 1e5, row.vent_area)), annex, remedy));
    report.push(info_check(format!("en1991.4.dust-pressure.{}", row.id), "DIN EN 1991-4", clause(), subject("designPressure"), copy("Reduction p_red / p_max of the explosion pressure", "Abminderung p_red / p_max des Explosionsdrucks"), Quantity::new(QuantityKind::Dimensionless, row.design_pressure / row.p_max), copy(&format!("The venting reduces the maximum explosion pressure {:.2} bar to {:.2} bar, a ratio of {:.3}.", row.p_max / 1e5, row.design_pressure / 1e5, row.design_pressure / row.p_max), &format!("Die Entlastung mindert den maximalen Explosionsdruck {:.2} bar auf {:.2} bar, ein Verhältnis von {:.3}.", row.p_max / 1e5, row.design_pressure / 1e5, row.design_pressure / row.p_max)), annex));
}

/// 🛤️ One rail load group of Table 6.11: the group and the component factors are valid, the assumed group actions cover the factored characteristic LM71 axle, LM71 distributed load, longitudinal force and nosing force, and the combination values of EN 1990 are reported.
fn check_rail_load_group(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport, row: &RailLoadGroup) {
    let base = format!("railLoadGroups[id={}]", row.id);
    let subject = |leaf: &str| SubjectRef::new(&row.id, &format!("{base}.{leaf}"), copy(&format!("Rail load group {}", row.id), &format!("Eisenbahn-Lastgruppe {}", row.id)));
    let clause = || ClauseId::new("EN 1991-2", "Table 6.11", "6.8.2");
    if document.structure_kind != StructureKind::Bridge {
        report.push(not_applicable(annex, format!("en1991.2.rail-group-axle.{}", row.id), "DIN EN 1991-2", clause(), subject("loadGroup"), ("Rail load group", "Eisenbahn-Lastgruppe"), ("Structure kind is building: rail load groups are not applicable.", "Tragwerksart ist Gebäude: Eisenbahn-Lastgruppen sind nicht anwendbar.")));
        return;
    }
    if !formulas::RAIL_GROUPS.contains(&row.load_group.as_str()) {
        invalid(report, annex, format!("en1991.input.railLoadGroups.{}.loadGroup", row.id), "DIN EN 1991-2", clause(), &row.id, format!("{base}.loadGroup"), ("rail load group", "Eisenbahn-Lastgruppe"), &row.load_group, formulas::RAIL_GROUPS.iter().map(|g| g.to_string()).collect());
        return;
    }
    for (leaf, value, label) in [("factorVertical", row.factor_vertical, ("vertical component factor", "Faktor der Vertikalkomponente")), ("factorLongitudinal", row.factor_longitudinal, ("longitudinal component factor", "Faktor der Längskomponente")), ("factorLateral", row.factor_lateral, ("lateral component factor", "Faktor der Querkomponente"))] {
        if !(value.is_finite() && (0.0..=1.0).contains(&value)) {
            invalid(report, annex, format!("en1991.input.railLoadGroups.{}.{leaf}", row.id), "DIN EN 1991-2", clause(), &row.id, format!("{base}.{leaf}"), label, &value.to_string(), vec!["0".into(), "0.5".into(), "1".into()]);
            return;
        }
    }
    let alpha = document.rail_alpha;
    let group = &row.load_group;
    let axle = row.factor_vertical * checks::part_2::rail_lm71_axle_n(alpha);
    let remedy = (row.assumed_axle_load + 1e-9 < axle).then(|| raise(subject("assumedAxleLoad"), QuantityKind::Force, ("group axle load", "Gruppen-Achslast"), "kN", 1000.0, row.assumed_axle_load, axle));
    report.push(covers(&format!("en1991.2.rail-group-axle.{}", row.id), "DIN EN 1991-2", clause(), subject("assumedAxleLoad"), copy("Group axle load of the LM71", "Gruppen-Achslast des LM71"), force(row.assumed_axle_load), force(axle), copy(&format!("{group}: {:.2} · α Q_vk = {:.2} · {alpha:.2} · 250 kN = {:.1} kN per axle; the assumed group axle load is {:.1} kN.", row.factor_vertical, row.factor_vertical, axle / 1000.0, row.assumed_axle_load / 1000.0), &format!("{group}: {:.2} · α Q_vk = {:.2} · {alpha:.2} · 250 kN = {:.1} kN je Achse; angesetzt sind {:.1} kN.", row.factor_vertical, row.factor_vertical, axle / 1000.0, row.assumed_axle_load / 1000.0)), annex, remedy));
    let udl = row.factor_vertical * checks::part_2::rail_lm71_udl_n_m(alpha);
    let remedy = (row.assumed_udl + 1e-9 < udl).then(|| raise(subject("assumedUdl"), QuantityKind::Force, ("group distributed load", "Gruppen-Streckenlast"), "kN/m", 1000.0, row.assumed_udl, udl));
    report.push(covers(&format!("en1991.2.rail-group-udl.{}", row.id), "DIN EN 1991-2", clause(), subject("assumedUdl"), copy("Group distributed load of the LM71", "Gruppen-Streckenlast des LM71"), force(row.assumed_udl), force(udl), copy(&format!("{group}: {:.2} · α q_vk = {:.2} · {alpha:.2} · 80 kN/m = {:.1} kN/m; the assumed group distributed load is {:.1} kN/m.", row.factor_vertical, row.factor_vertical, udl / 1000.0, row.assumed_udl / 1000.0), &format!("{group}: {:.2} · α q_vk = {:.2} · {alpha:.2} · 80 kN/m = {:.1} kN/m; angesetzt sind {:.1} kN/m.", row.factor_vertical, row.factor_vertical, udl / 1000.0, row.assumed_udl / 1000.0)), annex, remedy));
    if document.bridge_span.is_finite() && document.bridge_span > 0.0 {
        let longitudinal = row.factor_longitudinal * formulas::rail_longitudinal_n(document.bridge_span);
        let remedy = (row.assumed_longitudinal + 1e-9 < longitudinal).then(|| raise(subject("assumedLongitudinal"), QuantityKind::Force, ("group longitudinal force", "Gruppen-Längskraft"), "kN", 1000.0, row.assumed_longitudinal, longitudinal));
        report.push(covers(&format!("en1991.2.rail-group-longitudinal.{}", row.id), "DIN EN 1991-2", clause(), subject("assumedLongitudinal"), copy("Group longitudinal force (traction and braking)", "Gruppen-Längskraft (Anfahren und Bremsen)"), force(row.assumed_longitudinal), force(longitudinal), copy(&format!("{group}: {:.2} · max(Q_lak, Q_lbk) = {:.2} · {:.1} kN = {:.1} kN for L = {:.1} m; the assumed group longitudinal force is {:.1} kN.", row.factor_longitudinal, row.factor_longitudinal, formulas::rail_longitudinal_n(document.bridge_span) / 1000.0, longitudinal / 1000.0, document.bridge_span, row.assumed_longitudinal / 1000.0), &format!("{group}: {:.2} · max(Q_lak, Q_lbk) = {:.2} · {:.1} kN = {:.1} kN für L = {:.1} m; angesetzt sind {:.1} kN.", row.factor_longitudinal, row.factor_longitudinal, formulas::rail_longitudinal_n(document.bridge_span) / 1000.0, longitudinal / 1000.0, document.bridge_span, row.assumed_longitudinal / 1000.0)), annex, remedy));
    }
    let lateral = row.factor_lateral * formulas::rail_nosing_n(alpha);
    let remedy = (row.assumed_lateral + 1e-9 < lateral).then(|| raise(subject("assumedLateral"), QuantityKind::Force, ("group lateral force", "Gruppen-Querkraft"), "kN", 1000.0, row.assumed_lateral, lateral));
    report.push(covers(&format!("en1991.2.rail-group-lateral.{}", row.id), "DIN EN 1991-2", clause(), subject("assumedLateral"), copy("Group lateral force (nosing)", "Gruppen-Querkraft (Schlingern)"), force(row.assumed_lateral), force(lateral), copy(&format!("{group}: {:.2} · α Q_sk = {:.2} · {alpha:.2} · 100 kN = {:.1} kN; the assumed group lateral force is {:.1} kN.", row.factor_lateral, row.factor_lateral, lateral / 1000.0, row.assumed_lateral / 1000.0), &format!("{group}: {:.2} · α Q_sk = {:.2} · {alpha:.2} · 100 kN = {:.1} kN; angesetzt sind {:.1} kN.", row.factor_lateral, row.factor_lateral, lateral / 1000.0, row.assumed_lateral / 1000.0)), annex, remedy));
    let Some(psi) = ActionFactors::new(annex, ReliabilityClass::RC2, DesignSituation::Persistent).domain(semio_s_plugin_norm_compliance::action_factors::Domain::Bridge).psi_known("rail_traffic") else { return };
    report.push(info_check(format!("en1991.2.rail-group-combination.{}", row.id), "DIN EN 1991-2", clause(), subject("assumedAxleLoad"), copy("Combination values of the group axle load", "Kombinationswerte der Gruppen-Achslast"), force(psi.psi_1 * axle), copy(&format!("{group}: characteristic {:.1} kN, combination ψ₀ Q = {:.1} kN, frequent ψ₁ Q = {:.1} kN, quasi-permanent ψ₂ Q = {:.1} kN per axle (ψ₀/ψ₁/ψ₂ = {:.2}/{:.2}/{:.2} of EN 1990 Table A2.3, unverified).", axle / 1000.0, psi.psi_0 * axle / 1000.0, psi.psi_1 * axle / 1000.0, psi.psi_2 * axle / 1000.0, psi.psi_0, psi.psi_1, psi.psi_2), &format!("{group}: charakteristisch {:.1} kN, Kombination ψ₀ Q = {:.1} kN, häufig ψ₁ Q = {:.1} kN, quasi-ständig ψ₂ Q = {:.1} kN je Achse (ψ₀/ψ₁/ψ₂ = {:.2}/{:.2}/{:.2} nach EN 1990 Tabelle A2.3, ungeprüft).", axle / 1000.0, psi.psi_0 * axle / 1000.0, psi.psi_1 * axle / 1000.0, psi.psi_2 * axle / 1000.0, psi.psi_0, psi.psi_1, psi.psi_2)), annex));
}

/// 🚆 One derailment action on a railway bridge (EN 1991-2, 6.7.1): the design situation is I or II, the assumed line load covers the registry value of the situation, the loaded length covers the registry length and the offset from the track axis is reported against the registry multiple of the track gauge.
fn check_rail_derailment(document: &En1991Snapshot, annex: AnnexChoice, report: &mut CheckReport, row: &RailDerailment) {
    let base = format!("railDerailments[id={}]", row.id);
    let subject = |leaf: &str| SubjectRef::new(&row.id, &format!("{base}.{leaf}"), copy(&format!("Rail derailment {}", row.id), &format!("Eisenbahn-Entgleisung {}", row.id)));
    let clause = || ClauseId::new("EN 1991-2", "§6.7.1", "6.7.1");
    if document.structure_kind != StructureKind::Bridge {
        report.push(not_applicable(annex, format!("en1991.2.derail-line-load.{}", row.id), "DIN EN 1991-2", clause(), subject("situation"), ("Derailment line load", "Entgleisungs-Linienlast"), ("Structure kind is building: rail bridge derailment is not applicable.", "Tragwerksart ist Gebäude: die Entgleisung auf Eisenbahnbrücken ist nicht anwendbar.")));
        return;
    }
    let Some(line) = formulas::derail_line_load_n_m(&row.situation, document.rail_alpha) else {
        invalid(report, annex, format!("en1991.input.railDerailments.{}.situation", row.id), "DIN EN 1991-2", clause(), &row.id, format!("{base}.situation"), ("design situation (i or ii)", "Bemessungssituation (i oder ii)"), &row.situation, formulas::SITUATIONS.iter().map(|s| s.to_string()).collect());
        return;
    };
    if let Some((leaf, value, label)) = first_non_positive(&[("assumedLineLoad", row.assumed_line_load, ("assumed line load", "angesetzte Linienlast")), ("loadedLength", row.loaded_length, ("loaded length", "belastete Länge"))]) {
        invalid(report, annex, format!("en1991.input.railDerailments.{}.{leaf}", row.id), "DIN EN 1991-2", clause(), &row.id, format!("{base}.{leaf}"), label, &value.to_string(), positive());
        return;
    }
    if !(row.offset.is_finite() && row.offset >= 0.0) {
        invalid(report, annex, format!("en1991.input.railDerailments.{}.offset", row.id), "DIN EN 1991-2", clause(), &row.id, format!("{base}.offset"), ("offset from the track axis", "Abstand von der Gleisachse"), &row.offset.to_string(), vec!["0".into(), "1".into(), "2".into()]);
        return;
    }
    let name = if row.situation == "i" { "I" } else { "II" };
    let remedy = (row.assumed_line_load + 1e-9 < line).then(|| raise(subject("assumedLineLoad"), QuantityKind::Force, ("assumed line load", "angesetzte Linienlast"), "kN/m", 1000.0, row.assumed_line_load, line));
    report.push(covers(&format!("en1991.2.derail-line-load.{}", row.id), "DIN EN 1991-2", clause(), subject("assumedLineLoad"), copy("Derailment line load", "Entgleisungs-Linienlast"), force(row.assumed_line_load), force(line), copy(&format!("Design situation {name}: the line load is {:.1} kN/m (registry value for I, factor times α q_vk for II); the assumed line load is {:.1} kN/m.", line / 1000.0, row.assumed_line_load / 1000.0), &format!("Bemessungssituation {name}: die Linienlast beträgt {:.1} kN/m (Registerwert für I, Faktor mal α q_vk für II); angesetzt sind {:.1} kN/m.", line / 1000.0, row.assumed_line_load / 1000.0)), annex, remedy));
    let span = formulas::derail_length_m();
    let remedy = (row.loaded_length + 1e-9 < span).then(|| raise(subject("loadedLength"), QuantityKind::Length, ("loaded length", "belastete Länge"), "m", 1.0, row.loaded_length, span));
    report.push(covers(&format!("en1991.2.derail-length.{}", row.id), "DIN EN 1991-2", clause(), subject("loadedLength"), copy("Loaded length of the derailment line load", "Belastete Länge der Entgleisungs-Linienlast"), length(row.loaded_length), length(span), copy(&format!("The derailment line load acts over {span:.1} m along the track; the assumed loaded length is {:.1} m.", row.loaded_length), &format!("Die Entgleisungs-Linienlast wirkt über {span:.1} m entlang des Gleises; angesetzt sind {:.1} m.", row.loaded_length)), annex, remedy));
    let limit = formulas::derail_offset_limit_m();
    let remedy = (row.offset > limit + 1e-9).then(|| Remedy::at_most(subject("offset"), length(row.offset), length(limit), copy(&format!("Reduce the offset from {:.3} m to at most {limit:.3} m.", row.offset), &format!("Abstand von {:.3} m auf höchstens {limit:.3} m verringern.", row.offset))));
    report.push(covers(&format!("en1991.2.derail-offset.{}", row.id), "DIN EN 1991-2", clause(), subject("offset"), copy("Offset of the line load from the track axis", "Abstand der Linienlast von der Gleisachse"), length(limit), length(row.offset), copy(&format!("The line load lies {:.2} m from the track axis; the largest offset is {limit:.2} m (registry multiple of the track gauge).", row.offset), &format!("Die Linienlast liegt {:.2} m von der Gleisachse; der größte Abstand beträgt {limit:.2} m (Registervielfaches der Spurweite).", row.offset)), annex, remedy));
}

/// 🚐 The service vehicle of one footbridge (EN 1991-2, 5.6.3): without an assumed axle load (0) no check is emitted and none is assumed, otherwise the assumed axle load covers the registry axle load.
pub fn check_footbridge_service(bridge: &Footbridge, annex: AnnexChoice, report: &mut CheckReport) {
    let base = format!("footbridges[id={}]", bridge.id);
    let subject = |leaf: &str| SubjectRef::new(&bridge.id, &format!("{base}.{leaf}"), copy("Footbridge", "Fußgängerbrücke"));
    let clause = || ClauseId::new("EN 1991-2", "§5.6.3", "5.6.3");
    let assumed = bridge.assumed_service_axle;
    if !(assumed.is_finite() && assumed >= 0.0) {
        invalid(report, annex, format!("en1991.input.footbridges.{}.assumedServiceAxle", bridge.id), "DIN EN 1991-2", clause(), &bridge.id, format!("{base}.assumedServiceAxle"), ("service vehicle axle load (0 = none)", "Dienstfahrzeug-Achslast (0 = keine)"), &assumed.to_string(), vec!["0".into(), "40000".into(), "80000".into()]);
        return;
    }
    if assumed == 0.0 {
        return;
    }
    let required = formulas::footbridge_service_axle_n();
    let remedy = (assumed + 1e-9 < required).then(|| raise(subject("assumedServiceAxle"), QuantityKind::Force, ("service vehicle axle load", "Dienstfahrzeug-Achslast"), "kN", 1000.0, assumed, required));
    report.push(covers(&format!("en1991.2.footbridge-service.{}", bridge.id), "DIN EN 1991-2", clause(), subject("assumedServiceAxle"), copy("Service vehicle axle load Q_sv", "Dienstfahrzeug-Achslast Q_sv"), force(assumed), force(required), copy(&format!("The two-axle service vehicle carries {:.0} kN per axle; the assumed axle load is {:.1} kN.", required / 1000.0, assumed / 1000.0), &format!("Das zweiachsige Dienstfahrzeug trägt {:.0} kN je Achse; angesetzt sind {:.1} kN.", required / 1000.0, assumed / 1000.0)), annex, remedy));
}
