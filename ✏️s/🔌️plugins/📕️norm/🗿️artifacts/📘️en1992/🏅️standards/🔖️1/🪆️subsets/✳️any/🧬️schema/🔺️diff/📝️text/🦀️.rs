//! 🔺️ En1992 artifact — sparse field diff runtime.

use crate::artifacts::en1992::schema::diff::*;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::en1992::schema::En1992Artifact;
use crate::artifacts::en1992::En1992Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl En1992Diff {
    pub fn apply_to_artifact(&self, artifact: &En1992Artifact) -> protocol::MutationApplyResult<En1992Artifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(value) = &self.annex {
                next.annex = value.clone();
            }
            if let Some(value) = &self.m_ed_knm {
                next.m_ed_knm = value.clone();
            }
            if let Some(value) = &self.v_ed_kn {
                next.v_ed_kn = value.clone();
            }
            if let Some(value) = &self.f_ck {
                next.f_ck = value.clone();
            }
            if let Some(value) = &self.b_mm {
                next.b_mm = value.clone();
            }
            if let Some(value) = &self.d_mm {
                next.d_mm = value.clone();
            }
            if let Some(value) = &self.a_s_mm2 {
                next.a_s_mm2 = value.clone();
            }
            if let Some(value) = &self.f_yk {
                next.f_yk = value.clone();
            }
            if let Some(value) = &self.rho_l {
                next.rho_l = value.clone();
            }
            if let Some(value) = &self.n_ed_kn {
                next.n_ed_kn = value.clone();
            }
            if let Some(value) = &self.p_kn {
                next.p_kn = value.clone();
            }
            if let Some(value) = &self.a_c_mm2 {
                next.a_c_mm2 = value.clone();
            }
            if let Some(value) = &self.use_fem {
                next.use_fem = value.clone();
            }
            if let Some(value) = &self.span_m {
                next.span_m = value.clone();
            }
            if let Some(value) = &self.udl_kn_m {
                next.udl_kn_m = value.clone();
            }
            if let Some(value) = &self.fire_rating {
                next.fire_rating = value.clone();
            }
            if let Some(value) = &self.provided_axis_distance_mm {
                next.provided_axis_distance_mm = value.clone();
            }
            if let Some(value) = &self.bridge_sigma_c_mpa {
                next.bridge_sigma_c_mpa = value.clone();
            }
            if let Some(value) = &self.bridge_delta_sigma_s_mpa {
                next.bridge_delta_sigma_s_mpa = value.clone();
            }
            if let Some(value) = &self.tightness_class {
                next.tightness_class = value.clone();
            }
            if let Some(value) = &self.hd_over_h {
                next.hd_over_h = value.clone();
            }
            if let Some(value) = &self.liquid_sigma_s_mpa {
                next.liquid_sigma_s_mpa = value.clone();
            }
            if let Some(value) = &self.liquid_rho_p_eff {
                next.liquid_rho_p_eff = value.clone();
            }
            if let Some(value) = &self.liquid_f_ct_eff_mpa {
                next.liquid_f_ct_eff_mpa = value.clone();
            }
            if let Some(value) = &self.liquid_e_s_mpa {
                next.liquid_e_s_mpa = value.clone();
            }
            if let Some(value) = &self.liquid_s_r_max_mm {
                next.liquid_s_r_max_mm = value.clone();
            }
            if let Some(value) = &self.anchor_h_ef_mm {
                next.anchor_h_ef_mm = value.clone();
            }
            if let Some(value) = &self.anchor_cracked {
                next.anchor_cracked = value.clone();
            }
            if let Some(value) = &self.anchor_f_uk_mpa {
                next.anchor_f_uk_mpa = value.clone();
            }
            if let Some(value) = &self.anchor_f_yk_mpa {
                next.anchor_f_yk_mpa = value.clone();
            }
            if let Some(value) = &self.anchor_a_s_mm2 {
                next.anchor_a_s_mm2 = value.clone();
            }
            if let Some(value) = &self.anchor_d_mm {
                next.anchor_d_mm = value.clone();
            }
            if let Some(value) = &self.anchor_c1_mm {
                next.anchor_c1_mm = value.clone();
            }
            if let Some(value) = &self.anchor_n_ed_kn {
                next.anchor_n_ed_kn = value.clone();
            }
            if let Some(value) = &self.anchor_v_ed_kn {
                next.anchor_v_ed_kn = value.clone();
            }
            if let Some(value) = &self.selected_check_index {
                next.selected_check_index = *value;
            }
            next
        })
    }
}

