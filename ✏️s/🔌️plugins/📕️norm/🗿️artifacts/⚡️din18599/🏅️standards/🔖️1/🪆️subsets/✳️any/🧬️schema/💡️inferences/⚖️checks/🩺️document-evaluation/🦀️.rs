//! 🩺️ DIN V 18599 document evaluation: duplicate ids and the clause-by-clause evaluation of a snapshot.

use super::*;

//#region 🔖️Checks

/// 🆔 Fail when zone or envelope element ids are not unique (CORRECTION 14:42).
fn push_duplicate_ids(report: &mut CheckReport, table: &str, ids: &[String], path_for: &dyn Fn(&str) -> String, annex: AnnexChoice) {
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for id in ids {
        *counts.entry(id.clone()).or_insert(0) += 1;
    }
    for (id, count) in counts {
        if count < 2 {
            continue;
        }
        let path = path_for(&id);
        let subject = SubjectRef::new(id.clone(), &path, copy(&format!("Duplicate {table} id"), &format!("Doppelte {table}-Id")));
        let options: Vec<String> = ids.iter().filter(|x| x.as_str() != id).cloned().collect();
        let free = if options.is_empty() { vec![format!("{id}-unique")] } else { options };
        let builder = CheckResult::assess(
            format!("din18599.integrity.duplicate.{table}.{id}"),
            "DIN V 18599 integrity",
            ClauseId::new("DIN V 18599", "1", "id"),
            subject.clone(),
            copy(&format!("Unique {table} id"), &format!("Eindeutige {table}-Id")),
        )
        .annex(annex)
        .explanation(copy(
            &format!("Duplicate {table} id '{id}' appears {count} times; each entity id must be unique."),
            &format!("Doppelte {table}-Id '{id}' kommt {count}-mal vor; jede Entitäts-Id muss eindeutig sein."),
        ))
        .status(CheckStatus::Fail)
        .remedy(Remedy::one_of(
            subject,
            free,
            copy(
                &format!("Rename the duplicated '{id}' entry to a free id."),
                &format!("Den doppelten '{id}'-Eintrag auf eine freie Id umbenennen."),
            ),
        ));
        report.push(builder.build());
    }
}

