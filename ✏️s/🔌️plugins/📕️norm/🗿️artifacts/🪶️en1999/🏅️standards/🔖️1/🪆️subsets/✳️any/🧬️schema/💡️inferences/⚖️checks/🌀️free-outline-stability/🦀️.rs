//! 🌀️ EN 1999 free outline stability: torsional and torsional-flexural buckling (§6.3.1.4) and the member interaction (§6.3.3) from the principal-axis frames of `section_solver::stability`, which also holds the Wagner-effect lateral-torsional buckling moment.

use super::*;
use super::part_en1990::GoverningEffects;
use semio_s_artifact_norm_en1999_section_solver::stability;

//#region 🔖️ComplianceHelpers
fn ratio(x: f64, r: f64) -> f64 {
    if x.abs() < 1e-12 { 0.0 } else if r <= 0.0 { 1.0e6 } else { (x.abs() / r).min(1.0e6) }
}

/// 🌀️ Resistances the stability checks of a free outline share with the cross-section checks.
pub struct FreeResistances { pub class: part_1_1::SectionClass, pub a_eff: f64, pub n_b_y: f64, pub n_b_z: f64, pub chi_z: f64, pub m_rd_y: f64, pub m_rd_z: f64, pub m_b_rd: f64, pub omega_lt: f64 }

/// 🌀️ Torsional and torsional-flexural buckling (§6.3.1.4: χ_T for λ̄_T = √(A_eff·f_o/N_cr), N_cr the smaller of N_cr,T and N_cr,TF; unverified curve of `torsional_curve`) and the member interaction (§6.3.3, Eq. 6.59 and 6.60 with the smaller of the flexural and torsional resistances) of one member on a free outline.
pub fn free_stability(member: &AluminiumMember, k: &FreeConstants, r: &FreeResistances, alloy: &part_1_1::AlloyProps, annex: AnnexChoice, params: &na_de::AnnexParams, gov: &GoverningEffects) -> Vec<CheckResult> {
    let (fo, g1) = (alloy.f_o_pa, params.gamma_m1);
    let label = member_label(&member.id);
    let tag = format!(" (ULS {}, lead {}, free outline)", gov.combination, gov.action_id);
    let tag_de = tag.replace("ULS", "GZT").replace("lead", "führend").replace("free outline", "freier Umriss");
    let k_sup = match member.support { SupportCondition::Continuous => 0.7, SupportCondition::Cantilever => 2.0, SupportCondition::SimplySupported => 1.0 };
    let pick = |l: f64| if l > 0.0 { l } else { k_sup * member.length };
    let (l_y, l_z, l_t) = (pick(member.buckling_length_y), pick(member.buckling_length_z), pick(member.buckling_length_t));
    let (n_cr_t, n_cr_tf) = stability::n_cr_tf(na_de::E_PA, na_de::G_PA, r.a_eff, &k.frames.near, k.it, k.iw, l_z, l_y, l_t);
    let lam_t = (r.a_eff * fo / n_cr_tf.max(1e-18)).sqrt();
    let (alpha_t, lambda_0_t) = part_1_1::torsional_curve(alloy.buckling_class_a);
    let chi_t = part_1_1::chi_from_lambda(lam_t, alpha_t, lambda_0_t);
    let omega = if member.transverse_weld { part_1_1::omega_x(chi_t, alloy.rho_o_haz, member.transverse_weld_offset, l_t, 1.0) } else { 1.0 };
    let n_b_t = omega * chi_t * r.a_eff * fo / g1;
    let nc = (-gov.n_ed).max(0.0);
    let n_path = format!("members[id={}].actions[id={}].nK", member.id, gov.action_id);
    let mut out = Vec::new();
    let rem = leaf_remedies("members", &member.id, &member.actions, nc / n_b_t.max(1e-18), &label, "Reduce the compression force (torsional-flexural buckling)", "Druckkraft senken (Biegedrillknicken, Drillknicken)", "nK");
    let text = format!("N_cr,T = {:.1} kN, N_cr,TF = {:.1} kN (shear centre offset y0 = {:.2} mm, z0 = {:.2} mm), λ̄_T = {lam_t:.4}, χ_T = {chi_t:.4}, N_b,T,Rd = {:.1} kN", n_cr_t.min(1.0e12) / 1e3, n_cr_tf.min(1.0e12) / 1e3, k.frames.near.y0 * 1e3, k.frames.near.z0 * 1e3, n_b_t / 1e3);
    out.push(utilization_check(format!("en1999.6.3.1.tb.{}", member.id), "DIN EN 1999-1-1", clause("6.3.1.4"), SubjectRef::new(&member.id, format!("members[id={}].bucklingLengthT", member.id), label.clone()), loc("Torsional and torsional-flexural buckling N_b,T,Rd", "Drill- und Biegedrillknicken N_b,T,Rd"), q_force(nc), q_force(n_b_t), annex, loc(&format!("{text}{tag}"), &format!("{text}{tag_de}")), rem));
    let (n_y, n_z) = (r.n_b_y.min(n_b_t), r.n_b_z.min(n_b_t));
    let (eta_c, gamma_c, xi_c) = part_1_1::member_interaction_exponents(r.class, r.chi_z);
    let eq59 = ratio(nc, n_y).powf(eta_c) + ratio(gov.m_y_ed, r.omega_lt * r.m_rd_y).powf(gamma_c) + ratio(gov.m_z_ed, r.m_rd_z).powf(xi_c);
    let eq60 = ratio(nc, n_z).powf(eta_c) + ratio(gov.m_y_ed, r.m_b_rd).powf(gamma_c) + ratio(gov.m_z_ed, r.m_rd_z).powf(xi_c);
    let u = eq59.max(eq60);
    let rem = leaf_remedies("members", &member.id, &member.actions, u, &label, "Reduce the combined compression and bending demand", "Druck und Biegung gemeinsam senken", "nK");
    let text_en = format!("Eq. 6.59 = {eq59:.3}, Eq. 6.60 = {eq60:.3} with η_c = {eta_c:.2}, γ_c = {gamma_c:.2}, ξ_c = {xi_c:.2} (η_0, γ_0, ξ_0 unverified), N_b,Rd = min(flexural, N_b,T,Rd = {:.1} kN)", n_b_t / 1e3);
    let text_de = format!("Gl. 6.59 = {eq59:.3}, Gl. 6.60 = {eq60:.3} mit η_c = {eta_c:.2}, γ_c = {gamma_c:.2}, ξ_c = {xi_c:.2} (η_0, γ_0, ξ_0 ungeprüft), N_b,Rd = min(Biegeknicken, N_b,T,Rd = {:.1} kN)", n_b_t / 1e3);
    out.push(utilization_check(format!("en1999.6.3.3.int.{}", member.id), "DIN EN 1999-1-1", clause("6.3.3"), SubjectRef::new(&member.id, n_path, label), loc("Member interaction (6.59/6.60)", "Bauteilinteraktion (6.59/6.60)"), q_dim(u), q_dim(1.0), annex, loc(&format!("{text_en}{tag}"), &format!("{text_de}{tag_de}")), rem));
    out
}
//#endregion 🔖️ComplianceHelpers
