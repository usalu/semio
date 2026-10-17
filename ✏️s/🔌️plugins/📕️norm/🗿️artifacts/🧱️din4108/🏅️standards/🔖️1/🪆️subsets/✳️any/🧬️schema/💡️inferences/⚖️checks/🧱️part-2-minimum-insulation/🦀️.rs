//! 🧱️ DIN 4108-2 of the compliance checks: the minimum thermal insulation, the mould criterion and the surface temperature checks. See [`super`].

//#region 🔖️ComplianceHelpers
// #region 🔖️Part2
use crate::*;

/// 🧱 Area-weighted surface mass m′ = Σ(ρ·d) [kg/m²] for Table 3 light/heavy columns.
pub fn surface_mass_kg_m2(layers: &[LayerDocument]) -> f64 {
    layers.iter().map(layer_surface_mass_kg_m2).sum()
}

/// ⚖️ Area-weighted layer surface mass (DIN 4108-2 Table 3 / ISO 6946 inhomogeneous layers).
pub fn layer_surface_mass_kg_m2(layer: &LayerDocument) -> f64 {
    if layer.segments.is_empty() {
        layer.density * layer.thickness_m
    } else {
        let seg: f64 = layer.segments.iter().map(|seg| seg.fraction * seg.density * layer.thickness_m).sum();
        0.5 * (seg + layer.density * layer.thickness_m)
    }
}

/// 🧱 DIN 4108-2 Table 3 representative R_min rows (residential, ≥19 °C) — shared with catalogue.
pub const TABLE3_R_MIN_ROWS: &[(&str, &str, &str, f64)] = &[
    ("wall", "exterior", "heavy", 1.2),
    ("wall", "exterior", "light", 1.75),
    ("wall", "unheated", "heavy", 0.55),
    ("roof", "exterior", "heavy", 1.2),
    ("roof", "exterior", "light", 1.75),
    ("floor", "ground", "heavy", 0.90),
    ("floor", "ground", "light", 1.2),
    ("frameOpaque", "exterior", "heavy", 1.0),
    ("rollerShutterBox", "exterior", "heavy", 1.0),
];

/// 🧱 DIN 4108-2 Table 3 R_min [m²K/W] — usage sets interior temperature class; light ≤100 kg/m².
pub fn table3_r_min(kind: &str, adjacent: &str, usage: &str, t_int_c: f64, surface_mass_kg_m2: f64) -> f64 {
    let light = surface_mass_kg_m2 > 0.0 && surface_mass_kg_m2 <= 100.0;
    let low_temp = usage == "nonResidential" && t_int_c >= 12.0 && t_int_c < 19.0;
    match kind {
        "window" | "door" => 0.0,
        "frameOpaque" => {
            if low_temp { 0.55 } else if light { 1.4 } else { 1.0 }
        }
        "rollerShutterBox" => {
            if low_temp { 0.70 } else if light { 1.4 } else { 1.0 }
        }
        "wall" => match adjacent {
            "unheated" => {
                if low_temp { 0.35 } else if light { 0.80 } else { 0.55 }
            }
            _ => {
                if low_temp { 0.55 } else if light { 1.75 } else { 1.2 }
            }
        },
        "roof" => {
            if low_temp { 0.55 } else if light { 1.75 } else { 1.2 }
        }
        "floor" => match adjacent {
            "exterior" => {
                if low_temp { 0.55 } else if light { 1.75 } else { 1.2 }
            }
            _ => {
                if low_temp { 0.35 } else if light { 1.2 } else { 0.90 }
            }
        },
        _ => {
            if low_temp { 0.55 } else if light { 1.75 } else { 1.2 }
        }
    }
}

/// ☀️ Summer climate region A/B/C from heating climate zone (DIN 4108-2 Annex).
pub fn summer_region(climate: ClimateZoneDe) -> char {
    match climate {
        ClimateZoneDe::Zone1 => 'A',
        ClimateZoneDe::Zone2 => 'B',
        ClimateZoneDe::Zone3 | ClimateZoneDe::Zone4 => 'C',
    }
}

