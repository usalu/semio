//! 🧱️ Composite slab checks: EN 1994-1-1 §9.3, §9.7, §9.8 (C-33..C-39).

use super::*;

fn spath(slab: &CompositeSlab, leaf: &str) -> String {
    format!("slabs[id={}].{leaf}", slab.id)
}

fn sref(slab: &CompositeSlab, leaf: &str) -> SubjectRef {
    SubjectRef::new(slab.id.clone(), spath(slab, leaf), lc(format!("Composite slab {}", slab.id), format!("Verbunddecke {}", slab.id)))
}

fn slab_load_fix(slab: &CompositeSlab, factor: f64) -> Option<Remedy> {
    load_remedy(&slab.id, &format!("slabs[id={}]", slab.id), lc(format!("Composite slab {}", slab.id), format!("Verbunddecke {}", slab.id)), &slab.statics_actions(), factor, "Reduce the imposed load.", "Nutzlast reduzieren.")
}

pub(crate) fn evaluate_slab(report: &mut CheckReport, doc: &En1994Snapshot, slab: &CompositeSlab) {
    let a = doc.annex;
    let p = AnnexParams::for_annex(a);
    let cont = slab.support == semio_s_artifact_norm_en1994_model::SupportCondition::Continuous2Span;
    let uls = part_en1990::uls_composite(&slab.statics_actions(), slab.span_m, slab.span_m, slab.support, 1.0, a, doc.reliability_class);
    let m_ed = uls.m_nm;
    let v_ed = uls.v_n;
    let sec = part_slab::section(slab, a);
    let hp = slab.sheeting.height_m;
    let hc = (slab.concrete_thickness_m - hp).max(0.04);

    let wet = cat::constants::CONCRETE_DENSITY_N_M3 * (hc + hp * part_slab::rib_fill_fraction(slab));
    let cons_q: f64 = slab.actions.iter().filter(|x| x.stage == semio_s_artifact_norm_en1994_model::ActionStage::Construction || x.kind == semio_s_artifact_norm_en1994_model::ActionKind::Construction).map(|x| x.q_area_pa).sum();
    let cons_q = if cons_q > 0.0 { cons_q } else { 750.0 };
    let w_uls = p_g(a, doc.reliability_class) * wet + p_q(a, doc.reliability_class) * cons_q;
    let m_constr = w_uls * slab.span_m.powi(2) / 8.0;
    let m_sheet = part_1_1::sheeting_construction_mrd_nm_per_m(&slab.sheeting, cat::constants::SHEETING_FYP / cat::constants::gamma_ap());
    let (_, defl) = cat::sheeting_formwork(sec.ap, hp, cat::constants::SHEETING_FYP, wet, slab.span_m);
    let u_sh = (m_constr / m_sheet.max(1.0)).max(defl / (slab.span_m / 180.0));
    let mut sh = head(a, format!("en1994.9.3.sheeting.{}", slab.id), "1994-1-1", "9.3", sref(slab, "sheeting.thicknessM"), "Sheeting as formwork at construction stage", "Profilblech als Schalung im Bauzustand")
        .utilization(dim(u_sh), dim(1.0))
        .explanation(lc(format!("M_Ed = {:.2} kNm/m, M_Rd,sheeting = {:.2} kNm/m, wet-concrete deflection {:.1} mm ≤ L/180 = {:.1} mm.", m_constr / 1e3, m_sheet / 1e3, defl * 1e3, slab.span_m / 180.0 * 1e3), format!("M_Ed = {:.2} kNm/m, M_Rd,Blech = {:.2} kNm/m, Durchbiegung Frischbeton {:.1} mm ≤ L/180 = {:.1} mm.", m_constr / 1e3, m_sheet / 1e3, defl * 1e3, slab.span_m / 180.0 * 1e3)));
    if u_sh > 1.0 {
        sh = sh.remedy(Remedy::at_least(sref(slab, "sheeting.thicknessM"), length(slab.sheeting.thickness_m), length(slab.sheeting.thickness_m * u_sh * 1.05), lc("Increase the sheeting thickness.", "Profilblechdicke erhöhen.")));
        sh = sh.remedy(Remedy::at_most(sref(slab, "spanM"), length(slab.span_m), length(slab.span_m / u_sh.sqrt() * 0.95), lc("Shorten the span or prop the sheeting.", "Spannweite verkürzen oder Profilblech unterstützen.")));
    }
    report.push(sh.build());

    let nc = part_slab::bond_force_n_per_m(slab, a, slab.span_m / 2.0);
    let ncf = sec.ap * sec.fyp_d;
    let eta = (nc / ncf.max(1.0)).min(1.0);
    let m_rd = part_slab::bending_resistance_nm_per_m(slab, a);
    let mut mb = head(a, format!("en1994.9.7.2.mrd.{}", slab.id), "1994-1-1", "9.7.2", sref(slab, "actions"), "Composite slab bending with partial connection", "Biegung Verbunddecke mit Teilverbund")
        .utilization(moment(m_ed), moment(m_rd))
        .explanation(lc(format!("{}: m_Ed = {:.2} kNm/m, m_Rd = {:.2} kNm/m (N_c/N_cf = {eta:.2}, bond from m–k).", uls.label_en, m_ed / 1e3, m_rd / 1e3), format!("{}: m_Ed = {:.2} kNm/m, m_Rd = {:.2} kNm/m (N_c/N_cf = {eta:.2}, Verbund aus m–k).", uls.label_de, m_ed / 1e3, m_rd / 1e3)));
    if m_ed > m_rd {
        let d = (slab.concrete_thickness_m - 0.03).max(0.05);
        let f_sd = cat::constants::REBAR_FSK / p.gamma_s;
        let as_need = slab.as_m2_per_m + (m_ed - m_rd) / (f_sd * 0.9 * d).max(1e-12) * 1.05;
        mb = mb.remedy(Remedy::at_least(sref(slab, "asM2PerM"), area(slab.as_m2_per_m), area(as_need), lc(format!("Increase the slab reinforcement to ≥ {:.0} mm²/m.", as_need * 1e6), format!("Plattenbewehrung auf ≥ {:.0} mm²/m erhöhen.", as_need * 1e6))));
        mb = mb.remedy(Remedy::at_least(sref(slab, "concreteThicknessM"), length(slab.concrete_thickness_m), length(slab.concrete_thickness_m * (m_ed / m_rd.max(1.0)).sqrt() * 1.05), lc("Increase the slab thickness.", "Plattendicke erhöhen.")));
    }
    report.push(mb.build());

    let ls = part_slab::shear_span_m(slab);
    let v_l = part_slab::longitudinal_shear_mk_n_per_m(slab);
    let mut mk = head(a, format!("en1994.9.7.3.mk.{}", slab.id), "1994-1-1", "9.7.3", sref(slab, "mFactor"), "Composite slab longitudinal shear (m–k)", "Längsschub Verbunddecke (m–k)")
        .utilization(force(v_ed), force(v_l))
        .explanation(lc(format!("V_Ed = {:.1} kN/m, V_l,Rd = {:.1} kN/m (L_s = {:.2} m, γ_VS = 1.25).", v_ed / 1e3, v_l / 1e3, ls), format!("V_Ed = {:.1} kN/m, V_l,Rd = {:.1} kN/m (L_s = {:.2} m, γ_VS = 1,25).", v_ed / 1e3, v_l / 1e3, ls)));
    if v_ed > v_l {
        mk = mk.remedy(Remedy::at_least(sref(slab, "sheeting.thicknessM"), length(slab.sheeting.thickness_m), length(slab.sheeting.thickness_m * v_ed / v_l.max(1.0) * 1.05), lc("Increase the sheeting thickness (A_p).", "Profilblechdicke (A_p) erhöhen.")));
        if let Some(rem) = slab_load_fix(slab, v_l / v_ed.max(1.0) * 0.95) {
            mk = mk.remedy(rem);
        }
    }
    report.push(mk.build());

    let v_rd = part_slab::vertical_shear_resistance_n_per_m(slab, a);
    let mut vv = head(a, format!("en1994.9.7.5.vrd.{}", slab.id), "1994-1-1", "9.7.5", sref(slab, "concreteThicknessM"), "Composite slab vertical shear", "Querkraft Verbunddecke")
        .utilization(force(v_ed), force(v_rd))
        .explanation(lc(format!("V_Ed = {:.1} kN/m, V_v,Rd = {:.1} kN/m (b_o = {:.2} m/m).", v_ed / 1e3, v_rd / 1e3, part_slab::rib_width_sum_m_per_m(slab)), format!("V_Ed = {:.1} kN/m, V_v,Rd = {:.1} kN/m (b_o = {:.2} m/m).", v_ed / 1e3, v_rd / 1e3, part_slab::rib_width_sum_m_per_m(slab))));
    if v_ed > v_rd {
        vv = vv.remedy(Remedy::at_least(sref(slab, "concreteThicknessM"), length(slab.concrete_thickness_m), length(slab.concrete_thickness_m * v_ed / v_rd.max(1.0) * 1.05), lc("Increase the slab thickness.", "Plattendicke erhöhen.")));
    }
    report.push(vv.build());

    let point: Vec<&SlabAction> = slab.actions.iter().filter(|x| x.f_k_n.abs() > 1.0).collect();
    if !point.is_empty() {
        let f_ed = point.iter().map(|x| p_q(a, doc.reliability_class) * x.f_k_n.abs()).fold(0.0, f64::max);
        let v_p = cat::slab_punching(cat::constants::PUNCHING_FOOTPRINT_M, hc, sec.dp(), sec.ap, part_slab::rib_width_sum_m_per_m(slab), slab.f_ck_pa, p.gamma_c);
        let mut pc = head(a, format!("en1994.9.7.6.punching.{}", slab.id), "1994-1-1", "9.7.6", sref(slab, "concreteThicknessM"), "Composite slab punching shear", "Durchstanzen Verbunddecke")
            .utilization(force(f_ed), force(v_p))
            .explanation(lc(format!("F_Ed = {:.1} kN on a {:.0} mm square footprint, V_p,Rd = {:.1} kN.", f_ed / 1e3, cat::constants::PUNCHING_FOOTPRINT_M * 1e3, v_p / 1e3), format!("F_Ed = {:.1} kN auf {:.0} mm Quadrat-Lasteinleitung, V_p,Rd = {:.1} kN.", f_ed / 1e3, cat::constants::PUNCHING_FOOTPRINT_M * 1e3, v_p / 1e3)));
        if f_ed > v_p {
            pc = pc.remedy(Remedy::at_least(sref(slab, "concreteThicknessM"), length(slab.concrete_thickness_m), length(slab.concrete_thickness_m * (f_ed / v_p.max(1.0)).sqrt() * 1.1), lc("Increase the slab thickness.", "Plattendicke erhöhen.")));
        }
        report.push(pc.build());
    }

    let ecm = cat::ecm_pa(slab.f_ck_pa);
    let n = cat::modular_ratio(ecm, cat::constants::PSI_L_PERMANENT, cat::constants::CREEP_PHI);
    let i_eff = cat::slab_effective_i(slab.concrete_thickness_m, hc, sec.ap, sec.e, n);
    let ch = part_en1990::sls_characteristic(&slab.statics_actions(), slab.span_m, slab.span_m, slab.support, 1.0, a, doc.reliability_class);
    let q = m_to_q(ch.m_nm, slab.span_m, cont);
    let delta = (if cont { 0.0054 } else { 5.0 / 384.0 }) * q * slab.span_m.powi(4) / (cat::constants::E_A * i_eff.max(1e-18));
    let divisor = semio_s_plugin_norm_compliance::ndp_number("ndp.en1994.slab.deflection-divisor", a);
    let lim = slab.span_m / divisor;
    let (det_h, det_hc) = (slab.concrete_thickness_m / 0.08, 0.05 / hc);
    let as_floor = if cont { 0.002 * hc } else { 0.0 };
    let det = det_h.recip().max(det_hc).max(if as_floor > 0.0 { as_floor / slab.as_m2_per_m.max(1e-12) } else { 0.0 });
    let u_sls = (delta / lim).max(det);
    let mut sls = head(a, format!("en1994.9.8.sls.{}", slab.id), "1994-1-1", "9.8", sref(slab, "concreteThicknessM"), "Composite slab serviceability and detailing", "Verbunddecke Gebrauchstauglichkeit und Konstruktion").ndp("ndp.en1994.slab.deflection-divisor")
        .utilization(dim(u_sls), dim(1.0))
        .explanation(lc(format!("{}: δ = {:.1} mm ≤ L/{:.0} = {:.1} mm (I_eff mean of cracked and uncracked); h ≥ 80 mm, h_c ≥ 50 mm{}.", ch.label_en, delta * 1e3, divisor, lim * 1e3, if cont { ", A_s ≥ 0.2 % h_c over supports" } else { "" }), format!("{}: δ = {:.1} mm ≤ L/{:.0} = {:.1} mm (I_eff Mittel aus gerissen und ungerissen); h ≥ 80 mm, h_c ≥ 50 mm{}.", ch.label_de, delta * 1e3, divisor, lim * 1e3, if cont { ", A_s ≥ 0,2 % h_c über Auflagern" } else { "" })));
    if u_sls > 1.0 {
        sls = sls.remedy(Remedy::at_least(sref(slab, "concreteThicknessM"), length(slab.concrete_thickness_m), length((slab.concrete_thickness_m * u_sls.cbrt() * 1.05).max(0.08).max(hp + 0.05)), lc("Increase the slab thickness.", "Plattendicke erhöhen.")));
        if cont && as_floor > slab.as_m2_per_m {
            sls = sls.remedy(Remedy::at_least(sref(slab, "asM2PerM"), area(slab.as_m2_per_m), area(as_floor * 1.05), lc("Provide the minimum support reinforcement.", "Mindest-Stützbewehrung vorsehen.")));
        }
    }
    report.push(sls.build());
}

fn p_g(annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> f64 {
    part_en1990::gamma_g(annex, reliability_class)
}

fn p_q(annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> f64 {
    part_en1990::gamma_q(annex, reliability_class)
}
