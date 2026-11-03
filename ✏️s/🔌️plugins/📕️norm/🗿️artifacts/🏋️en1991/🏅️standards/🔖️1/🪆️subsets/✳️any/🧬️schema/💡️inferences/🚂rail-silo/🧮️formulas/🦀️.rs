//! 🧮️ Closed-form rail and dust formulas of the rail and silo rows: derailment line loads, the characteristic bases of the Table 6.11 load groups and the vent area of a dust explosion. Every constant is read from an unverified registry row, so the covering checks cite it.

use super::checks::part_2;
use semio_s_plugin_norm_compliance::ndp_common as ndp;

/// 🚆 The design situations of a derailment: I (vehicles stay in the track area) and II.
pub const SITUATIONS: [&str; 2] = ["i", "ii"];

/// 🛤️ The load groups of Table 6.11 (EN 1991-2), recalled and unverified.
pub const RAIL_GROUPS: [&str; 14] = ["gr11", "gr12", "gr13", "gr14", "gr15", "gr16", "gr17", "gr21", "gr22", "gr23", "gr24", "gr26", "gr27", "gr31"];

const BAR: f64 = 100_000.0;

/// 🚆 Derailment line load (N/m) of a design situation: the registry value for I, the registry factor times the LM71 distributed load α q_vk for II; `None` for any other situation.
pub fn derail_line_load_n_m(situation: &str, alpha: f64) -> Option<f64> {
    match situation {
        "i" => Some(ndp("ndp.en1991.2.derail-i-line-load")),
        "ii" => Some(ndp("ndp.en1991.2.derail-ii-factor") * part_2::rail_lm71_udl_n_m(alpha)),
        _ => None,
    }
}

/// 🚆 Loaded length (m) of the derailment line load.
pub fn derail_length_m() -> f64 { ndp("ndp.en1991.2.derail-length") }

/// 🚆 Largest distance (m) of the derailment line load from the track axis: the registry multiple of the track gauge.
pub fn derail_offset_limit_m() -> f64 { ndp("ndp.en1991.2.derail-offset-factor") * ndp("ndp.en1991.2.derail-gauge") }

/// 🛤️ Governing longitudinal force (N) of an influence length (m): the larger of the traction force and the braking force, each with its cap.
pub fn rail_longitudinal_n(span_m: f64) -> f64 {
    let traction = (ndp("ndp.en1991.2.rail-traction-line") * span_m).min(ndp("ndp.en1991.2.rail-traction-cap"));
    let braking = (ndp("ndp.en1991.2.rail-braking-line") * span_m).min(ndp("ndp.en1991.2.rail-braking-cap"));
    traction.max(braking)
}

/// 🛤️ Nosing force (N) of the classification factor α.
pub fn rail_nosing_n(alpha: f64) -> f64 { alpha * ndp("ndp.en1991.2.rail-nosing") }

/// 🚐 Axle load (N) of the footbridge service vehicle.
pub fn footbridge_service_axle_n() -> f64 { ndp("ndp.en1991.2.footbridge-service-axle") }

/// 💥 Vent area (m²) of a dust explosion for the reduced explosion pressure p_red: [c1 p_max K_St p_red^e1 + c2 (p_stat − 0.1 bar) p_red^e2] V^e3 with the pressures in bar.
pub fn dust_vent_area_m2(kst: f64, p_max_pa: f64, volume_m3: f64, p_stat_pa: f64, p_red_pa: f64) -> f64 {
    let (p_max, p_stat, p_red) = (p_max_pa / BAR, p_stat_pa / BAR, p_red_pa / BAR);
    let offset = ndp("ndp.en1991.4.dust-vent-pstat-offset") / BAR;
    (ndp("ndp.en1991.4.dust-vent-c1") * p_max * kst * p_red.powf(ndp("ndp.en1991.4.dust-vent-e1")) + ndp("ndp.en1991.4.dust-vent-c2") * (p_stat - offset) * p_red.powf(ndp("ndp.en1991.4.dust-vent-e2"))) * volume_m3.powf(ndp("ndp.en1991.4.dust-vent-e3"))
}

/// 💥 The first input of a dust explosion outside the validity range of the vent area formula: its leaf, the admissible bounds and the label; `p_red` must also reach `p_stat`.
pub fn dust_out_of_domain(kst: f64, p_max_pa: f64, volume_m3: f64, p_stat_pa: f64, p_red_pa: f64) -> Option<(&'static str, (f64, f64), (&'static str, &'static str))> {
    let bounds = |key: &str| (ndp(&format!("ndp.en1991.4.dust-{key}-min")), ndp(&format!("ndp.en1991.4.dust-{key}-max")));
    let rows = [("kst", kst, "kst", ("dust constant K_St", "Staubkennzahl K_St")), ("pMax", p_max_pa, "pmax", ("maximum explosion pressure p_max", "maximaler Explosionsdruck p_max")), ("volume", volume_m3, "volume", ("volume V", "Volumen V")), ("staticOpeningPressure", p_stat_pa, "pstat", ("static opening pressure p_stat", "statischer Ansprechdruck p_stat")), ("designPressure", p_red_pa, "pred", ("reduced explosion pressure p_red", "reduzierter Explosionsdruck p_red"))];
    for (leaf, value, key, label) in rows {
        let (low, high) = bounds(key);
        if !(value.is_finite() && (low..=high).contains(&value)) {
            return Some((leaf, (low, high), label));
        }
    }
    (p_red_pa < p_stat_pa).then(|| ("designPressure", (p_stat_pa, bounds("pred").1), ("reduced explosion pressure p_red (at least p_stat)", "reduzierter Explosionsdruck p_red (mindestens p_stat)")))
}
