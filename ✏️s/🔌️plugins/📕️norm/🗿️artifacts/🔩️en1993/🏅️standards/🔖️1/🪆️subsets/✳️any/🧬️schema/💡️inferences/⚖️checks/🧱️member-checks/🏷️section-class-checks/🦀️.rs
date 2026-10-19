//! 🏷️ Section class checks: the section classification and the plastic analysis admission of one member of the full steel structure check. See [`super`].

use super::*;

/// 🏷 Checks the section class of the member and the admission of the plastic analysis.
pub(super) fn check_section_class(
    annex: AnnexChoice,
    report: &mut CheckReport,
    member: &SteelMember,
    section: &SteelSection,
    material: &SteelMaterial,
    class: u8,
) {
    {
        let mut b = CheckResult::assess(
            format!("en1993.5.2.class.{}", member.id),
            "DIN EN 1993-1-1",
            ClauseId::new("EN 1993-1-1", "5", "5.2"),
            subject_member(member, &format!("members[id={}].sectionId", member.id)),
            loc("Cross-section classification", "Querschnittsklassifizierung"),
        )
        .annex(annex)
        .explanation(loc(
            &format!("Section {} is class {} per Table 5.2{}.", section.designation, class, if class == 4 { " — effective A_eff/W_eff per EN 1993-1-5; class 4 is not admitted without effective section redesign" } else { "" }),
            &format!("Querschnitt {} ist Klasse {} nach Tabelle 5.2{}.", section.designation, class, if class == 4 { " — wirksame A_eff/W_eff nach EN 1993-1-5; Klasse 4 ist ohne wirksamen Querschnitt unzulässig" } else { "" }),
        ));
        if class > 3 {
            let a_eff = part_1_1::effective_area_class4(section, material.fy);
            b = b.utilization(dimensionless(section.area), dimensionless(a_eff.max(1e-12)));
            let options = next_section_options(section, section.area, section.w_pl_y);
            b = b.remedy(Remedy::one_of(
                subject_member(member, &format!("members[id={}].sectionId", member.id)),
                options.clone(),
                loc(
                    &format!("Select a class ≤ 3 section; options: {}.", options.join(", ")),
                    &format!("Querschnitt Klasse ≤ 3 wählen; Optionen: {}.", options.join(", ")),
                ),
            ));
        }
        report.push(b.build());
    }

    if member.analysis.eq_ignore_ascii_case("plastic") && class > 1 {
        let mut b = CheckResult::assess(
            format!("en1993.5.2.plastic.{}", member.id),
            "DIN EN 1993-1-1",
            ClauseId::new("EN 1993-1-1", "5", "5.2.2"),
            subject_member(member, &format!("members[id={}].sectionId", member.id)),
            loc("Plastic global analysis class", "Klasse für plastische Schnittgrößenermittlung"),
        )
        .annex(annex)
        .utilization(dimensionless(class as f64), dimensionless(1.0))
        .explanation(loc(
            &format!("Plastic analysis requires class 1; section is class {}.", class),
            &format!("Plastische Schnittgrößenermittlung erfordert Klasse 1; Querschnitt ist Klasse {}.", class),
        ));
        let options = next_section_options(section, section.area, section.w_pl_y);
        b = b.remedy(Remedy::one_of(
            subject_member(member, &format!("members[id={}].sectionId", member.id)),
            options.clone(),
            loc(
                &format!("Select a class-1 section; options: {}.", options.join(", ")),
                &format!("Klasse-1-Querschnitt wählen; Optionen: {}.", options.join(", ")),
            ),
        ));
        report.push(b.build());
    }
}