/// ☀️ DIN 4108-2 Table 8 combined S_x for climate + construction + night ventilation.
pub fn s_x_climate_construction_night(region: char, heaviness: &str, night_ventilation: &str) -> f64 {
    let night = matches!(night_ventilation, "moderate" | "high" | "strong");
    let heavy = heaviness == "heavy";
    let medium = heaviness == "medium";
    match (region, heavy, medium, night) {
        ('A', true, _, false) => 0.10,
        ('A', _, true, false) => 0.07,
        ('A', _, _, false) => 0.04,
        ('A', true, _, true) => 0.13,
        ('A', _, true, true) => 0.10,
        ('A', _, _, true) => 0.06,
        ('B', true, _, false) => 0.08,
        ('B', _, true, false) => 0.06,
        ('B', _, _, false) => 0.03,
        ('B', true, _, true) => 0.10,
        ('B', _, true, true) => 0.08,
        ('B', _, _, true) => 0.05,
        ('C', true, _, false) => 0.06,
        ('C', _, true, false) => 0.045,
        ('C', _, _, false) => 0.025,
        ('C', true, _, true) => 0.08,
        ('C', _, true, true) => 0.06,
        ('C', _, _, true) => 0.035,
        _ => 0.06,
    }
}

/// ☀️ DIN 4108-2 §8 solar-control glazing partial S_x,Sonnenschutzglas.
pub fn s_x_solar_glazing(solar_control: bool) -> f64 {
    if solar_control { 0.03 } else { 0.0 }
}

/// ☀️ DIN 4108-2 §8 passive cooling partial S_x,bautechnische Kühlung.
pub fn s_x_passive_cooling(passive_cooling: bool) -> f64 {
    if passive_cooling { 0.04 } else { 0.0 }
}

/// ☀️ DIN 4108-2 §8 S_zul = Σ S_x (Table 8 climate/construction/night + solar + passive).
pub fn s_zul(region: char, heaviness: &str, night_ventilation: &str) -> f64 {
    s_zul_full(region, heaviness, night_ventilation, false, false)
}

/// ☀️ Full S_zul with solar-control and passive-cooling additives.
pub fn s_zul_full(region: char, heaviness: &str, night_ventilation: &str, solar_control: bool, passive_cooling: bool) -> f64 {
    s_x_climate_construction_night(region, heaviness, night_ventilation)
        + s_x_solar_glazing(solar_control)
        + s_x_passive_cooling(passive_cooling)
}

/// ☀️ Detect solar-control glazing (mean g ≤ 0.40) for S_x,Sonnenschutzglas.
pub fn zone_has_solar_control_glazing(zone: &ThermalZone) -> bool {
    !zone.windows.is_empty() && zone.windows.iter().all(|w| w.g_value <= 0.40)
}

/// ☀️ Detect passive cooling measures from night-ventilation class high/strong.
pub fn zone_has_passive_cooling(zone: &ThermalZone) -> bool {
    matches!(zone.night_ventilation.as_str(), "high" | "strong")
}

/// ☀️ Orientation factor F_w (DIN 4108-2 §8 — window orientation contribution).
pub fn orientation_factor_fw(orientation: &str) -> f64 {
    match orientation {
        "S" | "SE" | "SW" => 1.0,
        "E" | "W" => 0.90,
        "NE" | "NW" => 0.80,
        "N" => 0.70,
        _ => 1.0,
    }
}

/// ☀️ Inclination factor F_i (DIN 4108-2 §8 — 0° horizontal … 90° vertical).
pub fn inclination_factor_fi(inclination_deg: f64) -> f64 {
    let a = inclination_deg.max(0.0);
    0.70 + 0.30 * (a / 90.0)
}

/// ☀️ Orientation factor from element azimuth (degrees from north, clockwise).
pub fn orientation_factor_from_azimuth_deg(orientation_deg: f64) -> f64 {
    let d = orientation_deg.rem_euclid(360.0);
    let label = if (45.0..135.0).contains(&d) {
        "E"
    } else if (135.0..225.0).contains(&d) {
        "S"
    } else if (225.0..315.0).contains(&d) {
        "W"
    } else {
        "N"
    };
    orientation_factor_fw(label)
}

