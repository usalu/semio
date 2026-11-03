//! 🔗️ Ductility and masonry checks: the material specific ductility detailing and the simple masonry wall ratio of one structural system. See [`super`].

use super::*;

/// 🔗 Checks the material specific ductility detailing of every member.
pub fn check_member_ductility(
    annex: AnnexChoice,
    report: &mut CheckReport,
    building: &semio_s_artifact_norm_en1998_model::En1998Building,
    bid: &str,
    bpath: &str,
    system: &semio_s_artifact_norm_en1998_model::En1998System,
    q: f64,
) {
    let gov_dc = system.ductility_class;
    for member in &building.members {
        let mpath = format!("{}.members[id={}]", bpath, member.id);
        let mat = member.material;
        if mat == semio_s_artifact_norm_en1998_model::StructuralMaterial::Rc {
            let min_b = part_1::rc_column_min_b_m(gov_dc);
            if member.role == semio_s_artifact_norm_en1998_model::MemberKind::Column {
                let ok = member.min_dimension_m + 1e-12 >= min_b;
                let mut c = CheckResult::assess(
                    format!("en1998.1.{bid}.{}.member.{}.rc.minB", system.id, member.id),
                    "DIN EN 1998-1",
                    ClauseId::new("EN 1998-1", "1", "5.4.3.2.1"),
                    SubjectRef::new(bid, format!("{mpath}.minDimensionM"), lc(&member.id, &member.id)),
                    lc("RC column minimum dimension", "Stahlbetonstütze Mindestabmessung"),
                )
                .minimum(q_len(member.min_dimension_m), q_len(min_b))
                .annex(annex)
                .explanation(lc(&format!("b={:.3} m, required≥{min_b:.3} m", member.min_dimension_m), &format!("b={:.3} m, erforderlich≥{min_b:.3} m", member.min_dimension_m)));
                if !ok {
                    c = c.remedy(Remedy::at_least(
                        SubjectRef::new(bid, format!("{mpath}.minDimensionM"), lc("Min dimension", "Mindestabmessung")),
                        q_len(member.min_dimension_m),
                        q_len(min_b),
                        lc(&format!("Increase column min dimension to {min_b:.3} m.",), &format!("Stützenmindestabmessung auf {min_b:.3} m erhöhen.")),
                    ));
                }
                report.push(c.build());
            }
            
            {
                let mut c = CheckResult::assess(
                    format!("en1998.1.{bid}.{}.member.{}.rc.rhoPrimeAbs", system.id, member.id),
                    "DIN EN 1998-1",
                    ClauseId::new("EN 1998-1", "1", "5.4.3"),
                    SubjectRef::new(bid, format!("{mpath}.rhoPrime"), lc(&member.id, &member.id)),
                    lc("RC reinforcement ρ′", "Stahlbetonbewehrung ρ′"),
                )
                .utilization(q_dim(member.rho_prime), q_dim(member.rho.max(1e-6)))
                .annex(annex)
                .explanation(lc(
                    &format!("ρ′={:.4}, ρ={:.4}", member.rho_prime, member.rho),
                    &format!("ρ′={:.4}, ρ={:.4} (Bewehrung)", member.rho_prime, member.rho),
                ));
                if member.rho_prime > member.rho + 1e-12 {
                    c = c.remedy(Remedy::at_most(
                        SubjectRef::new(bid, format!("{mpath}.rhoPrime"), lc("ρ′", "ρ′")),
                        q_dim(member.rho_prime),
                        q_dim(member.rho),
                        lc("Reduce ρ′ to at most ρ.", "ρ′ auf höchstens ρ senken."),
                    ));
                }
                report.push(c.build());
            }
if member.role == semio_s_artifact_norm_en1998_model::MemberKind::Beam {
                let min_b = part_1::rc_beam_min_b_m(gov_dc);
                let ok = member.min_dimension_m + 1e-12 >= min_b;
                let mut c = CheckResult::assess(
                    format!("en1998.1.{bid}.{}.member.{}.rc.beamMinB", system.id, member.id),
                    "DIN EN 1998-1",
                    ClauseId::new("EN 1998-1", "1", "5.4.1.2.1"),
                    SubjectRef::new(bid, format!("{mpath}.minDimensionM"), lc(&member.id, &member.id)),
                    lc("RC beam minimum width", "Stahlbetonbalken Mindestbreite"),
                )
                .minimum(q_len(member.min_dimension_m), q_len(min_b))
                .annex(annex)
                .explanation(lc(
                    &format!("b={:.3} m, required≥{min_b:.3} m", member.min_dimension_m),
                    &format!("b={:.3} m, erforderlich≥{min_b:.3} m", member.min_dimension_m),
                ));
                if !ok {
                    c = c.remedy(Remedy::at_least(
                        SubjectRef::new(bid, format!("{mpath}.minDimensionM"), lc("Min dimension", "Mindestabmessung")),
                        q_len(member.min_dimension_m),
                        q_len(min_b),
                        lc(&format!("Increase beam min width to {min_b:.3} m."), &format!("Balkenmindestbreite auf {min_b:.3} m erhöhen.")),
                    ));
                }
                report.push(c.build());
            }
            let (rho_min, rho_max) = part_1::rc_rho_limits(member.role.as_str(), gov_dc);
            let ok_rho = member.rho + 1e-12 >= rho_min && member.rho <= rho_max + 1e-12;
            let mut c = CheckResult::assess(
                format!("en1998.1.{bid}.{}.member.{}.rc.rho", system.id, member.id),
                "DIN EN 1998-1",
                ClauseId::new("EN 1998-1", "1", "5.4.3"),
                SubjectRef::new(bid, format!("{mpath}.rho"), lc(&member.id, &member.id)),
                lc("RC longitudinal reinforcement ratio ρ", "Stahlbeton-Längsbewehrungsgrad ρ"),
            )
            .utilization(q_dim(member.rho), q_dim(if member.rho < rho_min { rho_min } else { rho_max }))
            .annex(annex)
            .explanation(lc(&format!("ρ={:.4}, limits [{rho_min:.4}, {rho_max:.4}]", member.rho), &format!("ρ={:.4}, Grenzen [{rho_min:.4}, {rho_max:.4}]", member.rho)));
            if !ok_rho {
                c = c.status(semio_s_plugin_norm_compliance::CheckStatus::Fail);
            }
            if member.rho + 1e-12 < rho_min {
                c = c.remedy(Remedy::at_least(
                    SubjectRef::new(bid, format!("{mpath}.rho"), lc("ρ", "ρ")),
                    q_dim(member.rho),
                    q_dim(rho_min),
                    lc(&format!("Increase ρ to at least {rho_min:.4}.",), &format!("ρ auf mindestens {rho_min:.4} erhöhen.")),
                ));
            } else if member.rho > rho_max + 1e-12 {
                c = c.remedy(Remedy::at_most(
                    SubjectRef::new(bid, format!("{mpath}.rho"), lc("ρ", "ρ")),
                    q_dim(member.rho),
                    q_dim(rho_max),
                    lc(&format!("Reduce ρ to at most {rho_max:.4}.",), &format!("ρ auf höchstens {rho_max:.4} senken.")),
                ));
            }
            report.push(c.build());

            {
                let mut sc = CheckResult::assess(
                    format!("en1998.1.{bid}.{}.member.{}.rc.noSteelClass", system.id, member.id),
                    "DIN EN 1998-1",
                    ClauseId::new("EN 1998-1", "1", "5"),
                    SubjectRef::new(bid, format!("{mpath}.steelSectionClass"), lc(&member.id, &member.id)),
                    lc("RC member has no steel section class", "Stahlbetonbauteil ohne Stahl-Querschnittsklasse"),
                )
                .utilization(q_dim(member.steel_section_class as f64), q_dim(0.5))
                .annex(annex)
                .explanation(lc(
                    &format!("steelSectionClass={} (must be 0 for RC)", member.steel_section_class),
                    &format!("Stahl-Querschnittsklasse={} (muss 0 für Stahlbeton sein)", member.steel_section_class),
                ));
                if member.steel_section_class != 0 {
                    sc = sc.status(semio_s_plugin_norm_compliance::CheckStatus::Fail).remedy(Remedy::exactly(
                        SubjectRef::new(bid, format!("{mpath}.steelSectionClass"), lc("Steel section class", "Stahl-Querschnittsklasse")),
                        q_dim(member.steel_section_class as f64),
                        q_dim(0.0),
                        lc("Set steelSectionClass to 0 for RC members.", "Stahl-Querschnittsklasse für Stahlbeton auf 0 setzen."),
                    ));
                }
                report.push(sc.build());
            }

            let w_min = part_1::rc_omega_wd_min(gov_dc);
            if w_min > 0.0 && member.role == semio_s_artifact_norm_en1998_model::MemberKind::Column {
                let ok = member.omega_wd + 1e-12 >= w_min;
                let mut c = CheckResult::assess(
                    format!("en1998.1.{bid}.{}.member.{}.rc.omegaWd", system.id, member.id),
                    "DIN EN 1998-1",
                    ClauseId::new("EN 1998-1", "1", "5.4.3.2.2"),
                    SubjectRef::new(bid, format!("{mpath}.omegaWd"), lc(&member.id, &member.id)),
                    lc("RC confinement ω_wd", "Stahlbeton-Umschnürung ω_wd"),
                )
                .minimum(q_dim(member.omega_wd), q_dim(w_min))
                .annex(annex)
                .explanation(lc(&format!("ω_wd={:.3}, required≥{w_min:.3}", member.omega_wd), &format!("ω_wd={:.3}, erforderlich≥{w_min:.3}", member.omega_wd)));
                if !ok {
                    c = c.remedy(Remedy::at_least(
                        SubjectRef::new(bid, format!("{mpath}.omegaWd"), lc("ω_wd", "ω_wd")),
                        q_dim(member.omega_wd),
                        q_dim(w_min),
                        lc(&format!("Increase ω_wd to at least {w_min:.3}.",), &format!("ω_wd auf mindestens {w_min:.3} erhöhen.")),
                    ));
                }
                report.push(c.build());
            }
            if member.role == semio_s_artifact_norm_en1998_model::MemberKind::Beam {
                let ratio = if member.rho > 1e-12 { member.rho_prime / member.rho } else { 0.0 };
                let ok = ratio + 1e-12 >= 0.5;
                let mut c = CheckResult::assess(
                    format!("en1998.1.{bid}.{}.member.{}.rc.rhoPrime", system.id, member.id),
                    "DIN EN 1998-1",
                    ClauseId::new("EN 1998-1", "1", "5.4.3.1.2"),
                    SubjectRef::new(bid, format!("{mpath}.rhoPrime"), lc(&member.id, &member.id)),
                    lc("Beam ρ′/ρ ≥ 0.5", "Träger ρ′/ρ ≥ 0.5"),
                )
                .minimum(q_dim(ratio), q_dim(0.5))
                .annex(annex)
                .explanation(lc(&format!("beam reinforcement ratio ρ′/ρ={ratio:.3}"), &format!("Träger-Bewehrungsverhältnis ρ′/ρ={ratio:.3}")));
                if !ok {
                    let req = 0.5 * member.rho;
                    c = c.remedy(Remedy::at_least(
                        SubjectRef::new(bid, format!("{mpath}.rhoPrime"), lc("ρ′", "ρ′")),
                        q_dim(member.rho_prime),
                        q_dim(req),
                        lc(&format!("Increase ρ′ to at least {req:.4}.",), &format!("ρ′ auf mindestens {req:.4} erhöhen.")),
                    ));
                }
                report.push(c.build());
            }
            if member.role != semio_s_artifact_norm_en1998_model::MemberKind::Column {
                let mut c = CheckResult::assess(
                    format!("en1998.1.{bid}.{}.member.{}.rc.omegaWdBeam", system.id, member.id),
                    "DIN EN 1998-1",
                    ClauseId::new("EN 1998-1", "1", "5.4.3.2.2"),
                    SubjectRef::new(bid, format!("{mpath}.omegaWd"), lc(&member.id, &member.id)),
                    lc("RC beam confinement ω_wd (recorded)", "Stahlbetonbalken-Umschnürung ω_wd (dokumentiert)"),
                )
                .utilization(q_dim(member.omega_wd), q_dim(0.30))
                .annex(annex)
                .explanation(lc(
                    &format!("ω_wd={:.3} (beam; limit 0.30 for excessive confinement steel)", member.omega_wd),
                    &format!("ω_wd={:.3} (Balken; Grenze 0.30 für übermässige Umschnürung)", member.omega_wd),
                ));
                if member.omega_wd > 0.30 {
                    c = c.remedy(Remedy::at_most(
                        SubjectRef::new(bid, format!("{mpath}.omegaWd"), lc("ω_wd", "ω_wd")),
                        q_dim(member.omega_wd),
                        q_dim(0.30),
                        lc("Reduce beam ω_wd to at most 0.30.", "Balken-ω_wd auf höchstens 0.30 senken."),
                    ));
                }
                report.push(c.build());
            }
        }
        if mat == semio_s_artifact_norm_en1998_model::StructuralMaterial::Steel {
            let max_class = part_1::steel_max_section_class(q);
            let ok = member.steel_section_class > 0 && member.steel_section_class <= max_class;
            let mut c = CheckResult::assess(
                format!("en1998.1.{bid}.{}.member.{}.steel.class", system.id, member.id),
                "DIN EN 1998-1",
                ClauseId::new("EN 1998-1", "1", "6.5"),
                SubjectRef::new(bid, format!("{mpath}.steelSectionClass"), lc(&member.id, &member.id)),
                lc("Steel section class vs q", "Stahl-Querschnittsklasse vs q"),
            )
            .utilization(q_dim(if ok { (member.steel_section_class as f64).min(max_class as f64) } else { member.steel_section_class as f64 }), q_dim(max_class as f64))
            .annex(annex)
            .explanation(lc(&format!("class={}, max for q={q:.2} is {max_class}", member.steel_section_class), &format!("Klasse={}, max für q={q:.2} ist {max_class}", member.steel_section_class)));
            if !ok {
                c = c.remedy(Remedy::at_most(
                    SubjectRef::new(bid, format!("{mpath}.steelSectionClass"), lc("Section class", "Querschnittsklasse")),
                    q_dim(member.steel_section_class as f64),
                    q_dim(max_class as f64),
                    lc(&format!("Use section class ≤ {max_class} (or reduce q0).",), &format!("Querschnittsklasse ≤ {max_class} verwenden (oder q0 senken).")),
                ));
            }
            report.push(c.build());
        }

        let ok = member.detailing_compatible_with_q;
        let mut mcheck = CheckResult::assess(
            format!("en1998.1.{bid}.{}.member.{}.qDetail", system.id, member.id),
            "DIN EN 1998-1",
            ClauseId::new("EN 1998-1", "1", "5.2"),
            SubjectRef::new(bid, format!("{mpath}.detailingCompatibleWithQ"), lc(&member.id, &member.id)),
            lc("q-compatible detailing declared", "q-gerechte Durchbildung erklärt"),
        )
        .status(if ok { semio_s_plugin_norm_compliance::CheckStatus::Pass } else { semio_s_plugin_norm_compliance::CheckStatus::Fail })
        .annex(annex)
        .explanation(lc(
            &format!("Member {} detailing_compatible_with_q={ok} for q={q:.2}", member.id),
            &format!("Bauteil {} detailing_compatible_with_q={ok} für q={q:.2}", member.id),
        ));
        if !ok {
            mcheck = mcheck.remedy(Remedy::exactly(
                SubjectRef::new(bid, format!("{mpath}.detailingCompatibleWithQ"), lc("Detailing flag", "Durchbildungsflag")),
                q_dim(0.0),
                q_dim(1.0),
                lc("Provide q-compatible detailing (or reduce q0).", "q-gerechte Durchbildung herstellen (oder q0 reduzieren)."),
            ));
        }
        report.push(mcheck.build());
    }
}

