//! 🔥️ EN 1999 fire and fatigue checks: `check_fire` (EN 1999-1-2) and `check_fatigue` (EN 1999-1-3).

use super::*;
use super::part_1_1::EffCtx;

//#region 🔖️ComplianceHelpers
fn fire_label(id: &str) -> LocalizedCopy { loc(&format!("Fire {id}"), &format!("Brand {id}")) }
struct FireResistance {
    n_t: f64,
    n_c: f64,
    m_y: f64,
    m_z: f64,
    n_b: f64,
    m_b: f64,
}

fn fire_resistance(section: &AluminiumSection, member: &AluminiumMember, alloy: &part_1_1::AlloyProps, params: &na_de::AnnexParams, theta: f64, field: Option<&part_1_2::FireField>) -> FireResistance {
    let fo = alloy.f_o_pa;
    let k = part_1_2::k_theta(theta);
    let k_e = part_1_2::k_e_theta(theta).max(1e-6);
    let g = params.gamma_m_fi;
    let base = EffCtx { f_o_pa: fo, class_a: alloy.buckling_class_a, eta_web: 1.0, local: true, haz_rho: None, haz_extents: &[] };
    let loc_p = part_1_1::props(section, &base);
    let eff = part_1_1::props(section, &EffCtx { haz_rho: Some(alloy.rho_o_haz), ..base });
    let class = part_1_1::section_class(section, fo, alloy.buckling_class_a, 1.0);
    let (_, i_y, i_z, _, _, _, _) = part_1_1::section_geometry(section);
    let k_sup = match member.support { SupportCondition::Continuous => 0.7, SupportCondition::Cantilever => 2.0, SupportCondition::SimplySupported => 1.0 };
    let pick = |l: f64| if l > 0.0 { l } else { k_sup * member.length };
    let (alpha, l0) = part_1_1::buckling_curve(alloy.buckling_class_a);
    let scale = (k / k_e).sqrt();
    let (r_n, r_my, r_mz) = field.map_or((1.0, 1.0, 1.0), |f| (f.n_ratio, f.m_y_ratio, f.m_z_ratio));
    let chi_axis = |i: f64, l: f64| part_1_1::chi_from_lambda(part_1_1::lambda_bar(l, i, loc_p.a, fo) * scale, alpha, l0);
    let chi = chi_axis(i_y, pick(member.buckling_length_y)).min(chi_axis(i_z, pick(member.buckling_length_z)));
    let m_y = eff.wy * k * fo / g * r_my;
    let l_lt = if member.ltb_length > 0.0 { member.ltb_length } else { member.length };
    let m_cr = if member.restrained_ltb { f64::INFINITY } else { part_1_1::m_cr_of(section, member.c1, l_lt) };
    let (alpha_lt, l0_lt) = part_1_1::ltb_curve(class);
    let lam_lt = if m_cr.is_finite() && m_cr > 0.0 { (eff.wy * fo / m_cr).sqrt() * scale } else { 0.0 };
    let chi_lt = part_1_1::chi_from_lambda(lam_lt, alpha_lt, l0_lt);
    FireResistance {
        n_t: k * eff.a * fo / g * r_n,
        n_c: k * eff.a * fo / g * r_n,
        m_y,
        m_z: eff.wz * k * fo / g * r_mz,
        n_b: chi * k * loc_p.a * fo / g * r_n,
        m_b: chi_lt * m_y,
    }
}

