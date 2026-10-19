//! 🌀️ DIN EN 16798 ventilation system and envelope airtightness evaluation.

use super::*;

pub(super) fn evaluate_vent(doc: &semio_s_artifact_norm_din16798_model::Din16798Snapshot, _vi: usize, vent: &VentSystemDocument, _annex: &annex_params::AnnexParams) -> Vec<CheckResult> {
    use part_3::*;
    let mut out = Vec::new();
    let path = |field: &str| format!("ventSystems[id={}].{field}", vent.id);
    let served: Vec<&ZoneDocument> = doc.zones.iter().filter(|z| z.vent_system_id == vent.id).collect();
    let q_served: f64 = served.iter().map(|z| z.outdoor_air_supplied_m3_h).sum();
    let q_design = vent.design_airflow_m3_h;
    let q_design_m3_s = (q_design / 3600.0).max(1e-9);

    {
        let mut bc = CheckResult::assess(
            format!("din16798-3.capacity.{}", vent.id), "DIN EN 16798-3", ClauseId::new("EN 16798-3", "§6", "q_design"),
            vent_subject(vent, path("designAirflowM3H")), loc("Design outdoor-air capacity", "Auslegungs-Außenluftkapazität"),
        ).annex(doc.annex);
        if served.is_empty() {
            bc = bc.not_applicable(loc(
                "No zones link this ventilation system.",
                "Keine Zone verweist auf diese Lüftungsanlage.",
            ));
        } else {
            bc = bc.utilization(Quantity::new(QuantityKind::VentilationRate, q_served), Quantity::new(QuantityKind::VentilationRate, q_design.max(1e-9)))
                .explanation(loc(
                    format!("q_served={q_served:.1} m³/h from {} zone(s); design capacity {q_design:.1} m³/h", served.len()),
                    format!("q_served={q_served:.1} m³/h aus {} Zone(n); Auslegungskapazität {q_design:.1} m³/h", served.len()),
                ));
            if q_served > q_design {
                bc = bc.remedy(Remedy::at_least(vent_subject(vent, path("designAirflowM3H")), Quantity::new(QuantityKind::VentilationRate, q_design), Quantity::new(QuantityKind::VentilationRate, q_served),
                    loc(
                        format!("Increase design outdoor-air capacity to at least {q_served:.1} m³/h."),
                        format!("Auslegungs-Außenluftkapazität auf mindestens {q_served:.1} m³/h erhöhen."),
                    )));
            }
        }
        out.push(bc.build());
    }

    {
        let q_fan_m3_h = vent.fan_q_v_m3_s * 3600.0;
        let mut bf = CheckResult::assess(
            format!("din16798-5-1.fan-flow.{}", vent.id), "DIN EN 16798-5-1", ClauseId::new("EN 16798-5-1", "§6.1", "q_fan"),
            vent_subject(vent, path("fanQVM3S")), loc("Fan operating flow vs design", "Ventilator-Betriebsvolumenstrom vs. Auslegung"),
        ).annex(doc.annex);
        if q_fan_m3_h <= q_design * (1.0 + 1e-6) {
            bf = bf.status(CheckStatus::Pass)
                .utilization(Quantity::new(QuantityKind::VentilationRate, q_fan_m3_h.min(q_design)), Quantity::new(QuantityKind::VentilationRate, q_design.max(1e-9)))
                .explanation(loc(
                    format!("Fan flow {q_fan_m3_h:.1} m³/h vs design {q_design:.1} m³/h"),
                    format!("Ventilatorvolumenstrom {q_fan_m3_h:.1} m³/h gegenüber Auslegung {q_design:.1} m³/h"),
                ));
        } else {
            bf = bf.utilization(Quantity::new(QuantityKind::VentilationRate, q_fan_m3_h), Quantity::new(QuantityKind::VentilationRate, q_design.max(1e-9)))
                .explanation(loc(
                    format!("Fan flow {q_fan_m3_h:.1} m³/h exceeds design {q_design:.1} m³/h"),
                    format!("Ventilatorvolumenstrom {q_fan_m3_h:.1} m³/h überschreitet Auslegung {q_design:.1} m³/h"),
                ))
                .remedy(Remedy::at_most(vent_subject(vent, path("fanQVM3S")), Quantity::new(QuantityKind::VentilationRate, vent.fan_q_v_m3_s), Quantity::new(QuantityKind::VentilationRate, q_design_m3_s),
                    loc(
                        format!("Reduce fan operating flow to at most {q_design:.1} m³/h ({q_design_m3_s:.4} m³/s)."),
                        format!("Ventilator-Betriebsvolumenstrom auf höchstens {q_design:.1} m³/h ({q_design_m3_s:.4} m³/s) senken."),
                    )));
        }
        out.push(bf.build());
    }

    let bound = sfp_bound(vent.sfp_required_class);
    let mut bs = CheckResult::assess(
        format!("din16798-3.sfp.{}", vent.id), "DIN EN 16798-3", ClauseId::new("EN 16798-3", "Table 10", "SFP"),
        vent_subject(vent, path("sfpWM3S")), loc("Specific fan power", "Spezifische Ventilatorleistung"),
    ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Power, vent.sfp_w_m3_s), Quantity::new(QuantityKind::Power, bound))
    .explanation(loc(format!("SFP={:.0} W/(m³/s); class {} bound {:.0}", vent.sfp_w_m3_s, vent.sfp_required_class, bound), format!("SFP={:.0} W/(m³/s); Klasse {} Grenze {:.0}", vent.sfp_w_m3_s, vent.sfp_required_class, bound)));
    if vent.sfp_w_m3_s > bound {
        bs = bs.remedy(Remedy::at_most(vent_subject(vent, path("sfpWM3S")), Quantity::new(QuantityKind::Power, vent.sfp_w_m3_s), Quantity::new(QuantityKind::Power, bound),
            loc(format!("Reduce SFP to at most {bound:.0} W/(m³/s) (SFP class {}).", vent.sfp_required_class), format!("SFP auf höchstens {bound:.0} W/(m³/s) senken (SFP-Klasse {}).", vent.sfp_required_class))));
    }
    out.push(bs.build());

    match heat_recovery_eta_min(&vent.system_type) {
        None => out.push(CheckResult::assess(
            format!("din16798-3.hr.{}", vent.id), "DIN EN 16798-3", ClauseId::new("EN 16798-3", "§7.3", "η"),
            vent_subject(vent, path("heatRecoveryEta")), loc("Heat recovery efficiency", "Wärmerückgewinnungsgrad"),
        ).not_applicable(loc("Natural/hybrid systems have no HR efficiency requirement.", "Natürliche/hybride Systeme ohne WRG-Anforderung.")).annex(doc.annex).build()),
        Some(eta_min) => {
            let mut bh = CheckResult::assess(
                format!("din16798-3.hr.{}", vent.id), "DIN EN 16798-3", ClauseId::new("EN 16798-3", "§7.3", "η"),
                vent_subject(vent, path("heatRecoveryEta")), loc("Heat recovery efficiency", "Wärmerückgewinnungsgrad"),
            ).annex(doc.annex).minimum(Quantity::new(QuantityKind::Dimensionless, vent.heat_recovery_eta), Quantity::new(QuantityKind::Dimensionless, eta_min))
            .explanation(loc(format!("Heat-recovery η={:.2}; minimum {:.2}", vent.heat_recovery_eta, eta_min), format!("Wärmerückgewinnungsgrad η={:.2}; Mindestwert {:.2}", vent.heat_recovery_eta, eta_min)));
            if vent.heat_recovery_eta < eta_min {
                bh = bh.remedy(Remedy::at_least(vent_subject(vent, path("heatRecoveryEta")), Quantity::new(QuantityKind::Dimensionless, vent.heat_recovery_eta), Quantity::new(QuantityKind::Dimensionless, eta_min),
                    loc(format!("Increase heat-recovery efficiency to at least {eta_min:.2}."), format!("Wärmerückgewinnungsgrad auf mindestens {eta_min:.2} erhöhen."))));
            }
            out.push(bh.build());
        }
    }

    let (req_filter, req_rank) = required_filter_for_oda(&vent.oda_class);
    let have = filter_rank(&vent.filter_sup_class);
    let mut bf = CheckResult::assess(
        format!("din16798-3.filter.{}", vent.id), "DIN EN 16798-3", ClauseId::new("EN 16798-3", "§7", "filter"),
        vent_subject(vent, path("filterSupClass")), loc("SUP filter vs ODA class", "Zuluftfilter vs. ODA-Klasse"),
    ).annex(doc.annex);
    if have >= req_rank {
        bf = bf.utilization(Quantity::new(QuantityKind::Dimensionless, have as f64), Quantity::new(QuantityKind::Dimensionless, req_rank.max(1) as f64))
            .status(CheckStatus::Pass)
            .explanation(loc(format!("{} with {} OK (need ≥{})", vent.oda_class, vent.filter_sup_class, req_filter), format!("{} mit {} OK (mind. {})", vent.oda_class, vent.filter_sup_class, req_filter)));
    } else {
        let filter_remedy = Remedy {
            target: vent_subject(vent, path("filterSupClass")),
            current: Quantity::new(QuantityKind::Dimensionless, have as f64),
            required: Quantity::new(QuantityKind::Dimensionless, req_rank as f64),
            bound: RemedyBound::OneOf,
            options: vec![req_filter.to_string()],
            action: loc(
                format!("Set SUP filter class to {req_filter}."),
                format!("Zuluftfilterklasse auf {req_filter} setzen."),
            ),
            applicable: true,
        };
        bf = bf.status(CheckStatus::Fail).utilization(Quantity::new(QuantityKind::Dimensionless, (req_rank - have) as f64 + 1.0), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .explanation(loc(format!("{} requires ≥{}; have {}", vent.oda_class, req_filter, vent.filter_sup_class), format!("{} erfordert ≥{}; vorhanden {}", vent.oda_class, req_filter, vent.filter_sup_class)))
            .remedy(filter_remedy);
    }
    out.push(bf.build());

    let interval = inspection_interval_years(&vent.system_type);
    let mut bi = CheckResult::assess(
        format!("din16798-3.inspection.{}", vent.id), "DIN EN 16798-3", ClauseId::new("EN 16798-3", "§8.1", "inspection"),
        vent_subject(vent, path("yearsSinceInspection")), loc("Ventilation inspection interval", "Lüftungsinspektionsintervall"),
    ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, vent.years_since_inspection as f64), Quantity::new(QuantityKind::Dimensionless, interval as f64))
    .explanation(loc(format!("{} years since inspection; allowed interval {} years", vent.years_since_inspection, interval), format!("{} Jahre seit der Inspektion; zulässiges Intervall {} Jahre", vent.years_since_inspection, interval)));
    if vent.years_since_inspection > interval {
        bi = bi.remedy(Remedy::at_most(vent_subject(vent, path("yearsSinceInspection")), Quantity::new(QuantityKind::Dimensionless, vent.years_since_inspection as f64), Quantity::new(QuantityKind::Dimensionless, 0.0),
            loc("Perform inspection and reset years since inspection to 0.", "Inspektion durchführen und Jahre seit Inspektion auf 0 setzen.")));
    }
    out.push(bi.build());

    let vent_kind = VentSystemType::parse(&vent.system_type);
    if matches!(vent_kind, VentSystemType::Natural) {
        out.push(CheckResult::assess(
            format!("din16798-3.humid.{}", vent.id), "DIN EN 16798-3", ClauseId::new("EN 16798-3", "§7.4", "humidification"),
            vent_subject(vent, path("humidificationProvidedKgH")), loc("Humidification capacity", "Befeuchtungsleistung"),
        ).not_applicable(loc(
            "Natural ventilation systems have no humidification requirement.",
            "Natürliche Lüftung ohne Befeuchtungsanforderung.",
        )).annex(doc.annex).build());
    } else if vent.humidification_required_kg_h <= 0.0 {
        out.push(CheckResult::assess(
            format!("din16798-3.humid.{}", vent.id), "DIN EN 16798-3", ClauseId::new("EN 16798-3", "§7.4", "humidification"),
            vent_subject(vent, path("humidificationProvidedKgH")), loc("Humidification capacity", "Befeuchtungsleistung"),
        ).annex(doc.annex)
         .status(CheckStatus::Pass)
         .utilization(
            Quantity::new(QuantityKind::Mass, vent.humidification_provided_kg_h),
            Quantity::new(QuantityKind::Mass, vent.humidification_provided_kg_h.abs().max(1e-9)),
         )
         .explanation(loc(
            format!("No humidification required; provided {:.2} kg/h recorded", vent.humidification_provided_kg_h),
            format!("Keine Befeuchtung erforderlich; bereitgestellt {:.2} kg/h erfasst", vent.humidification_provided_kg_h),
         ))
         .build());
    } else {
        let mut bh = CheckResult::assess(
            format!("din16798-3.humid.{}", vent.id), "DIN EN 16798-3", ClauseId::new("EN 16798-3", "§7.4", "humidification"),
            vent_subject(vent, path("humidificationProvidedKgH")), loc("Humidification capacity", "Befeuchtungsleistung"),
        ).annex(doc.annex).minimum(Quantity::new(QuantityKind::Mass, vent.humidification_provided_kg_h), Quantity::new(QuantityKind::Mass, vent.humidification_required_kg_h));
        if vent.humidification_provided_kg_h < vent.humidification_required_kg_h {
            bh = bh.remedy(Remedy::at_least(vent_subject(vent, path("humidificationProvidedKgH")), Quantity::new(QuantityKind::Mass, vent.humidification_provided_kg_h), Quantity::new(QuantityKind::Mass, vent.humidification_required_kg_h),
                loc(format!("Increase humidification capacity to at least {:.2} kg/h.", vent.humidification_required_kg_h), format!("Befeuchtungsleistung auf mindestens {:.2} kg/h erhöhen.", vent.humidification_required_kg_h))));
        }
        out.push(bh.build());
    }

    let lim = duct_leakage_limit(&vent.duct_class, vent.duct_test_pressure_pa);
    let mut bd = CheckResult::assess(
        format!("din16798-17.duct.{}", vent.id), "DIN EN 16798-17", ClauseId::new("EN 16798-17", "§8.2", "leakage"),
        vent_subject(vent, path("ductLeakageM3SM2")), loc("Ductwork leakage", "Kanalnetzleckage"),
    ).annex(doc.annex).utilization(Quantity::new(QuantityKind::VentilationRate, vent.duct_leakage_m3_s_m2), Quantity::new(QuantityKind::VentilationRate, lim))
    .explanation(loc(format!("f={:.4}; limit {:.4} at {:.0} Pa class {}", vent.duct_leakage_m3_s_m2, lim, vent.duct_test_pressure_pa, vent.duct_class), format!("f={:.4}; Grenzwert {:.4} bei {:.0} Pa Klasse {}", vent.duct_leakage_m3_s_m2, lim, vent.duct_test_pressure_pa, vent.duct_class)));
    if vent.duct_leakage_m3_s_m2 > lim {
        bd = bd.remedy(Remedy::at_most(vent_subject(vent, path("ductLeakageM3SM2")), Quantity::new(QuantityKind::VentilationRate, vent.duct_leakage_m3_s_m2), Quantity::new(QuantityKind::VentilationRate, lim),
            loc(format!("Reduce duct leakage to at most {lim:.4} m³/(s·m²)."), format!("Kanalnetzleckage auf höchstens {lim:.4} m³/(s·m²) senken."))));
    }
    out.push(bd.build());

    const FAN_ENERGY_BENCHMARK_SFP_W_M3_S: f64 = 1000.0;
    let e = part_5_1::fan_energy_kwh(vent.sfp_w_m3_s, q_design_m3_s, vent.fan_t_run_h);
    let e_ref = part_5_1::fan_energy_kwh(FAN_ENERGY_BENCHMARK_SFP_W_M3_S, q_design_m3_s, vent.fan_t_run_h).max(1e-9);
    let mut be = CheckResult::assess(
        format!("din16798-5-1.fan.{}", vent.id), "DIN EN 16798-5-1", ClauseId::new("EN 16798-5-1", "§6.1", "E_fan"),
        vent_subject(vent, path("sfpWM3S")), loc("Annual fan electrical energy", "Jährliche Ventilator-Elektroenergie"),
    ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Energy, e), Quantity::new(QuantityKind::Energy, e_ref))
    .explanation(loc(
        format!("E_fan={e:.1} kWh/a from SFP·q·t; benchmark {e_ref:.1} kWh/a at SFP_ref={FAN_ENERGY_BENCHMARK_SFP_W_M3_S:.0} W/(m³/s)"),
        format!("E_fan={e:.1} kWh/a aus SFP·q·t; Richtwert {e_ref:.1} kWh/a bei SFP_ref={FAN_ENERGY_BENCHMARK_SFP_W_M3_S:.0} W/(m³/s)"),
    ));
    if e > e_ref {
        be = be.remedy(Remedy::at_most(vent_subject(vent, path("sfpWM3S")), Quantity::new(QuantityKind::Power, vent.sfp_w_m3_s), Quantity::new(QuantityKind::Power, FAN_ENERGY_BENCHMARK_SFP_W_M3_S),
            loc(format!("Reduce SFP to ≤{FAN_ENERGY_BENCHMARK_SFP_W_M3_S:.0} W/(m³/s) so annual fan energy ≤{e_ref:.1} kWh."), format!("SFP auf ≤{FAN_ENERGY_BENCHMARK_SFP_W_M3_S:.0} W/(m³/s) senken, damit die jährliche Ventilatorenergie ≤{e_ref:.1} kWh bleibt."))));
    }
    out.push(be.build());

    out
}