/// ☀️ S_vorh = Σ(A_w · g · F_c · F_w · F_i) / A_G (DIN 4108-2 §8).
pub fn s_vorhanden(zone: &ThermalZone) -> f64 {
    if zone.floor_area_m2 <= 0.0 {
        return f64::INFINITY;
    }
    let sum: f64 = zone
        .windows
        .iter()
        .map(|w| w.area_m2 * w.g_value * w.shading_fc * orientation_factor_fw(&w.orientation) * inclination_factor_fi(w.inclination_deg))
        .sum();
    sum / zone.floor_area_m2
}

/// ☀️ Additional solar entry from transparent envelope elements using orientationDeg/inclinationDeg.
pub fn s_vorhanden_with_elements(zone: &ThermalZone, elements: &[EnvelopeElement]) -> f64 {
    if zone.floor_area_m2 <= 0.0 {
        return f64::INFINITY;
    }
    let mut sum: f64 = zone
        .windows
        .iter()
        .map(|w| w.area_m2 * w.g_value * w.shading_fc * orientation_factor_fw(&w.orientation) * inclination_factor_fi(w.inclination_deg))
        .sum();
    for el in elements.iter().filter(|e| e.zone_id == zone.id && (e.kind == "window" || e.kind == "door")) {
        let g = 0.50;
        let fc = 1.0;
        sum += el.area_m2 * g * fc * orientation_factor_from_azimuth_deg(el.orientation_deg) * inclination_factor_fi(el.inclination_deg);
    }
    sum / zone.floor_area_m2
}

/// 🌡 Zone transmission heat-loss coefficient H_T = Σ(A·U) [W/K] for opaque elements linked to the zone.
pub fn zone_transmission_ht_wk(zone_id: &str, elements: &[EnvelopeElement]) -> f64 {
    elements
        .iter()
        .filter(|e| e.zone_id == zone_id && e.kind != "window" && e.kind != "door" && !e.layers.is_empty())
        .map(|e| {
            let (r_si, r_se) = surface_resistances_inclined(&e.kind, &e.adjacent, e.inclination_deg);
            let u = u_value(total_resistance(&e.layers, r_si, r_se)) + e.delta_u_g + e.delta_u_f + e.delta_u_r;
            e.area_m2 * u
        })
        .sum()
}

/// 🌡 Zone Table 3 capacity H_T,max = Σ(A/R_min) [W/K] for the same opaque set.
pub fn zone_transmission_ht_max_wk(zone_id: &str, elements: &[EnvelopeElement], usage: &str, t_int_c: f64) -> f64 {
    elements
        .iter()
        .filter(|e| e.zone_id == zone_id && e.kind != "window" && e.kind != "door" && !e.layers.is_empty())
        .map(|e| {
            let mass = surface_mass_kg_m2(&e.layers);
            let r_min = table3_r_min(&e.kind, &e.adjacent, usage, t_int_c, mass);
            if r_min > 0.0 {
                e.area_m2 / r_min
            } else {
                0.0
            }
        })
        .sum()
}

