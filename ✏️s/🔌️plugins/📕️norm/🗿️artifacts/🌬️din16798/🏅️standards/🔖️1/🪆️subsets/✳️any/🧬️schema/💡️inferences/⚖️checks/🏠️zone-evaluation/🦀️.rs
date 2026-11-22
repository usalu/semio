//! 🏠️ DIN EN 16798 zone evaluation: comfort, pollution, acoustics and thermal checks of one zone.

use super::*;

pub(super) fn evaluate_zone(doc: &semio_s_artifact_norm_din16798_model::Din16798Snapshot, _zi: usize, zone: &ZoneDocument, annex: &annex_params::AnnexParams) -> Vec<CheckResult> {
    use part_1::*;
    let mut out = Vec::new();
    let cat = parse_category(&zone.comfort_category);
    let usage = parse_usage(&zone.usage_type);
    let pollution = parse_pollution(&zone.pollution_class);
    let method = VentMethod::parse(&zone.vent_method);
    let path = |field: &str| format!("zones[id={}].{field}", zone.id);
    let linked = doc.vent_systems.iter().find(|v| v.id == zone.vent_system_id);
    let linked_name = linked.map(|v| v.name.as_str()).unwrap_or("—");
    let linked_type = linked.map(|v| VentSystemType::parse(&v.system_type));
    check_zone_ventilation_link(doc, zone, &mut out, &path, linked, linked_name);
    check_zone_temperatures(doc, zone, &mut out, cat, usage, &path);
    check_zone_clothing_insulation(doc, zone, &mut out, cat, &path);
    check_zone_air_quality(doc, zone, annex, &mut out, cat, usage, pollution, method, &path, linked_name, linked_type);
    if let Some(result) = unoccupied_ventilation_report::check(zone, usage, doc.annex) {
        out.push(result);
    }
    check_zone_humidity(doc, zone, linked, &mut out, cat, &path);
    check_zone_light_noise_and_draught(doc, zone, &mut out, cat, usage, &path);
    out
}

/// 🧱️ Checks the ventilation system a zone links to.
fn check_zone_ventilation_link(
    doc: &semio_s_artifact_norm_din16798_model::Din16798Snapshot,
    zone: &ZoneDocument,
    out: &mut Vec<CheckResult>,
    path: &dyn Fn(&str) -> String,
    linked: Option<&semio_s_artifact_norm_din16798_model::VentSystemDocument>,
    linked_name: &str,
) {
    {
        let mut bi = CheckResult::assess(
            format!("din16798.zone.ventSystem.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§6.3", "system"),
            zone_subject(zone, path("ventSystemId")), loc("Zone ventilation-system link", "Zonen-Lüftungsanlagen-Zuordnung"),
        ).annex(doc.annex);
        if linked.is_some() {
            bi = bi.status(CheckStatus::Pass).explanation(loc(
                format!("Zone linked to ventilation system «{linked_name}» ({})", zone.vent_system_id),
                format!("Zone verknüpft mit Lüftungsanlage «{linked_name}» ({})", zone.vent_system_id),
            ));
        } else {
            let options: Vec<String> = doc.vent_systems.iter().map(|v| v.id.clone()).collect();
            bi = bi.status(CheckStatus::Fail)
                .explanation(loc(
                    format!("Unresolved ventSystemId «{}» — no matching ventSystems[].id", zone.vent_system_id),
                    format!("Unaufgelöste ventSystemId «{}» — keine passende ventSystems[].id", zone.vent_system_id),
                ))
                .remedy({
                    let labels_en: Vec<String> = doc.vent_systems.iter().map(|v| format!("{} ({})", v.name, v.id)).collect();
                    let labels_de: Vec<String> = doc.vent_systems.iter().map(|v| format!("{} ({})", v.name, v.id)).collect();
                    Remedy {
                        target: zone_subject(zone, path("ventSystemId")),
                        current: Quantity::new(QuantityKind::Dimensionless, 0.0),
                        required: Quantity::new(QuantityKind::Dimensionless, 1.0),
                        bound: RemedyBound::OneOf,
                        options: if options.is_empty() { vec!["vent-central".into()] } else { options },
                        action: loc(
                            format!(
                                "Set ventSystemId to an existing system (unresolved «{}»). Choices: {}.",
                                zone.vent_system_id,
                                if labels_en.is_empty() { "—".into() } else { labels_en.join(", ") }
                            ),
                            format!(
                                "ventSystemId auf eine bestehende Anlage setzen (unaufgelöst «{}»). Auswahl: {}.",
                                zone.vent_system_id,
                                if labels_de.is_empty() { "—".into() } else { labels_de.join(", ") }
                            ),
                        ),
                        applicable: true,
                    }
                });
        }
        out.push(bi.build());
    }
}

