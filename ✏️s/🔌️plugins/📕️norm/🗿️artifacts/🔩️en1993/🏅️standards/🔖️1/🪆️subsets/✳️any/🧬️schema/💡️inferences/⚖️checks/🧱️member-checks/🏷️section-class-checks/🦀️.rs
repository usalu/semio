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

    if member.analysis == semio_s_artifact_norm_en1993_model::AnalysisKind::Plastic && class > 1 {
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

/// 🏷️ Hybrid girder (EN 1993-1-1 5.5.2(11)): the flanges keep the member material, the web has its own yield strength. The flange class uses f_y,f and the web class f_y,w (envelope of pure bending and pure compression), the section class is the larger one; the plastic moment adds the flange and web contributions with their own strengths. Informative row; the reduced ε of the stress ratio is not applied.
pub(super) fn check_hybrid_class(annex: AnnexChoice, report: &mut CheckReport, member: &SteelMember, section: &SteelSection, flange: &SteelMaterial, web: &SteelMaterial, params: AnnexParams) {
    use crate::clauses::classification::{classify_i_section, Geometry};
    if !part_1_1::is_i_section(section.kind) || flange.id == web.id {
        return;
    }
    let g = Geometry { h: section.h * 1e3, b: section.b * 1e3, tw: section.tw * 1e3, tf: section.tf * 1e3, r: section.r * 1e3, weld_throat: 0.0 };
    let envelope = |fy_pa: f64| {
        let fy = fy_pa / 1.0e6;
        let (bending, compression) = (classify_i_section(&g, fy, 0.0, -1.0, true), classify_i_section(&g, fy, 1.0e12, 1.0, true));
        (bending.flange.max(compression.flange), bending.web.max(compression.web))
    };
    let (flange_class, _) = envelope(flange.fy);
    let (_, web_class) = envelope(web.fy);
    let class = flange_class.max(web_class);
    let uniform = part_1_1::section_class_rolled_i(section, flange.fy);
    let w_web = section.tw * (section.h - 2.0 * section.tf).powi(2) / 4.0;
    let m_pl = (flange.fy * (section.w_pl_y - w_web) + web.fy * w_web) / params.gamma_m0;
    let m_uniform = flange.fy * section.w_pl_y / params.gamma_m0;
    let explanation = loc(
        &format!("Hybrid girder: flanges {} (f_y,f={:.0} MPa) class {flange_class}, web {} (f_y,w={:.0} MPa) class {web_class}; section class {class} (class {uniform} if the whole section had f_y,f). M_pl,Rd=Σ f_y,i W_pl,i/γM0={:.1} kNm against {:.1} kNm for the flange material throughout (5.5.2(11); reduced ε of the stress ratio not applied).", flange.grade, flange.fy / 1e6, web.grade, web.fy / 1e6, m_pl / 1e3, m_uniform / 1e3),
        &format!("Hybridträger: Gurte {} (f_y,f={:.0} MPa) Klasse {flange_class}, Steg {} (f_y,w={:.0} MPa) Klasse {web_class}; Querschnittsklasse {class} (Klasse {uniform} bei f_y,f im ganzen Querschnitt). M_pl,Rd=Σ f_y,i W_pl,i/γM0={:.1} kNm gegen {:.1} kNm mit dem Gurtwerkstoff im ganzen Querschnitt (5.5.2(11); abgeminderte ε aus dem Spannungsverhältnis nicht angewendet).", flange.grade, flange.fy / 1e6, web.grade, web.fy / 1e6, m_pl / 1e3, m_uniform / 1e3),
    );
    let mut b = CheckResult::assess(
        format!("en1993.1-1.5.5.hybrid.{}", member.id),
        "DIN EN 1993-1-1",
        ClauseId::new("EN 1993-1-1", "5", "5.5.2"),
        subject_member(member, &format!("members[id={}].webMaterialId", member.id)),
        loc("Hybrid girder: classification with the web yield strength", "Hybridträger: Klassifizierung mit der Stegstreckgrenze"),
    )
    .annex(annex)
    .explanation(explanation);
    b = if class > 3 {
        b.utilization(dimensionless(f64::from(class)), dimensionless(3.0)).remedy(Remedy::at_least(
            subject_member(member, &format!("members[id={}].webMaterialId", member.id)),
            stress_pa(web.fy),
            stress_pa(flange.fy),
            loc("Use a web material of the flange strength, or a stockier section, to reach class 3.", "Stegwerkstoff mit der Gurtfestigkeit oder gedrungeneren Querschnitt wählen, um Klasse 3 zu erreichen."),
        ))
    } else {
        b.informative(dimensionless(f64::from(class)))
    };
    report.push(b.build());
}
