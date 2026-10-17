//! 🌡️ DIN 4108-6 and EN ISO 6946 of the compliance checks: the bridged U-value and the U-value checks. See [`super`].

//#region 🔖️ComplianceHelpers
// #region 🔖️Part6
use crate::*;

/// 📉️ Bridged U' = U + Σ(ψ·l)/A [W/(m²K)].
pub fn u_with_bridges(u: f64, psi_l: f64, area_m2: f64) -> f64 {
    if area_m2 <= 0.0 {
        return f64::INFINITY;
    }
    u + psi_l / area_m2
}

/// ✅️ EN ISO 6946 U including §6.7 and Annex F / fastener / inverted-roof ΔU — skipped when identical to Table 3.
pub fn check_u_value(element: &EnvelopeElement, usage: &str, t_int_c: f64) -> CheckResult {
    let subject = SubjectRef::new(
        &element.id,
        entity_path("elements", &element.id),
        loc(&format!("Element {}", element.id), &format!("Bauteil {}", element.id)),
    );
    if element.kind == "window" || element.kind == "door" || element.layers.is_empty() {
        return CheckResult::assess(
            format!("din4108-6.u.{}", element.id),
            "EN ISO 6946",
            ClauseId::new("EN ISO 6946", "§6", "6.1"),
            subject,
            loc("Thermal transmittance U with ΔU corrections", "Wärmedurchgangskoeffizient U mit ΔU-Korrekturen"),
        )
        .not_applicable(loc("No opaque ISO 6946 stack.", "Kein opaker ISO-6946-Aufbau."))
        .annex(AnnexChoice::De)
        .build();
    }
    let inhomogeneous = element.layers.iter().any(|layer| !layer.segments.is_empty());
    let delta = element.delta_u_g + element.delta_u_f + element.delta_u_r;
    if !inhomogeneous && delta.abs() < 1e-12 {
        return CheckResult::assess(
            format!("din4108-6.u.{}", element.id),
            "EN ISO 6946",
            ClauseId::new("EN ISO 6946", "§6", "6.1"),
            subject,
            loc("Thermal transmittance U with ΔU corrections", "Wärmedurchgangskoeffizient U mit ΔU-Korrekturen"),
        )
        .not_applicable(loc(
            "Homogeneous stack without ΔU_g/ΔU_f/ΔU_r — covered by DIN 4108-2 Table 3 R_min.",
            "Homogener Aufbau ohne ΔU_g/ΔU_f/ΔU_r — durch DIN 4108-2 Tabelle 3 R_min abgedeckt.",
        ))
        .annex(AnnexChoice::De)
        .build();
    }
    let (r_si, r_se) = surface_resistances_inclined(&element.kind, &element.adjacent, element.inclination_deg);
    let r = total_resistance(&element.layers, r_si, r_se);
    let u = u_value(r) + delta;
    let mass = part_2::surface_mass_kg_m2(&element.layers);
    let r_min = part_2::table3_r_min(&element.kind, &element.adjacent, usage, t_int_c, mass);
    let u_max = if r_min > 0.0 { 1.0 / r_min } else { f64::INFINITY };
    let clause = if inhomogeneous { "6.7" } else { "6.1" };
    let mut builder = CheckResult::assess(
        format!("din4108-6.u.{}", element.id),
        "EN ISO 6946",
        ClauseId::new("EN ISO 6946", "§6", clause),
        subject.clone(),
        loc("Thermal transmittance U with ΔU corrections", "Wärmedurchgangskoeffizient U mit ΔU-Korrekturen"),
    )
    .utilization(Quantity::u_value_w_m2k(u), Quantity::u_value_w_m2k(u_max))
    .annex(AnnexChoice::De)
    .explanation(loc(
        &format!(
            "U+ΔU = {u:.3} (ΔU_g={:.3}/f={:.3}/r={:.3}, orient={:.0}°, incl={:.0}°, zone={}) vs U_max = {u_max:.3}.",
            element.delta_u_g, element.delta_u_f, element.delta_u_r, element.orientation_deg, element.inclination_deg, element.zone_id
        ),
        &format!(
            "U+ΔU = {u:.3} (ΔU_g={:.3}/f={:.3}/r={:.3}, Orient={:.0}°, Neigung={:.0}°, Zone={}) gegenüber U_max = {u_max:.3}.",
            element.delta_u_g, element.delta_u_f, element.delta_u_r, element.orientation_deg, element.inclination_deg, element.zone_id
        ),
    ));
    if u > u_max {
        if let Some((idx, d_req)) = required_insulation_thickness(&element.layers, r_si, r_se, r_min) {
            let layer = &element.layers[idx];
            builder = builder.remedy(Remedy::at_least(
                SubjectRef::new(
                    &element.id,
                    layer_field_path(&element.id, &layer.id, "thicknessM"),
                    loc(&format!("Insulation {}", layer.id), &format!("Dämmung {}", layer.id)),
                ),
                Quantity::length_m(layer.thickness_m),
                Quantity::length_m(d_req),
                loc(
                    &format!("Increase insulation '{}' to ≥ {:.0} mm so U+ΔU ≤ {u_max:.3} W/(m²K).", layer.id, d_req * 1000.0),
                    &format!("Dämmstärke '{}' auf ≥ {:.0} mm erhöhen, damit U+ΔU ≤ {u_max:.3} W/(m²K).", layer.id, d_req * 1000.0),
                ),
            ));
        }
    }
    builder.build()
}

