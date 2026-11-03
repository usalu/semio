//! 🧱️ EN 1993 member checks: `check_full_steel_structure` verifies every member against EN 1993-1-1 and then runs the special element checks.

use super::*;

#[path = "🏷️section-class-checks/🦀️.rs"]
mod section_class_checks;
use section_class_checks::{check_section_class};

#[path = "🧮️action-resistance-checks/🦀️.rs"]
mod action_resistance_checks;
use action_resistance_checks::{check_uls_actions, check_sls_deflection};

#[path = "📈️member-profile/🦀️.rs"]
mod member_profile;

#[path = "🧾️vid-checks/🦀️.rs"]
mod vid_checks;
pub use vid_checks::{fire_ltb_summary, torsional_summary, FireLtbSummary, TorsionalSummary};

//#region 🔖️ComplianceHelpers
pub fn check_full_steel_structure(document: &En1993Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    check_scoped(document, Scope::default(), true, &mut report);
    report
}

/// 🧱️ Referential integrity, duplicate ids and the empty-document verdict: the first unit of the ordered fold.
pub fn check_integrity(document: &En1993Snapshot) -> Vec<CheckResult> {
    let mut report = CheckReport::default();
    check_scoped(document, Scope::nothing(), true, &mut report);
    report.checks
}

/// 🎯️ Every check whose subject is the row `id` of `collection` (a scope tag of [`Scope`]); the unit of incremental evaluation.
pub fn check_entity(document: &En1993Snapshot, collection: &str, id: &str) -> Vec<CheckResult> {
    let mut report = CheckReport::default();
    check_scoped(document, Scope::row(collection, id), false, &mut report);
    report.checks
}

fn check_scoped(document: &En1993Snapshot, scope: Scope<'_>, integrity: bool, report: &mut CheckReport) {
    let annex = document.annex;
    let mut params = AnnexParams::for_choice(annex);
    if integrity {
        push_referential_integrity(report, document);
    }

    if integrity && document.members.is_empty()
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
        && document.storeys.is_empty()
        && document.restraints.is_empty()
        && document.built_up_members.is_empty()
        && document.weld_groups.is_empty()
        && document.parametric_sections.is_empty()
        && document.hollow_joints.is_empty()
        && document.joint_rows.is_empty()
        && document.frame_nodes.is_empty()
        && document.frame_bars.is_empty()
        && document.joint_plates.is_empty()
        && document.foundation_blocks.is_empty()
        && document.fatigue_stresses.is_empty()
        && document.buckling_lengths.is_empty()
        && document.forming_gains.is_empty()
        && document.fastened_joints.is_empty()
        && document.material_ductility.is_empty()
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
        return;
    }

    for member in document.members.iter().filter(|m| scope.admits("members", &m.id)) {
        let Some(section) = find_section(document, &member.section_id) else { continue };
        let Some(material) = find_material(document, &member.material_id) else { continue };
        let kind = member_kind(member.member_type.as_str());
        let class = part_1_1::section_class_rolled_i(section, material.fy);
        check_section_class(annex, report, member, section, material, class);
        if part_1_1::hollow_route_open(section) {
            report.push(
                CheckResult::assess(
                    format!("en1993.6.3.1.route.{}", member.id),
                    "DIN EN 1993-1-1",
                    ClauseId::new("EN 1993-1-1", "6", "6.3.1"),
                    subject_member(member, &format!("sections[id={}].fabrication", section.id)),
                    loc("Hollow section fabrication route", "Herstellungsart des Hohlprofils"),
                )
                .annex(annex)
                .not_applicable(loc(
                    "The buckling curve of Table 6.2 depends on the fabrication route of the hollow section (hot finished: a, a0 for S460; cold formed: c). Choose it on the section; no curve is assumed, so the buckling and ultimate rows of this member are not evaluated.",
                    "Die Knickspannungslinie nach Tabelle 6.2 hängt von der Herstellungsart des Hohlprofils ab (warmgefertigt: a, a0 bei S460; kaltgefertigt: c). Herstellungsart am Querschnitt wählen; es wird keine Linie angenommen, daher werden die Knick- und Grenzzustandszeilen dieses Bauteils nicht ausgewertet.",
                ))
                .build(),
            );
            continue;
        }
        if !member.web_material_id.is_empty() {
            if let Some(web) = find_material(document, &member.web_material_id) {
                section_class_checks::check_hybrid_class(annex, report, member, section, material, web, AnnexParams::for_choice(annex));
            }
        }
        let a_res = if class == 4 { part_1_1::effective_area_class4(section, material.fy) } else { section.area };
        let w_y = part_1_1::bending_modulus_y(section, class, material.fy);
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
        let class4_factor = (a_res / section.area.max(1e-12)).min(1.0).sqrt();
        let lambda_y = part_1_1::lambda_bar(member.buckling_length_y, iy, material.e_modulus, material.fy) * class4_factor;
        let lambda_z = part_1_1::lambda_bar(member.buckling_length_z, iz, material.e_modulus, material.fy) * class4_factor;
        let chi_y = part_1_1::chi(lambda_y, curve_y);
        let chi_z = part_1_1::chi(lambda_z, curve_z);
        let n_b_rd_y = part_1_1::buckling_resistance_n(a_res, material.fy, chi_y, params);
        let n_b_rd_z = part_1_1::buckling_resistance_n(a_res, material.fy, chi_z, params);
        let n_b_rd = n_b_rd_y.min(n_b_rd_z);
        let m_cr = part_1_1::m_cr_rolled_nm(section, material, member.ltb_length.max(member.ltb_restraint_spacing), member.end_moment_ratio_psi, member.load_application.as_str(), member.moment_diagram.as_str());
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
        check_uls_actions(annex, report, params, member, section, material, &kind, class, a_res, w_y, w_z, iy, iz, curve_y, curve_z, lambda_y, lambda_z, chi_y, chi_z, n_b_rd_y, n_b_rd_z, n_b_rd, m_cr, lambda_lt, curve_lt, chi_lt, m_b_rd, &combinations);
        check_sls_deflection(annex, report, member, section, material, w_y, &combinations);
    }

    check_special_elements(document, annex, params, scope, report);
    vid_checks::check_vids(document, annex, params, scope, report);
}
//#endregion 🔖️ComplianceHelpers
