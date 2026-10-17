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

    // Referential integrity — zone must link an existing ventilation system
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

    // Operative temperature — summer design
    let (t_min, t_max) = operative_temp_band(cat, "summer");
    let t = zone.t_op_summer_c;
    let mut b = CheckResult::assess(
        format!("din16798-1.top.summer.{}", zone.id), "DIN EN 16798-1",
        ClauseId::new("EN 16798-1", "Table B.2", "θ_op"),
        zone_subject(zone, path("tOpSummerC")),
        loc("Summer operative temperature", "Operative Sommertemperatur"),
    ).annex(doc.annex);
    if t < t_min {
        b = b.minimum(Quantity::new(QuantityKind::Temperature, t), Quantity::new(QuantityKind::Temperature, t_min))
            .explanation(loc(format!("Summer θ_op={t:.1} °C is below the {t_min:.1} °C category floor."), format!("Operative Sommertemperatur θ_op={t:.1} °C unterschreitet die Kategorieuntergrenze {t_min:.1} °C.")))
            .remedy(Remedy::at_least(zone_subject(zone, path("tOpSummerC")), Quantity::new(QuantityKind::Temperature, t), Quantity::new(QuantityKind::Temperature, t_min),
                loc(format!("Raise summer operative temperature to at least {t_min:.1} °C."), format!("Operative Sommertemperatur auf mindestens {t_min:.1} °C erhöhen."))));
    } else if t > t_max {
        b = b.utilization(Quantity::new(QuantityKind::Temperature, t), Quantity::new(QuantityKind::Temperature, t_max))
            .explanation(loc(format!("Summer θ_op={t:.1} °C exceeds the {t_max:.1} °C category ceiling."), format!("Operative Sommertemperatur θ_op={t:.1} °C überschreitet die Kategorieobergrenze {t_max:.1} °C.")))
            .remedy(Remedy::at_most(zone_subject(zone, path("tOpSummerC")), Quantity::new(QuantityKind::Temperature, t), Quantity::new(QuantityKind::Temperature, t_max),
                loc(format!("Lower summer operative temperature to at most {t_max:.1} °C."), format!("Operative Sommertemperatur auf höchstens {t_max:.1} °C senken."))));
    } else {
        b = b.utilization(Quantity::new(QuantityKind::Temperature, t), Quantity::new(QuantityKind::Temperature, t_max))
            .explanation(loc(format!("Summer θ_op={t:.1} °C lies within {t_min:.1}–{t_max:.1} °C."), format!("Operative Sommertemperatur θ_op={t:.1} °C liegt im Band {t_min:.1}–{t_max:.1} °C.")));
    }
    out.push(b.build());

    // Winter operative temperature
    let (tw_min, tw_max) = operative_temp_band(cat, "winter");
    let tw = zone.t_op_winter_c;
    let mut bw = CheckResult::assess(
        format!("din16798-1.top.winter.{}", zone.id), "DIN EN 16798-1",
        ClauseId::new("EN 16798-1", "Table B.2", "θ_op,w"),
        zone_subject(zone, path("tOpWinterC")),
        loc("Winter operative temperature", "Operative Wintertemperatur"),
    ).annex(doc.annex);
    if tw < tw_min {
        bw = bw.minimum(Quantity::new(QuantityKind::Temperature, tw), Quantity::new(QuantityKind::Temperature, tw_min))
            .explanation(loc(format!("Winter θ_op={tw:.1} °C is below the {tw_min:.1} °C category floor."), format!("Operative Wintertemperatur θ_op={tw:.1} °C unterschreitet die Kategorieuntergrenze {tw_min:.1} °C.")))
            .remedy(Remedy::at_least(zone_subject(zone, path("tOpWinterC")), Quantity::new(QuantityKind::Temperature, tw), Quantity::new(QuantityKind::Temperature, tw_min),
                loc(format!("Raise winter operative temperature to at least {tw_min:.1} °C."), format!("Operative Wintertemperatur auf mindestens {tw_min:.1} °C erhöhen."))));
    } else if tw > tw_max {
        bw = bw.utilization(Quantity::new(QuantityKind::Temperature, tw), Quantity::new(QuantityKind::Temperature, tw_max))
            .explanation(loc(format!("Winter θ_op={tw:.1} °C exceeds the {tw_max:.1} °C category ceiling."), format!("Operative Wintertemperatur θ_op={tw:.1} °C überschreitet die Kategorieobergrenze {tw_max:.1} °C.")))
            .remedy(Remedy::at_most(zone_subject(zone, path("tOpWinterC")), Quantity::new(QuantityKind::Temperature, tw), Quantity::new(QuantityKind::Temperature, tw_max),
                loc(format!("Lower winter operative temperature to at most {tw_max:.1} °C."), format!("Operative Wintertemperatur auf höchstens {tw_max:.1} °C senken."))));
    } else {
        bw = bw.utilization(Quantity::new(QuantityKind::Temperature, tw), Quantity::new(QuantityKind::Temperature, tw_max))
            .explanation(loc(format!("Winter θ_op={tw:.1} °C lies within {tw_min:.1}–{tw_max:.1} °C."), format!("Operative Wintertemperatur θ_op={tw:.1} °C liegt im Band {tw_min:.1}–{tw_max:.1} °C.")));
    }
    out.push(bw.build());

    // PMV / PPD (fixed HVAC, both seasons) vs adaptive Annex B.2 — ComfortModel wire is the sole gate.
    let clo_summer = zone.clothing_clo;
    let clo_winter = 1.0; // EN 16798-1 typical winter clothing when only one clo field is stored
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
        let centre = adaptive_comfort_temperature_c(doc.theta_rm_c);
        let band = adaptive_band_k(cat);
        let deviation = (zone.t_op_summer_c - centre).abs();
        let mut ba = CheckResult::assess(
            format!("din16798-1.adaptive.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "Annex B.2", "θ_c"),
            zone_subject(zone, path("tOpSummerC")), loc("Adaptive comfort (running-mean)", "Adaptiver Komfort (Außenluftmittel)"),
        ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Temperature, deviation), Quantity::new(QuantityKind::Temperature, band))
        .explanation(loc(
            format!("|θ_op−θ_c|={deviation:.2} K; Cat band ±{band:.1} K around θ_c={centre:.1} °C (θ_rm={:.1} °C)", doc.theta_rm_c),
            format!("|θ_op−θ_c|={deviation:.2} K; Kat.-Band ±{band:.1} K um θ_c={centre:.1} °C (θ_rm={:.1} °C)", doc.theta_rm_c),
        ));
        if deviation > band {
            let target = centre + band.copysign(zone.t_op_summer_c - centre);
            ba = ba.remedy(Remedy::at_most(zone_subject(zone, path("tOpSummerC")), Quantity::new(QuantityKind::Temperature, zone.t_op_summer_c), Quantity::new(QuantityKind::Temperature, target),
                loc(format!("Adjust operative temperature toward {centre:.1} °C (within ±{band:.1} K)."), format!("Operative Temperatur Richtung {centre:.1} °C anpassen (innerhalb ±{band:.1} K)."))));
        }
        out.push(ba.build());
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
        // Summer PMV/PPD — subject clothing (design summer clo)
        let pmv_s = pmv_iso7730(zone.t_op_summer_c, zone.rh_percent, zone.air_speed_m_s, zone.metabolic_rate_met, clo_summer);
        let mut bp_s = CheckResult::assess(
            format!("din16798-1.pmv.summer.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§7.2.2", "PMV"),
            zone_subject(zone, path("tOpSummerC")), loc("Summer PMV (ISO 7730)", "Sommer-PMV (ISO 7730)"),
        ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, pmv_s.abs()), Quantity::new(QuantityKind::Dimensionless, lim))
        .explanation(loc(format!("Summer PMV={pmv_s:.3} at {clo_summer:.2} clo; |PMV|≤{lim}"), format!("Sommer-PMV={pmv_s:.3} bei {clo_summer:.2} clo; |PMV|≤{lim}")));
        if pmv_s.abs() > lim {
            let t_req = invert_t_op_for_pmv(zone.rh_percent, zone.air_speed_m_s, zone.metabolic_rate_met, clo_summer, lim * 0.55, pmv_s > 0.0);
            bp_s = bp_s.remedy(Remedy::exactly(zone_subject(zone, path("tOpSummerC")), Quantity::new(QuantityKind::Temperature, zone.t_op_summer_c), Quantity::new(QuantityKind::Temperature, t_req),
                loc(format!("Set summer operative temperature to ≈{t_req:.1} °C so |PMV|≤{lim}."), format!("Operative Sommertemperatur auf ≈{t_req:.1} °C setzen, damit |PMV|≤{lim}."))));
        }
        out.push(bp_s.build());
        let ppd_s = ppd_from_pmv(pmv_s);
        let mut bppd_s = CheckResult::assess(
            format!("din16798-1.ppd.summer.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§7.2.2", "PPD"),
            zone_subject(zone, path("tOpSummerC")), loc("Summer PPD (ISO 7730)", "Sommer-PPD (ISO 7730)"),
        ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, ppd_s), Quantity::new(QuantityKind::Dimensionless, plim))
        .explanation(loc(format!("Summer PPD={ppd_s:.1} %; limit {plim:.0} %"), format!("Sommer-PPD={ppd_s:.1} %; Grenzwert {plim:.0} %")));
        if ppd_s > plim {
            let t_req = invert_t_op_for_pmv(zone.rh_percent, zone.air_speed_m_s, zone.metabolic_rate_met, clo_summer, lim * 0.55, pmv_s > 0.0);
            bppd_s = bppd_s.remedy(Remedy::exactly(zone_subject(zone, path("tOpSummerC")), Quantity::new(QuantityKind::Temperature, zone.t_op_summer_c), Quantity::new(QuantityKind::Temperature, t_req),
                loc(format!("Adjust summer θ_op to ≈{t_req:.1} °C to keep PPD ≤{plim:.0} %."), format!("Operative Sommertemperatur auf ≈{t_req:.1} °C anpassen, damit PPD ≤{plim:.0} %."))));
        }
        out.push(bppd_s.build());

        // Winter PMV/PPD — EN 16798-1 typical winter clothing 1.0 clo
        let pmv_w = pmv_iso7730(zone.t_op_winter_c, zone.rh_percent, zone.air_speed_m_s, zone.metabolic_rate_met, clo_winter);
        let mut bp_w = CheckResult::assess(
            format!("din16798-1.pmv.winter.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§7.2.2", "PMV"),
            zone_subject(zone, path("tOpWinterC")), loc("Winter PMV (ISO 7730)", "Winter-PMV (ISO 7730)"),
        ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, pmv_w.abs()), Quantity::new(QuantityKind::Dimensionless, lim))
        .explanation(loc(format!("Winter PMV={pmv_w:.3} at {clo_winter:.2} clo; |PMV|≤{lim}"), format!("Winter-PMV={pmv_w:.3} bei {clo_winter:.2} clo; |PMV|≤{lim}")));
        if pmv_w.abs() > lim {
            let t_req = invert_t_op_for_pmv(zone.rh_percent, zone.air_speed_m_s, zone.metabolic_rate_met, clo_winter, lim * 0.55, pmv_w > 0.0);
            bp_w = bp_w.remedy(Remedy::exactly(zone_subject(zone, path("tOpWinterC")), Quantity::new(QuantityKind::Temperature, zone.t_op_winter_c), Quantity::new(QuantityKind::Temperature, t_req),
                loc(format!("Set winter operative temperature to ≈{t_req:.1} °C so |PMV|≤{lim}."), format!("Operative Wintertemperatur auf ≈{t_req:.1} °C setzen, damit |PMV|≤{lim}."))));
        }
        out.push(bp_w.build());
        let ppd_w = ppd_from_pmv(pmv_w);
        let mut bppd_w = CheckResult::assess(
            format!("din16798-1.ppd.winter.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§7.2.2", "PPD"),
            zone_subject(zone, path("tOpWinterC")), loc("Winter PPD (ISO 7730)", "Winter-PPD (ISO 7730)"),
        ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, ppd_w), Quantity::new(QuantityKind::Dimensionless, plim))
        .explanation(loc(format!("Winter PPD={ppd_w:.1} %; limit {plim:.0} %"), format!("Winter-PPD={ppd_w:.1} %; Grenzwert {plim:.0} %")));
        if ppd_w > plim {
            let t_req = invert_t_op_for_pmv(zone.rh_percent, zone.air_speed_m_s, zone.metabolic_rate_met, clo_winter, lim * 0.55, pmv_w > 0.0);
            bppd_w = bppd_w.remedy(Remedy::exactly(zone_subject(zone, path("tOpWinterC")), Quantity::new(QuantityKind::Temperature, zone.t_op_winter_c), Quantity::new(QuantityKind::Temperature, t_req),
                loc(format!("Adjust winter θ_op to ≈{t_req:.1} °C to keep PPD ≤{plim:.0} %."), format!("Operative Wintertemperatur auf ≈{t_req:.1} °C anpassen, damit PPD ≤{plim:.0} %."))));
        }
        out.push(bppd_w.build());
    }

    // Ventilation rate — EN 16798-1 §6.3 method selector
    let dlim_for_method = co2_above_outdoor_ppm(cat);
    let abs_lim_early = match usage {
        OccupancyType::Residential => annex.co2_absolute_residential_ppm,
        OccupancyType::Classroom => annex.co2_absolute_classroom_ppm,
        _ => annex.co2_absolute_other_ppm,
    };
    let method_delta = if doc.annex == AnnexChoice::De { (abs_lim_early - doc.outdoor_co2_ppm).min(dlim_for_method) } else { dlim_for_method };
    let q_req = required_outdoor_air_for_method(method, zone.occupants, zone.floor_area_m2, cat, pollution, zone.metabolic_rate_met, method_delta);
    let q = zone.outdoor_air_supplied_m3_h;
    let method_clause = match method {
        VentMethod::Method1PerceivedAirQuality => ("Table B.6/B.7", "Method 1 perceived air quality", "Methode 1 empfundene Luftqualität"),
        VentMethod::Method2LimitConcentration => ("§6.3 method 2", "Method 2 limit concentration", "Methode 2 Grenzkonzentration"),
        VentMethod::Method3PredefinedRates => ("Annex B predefined", "Method 3 predefined rates", "Methode 3 vorgegebene Raten"),
    };
    let system_note = match linked_type {
        Some(VentSystemType::Natural) => ("natural ventilation system", "natürliche Lüftung"),
        Some(VentSystemType::DecentralMech) => ("decentral mechanical system", "dezentrale mechanische Lüftung"),
        Some(VentSystemType::CentralMech) => ("central mechanical system", "zentrale mechanische Lüftung"),
        None => ("unresolved system", "unaufgelöste Anlage"),
    };
    let mut bv = CheckResult::assess(
        format!("din16798-1.vent.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", method_clause.0, "q"),
        zone_subject(zone, path("outdoorAirSuppliedM3H")), loc(method_clause.1, method_clause.2),
    ).annex(doc.annex).minimum(Quantity::new(QuantityKind::VentilationRate, q), Quantity::new(QuantityKind::VentilationRate, q_req))
    .explanation(loc(
        format!("q={q:.1} m³/h; required {q_req:.1} m³/h via {}; served by «{linked_name}» ({})", method.as_str(), system_note.0),
        format!("q={q:.1} m³/h; erforderlich {q_req:.1} m³/h nach {}; versorgt durch «{linked_name}» ({})", method.as_str(), system_note.1),
    ));
    if q < q_req {
        bv = bv.remedy(Remedy::at_least(zone_subject(zone, path("outdoorAirSuppliedM3H")), Quantity::new(QuantityKind::VentilationRate, q), Quantity::new(QuantityKind::VentilationRate, q_req),
            loc(
                format!("Raise outdoor air on this zone (system «{linked_name}») to at least {q_req:.1} m³/h."),
                format!("Außenluft dieser Zone (Anlage «{linked_name}») auf mindestens {q_req:.1} m³/h erhöhen."),
            )));
    }
    out.push(bv.build());

    // CO2 above outdoor
    let dco2 = (zone.co2_ppm - doc.outdoor_co2_ppm).max(0.0);
    let dlim = co2_above_outdoor_ppm(cat);
    // Also respect DE absolute cap when annex=De
    let abs_lim = match usage {
        OccupancyType::Residential => annex.co2_absolute_residential_ppm,
        OccupancyType::Classroom => annex.co2_absolute_classroom_ppm,
        _ => annex.co2_absolute_other_ppm,
    };
    let effective_limit_above = if doc.annex == AnnexChoice::De {
        (abs_lim - doc.outdoor_co2_ppm).min(dlim)
    } else { dlim };
    let mut bc = CheckResult::assess(
        format!("din16798-1.co2.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", annex.choice.label(), "CO2"),
        zone_subject(zone, path("co2Ppm")), loc("CO₂ above outdoor", "CO₂ über Außenluft"),
    ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, dco2), Quantity::new(QuantityKind::Dimensionless, effective_limit_above))
    .explanation(loc(format!("ΔCO₂={dco2:.0} ppm; limit {effective_limit_above:.0} ppm (Cat {:?}, {})", cat, annex.choice.label()), format!("ΔCO₂={dco2:.0} ppm; Grenzwert {effective_limit_above:.0} ppm (Kat. {:?}, {})", cat, annex.choice.label())));
    if dco2 > effective_limit_above {
        let target_ppm = doc.outdoor_co2_ppm + effective_limit_above;
        // Remedy via more outdoor air (proportional) or lower CO2 design
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

    // Humidity
    let (rh_min, rh_max) = humidity_band(cat);
    let rh = zone.rh_percent;
    let mut brh = CheckResult::assess(
        format!("din16798-1.rh.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "Table B.16", "RH"),
        zone_subject(zone, path("rhPercent")), loc("Relative humidity", "Relative Feuchte"),
    ).annex(doc.annex);
    if rh < rh_min {
        brh = brh.minimum(Quantity::new(QuantityKind::Dimensionless, rh), Quantity::new(QuantityKind::Dimensionless, rh_min))
            .remedy(Remedy::at_least(zone_subject(zone, path("rhPercent")), Quantity::new(QuantityKind::Dimensionless, rh), Quantity::new(QuantityKind::Dimensionless, rh_min),
                loc(format!("Raise relative humidity to at least {rh_min:.0} %."), format!("Relative Feuchte auf mindestens {rh_min:.0} % erhöhen."))));
    } else if rh > rh_max {
        brh = brh.utilization(Quantity::new(QuantityKind::Dimensionless, rh), Quantity::new(QuantityKind::Dimensionless, rh_max))
            .remedy(Remedy::at_most(zone_subject(zone, path("rhPercent")), Quantity::new(QuantityKind::Dimensionless, rh), Quantity::new(QuantityKind::Dimensionless, rh_max),
                loc(format!("Lower relative humidity to at most {rh_max:.0} %."), format!("Relative Feuchte auf höchstens {rh_max:.0} % senken."))));
    } else {
        brh = brh.utilization(Quantity::new(QuantityKind::Dimensionless, rh), Quantity::new(QuantityKind::Dimensionless, rh_max));
    }
    out.push(brh.explanation(loc(format!("RH={rh:.0} %; band {rh_min:.0}–{rh_max:.0} %"), format!("RF={rh:.0} %; Band {rh_min:.0}–{rh_max:.0} %"))).build());

    // Illuminance
    let e_min = illuminance_min_lx(usage, cat);
    let mut bi = CheckResult::assess(
        format!("din16798-1.illuminance.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "Annex B", "E"),
        zone_subject(zone, path("illuminanceLx")), loc("Illuminance", "Beleuchtungsstärke"),
    ).annex(doc.annex).minimum(Quantity::new(QuantityKind::Dimensionless, zone.illuminance_lx), Quantity::new(QuantityKind::Dimensionless, e_min))
    .explanation(loc(format!("E={:.0} lx; min {:.0} lx", zone.illuminance_lx, e_min), format!("E={:.0} lx; min. {:.0} lx", zone.illuminance_lx, e_min)));
    if zone.illuminance_lx < e_min {
        bi = bi.remedy(Remedy::at_least(zone_subject(zone, path("illuminanceLx")), Quantity::new(QuantityKind::Dimensionless, zone.illuminance_lx), Quantity::new(QuantityKind::Dimensionless, e_min),
            loc(format!("Raise design illuminance to at least {e_min:.0} lx."), format!("Beleuchtungsstärke auf mindestens {e_min:.0} lx erhöhen."))));
    }
    out.push(bi.build());

    // Noise
    let nlim = acoustic_limit_db(cat, usage, annex);
    let mut bn = CheckResult::assess(
        format!("din16798-1.noise.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "Annex B", "L_Aeq"),
        zone_subject(zone, path("noiseDb")), loc("Indoor noise level", "Innengeräuschpegel"),
    ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, zone.noise_db), Quantity::new(QuantityKind::Dimensionless, nlim))
    .explanation(loc(format!("L_Aeq={:.1} dB; limit {:.1} dB", zone.noise_db, nlim), format!("L_Aeq={:.1} dB; Grenzwert {:.1} dB", zone.noise_db, nlim)));
    if zone.noise_db > nlim {
        bn = bn.remedy(Remedy::at_most(zone_subject(zone, path("noiseDb")), Quantity::new(QuantityKind::Dimensionless, zone.noise_db), Quantity::new(QuantityKind::Dimensionless, nlim),
            loc(format!("Reduce ventilation noise to at most {nlim:.1} dB(A)."), format!("Lüftungsgeräusch auf höchstens {nlim:.1} dB(A) senken."))));
    }
    out.push(bn.build());

    // Draught rate DR — ISO 7730 / EN 16798-1 category limits
    let dr = draught_rate_percent(zone.t_op_summer_c, zone.air_speed_m_s, zone.turbulence_intensity_percent);
    let dr_lim = draught_limit_percent(cat);
    let mut bd = CheckResult::assess(
        format!("din16798-1.draught.{}", zone.id), "DIN EN 16798-1", ClauseId::new("EN 16798-1", "§7.2 / ISO 7730", "DR"),
        zone_subject(zone, path("airSpeedMS")), loc("Draught rate (ISO 7730)", "Zugluftrate (ISO 7730)"),
    ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, dr), Quantity::new(QuantityKind::Dimensionless, dr_lim))
    .explanation(loc(
        format!("DR={dr:.1} % at v={:.2} m/s, Tu={:.0} %, θ={:.1} °C; Cat limit {dr_lim:.0} %", zone.air_speed_m_s, zone.turbulence_intensity_percent, zone.t_op_summer_c),
        format!("DR={dr:.1} % bei v={:.2} m/s, Tu={:.0} %, θ={:.1} °C; Kat.-Grenzwert {dr_lim:.0} %", zone.air_speed_m_s, zone.turbulence_intensity_percent, zone.t_op_summer_c),
    ));
    if dr > dr_lim {
        // Invert roughly: lower air speed until DR ≤ limit (bounded search)
        let mut v_req = zone.air_speed_m_s;
        for step in 0..40 {
            let trial = (zone.air_speed_m_s * (1.0 - 0.025 * (step as f64 + 1.0))).max(0.05);
            if draught_rate_percent(zone.t_op_summer_c, trial, zone.turbulence_intensity_percent) <= dr_lim {
                v_req = trial;
                break;
            }
            v_req = trial;
        }
        bd = bd.remedy(Remedy::at_most(zone_subject(zone, path("airSpeedMS")), Quantity::new(QuantityKind::Dimensionless, zone.air_speed_m_s), Quantity::new(QuantityKind::Dimensionless, v_req),
            loc(
                format!("Reduce local air speed to at most {v_req:.2} m/s so DR ≤ {dr_lim:.0} %."),
                format!("Lokale Luftgeschwindigkeit auf höchstens {v_req:.2} m/s senken, damit DR ≤ {dr_lim:.0} %."),
            )));
    }
    out.push(bd.build());

    // Night setback deferred to envelope
    out
}
