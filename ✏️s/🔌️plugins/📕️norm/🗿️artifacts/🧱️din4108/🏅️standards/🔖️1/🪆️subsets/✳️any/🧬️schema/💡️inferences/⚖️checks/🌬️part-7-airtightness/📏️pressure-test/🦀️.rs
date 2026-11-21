//! 📏️ Verification of measured airflows under [GModG §26](https://www.gesetze-im-internet.de/geg/__26.html).

#[path = "📇️catalogue/🦀️.rs"]
pub mod catalogue;

/// 🌬️ Normalisation used for the maximum allowable airflow in GModG §26(2) or (3).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AirflowBasis {
    Volume,
    EnvelopeArea { envelope_area_m2: f64 },
}

/// 🏢 Subject of a documented test under GModG §26(1), (4) or (5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestScope {
    WholeBuilding,
    NonResidentialZone {
        non_residential: bool,
        only_selected_zones: bool,
        different_requirements: bool,
    },
    ExteriorUnitSample {
        identical_units: bool,
        exterior_access_only: bool,
        annex_nb_confirmed: bool,
    },
}

/// 📋️ Two measured airflow magnitudes at +50 Pa and −50 Pa and their test basis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PressureTest {
    pub before_completion: bool,
    pub annex_na_confirmed: bool,
    pub conditioned_volume_m3: f64,
    pub mechanical_ventilation: bool,
    pub pressurisation_m3_h: f64,
    pub depressurisation_m3_h: f64,
    pub basis: AirflowBasis,
    pub scope: TestScope,
}

/// 🧾 Outcome of the two pressure-polarity comparisons. The limit is shared by both measurements.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PressureTestResult {
    pub limit_m3_h: f64,
    pub pressurisation_passes: bool,
    pub depressurisation_passes: bool,
}

impl PressureTestResult {
    /// ✅️ Whether both measured airflows satisfy the chosen statutory maximum.
    pub fn eligible(self) -> bool {
        self.pressurisation_passes && self.depressurisation_passes
    }
}

/// 📝 Evidence or input needed before evaluating the statutory limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PressureTestGap {
    MeasurementBasis,
    InvalidTestScope,
    ConditionedVolume,
    MeasuredAirflow,
    IneligibleAreaMethod,
    EnvelopeArea,
    VentilationDeclaration,
    NormalisationBasis,
}

/// 🧮️ Maximum leakage and the separate positive and negative pressure results under GModG §26.
pub fn evaluate_pressure_test(test: PressureTest) -> Result<PressureTestResult, PressureTestGap> {
    if !(test.before_completion && test.annex_na_confirmed) {
        return Err(PressureTestGap::MeasurementBasis);
    }
    let scope_ok = match test.scope {
        TestScope::WholeBuilding => true,
        TestScope::NonResidentialZone { non_residential, only_selected_zones, different_requirements } => {
            non_residential && (only_selected_zones || different_requirements)
        }
        TestScope::ExteriorUnitSample { identical_units, exterior_access_only, annex_nb_confirmed } => {
            identical_units && exterior_access_only && annex_nb_confirmed
        }
    };
    if !scope_ok {
        return Err(PressureTestGap::InvalidTestScope);
    }
    let volume = test.conditioned_volume_m3;
    if !volume.is_finite() || volume <= 0.0 {
        return Err(PressureTestGap::ConditionedVolume);
    }
    if ![test.pressurisation_m3_h, test.depressurisation_m3_h].into_iter().all(|value| value.is_finite() && value >= 0.0) {
        return Err(PressureTestGap::MeasuredAirflow);
    }
    let limit_m3_h = match test.basis {
        AirflowBasis::Volume => volume * super::n50_limit(test.mechanical_ventilation),
        AirflowBasis::EnvelopeArea { envelope_area_m2 } => {
            if volume <= 1500.0 {
                return Err(PressureTestGap::IneligibleAreaMethod);
            }
            if !envelope_area_m2.is_finite() || envelope_area_m2 <= 0.0 {
                return Err(PressureTestGap::EnvelopeArea);
            }
            let limit_per_area = if test.mechanical_ventilation { 2.5 } else { 4.5 };
            envelope_area_m2 * limit_per_area
        }
    };
    if !limit_m3_h.is_finite() {
        return Err(match test.basis {
            AirflowBasis::Volume => PressureTestGap::ConditionedVolume,
            AirflowBasis::EnvelopeArea { .. } => PressureTestGap::EnvelopeArea,
        });
    }
    Ok(PressureTestResult {
        limit_m3_h,
        pressurisation_passes: test.pressurisation_m3_h <= limit_m3_h,
        depressurisation_passes: test.depressurisation_m3_h <= limit_m3_h,
    })
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