/// 🔥️ Utilisation Σ(N_t, N_c, M_y, M_z) of the cross-section of EN 1999-1-2 4.2 at the uniform member temperature `theta` [°C] for the fire-situation effects of the member (`None` for an unknown alloy); the member resistance over time follows by inserting the member temperature history. k_θ values unverified.
pub fn fire_utilisation_at(member: &AluminiumMember, section: &AluminiumSection, material: &AluminiumMaterial, annex: AnnexChoice, theta: f64) -> Option<f64> {
    let alloy = part_1_1::resolve_alloy_for_section(&material.designation, section)?;
    let gov = part_en1990::fire_member_effects(member);
    let r = fire_resistance(section, member, &alloy, &na_de::AnnexParams::for_choice(annex), theta, None);
    Some(fire_ratio(gov.n_ed.max(0.0), r.n_t) + fire_ratio((-gov.n_ed).max(0.0), r.n_c) + fire_ratio(gov.m_y_ed, r.m_y) + fire_ratio(gov.m_z_ed, r.m_z))
}

fn fire_ratio(x: f64, r: f64) -> f64 {
    if x.abs() < 1e-12 { 0.0 } else if r <= 1e-12 { 1.0e6 } else { (x.abs() / r).min(1.0e6) }
}

pub fn check_fire(
    fire: &FireScenario,
    member: &AluminiumMember,
    section: &AluminiumSection,
    material: &AluminiumMaterial,
    insulation: Option<&semio_s_artifact_norm_en1999_model::FireInsulation>,
    annex: AnnexChoice,
) -> Vec<CheckResult> {
    let Some(alloy) = part_1_1::resolve_alloy_for_section(&material.designation, section) else {
        return vec![alloy_unknown(format!("en1999.1-2.fire.{}", fire.id), &fire.id, material, annex)];
    };
    let params = na_de::AnnexParams::for_choice(annex);
    let gov = part_en1990::fire_member_effects(member);
    let a_m_v = part_1_2::section_factor(section);
    let cover = insulation.map(part_1_2::Insulation::from).filter(|i| i.thickness > 0.0);
    let temperature_at = |t: f64| part_1_2::member_temperature_c(fire.theta_a, a_m_v, t, cover.as_ref());
    let theta_calc = temperature_at(fire.duration_s.max(0.0));
    let theta = theta_calc;
    let k = part_1_2::k_theta(theta);
    let field = cover.is_none().then(|| part_1_2::temperature_field(section, alloy.f_o_pa, alloy.buckling_class_a, fire.theta_a, fire.duration_s.max(0.0), theta_calc)).flatten();
    let r = fire_resistance(section, member, &alloy, &params, theta, field.as_ref());
    let r20 = fire_resistance(section, member, &alloy, &params, 20.0, None);
    let (n, my, mz) = (gov.n_ed, gov.m_y_ed, gov.m_z_ed);
    let nc = (-n).max(0.0);
    let sum = |rs: &FireResistance| fire_ratio(n.max(0.0), rs.n_t) + fire_ratio(nc, rs.n_c) + fire_ratio(my, rs.m_y) + fire_ratio(mz, rs.m_z);
    let u_cs = sum(&r);
    let u_buck = fire_ratio(nc, r.n_b);
    let u_ltb = fire_ratio(my, r.m_b);
    let mu0 = sum(&r20);
    let theta_cr = part_1_2::critical_temperature_c(mu0);
    let mu0_new = mu0.min(0.8);
    let target = (part_1_2::critical_temperature_c(mu0_new) - 5.0).max(20.0);
    let mut t_hi = fire.duration_s.max(0.0);
    let mut t_lo = 0.0;
    for _ in 0..40 {
        let mid = 0.5 * (t_lo + t_hi);
        if temperature_at(mid) <= target { t_lo = mid; } else { t_hi = mid; }
    }
    let t_target = t_lo;
    let u_loads = if mu0 > 0.8 { (mu0 * 1.2375).max(1.0001) } else { 0.0 };
    let temperature_remedies = |u: f64| -> Vec<Remedy> {
        let mut rem = Vec::new();
        if u <= 1.0 { return rem; }
        let subject = |path: &str| SubjectRef::new(&fire.id, format!("fireScenarios[id={}].{path}", fire.id), loc("Fire scenario", "Brandszenario"));
        if fire.theta_a > target {
            rem.push(Remedy::at_most(subject("thetaA"), q_temp(fire.theta_a), q_temp(target), loc("Lower the member design temperature below the critical temperature.", "Bemessungstemperatur des Bauteils unter die kritische Temperatur senken.")));
        }
        if theta_calc > target {
            rem.push(Remedy::at_most(subject("durationS"), q_time(fire.duration_s), q_time(t_target), loc("Shorten the required fire resistance duration.", "Geforderte Feuerwiderstandsdauer verkürzen.")));
        }
        if let Some(i) = insulation.filter(|i| i.thickness > 0.0 && theta_calc > target) {
            let covered = |d: f64| part_1_2::protected_temperature_c(fire.theta_a, a_m_v, fire.duration_s.max(0.0), &part_1_2::Insulation { thickness: d, ..part_1_2::Insulation::from(i) });
            if let Some(d) = (1..=40).map(|n| i.thickness * 1.25f64.powi(n)).find(|d| covered(*d) <= target) {
                rem.push(Remedy::at_least(SubjectRef::new(&i.id, format!("fireInsulations[id={}].thickness", i.id), loc("Fire insulation", "Brandschutzbekleidung")), q_length(i.thickness), q_length(d), loc("Thicken the fire insulation.", "Brandschutzbekleidung verstärken.")));
            }
        }
        rem.extend(leaf_remedies("members", &member.id, &member.actions, u_loads, &member_label(&member.id), "Reduce the fire-situation load", "Last der Brandsituation senken", "mYK"));
        if rem.is_empty() {
            rem.extend(leaf_remedies("members", &member.id, &member.actions, u.max(1.0001), &member_label(&member.id), "Reduce the fire-situation load", "Last der Brandsituation senken", "mYK"));
        }
        rem
    };
    let tag_en = format!("θ_a = {theta:.0} °C (start {:.0} °C, A_m/V = {a_m_v:.0} 1/m), k_θ = {k:.3}, γ_M,fi = {:.2} (fire {}, lead {})", fire.theta_a, params.gamma_m_fi, gov.combination, gov.action_id);
    let tag_de = format!("θ_a = {theta:.0} °C (Start {:.0} °C, A_m/V = {a_m_v:.0} 1/m), k_θ = {k:.3}, γ_M,fi = {:.2} (Brand {}, führend {})", fire.theta_a, params.gamma_m_fi, gov.combination, gov.action_id);
    let subject = SubjectRef::new(&fire.id, format!("fireScenarios[id={}].durationS", fire.id), fire_label(&fire.id));
    let cl = |s: &str| ClauseId::new("EN 1999-1-2", format!("§{s}"), s);
    let mk = |id: String, sec: &str, title: (&str, &str), computed: Quantity, limit: Quantity, expl_en: String, expl_de: String, u: f64| {
        utilization_check(id, "DIN EN 1999-1-2", cl(sec), subject.clone(), loc(title.0, title.1), computed, limit, annex, loc(&expl_en, &expl_de), temperature_remedies(u))
    };
    let mut out = vec![
        mk(format!("en1999.1-2.fire.{}", fire.id), "4.2", ("Fire resistance of the cross-section", "Feuerwiderstand des Querschnitts"), q_dim(u_cs), q_dim(1.0),
            format!("N_fi,Rd = {:.1} kN (tension) / {:.1} kN (compression), M_fi,y,Rd = {:.2} kNm, M_fi,z,Rd = {:.2} kNm, N_Ed = {:.1} kN, M_y,Ed = {:.2} kNm; {tag_en}", r.n_t / 1e3, r.n_c / 1e3, r.m_y / 1e3, r.m_z / 1e3, n / 1e3, my / 1e3),
            format!("N_fi,Rd = {:.1} kN (Zug) / {:.1} kN (Druck), M_fi,y,Rd = {:.2} kNm, M_fi,z,Rd = {:.2} kNm, N_Ed = {:.1} kN, M_y,Ed = {:.2} kNm; {tag_de}", r.n_t / 1e3, r.n_c / 1e3, r.m_y / 1e3, r.m_z / 1e3, n / 1e3, my / 1e3), u_cs),
        mk(format!("en1999.1-2.fire.buckling.{}", fire.id), "4.2.3", ("Flexural buckling in fire", "Biegeknicken im Brandfall"), q_force(nc), q_force(floor_limit(nc, r.n_b)),
            format!("N_b,fi,Rd = {:.1} kN with λ̄_θ = λ̄·√(k_0.2/k_E); {tag_en}", r.n_b / 1e3),
            format!("N_b,fi,Rd = {:.1} kN mit λ̄_θ = λ̄·√(k_0.2/k_E); {tag_de}", r.n_b / 1e3), u_buck),
        mk(format!("en1999.1-2.fire.ltb.{}", fire.id), "4.2.3", ("Lateral-torsional buckling in fire", "Biegedrillknicken im Brandfall"), q_moment(my.abs()), q_moment(floor_limit(my, r.m_b)),
            format!("M_b,fi,Rd = {:.2} kNm; {tag_en}", r.m_b / 1e3),
            format!("M_b,fi,Rd = {:.2} kNm; {tag_de}", r.m_b / 1e3), u_ltb),
        mk(format!("en1999.1-2.fire.theta-cr.{}", fire.id), "4.2.5", ("Member temperature against critical temperature", "Bauteiltemperatur gegen kritische Temperatur"), q_temp(theta), q_temp(theta_cr.max(1.0)),
            format!("μ_0 = {mu0:.3} gives θ_cr = {theta_cr:.0} °C; {tag_en}"),
            format!("μ_0 = {mu0:.3} ergibt θ_cr = {theta_cr:.0} °C; {tag_de}"), theta / theta_cr.max(1.0)),
    ];
    if let Some(f) = &field {
        let en = format!("Strip temperatures from {:.0} °C to {:.0} °C after {:.0} min (uniform member temperature {theta:.0} °C): the strength retained is {:.0} % in tension and compression, {:.0} % about y and {:.0} % about z of the uniform estimate; the fire resistances above already contain these factors (EN 1999-1-2 Table 3.1 unverified)", f.theta_min, f.theta_max, fire.duration_s / 60.0, f.n_ratio * 100.0, f.m_y_ratio * 100.0, f.m_z_ratio * 100.0);
        let de = format!("Streifentemperaturen von {:.0} °C bis {:.0} °C nach {:.0} min (einheitliche Bauteiltemperatur {theta:.0} °C): die verbleibende Festigkeit beträgt {:.0} % bei Zug und Druck, {:.0} % um y und {:.0} % um z der einheitlichen Schätzung; die obigen Feuerwiderstände enthalten diese Faktoren bereits (EN 1999-1-2 Tabelle 3.1 ungeprüft)", f.theta_min, f.theta_max, fire.duration_s / 60.0, f.n_ratio * 100.0, f.m_y_ratio * 100.0, f.m_z_ratio * 100.0);
        out.push(CheckResult::assess(format!("en1999.1-2.fire.field.{}", fire.id), "DIN EN 1999-1-2", cl("3.2"), subject.clone(), loc("Temperature field over the section", "Temperaturfeld über den Querschnitt")) .status(semio_s_plugin_norm_compliance::CheckStatus::Info) .annex(annex) .explanation(loc(&en, &de)) .build());
    }
    out
}

