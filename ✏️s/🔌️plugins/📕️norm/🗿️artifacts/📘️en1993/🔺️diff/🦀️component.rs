//! 🔺️ En1993 artifact — sparse field diff runtime.

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use super::schema::*;

use crate::artifacts::en1993::schema::En1993Artifact;
use crate::artifacts::en1993::En1993Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl En1993Diff {
    pub fn apply_to_artifact(&self, artifact: &En1993Artifact) -> En1993Artifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(value) = self.annex { next.annex = value; }
        if let Some(value) = self.n_ed_kn { next.n_ed_kn = value; }
        if let Some(value) = self.m_ed_knm { next.m_ed_knm = value; }
        if let Some(value) = self.v_ed_kn { next.v_ed_kn = value; }
        if let Some(value) = self.a_mm2 { next.a_mm2 = value; }
        if let Some(value) = self.a_v_mm2 { next.a_v_mm2 = value; }
        if let Some(value) = self.w_pl_mm3 { next.w_pl_mm3 = value; }
        if let Some(value) = self.f_y_mpa { next.f_y_mpa = value; }
        if let Some(value) = self.f_u_mpa { next.f_u_mpa = value; }
        if let Some(value) = self.chi { next.chi = value; }
        if let Some(value) = self.a_net_mm2 { next.a_net_mm2 = value; }
        if let Some(value) = self.tension_n_ed_kn { next.tension_n_ed_kn = value; }
        if let Some(value) = self.fire_thickness_mm { next.fire_thickness_mm = value; }
        if let Some(value) = &self.fire_rating { next.fire_rating = value.clone(); }
        if let Some(value) = self.fire_massivity { next.fire_massivity = value; }
        if let Some(value) = self.fire_mu_0 { next.fire_mu_0 = value; }
        if let Some(value) = self.fire_design_temperature_c { next.fire_design_temperature_c = value; }
        if let Some(value) = self.cf_b_bar_mm { next.cf_b_bar_mm = value; }
        if let Some(value) = self.cf_t_mm { next.cf_t_mm = value; }
        if let Some(value) = self.cf_k_sigma { next.cf_k_sigma = value; }
        if let Some(value) = self.cf_psi { next.cf_psi = value; }
        if let Some(value) = self.cf_n_ed_kn { next.cf_n_ed_kn = value; }
        if let Some(value) = self.cf_gross_resistance_kn { next.cf_gross_resistance_kn = value; }
        if let Some(value) = self.stainless_m_ed_knm { next.stainless_m_ed_knm = value; }
        if let Some(value) = self.stainless_w_pl_mm3 { next.stainless_w_pl_mm3 = value; }
        if let Some(value) = self.stainless_f_y_mpa { next.stainless_f_y_mpa = value; }
        if let Some(value) = self.plated_lambda_p { next.plated_lambda_p = value; }
        if let Some(value) = self.plated_sigma_ed_mpa { next.plated_sigma_ed_mpa = value; }
        if let Some(value) = self.silo_t_mm { next.silo_t_mm = value; }
        if let Some(value) = self.silo_r_mm { next.silo_r_mm = value; }
        if let Some(value) = self.shell_sigma_x_ed_mpa { next.shell_sigma_x_ed_mpa = value; }
        if let Some(value) = self.silo_k { next.silo_k = value; }
        if let Some(value) = self.silo_gamma_kn_m3 { next.silo_gamma_kn_m3 = value; }
        if let Some(value) = self.silo_depth_m { next.silo_depth_m = value; }
        if let Some(value) = self.bolt_f_ed_kn { next.bolt_f_ed_kn = value; }
        if let Some(value) = self.bolt_n_bolts { next.bolt_n_bolts = value; }
        if let Some(value) = self.bolt_a_s_mm2 { next.bolt_a_s_mm2 = value; }
        if let Some(value) = self.bolt_e1_mm { next.bolt_e1_mm = value; }
        if let Some(value) = self.bolt_e2_mm { next.bolt_e2_mm = value; }
        if let Some(value) = self.bolt_d0_mm { next.bolt_d0_mm = value; }
        if let Some(value) = self.bolt_d_mm { next.bolt_d_mm = value; }
        if let Some(value) = self.bolt_t_mm { next.bolt_t_mm = value; }
        if let Some(value) = self.bolt_f_u_mpa { next.bolt_f_u_mpa = value; }
        if let Some(value) = self.bolt_f_ub_mpa { next.bolt_f_ub_mpa = value; }
        if let Some(value) = self.weld_a_mm { next.weld_a_mm = value; }
        if let Some(value) = self.weld_l_mm { next.weld_l_mm = value; }
        if let Some(value) = self.weld_f_u_mpa { next.weld_f_u_mpa = value; }
        if let Some(value) = &self.weld_steel_grade { next.weld_steel_grade = value.clone(); }
        if let Some(value) = self.weld_f_ed_kn { next.weld_f_ed_kn = value; }
        if let Some(value) = self.delta_sigma_mpa { next.delta_sigma_mpa = value; }
        if let Some(value) = self.fatigue_category { next.fatigue_category = value; }
        if let Some(value) = &self.fatigue_method { next.fatigue_method = value.clone(); }
        if let Some(value) = &self.t10_steel_subgrade { next.t10_steel_subgrade = value.clone(); }
        if let Some(value) = self.t10_actual_thickness_mm { next.t10_actual_thickness_mm = value; }
        if let Some(value) = self.t10_t_ed_c { next.t10_t_ed_c = value; }
        if let Some(value) = self.tension_component_f_uk_kn { next.tension_component_f_uk_kn = value; }
        if let Some(value) = self.tension_component_f_k_kn { next.tension_component_f_k_kn = value; }
        if let Some(value) = self.tension_component_n_ed_kn { next.tension_component_n_ed_kn = value; }
        if let Some(value) = self.hss_w_el_mm3 { next.hss_w_el_mm3 = value; }
        if let Some(value) = self.hss_f_y_mpa { next.hss_f_y_mpa = value; }
        if let Some(value) = self.hss_section_class { next.hss_section_class = value; }
        if let Some(value) = self.hss_m_ed_knm { next.hss_m_ed_knm = value; }
        if let Some(value) = self.bridge_lambda { next.bridge_lambda = value; }
        if let Some(value) = self.bridge_phi_2 { next.bridge_phi_2 = value; }
        if let Some(value) = self.bridge_delta_sigma_p_mpa { next.bridge_delta_sigma_p_mpa = value; }
        if let Some(value) = self.tower_wind_factor { next.tower_wind_factor = value; }
        if let Some(value) = self.tower_n_ed_kn { next.tower_n_ed_kn = value; }
        if let Some(value) = self.pile_sigma_mpa { next.pile_sigma_mpa = value; }
        if let Some(value) = self.pile_k_red { next.pile_k_red = value; }
        if let Some(value) = self.pile_n_ed_kn { next.pile_n_ed_kn = value; }
        if let Some(value) = self.crane_f_z_ed_kn { next.crane_f_z_ed_kn = value; }
        if let Some(value) = self.crane_wheel_contact_length_mm { next.crane_wheel_contact_length_mm = value; }
        if let Some(value) = self.crane_dispersion_mm { next.crane_dispersion_mm = value; }
        if let Some(value) = self.crane_t_w_mm { next.crane_t_w_mm = value; }
        if let Some(value) = &self.selected_check_index {
            next.selected_check_index = *value;
        }
        next
    }
}