/// 🪵️ Compares the zone's seasonal design temperatures with the relevant reference values.
fn check_zone_temperatures(
    doc: &semio_s_artifact_norm_din16798_model::Din16798Snapshot,
    zone: &ZoneDocument,
    out: &mut Vec<CheckResult>,
    cat: ComfortCategory,
    usage: OccupancyType,
    path: &dyn Fn(&str) -> String,
) {
    use part_1::*;
    let clause = if doc.annex == AnnexChoice::De { "Table NA.3" } else { "Table B.2" };
    let limits = seasonal_design_temperatures(doc.annex, usage, cat);
    for (summer, season, measure, value, label) in [
        (true, "summer", "tOpSummerC", zone.t_op_summer_c, loc("Summer design temperature", "Auslegungstemperatur Sommer")),
        (false, "winter", "tOpWinterC", zone.t_op_winter_c, loc("Winter design temperature", "Auslegungstemperatur Winter")),
    ] {
        let target = zone_subject(zone, path(measure));
        let b = CheckResult::assess(
            format!("din16798-1.top.{season}.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", clause, "θ_op"),
            target.clone(), label,
        ).annex(doc.annex);
        if summer && ComfortModel::parse(&zone.comfort_model) == ComfortModel::Adaptive {
            out.push(b.not_applicable(loc(
                "Use the selected adaptive comfort assessment for summer.",
                "Für den Sommer gilt die gewählte adaptive Komfortbewertung.",
            )).build());
            continue;
        }
        let Some((heating, cooling)) = limits else {
            out.push(b.status(CheckStatus::UnverifiedBasis).explanation(loc(
                "Confirm a room use and design basis covered by the temperature table.",
                "Eine von der Temperaturtabelle erfasste Raumnutzung und Auslegungsgrundlage bestätigen.",
            )).build());
            continue;
        };
        let current = Quantity::new(QuantityKind::Temperature, value);
        let result = if summer {
            let limit = Quantity::new(QuantityKind::Temperature, cooling);
            let b = b.utilization(current, limit).explanation(loc(
                format!("Summer design temperature {value:.1} °C. Reference cooling maximum {cooling:.1} °C."),
                format!("Auslegungstemperatur Sommer {value:.1} °C. Richtwert Kühlmaximum {cooling:.1} °C."),
            ));
            if value > cooling {
                b.remedy(Remedy::at_most(target, current, limit, loc(
                    format!("Lower the summer design temperature to at most {cooling:.1} °C."),
                    format!("Auslegungstemperatur Sommer auf höchstens {cooling:.1} °C senken."),
                )))
            } else { b }
        } else {
            let limit = Quantity::new(QuantityKind::Temperature, heating);
            let b = b.minimum(current, limit).explanation(loc(
                format!("Winter design temperature {value:.1} °C. Reference heating minimum {heating:.1} °C."),
                format!("Auslegungstemperatur Winter {value:.1} °C. Richtwert Heizminimum {heating:.1} °C."),
            ));
            if value < heating {
                b.remedy(Remedy::at_least(target, current, limit, loc(
                    format!("Raise the winter design temperature to at least {heating:.1} °C."),
                    format!("Auslegungstemperatur Winter auf mindestens {heating:.1} °C erhöhen."),
                )))
            } else { b }
        };
        out.push(result.build());
    }
}