fn fatigue_fail(fat: &FatigueDetail, id: &str, title: (&str, &str), en: String, de: String, subject: SubjectRef, one_of: Vec<String>, annex: AnnexChoice) -> CheckResult {
    CheckResult::assess(id, "DIN EN 1999-1-3", ClauseId::new("EN 1999-1-3", "§7.1", "7.1"), subject.clone(), loc(title.0, title.1))
        .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
        .explanation(loc(&format!("{en} (fatigue detail {}).", fat.id), &format!("{de} (Ermüdungsdetail {}).", fat.id)))
        .annex(annex)
        .remedy(Remedy::one_of(subject, one_of, loc("Pick a valid value.", "Gültigen Wert wählen.")))
        .build()
}

pub fn check_fatigue(fat: &FatigueDetail, member: Option<&AluminiumMember>, annex: AnnexChoice) -> Vec<CheckResult> {
    let params = na_de::AnnexParams::for_choice(annex);
    let subject = |path: &str| SubjectRef::new(&fat.id, format!("fatigueDetails[id={}].{path}", fat.id), loc(&format!("Fatigue {}", fat.id), &format!("Ermüdung {}", fat.id)));
    if member.is_none() && !fat.member_id.is_empty() {
        return vec![fatigue_fail(fat, &format!("en1999.1-3.fat.{}", fat.id), ("Fatigue member reference", "Ermüdungs-Bauteilverweis"),
            format!("Fatigue detail '{}' references missing member '{}'", fat.id, fat.member_id),
            format!("Ermüdungsdetail '{}' verweist auf fehlendes Bauteil '{}'", fat.id, fat.member_id),
            subject("memberId"), vec![fat.member_id.clone()], annex)];
    }
    let Some(dc) = part_1_3::detail_category_delta_sigma_c(&fat.detail_category) else {
        return vec![fatigue_fail(fat, &format!("en1999.1-3.fat.{}", fat.id), ("Fatigue detail category", "Ermüdungskerbfall"),
            format!("Unknown detail category '{}'", fat.detail_category),
            format!("Unbekannter Kerbfall '{}'", fat.detail_category),
            subject("detailCategory"), vec!["71".into(), "80".into(), "90".into()], annex)];
    };
    if fat.m1 <= 0.0 {
        return vec![fatigue_fail(fat, &format!("en1999.1-3.fat.{}", fat.id), ("Fatigue slope m1", "Ermüdungsneigung m1"),
            format!("The first S–N slope m1 = {} must be positive", fat.m1),
            format!("Die erste Neigung m1 = {} der Wöhlerlinie muss positiv sein", fat.m1),
            subject("m1"), vec!["3.4".into(), "4.3".into()], annex)];
    }
    let m1 = fat.m1;
    let m2 = if fat.m2 > 0.0 { fat.m2 } else { m1 + 2.0 };
    let d = part_1_3::damage(fat.delta_sigma_ed, fat.n_cycles, dc, m1, m2, params.gamma_ff, params.gamma_mf);
    let rd = part_1_3::fatigue_strength_pa(dc, m1, m2, fat.n_cycles) / params.gamma_mf;
    let ed = fat.delta_sigma_ed * params.gamma_ff;
    let make_remedies = |u: f64| -> Vec<Remedy> {
        let mut rem = Vec::new();
        if u <= 1.0 { return rem; }
        let (mut lo, mut hi) = (0.0, fat.delta_sigma_ed);
        for _ in 0..60 {
            let mid = 0.5 * (lo + hi);
            if part_1_3::damage(mid, fat.n_cycles, dc, m1, m2, params.gamma_ff, params.gamma_mf) <= 0.98 && mid * params.gamma_ff <= rd { lo = mid; } else { hi = mid; }
        }
        rem.push(Remedy::at_most(subject("deltaSigmaEd"), q_stress(fat.delta_sigma_ed), q_stress(lo), loc(&format!("Reduce Δσ_Ed to at most {:.1} MPa.", lo / 1e6), &format!("Δσ_Ed auf höchstens {:.1} MPa reduzieren.", lo / 1e6))));
        let better: Vec<String> = ["45", "50", "56", "63", "71", "80", "90", "100", "112", "125"].iter()
            .filter(|c| {
                let v = part_1_3::detail_category_delta_sigma_c(c).unwrap_or(0.0);
                part_1_3::damage(fat.delta_sigma_ed, fat.n_cycles, v, m1, m2, params.gamma_ff, params.gamma_mf) <= 0.98 && ed <= part_1_3::fatigue_strength_pa(v, m1, m2, fat.n_cycles) / params.gamma_mf
            })
            .map(|c| (*c).to_string()).collect();
        if !better.is_empty() {
            rem.push(Remedy::one_of(subject("detailCategory"), better, loc("Select a higher Annex J detail category.", "Höheren Annex-J-Kerbfall wählen.")));
        }
        rem
    };
    let sn_en = format!("Detail {}, Δσ_C = {:.0} MPa, m1 = {m1:.1}, m2 = {m2:.1}, Δσ_D = {:.1} MPa, Δσ_L = {:.1} MPa, γ_Ff = {:.2}, γ_Mf = {:.2}, n = {:.3e}", fat.detail_category, dc / 1e6, part_1_3::delta_sigma_d(dc, m1) / 1e6, part_1_3::delta_sigma_l(dc, m1, m2) / 1e6, params.gamma_ff, params.gamma_mf, fat.n_cycles);
    let sn_de = sn_en.replacen("Detail", "Kerbfall", 1);
    let cl = ClauseId::new("EN 1999-1-3", "§7.1", "7.1");
    vec![
        utilization_check(format!("en1999.1-3.fat.{}", fat.id), "DIN EN 1999-1-3", cl.clone(), subject("deltaSigmaEd"),
            loc("Fatigue damage D = Σ n/N (Palmgren–Miner)", "Ermüdungsschädigung D = Σ n/N (Palmgren–Miner)"), q_dim(d), q_dim(1.0), annex,
            loc(&format!("D = {d:.3} ≤ 1; {sn_en}; member {}", fat.member_id), &format!("D = {d:.3} ≤ 1; {sn_de}; Bauteil {}", fat.member_id)), make_remedies(d)),
        utilization_check(format!("en1999.1-3.fat.range.{}", fat.id), "DIN EN 1999-1-3", cl, subject("deltaSigmaEd"),
            loc("Stress range γ_Ff·Δσ_Ed ≤ Δσ_R(N)/γ_Mf", "Spannungsschwingbreite γ_Ff·Δσ_Ed ≤ Δσ_R(N)/γ_Mf"), q_stress(ed), q_stress(rd.max(1e-9)), annex,
            loc(&format!("Δσ_R(N)/γ_Mf = {:.1} MPa; {sn_en}; member {}", rd / 1e6, fat.member_id), &format!("Δσ_R(N)/γ_Mf = {:.1} MPa; {sn_de}; Bauteil {}", rd / 1e6, fat.member_id)), make_remedies(ed / rd.max(1e-9))),
    ]
}
/// 📊 The Palmgren–Miner check of one stress-range spectrum (EN 1999-1-3 §7: D = Σ n_i/N_i ≤ 1): `group` holds the fatigue details of one member and one detail category, each row one block (Δσ_Ed, n); a group of one row is covered by [`check_fatigue`]. The remedies scale every block range down (all at once) or raise the category of every row.
pub fn check_fatigue_spectrum(group: &[&FatigueDetail], annex: AnnexChoice) -> Vec<CheckResult> {
    let Some(first) = group.first().filter(|_| group.len() > 1) else { return Vec::new() };
    let Some((dc, m1, m2)) = part_1_3::spectrum_curve(first) else { return Vec::new() };
    let params = na_de::AnnexParams::for_choice(annex);
    let blocks: Vec<(f64, f64)> = group.iter().map(|d| (d.delta_sigma_ed, d.n_cycles)).collect();
    let total = |dc: f64, scale: f64| part_1_3::spectrum_damage(&blocks.iter().map(|(ds, n)| (ds * scale, *n)).collect::<Vec<_>>(), dc, m1, m2, params.gamma_ff, params.gamma_mf);
    let d_total = total(dc, 1.0);
    let shares: Vec<f64> = blocks.iter().map(|(ds, n)| part_1_3::damage(*ds, *n, dc, m1, m2, params.gamma_ff, params.gamma_mf)).collect();
    let governing = shares.iter().enumerate().max_by(|a, b| a.1.total_cmp(b.1)).map_or(0, |(i, _)| i);
    let subject = |row: &FatigueDetail, path: &str| SubjectRef::new(&row.id, format!("fatigueDetails[id={}].{path}", row.id), loc(&format!("Fatigue {}", row.id), &format!("Ermüdung {}", row.id)));
    let mut remedies = Vec::new();
    if d_total > 1.0 {
        let (mut lo, mut hi) = (0.0, 1.0);
        for _ in 0..60 {
            let mid = 0.5 * (lo + hi);
            if total(dc, mid) <= 0.98 { lo = mid; } else { hi = mid; }
        }
        for row in group {
            remedies.push(Remedy::at_most(subject(row, "deltaSigmaEd"), q_stress(row.delta_sigma_ed), q_stress(row.delta_sigma_ed * lo), loc(&format!("Scale every block range of the spectrum by {lo:.2}."), &format!("Alle Blockspannungsschwingbreiten des Kollektivs mit {lo:.2} skalieren."))));
        }
        let better: Vec<String> = part_1_3::CATEGORY_MPA.iter().filter(|c| total(*c * 1.0e6, 1.0) <= 0.98).map(|c| format!("{c:.0}")).collect();
        if !better.is_empty() {
            for row in group {
                remedies.push(Remedy::one_of(subject(row, "detailCategory"), better.clone(), loc("Select a higher Annex J detail category for every block of the spectrum.", "Höheren Annex-J-Kerbfall für alle Blöcke des Kollektivs wählen.")));
            }
        }
    }
    let list = group.iter().zip(&shares).map(|(d, s)| format!("{}: Δσ = {:.1} MPa, n = {:.2e}, D = {s:.3}", d.id, d.delta_sigma_ed / 1e6, d.n_cycles)).collect::<Vec<_>>().join("; ");
    let eq = part_1_3::equivalent_range(&blocks, m1);
    let text = format!("D = Σ n/N = {d_total:.3} over {} blocks of detail category {} (Δσ_C = {:.0} MPa, m1 = {m1:.1}, m2 = {m2:.1}, γ_Ff = {:.2}, γ_Mf = {:.2}); governing block {} with {:.0} % of the damage; Δσ_eq(m1) = {:.1} MPa; {list}", group.len(), first.detail_category, dc / 1e6, params.gamma_ff, params.gamma_mf, group[governing].id, 100.0 * shares[governing] / d_total.max(1e-300), eq / 1e6);
    let text_de = format!("D = Σ n/N = {d_total:.3} über {} Blöcke des Kerbfalls {} (Δσ_C = {:.0} MPa, m1 = {m1:.1}, m2 = {m2:.1}, γ_Ff = {:.2}, γ_Mf = {:.2}); maßgebender Block {} mit {:.0} % der Schädigung; Δσ_eq(m1) = {:.1} MPa; {list}", group.len(), first.detail_category, dc / 1e6, params.gamma_ff, params.gamma_mf, group[governing].id, 100.0 * shares[governing] / d_total.max(1e-300), eq / 1e6);
    vec![utilization_check(format!("en1999.1-3.fat.miner.{}", first.id), "DIN EN 1999-1-3", ClauseId::new("EN 1999-1-3", "§7.1", "7.1"), subject(first, "deltaSigmaEd"), loc("Fatigue damage of the spectrum D = Σ n/N (Palmgren–Miner)", "Ermüdungsschädigung des Kollektivs D = Σ n/N (Palmgren–Miner)"), q_dim(d_total), q_dim(1.0), annex, loc(&text, &text_de), remedies)]
}
//#endregion 🔖️ComplianceHelpers