/// ✅️ DIN 4108-2 zone-aggregated transmission heat loss vs Table 3 capacity.
pub fn check_zone_transmission_loss(zone: &ThermalZone, elements: &[EnvelopeElement], usage: &str, t_int_c: f64) -> CheckResult {
    let subject = SubjectRef::new(
        &zone.id,
        entity_path("zones", &zone.id),
        loc(&format!("Zone {}", zone.id), &format!("Zone {}", zone.id)),
    );
    let ht = zone_transmission_ht_wk(&zone.id, elements);
    let ht_max = zone_transmission_ht_max_wk(&zone.id, elements, usage, t_int_c);
    let mut builder = CheckResult::assess(
        format!("din4108-2.zone-ht.{}", zone.id),
        "DIN 4108-2",
        ClauseId::new("DIN 4108-2", "Table 3", "H_T"),
        subject.clone(),
        loc("Zone transmission heat-loss H_T", "Zonaler Transmissionswärmeverlust H_T"),
    )
    .annex(AnnexChoice::De);
    if ht_max <= 0.0 && ht <= 0.0 {
        builder = builder.not_applicable(loc(
            "No opaque elements assigned to this zone.",
            "Dieser Zone sind keine opaken Bauteile zugeordnet.",
        ));
    } else {
        let limit = ht_max.max(1e-9);
        builder = builder
            .utilization(Quantity::new(QuantityKind::HeatTransferCoefficient, ht), Quantity::new(QuantityKind::HeatTransferCoefficient, limit))
            .explanation(loc(
                &format!("H_T = Σ(A·U) = {ht:.3} W/K vs Table 3 capacity {ht_max:.3} W/K for zone '{}'.", zone.id),
                &format!("H_T = Σ(A·U) = {ht:.3} W/K gegenüber Tab.3-Kapazität {ht_max:.3} W/K für Zone '{}'.", zone.id),
            ));
        if ht > ht_max {
            let scale = (ht_max / ht).clamp(0.0, 1.0);
            for el in elements.iter().filter(|e| e.zone_id == zone.id && e.kind != "window" && e.kind != "door" && !e.layers.is_empty()) {
                let (r_si, r_se) = surface_resistances_inclined(&el.kind, &el.adjacent, el.inclination_deg);
                let mass = surface_mass_kg_m2(&el.layers);
                let r_min = table3_r_min(&el.kind, &el.adjacent, usage, t_int_c, mass);
                let delta = el.delta_u_g + el.delta_u_f + el.delta_u_r;
                let mut added_thickness = false;
                if r_min > 0.0 {
                    let u_plain_cap = (1.0 / r_min - delta).max(1e-6);
                    let r_need = 1.0 / u_plain_cap;
                    if let Some((idx, d_req)) = required_insulation_thickness(&el.layers, r_si, r_se, r_need) {
                        let layer = &el.layers[idx];
                        if d_req > layer.thickness_m + 1e-12 {
                            builder = builder.remedy(Remedy::at_least(
                                SubjectRef::new(
                                    &el.id,
                                    layer_field_path(&el.id, &layer.id, "thicknessM"),
                                    loc(&format!("Insulation {}", layer.id), &format!("Dämmung {}", layer.id)),
                                ),
                                Quantity::length_m(layer.thickness_m),
                                Quantity::length_m(d_req),
                                loc(
                                    &format!("Increase insulation on '{}' so zone '{}' H_T ≤ {ht_max:.3} W/K.", el.id, zone.id),
                                    &format!("Dämmung an '{}' erhöhen, damit H_T der Zone '{}' ≤ {ht_max:.3} W/K.", el.id, zone.id),
                                ),
                            ));
                            added_thickness = true;
                        }
                    }
                }
                if !added_thickness && el.area_m2 > 0.0 && scale + 1e-12 < 1.0 {
                    let area_req = el.area_m2 * scale;
                    builder = builder.remedy(Remedy::at_most(
                        SubjectRef::new(
                            &el.id,
                            entity_field_path("elements", &el.id, "areaM2"),
                            loc(&format!("Element {}", el.id), &format!("Bauteil {}", el.id)),
                        ),
                        Quantity::area_m2(el.area_m2),
                        Quantity::area_m2(area_req),
                        loc(
                            &format!("Reduce opaque area of '{}' from {:.1} m² to ≤ {:.1} m² so zone H_T ≤ {ht_max:.3} W/K.", el.id, el.area_m2, area_req),
                            &format!("Opake Fläche von '{}' von {:.1} m² auf ≤ {:.1} m² senken, damit H_T ≤ {ht_max:.3} W/K.", el.id, el.area_m2, area_req),
                        ),
                    ));
                }
            }
        }
    }
    builder.build()
}

