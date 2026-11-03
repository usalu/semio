//! 🌍️ The full seismic check of the EN 1998 inference: the clause-by-clause evaluation across the claimed parts. See [`super`].

use super::*;

#[path = "🏢️building-regularity-and-base-shear-checks/🦀️.rs"]
mod building_regularity_and_base_shear_checks;
use building_regularity_and_base_shear_checks::{check_building_base_shear};

#[path = "🧩️building-details/🦀️.rs"]
mod building_details;
use building_details::{check_building_details, check_capacity_joints, check_site_ground};

use semio_s_artifact_norm_en1998_inferences_structural_system::check_building_systems;

/// 📋️ Full EN 1998 evaluate across claimed parts (scoped; empty lists → NotApplicable).
pub fn check_full_seismic(document: &En1998Snapshot) -> CheckReport {
    check_full_seismic_scoped(document, Scope::All, &mut |_| true)
}

/// 🎯️ The full seismic check restricted to `scope` (`step(fraction)` is polled once per building and returns `false` to cancel): the parts `integrity` (annex gate, site and referential checks), `buildings-na`, `bridges-na`, `assessments-na`, `silos-na`, `tanks-na`, `foundations-na`, `walls-na`, `towers-na` or one row of `buildings`, `bridges`, `assessments`, `silos`, `tanks`, `foundations`, `retainingWalls`, `towers`; the ordered concatenation of the units in [`En1998Units`] is [`check_full_seismic`].
pub fn check_full_seismic_scoped(document: &En1998Snapshot, scope: Scope<'_>, step: &mut dyn FnMut(f32) -> bool) -> CheckReport {
    let mut report = CheckReport::default();
    let annex_params = match resolve_annex(document) {
        Ok(p) => p,
        Err(fail) => {
            if scope.part("integrity") {
                report.push(fail);
            }
            return report;
        }
    };
    let annex = annex_params.choice();
    let (a_g, s, tb, tc, td) = annex_params.ground_params();
    let gamma_i = part_1::gamma_i(document.site.importance_class);

    if scope.part("integrity") {
    if matches!(annex_params, AnnexParams::En { .. }) {
        let annex = AnnexChoice::En;
        let combo = na_de::GroundCombo::from(document.site.de_ground_combo);
        let en_g = document.site.en_ground_type.letter();
        let expected_combo = match en_g {
            'A' => na_de::GroundCombo::AR,
            'B' => na_de::GroundCombo::BR,
            'C' => na_de::GroundCombo::CR,
            'D' | 'E' => na_de::GroundCombo::CS,
            _ => na_de::GroundCombo::BR,
        };
        let combo_ok = combo == expected_combo;
        let mut zc = CheckResult::assess(
            "en1998.site.deNaConsistencyUnderEn",
            "EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "3.2.1"),
            SubjectRef::new("", "site.deGroundCombo", lc("DE ground combo", "DE-Baugrundkombination")),
            lc("DE NA combo consistent with EN site", "DE-NA Kombination konsistent mit EN-Standort"),
        )
        .status(if combo_ok { semio_s_plugin_norm_compliance::CheckStatus::Pass } else { semio_s_plugin_norm_compliance::CheckStatus::Fail })
        .annex(annex)
        .explanation(lc(
            &format!("combo={combo:?} expected={expected_combo:?} for enGround={en_g}"),
            &format!("Kombi={combo:?} erwartet={expected_combo:?} für enGround={en_g}"),
        ));
        if !combo_ok {
            let token = match en_g {
                'A' => "A-R",
                'C' => "C-R",
                'D' | 'E' => "C-S",
                _ => "B-R",
            };
            zc = zc.remedy(Remedy::one_of(
                SubjectRef::new("", "site.deGroundCombo", lc("DE ground combo", "DE-Baugrundkombination")),
                vec![token.to_string()],
                lc(&format!("Align the DE ground combination with the EN ground type: {token}."), &format!("Die DE-Baugrundkombination an die EN-Baugrundklasse anpassen: {token}.")),
            ));
        }
        report.push(zc.build());
    }

    if matches!(annex, AnnexChoice::De) {
        let derived = na_de::a_gr_from_s_ap_r(document.site.s_ap_r);
        if document.site.a_gr > 0.0 && (document.site.a_gr - derived).abs() > 1e-9 {
            report.push(
                CheckResult::assess(
                    "en1998.site.aGr.derived",
                    "DIN EN 1998-1/NA",
                    ClauseId::new("EN 1998-1", "1", "3.2.1"),
                    SubjectRef::new("", "site.aGr", lc("Reference PGA a_gR", "Bezugsspitzenwert a_gR")),
                    lc("DE a_gR derived from plateau acceleration S_aP,R", "DE a_gR aus Plateau-Beschleunigung S_aP,R"),
                )
                .status(semio_s_plugin_norm_compliance::CheckStatus::Warning)
                .annex(annex)
                .explanation(lc(
                    &format!("DE annex derives a_gR={derived:.3} m/s² from S_aP,R={:.3} m/s²; site.aGr={:.3} is ignored for the spectrum.", document.site.s_ap_r, document.site.a_gr),
                    &format!("DE-NA leitet a_gR={derived:.3} m/s² aus S_aP,R={:.3} m/s² ab; site.aGr={:.3} wird für das Spektrum ignoriert.", document.site.s_ap_r, document.site.a_gr),
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

    check_site_ground(&mut report, document, annex);
    push_referential_integrity(&mut report, document, annex);
    }

    let low_seismicity = a_g <= 0.0;

    if document.buildings.is_empty() && scope.part("buildings-na") {
        report.push(
            CheckResult::assess("en1998.1.buildings.na", "DIN EN 1998-1", ClauseId::new("EN 1998-1", "1", "4"), SubjectRef::whole(lc("Buildings", "Hochbauten")), lc("Building seismic checks", "Hochbau-Erdbebennachweise"))
                .not_applicable(lc("No buildings in the subject.", "Keine Hochbauten im Gegenstand."))
                .annex(annex)
                .build(),
        );
    }

    for (index, building) in document.buildings.iter().enumerate().filter(|(index, _)| scope.admits("buildings", *index)) {
        if !step(0.0) {
            report.push(cancelled_check(annex));
            return report;
        }
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
        check_building_details(annex, &mut report, gamma_i, a_g, s, building, document.buildings.get(index + 1), bid, &bpath, &blabel, t1);
        check_capacity_joints(&mut report, annex, building, &document.joints, bid);
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

    evaluate_bridges(&mut report, document, &annex_params, gamma_i, a_g, s, tb, tc, td, scope);
    evaluate_isolation_systems(&mut report, document, &annex_params, gamma_i, a_g, s, tb, tc, td, scope);
    evaluate_pier_bases(&mut report, document, &annex_params, a_g, s, tb, tc, td, scope);
    evaluate_assessments(&mut report, document, &annex_params, gamma_i, a_g, s, tb, tc, td, scope);
    evaluate_silos_tanks(&mut report, document, &annex_params, gamma_i, a_g, s, tb, tc, td, scope);
    evaluate_foundations_walls(&mut report, document, &annex_params, a_g, s, tb, tc, td, gamma_i, scope);
    evaluate_towers(&mut report, document, &annex_params, gamma_i, a_g, s, tb, tc, td, scope);
    report
}


/// 🛑 A cancelled evaluation is incomplete: it must never read as compliant, so it carries one Fail check.
fn cancelled_check(annex: AnnexChoice) -> CheckResult {
    CheckResult::assess(
        "en1998.cancelled",
        "DIN EN 1998-1",
        ClauseId::new("EN 1998-1", "1", "2"),
        SubjectRef::whole(lc("Evaluation", "Auswertung")),
        lc("Evaluation cancelled, the report is incomplete", "Auswertung abgebrochen, der Bericht ist unvollständig"),
    )
    .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
    .annex(annex)
    .explanation(lc("The run was cancelled before every building was checked; no verdict can be derived.", "Der Lauf wurde abgebrochen, bevor alle Gebäude geprüft waren; es ist kein Urteil ableitbar."))
    .remedy(Remedy::one_of(SubjectRef::whole(lc("Evaluation", "Auswertung")), vec!["rerun".into()], lc("Run the evaluation again to completion.", "Auswertung erneut vollständig ausführen.")))
    .build()
}