/// 🧱 Checks the simple masonry wall area ratio.
pub fn check_simple_masonry(
    document: &En1998Snapshot,
    annex: AnnexChoice,
    report: &mut CheckReport,
    building: &semio_s_artifact_norm_en1998_model::En1998Building,
    bid: &str,
    bpath: &str,
    blabel: &LocalizedCopy,
    system: &semio_s_artifact_norm_en1998_model::En1998System,
) {
    {
        let a_gr = if matches!(annex, AnnexChoice::De) { na_de::a_gr_from_s_ap_r(document.site.s_ap_r) } else { document.site.a_gr };
        let limit = part_1::masonry_wall_ratio_limit(a_gr, document.site.importance_class);
        let claiming = building.claims_simple_masonry || system.system_type == semio_s_artifact_norm_en1998_model::SystemType::Masonry;
        let walls = part_1::masonry_walls_of(&building.members);
        let area = building.plan_width_m * building.plan_length_m;
        let storey_ids: Vec<&str> = building.storeys.iter().map(|s| s.id.as_str()).collect();
        let governing = part_1::masonry_governing_density(&walls, area, &storey_ids);
        let ratio = governing.as_ref().map_or(building.masonry_wall_area_ratio, |g| g.0);
        let location = governing.as_ref().map_or((String::new(), String::new()), |(_, along_y, storey)| (format!(", governing direction {} at storey {storey} from {} wall rows", if *along_y { "y" } else { "x" }, walls.len()), format!(", maßgebende Richtung {} im Geschoss {storey} aus {} Wandzeilen", if *along_y { "y" } else { "x" }, walls.len())));
        let mut m = CheckResult::assess(
            format!("en1998.1.{bid}.{}.masonry.simple", system.id),
            "DIN EN 1998-1/NA",
            ClauseId::new("EN 1998-1", "1", "9.7.2"),
            SubjectRef::new(bid, format!("{bpath}.masonryWallAreaRatio"), blabel.clone()),
            lc("Simple masonry building wall ratio (DE Table NA.12)", "Einfache Mauerwerksbauten Wandanteil (DE Tab. NA.12)"),
        )
        .minimum(q_dim(ratio), q_dim(limit))
        .annex(annex)
        .ndps(if claiming { &part_1::MASONRY_NDP_IDS[..] } else { &[] })
        .explanation(lc(
            &format!("claims={}, wall ratio={:.4}{}, required≥{:.4}", claiming, ratio, location.0, limit),
            &format!("Beansprucht={}, Wandanteil={:.4}{}, erforderlich≥{:.4}", claiming, ratio, location.1, limit),
        ));
        if claiming && ratio + 1e-12 < limit {
            m = m.status(semio_s_plugin_norm_compliance::CheckStatus::Fail);
            m = match governing.as_ref().and_then(|(_, along_y, storey)| walls_in_group(building, *along_y, storey)) {
                Some((id, length, thickness)) => m.remedy(Remedy::at_least(
                    SubjectRef::new(bid, format!("{bpath}.members[id={id}].wallLengthM"), lc("Wall length", "Wandlänge")),
                    q_len(length),
                    q_len(length + (limit - ratio) * area / thickness),
                    lc(&format!("Lengthen wall {id} to at least {:.2} m.", length + (limit - ratio) * area / thickness), &format!("Wand {id} auf mindestens {:.2} m verlängern.", length + (limit - ratio) * area / thickness)),
                )),
                None => m.remedy(Remedy::at_least(
                    SubjectRef::new(bid, format!("{bpath}.masonryWallAreaRatio"), lc("Wall area ratio", "Wandflächenanteil")),
                    q_dim(building.masonry_wall_area_ratio),
                    q_dim(limit),
                    lc(&format!("Increase masonry wall area ratio to at least {limit:.3}."), &format!("Mauerwerks-Wandanteil auf mindestens {limit:.3} erhöhen.")),
                )),
            };
        } else {
            m = m.status(semio_s_plugin_norm_compliance::CheckStatus::Pass);
        }
        report.push(m.build());
    }
}