impl MutationDiff<En1993Snapshot> for En1993Diff {
    fn apply(&self, snapshot: &En1993Snapshot) -> En1993Snapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(value) = self.annex { next.annex = value; }
        if let Some(value) = self.n_ed_kn { next.n_ed_kn = value; }
        if let Some(value) = self.m_ed_knm { next.m_ed_knm = value; }
        if let Some(value) = self.v_ed_kn { next.v_ed_kn = value; }
        if let Some(value) = self.a_mm2 { next.a_mm2 = value; }
        if let Some(value) = self.a_v_mm2 { next.a_v_mm2 = value; }
        if let Some(value) = self.w_pl_mm3 { next.w_pl_mm3 = value; }
        if let Some(value) = self.f_y_mpa { next.f_y_mpa = value; }
        if let Some(value) = self.f_u_mpa { next.f_u_mpa = value; }
        if let Some(value) = self.chi { next.chi = value; }
        if let Some(value) = self.a_net_mm2 { next.a_net_mm2 = value; }
        if let Some(value) = self.tension_n_ed_kn { next.tension_n_ed_kn = value; }
        if let Some(value) = self.fire_thickness_mm { next.fire_thickness_mm = value; }
        if let Some(value) = &self.fire_rating { next.fire_rating = value.clone(); }
        if let Some(value) = self.fire_massivity { next.fire_massivity = value; }
        if let Some(value) = self.fire_mu_0 { next.fire_mu_0 = value; }
        if let Some(value) = self.fire_design_temperature_c { next.fire_design_temperature_c = value; }
        if let Some(value) = self.cf_b_bar_mm { next.cf_b_bar_mm = value; }
        if let Some(value) = self.cf_t_mm { next.cf_t_mm = value; }
        if let Some(value) = self.cf_k_sigma { next.cf_k_sigma = value; }
        if let Some(value) = self.cf_psi { next.cf_psi = value; }
        if let Some(value) = self.cf_n_ed_kn { next.cf_n_ed_kn = value; }
        if let Some(value) = self.cf_gross_resistance_kn { next.cf_gross_resistance_kn = value; }
        if let Some(value) = self.stainless_m_ed_knm { next.stainless_m_ed_knm = value; }
        if let Some(value) = self.stainless_w_pl_mm3 { next.stainless_w_pl_mm3 = value; }
        if let Some(value) = self.stainless_f_y_mpa { next.stainless_f_y_mpa = value; }
        if let Some(value) = self.plated_lambda_p { next.plated_lambda_p = value; }
        if let Some(value) = self.plated_sigma_ed_mpa { next.plated_sigma_ed_mpa = value; }
        if let Some(value) = self.silo_t_mm { next.silo_t_mm = value; }
        if let Some(value) = self.silo_r_mm { next.silo_r_mm = value; }
        if let Some(value) = self.shell_sigma_x_ed_mpa { next.shell_sigma_x_ed_mpa = value; }
        if let Some(value) = self.silo_k { next.silo_k = value; }
        if let Some(value) = self.silo_gamma_kn_m3 { next.silo_gamma_kn_m3 = value; }
        if let Some(value) = self.silo_depth_m { next.silo_depth_m = value; }
        if let Some(value) = self.bolt_f_ed_kn { next.bolt_f_ed_kn = value; }
        if let Some(value) = self.bolt_n_bolts { next.bolt_n_bolts = value; }
        if let Some(value) = self.bolt_a_s_mm2 { next.bolt_a_s_mm2 = value; }
        if let Some(value) = self.bolt_e1_mm { next.bolt_e1_mm = value; }
        if let Some(value) = self.bolt_e2_mm { next.bolt_e2_mm = value; }
        if let Some(value) = self.bolt_d0_mm { next.bolt_d0_mm = value; }
        if let Some(value) = self.bolt_d_mm { next.bolt_d_mm = value; }
        if let Some(value) = self.bolt_t_mm { next.bolt_t_mm = value; }
        if let Some(value) = self.bolt_f_u_mpa { next.bolt_f_u_mpa = value; }
        if let Some(value) = self.bolt_f_ub_mpa { next.bolt_f_ub_mpa = value; }
        if let Some(value) = self.weld_a_mm { next.weld_a_mm = value; }
        if let Some(value) = self.weld_l_mm { next.weld_l_mm = value; }
        if let Some(value) = self.weld_f_u_mpa { next.weld_f_u_mpa = value; }
        if let Some(value) = &self.weld_steel_grade { next.weld_steel_grade = value.clone(); }
        if let Some(value) = self.weld_f_ed_kn { next.weld_f_ed_kn = value; }
        if let Some(value) = self.delta_sigma_mpa { next.delta_sigma_mpa = value; }
        if let Some(value) = self.fatigue_category { next.fatigue_category = value; }
        if let Some(value) = &self.fatigue_method { next.fatigue_method = value.clone(); }
        if let Some(value) = &self.t10_steel_subgrade { next.t10_steel_subgrade = value.clone(); }
        if let Some(value) = self.t10_actual_thickness_mm { next.t10_actual_thickness_mm = value; }
        if let Some(value) = self.t10_t_ed_c { next.t10_t_ed_c = value; }
        if let Some(value) = self.tension_component_f_uk_kn { next.tension_component_f_uk_kn = value; }
        if let Some(value) = self.tension_component_f_k_kn { next.tension_component_f_k_kn = value; }
        if let Some(value) = self.tension_component_n_ed_kn { next.tension_component_n_ed_kn = value; }
        if let Some(value) = self.hss_w_el_mm3 { next.hss_w_el_mm3 = value; }
        if let Some(value) = self.hss_f_y_mpa { next.hss_f_y_mpa = value; }
        if let Some(value) = self.hss_section_class { next.hss_section_class = value; }
        if let Some(value) = self.hss_m_ed_knm { next.hss_m_ed_knm = value; }
        if let Some(value) = self.bridge_lambda { next.bridge_lambda = value; }
        if let Some(value) = self.bridge_phi_2 { next.bridge_phi_2 = value; }
        if let Some(value) = self.bridge_delta_sigma_p_mpa { next.bridge_delta_sigma_p_mpa = value; }
        if let Some(value) = self.tower_wind_factor { next.tower_wind_factor = value; }
        if let Some(value) = self.tower_n_ed_kn { next.tower_n_ed_kn = value; }
        if let Some(value) = self.pile_sigma_mpa { next.pile_sigma_mpa = value; }
        if let Some(value) = self.pile_k_red { next.pile_k_red = value; }
        if let Some(value) = self.pile_n_ed_kn { next.pile_n_ed_kn = value; }
        if let Some(value) = self.crane_f_z_ed_kn { next.crane_f_z_ed_kn = value; }
        if let Some(value) = self.crane_wheel_contact_length_mm { next.crane_wheel_contact_length_mm = value; }
        if let Some(value) = self.crane_dispersion_mm { next.crane_dispersion_mm = value; }
        if let Some(value) = self.crane_t_w_mm { next.crane_t_w_mm = value; }
        next
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
        take!(n_ed_kn);
        take!(m_ed_knm);
        take!(v_ed_kn);
        take!(a_mm2);
        take!(a_v_mm2);
        take!(w_pl_mm3);
        take!(f_y_mpa);
        take!(f_u_mpa);
        take!(chi);
        take!(a_net_mm2);
        take!(tension_n_ed_kn);
        take!(fire_thickness_mm);
        take!(fire_rating);
        take!(fire_massivity);
        take!(fire_mu_0);
        take!(fire_design_temperature_c);
        take!(cf_b_bar_mm);
        take!(cf_t_mm);
        take!(cf_k_sigma);
        take!(cf_psi);
        take!(cf_n_ed_kn);
        take!(cf_gross_resistance_kn);
        take!(stainless_m_ed_knm);
        take!(stainless_w_pl_mm3);
        take!(stainless_f_y_mpa);
        take!(plated_lambda_p);
        take!(plated_sigma_ed_mpa);
        take!(silo_t_mm);
        take!(silo_r_mm);
        take!(shell_sigma_x_ed_mpa);
        take!(silo_k);
        take!(silo_gamma_kn_m3);
        take!(silo_depth_m);
        take!(bolt_f_ed_kn);
        take!(bolt_n_bolts);
        take!(bolt_a_s_mm2);
        take!(bolt_e1_mm);
        take!(bolt_e2_mm);
        take!(bolt_d0_mm);
        take!(bolt_d_mm);
        take!(bolt_t_mm);
        take!(bolt_f_u_mpa);
        take!(bolt_f_ub_mpa);
        take!(weld_a_mm);
        take!(weld_l_mm);
        take!(weld_f_u_mpa);
        take!(weld_steel_grade);
        take!(weld_f_ed_kn);
        take!(delta_sigma_mpa);
        take!(fatigue_category);
        take!(fatigue_method);
        take!(t10_steel_subgrade);
        take!(t10_actual_thickness_mm);
        take!(t10_t_ed_c);
        take!(tension_component_f_uk_kn);
        take!(tension_component_f_k_kn);
        take!(tension_component_n_ed_kn);
        take!(hss_w_el_mm3);
        take!(hss_f_y_mpa);
        take!(hss_section_class);
        take!(hss_m_ed_knm);
        take!(bridge_lambda);
        take!(bridge_phi_2);
        take!(bridge_delta_sigma_p_mpa);
        take!(tower_wind_factor);
        take!(tower_n_ed_kn);
        take!(pile_sigma_mpa);
        take!(pile_k_red);
        take!(pile_n_ed_kn);
        take!(crane_f_z_ed_kn);
        take!(crane_wheel_contact_length_mm);
        take!(crane_dispersion_mm);
        take!(crane_t_w_mm);
        take!(selected_check_index);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn diff_set_snapshot(snapshot: &En1993Snapshot) -> En1993Diff {
    En1993Diff {
        artifact: Some(Box::new(En1993Artifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}
//#endregion 🔖️Helpers