impl MutationDiff<En1992Snapshot> for En1992Diff {
    fn apply(&self, snapshot: &En1992Snapshot) -> protocol::MutationApplyResult<En1992Snapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(value) = &self.annex {
                next.annex = value.clone();
            }
            if let Some(value) = &self.m_ed_knm {
                next.m_ed_knm = value.clone();
            }
            if let Some(value) = &self.v_ed_kn {
                next.v_ed_kn = value.clone();
            }
            if let Some(value) = &self.f_ck {
                next.f_ck = value.clone();
            }
            if let Some(value) = &self.b_mm {
                next.b_mm = value.clone();
            }
            if let Some(value) = &self.d_mm {
                next.d_mm = value.clone();
            }
            if let Some(value) = &self.a_s_mm2 {
                next.a_s_mm2 = value.clone();
            }
            if let Some(value) = &self.f_yk {
                next.f_yk = value.clone();
            }
            if let Some(value) = &self.rho_l {
                next.rho_l = value.clone();
            }
            if let Some(value) = &self.n_ed_kn {
                next.n_ed_kn = value.clone();
            }
            if let Some(value) = &self.p_kn {
                next.p_kn = value.clone();
            }
            if let Some(value) = &self.a_c_mm2 {
                next.a_c_mm2 = value.clone();
            }
            if let Some(value) = &self.use_fem {
                next.use_fem = value.clone();
            }
            if let Some(value) = &self.span_m {
                next.span_m = value.clone();
            }
            if let Some(value) = &self.udl_kn_m {
                next.udl_kn_m = value.clone();
            }
            if let Some(value) = &self.fire_rating {
                next.fire_rating = value.clone();
            }
            if let Some(value) = &self.provided_axis_distance_mm {
                next.provided_axis_distance_mm = value.clone();
            }
            if let Some(value) = &self.bridge_sigma_c_mpa {
                next.bridge_sigma_c_mpa = value.clone();
            }
            if let Some(value) = &self.bridge_delta_sigma_s_mpa {
                next.bridge_delta_sigma_s_mpa = value.clone();
            }
            if let Some(value) = &self.tightness_class {
                next.tightness_class = value.clone();
            }
            if let Some(value) = &self.hd_over_h {
                next.hd_over_h = value.clone();
            }
            if let Some(value) = &self.liquid_sigma_s_mpa {
                next.liquid_sigma_s_mpa = value.clone();
            }
            if let Some(value) = &self.liquid_rho_p_eff {
                next.liquid_rho_p_eff = value.clone();
            }
            if let Some(value) = &self.liquid_f_ct_eff_mpa {
                next.liquid_f_ct_eff_mpa = value.clone();
            }
            if let Some(value) = &self.liquid_e_s_mpa {
                next.liquid_e_s_mpa = value.clone();
            }
            if let Some(value) = &self.liquid_s_r_max_mm {
                next.liquid_s_r_max_mm = value.clone();
            }
            if let Some(value) = &self.anchor_h_ef_mm {
                next.anchor_h_ef_mm = value.clone();
            }
            if let Some(value) = &self.anchor_cracked {
                next.anchor_cracked = value.clone();
            }
            if let Some(value) = &self.anchor_f_uk_mpa {
                next.anchor_f_uk_mpa = value.clone();
            }
            if let Some(value) = &self.anchor_f_yk_mpa {
                next.anchor_f_yk_mpa = value.clone();
            }
            if let Some(value) = &self.anchor_a_s_mm2 {
                next.anchor_a_s_mm2 = value.clone();
            }
            if let Some(value) = &self.anchor_d_mm {
                next.anchor_d_mm = value.clone();
            }
            if let Some(value) = &self.anchor_c1_mm {
                next.anchor_c1_mm = value.clone();
            }
            if let Some(value) = &self.anchor_n_ed_kn {
                next.anchor_n_ed_kn = value.clone();
            }
            if let Some(value) = &self.anchor_v_ed_kn {
                next.anchor_v_ed_kn = value.clone();
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(annex);
        take!(m_ed_knm);
        take!(v_ed_kn);
        take!(f_ck);
        take!(b_mm);
        take!(d_mm);
        take!(a_s_mm2);
        take!(f_yk);
        take!(rho_l);
        take!(n_ed_kn);
        take!(p_kn);
        take!(a_c_mm2);
        take!(use_fem);
        take!(span_m);
        take!(udl_kn_m);
        take!(fire_rating);
        take!(provided_axis_distance_mm);
        take!(bridge_sigma_c_mpa);
        take!(bridge_delta_sigma_s_mpa);
        take!(tightness_class);
        take!(hd_over_h);
        take!(liquid_sigma_s_mpa);
        take!(liquid_rho_p_eff);
        take!(liquid_f_ct_eff_mpa);
        take!(liquid_e_s_mpa);
        take!(liquid_s_r_max_mm);
        take!(anchor_h_ef_mm);
        take!(anchor_cracked);
        take!(anchor_f_uk_mpa);
        take!(anchor_f_yk_mpa);
        take!(anchor_a_s_mm2);
        take!(anchor_d_mm);
        take!(anchor_c1_mm);
        take!(anchor_n_ed_kn);
        take!(anchor_v_ed_kn);
        take!(selected_check_index);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn diff_set_snapshot(snapshot: &En1992Snapshot) -> En1992Diff {
    En1992Diff { artifact: Some(Box::new(En1992Artifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}
//#endregion 🔖️Helpers
