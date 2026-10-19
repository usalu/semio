//! 🧱️ EN 1993 member checks: `check_full_steel_structure` verifies every member against EN 1993-1-1 and then runs the special element checks.

use super::*;

#[path = "🏷️section-class-checks/🦀️.rs"]
mod section_class_checks;
use section_class_checks::{check_section_class};

#[path = "🧮️action-resistance-checks/🦀️.rs"]
mod action_resistance_checks;
use action_resistance_checks::{check_uls_actions, check_sls_deflection};

//#region 🔖️ComplianceHelpers
pub fn check_full_steel_structure(document: &En1993Snapshot) -> CheckReport {
    let annex = document.annex;
    let mut params = AnnexParams::for_choice(annex);
    let mut report = CheckReport::default();
    push_referential_integrity(&mut report, document);

    if document.members.is_empty()
        && document.joints.is_empty()
        && document.fatigue_details.is_empty()
        && document.fire_exposures.is_empty()
        && document.cold_formed_members.is_empty()
        && document.plated_panels.is_empty()
        && document.silo_shells.is_empty()
        && document.tension_components.is_empty()
        && document.bridge_fatigue.is_empty()
        && document.tower_legs.is_empty()
        && document.piles.is_empty()
        && document.crane_runways.is_empty()
    {
        report.push(
            CheckResult::assess(
                "en1993.empty",
                "DIN EN 1993-1-1",
                ClauseId::new("EN 1993", "1-1", "1"),
                SubjectRef::whole(loc("Steel structure", "Stahltragwerk")),
                loc("Subject presence", "Gegenstand vorhanden"),
            )
            .annex(annex)
            .not_applicable(loc("No members or special elements to check.", "Keine Bauteile oder Sonderelemente zu prüfen."))
            .build(),
        );
        return report;
    }

    for member in &document.members {
        let Some(section) = find_section(document, &member.section_id) else { continue };
        let Some(material) = find_material(document, &member.material_id) else { continue };
        let kind = member_kind(&member.member_type);
        let class = part_1_1::section_class_rolled_i(section, material.fy);
        check_section_class(annex, &mut report, member, section, material, class);
        let a_res = if class == 4 { part_1_1::effective_area_class4(section, material.fy) } else { section.area };
        let w_y = if class == 4 {
            part_1_1::effective_w_el_y_class4(section, material.fy)
        } else if class <= 2 {
            section.w_pl_y
        } else {
            section.w_el_y
        };
        let w_z = if class == 4 {
            section.w_el_z * (a_res / section.area.max(1e-12)).min(1.0)
        } else if class <= 2 {
            section.w_pl_z
        } else {
            section.w_el_z
        };
        let iy = part_1_1::radius_of_gyration(section.iy, section.area);
        let iz = part_1_1::radius_of_gyration(section.iz, section.area);
        let curve_y = part_1_1::buckling_curve_rolled_i(section, 'y', material.fy);
        let curve_z = part_1_1::buckling_curve_rolled_i(section, 'z', material.fy);
        let lambda_y = part_1_1::lambda_bar(member.buckling_length_y, iy, material.e_modulus, material.fy);
        let lambda_z = part_1_1::lambda_bar(member.buckling_length_z, iz, material.e_modulus, material.fy);
        let chi_y = part_1_1::chi(lambda_y, curve_y);
        let chi_z = part_1_1::chi(lambda_z, curve_z);
        let n_b_rd_y = part_1_1::buckling_resistance_n(a_res, material.fy, chi_y, params);
        let n_b_rd_z = part_1_1::buckling_resistance_n(a_res, material.fy, chi_z, params);
        let n_b_rd = n_b_rd_y.min(n_b_rd_z);
        let m_cr = part_1_1::m_cr_rolled_nm(section, material, member.ltb_length.max(member.ltb_restraint_spacing), member.end_moment_ratio_psi, &member.load_application, &member.moment_diagram);
        let lambda_lt = part_1_1::lambda_lt(w_y, material.fy, m_cr);
        let curve_lt = part_1_1::ltb_curve_table_6_4(section);
        let chi_lt = part_1_1::chi_lt(lambda_lt, curve_lt);
        let m_b_rd = part_1_1::ltb_resistance_nm(w_y, material.fy, chi_lt, params);
        let combinations = combine_member_actions(document, &member.id);
        if governing_effects(&combinations, "uls").is_none() {
            report.push(
                CheckResult::assess(
                    format!("en1993.6.2.uls.{}", member.id),
                    "DIN EN 1993-1-1",
                    ClauseId::new("EN 1993-1-1", "6", "6.2"),
                    subject_member(member, &format!("members[id={}].id", member.id)),
                    loc("ULS action effects", "Einwirkungen im GZT"),
                )
                .annex(annex)
                .not_applicable(loc(
                    "No characteristic actions to combine for this member.",
                    "Keine charakteristischen Einwirkungen zum Kombinieren für dieses Bauteil.",
                ))
                .build(),
            );
        }
        check_uls_actions(annex, &mut report, params, member, section, material, &kind, class, a_res, w_y, w_z, iy, iz, curve_y, curve_z, lambda_y, lambda_z, chi_y, chi_z, n_b_rd_y, n_b_rd_z, n_b_rd, m_cr, lambda_lt, curve_lt, chi_lt, m_b_rd, &combinations);
        check_sls_deflection(annex, &mut report, member, section, material, w_y, &combinations);
    }

    check_special_elements(document, annex, params, &mut report);

    report
}
//#endregion 🔖️ComplianceHelpers