/// 🧱 The longest wall row (id, length, thickness) running along the governing plan direction in the governing storey row, the row a remedy lengthens.
fn walls_in_group(building: &semio_s_artifact_norm_en1998_model::En1998Building, along_y: bool, storey_id: &str) -> Option<(String, f64, f64)> {
    building.members.iter()
        .filter(|m| part_1::is_masonry_wall(m))
        .filter(|m| (m.wall_storey_id.is_empty() || m.wall_storey_id == storey_id) && (m.wall_direction_rad.sin().abs() > m.wall_direction_rad.cos().abs()) == along_y)
        .max_by(|a, b| a.wall_length_m.total_cmp(&b.wall_length_m))
        .map(|m| (m.id.clone(), m.wall_length_m, m.wall_thickness_m))
}

/// 🎚️ Reports the global displacement ductility μ_δ the behaviour factor q asks for (EN 1998-1 4.4.2 with the relation μ_δ = 2 q − 1 for T1 ≥ T_C and 1 + 2 (q − 1) T_C/T1 for T1 < T_C) and the diaphragm force of the governing storey amplified for transfers (4.4.2.5); both are the ductility and horizontal element parts of SEI-46 and SEI-48.
pub fn check_global_ductility_and_diaphragm(annex: AnnexChoice, report: &mut CheckReport, bid: &str, bpath: &str, blabel: &LocalizedCopy, system: &semio_s_artifact_norm_en1998_model::En1998System, q: f64, t1: f64, tc: f64, forces: &[f64], diaphragm_rows: bool) {
    let mu = if t1 >= tc { 2.0 * q - 1.0 } else if t1 > 0.0 { 1.0 + 2.0 * (q - 1.0) * tc / t1 } else { f64::NAN };
    let spath = format!("{bpath}.systems[id={}]", system.id);
    if mu.is_finite() {
        report.push(
            CheckResult::assess(format!("en1998.1.{bid}.{}.ductility", system.id), "DIN EN 1998-1", ClauseId::new("EN 1998-1", "1", "4.4.2"), SubjectRef::new(bid, format!("{spath}.q0"), blabel.clone()), lc("Global displacement ductility demanded by q", "Globale Verschiebungsduktilität, die q verlangt"))
                .informative(q_dim(mu))
                .annex(annex)
                .explanation(lc(
                    &format!("μ_δ = {mu:.2} for q = {q:.2}, T1 = {t1:.3} s, T_C = {tc:.3} s ({}); the local ductility is checked by the member detailing checks", if t1 >= tc { "2 q − 1" } else { "1 + 2 (q − 1) T_C/T1" }),
                    &format!("μ_δ = {mu:.2} für q = {q:.2}, T1 = {t1:.3} s, T_C = {tc:.3} s ({}); die lokale Duktilität prüfen die Bauteil-Konstruktionsregeln", if t1 >= tc { "2 q − 1" } else { "1 + 2 (q − 1) T_C/T1" }),
                ))
                .build(),
        );
    }
    if diaphragm_rows {
        return;
    }
    let Some((index, governing)) = forces.iter().copied().enumerate().fold(None, |best: Option<(usize, f64)>, (i, f)| match best { Some(b) if b.1 >= f => Some(b), _ => Some((i, f)) }) else { return };
    let id = "ndp.en1998.en1.diaphragm.transfer-factor";
    let factor = semio_s_plugin_norm_compliance::ndp_number(id, annex);
    report.push(
        CheckResult::assess(format!("en1998.1.{bid}.{}.diaphragm", system.id), "DIN EN 1998-1", ClauseId::new("EN 1998-1", "1", "4.4.2.5"), SubjectRef::new(bid, format!("{spath}.baseShearResistanceN"), blabel.clone()), lc("Diaphragm force of the governing storey, amplified for transfers", "Scheibenkraft des maßgebenden Geschosses, für Umlagerungen vergrößert"))
            .informative(q_force(factor * governing))
            .annex(annex)
            .explanation(lc(
                &format!("F_i = {governing:.0} N at storey {} of {}, transfer amplification {factor:.2} gives {:.0} N; no diaphragm is modelled for this building, so the diaphragm and chord resistance is not checked", index + 1, forces.len(), factor * governing),
                &format!("F_i = {governing:.0} N im Geschoss {} von {}, Umlagerungsfaktor {factor:.2} ergibt {:.0} N; für dieses Gebäude ist keine Scheibe modelliert, Scheiben- und Gurtwiderstand werden daher nicht nachgewiesen", index + 1, forces.len(), factor * governing),
            ))
            .ndp(id)
            .build(),
    );
}
