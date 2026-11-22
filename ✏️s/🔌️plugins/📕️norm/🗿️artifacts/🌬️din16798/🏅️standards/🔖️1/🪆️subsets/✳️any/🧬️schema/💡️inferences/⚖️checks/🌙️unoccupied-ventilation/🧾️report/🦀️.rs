//! 🧾️ Report the documented unoccupied ventilation regime without substituting the occupied airflow.

use semio_s_artifact_norm_din16798_model::{UnoccupiedVentilationOperation as Operation, ZoneDocument};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, CheckStatus, ClauseId, OccupancyType, Quantity, QuantityKind};
use super::unoccupied_ventilation::{AirflowAssessment, AirflowEvidence, assess_unoccupied_airflow};
use super::{loc, zone_subject};

/// 🌬️ Connects the ventilation evidence to the full and incremental zone assessment.
pub(super) fn check(zone: &ZoneDocument, usage: OccupancyType, annex: AnnexChoice) -> Option<CheckResult> {
    let record = zone.unoccupied_ventilation.as_ref()?;
    let residential = matches!(record.operation, Operation::ResidentialReduced | Operation::ResidentialShutdown);
    let clause = if residential { "B.3.2.5" } else { "B.3.1.5" };
    let target = format!("zones[id={}].unoccupiedVentilation", zone.id);
    let builder = CheckResult::assess(
        format!("din16798-1.vent.unoccupied.{}", zone.id), "DIN EN 16798-1",
        ClauseId::new("EN 16798-1:2019", "Annex B", clause),
        zone_subject(zone, target),
        loc("Ventilation outside occupancy", "Lüftung bei Nichtbelegung"),
    ).annex(annex);
    if annex != AnnexChoice::En {
        return Some(builder.status(CheckStatus::UnverifiedBasis).explanation(loc(
            "Verify the applicable national basis for the unoccupied regime.",
            "Geltende nationale Grundlage für den Betrieb bei Nichtbelegung prüfen.",
        )).build());
    }
    if residential != matches!(usage, OccupancyType::Residential) {
        return Some(builder.status(CheckStatus::UnverifiedBasis).explanation(loc(
            "Confirm the use and assessment scope of the documented operating regime.",
            "Nutzung und Bewertungsbereich der dokumentierten Betriebsweise bestätigen.",
        )).build());
    }
    let evidence = match record.operation {
        Operation::NonResidentialPurge => AirflowEvidence::NonResidentialPurge {
            zone_volume_m3: record.zone_volume_m3,
            minutes_before_occupancy: record.minutes_before_occupancy,
        },
        Operation::NonResidentialReduced => AirflowEvidence::NonResidentialSetback {
            room_area_m2: record.assessed_area_m2,
        },
        Operation::ResidentialReduced => AirflowEvidence::ResidentialSetback {
            dwelling_area_m2: record.assessed_area_m2,
            basis_l_s_m2: Some(record.residential_design_rate_l_per_s_m2),
        },
        Operation::ResidentialShutdown => AirflowEvidence::ResidentialShutdown,
    };
    let supplied = record.outdoor_air_m3_h;
    let builder = match assess_unoccupied_airflow(true, evidence, supplied) {
        AirflowAssessment::Checked { required_m3_h, meets } => builder
            .minimum(Quantity::new(QuantityKind::Dimensionless, supplied / required_m3_h), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .status(if meets { CheckStatus::Pass } else { CheckStatus::Fail })
            .explanation(loc(
                format!("Documented outdoor airflow {supplied:.1} m³/h. Annex B design airflow {required_m3_h:.1} m³/h."),
                format!("Dokumentierter Außenluftvolumenstrom {supplied:.1} m³/h. Auslegungsvolumenstrom nach Anhang B {required_m3_h:.1} m³/h."),
            )),
        AirflowAssessment::MissingPurge => builder.status(CheckStatus::Fail).explanation(loc(
            "Provide one zone air volume within the two hours before occupancy.",
            "In den zwei Stunden vor Belegung einmal das Zonenluftvolumen zuführen.",
        )),
        AirflowAssessment::Unverified => builder.status(CheckStatus::UnverifiedBasis).explanation(loc(
            "Document the operating regime, assessment scope and delivered outdoor air. For a residential setback, confirm the applicable design rate.",
            "Betriebsweise, Bewertungsbereich und zugeführte Außenluft dokumentieren. Für abgesenkte Wohnungslüftung den geltenden Auslegungswert bestätigen.",
        )),
    };
    Some(builder.build())
}