/// 🔩️ Checks the clothing insulation and the seasonal comfort bands of a zone.
fn check_zone_clothing_insulation(
    doc: &semio_s_artifact_norm_din16798_model::Din16798Snapshot,
    zone: &ZoneDocument,
    out: &mut Vec<CheckResult>,
    cat: ComfortCategory,
    path: &dyn Fn(&str) -> String,
) {
    use part_1::*;
    let clo_summer = zone.clothing_clo;
    let clo_winter = 1.0;
    if ComfortModel::parse(&zone.comfort_model) == ComfortModel::Adaptive {
        for (suffix, title_en, title_de) in [
            ("pmv.summer", "Summer PMV (ISO 7730)", "Sommer-PMV (ISO 7730)"),
            ("ppd.summer", "Summer PPD (ISO 7730)", "Sommer-PPD (ISO 7730)"),
            ("pmv.winter", "Winter PMV (ISO 7730)", "Winter-PMV (ISO 7730)"),
            ("ppd.winter", "Winter PPD (ISO 7730)", "Winter-PPD (ISO 7730)"),
        ] {
            out.push(CheckResult::assess(
                format!("din16798-1.{suffix}.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§7.2", "PMV"),
                zone_subject(zone, path("comfortModel")), loc(title_en, title_de),
            ).annex(doc.annex)
            .not_applicable(loc(
                "Adaptive model selected; PMV/PPD (ISO 7730) are not evaluated under Annex B.2.",
                "Adaptives Modell gewählt; PMV/PPD (ISO 7730) werden unter Anhang B.2 nicht bewertet.",
            )).build());
        }
        let target = zone_subject(zone, path("tOpSummerC"));
        let clause = if doc.annex == AnnexChoice::De { "NA.4.1" } else { "B.2.2 / Figure B.1" };
        let b = CheckResult::assess(
            format!("din16798-1.adaptive.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", clause, "θ_c"),
            target.clone(), loc("Adaptive comfort (running-mean)", "Adaptiver Komfort (Außenluftmittel)"),
        ).annex(doc.annex);
        if doc.annex == AnnexChoice::De {
            out.push(b.status(CheckStatus::UnverifiedBasis).explanation(loc(
                "The German occupied-temperature assessment requires hourly outdoor and indoor data under NA.4.1.",
                "Für die deutsche Temperaturbewertung während der Nutzung werden nach NA.4.1 stündliche Außen- und Innendaten benötigt.",
            )).build());
            return;
        }
        let Some((lower, upper)) = adaptive_operative_bounds(doc.theta_rm_c, cat) else {
            out.push(b.status(CheckStatus::UnverifiedBasis).explanation(loc(
                "Figure B.1 defines categories I to III for a running-mean outdoor temperature strictly between 10 and 30 °C.",
                "Bild B.1 definiert Kategorien I bis III für gleitende Außenluftmittelwerte größer als 10 und kleiner als 30 °C.",
            )).build());
            return;
        };
        let t = zone.t_op_summer_c;
        let current = Quantity::new(QuantityKind::Temperature, t);
        let minimum = Quantity::new(QuantityKind::Temperature, lower);
        let maximum = Quantity::new(QuantityKind::Temperature, upper);
        let explanation = loc(
            format!("Operative temperature {t:.1} °C. Category {:?} design range {lower:.1}–{upper:.1} °C at running mean {:.1} °C.", cat, doc.theta_rm_c),
            format!("Operative Temperatur {t:.1} °C. Auslegungsbereich für Kategorie {:?} {lower:.1}–{upper:.1} °C bei gleitendem Mittelwert {:.1} °C.", cat, doc.theta_rm_c),
        );
        let assessment = if t < lower {
            b.minimum(current, minimum).remedy(Remedy::at_least(target, current, minimum, loc(
                format!("Raise the operative temperature to at least {lower:.1} °C."),
                format!("Operative Temperatur auf mindestens {lower:.1} °C erhöhen."),
            )))
        } else if t > upper {
            b.utilization(current, maximum).remedy(Remedy::at_most(target, current, maximum, loc(
                format!("Lower the operative temperature to at most {upper:.1} °C."),
                format!("Operative Temperatur auf höchstens {upper:.1} °C senken."),
            )))
        } else {
            b.utilization(current, maximum)
        };
        out.push(assessment.explanation(explanation).build());
    } else {
        out.push(CheckResult::assess(
            format!("din16798-1.adaptive.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "Annex B.2", "θ_c"),
            zone_subject(zone, path("comfortModel")),
            loc("Adaptive comfort (running-mean)", "Adaptiver Komfort (Außenluftmittel)"),
        ).annex(doc.annex)
        .not_applicable(loc(
            "Fixed HVAC comfort model selected; Annex B.2 adaptive comfort does not apply.",
            "Festes HVAC-Komfortmodell gewählt; adaptiver Komfort nach Anhang B.2 gilt nicht.",
        )).build());

        let lim = pmv_limit(cat);
        let plim = ppd_limit(cat);
        let pmv_s = pmv_iso7730(zone.t_op_summer_c, zone.rh_percent, zone.air_speed_m_s, zone.metabolic_rate_met, clo_summer);
        let mut bp_s = CheckResult::assess(
            format!("din16798-1.pmv.summer.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§7.2.2", "PMV"),
            zone_subject(zone, path("tOpSummerC")), loc("Summer PMV (ISO 7730)", "Sommer-PMV (ISO 7730)"),
        ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, pmv_s.abs()), Quantity::new(QuantityKind::Dimensionless, lim))
        .explanation(loc(format!("Summer PMV={pmv_s:.3} at {clo_summer:.2} clo; |PMV|≤{lim}"), format!("Sommer-PMV={pmv_s:.3} bei {clo_summer:.2} clo; |PMV|≤{lim}")));
        if pmv_s.abs() > lim {
            if let Some(t_req) = invert_t_op_for_pmv(zone.rh_percent, zone.air_speed_m_s, zone.metabolic_rate_met, clo_summer, lim * 0.55, pmv_s > 0.0) {
            bp_s = bp_s.remedy(Remedy::exactly(zone_subject(zone, path("tOpSummerC")), Quantity::new(QuantityKind::Temperature, zone.t_op_summer_c), Quantity::new(QuantityKind::Temperature, t_req),
                loc(format!("Set summer operative temperature to ≈{t_req:.1} °C so |PMV|≤{lim}."), format!("Operative Sommertemperatur auf ≈{t_req:.1} °C setzen, damit |PMV|≤{lim}."))));
            }
        }
        out.push(bp_s.build());
        let ppd_s = ppd_from_pmv(pmv_s);
        let mut bppd_s = CheckResult::assess(
            format!("din16798-1.ppd.summer.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§7.2.2", "PPD"),
            zone_subject(zone, path("tOpSummerC")), loc("Summer PPD (ISO 7730)", "Sommer-PPD (ISO 7730)"),
        ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, ppd_s), Quantity::new(QuantityKind::Dimensionless, plim))
        .explanation(loc(format!("Summer PPD={ppd_s:.1} %; limit {plim:.0} %"), format!("Sommer-PPD={ppd_s:.1} %; Grenzwert {plim:.0} %")));
        if ppd_s > plim {
            if let Some(t_req) = invert_t_op_for_pmv(zone.rh_percent, zone.air_speed_m_s, zone.metabolic_rate_met, clo_summer, lim * 0.55, pmv_s > 0.0) {
            bppd_s = bppd_s.remedy(Remedy::exactly(zone_subject(zone, path("tOpSummerC")), Quantity::new(QuantityKind::Temperature, zone.t_op_summer_c), Quantity::new(QuantityKind::Temperature, t_req),
                loc(format!("Adjust summer θ_op to ≈{t_req:.1} °C to keep PPD ≤{plim:.0} %."), format!("Operative Sommertemperatur auf ≈{t_req:.1} °C anpassen, damit PPD ≤{plim:.0} %."))));
            }
        }
        out.push(bppd_s.build());

        let pmv_w = pmv_iso7730(zone.t_op_winter_c, zone.rh_percent, zone.air_speed_m_s, zone.metabolic_rate_met, clo_winter);
        let mut bp_w = CheckResult::assess(
            format!("din16798-1.pmv.winter.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§7.2.2", "PMV"),
            zone_subject(zone, path("tOpWinterC")), loc("Winter PMV (ISO 7730)", "Winter-PMV (ISO 7730)"),
        ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, pmv_w.abs()), Quantity::new(QuantityKind::Dimensionless, lim))
        .explanation(loc(format!("Winter PMV={pmv_w:.3} at {clo_winter:.2} clo; |PMV|≤{lim}"), format!("Winter-PMV={pmv_w:.3} bei {clo_winter:.2} clo; |PMV|≤{lim}")));
        if pmv_w.abs() > lim {
            if let Some(t_req) = invert_t_op_for_pmv(zone.rh_percent, zone.air_speed_m_s, zone.metabolic_rate_met, clo_winter, lim * 0.55, pmv_w > 0.0) {
            bp_w = bp_w.remedy(Remedy::exactly(zone_subject(zone, path("tOpWinterC")), Quantity::new(QuantityKind::Temperature, zone.t_op_winter_c), Quantity::new(QuantityKind::Temperature, t_req),
                loc(format!("Set winter operative temperature to ≈{t_req:.1} °C so |PMV|≤{lim}."), format!("Operative Wintertemperatur auf ≈{t_req:.1} °C setzen, damit |PMV|≤{lim}."))));
            }
        }
        out.push(bp_w.build());
        let ppd_w = ppd_from_pmv(pmv_w);
        let mut bppd_w = CheckResult::assess(
            format!("din16798-1.ppd.winter.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§7.2.2", "PPD"),
            zone_subject(zone, path("tOpWinterC")), loc("Winter PPD (ISO 7730)", "Winter-PPD (ISO 7730)"),
        ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, ppd_w), Quantity::new(QuantityKind::Dimensionless, plim))
        .explanation(loc(format!("Winter PPD={ppd_w:.1} %; limit {plim:.0} %"), format!("Winter-PPD={ppd_w:.1} %; Grenzwert {plim:.0} %")));
        if ppd_w > plim {
            if let Some(t_req) = invert_t_op_for_pmv(zone.rh_percent, zone.air_speed_m_s, zone.metabolic_rate_met, clo_winter, lim * 0.55, pmv_w > 0.0) {
            bppd_w = bppd_w.remedy(Remedy::exactly(zone_subject(zone, path("tOpWinterC")), Quantity::new(QuantityKind::Temperature, zone.t_op_winter_c), Quantity::new(QuantityKind::Temperature, t_req),
                loc(format!("Adjust winter θ_op to ≈{t_req:.1} °C to keep PPD ≤{plim:.0} %."), format!("Operative Wintertemperatur auf ≈{t_req:.1} °C anpassen, damit PPD ≤{plim:.0} %."))));
            }
        }
        out.push(bppd_w.build());
    }
}