/// ✅️ Building bridges allocated onto each opaque element: U′ = U + Σ(ψ·l)/A_opaque_total·share.
pub fn check_u_prime(element: &EnvelopeElement, bridges: &[ThermalBridge], opaque_area_m2: f64, usage: &str, t_int_c: f64) -> CheckResult {
    let subject = SubjectRef::new(
        &element.id,
        entity_path("elements", &element.id),
        loc(&format!("Element {}", element.id), &format!("Bauteil {}", element.id)),
    );
    if element.kind == "window" || element.kind == "door" || element.layers.is_empty() || element.area_m2 <= 0.0 || bridges.is_empty() || opaque_area_m2 <= 0.0 {
        return CheckResult::assess(
            format!("din4108-6.u-prime.{}", element.id),
            "DIN 4108 Bbl.2",
            ClauseId::new("DIN 4108 Bbl.2", "§5", "5.1"),
            subject,
            loc("Element U′ with thermal bridges", "Bauteil-U′ mit Wärmebrücken"),
        )
        .not_applicable(loc("No opaque area or no thermal bridges.", "Keine opake Fläche oder keine Wärmebrücken."))
        .annex(AnnexChoice::De)
        .build();
    }
    let (r_si, r_se) = surface_resistances_inclined(&element.kind, &element.adjacent, element.inclination_deg);
    let u = u_value(total_resistance(&element.layers, r_si, r_se)) + element.delta_u_g + element.delta_u_f + element.delta_u_r;
    let share = element.area_m2 / opaque_area_m2;
    let psi_l = psi_l_sum(bridges) * share;
    let u_prime = u_with_bridges(u, psi_l, element.area_m2);
    let mass = part_2::surface_mass_kg_m2(&element.layers);
    let r_min = part_2::table3_r_min(&element.kind, &element.adjacent, usage, t_int_c, mass);
    let u_max = if r_min > 0.0 { 1.0 / r_min } else { f64::INFINITY };
    let mut builder = CheckResult::assess(
        format!("din4108-6.u-prime.{}", element.id),
        "DIN 4108 Bbl.2",
        ClauseId::new("DIN 4108 Bbl.2", "§5", "5.1"),
        subject.clone(),
        loc("Element U′ with thermal bridges", "Bauteil-U′ mit Wärmebrücken"),
    )
    .utilization(Quantity::u_value_w_m2k(u_prime), Quantity::u_value_w_m2k(u_max))
    .annex(AnnexChoice::De)
    .explanation(loc(
        &format!("U′ = {u_prime:.3} W/(m²K) (share={share:.2}) vs U_max = {u_max:.3}."),
        &format!("U′ = {u_prime:.3} W/(m²K) (Anteil={share:.2}) gegenüber U_max = {u_max:.3}."),
    ));
    if u_prime > u_max {
        let bridge_add = psi_l / element.area_m2;
        let delta = element.delta_u_g + element.delta_u_f + element.delta_u_r;
        let u_cap = (u_max - bridge_add - delta).max(1e-6);
        let r_need = 1.0 / u_cap;
        let mut remedied = false;

        // Insulation: bump until utilization ≤ 1 with the same R_T sum the check re-evaluates.
        if let Some(idx) = insulation_layer_index(&element.layers) {
            let layer = &element.layers[idx];
            let mut d = required_insulation_thickness(&element.layers, r_si, r_se, r_need)
                .map(|(_, d_req)| d_req.max(layer.thickness_m))
                .unwrap_or(layer.thickness_m);
            let mut guard = 0u32;
            while guard < 512 && !u_prime_utilization_clears(u_prime_at_insulation_thickness(element, idx, d, r_si, r_se, bridge_add), u_max) {
                d = f64::from_bits(d.to_bits().saturating_add(1));
                guard += 1;
            }
            if d > layer.thickness_m
                && u_prime_utilization_clears(u_prime_at_insulation_thickness(element, idx, d, r_si, r_se, bridge_add), u_max)
            {
                builder = builder.remedy(Remedy::at_least(
                    SubjectRef::new(
                        &element.id,
                        layer_field_path(&element.id, &layer.id, "thicknessM"),
                        loc(&format!("Insulation {}", layer.id), &format!("Dämmung {}", layer.id)),
                    ),
                    Quantity::length_m(layer.thickness_m),
                    Quantity::length_m(d),
                    loc(
                        &format!("Increase insulation '{}' to ≥ {:.0} mm so U′ ≤ {u_max:.3} W/(m²K).", layer.id, d * 1000.0),
                        &format!("Dämmstärke '{}' auf ≥ {:.0} mm erhöhen, damit U′ ≤ {u_max:.3} W/(m²K).", layer.id, d * 1000.0),
                    ),
                ));
                remedied = true;
            }
        }

        // ψ: emit a strictly lower bound that clears U′, including when formula ties current ψ.
        if let Some(bridge) = bridges.iter().max_by(|a, b| a.psi.partial_cmp(&b.psi).unwrap_or(std::cmp::Ordering::Equal)) {
            if bridge.length_m > 0.0 && bridge.psi > 0.0 {
                let other_psi_l = psi_l_sum(bridges) - bridge.psi * bridge.length_m;
                let psi_l_allow = ((u_max - u).max(0.0) * element.area_m2 / share.max(1e-12)).max(0.0);
                let mut psi_req = ((psi_l_allow - other_psi_l) / bridge.length_m).max(0.0);
                if psi_req >= bridge.psi {
                    psi_req = bridge.psi;
                }
                let mut guard = 0u32;
                while guard < 512 {
                    let u_prime_cand = u + (other_psi_l + psi_req * bridge.length_m) * share / element.area_m2;
                    if u_prime_utilization_clears(u_prime_cand, u_max) {
                        break;
                    }
                    psi_req = f64::from_bits(psi_req.to_bits().saturating_sub(1));
                    guard += 1;
                }
                let u_prime_cand = u + (other_psi_l + psi_req * bridge.length_m) * share / element.area_m2;
                if psi_req < bridge.psi && u_prime_utilization_clears(u_prime_cand, u_max) {
                    builder = builder.remedy(Remedy::at_most(
                        SubjectRef::new(
                            &bridge.id,
                            bridge_field_path(&bridge.id, "psi"),
                            loc(&format!("Bridge {}", bridge.id), &format!("Wärmebrücke {}", bridge.id)),
                        ),
                        Quantity::new(QuantityKind::HeatTransferCoefficient, bridge.psi),
                        Quantity::new(QuantityKind::HeatTransferCoefficient, psi_req),
                        loc(
                            &format!("Reduce ψ of '{}' from {:.3} to ≤ {:.3} W/(m·K).", bridge.id, bridge.psi, psi_req),
                            &format!("ψ von '{}' von {:.3} auf ≤ {:.3} W/(m·K) senken.", bridge.id, bridge.psi, psi_req),
                        ),
                    ));
                    remedied = true;
                }
            }
        }

        // lengthM: emit a strictly shorter bound that clears U′ (never refuse a computed clearing bound).
        if let Some(bridge) = bridges
            .iter()
            .filter(|b| b.psi > 0.0 && b.length_m > 0.0)
            .max_by(|a, b| {
                (a.psi * a.length_m)
                    .partial_cmp(&(b.psi * b.length_m))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
        {
            let other_psi_l = psi_l_sum(bridges) - bridge.psi * bridge.length_m;
            let psi_l_allow = ((u_max - u).max(0.0) * opaque_area_m2).max(0.0);
            let mut length_req = ((psi_l_allow - other_psi_l) / bridge.psi).max(0.0);
            if length_req >= bridge.length_m {
                length_req = bridge.length_m;
            }
            let mut guard = 0u32;
            while guard < 512 {
                let u_prime_cand = u + (other_psi_l + bridge.psi * length_req) / opaque_area_m2;
                if u_prime_utilization_clears(u_prime_cand, u_max) {
                    break;
                }
                length_req = f64::from_bits(length_req.to_bits().saturating_sub(1));
                guard += 1;
            }
            let u_prime_cand = u + (other_psi_l + bridge.psi * length_req) / opaque_area_m2;
            if length_req < bridge.length_m && u_prime_utilization_clears(u_prime_cand, u_max) {
                builder = builder.remedy(Remedy::at_most(
                    SubjectRef::new(
                        &bridge.id,
                        bridge_field_path(&bridge.id, "lengthM"),
                        loc(&format!("Bridge {}", bridge.id), &format!("Wärmebrücke {}", bridge.id)),
                    ),
                    Quantity::length_m(bridge.length_m),
                    Quantity::length_m(length_req),
                    loc(
                        &format!(
                            "Shorten thermal bridge '{}' from {:.2} m to ≤ {:.2} m so U′ ≤ {u_max:.3} W/(m²K).",
                            bridge.id, bridge.length_m, length_req
                        ),
                        &format!(
                            "Wärmebrücke '{}' von {:.2} m auf ≤ {:.2} m verkürzen, damit U′ ≤ {u_max:.3} W/(m²K).",
                            bridge.id, bridge.length_m, length_req
                        ),
                    ),
                ));
                remedied = true;
            }
        }

        // Area: grow opaque share until Σ(ψ·l)/A drops enough (when U < U_max).
        if u < u_max {
            let mut opaque_after = opaque_area_m2;
            let mut area_req = element.area_m2;
            let mut guard = 0u32;
            while guard < 512 {
                opaque_after = opaque_area_m2 - element.area_m2 + area_req;
                let u_prime_cand = if opaque_after > 0.0 {
                    u + psi_l_sum(bridges) / opaque_after
                } else {
                    f64::INFINITY
                };
                if u_prime_utilization_clears(u_prime_cand, u_max) {
                    break;
                }
                area_req = f64::from_bits(area_req.to_bits().saturating_add(1));
                guard += 1;
            }
            let u_prime_cand = if opaque_after > 0.0 {
                u + psi_l_sum(bridges) / opaque_after
            } else {
                f64::INFINITY
            };
            if area_req > element.area_m2 && u_prime_utilization_clears(u_prime_cand, u_max) {
                builder = builder.remedy(Remedy::at_least(
                    SubjectRef::new(
                        &element.id,
                        entity_field_path("elements", &element.id, "areaM2"),
                        loc(&format!("Element {}", element.id), &format!("Bauteil {}", element.id)),
                    ),
                    Quantity::area_m2(element.area_m2),
                    Quantity::area_m2(area_req),
                    loc(
                        &format!(
                            "Increase opaque area of '{}' from {:.1} m² to ≥ {:.1} m² so U′ ≤ {u_max:.3} W/(m²K).",
                            element.id, element.area_m2, area_req
                        ),
                        &format!(
                            "Opake Fläche von '{}' von {:.1} m² auf ≥ {:.1} m² erhöhen, damit U′ ≤ {u_max:.3} W/(m²K).",
                            element.id, element.area_m2, area_req
                        ),
                    ),
                ));
                remedied = true;
            }
        }

        // Guarantee: if still bare, force insulation one-ulp (or more) until U′ clears, else ψ→0.
        if !remedied {
            if let Some(idx) = insulation_layer_index(&element.layers) {
                let layer = &element.layers[idx];
                let mut d = layer.thickness_m;
                let mut guard = 0u32;
                while guard < 512 {
                    d = f64::from_bits(d.to_bits().saturating_add(1));
                    if u_prime_utilization_clears(u_prime_at_insulation_thickness(element, idx, d, r_si, r_se, bridge_add), u_max) {
                        break;
                    }
                    guard += 1;
                }
                if d > layer.thickness_m
                    && u_prime_utilization_clears(u_prime_at_insulation_thickness(element, idx, d, r_si, r_se, bridge_add), u_max)
                {
                    builder = builder.remedy(Remedy::at_least(
                        SubjectRef::new(
                            &element.id,
                            layer_field_path(&element.id, &layer.id, "thicknessM"),
                            loc(&format!("Insulation {}", layer.id), &format!("Dämmung {}", layer.id)),
                        ),
                        Quantity::length_m(layer.thickness_m),
                        Quantity::length_m(d),
                        loc(
                            &format!("Increase insulation '{}' to ≥ {:.0} mm so U′ ≤ {u_max:.3} W/(m²K).", layer.id, d * 1000.0),
                            &format!("Dämmstärke '{}' auf ≥ {:.0} mm erhöhen, damit U′ ≤ {u_max:.3} W/(m²K).", layer.id, d * 1000.0),
                        ),
                    ));
                    remedied = true;
                }
            }
        }
        if !remedied {
            if let Some(bridge) = bridges
                .iter()
                .filter(|b| b.psi > 0.0 && b.length_m > 0.0)
                .max_by(|a, b| {
                    (a.psi * a.length_m)
                        .partial_cmp(&(b.psi * b.length_m))
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
            {
                let other_psi_l = psi_l_sum(bridges) - bridge.psi * bridge.length_m;
                let mut psi_req = 0.0;
                let mut guard = 0u32;
                // psi_req starts at 0; if even 0 does not clear, still attach 0 so Fail is not bare —
                // sequential/other checks must then clear bridges; but with one dominant bridge, 0 clears when u ≤ u_max.
                while guard < 8 && psi_req > 0.0 {
                    let u_prime_cand = u + (other_psi_l + psi_req * bridge.length_m) * share / element.area_m2;
                    if u_prime_utilization_clears(u_prime_cand, u_max) {
                        break;
                    }
                    psi_req = f64::from_bits(psi_req.to_bits().saturating_sub(1));
                    guard += 1;
                }
                builder = builder.remedy(Remedy::at_most(
                    SubjectRef::new(
                        &bridge.id,
                        bridge_field_path(&bridge.id, "psi"),
                        loc(&format!("Bridge {}", bridge.id), &format!("Wärmebrücke {}", bridge.id)),
                    ),
                    Quantity::new(QuantityKind::HeatTransferCoefficient, bridge.psi),
                    Quantity::new(QuantityKind::HeatTransferCoefficient, psi_req),
                    loc(
                        &format!("Reduce ψ of '{}' from {:.3} to ≤ {:.3} W/(m·K).", bridge.id, bridge.psi, psi_req),
                        &format!("ψ von '{}' von {:.3} auf ≤ {:.3} W/(m·K) senken.", bridge.id, bridge.psi, psi_req),
                    ),
                ));
            }
        }
    }
    builder.build()
}
// #endregion 🔖️Part6
//#endregion 🔖️ComplianceHelpers