/// ⚖️ DIN 4108-2 Table 3 areal mass m′ used for the light/heavy column (limit 100 kg/m²).
pub fn check_surface_mass(element: &EnvelopeElement) -> CheckResult {
    let subject = SubjectRef::new(
        &element.id,
        entity_path("elements", &element.id),
        loc(&format!("Element {}", element.id), &format!("Bauteil {}", element.id)),
    );
    if element.kind == "window" || element.kind == "door" || element.layers.is_empty() {
        return CheckResult::assess(
            format!("din4108-2.mass.{}", element.id),
            "DIN 4108-2",
            ClauseId::new("DIN 4108-2", "Table 3", "m′"),
            subject,
            loc("Areal mass m′ for Table 3", "Flächenbezogene Masse m′ für Tabelle 3"),
        )
        .not_applicable(loc("No opaque stack for areal mass.", "Kein opaker Aufbau für flächenbezogene Masse."))
        .annex(AnnexChoice::De)
        .build();
    }
    let mass = surface_mass_kg_m2(&element.layers);
    CheckResult::assess(
        format!("din4108-2.mass.{}", element.id),
        "DIN 4108-2",
        ClauseId::new("DIN 4108-2", "Table 3", "m′"),
        subject,
        loc("Areal mass m′ for Table 3", "Flächenbezogene Masse m′ für Tabelle 3"),
    )
    .utilization(
        Quantity::new(QuantityKind::Dimensionless, mass),
        Quantity::new(QuantityKind::Dimensionless, 100.0),
    )
    .status(semio_s_plugin_norm_compliance::CheckStatus::Pass)
    .annex(AnnexChoice::De)
    .explanation(loc(
        &format!("m′ = Σ(ρ·d) = {mass:.1} kg/m² (Table 3 light column when m′ ≤ 100)."),
        &format!("m′ = Σ(ρ·d) = {mass:.1} kg/m² (Tabelle 3 leichte Bauweise bei m′ ≤ 100)."),
    ))
    .remedy(Remedy::at_most(
        SubjectRef::new(
            &element.id,
            layer_field_path(&element.id, &element.layers[0].id, "density"),
            loc(&format!("Layer {}", element.layers[0].id), &format!("Schicht {}", element.layers[0].id)),
        ),
        Quantity::new(QuantityKind::Dimensionless, mass),
        Quantity::new(QuantityKind::Dimensionless, 100.0),
        loc(
            "Areal mass is recorded for Table 3 column selection (informational Pass).",
            "Flächenbezogene Masse wird für die Tab.3-Spalte erfasst (informativ bestanden).",
        ),
    ))
    .build()
}

/// ✅️ Minimum R per DIN 4108-2 Table 3 for one opaque element.
pub fn check_minimum_r(element: &EnvelopeElement, usage: &str, t_int_c: f64) -> CheckResult {
    let subject = SubjectRef::new(
        &element.id,
        entity_path("elements", &element.id),
        loc(&format!("Element {}", element.id), &format!("Bauteil {}", element.id)),
    );
    if element.kind == "window" || element.kind == "door" {
        return CheckResult::assess(
            format!("din4108-2.table3.{}", element.id),
            "DIN 4108-2",
            ClauseId::new("DIN 4108-2", "Table 3", "R_min"),
            subject,
            loc("Minimum thermal resistance (Table 3)", "Mindestwärmedurchlasswiderstand (Tabelle 3)"),
        )
        .not_applicable(loc("Transparent elements are outside Table 3 opaque R_min.", "Transparente Bauteile liegen außerhalb von Tabelle 3."))
        .annex(AnnexChoice::De)
        .build();
    }
    let (r_si, r_se) = surface_resistances_inclined(&element.kind, &element.adjacent, element.inclination_deg);
    let r = total_resistance(&element.layers, r_si, r_se);
    let mass = surface_mass_kg_m2(&element.layers);
    let r_min = table3_r_min(&element.kind, &element.adjacent, usage, t_int_c, mass);
    let mut builder = CheckResult::assess(
        format!("din4108-2.table3.{}", element.id),
        "DIN 4108-2",
        ClauseId::new("DIN 4108-2", "Table 3", "R_min"),
        subject.clone(),
        loc("Minimum thermal resistance (Table 3)", "Mindestwärmedurchlasswiderstand (Tabelle 3)"),
    )
    .minimum(Quantity::thermal_resistance_m2k_w(r), Quantity::thermal_resistance_m2k_w(r_min))
    .annex(AnnexChoice::De)
    .explanation(loc(
        &format!(
            "R = {r:.3} vs R_min = {r_min:.2} (mass={:.0}, usage={usage}, zone={}, orient={:.0}, incl={:.0}) for {} / {}.",
            surface_mass_kg_m2(&element.layers), element.zone_id, element.orientation_deg, element.inclination_deg, element.kind, element.adjacent
        ),
        &format!(
            "R = {r:.3} gegenüber R_min = {r_min:.2} (Masse={:.0}, Nutzung={usage}, Zone={}, Orient={:.0}, Neigung={:.0}) für {} / {}.",
            surface_mass_kg_m2(&element.layers), element.zone_id, element.orientation_deg, element.inclination_deg, element.kind, element.adjacent
        ),
    ));
    if r < r_min {
        if let Some((idx, d_req)) = required_insulation_thickness(&element.layers, r_si, r_se, r_min) {
            let layer = &element.layers[idx];
            let target = SubjectRef::new(
                &element.id,
                layer_field_path(&element.id, &layer.id, "thicknessM"),
                loc(&format!("Insulation {}", layer.id), &format!("Dämmung {}", layer.id)),
            );
            builder = builder.remedy(Remedy::at_least(
                target,
                Quantity::length_m(layer.thickness_m),
                Quantity::length_m(d_req),
                loc(
                    &format!(
                        "Increase insulation layer '{}' from {:.0} mm to at least {:.0} mm so R ≥ {:.2} m²K/W.",
                        layer.id,
                        layer.thickness_m * 1000.0,
                        d_req * 1000.0,
                        r_min
                    ),
                    &format!(
                        "Dämmstärke der Schicht '{}' von {:.0} mm auf mindestens {:.0} mm erhöhen, damit R ≥ {:.2} m²K/W.",
                        layer.id,
                        layer.thickness_m * 1000.0,
                        d_req * 1000.0,
                        r_min
                    ),
                ),
            ));
        }
    }
    builder.build()
}