/// ⚙️ Checks the ventilation method, the outdoor air flow and the carbon dioxide level of a zone.
#[allow(clippy::too_many_arguments, reason = "the extracted phase keeps the locals of its original function as explicit parameters")]
fn check_zone_air_quality(
    doc: &semio_s_artifact_norm_din16798_model::Din16798Snapshot,
    zone: &ZoneDocument,
    annex: &annex_params::AnnexParams,
    out: &mut Vec<CheckResult>,
    cat: ComfortCategory,
    usage: OccupancyType,
    pollution: PollutionClass,
    method: VentMethod,
    path: &dyn Fn(&str) -> String,
    linked_name: &str,
    linked_type: Option<VentSystemType>,
) {
    use part_1::*;
    let method_delta = co2_above_outdoor_ppm(annex.choice, cat);
    let q_design = required_outdoor_air_for_method(annex.choice, method, zone.occupants, zone.floor_area_m2, cat, usage, pollution, method_delta);
    let q_method = q_design.unwrap_or(0.0);
    let q_hygiene = hygienic_outdoor_air_minimum_m3_h(zone.occupants, usage);
    let q_req = q_method.max(q_hygiene);
    let hygiene_clause = match annex.choice {
        AnnexChoice::De => "NA.5.1.1",
        AnnexChoice::En => "B.3.1.1",
    };
    let q = zone.outdoor_air_supplied_m3_h;
    let method_clause = match (method, usage, annex.choice) {
        (VentMethod::Method3PredefinedRates, _, AnnexChoice::De)
            => ("NA.5.1.4", "Applicability of Method 3", "Anwendbarkeit von Verfahren 3"),
        (_, OccupancyType::Residential, AnnexChoice::De)
            => ("NA.5.2", "Dwelling ventilation design basis", "Auslegungsgrundlage für Wohnungslüftung"),
        (VentMethod::Method1PerceivedAirQuality, OccupancyType::Residential, _)
            => ("Table B.11 (3)", "Method 1 residential supply", "Methode 1 Wohnraum-Zuluft"),
        (VentMethod::Method1PerceivedAirQuality, _, AnnexChoice::De)
            => ("Tables NA.6/NA.7", "Method 1 perceived air quality", "Methode 1 empfundene Luftqualität"),
        (VentMethod::Method1PerceivedAirQuality, _, AnnexChoice::En)
            => ("Tables B.6/B.7", "Method 1 perceived air quality", "Methode 1 empfundene Luftqualität"),
        (VentMethod::Method2LimitConcentration, _, AnnexChoice::De)
            => ("Table NA.9", "Method 2 CO₂ limit concentration", "Methode 2 CO₂-Grenzkonzentration"),
        (VentMethod::Method2LimitConcentration, _, AnnexChoice::En)
            => ("Table B.9", "Method 2 CO₂ limit concentration", "Methode 2 CO₂-Grenzkonzentration"),
        (VentMethod::Method3PredefinedRates, OccupancyType::Residential, _)
            => ("Table B.11 (2)", "Method 3 residential supply per person", "Methode 3 Wohnraum-Zuluft je Person"),
        (VentMethod::Method3PredefinedRates, _, _)
            => ("Table B.10", "Method 3 predefined office rates", "Methode 3 vorgegebene Büro-Luftvolumenströme"),
    };
    let system_note = match linked_type {
        Some(VentSystemType::Natural) => ("natural ventilation system", "natürliche Lüftung"),
        Some(VentSystemType::DecentralMech) => ("decentral mechanical system", "dezentrale mechanische Lüftung"),
        Some(VentSystemType::CentralMech) => ("central mechanical system", "zentrale mechanische Lüftung"),
        None => ("unresolved system", "unaufgelöste Anlage"),
    };
    let governing_clause = if q_hygiene > q_method { hygiene_clause } else { method_clause.0 };
    let (mut explanation_en, mut explanation_de) = (
        format!("Supplied {q:.1} m³/h. Required {q_req:.1} m³/h. Served by «{linked_name}» ({}).", system_note.0),
        format!("Zugeführt {q:.1} m³/h. Erforderlich {q_req:.1} m³/h. Versorgt durch «{linked_name}» ({}).", system_note.1),
    );
    if q_hygiene > 0.0 && q_design.is_ok() {
        explanation_en.push_str(&format!(" Design method {q_method:.1} m³/h. Hygienic minimum {q_hygiene:.1} m³/h at 4 L/s per person with complete mixing ({hygiene_clause})."));
        explanation_de.push_str(&format!(" Auslegungsverfahren {q_method:.1} m³/h. Hygienischer Mindestwert {q_hygiene:.1} m³/h bei 4 l/s je Person und vollständiger Vermischung ({hygiene_clause})."));
    }
    if let Err(gap) = q_design {
        let reason = match gap {
            OutdoorAirBasisGap::OutsideDefaultTable => (
                "The published default table does not define an airflow rate for these inputs. Use a verified project design basis.",
                "Die veröffentlichte Standardtabelle enthält für diese Angaben keinen Luftvolumenstrom. Eine verifizierte Projektauslegung verwenden.",
            ),
            OutdoorAirBasisGap::GermanMethod3 => (
                "The German national annex excludes Method 3. Use the design basis applicable to the building.",
                "Der deutsche Nationale Anhang schließt Verfahren 3 aus. Die für das Gebäude geltende Auslegungsgrundlage verwenden.",
            ),
            OutdoorAirBasisGap::GermanResidentialBasis => (
                "German dwelling design requires the entire unit's floor area. Apply DIN 1946-6 up to 250 m² and the non-residential design rules above 250 m².",
                "Die deutsche Wohnungslüftung erfordert die Fläche der gesamten Nutzeinheit. Bis 250 m² gilt DIN 1946-6, oberhalb gelten die Auslegungsregeln für Nichtwohngebäude.",
            ),
            OutdoorAirBasisGap::GermanVeryLowPollutionEvidence => (
                "NA.6.2 requires written client approval, product-selection documentation and an indoor-air acceptance plan for the very-low-pollution design class.",
                "Für die Auslegung als sehr schadstoffarm nach NA.6.2 sind die schriftliche Zustimmung des Bauherrn, die Dokumentation der Bauproduktauswahl und ein Abnahmeplan für die Raumluft erforderlich.",
            ),
            OutdoorAirBasisGap::NoPositiveCo2Gradient => (
                "The permitted indoor CO₂ concentration must be above the outdoor concentration for dilution to determine a flow.",
                "Die zulässige CO₂-Innenkonzentration muss über der Außenkonzentration liegen, damit sich ein Verdünnungsvolumenstrom bestimmen lässt.",
            ),
        };
        explanation_en = reason.0.into();
        explanation_de = reason.1.into();
        if q_hygiene > 0.0 {
            explanation_en.push_str(&format!(" Complete-mixing hygienic minimum {q_hygiene:.1} m³/h."));
            explanation_de.push_str(&format!(" Hygienischer Mindestwert bei vollständiger Vermischung {q_hygiene:.1} m³/h."));
        }
    }
    let mut bv = CheckResult::assess(
        format!("din16798-1.vent.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", governing_clause, "q"),
        zone_subject(zone, path("outdoorAirSuppliedM3H")), loc(method_clause.1, method_clause.2),
    ).annex(doc.annex).minimum(Quantity::new(QuantityKind::VentilationRate, q), Quantity::new(QuantityKind::VentilationRate, q_req))
    .explanation(loc(explanation_en, explanation_de));
    if q_design.is_err() && q >= q_req {
        bv = bv.status(CheckStatus::UnverifiedBasis);
    }
    if q < q_req {
        bv = bv.remedy(Remedy::at_least(zone_subject(zone, path("outdoorAirSuppliedM3H")), Quantity::new(QuantityKind::VentilationRate, q), Quantity::new(QuantityKind::VentilationRate, q_req),
            loc(
                format!("Raise outdoor air on this zone (system «{linked_name}») to at least {q_req:.1} m³/h."),
                format!("Außenluft dieser Zone (Anlage «{linked_name}») auf mindestens {q_req:.1} m³/h erhöhen."),
            )));
    }
    out.push(bv.build());

    let dco2 = (zone.co2_ppm - doc.outdoor_co2_ppm).max(0.0);
    let effective_limit_above = method_delta;
    let co2_clause = match (annex.choice, usage) {
        (AnnexChoice::De, OccupancyType::Residential) => "NA.5.2",
        (AnnexChoice::De, _) => "Table NA.9",
        (AnnexChoice::En, OccupancyType::Residential) => "Table B.12 (living rooms)",
        (AnnexChoice::En, _) => "Table B.9",
    };
    let workplace_en = if annex.choice == AnnexChoice::De && usage != OccupancyType::Residential && zone.co2_ppm > 1000.0 {
        " For German workplaces, review the measurement-based actions in ASR A3.6."
    } else { "" };
    let workplace_de = if workplace_en.is_empty() { "" } else {
        " Für deutsche Arbeitsstätten die messwertbezogenen Maßnahmen der ASR A3.6 berücksichtigen."
    };
    let mut bc = CheckResult::assess(
        format!("din16798-1.co2.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", co2_clause, "CO2"),
        zone_subject(zone, path("co2Ppm")), loc("CO₂ above outdoor", "CO₂ über Außenluft"),
    ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, dco2), Quantity::new(QuantityKind::Dimensionless, effective_limit_above))
    .explanation(loc(format!("ΔCO₂={dco2:.0} ppm; limit {effective_limit_above:.0} ppm (Cat {:?}, {}).{workplace_en}", cat, annex.choice.label()), format!("ΔCO₂={dco2:.0} ppm; Grenzwert {effective_limit_above:.0} ppm (Kat. {:?}, {}).{workplace_de}", cat, annex.choice.label())));
    if annex.choice == AnnexChoice::De && usage == OccupancyType::Residential {
        bc = bc.status(CheckStatus::UnverifiedBasis).explanation(loc(
            "Confirm the dwelling-unit design basis under NA.5.2 before applying a design criterion.",
            "Vor Anwendung eines Auslegungskriteriums die Auslegungsgrundlage der Wohneinheit nach NA.5.2 bestätigen.",
        ));
    } else if dco2 > effective_limit_above {
        let target_ppm = doc.outdoor_co2_ppm + effective_limit_above;
        let q_boost = if dco2 > 0.0 { q * (dco2 / effective_limit_above) } else { q_req };
        let q_needed = q_boost.max(q_req);
        bc = bc
            .remedy(Remedy::at_most(
                zone_subject(zone, path("co2Ppm")),
                Quantity::new(QuantityKind::Dimensionless, zone.co2_ppm),
                Quantity::new(QuantityKind::Dimensionless, target_ppm),
                loc(
                    format!("Reduce design CO₂ to at most {target_ppm:.0} ppm, or raise outdoor air on system «{linked_name}» to ≥{q_boost:.0} m³/h."),
                    format!("Auslegungs-CO₂ auf höchstens {target_ppm:.0} ppm senken oder Außenluft an Anlage «{linked_name}» auf ≥{q_boost:.0} m³/h erhöhen."),
                ),
            ))
            .remedy(Remedy::at_least(
                zone_subject(zone, path("outdoorAirSuppliedM3H")),
                Quantity::new(QuantityKind::VentilationRate, q),
                Quantity::new(QuantityKind::VentilationRate, q_needed),
                loc(
                    format!("Raise outdoor-air flow to at least {q_needed:.1} m³/h to dilute CO₂."),
                    format!("Außenluftvolumenstrom auf mindestens {q_needed:.1} m³/h erhöhen zur CO₂-Verdünnung."),
                ),
            ));
    }
    out.push(bc.build());
}

