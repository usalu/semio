//! 📐 DIN 4108-3:2024-03 prescriptive floor, wall, roof and attic ceiling constructions.

#[path = "🌬️ventilated-roof/🦀️.rs"]
pub mod ventilated_roof;

/// 🧱 Room-side ground slab recommendation in §5.3.3.3 after the construction prerequisites are established.
pub fn internal_slab_sd_recommendation(sd_m: f64) -> Option<bool> {
    (sd_m.is_finite() && sd_m >= 0.0).then_some(sd_m >= 20.0)
}

/// 🏠 Entered values for the ventilated attic ceiling construction of Figure 17.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AtticCeiling {
    pub attic_area_m2: f64,
    pub ventilation_openings_m2: f64,
    pub upper_sd_m: f64,
    pub insulation_sd_m: f64,
    pub airtight_layer_sd_m: f64,
    pub room_side_resistance_fraction: f64,
}

/// 📏 Figure 17 limits and the outcome for the entered attic ceiling.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AtticCeilingLimits {
    pub minimum_openings_m2: f64,
    pub maximum_upper_sd_m: f64,
    pub meets_limits: bool,
}

/// 🧮 DIN 4108-3 §5.3.5 and Figure 17, for an attic with the required outdoor openings.
pub fn attic_ceiling_limits(ceiling: AtticCeiling) -> Option<AtticCeilingLimits> {
    if ceiling.attic_area_m2 <= 0.0
        || ![
            ceiling.attic_area_m2, ceiling.ventilation_openings_m2, ceiling.upper_sd_m,
            ceiling.insulation_sd_m, ceiling.airtight_layer_sd_m, ceiling.room_side_resistance_fraction,
        ].into_iter().all(|value| value.is_finite() && value >= 0.0)
        || ceiling.room_side_resistance_fraction > 1.0
    {
        return None;
    }
    let minimum_openings_m2 = ceiling.attic_area_m2 * 0.002;
    let maximum_upper_sd_m = if ceiling.insulation_sd_m >= 1500.0 { 25.0 } else { 2.0 };
    Some(AtticCeilingLimits {
        minimum_openings_m2,
        maximum_upper_sd_m,
        meets_limits: ceiling.ventilation_openings_m2 >= minimum_openings_m2
            && ceiling.upper_sd_m <= maximum_upper_sd_m
            && ceiling.airtight_layer_sd_m >= 10.0
            && ceiling.room_side_resistance_fraction <= 0.20,
    })
}


/// 🧱 The §5.3.3.2 limit on resistance on the room side of a perimeter-insulated slab's waterproofing.
pub fn perimeter_slab_limit(inboard_fraction: f64) -> Option<bool> {
    (0.0..=1.0).contains(&inboard_fraction).then_some(inboard_fraction <= 0.20)
}

/// 🧱 The §5.3.2.2 diffusion limit for walls with room-side insulation and verified rain protection.
pub fn interior_wall_limit(insulation_resistance: f64, inboard_sd_m: f64) -> Option<bool> {
    if !insulation_resistance.is_finite() || insulation_resistance <= 0.0 {
        return None;
    }
    if insulation_resistance <= 0.5 || insulation_resistance > 1.0 {
        return Some(insulation_resistance <= 0.5);
    }
    (inboard_sd_m.is_finite() && inboard_sd_m >= 0.0).then_some(inboard_sd_m >= 0.5)
}

/// 🏠 The minimum room-side diffusion resistance in Table 3 for Figure 4 and 5 roofs.
pub fn unventilated_roof_minimum_sd(outboard_sd_m: f64) -> Option<f64> {
    if !(0.0..=2.0).contains(&outboard_sd_m) {
        return None;
    }
    Some(if outboard_sd_m <= 0.1 { 1.0 } else if outboard_sd_m <= 0.3 { 2.0 } else { 6.0 * outboard_sd_m })
}

/// 🏠 Table 3 and §5.3.4.2.1 limits for Figure 4 and 5 roofs.
pub fn unventilated_roof_limit(outboard_sd_m: f64, inboard_sd_m: f64, inboard_fraction: f64) -> Option<bool> {
    if !outboard_sd_m.is_finite() || outboard_sd_m < 0.0
        || !inboard_sd_m.is_finite() || inboard_sd_m < 0.0
        || !(0.0..=1.0).contains(&inboard_fraction)
    {
        return None;
    }
    Some(inboard_fraction <= 0.20 && unventilated_roof_minimum_sd(outboard_sd_m)
        .is_some_and(|minimum| inboard_sd_m >= minimum))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction_thresholds_match_neutral_fixtures() {
        let cases: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/📐annex-a-constructions/🔣️.json")).unwrap();
        for case in cases["attic"].as_array().unwrap() {
            let [area, openings, upper, insulation, air_seal, fraction] = serde_json::from_value::<[f64; 6]>(case["inputs"].clone()).unwrap();
            let outcome = attic_ceiling_limits(AtticCeiling {
                attic_area_m2: area,
                ventilation_openings_m2: openings,
                upper_sd_m: upper,
                insulation_sd_m: insulation,
                airtight_layer_sd_m: air_seal,
                room_side_resistance_fraction: fraction,
            }).map(|limits| limits.meets_limits);
            assert_eq!(outcome, case["outcome"].as_bool(), "{case}");
        }
        for case in cases["perimeter"].as_array().unwrap() {
            assert_eq!(perimeter_slab_limit(case[0].as_f64().unwrap()), case[1].as_bool(), "{case}");
        }
        for case in cases["wall"].as_array().unwrap() {
            let sd = case[1].as_f64().unwrap_or(f64::NAN);
            assert_eq!(interior_wall_limit(case[0].as_f64().unwrap(), sd), case[2].as_bool(), "{case}");
        }
        for case in cases["roof_table3"].as_array().unwrap() {
            let [outboard, inboard, fraction] = serde_json::from_value::<[f64; 3]>(case["inputs"].clone()).unwrap();
            assert_eq!(unventilated_roof_limit(outboard, inboard, fraction), case["outcome"].as_bool(), "{case}");
        }
        for row in cases["slab"].as_array().unwrap() {
            assert_eq!(internal_slab_sd_recommendation(row[0].as_f64().unwrap()), row[1].as_bool(), "{row}");
        }
    }
}