/// ✅️ Interior surface temperature factor f_Rsi ≥ 0.70 (DIN 4108-2 mould criterion).
pub fn check_f_rsi(element: &EnvelopeElement, t_int_c: f64, t_ext_c: f64) -> CheckResult {
    let subject = SubjectRef::new(
        &element.id,
        entity_path("elements", &element.id),
        loc(&format!("Element {}", element.id), &format!("Bauteil {}", element.id)),
    );
    if element.kind == "window" || element.kind == "door" || element.layers.is_empty() {
        return CheckResult::assess(
            format!("din4108-2.frsi.{}", element.id),
            "DIN 4108-2",
            ClauseId::new("DIN 4108-2", "§6", "6.2"),
            subject,
            loc("Mould criterion f_Rsi", "Schimmelpilzkriterium f_Rsi"),
        )
        .not_applicable(loc("No opaque layer stack to evaluate f_Rsi.", "Kein opaker Schichtaufbau für f_Rsi."))
        .annex(AnnexChoice::De)
        .build();
    }
    let (r_si, r_se) = surface_resistances_inclined(&element.kind, &element.adjacent, element.inclination_deg);
    let r_t = total_resistance(&element.layers, r_si, r_se);
    let f_rsi = if (t_int_c - t_ext_c).abs() < f64::EPSILON {
        1.0
    } else {
        1.0 - r_si / r_t
    };
    let mut builder = CheckResult::assess(
        format!("din4108-2.frsi.{}", element.id),
        "DIN 4108-2",
        ClauseId::new("DIN 4108-2", "§6", "6.2"),
        subject,
        loc("Mould criterion f_Rsi", "Schimmelpilzkriterium f_Rsi"),
    )
    .minimum(Quantity::new(QuantityKind::Dimensionless, f_rsi), Quantity::new(QuantityKind::Dimensionless, F_RSI_MINIMUM))
    .annex(AnnexChoice::De)
    .explanation(loc(
        &format!("f_Rsi = 1 − R_si/R_T = {f_rsi:.3} (θ_i={t_int_c:.1} °C, θ_e={t_ext_c:.1} °C)."),
        &format!("f_Rsi = 1 − R_si/R_T = {f_rsi:.3} (θ_i={t_int_c:.1} °C, θ_e={t_ext_c:.1} °C, Innen-/Außentemperatur)."),
    ));
    if f_rsi < F_RSI_MINIMUM {
        let r_needed = r_si / (1.0 - F_RSI_MINIMUM);
        if let Some((idx, d_req)) = required_insulation_thickness(&element.layers, r_si, r_se, r_needed) {
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
                    &format!(
                        "Increase insulation '{}' from {:.0} mm to ≥ {:.0} mm so f_Rsi ≥ 0.70.",
                        layer.id,
                        layer.thickness_m * 1000.0,
                        d_req * 1000.0
                    ),
                    &format!(
                        "Dämmstärke '{}' von {:.0} mm auf ≥ {:.0} mm erhöhen, damit f_Rsi ≥ 0,70.",
                        layer.id,
                        layer.thickness_m * 1000.0,
                        d_req * 1000.0
                    ),
                ),
            ));
        }
    }
    builder.build()
}

