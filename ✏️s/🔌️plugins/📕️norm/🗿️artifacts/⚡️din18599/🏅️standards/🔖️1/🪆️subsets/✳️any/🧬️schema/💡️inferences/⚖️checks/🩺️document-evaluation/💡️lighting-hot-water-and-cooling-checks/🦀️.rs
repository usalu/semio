//! 💡️ Lighting, hot water and cooling checks: the lighting power, the heating system efficiency, the hot water, the cooling temperature and the cooling demand of the document evaluation. See [`super`].

use super::*;

/// 💡 Checks the installed lighting power against the profile limit.
pub(super) fn check_lighting(doc: &Din18599Snapshot, report: &mut CheckReport, annex: AnnexChoice) {
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
}

/// ⚙ Checks the heating system efficiency against the minimum.
pub(super) fn check_heating_system_efficiency(doc: &Din18599Snapshot, report: &mut CheckReport, annex: AnnexChoice, derived: &BalanceDerived) {
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
}

/// 🚿 Checks the hot water demand and the loss allowances.
pub(super) fn check_hot_water(doc: &Din18599Snapshot, report: &mut CheckReport, annex: AnnexChoice, derived: &BalanceDerived) {
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
}

/// 🌡 Checks the cooling set temperature against the zone defaults.
pub(super) fn check_cooling_temperature(doc: &Din18599Snapshot, report: &mut CheckReport, annex: AnnexChoice, derived: &BalanceDerived) {
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
}

/// 🧊 Checks the cooling demand against the heating demand ratio.
pub(super) fn check_cooling_demand(doc: &Din18599Snapshot, report: &mut CheckReport, annex: AnnexChoice, derived: &BalanceDerived) {
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
}
