//! 🛖️ Pressure coefficients of the zones F to N of a hipped roof (EN 1991-1-4 Figure 7.9, Table 7.5): the cells are unverified registry rows that stay unset until the table is supplied, so the check reports "awaiting values"
//! instead of a coefficient; once both cells of a zone are keyed the entered coefficient is compared with the registered one like every other zone.

use super::*;
use semio_s_artifact_norm_en1991_model::{RoofSurface, WindFace};
use semio_s_plugin_norm_compliance::{ndp_number_opt, AnnexChoice, CheckStatus};

/// 🛖️ The check of one coefficient (`cpe10` or `cpe1`) of a wind face in a hipped-roof zone: awaiting values while the registry cell is unset, the deviation from the registered cell otherwise.
pub(super) fn cpe_check(annex: AnnexChoice, face: &WindFace, suffix: &str, user: f64, label: &str, leaf: &str) -> CheckResult {
    let id = part_1_4::hipped_cpe_ndp_id(suffix, &face.zone);
    let path = format!("windFaces[id={}].{leaf}", face.id);
    let subject = || SubjectRef::new(&face.id, &path, copy(&format!("Face {}", face.id), &format!("Fläche {}", face.id)));
    let base = CheckResult::assess(format!("en1991.1-4.{suffix}.{}", face.id), "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "Figure 7.9 / Table 7.5", "7.2"), subject(), copy(&format!("External pressure coefficient {label} of the hipped roof"), &format!("Äußerer Druckbeiwert {label} des Walmdachs"))).annex(annex);
    match ndp_number_opt(&id, annex) {
        None => base
            .status(CheckStatus::UnverifiedBasis)
            .explanation(copy(&format!("Awaiting values: {label} of zone {} on a hipped roof is read from Table 7.5, which is not available; the registry cell {id} is unverified and unset. Entered {user:.3} is not compared. Zone layout unverified (reconstruction of Figure 7.9).", face.zone), &format!("Werte ausstehend: {label} der Zone {} eines Walmdachs stammt aus Tabelle 7.5, die nicht vorliegt; die Registerzelle {id} ist ungeprüft und nicht gesetzt. Der eingegebene Wert {user:.3} wird nicht verglichen. Zonenlage ungeprüft (Rekonstruktion von Bild 7.9).", face.zone)))
            .ndp(&id)
            .ndp_reads()
            .build(),
        Some(table) => {
            let deviation = (user - table).abs();
            let quantity = |value: f64| Quantity::new(QuantityKind::Dimensionless, value);
            let mut check = base
                .utilization(quantity(deviation), quantity(CPE_TOLERANCE))
                .warn_near_limit()
                .explanation(copy(&format!("Entered {label}={user:.3}; registered {table:.3} for zone {} of the hipped roof (deviation {deviation:.3}, tolerance {CPE_TOLERANCE}). Zone layout unverified.", face.zone), &format!("Eingegebenes {label}={user:.3}; registriert {table:.3} für Zone {} des Walmdachs (Abweichung {deviation:.3}, Toleranz {CPE_TOLERANCE}). Zonenlage ungeprüft.", face.zone)))
                .ndp(&id);
            if deviation > CPE_TOLERANCE {
                check = check.remedy(Remedy::exactly(subject(), quantity(user), quantity(table), copy(&format!("Set {label} to the registered {table:.3} for zone {}.", face.zone), &format!("{label} auf den registrierten Wert {table:.3} der Zone {} setzen.", face.zone))));
            }
            check.ndp_reads().build()
        }
    }
}

/// 🛖️ The coefficient check of a hipped roof surface: its wind face's c_pe,10 against the registered cell of the zone, or awaiting values while that cell is unset.
pub(super) fn surface_cpe(annex: AnnexChoice, surface: &RoofSurface, face: &WindFace) -> CheckResult {
    let id = part_1_4::hipped_cpe_ndp_id("cpe10", &face.zone);
    let subject = SubjectRef::new(&surface.id, &format!("roofSurfaces[id={}].pitchDeg", surface.id), copy(&format!("Roof surface {}", surface.id), &format!("Dachfläche {}", surface.id)));
    let base = CheckResult::assess(format!("en1991.1-4.roof-surface-cpe.{}", surface.id), "DIN EN 1991-1-4", ClauseId::new("EN 1991-1-4", "Figure 7.9 / Table 7.5", "7.2"), subject, copy("Pressure coefficient of the hipped roof surface", "Druckbeiwert der Walmdachfläche")).annex(annex);
    match ndp_number_opt(&id, annex) {
        None => base
            .status(CheckStatus::UnverifiedBasis)
            .explanation(copy(&format!("Awaiting values: zone {} of the hipped roof surface {} is read from Table 7.5, which is not available; the registry cell {id} is unverified and unset. Face {} has {:.3}, not compared. Zone layout unverified.", face.zone, surface.id, face.id, face.c_pe10), &format!("Werte ausstehend: Zone {} der Walmdachfläche {} stammt aus Tabelle 7.5, die nicht vorliegt; die Registerzelle {id} ist ungeprüft und nicht gesetzt. Fläche {} hat {:.3}, nicht verglichen. Zonenlage ungeprüft.", face.zone, surface.id, face.id, face.c_pe10)))
            .ndp(&id)
            .ndp_reads()
            .build(),
        Some(table) => {
            let deviation = (face.c_pe10 - table).abs();
            let quantity = |value: f64| Quantity::new(QuantityKind::Dimensionless, value);
            let mut compared = base.utilization(quantity(deviation), quantity(CPE_TOLERANCE))
                .warn_near_limit()
                .explanation(copy(&format!("Hipped roof surface {} in zone {}: registered c_pe,10 {table:.3}, face {} has {:.3} (deviation {deviation:.3}).", surface.id, face.zone, face.id, face.c_pe10), &format!("Walmdachfläche {} in Zone {}: registrierter Wert c_pe,10 {table:.3}, Fläche {} hat {:.3} (Abweichung {deviation:.3}).", surface.id, face.zone, face.id, face.c_pe10)))
                .ndp(&id)
                .ndp_reads();
            if deviation / CPE_TOLERANCE > semio_s_plugin_norm_compliance::near_limit().threshold() {
                compared = compared.remedy(Remedy::exactly(SubjectRef::new(&face.id, &format!("windFaces[id={}].cPe10", face.id), copy(&format!("Face {}", face.id), &format!("Fläche {}", face.id))), quantity(face.c_pe10), quantity(table), copy(&format!("Set c_pe,10 of face {} to the registered {table:.3} of Table 7.5 for zone {}.", face.id, face.zone), &format!("c_pe,10 der Fläche {} auf den registrierten Wert {table:.3} der Tabelle 7.5 für Zone {} setzen.", face.id, face.zone))));
            }
            compared.build()
        }
    }
}