/// ✅️ Summer heat protection S_vorh ≤ S_zul (DIN 4108-2 §8).
pub fn check_summer_heat(zone: &ThermalZone, climate: ClimateZoneDe, elements: &[EnvelopeElement], usage: &str) -> CheckResult {
    let subject = SubjectRef::new(
        &zone.id,
        entity_field_path("zones", &zone.id, "windows"),
        loc(&format!("Zone {}", zone.id), &format!("Zone {}", zone.id)),
    );
    if usage != "residential" {
        return CheckResult::assess(
            format!("din4108-2.summer.{}", zone.id),
            "DIN 4108-2",
            ClauseId::new("DIN 4108-2", "§8", "8.3"),
            subject,
            loc("Summer heat protection S_vorh", "Sommerlicher Wärmeschutz S_vorh"),
        )
        .not_applicable(loc(
            "DIN 4108-2 §8 summer heat protection applies to residential usage.",
            "DIN 4108-2 §8 sommerlicher Wärmeschutz gilt für Wohnnutzung.",
        ))
        .annex(AnnexChoice::De)
        .build();
    }
    if zone.windows.is_empty() || zone.floor_area_m2 <= 0.0 {
        return CheckResult::assess(
            format!("din4108-2.summer.{}", zone.id),
            "DIN 4108-2",
            ClauseId::new("DIN 4108-2", "§8", "8.3"),
            subject,
            loc("Summer heat protection S_vorh", "Sommerlicher Wärmeschutz S_vorh"),
        )
        .not_applicable(loc("No windows or zero floor area in zone.", "Keine Fenster oder A_G = 0 in der Zone."))
        .annex(AnnexChoice::De)
        .build();
    }
    let region = summer_region(climate);
    let s_v = s_vorhanden_with_elements(zone, elements);
    let solar = zone_has_solar_control_glazing(zone);
    let passive = zone_has_passive_cooling(zone);
    let s_z = s_zul_full(region, &zone.heaviness, &zone.night_ventilation, solar, passive);
    let mut builder = CheckResult::assess(
        format!("din4108-2.summer.{}", zone.id),
        "DIN 4108-2",
        ClauseId::new("DIN 4108-2", "§8", "8.3"),
        subject,
        loc("Summer heat protection S_vorh", "Sommerlicher Wärmeschutz S_vorh"),
    )
    .utilization(Quantity::new(QuantityKind::Dimensionless, s_v), Quantity::new(QuantityKind::Dimensionless, s_z))
    .annex(AnnexChoice::De)
    .explanation(loc(
        &format!(
            "S_vorh = {s_v:.4}, S_zul = {s_z:.4} (region {region}, {}, night={}, windowIncl={:?}).",
            zone.heaviness, zone.night_ventilation, zone.windows.iter().map(|w| w.inclination_deg).collect::<Vec<_>>()
        ),
        &format!(
            "S_vorh = {s_v:.4}, S_zul = {s_z:.4} (Region {region}, {}, Nachtlüftung={}, FensterNeigung={:?}).",
            zone.heaviness, zone.night_ventilation, zone.windows.iter().map(|w| w.inclination_deg).collect::<Vec<_>>()
        ),
    ));
    if s_v > s_z {
        let solar_sum = s_v * zone.floor_area_m2;
        let floor_req = if s_z > 0.0 { solar_sum / s_z } else { zone.floor_area_m2 };
        if floor_req > zone.floor_area_m2 + 1e-12 {
            builder = builder.remedy(Remedy::at_least(
                SubjectRef::new(
                    &zone.id,
                    entity_field_path("zones", &zone.id, "floorAreaM2"),
                    loc(&format!("Zone {}", zone.id), &format!("Zone {}", zone.id)),
                ),
                Quantity::area_m2(zone.floor_area_m2),
                Quantity::area_m2(floor_req),
                loc(
                    &format!("Increase floor area of '{}' from {:.1} m² to ≥ {:.1} m² so S_vorh ≤ S_zul.", zone.id, zone.floor_area_m2, floor_req),
                    &format!("Nutzfläche von '{}' von {:.1} m² auf ≥ {:.1} m² erhöhen, damit S_vorh ≤ S_zul.", zone.id, zone.floor_area_m2, floor_req),
                ),
            ));
        }
        let scale = (s_z / s_v).clamp(0.0, 1.0);
        for win in &zone.windows {
            let fc_req = win.shading_fc * scale;
            if fc_req + 1e-12 < win.shading_fc {
                builder = builder.remedy(Remedy::at_most(
                    SubjectRef::new(
                        &win.id,
                        window_field_path(&zone.id, &win.id, "shadingFc"),
                        loc(&format!("Window {}", win.id), &format!("Fenster {}", win.id)),
                    ),
                    Quantity::new(QuantityKind::Dimensionless, win.shading_fc),
                    Quantity::new(QuantityKind::Dimensionless, fc_req),
                    loc(
                        &format!("Reduce shading factor F_c of '{}' from {:.2} to ≤ {:.2} (or reduce g / window area).", win.id, win.shading_fc, fc_req),
                        &format!("Abminderungsfaktor F_c von '{}' von {:.2} auf ≤ {:.2} senken (oder g / Fensterfläche reduzieren).", win.id, win.shading_fc, fc_req),
                    ),
                ));
            }
            let area_req = win.area_m2 * scale;
            if area_req + 1e-12 < win.area_m2 {
                builder = builder.remedy(Remedy::at_most(
                    SubjectRef::new(
                        &win.id,
                        window_field_path(&zone.id, &win.id, "areaM2"),
                        loc(&format!("Window {}", win.id), &format!("Fenster {}", win.id)),
                    ),
                    Quantity::area_m2(win.area_m2),
                    Quantity::area_m2(area_req),
                    loc(
                        &format!("Reduce window area of '{}' from {:.1} m² to ≤ {:.1} m².", win.id, win.area_m2, area_req),
                        &format!("Fensterfläche von '{}' von {:.1} m² auf ≤ {:.1} m² reduzieren.", win.id, win.area_m2, area_req),
                    ),
                ));
            }
        }
        for el in elements.iter().filter(|e| e.zone_id == zone.id && (e.kind == "window" || e.kind == "door") && e.area_m2 > 0.0) {
            let area_req = el.area_m2 * scale;
            if area_req + 1e-12 < el.area_m2 {
                builder = builder.remedy(Remedy::at_most(
                    SubjectRef::new(
                        &el.id,
                        entity_field_path("elements", &el.id, "areaM2"),
                        loc(&format!("Element {}", el.id), &format!("Bauteil {}", el.id)),
                    ),
                    Quantity::area_m2(el.area_m2),
                    Quantity::area_m2(area_req),
                    loc(
                        &format!("Reduce transparent envelope area of '{}' from {:.1} m² to ≤ {:.1} m².", el.id, el.area_m2, area_req),
                        &format!("Transparente Hüllfläche von '{}' von {:.1} m² auf ≤ {:.1} m² reduzieren.", el.id, el.area_m2, area_req),
                    ),
                ));
            }
        }
    }
    builder.build()
}
// #endregion 🔖️Part2
//#endregion 🔖️ComplianceHelpers