/// ⚖️ Evaluate DIN V 18599 / GEG compliance for the building subject.
pub fn evaluate_document(doc: &Din18599Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    let annex = AnnexChoice::De;
    push_duplicate_ids(
        &mut report,
        "zones",
        &doc.zones.iter().map(|z| z.id.clone()).collect::<Vec<_>>(),
        &|id| format!("zones[id={id}].id"),
        annex,
    );
    push_duplicate_ids(
        &mut report,
        "elements",
        &doc.elements.iter().map(|e| e.id.clone()).collect::<Vec<_>>(),
        &|id| format!("elements[id={id}].id"),
        annex,
    );
    let derived = derive_balance(doc);

    // --- DIN V 18599-1 heated gross volume V_e plausibility vs Σ zone net volumes ---
    {
        let v_e = doc.heated_volume_m3;
        let v_net: f64 = doc.zones.iter().map(|z| z.volume_m3).sum();
        let v_net_min = DIN_V_18599_1_NET_TO_GROSS_VOLUME_RATIO * v_e;
        let mut b = CheckResult::assess(
            "din18599.1.heated-volume",
            "DIN V 18599-1",
            ClauseId::new("DIN V 18599", "1", "6.1.1"),
            SubjectRef::new("", "heatedVolumeM3", copy("Heated gross volume V_e", "Beheiztes Bruttovolumen V_e")),
            copy("Heated volume V_e vs zone net volumes", "Bruttovolumen V_e gegenüber Nettovolumina"),
        )
        .annex(annex)
        .explanation(copy(
            &format!("V_e = {v_e:.1} m³; ΣV_zone = {v_net:.1} m³; A/V_e = {:.3} m⁻¹; DIN V 18599-1 expects V ≈ 0.80·V_e.", derived.a_over_v_e),
            &format!("V_e = {v_e:.1} m³; ΣV_zone = {v_net:.1} m³; A/V_e = {:.3} m⁻¹; DIN V 18599-1 erwartet V ≈ 0,80·V_e.", derived.a_over_v_e),
        ));
        if v_net > v_e + 1e-6 {
            b = b
                .utilization(Quantity::new(QuantityKind::Volume, v_net), Quantity::new(QuantityKind::Volume, v_e))
                .remedy(Remedy::at_least(
                    SubjectRef::new("", "heatedVolumeM3", copy("Heated gross volume V_e", "Beheiztes Bruttovolumen V_e")),
                    Quantity::new(QuantityKind::Volume, v_e),
                    Quantity::new(QuantityKind::Volume, v_net),
                    copy(
                        &format!("Raise V_e to at least {v_net:.1} m³ so gross volume covers Σ zone net volumes."),
                        &format!("V_e auf mindestens {v_net:.1} m³ anheben, damit das Bruttovolumen Σ Nettovolumina abdeckt."),
                    ),
                ));
        } else if v_net + 1e-6 < v_net_min {
            b = b
                .utilization(Quantity::new(QuantityKind::Volume, v_net), Quantity::new(QuantityKind::Volume, v_net_min))
                .remedy(Remedy::at_most(
                    SubjectRef::new("", "heatedVolumeM3", copy("Heated gross volume V_e", "Beheiztes Bruttovolumen V_e")),
                    Quantity::new(QuantityKind::Volume, v_e),
                    Quantity::new(QuantityKind::Volume, v_net / DIN_V_18599_1_NET_TO_GROSS_VOLUME_RATIO),
                    copy(
                        &format!("Lower V_e toward {:.1} m³ so ΣV_zone / V_e ≥ 0.80 (DIN V 18599-1).", v_net / DIN_V_18599_1_NET_TO_GROSS_VOLUME_RATIO),
                        &format!("V_e Richtung {:.1} m³ senken, damit ΣV_zone / V_e ≥ 0,80 (DIN V 18599-1).", v_net / DIN_V_18599_1_NET_TO_GROSS_VOLUME_RATIO),
                    ),
                ));
        } else {
            b = b.utilization(Quantity::new(QuantityKind::Volume, v_net), Quantity::new(QuantityKind::Volume, v_e));
        }
        report.push(b.build());
    }

    // --- Derived climate table handle (DIN V 18599-10 climate composed as s.stdio.semio@v1/table) ---
    {
        let child_id = doc.climate_table.child_id.as_str();
        let target = &doc.climate_table.target;
        let derived = semio_s_artifact_norm_din18599_model::din18599_climate_table_child(&doc.climate);
        let ok = doc.climate_table == derived;
        let mut b = CheckResult::assess(
            "din18599.1.climate-composition",
            "DIN V 18599-10",
            ClauseId::new("DIN V 18599", "10", "climate"),
            SubjectRef::new("", "climateTable.childId", copy("Climate table", "Klimatabelle")),
            copy("Climate table derived from the monthly climate", "Klimatabelle aus dem Monatsklima abgeleitet"),
        )
        .annex(annex)
        .explanation(copy(
            &format!("Climate table '{child_id}' → {}@{}/{}.", target.dialect.artifact_kind, target.dialect.standard, target.dialect.subset),
            &format!("Klimatabelle '{child_id}' → {}@{}/{}.", target.dialect.artifact_kind, target.dialect.standard, target.dialect.subset),
        ));
        if ok {
            b = b.utilization(dimensionless(1.0), dimensionless(1.0));
        } else {
            b = b
                .utilization(dimensionless(0.0), dimensionless(1.0))
                .remedy(Remedy::one_of(
                    SubjectRef::new("", "climateTable.childId", copy("Climate table id", "Klimatabellen-ID")),
                    vec![derived.child_id.clone()],
                    copy(
                        "Restore the s.stdio.semio@v1/table handle derived from the monthly climate.",
                        "Den aus dem Monatsklima abgeleiteten s.stdio.semio@v1/table-Handle wiederherstellen.",
                    ),
                ));
        }
        report.push(b.build());
    }

    // --- Net floor area A_N vs Σ zone areas ---
    {
        let a_n = doc.net_floor_area_m2;
        let a_zones: f64 = doc.zones.iter().map(|z| z.area_m2).sum();
        let mut b = CheckResult::assess(
            "din18599.1.net-floor-area",
            "DIN V 18599-1",
            ClauseId::new("DIN V 18599", "1", "A_N"),
            SubjectRef::new("", "netFloorAreaM2", copy("Net floor area A_N", "Nettogrundfläche A_N")),
            copy("Net floor area vs zone areas", "Nettogrundfläche gegenüber Zonenflächen"),
        )
        .utilization(Quantity::new(QuantityKind::Area, a_zones), Quantity::new(QuantityKind::Area, a_n.max(1e-9)))
        .annex(annex)
        .explanation(copy(
            &format!("Net floor area A_N = {a_n:.1} m²; sum of zone areas ΣA_zone = {a_zones:.1} m²."),
            &format!("Nettogrundfläche A_N = {a_n:.1} m²; Summe der Zonenflächen ΣA_zone = {a_zones:.1} m²."),
        ));
        if a_zones > a_n {
            b = b.remedy(Remedy::at_least(
                SubjectRef::new("", "netFloorAreaM2", copy("Net floor area A_N", "Nettogrundfläche A_N")),
                Quantity::new(QuantityKind::Area, a_n),
                Quantity::new(QuantityKind::Area, a_zones),
                copy(
                    &format!("Set A_N to {a_zones:.1} m² to match Σ zone areas."),
                    &format!("A_N auf {a_zones:.1} m² setzen, damit Σ Zonenflächen übereinstimmt."),
                ),
            ));
        }
        report.push(b.build());
    }

    // --- Element zoneId linkage ---
    {
        let zone_ids: Vec<String> = doc.zones.iter().map(|z| z.id.clone()).collect();
        for el in &doc.elements {
            let path = format!("elements[id={}].zoneId", el.id);
            let ok = zone_ids.iter().any(|z| z == &el.zone_id);
            let mut b = CheckResult::assess(
                format!("din18599.1.element-zone.{}", el.id),
                "DIN V 18599-1",
                ClauseId::new("DIN V 18599", "1", "zone"),
                SubjectRef::new(&el.id, &path, copy("Element zone", "Bauteilzone")),
                copy("Envelope element zone assignment", "Zonenzuordnung Hüllbauteil"),
            )
            .annex(annex)
            .explanation(copy(
                &format!("Element {} assigned to zone '{}'.", el.label_en, el.zone_id),
                &format!("Bauteil {} der Zone '{}' zugeordnet.", el.label_de, el.zone_id),
            ));
            if ok {
                b = b.utilization(dimensionless(1.0), dimensionless(1.0));
            } else {
                b = b
                    .utilization(dimensionless(0.0), dimensionless(1.0))
                    .remedy(Remedy::one_of(
                        SubjectRef::new(&el.id, &path, copy("Element zone", "Bauteilzone")),
                        zone_ids.clone(),
                        copy(
                            &format!("Assign {} to an existing zone id.", el.label_en),
                            &format!("{} einer vorhandenen Zonen-ID zuordnen.", el.label_de),
                        ),
                    ));
            }
            report.push(b.build());
        }
    }

    // --- GEG H′T (residential) / mean U (non-residential) ---
    match doc.building_category {
        BuildingCategory::Residential => {
            let limit = geg_ht_prime_limit(doc.attachment, doc.net_floor_area_m2);
            let computed = derived.h_t_prime;
            let mut b = CheckResult::assess(
                "din18599.geg.ht-prime",
                "GEG §16 / Anlage 2",
                ClauseId::new("GEG", "Anlage 2", "H'T"),
                SubjectRef::new("", "deltaUWbWM2k", copy("Thermal-bridge surcharge ΔU_WB", "Wärmebrückenzuschlag ΔU_WB")),
                copy("Specific transmission heat loss H′T", "Spezifischer Transmissionswärmeverlust H′T"),
            )
            .utilization(heat_transfer(computed), heat_transfer(limit))
            .annex(annex)
            .explanation(copy(
                &format!("H′T = H_T/A = {computed:.3} W/(m²·K); limit {limit:.2} W/(m²·K) for {:?} with A_N={:.0} m².", doc.attachment, doc.net_floor_area_m2),
                &format!("H′T = H_T/A = {computed:.3} W/(m²·K); Grenzwert {limit:.2} W/(m²·K) für {:?} mit A_N={:.0} m².", doc.attachment, doc.net_floor_area_m2),
            ));
            if computed > limit {
                for el in &doc.elements {
                    let target = reference_u(el.kind);
                    if el.u_value_w_m2k > target + 1e-9 {
                        let path = element_u_path(el);
                        b = b.remedy(Remedy::at_most(
                            element_ref(el, &path),
                            heat_transfer(el.u_value_w_m2k),
                            heat_transfer(target),
                            copy(
                                &format!("Reduce U of {} from {:.3} to GEG reference {:.3} W/(m²·K).", el.label_en, el.u_value_w_m2k, target),
                                &format!("U-Wert von {} von {:.3} auf GEG-Referenz {:.3} W/(m²·K) senken.", el.label_de, el.u_value_w_m2k, target),
                            ),
                        ));
                    }
                }
                if doc.delta_u_wb_w_m2k > GEG_ANLAGE1_DELTA_U_WB_REF + 1e-9 {
                    b = b.remedy(Remedy::at_most(
                        SubjectRef::new("", "deltaUWbWM2k", copy("Thermal-bridge surcharge ΔU_WB", "Wärmebrückenzuschlag ΔU_WB")),
                        heat_transfer(doc.delta_u_wb_w_m2k),
                        heat_transfer(GEG_ANLAGE1_DELTA_U_WB_REF),
                        copy(
                            "Lower ΔU_WB to the GEG reference surcharge 0.05 W/(m²·K).",
                            "ΔU_WB auf den GEG-Referenzzuschlag 0.05 W/(m²·K) senken.",
                        ),
                    ));
                }
            }
            report.push(b.build());
        }
        BuildingCategory::NonResidential => {
            for kind in [ElementKind::Wall, ElementKind::Roof, ElementKind::Floor, ElementKind::Window] {
                match mean_u_for_kind(doc, kind) {
                    None => {
                        report.push(
                            CheckResult::assess(
                                format!("din18599.geg.mean-u.{kind:?}").to_lowercase(),
                                "GEG Anlage 3",
                                ClauseId::new("GEG", "Anlage 3", format!("{kind:?}")),
                                whole_building(),
                                copy(&format!("Mean U ({kind:?})"), &format!("Mittlerer U-Wert ({kind:?})")),
                            )
                            .not_applicable(copy("No elements of this kind.", "Keine Bauteile dieser Art vorhanden."))
                            .annex(annex)
                            .build(),
                        );
                    }
                    Some((mean, limit)) => {
                        let mut b = CheckResult::assess(
                            format!("din18599.geg.mean-u.{kind:?}").to_lowercase(),
                            "GEG Anlage 3",
                            ClauseId::new("GEG", "Anlage 3", format!("{kind:?}")),
                            whole_building(),
                            copy(&format!("Mean U-value ({kind:?})"), &format!("Mittlerer U-Wert ({kind:?})")),
                        )
                        .utilization(heat_transfer(mean), heat_transfer(limit))
                        .annex(annex)
                        .explanation(copy(
                            &format!("Ū = {mean:.3} W/(m²·K) vs limit {limit:.2}."),
                            &format!("Ū = {mean:.3} W/(m²·K) gegenüber Grenzwert {limit:.2}."),
                        ));
                        if mean > limit {
                            if let Some(el) = doc.elements.iter().filter(|e| e.kind == kind).max_by(|a, b| a.u_value_w_m2k.partial_cmp(&b.u_value_w_m2k).unwrap_or(std::cmp::Ordering::Equal)) {
                                let path = element_u_path(el);
                                b = b.remedy(Remedy::at_most(
                                    element_ref(el, &path),
                                    heat_transfer(el.u_value_w_m2k),
                                    heat_transfer(limit),
                                    copy(
                                        &format!("Reduce U of {} to at most {limit:.2} W/(m²·K).", el.label_en),
                                        &format!("U-Wert von {} auf höchstens {limit:.2} W/(m²·K) senken.", el.label_de),
                                    ),
                                ));
                            }
                        }
                        report.push(b.build());
                    }
                }
            }
        }
    }

    // --- Part 2 monthly heating demand vs GEG Anlage 1 reference Q_H,nd ---
    {
        let q_h = derived.q_h_nd_kwh;
        let q_h_ref = derived.q_h_ref_kwh.max(1.0);
        let mut b = CheckResult::assess(
            "din18599.2.heating-demand",
            "DIN V 18599-2",
            ClauseId::new("DIN V 18599", "2", "5"),
            whole_building(),
            copy("Net heating demand Q_H,nd", "Heizwärmebedarf Q_H,nd"),
        )
        .utilization(energy(q_h), energy(q_h_ref))
        .annex(annex)
        .explanation(copy(
            &format!("Q_H,nd = {q_h:.0} kWh/a vs reference building Q_H,nd,Ref = {q_h_ref:.0} kWh/a (H_T={:.1} W/K, H_V={:.1} W/K).", derived.h_t, derived.h_v),
            &format!("Q_H,nd = {q_h:.0} kWh/a gegenüber Referenzgebäude Q_H,nd,Ref = {q_h_ref:.0} kWh/a (H_T={:.1} W/K, H_V={:.1} W/K).", derived.h_t, derived.h_v),
        ));
        if q_h > q_h_ref {
            for el in &doc.elements {
                let target = reference_u(el.kind);
                if el.u_value_w_m2k > target + 1e-9 {
                    let path = element_u_path(el);
                    b = b.remedy(Remedy::at_most(
                        element_ref(el, &path),
                        heat_transfer(el.u_value_w_m2k),
                        heat_transfer(target),
                        copy(
                            &format!("Improve envelope: set U of {} ≤ {:.3} W/(m²·K) (GEG reference).", el.label_en, target),
                            &format!("Hülle verbessern: U von {} ≤ {:.3} W/(m²·K) (GEG-Referenz).", el.label_de, target),
                        ),
                    ));
                }
                if el.kind == ElementKind::Window {
                    if el.g_value + 1e-9 < GEG_ANLAGE1_WINDOW_G_REF {
                        let path = element_g_path(el);
                        b = b.remedy(Remedy::at_least(
                            element_ref(el, &path),
                            dimensionless(el.g_value),
                            dimensionless(GEG_ANLAGE1_WINDOW_G_REF),
                            copy(
                                &format!("Raise g-value of {} to at least {:.2} (GEG reference) to recover solar gains toward Q_H,nd,Ref.", el.label_en, GEG_ANLAGE1_WINDOW_G_REF),
                                &format!("g-Wert von {} auf mindestens {:.2} anheben (GEG-Referenz), um solare Gewinne Richtung Q_H,nd,Ref zu stärken.", el.label_de, GEG_ANLAGE1_WINDOW_G_REF),
                            ),
                        ));
                    }
                    if el.fc + 1e-9 < 1.0 {
                        let path = element_fc_path(el);
                        b = b.remedy(Remedy::at_least(
                            element_ref(el, &path),
                            dimensionless(el.fc),
                            dimensionless(1.0),
                            copy(
                                &format!("Raise shading factor F_c of {} to 1.0 so solar gains match the GEG reference building.", el.label_en),
                                &format!("Abminderungsfaktor F_c von {} auf 1.0 anheben, damit solare Gewinne dem GEG-Referenzgebäude entsprechen.", el.label_de),
                            ),
                        ));
                    }
                }
            }
            if doc.delta_u_wb_w_m2k > GEG_ANLAGE1_DELTA_U_WB_REF + 1e-9 {
                b = b.remedy(Remedy::at_most(
                    SubjectRef::new("", "deltaUWbWM2k", copy("Thermal-bridge surcharge ΔU_WB", "Wärmebrückenzuschlag ΔU_WB")),
                    heat_transfer(doc.delta_u_wb_w_m2k),
                    heat_transfer(GEG_ANLAGE1_DELTA_U_WB_REF),
                    copy("Lower ΔU_WB to 0.05 W/(m²·K) to match the reference heat demand.", "ΔU_WB auf 0.05 W/(m²·K) senken, um den Referenz-Heizwärmebedarf zu erreichen."),
                ));
            }
        }
        report.push(b.build());
    }

    // --- Part 6/7 ventilation heat recovery ---
    {
        let eta = doc.ventilation.heat_recovery_eta;
        let min_eta = GEG_ANLAGE1_TABELLE1_ETA_WRG_REF;
        let mut b = CheckResult::assess(
            "din18599.6.heat-recovery",
            "DIN V 18599-6",
            ClauseId::new("DIN V 18599", "6", "6"),
            SubjectRef::new("", "ventilation.heatRecoveryEta", copy("Heat recovery η_WRG", "Wärmerückgewinnung η_WRG")),
            copy("Ventilation heat recovery", "Wärmerückgewinnung Lüftung"),
        )
        .minimum(dimensionless(eta), dimensionless(min_eta))
        .annex(annex)
        .explanation(copy(
            &format!("η_WRG = {eta:.2}; H_V = {:.1} W/K at {:.0} m³/h.", derived.h_v, doc.ventilation.airflow_m3_h),
            &format!("η_WRG = {eta:.2}; H_V = {:.1} W/K bei {:.0} m³/h.", derived.h_v, doc.ventilation.airflow_m3_h),
        ));
        if eta < min_eta {
            b = b.remedy(Remedy::at_least(
                SubjectRef::new("", "ventilation.heatRecoveryEta", copy("Heat recovery η_WRG", "Wärmerückgewinnung η_WRG")),
                dimensionless(eta),
                dimensionless(min_eta),
                copy(
                    &format!("Increase heat recovery from {:.0}% to at least {:.0}%.", eta * 100.0, min_eta * 100.0),
                    &format!("Wärmerückgewinnung von {:.0}% auf mindestens {:.0}% erhöhen.", eta * 100.0, min_eta * 100.0),
                ),
            ));
        }
        report.push(b.build());
    }

    // --- Part 4 lighting (DIN V 18599-4) — zone installed power vs profile limit ---
    {
        for zone in &doc.zones {
            let row = usage_profile_row(zone.usage_profile);
            let lpd = zone.lighting_power_w_m2;
            let limit = row.lighting_lpd_limit_w_m2;
            let path = format!("zones[id={}].lightingPowerWM2", zone.id);
            let mut b = CheckResult::assess(
                format!("din18599.4.lighting-power.{}", zone.id),
                "DIN V 18599-4",
                ClauseId::new("DIN V 18599", "4", "5"),
                SubjectRef::new(&zone.id, &path, copy("Zone lighting power", "Zonenbeleuchtungsleistung")),
                copy("Lighting installed power density", "Spezifische Beleuchtungsleistung"),
            )
            .utilization(Quantity::new(QuantityKind::Power, lpd), Quantity::new(QuantityKind::Power, limit))
            .annex(annex)
            .explanation(copy(
                &format!("Zone {} LPD = {lpd:.1} W/m² vs profile {:?} limit {limit:.1} W/m²; t_L = {:.0} h/a.", zone.label_en, zone.usage_profile, row.lighting_hours_a),
                &format!("Zone {} LPD = {lpd:.1} W/m² gegenüber Profil {:?} Grenzwert {limit:.1} W/m²; t_L = {:.0} h/a.", zone.label_de, zone.usage_profile, row.lighting_hours_a),
            ));
            if lpd > limit + 1e-9 {
                b = b.remedy(Remedy::at_most(
                    SubjectRef::new(&zone.id, &path, copy("Zone lighting power", "Zonenbeleuchtungsleistung")),
                    Quantity::new(QuantityKind::Power, lpd),
                    Quantity::new(QuantityKind::Power, limit),
                    copy(
                        &format!("Reduce lighting power in {} to at most {limit:.1} W/m².", zone.label_en),
                        &format!("Beleuchtungsleistung in {} auf höchstens {limit:.1} W/m² senken.", zone.label_de),
                    ),
                ));
            }
            report.push(b.build());
        }
    }

// --- Part 5 heating system efficiency ---
    {
        let eff = heating_system_efficiency(doc);
        let min_eff = DIN_V_18599_5_ETA_SYS_MIN;
        let mut b = CheckResult::assess(
            "din18599.5.heating-efficiency",
            "DIN V 18599-5",
            ClauseId::new("DIN V 18599", "5", "6"),
            SubjectRef::new("", "heating.generationEfficiency", copy("Generation efficiency", "Erzeugeraufwand")),
            copy("Heating overall system efficiency", "Gesamtaufwandzahl Heizung"),
        )
        .minimum(dimensionless(eff), dimensionless(min_eff))
        .annex(annex)
        .explanation(copy(
            &format!("η_sys = {eff:.3}; Q_f,heat = {:.0} kWh/a.", derived.q_f_heat_kwh),
            &format!("η_sys = {eff:.3}; Q_f,Wärme = {:.0} kWh/a.", derived.q_f_heat_kwh),
        ));
        if eff < min_eff {
            let gen_req = (min_eff / (doc.heating.distribution_efficiency * doc.heating.storage_efficiency * doc.heating.transfer_efficiency).max(DIN_V_18599_5_ETA_SYS_FLOOR)).min(1.05);
            b = b.remedy(Remedy::at_least(
                SubjectRef::new("", "heating.generationEfficiency", copy("Generation efficiency", "Erzeugungswirkungsgrad")),
                dimensionless(doc.heating.generation_efficiency),
                dimensionless(gen_req),
                copy(
                    &format!("Raise generation efficiency from {:.2} to at least {:.2}.", doc.heating.generation_efficiency, gen_req),
                    &format!("Erzeugungswirkungsgrad von {:.2} auf mindestens {:.2} erhöhen.", doc.heating.generation_efficiency, gen_req),
                ),
            ));
        }
        report.push(b.build());
    }

    // --- Part 8 DHW (DIN V 18599-8 / -10 profile + loss allowances) ---
    {
        let q_w = derived.q_w_kwh;
        let n = occupants(doc) as f64;
        let profile = dhw_profile_specific_kwh_person_a(doc.use_class);
        let limit = dhw_limit_kwh_a(doc);
        let mut b = CheckResult::assess(
            "din18599.8.dhw",
            "DIN V 18599-8",
            ClauseId::new("DIN V 18599", "8", "6"),
            SubjectRef::new("", "dhw.specificDemandKwhPersonA", copy("DHW specific demand", "TWW-spezifischer Bedarf")),
            copy("Domestic hot water demand", "Trinkwarmwasserbedarf"),
        )
        .utilization(energy(q_w), energy(limit))
        .annex(annex)
        .explanation(copy(
            &format!(
                "Q_W = {q_w:.0} kWh/a (= {n:.0}·{:.0} + storage {:.0} + distribution {:.0}); limit {limit:.0} from DIN V 18599-10 profile {profile:.0} kWh/(Person·a) plus -8 loss caps.",
                doc.dhw.specific_demand_kwh_person_a, doc.dhw.storage_loss_kwh_a, doc.dhw.distribution_loss_kwh_a
            ),
            &format!(
                "Q_W = {q_w:.0} kWh/a (= {n:.0}·{:.0} + Speicher {:.0} + Verteilung {:.0}); Grenzwert {limit:.0} aus DIN-V-18599-10-Profil {profile:.0} kWh/(Person·a) zuzüglich -8-Verlustgrenzen.",
                doc.dhw.specific_demand_kwh_person_a, doc.dhw.storage_loss_kwh_a, doc.dhw.distribution_loss_kwh_a
            ),
        ));
        if q_w > limit {
            if doc.dhw.specific_demand_kwh_person_a > profile + 1e-9 {
                b = b.remedy(Remedy::at_most(
                    SubjectRef::new("", "dhw.specificDemandKwhPersonA", copy("DHW specific demand", "TWW-spezifischer Bedarf")),
                    energy(doc.dhw.specific_demand_kwh_person_a),
                    energy(profile),
                    copy(
                        &format!("Reduce specific DHW demand from {:.0} to at most {profile:.0} kWh/(Person·a) (DIN V 18599-10).", doc.dhw.specific_demand_kwh_person_a),
                        &format!("Spezifischen TWW-Bedarf von {:.0} auf höchstens {profile:.0} kWh/(Person·a) senken (DIN V 18599-10).", doc.dhw.specific_demand_kwh_person_a),
                    ),
                ));
            }
            let room = (limit - n * doc.dhw.specific_demand_kwh_person_a.min(profile) - doc.dhw.distribution_loss_kwh_a).max(0.0);
            if doc.dhw.storage_loss_kwh_a > room + 1e-9 {
                b = b.remedy(Remedy::at_most(
                    SubjectRef::new("", "dhw.storageLossKwhA", copy("DHW storage loss", "TWW-Speicherverlust")),
                    energy(doc.dhw.storage_loss_kwh_a),
                    energy(room),
                    copy(
                        &format!("Reduce DHW storage losses from {:.0} to at most {room:.0} kWh/a.", doc.dhw.storage_loss_kwh_a),
                        &format!("TWW-Speicherverluste von {:.0} auf höchstens {room:.0} kWh/a senken.", doc.dhw.storage_loss_kwh_a),
                    ),
                ));
            }
            let room_dist = (limit - n * doc.dhw.specific_demand_kwh_person_a.min(profile) - doc.dhw.storage_loss_kwh_a.min(room)).max(0.0);
            if doc.dhw.distribution_loss_kwh_a > room_dist + 1e-9 {
                b = b.remedy(Remedy::at_most(
                    SubjectRef::new("", "dhw.distributionLossKwhA", copy("DHW distribution loss", "TWW-Verteilverlust")),
                    energy(doc.dhw.distribution_loss_kwh_a),
                    energy(room_dist),
                    copy(
                        &format!("Reduce DHW distribution losses from {:.0} to at most {room_dist:.0} kWh/a.", doc.dhw.distribution_loss_kwh_a),
                        &format!("TWW-Verteilverluste von {:.0} auf höchstens {room_dist:.0} kWh/a senken.", doc.dhw.distribution_loss_kwh_a),
                    ),
                ));
            }
        }
        report.push(b.build());
    }

        // --- Part 2 cooling setpoint per zone (DIN V 18599-10). A lower θ_i,c raises Q_C,nd. ---
    {
        let q_c = derived.q_c_nd_kwh;
        let min_theta = din_v_18599_10_zone_defaults::THETA_I_COOL_C;
        for zone in &doc.zones {
            let path = format!("zones[id={}].thetaICoolC", zone.id);
            let mut b = CheckResult::assess(
                format!("din18599.2.cooling-need.{}", zone.id),
                "DIN V 18599-2",
                ClauseId::new("DIN V 18599", "2", "Q_C,nd"),
                SubjectRef::new(&zone.id, &path, copy("Indoor cooling setpoint", "Kühlsolltemperatur")),
                copy("Cooling setpoint for Q_C,nd", "Kühlsollwert für Q_C,nd"),
            )
            .annex(annex)
            .minimum(dimensionless(zone.theta_i_cool_c), dimensionless(min_theta))
            .explanation(copy(
                &format!("θ_i,c = {:.1} °C vs DIN V 18599-10 {min_theta:.0} °C. Q_C,nd = {q_c:.0} kWh/a.", zone.theta_i_cool_c),
                &format!("θ_i,c = {:.1} °C gegenüber DIN V 18599-10 {min_theta:.0} °C. Q_C,nd = {q_c:.0} kWh/a.", zone.theta_i_cool_c),
            ));
            if zone.theta_i_cool_c + 1e-9 < min_theta {
                b = b.remedy(Remedy::at_least(
                    SubjectRef::new(&zone.id, &path, copy("Indoor cooling setpoint", "Kühlsolltemperatur")),
                    dimensionless(zone.theta_i_cool_c),
                    dimensionless(min_theta),
                    copy(
                        &format!("Raise the cooling setpoint from {:.1} °C to at least {min_theta:.0} °C.", zone.theta_i_cool_c),
                        &format!("Kühlsollwert von {:.1} °C auf mindestens {min_theta:.0} °C anheben.", zone.theta_i_cool_c),
                    ),
                ));
            }
            report.push(b.build());
        }
    }

    // --- Part 7 cooling ---
    if doc.cooling.is_installed() {
        let q_c = derived.q_c_nd_kwh;
        let limit = derived.q_h_nd_kwh.max(1.0) * DIN_V_18599_7_COOLING_TO_HEATING_RATIO;
        let mut b = CheckResult::assess(
            "din18599.7.cooling",
            "DIN V 18599-7",
            ClauseId::new("DIN V 18599", "7", "6"),
            whole_building(),
            copy("Net cooling demand Q_C,nd", "Kühlbedarf Q_C,nd"),
        )
        .utilization(energy(q_c), energy(limit))
        .annex(annex)
        .explanation(copy(&format!("Q_C,nd = {q_c:.0} kWh/a (limit 0.5 · Q_H,nd)."), &format!("Q_C,nd = {q_c:.0} kWh/a (Grenzwert 0,5 · Q_H,nd).")));
        if q_c > limit {
            if let Some(el) = doc.elements.iter().filter(|e| e.kind == ElementKind::Window).max_by(|a, b| a.g_value.partial_cmp(&b.g_value).unwrap()) {
                let g_req = (el.g_value * din_v_18599_7_g_remedy::REDUCE_FACTOR).max(din_v_18599_7_g_remedy::FLOOR);
                b = b.remedy(Remedy::at_most(
                    element_ref(el, &element_g_path(el)),
                    dimensionless(el.g_value),
                    dimensionless(g_req),
                    copy(
                        &format!("Reduce g-value of {} from {:.2} to at most {:.2}.", el.label_en, el.g_value, g_req),
                        &format!("g-Wert von {} von {:.2} auf höchstens {:.2} senken.", el.label_de, el.g_value, g_req),
                    ),
                ));
            }
        }
        report.push(b.build());
    } else {
        report.push(
            CheckResult::assess(
                "din18599.7.cooling",
                "DIN V 18599-7",
                ClauseId::new("DIN V 18599", "7", "6"),
                whole_building(),
                copy("Net cooling demand Q_C,nd", "Kühlbedarf Q_C,nd"),
            )
            .not_applicable(copy("No cooling system present.", "Keine Kühlanlage vorhanden."))
            .annex(annex)
            .build(),
        );
    }

    // --- Part 11 automation ---
    {
        let f = automation_factor(doc.automation_class);
        let computed = if matches!(doc.automation_class, AutomationClass::D) { 1.1 } else { f };
        let mut b = CheckResult::assess(
            "din18599.11.automation",
            "DIN V 18599-11",
            ClauseId::new("DIN V 18599", "11", "5"),
            SubjectRef::new("", "automationClass", copy("Automation class", "Automationsklasse")),
            copy("Building automation class", "Gebäudeautomation"),
        )
        .utilization(dimensionless(computed), dimensionless(1.0))
        .annex(annex)
        .explanation(copy(
            &format!("BACS class {:?}, factor f_BAC = {f:.2}.", doc.automation_class),
            &format!("GA-Klasse {:?}, Faktor f_BAC = {f:.2}.", doc.automation_class),
        ));
        if matches!(doc.automation_class, AutomationClass::D) {
            b = b.remedy(Remedy::one_of(
                SubjectRef::new("", "automationClass", copy("Automation class", "Automationsklasse")),
                vec!["A".into(), "B".into(), "C".into()],
                copy("Upgrade building automation to class C or better.", "Gebäudeautomation auf Klasse C oder besser anheben."),
            ));
        }
        report.push(b.build());
    }

    // --- Part 9 / renewables PV credit (minimum PV for failing primary energy) ---
    // --- GEG primary energy vs reference × 0.55 ---
    {
        let q_p = derived.q_p_kwh;
        let limit = derived.q_p_limit_kwh.max(1.0);
        let mut b = CheckResult::assess(
            "din18599.geg.qp",
            "GEG §10 / Anlage 1–2",
            ClauseId::new("GEG", "§10", "Q_P"),
            SubjectRef::new("", "heating.energyCarrier", copy("Heating energy carrier", "Energieträger Heizung")),
            copy("Primary energy Q_P ≤ 0.55 · Q_P,Ref", "Primärenergie Q_P ≤ 0,55 · Q_P,Ref"),
        )
        .utilization(energy(q_p), energy(limit))
        .annex(annex)
        .explanation(copy(
            &format!("Q_P = {q_p:.0} kWh/a; Q_P,Ref = {:.0}; limit = 0.55·Q_P,Ref = {limit:.0} kWh/a.", derived.q_p_ref_kwh),
            &format!("Q_P = {q_p:.0} kWh/a; Q_P,Ref = {:.0}; Grenzwert = 0,55·Q_P,Ref = {limit:.0} kWh/a.", derived.q_p_ref_kwh),
        ));
        if q_p > limit {
            let deficit = q_p - limit;
            let fp_el = primary_energy_factor("electricity");
            let annual_g = doc.climate.g_h_w_m2.iter().sum::<f64>() / 12.0 * 8760.0 / 1000.0;
            let pv_extra = if doc.renewables.pv_efficiency * annual_g * fp_el <= 1e-9 {
                50.0
            } else {
                deficit / (doc.renewables.pv_efficiency.max(DIN_V_18599_5_ETA_SYS_FLOOR) * annual_g * fp_el)
            };
            let pv_req = doc.renewables.pv_area_m2 + pv_extra;
            b = b
                .remedy(Remedy::at_least(
                    SubjectRef::new("", "renewables.pvAreaM2", copy("PV area", "PV-Fläche")),
                    Quantity::new(QuantityKind::Area, doc.renewables.pv_area_m2),
                    Quantity::new(QuantityKind::Area, pv_req),
                    copy(
                        &format!("Increase PV area from {:.1} m² to at least {:.1} m².", doc.renewables.pv_area_m2, pv_req),
                        &format!("PV-Fläche von {:.1} m² auf mindestens {:.1} m² erhöhen.", doc.renewables.pv_area_m2, pv_req),
                    ),
                ))
                .remedy(Remedy::one_of(
                    SubjectRef::new("", "heating.energyCarrier", copy("Heating energy carrier", "Energieträger Heizung")),
                    vec!["natural_gas".into(), "biomass".into(), "district_heating".into()],
                    copy(
                        &format!("Switch heating carrier from '{}' (f_p={:.2}) to biomass or district heating.", doc.heating.energy_carrier, primary_energy_factor(&doc.heating.energy_carrier)),
                        &format!("Energieträger Heizung von '{}' (f_p={:.2}) auf Biomasse oder Fernwärme umstellen.", doc.heating.energy_carrier, primary_energy_factor(&doc.heating.energy_carrier)),
                    ),
                ));
            let gen_req = (doc.heating.generation_efficiency * (q_p / limit).min(1.4)).min(1.05);
            if gen_req > doc.heating.generation_efficiency {
                b = b.remedy(Remedy::at_least(
                    SubjectRef::new("", "heating.generationEfficiency", copy("Generation efficiency", "Erzeugungswirkungsgrad")),
                    dimensionless(doc.heating.generation_efficiency),
                    dimensionless(gen_req),
                    copy(
                        &format!("Raise generation efficiency from {:.2} to at least {:.2}.", doc.heating.generation_efficiency, gen_req),
                        &format!("Erzeugungswirkungsgrad von {:.2} auf mindestens {:.2} erhöhen.", doc.heating.generation_efficiency, gen_req),
                    ),
                ));
            }
        }
        report.push(b.build());
    }

    // --- Part 12 tabular (applicability gated) ---
    match doc.method {
        CalculationMethod::Tabular => {
            let q_tab = tabular_specific_primary_energy_kwh_m2(doc.use_class) * doc.net_floor_area_m2 * automation_factor(doc.automation_class);
            let limit = derived.q_p_limit_kwh.max(1.0);
            let mut b = CheckResult::assess(
                "din18599.12.tabular",
                "DIN V 18599-12",
                ClauseId::new("DIN V 18599", "12", "4"),
                whole_building(),
                copy("Tabular primary energy", "Tabellarische Primärenergie"),
            )
            .utilization(energy(q_tab), energy(limit))
            .annex(annex)
            .explanation(copy(
                &format!("Q_P,tab = {q_tab:.0} kWh/a vs GEG limit {limit:.0}."),
                &format!("Q_P,tab = {q_tab:.0} kWh/a gegenüber GEG-Grenzwert {limit:.0}."),
            ));
            if q_tab > limit {
                b = b.remedy(Remedy::one_of(
                    SubjectRef::new("", "method", copy("Calculation method", "Berechnungsverfahren")),
                    vec!["DetailedMonthly".into()],
                    copy("Switch to detailed monthly method to resolve tabular exceedance.", "Auf detailliertes Monatsbilanzverfahren wechseln."),
                ));
            }
            report.push(b.build());
        }
        CalculationMethod::DetailedMonthly => {
            report.push(
                CheckResult::assess(
                    "din18599.12.tabular",
                    "DIN V 18599-12",
                    ClauseId::new("DIN V 18599", "12", "4"),
                    whole_building(),
                    copy("Tabular primary energy", "Tabellarische Primärenergie"),
                )
                .not_applicable(copy("Detailed monthly method selected; tabular method not applicable.", "Detailliertes Monatsverfahren gewählt; Tabellenverfahren nicht anwendbar."))
                .annex(annex)
                .build(),
            );
        }
    }

    report
}
//#endregion 🔖️Checks