/// 💧 Reports the humidity control design values and checks the known humidification equipment.
fn check_zone_humidity(
    doc: &semio_s_artifact_norm_din16798_model::Din16798Snapshot,
    zone: &ZoneDocument,
    linked: Option<&VentSystemDocument>,
    out: &mut Vec<CheckResult>,
    cat: ComfortCategory,
    path: &dyn Fn(&str) -> String,
) {
    let clause = if doc.annex == AnnexChoice::De { "Table NA.10" } else { "Table B.16" };
    let subject = zone_subject(zone, path("rhPercent"));
    let mut brh = CheckResult::assess(
        format!("din16798-1.rh.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", clause, "RH"),
        subject.clone(), loc("Humidity control design", "Auslegung der Luftfeuchteregelung"),
    ).annex(doc.annex);
    let Some((rh_min, rh_max)) = part_1::humidity_band(doc.annex, cat) else {
        out.push(brh.status(CheckStatus::Info).explanation(loc(
            "The table provides humidity-control design references for categories I through III.",
            "Die Tabelle enthält Auslegungsrichtwerte für die Luftfeuchteregelung für die Kategorien I bis III.",
        )).build());
        return;
    };
    let rh = zone.rh_percent;
    let humidifier = linked.is_some_and(|system| system.humidification_provided_kg_h > 0.0);
    let current = Quantity::new(QuantityKind::Dimensionless, rh);
    let minimum = Quantity::new(QuantityKind::Dimensionless, rh_min);
    let maximum = Quantity::new(QuantityKind::Dimensionless, rh_max);
    if rh <= rh_max && humidifier {
        brh = brh.minimum(current, minimum);
        if rh < rh_min {
            brh = brh.remedy(Remedy::at_least(subject, current, minimum, loc(
                format!("Raise the humidification design setpoint to at least {rh_min:.0} %."),
                format!("Auslegungssollwert der Befeuchtung auf mindestens {rh_min:.0} % erhöhen."),
            )));
        }
    } else {
        brh = brh.utilization(current, maximum).status(CheckStatus::Info);
    }
    out.push(brh.explanation(loc(
        format!("Design relative humidity {rh:.0} %. For installed humidity control, the humidification reference is {rh_min:.0} % and the dehumidification reference is {rh_max:.0} %."),
        format!("Relative Auslegungsfeuchte {rh:.0} %. Bei vorhandener Luftfeuchteregelung beträgt der Richtwert für Befeuchtung {rh_min:.0} % und für Entfeuchtung {rh_max:.0} %."),
    )).build());
}

