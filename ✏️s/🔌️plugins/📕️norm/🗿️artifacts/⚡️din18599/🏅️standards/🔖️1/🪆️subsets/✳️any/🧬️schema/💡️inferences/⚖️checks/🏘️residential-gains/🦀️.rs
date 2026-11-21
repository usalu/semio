//! 🏘️ Residential internal heat gains from DIN V 18599-10:2018-09 § 5, Table 4.

use super::{BuildingCategory, CalculationMethod, Din18599Snapshot, ThermalZone, UsageProfile};

pub const RESIDENTIAL_GAINS_STANDARD: &str = "DIN V 18599-10:2018-09";

/// 🏠️ Table 4 daily internal gains per square metre of net floor area for a residential building in the monthly method.
pub fn residential_daily_gain_wh_m2(doc: &Din18599Snapshot) -> Option<f64> {
    if doc.building_category != BuildingCategory::Residential || doc.method != CalculationMethod::DetailedMonthly {
        return None;
    }
    match doc.dhw.dwelling_count {
        0 => None,
        1..=2 => Some(45.0),
        _ => Some(90.0),
    }
}

/// 🌡️ Table 4 rate for residential zones, or the entered rate where the residential standard profile is absent.
pub fn internal_gains_w_m2(doc: &Din18599Snapshot, zone: &ThermalZone) -> f64 {
    if zone.usage_profile == UsageProfile::WFH {
        if let Some(daily) = residential_daily_gain_wh_m2(doc) {
            return daily / 24.0;
        }
    }
    zone.internal_gains_w_m2
}
