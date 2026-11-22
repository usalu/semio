//! ⏱️ DIN EN 16798-1 NA.4.1 operative temperatures over occupied hours.

#[derive(Clone, Copy, Debug)]
pub(super) struct OccupiedHour {
    pub outdoor_mean_c: f64,
    pub operative_c: f64,
    pub occupied_fraction: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct Deviation {
    pub hours: f64,
    pub degree_hours: f64,
    pub greatest_k: f64,
}

impl Deviation {
    fn record(&mut self, excess_k: f64, occupied_fraction: f64) {
        if excess_k > 0.0 {
            self.hours += occupied_fraction;
            self.degree_hours += excess_k * occupied_fraction;
            self.greatest_k = self.greatest_k.max(excess_k);
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct HourlyComfort {
    pub occupied_hours: f64,
    pub colder: Deviation,
    pub warmer: Deviation,
}

impl HourlyComfort {
    /// 📏️ Return the Category II degree-hour allowance for each direction.
    pub(super) fn category_ii_degree_hour_allowance(&self) -> f64 {
        self.occupied_hours * 0.01 * 2.0
    }
}

/// 🌡️ Accumulate departures from the applicable band in each direction.
pub(super) fn hourly_comfort(hours: impl IntoIterator<Item = OccupiedHour>) -> Option<HourlyComfort> {
    let mut outcome = HourlyComfort::default();
    for hour in hours {
        let OccupiedHour {
            outdoor_mean_c,
            operative_c,
            occupied_fraction,
        } = hour;
        if occupied_fraction == 0.0 {
            continue;
        }
        if !outdoor_mean_c.is_finite()
            || !operative_c.is_finite()
            || !(0.0..=1.0).contains(&occupied_fraction)
        {
            return None;
        }
        let comfort_c = 18.0 + 0.25 * outdoor_mean_c.clamp(16.0, 32.0);
        outcome.occupied_hours += occupied_fraction;
        outcome.colder.record(comfort_c - 2.0 - operative_c, occupied_fraction);
        outcome.warmer.record(operative_c - (comfort_c + 2.0), occupied_fraction);
    }
    (outcome.occupied_hours > 0.0
        && outcome.colder.degree_hours.is_finite()
        && outcome.warmer.degree_hours.is_finite())
    .then_some(outcome)
}
