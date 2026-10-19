//! 🌍️ The full seismic check of the EN 1998 inference: the clause-by-clause evaluation across the claimed parts. See [`super`].

use super::*;

#[path = "🏢️building-regularity-and-base-shear-checks/🦀️.rs"]
mod building_regularity_and_base_shear_checks;
use building_regularity_and_base_shear_checks::{check_building_base_shear};

#[path = "🧱️structural-system-checks/🦀️.rs"]
mod structural_system_checks;
use structural_system_checks::{check_building_systems};

/// 📋️ Full EN 1998 evaluate across claimed parts (scoped; empty lists → NotApplicable).
pub fn check_full_seismic(document: &En1998Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    let annex_params = match resolve_annex(document) {
        Ok(p) => p,
        Err(fail) => {
            report.push(fail);
            return report;
        }
    };
    let annex = annex_params.choice();
    let (a_g, s, tb, tc, td) = annex_params.ground_params();
    let gamma_i = part_1::gamma_i(&document.site.importance_class);

    if matches!(annex_params, AnnexParams::En { .. }) {
        let annex = AnnexChoice::En;
        let zone = na_de::SeismicZone::from(document.site.seismic_zone);
        let combo = na_de::GroundCombo::from(document.site.de_ground_combo);
        let zone_agr = zone.a_gr();
        let zone_ok = (zone_agr - document.site.a_gr).abs() <= 1e-6 || document.site.a_gr <= 0.0;
        let en_g = document.site.en_ground_type.chars().next().unwrap_or('?').to_ascii_uppercase();
        let expected_combo = match en_g {
            'A' => na_de::GroundCombo::AR,
            'B' => na_de::GroundCombo::BR,
            'C' => na_de::GroundCombo::CR,
            'D' | 'E' => na_de::GroundCombo::CS,
            _ => na_de::GroundCombo::BR,
        };
        let combo_ok = combo == expected_combo || document.site.en_ground_type.is_empty();
        let mut zc = CheckResult::assess(
            "en1998.site.deNaConsistencyUnderEn",
            "EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "3.2.1"),
            SubjectRef::new("", "site.seismicZone", lc("Seismic zone", "Erdbebenzone")),
            lc("DE NA zone/combo consistent with EN site", "DE-NA Zone/Kombination konsistent mit EN-Standort"),
        )
        .utilization(q_dim(if zone_ok && combo_ok { 0.5 } else { 1.5 }), q_dim(1.0))
        .annex(annex)
        .explanation(lc(
            &format!("zone a_gR={zone_agr:.3} vs site a_gR={:.3}; combo={combo:?} expected={expected_combo:?} for enGround={en_g}", document.site.a_gr),
            &format!("Zone a_gR={zone_agr:.3} gegen Standort a_gR={:.3}; Kombi={combo:?} erwartet={expected_combo:?} für enGround={en_g}", document.site.a_gr),
        ));
        if !zone_ok {
            zc = zc.remedy(Remedy::exactly(
                SubjectRef::new("", "site.seismicZone", lc("Seismic zone", "Erdbebenzone")),
                q_dim(0.0),
                q_dim(1.0),
                lc("Align seismicZone with site a_gR under EN annex.", "seismicZone unter EN-Anhang an Standort-a_gR anpassen."),
            ));
        }
        if !combo_ok {
            zc = zc.remedy(Remedy::exactly(
                SubjectRef::new("", "site.deGroundCombo", lc("DE ground combo", "DE-Baugrundkombination")),
                q_dim(0.0),
                q_dim(1.0),
                lc("Align deGroundCombo letter with enGroundType under EN annex.", "deGroundCombo-Buchstaben unter EN-Anhang an enGroundType anpassen."),
            ));
        }
        report.push(zc.build());
    }

    if matches!(annex, AnnexChoice::De) {
        let derived = document.site.seismic_zone.a_gr();
        if (document.site.a_gr - derived).abs() > 1e-9 {
            report.push(
                CheckResult::assess(
                    "en1998.site.aGr.derived",
                    "DIN EN 1998-1/NA",
                    ClauseId::new("EN 1998-1", "1", "3.2.1"),
                    SubjectRef::new("", "site.aGr", lc("Reference PGA a_gR", "Bezugsspitzenwert a_gR")),
                    lc("DE a_gR derived from seismic zone", "DE a_gR aus Erdbebenzone"),
                )
                .status(semio_s_plugin_norm_compliance::CheckStatus::Warning)
                .annex(annex)
                .explanation(lc(
                    &format!("DE annex derives a_gR={derived} m/s² from zone; site.aGr={:.3} is ignored for spectrum.", document.site.a_gr),
                    &format!("DE-NA leitet a_gR={derived} m/s² aus der Zone ab; site.aGr={:.3} wird für das Spektrum ignoriert.", document.site.a_gr),
                ))
                .remedy(Remedy::exactly(
                    SubjectRef::new("", "site.aGr", lc("Reference PGA a_gR", "Bezugsspitzenwert a_gR")),
                    q_acc(document.site.a_gr),
                    q_acc(derived),
                    lc("Set a_gR to the zone Table NA.1 value (or switch annex to EN).", "a_gR auf Tab. NA.1-Zonenwert setzen (oder Anhang EN wählen)."),
                ))
                .build(),
            );
        }
    }

    push_referential_integrity(&mut report, document, annex);

    let low_seismicity = matches!(annex_params, AnnexParams::De { zone: na_de::SeismicZone::Zone0, .. }) || a_g <= 0.0;

    if document.buildings.is_empty() {
        report.push(
            CheckResult::assess("en1998.1.buildings.na", "DIN EN 1998-1", ClauseId::new("EN 1998-1", "1", "4"), SubjectRef::whole(lc("Buildings", "Hochbauten")), lc("Building seismic checks", "Hochbau-Erdbebennachweise"))
                .not_applicable(lc("No buildings in the subject.", "Keine Hochbauten im Gegenstand."))
                .annex(annex)
                .build(),
        );
    }

    for building in &document.buildings {
        let bid = &building.id;
        let bpath = id_sel("buildings", bid);
        let blabel = lc(&building.name, &building.name);
        if low_seismicity {
            report.push(
                CheckResult::assess(format!("en1998.1.{bid}.zone0"), "DIN EN 1998-1/NA", ClauseId::new("EN 1998-1", "1", "3.2.1"), SubjectRef::new(bid, format!("{bpath}.id"), blabel.clone()), lc("Very low seismicity exemption", "Befreiung bei sehr geringer Seismizität"))
                    .not_applicable(lc("DE zone 0 / a_gR = 0: EN 1998 provisions need not be applied.", "DE-Zone 0 / a_gR = 0: EN-1998-Regeln brauchen nicht angewendet zu werden."))
                    .annex(annex)
                    .build(),
            );
            continue;
        }
        let (masses, heights, total_mass, t1) = check_building_base_shear(annex, &mut report, s, tc, building, bid, &bpath, &blabel);
        let systems = if building.systems.is_empty() {
            report.push(
                CheckResult::assess(format!("en1998.1.{bid}.system.na"), "DIN EN 1998-1", ClauseId::new("EN 1998-1", "1", "5"), SubjectRef::new(bid, format!("{bpath}.systems"), blabel.clone()), lc("Structural system", "Aussteifungssystem"))
                    .not_applicable(lc("No structural systems defined.", "Keine Aussteifungssysteme definiert."))
                    .annex(annex)
                    .build(),
            );
            continue;
        } else {
            &building.systems
        };
        check_building_systems(document, annex, &mut report, gamma_i, a_g, s, tb, tc, td, building, bid, &bpath, &blabel, &masses, &heights, total_mass, t1, systems);
    }

    evaluate_bridges(&mut report, document, &annex_params, gamma_i, a_g, s, tb, tc, td);
    evaluate_assessments(&mut report, document, &annex_params, gamma_i, a_g, s, tb, tc, td);
    evaluate_silos_tanks(&mut report, document, &annex_params, gamma_i, a_g, s, tb, tc, td);
    evaluate_foundations_walls(&mut report, document, &annex_params, a_g, s, tb, tc, td, gamma_i);
    evaluate_towers(&mut report, document, &annex_params, gamma_i, a_g, s, tb, tc, td);
    report
}