pub(super) fn evaluate_envelope(doc: &semio_s_artifact_norm_din16798_model::Din16798Snapshot, _annex: &annex_params::AnnexParams) -> Vec<CheckResult> {
    use part_7::*;
    let mut out = Vec::new();
    let n50_max = n50_max_mechanical();
    let mut bn = CheckResult::assess(
        "din16798-7.n50", "DIN EN 16798-7", ClauseId::new("EN 16798-7", "§6.1", "n50"),
        SubjectRef::new("", "envelopeN50HInv", loc("Building envelope", "Gebäudehülle")),
        loc("Airtightness n50", "Luftdichtheit n50"),
    ).annex(doc.annex).utilization(Quantity::new(QuantityKind::Dimensionless, doc.envelope_n50_h_inv), Quantity::new(QuantityKind::Dimensionless, n50_max))
    .explanation(loc(format!("n50={:.2} 1/h; max {:.1} 1/h for mechanical ventilation", doc.envelope_n50_h_inv, n50_max), format!("n50={:.2} 1/h; max. {:.1} 1/h bei mechanischer Lüftung", doc.envelope_n50_h_inv, n50_max)));
    if doc.envelope_n50_h_inv > n50_max {
        bn = bn.remedy(Remedy::at_most(SubjectRef::new("", "envelopeN50HInv", loc("Building envelope", "Gebäudehülle")), Quantity::new(QuantityKind::Dimensionless, doc.envelope_n50_h_inv), Quantity::new(QuantityKind::Dimensionless, n50_max),
            loc(format!("Improve airtightness to n50 ≤ {n50_max:.1} 1/h."), format!("Luftdichtheit auf n50 ≤ {n50_max:.1} 1/h verbessern."))));
    }
    out.push(bn.build());

    if doc.envelope_volume_m3 > 0.0 {
        let q_inf = infiltration_m3_h(doc.envelope_n50_h_inv, doc.envelope_volume_m3);
        let q_cap = infiltration_m3_h(n50_max, doc.envelope_volume_m3);
        let mut bi = CheckResult::assess(
            "din16798-7.infiltration", "DIN EN 16798-7", ClauseId::new("EN 16798-7", "§6.1", "q_inf"),
            SubjectRef::new("", "envelopeVolumeM3", loc("Building envelope volume", "Gebäudehüllvolumen")),
            loc("Infiltration volume flow", "Infiltrationsvolumenstrom"),
        ).annex(doc.annex).utilization(Quantity::new(QuantityKind::VentilationRate, q_inf), Quantity::new(QuantityKind::VentilationRate, q_cap.max(1e-9)))
        .explanation(loc(
            format!("q_inf≈{q_inf:.1} m³/h from n50·V/20; cap {q_cap:.1} m³/h at n50≤{n50_max:.1}"),
            format!("q_inf≈{q_inf:.1} m³/h aus n50·V/20; Grenze {q_cap:.1} m³/h bei n50≤{n50_max:.1}"),
        ));
        if q_inf > q_cap {
            bi = bi.remedy(Remedy::at_most(SubjectRef::new("", "envelopeN50HInv", loc("Building envelope", "Gebäudehülle")), Quantity::new(QuantityKind::Dimensionless, doc.envelope_n50_h_inv), Quantity::new(QuantityKind::Dimensionless, n50_max),
                loc(format!("Improve airtightness to n50 ≤ {n50_max:.1} 1/h."), format!("Luftdichtheit auf n50 ≤ {n50_max:.1} 1/h verbessern."))));
        }
        out.push(bi.build());
    }

    if doc.cellar_area_m2 <= 0.0 {
        out.push(CheckResult::assess(
            "din16798-7.cellar", "DIN EN 16798-7", ClauseId::new("EN 16798-7", "§6.2", "cellar"),
            SubjectRef::new("", "cellarAreaM2", loc("Cellar floor area", "Kellerfläche")), loc("Cellar ventilation", "Kellerlüftung"),
        ).annex(doc.annex)
         .not_applicable(loc(
            "No cellar floor area; EN 16798-7 §6.2 cellar ventilation does not apply.",
            "Keine Kellerfläche; Kellerlüftung nach EN 16798-7 §6.2 gilt nicht.",
         ))
         .build());
    } else {
        let req = cellar_ventilation_required_m3_h(doc.cellar_area_m2);
        let mut bc = CheckResult::assess(
            "din16798-7.cellar", "DIN EN 16798-7", ClauseId::new("EN 16798-7", "§6.2", "cellar"),
            SubjectRef::new("", "cellarVentilationM3H", loc("Cellar", "Keller")),
            loc("Cellar ventilation", "Kellerlüftung"),
        ).annex(doc.annex).minimum(Quantity::new(QuantityKind::VentilationRate, doc.cellar_ventilation_m3_h), Quantity::new(QuantityKind::VentilationRate, req))
         .explanation(loc(
            format!("Cellar ventilation {v:.1} m³/h vs required {req:.1} m³/h for {a:.1} m².", v=doc.cellar_ventilation_m3_h, req=req, a=doc.cellar_area_m2),
            format!("Kellerlüftung {v:.1} m³/h gegenüber erforderlich {req:.1} m³/h für {a:.1} m².", v=doc.cellar_ventilation_m3_h, req=req, a=doc.cellar_area_m2),
         ));
        if doc.cellar_ventilation_m3_h < req {
            bc = bc.remedy(Remedy::at_least(SubjectRef::new("", "cellarVentilationM3H", loc("Cellar", "Keller")), Quantity::new(QuantityKind::VentilationRate, doc.cellar_ventilation_m3_h), Quantity::new(QuantityKind::VentilationRate, req),
                loc(format!("Raise cellar ventilation to at least {req:.1} m³/h."), format!("Kellerlüftung auf mindestens {req:.1} m³/h erhöhen."))));
        }
        out.push(bc.build());
    }

    let usage = doc.zones.first().map(|z| parse_usage(&z.usage_type)).unwrap_or(OccupancyType::Office);
    let sb_min = part_5_1::night_setback_min_k(usage);
    let mut bs = CheckResult::assess(
        "din16798-5-1.setback", "DIN EN 16798-5-1", ClauseId::new("EN 16798-5-1", "§6.2", "setback"),
        SubjectRef::new("", "nightSetbackK", loc("Building", "Gebäude")),
        loc("Night setback", "Nachtabsenkung"),
    ).annex(doc.annex).minimum(Quantity::new(QuantityKind::Temperature, doc.night_setback_k), Quantity::new(QuantityKind::Temperature, sb_min))
    .explanation(loc(format!("Δθ_night={:.1} K; min {:.1} K", doc.night_setback_k, sb_min), format!("Δθ_Nacht={:.1} K; min. {:.1} K", doc.night_setback_k, sb_min)));
    if doc.night_setback_k < sb_min {
        bs = bs.remedy(Remedy::at_least(SubjectRef::new("", "nightSetbackK", loc("Building", "Gebäude")), Quantity::new(QuantityKind::Temperature, doc.night_setback_k), Quantity::new(QuantityKind::Temperature, sb_min),
            loc(format!("Increase night setback to at least {sb_min:.1} K."), format!("Nachtabsenkung auf mindestens {sb_min:.1} K erhöhen."))));
    }
    out.push(bs.build());
    out
}
