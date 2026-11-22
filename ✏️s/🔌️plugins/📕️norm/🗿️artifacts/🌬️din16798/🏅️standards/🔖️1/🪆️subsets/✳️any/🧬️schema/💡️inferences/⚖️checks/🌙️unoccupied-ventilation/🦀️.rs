//! 🌙️ EN 16798-1 Annex B airflow during an unoccupied period.

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum AirflowEvidence {
    NonResidentialPurge {
        zone_volume_m3: f64,
        minutes_before_occupancy: f64,
    },
    NonResidentialSetback {
        room_area_m2: f64,
    },
    ResidentialSetback {
        dwelling_area_m2: f64,
        basis_l_s_m2: Option<f64>,
    },
    ResidentialShutdown,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum AirflowAssessment {
    Checked { required_m3_h: f64, meets: bool },
    MissingPurge,
    Unverified,
}

/// 🌬️ Assess one documented zone flush, room setback or dwelling setback.
pub(super) fn assess_unoccupied_airflow(
    annex_b: bool,
    evidence: AirflowEvidence,
    delivered_m3_h: f64,
) -> AirflowAssessment {
    if !annex_b || !delivered_m3_h.is_finite() || delivered_m3_h < 0.0 {
        return AirflowAssessment::Unverified;
    }
    let required_m3_h = match evidence {
        AirflowEvidence::NonResidentialPurge {
            zone_volume_m3,
            minutes_before_occupancy,
        } if positive(zone_volume_m3) && positive(minutes_before_occupancy) => {
            zone_volume_m3 / (minutes_before_occupancy.min(120.0) / 60.0)
        }
        AirflowEvidence::NonResidentialPurge {
            zone_volume_m3,
            minutes_before_occupancy: 0.0,
        } if positive(zone_volume_m3) => return AirflowAssessment::MissingPurge,
        AirflowEvidence::NonResidentialSetback { room_area_m2 } if positive(room_area_m2) => {
            room_area_m2 * 0.54
        }
        AirflowEvidence::ResidentialSetback {
            dwelling_area_m2,
            basis_l_s_m2: Some(basis),
        } if positive(dwelling_area_m2) && (0.10..=0.15).contains(&basis) => {
            dwelling_area_m2 * basis * 3.6
        }
        _ => return AirflowAssessment::Unverified,
    };
    if !positive(required_m3_h) {
        return AirflowAssessment::Unverified;
    }
    let roundoff = required_m3_h * (4.0 * f64::EPSILON);
    AirflowAssessment::Checked {
        required_m3_h,
        meets: delivered_m3_h >= required_m3_h - roundoff,
    }
}

fn positive(value: f64) -> bool {
    value.is_finite() && value > 0.0
}