/// 🔅️ Checks the available illuminance, sound and draught inputs.
fn check_zone_light_noise_and_draught(
    doc: &semio_s_artifact_norm_din16798_model::Din16798Snapshot,
    zone: &ZoneDocument,
    out: &mut Vec<CheckResult>,
    cat: ComfortCategory,
    usage: OccupancyType,
    path: &dyn Fn(&str) -> String,
) {
    use part_1::*;
    let lighting_rule = if doc.annex == AnnexChoice::De { "NA.7 / DIN EN 12464-1" } else { "Table B.18" };
    let light_target = zone_subject(zone, path("illuminanceLx"));
    let mut bi = CheckResult::assess(
        format!("din16798-1.illuminance.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", lighting_rule, "E_m"),
        light_target.clone(), loc("Maintained task illuminance", "Wartungswert der Beleuchtungsstärke"),
    ).annex(doc.annex);
    if let Some(minimum) = illuminance_min_lx(usage) {
        let reading = Quantity::new(QuantityKind::Dimensionless, zone.illuminance_lx);
        let required = Quantity::new(QuantityKind::Dimensionless, minimum);
        bi = bi.minimum(reading, required).explanation(loc(
            format!("Maintained task illuminance {:.0} lx. Reference minimum {minimum:.0} lx.", zone.illuminance_lx),
            format!("Wartungswert der Beleuchtungsstärke {:.0} lx. Referenzminimum {minimum:.0} lx.", zone.illuminance_lx),
        ));
        if zone.illuminance_lx < minimum {
            bi = bi.remedy(Remedy::at_least(light_target, reading, required, loc(
                format!("Raise the maintained task illuminance to at least {minimum:.0} lx."),
                format!("Wartungswert der Beleuchtungsstärke auf mindestens {minimum:.0} lx erhöhen."),
            )));
        }
    } else {
        bi = bi.status(CheckStatus::UnverifiedBasis).explanation(loc(
            "Select an applicable task-specific criterion for this room's lighting design.",
            "Ein zutreffendes tätigkeitsbezogenes Kriterium für die Lichtplanung dieses Raums wählen.",
        ));
    }
    out.push(bi.build());

    let sound_rule = if doc.annex == AnnexChoice::De { "Table NA.11" } else { "Table B.20" };
    let sound_target = zone_subject(zone, path("noiseDb"));
    let mut bn = CheckResult::assess(
        format!("din16798-1.noise.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", sound_rule, "L_Aeq,nT"),
        sound_target.clone(), loc("Indoor building-services noise", "Anlagenbedingter Innengeräuschpegel"),
    ).annex(doc.annex);
    if let Some(maximum) = acoustic_limit_db(doc.annex, usage, cat) {
        let reading = Quantity::new(QuantityKind::Dimensionless, zone.noise_db);
        let limit = Quantity::new(QuantityKind::Dimensionless, maximum);
        let profile = match usage {
            OccupancyType::Office => ("Small-office reference.", "Richtwert für kleine Büros."),
            OccupancyType::Residential => ("Living-room reference.", "Richtwert für Wohnzimmer."),
            OccupancyType::Retail => ("Shop reference.", "Richtwert für Ladengeschäfte."),
            OccupancyType::Kitchen => ("Commercial kitchen reference.", "Richtwert für gewerbliche Küchen."),
            _ => ("", ""),
        };
        bn = bn.utilization(reading, limit).explanation(loc(
            format!("Equipment L_Aeq,nT {:.1} dB(A). Design maximum {maximum:.1} dB(A). {}", zone.noise_db, profile.0),
            format!("Anlagenbedingter L_Aeq,nT {:.1} dB(A). Auslegungsmaximum {maximum:.1} dB(A). {}", zone.noise_db, profile.1),
        ));
        if zone.noise_db > maximum {
            bn = bn.remedy(Remedy::at_most(sound_target, reading, limit, loc(
                format!("Reduce building-services noise to at most {maximum:.1} dB(A)."),
                format!("Anlagenbedingten Geräuschpegel auf höchstens {maximum:.1} dB(A) senken."),
            )));
        }
    } else {
        bn = bn.status(CheckStatus::UnverifiedBasis).explanation(loc(
            "The building-services noise table specifies categories I through III.",
            "Die Tabelle für anlagenbedingte Geräuschpegel enthält die Kategorien I bis III.",
        ));
    }
    out.push(bn.build());

    let subject = zone_subject(zone, path("airSpeedMS"));
    let clause = if doc.annex == AnnexChoice::De { "Table NA.4" } else { "Table B.3" };
    let mut bd = CheckResult::assess(
        format!("din16798-1.draught.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", clause, "DR"),
        subject.clone(), loc("Draught rate (ISO 7730)", "Zugluftrate (ISO 7730)"),
    ).annex(doc.annex);
    let t = zone.t_op_summer_c;
    let speed = zone.air_speed_m_s;
    let turbulence = zone.turbulence_intensity_percent;
    let Some(limit) = draught_limit_percent(doc.annex, cat) else {
        out.push(bd.not_applicable(loc(
            "The local draught table contains limits for categories I through III.",
            "Die Tabelle für lokale Zugluft enthält Grenzwerte für die Kategorien I bis III.",
        )).build());
        return;
    };
    let Some(dr) = draught_rate_percent(t, speed, turbulence) else {
        out.push(bd.status(CheckStatus::UnverifiedBasis).explanation(loc(
            "The draught equation requires local air temperature between 20 and 26 °C, speed below 0.5 m/s and turbulence from 10 to 60 %.",
            "Die Zugluftgleichung erfordert eine örtliche Lufttemperatur von 20 bis 26 °C, Luftgeschwindigkeiten unter 0,5 m/s und einen Turbulenzgrad von 10 bis 60 %.",
        )).build());
        return;
    };
    bd = bd.utilization(Quantity::new(QuantityKind::Dimensionless, dr), Quantity::new(QuantityKind::Dimensionless, limit))
        .explanation(loc(
            format!("Estimated DR {dr:.1} % at v={speed:.2} m/s and turbulence {turbulence:.0} %, assuming local air temperature equals the operative design temperature of {t:.1} °C. Category limit {limit:.0} %."),
            format!("Geschätzte Zugluftrate {dr:.1} % bei v={speed:.2} m/s und Turbulenzgrad {turbulence:.0} %, bei Übereinstimmung von örtlicher Lufttemperatur und operativer Auslegungstemperatur von {t:.1} °C. Kategoriegrenzwert {limit:.0} %."),
        ));
    if dr > limit {
        let maximum_speed = maximum_air_speed_for_draught(t, speed, turbulence, limit);
        bd = bd.remedy(Remedy::at_most(
            subject,
            Quantity::new(QuantityKind::Dimensionless, speed),
            Quantity::new(QuantityKind::Dimensionless, maximum_speed),
            loc(
                format!("Reduce local air speed to at most {maximum_speed:.3} m/s under the stated temperature and turbulence."),
                format!("Lokale Luftgeschwindigkeit bei der angegebenen Temperatur und Turbulenz auf höchstens {maximum_speed:.3} m/s senken."),
            ),
        ));
    }
    out.push(bd.build());
}
