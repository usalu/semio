
use super::*;
use protocol::{Mutation, MutationDiff, SemanticMutation};

/// 🔁️ Applies `operation`'s diff to `base`, then applies every mutation in its inverse (computed
/// from `base`) back onto the forward result — asserts the base fixture is exactly restored, and
/// returns the forward (post-mutation) snapshot for the caller's own field assertions.
fn round_trip(base: &En1993Snapshot, operation: &En1993Mutation) -> En1993Snapshot {
    let forward = operation.diff(base).diff().apply(base).expect("valid mutation diff");
    let backwards = operation.inverse(base);
    let mut restored = forward.clone();
    for back in &backwards {
        restored = back.diff(base).diff().apply(&restored).expect("valid mutation diff");
    }
    assert_eq!(&restored, base, "inverse must exactly restore the pre-operation fixture");
    forward
}

#[semio_framework_async_macros::async_test]
async fn change_annex_round_trips() {
    let base = En1993Snapshot::default();
    let mutation = En1993Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.annex, crate::document::AnnexChoice::En, "annex must take the new value after change-annex");
}

#[semio_framework_async_macros::async_test]
async fn update_member_properties_round_trips() {
    let base = En1993Snapshot::default();
    let mutation = En1993Mutation::UpdateMemberProperties(update_member_properties::UpdateMemberProperties {
        new_n_ed_kn: 999.0,
        new_m_ed_knm: 999.0,
        new_v_ed_kn: 999.0,
        new_a_mm2: 999.0,
        new_a_v_mm2: 999.0,
        new_w_pl_mm3: 999.0,
        new_f_y_mpa: 999.0,
        new_f_u_mpa: 999.0,
        new_chi: 999.0,
        new_a_net_mm2: 999.0,
        new_tension_n_ed_kn: 999.0,
    });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.n_ed_kn, 999.0, "n_ed_kn must take the new value after update-member-properties");
    assert_eq!(after.m_ed_knm, 999.0, "m_ed_knm must take the new value after update-member-properties");
    assert_eq!(after.v_ed_kn, 999.0, "v_ed_kn must take the new value after update-member-properties");
    assert_eq!(after.a_mm2, 999.0, "a_mm2 must take the new value after update-member-properties");
    assert_eq!(after.a_v_mm2, 999.0, "a_v_mm2 must take the new value after update-member-properties");
    assert_eq!(after.w_pl_mm3, 999.0, "w_pl_mm3 must take the new value after update-member-properties");
    assert_eq!(after.f_y_mpa, 999.0, "f_y_mpa must take the new value after update-member-properties");
    assert_eq!(after.f_u_mpa, 999.0, "f_u_mpa must take the new value after update-member-properties");
    assert_eq!(after.chi, 999.0, "chi must take the new value after update-member-properties");
    assert_eq!(after.a_net_mm2, 999.0, "a_net_mm2 must take the new value after update-member-properties");
    assert_eq!(after.tension_n_ed_kn, 999.0, "tension_n_ed_kn must take the new value after update-member-properties");
}

#[semio_framework_async_macros::async_test]
async fn update_fire_inputs_round_trips() {
    let base = En1993Snapshot::default();
    let mutation = En1993Mutation::UpdateFireInputs(update_fire_inputs::UpdateFireInputs { new_fire_thickness_mm: 999.0, new_fire_rating: "changed".to_string(), new_fire_massivity: 999.0, new_fire_mu_0: 999.0, new_fire_design_temperature_c: 999.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.fire_thickness_mm, 999.0, "fire_thickness_mm must take the new value after update-fire-inputs");
    assert_eq!(after.fire_rating, "changed".to_string(), "fire_rating must take the new value after update-fire-inputs");
    assert_eq!(after.fire_massivity, 999.0, "fire_massivity must take the new value after update-fire-inputs");
    assert_eq!(after.fire_mu_0, 999.0, "fire_mu_0 must take the new value after update-fire-inputs");
    assert_eq!(after.fire_design_temperature_c, 999.0, "fire_design_temperature_c must take the new value after update-fire-inputs");
}

#[semio_framework_async_macros::async_test]
async fn update_cold_formed_inputs_round_trips() {
    let base = En1993Snapshot::default();
    let mutation =
        En1993Mutation::UpdateColdFormedInputs(update_cold_formed_inputs::UpdateColdFormedInputs { new_cf_b_bar_mm: 999.0, new_cf_t_mm: 999.0, new_cf_k_sigma: 999.0, new_cf_psi: 999.0, new_cf_n_ed_kn: 999.0, new_cf_gross_resistance_kn: 999.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.cf_b_bar_mm, 999.0, "cf_b_bar_mm must take the new value after update-cold-formed-inputs");
    assert_eq!(after.cf_t_mm, 999.0, "cf_t_mm must take the new value after update-cold-formed-inputs");
    assert_eq!(after.cf_k_sigma, 999.0, "cf_k_sigma must take the new value after update-cold-formed-inputs");
    assert_eq!(after.cf_psi, 999.0, "cf_psi must take the new value after update-cold-formed-inputs");
    assert_eq!(after.cf_n_ed_kn, 999.0, "cf_n_ed_kn must take the new value after update-cold-formed-inputs");
    assert_eq!(after.cf_gross_resistance_kn, 999.0, "cf_gross_resistance_kn must take the new value after update-cold-formed-inputs");
}

#[semio_framework_async_macros::async_test]
async fn update_stainless_inputs_round_trips() {
    let base = En1993Snapshot::default();
    let mutation = En1993Mutation::UpdateStainlessInputs(update_stainless_inputs::UpdateStainlessInputs { new_stainless_m_ed_knm: 999.0, new_stainless_w_pl_mm3: 999.0, new_stainless_f_y_mpa: 999.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.stainless_m_ed_knm, 999.0, "stainless_m_ed_knm must take the new value after update-stainless-inputs");
    assert_eq!(after.stainless_w_pl_mm3, 999.0, "stainless_w_pl_mm3 must take the new value after update-stainless-inputs");
    assert_eq!(after.stainless_f_y_mpa, 999.0, "stainless_f_y_mpa must take the new value after update-stainless-inputs");
}

#[semio_framework_async_macros::async_test]
async fn update_plated_inputs_round_trips() {
    let base = En1993Snapshot::default();
    let mutation = En1993Mutation::UpdatePlatedInputs(update_plated_inputs::UpdatePlatedInputs { new_plated_lambda_p: 999.0, new_plated_sigma_ed_mpa: 999.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.plated_lambda_p, 999.0, "plated_lambda_p must take the new value after update-plated-inputs");
    assert_eq!(after.plated_sigma_ed_mpa, 999.0, "plated_sigma_ed_mpa must take the new value after update-plated-inputs");
}

#[semio_framework_async_macros::async_test]
async fn update_silo_shell_inputs_round_trips() {
    let base = En1993Snapshot::default();
    let mutation =
        En1993Mutation::UpdateSiloShellInputs(update_silo_shell_inputs::UpdateSiloShellInputs { new_silo_t_mm: 999.0, new_silo_r_mm: 999.0, new_shell_sigma_x_ed_mpa: 999.0, new_silo_k: 999.0, new_silo_gamma_kn_m3: 999.0, new_silo_depth_m: 999.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.silo_t_mm, 999.0, "silo_t_mm must take the new value after update-silo-shell-inputs");
    assert_eq!(after.silo_r_mm, 999.0, "silo_r_mm must take the new value after update-silo-shell-inputs");
    assert_eq!(after.shell_sigma_x_ed_mpa, 999.0, "shell_sigma_x_ed_mpa must take the new value after update-silo-shell-inputs");
    assert_eq!(after.silo_k, 999.0, "silo_k must take the new value after update-silo-shell-inputs");
    assert_eq!(after.silo_gamma_kn_m3, 999.0, "silo_gamma_kn_m3 must take the new value after update-silo-shell-inputs");
    assert_eq!(after.silo_depth_m, 999.0, "silo_depth_m must take the new value after update-silo-shell-inputs");
}

#[semio_framework_async_macros::async_test]
async fn update_bolt_inputs_round_trips() {
    let base = En1993Snapshot::default();
    let mutation = En1993Mutation::UpdateBoltInputs(update_bolt_inputs::UpdateBoltInputs {
        new_bolt_f_ed_kn: 999.0,
        new_bolt_n_bolts: 9,
        new_bolt_a_s_mm2: 999.0,
        new_bolt_e1_mm: 999.0,
        new_bolt_e2_mm: 999.0,
        new_bolt_d0_mm: 999.0,
        new_bolt_d_mm: 999.0,
        new_bolt_t_mm: 999.0,
        new_bolt_f_u_mpa: 999.0,
        new_bolt_f_ub_mpa: 999.0,
    });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.bolt_f_ed_kn, 999.0, "bolt_f_ed_kn must take the new value after update-bolt-inputs");
    assert_eq!(after.bolt_n_bolts, 9, "bolt_n_bolts must take the new value after update-bolt-inputs");
    assert_eq!(after.bolt_a_s_mm2, 999.0, "bolt_a_s_mm2 must take the new value after update-bolt-inputs");
    assert_eq!(after.bolt_e1_mm, 999.0, "bolt_e1_mm must take the new value after update-bolt-inputs");
    assert_eq!(after.bolt_e2_mm, 999.0, "bolt_e2_mm must take the new value after update-bolt-inputs");
    assert_eq!(after.bolt_d0_mm, 999.0, "bolt_d0_mm must take the new value after update-bolt-inputs");
    assert_eq!(after.bolt_d_mm, 999.0, "bolt_d_mm must take the new value after update-bolt-inputs");
    assert_eq!(after.bolt_t_mm, 999.0, "bolt_t_mm must take the new value after update-bolt-inputs");
    assert_eq!(after.bolt_f_u_mpa, 999.0, "bolt_f_u_mpa must take the new value after update-bolt-inputs");
    assert_eq!(after.bolt_f_ub_mpa, 999.0, "bolt_f_ub_mpa must take the new value after update-bolt-inputs");
}

#[semio_framework_async_macros::async_test]
async fn update_weld_inputs_round_trips() {
    let base = En1993Snapshot::default();
    let mutation = En1993Mutation::UpdateWeldInputs(update_weld_inputs::UpdateWeldInputs { new_weld_a_mm: 999.0, new_weld_l_mm: 999.0, new_weld_f_u_mpa: 999.0, new_weld_steel_grade: "changed".to_string(), new_weld_f_ed_kn: 999.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.weld_a_mm, 999.0, "weld_a_mm must take the new value after update-weld-inputs");
    assert_eq!(after.weld_l_mm, 999.0, "weld_l_mm must take the new value after update-weld-inputs");
    assert_eq!(after.weld_f_u_mpa, 999.0, "weld_f_u_mpa must take the new value after update-weld-inputs");
    assert_eq!(after.weld_steel_grade, "changed".to_string(), "weld_steel_grade must take the new value after update-weld-inputs");
    assert_eq!(after.weld_f_ed_kn, 999.0, "weld_f_ed_kn must take the new value after update-weld-inputs");
}

#[semio_framework_async_macros::async_test]
async fn update_fatigue_inputs_round_trips() {
    let base = En1993Snapshot::default();
    let mutation = En1993Mutation::UpdateFatigueInputs(update_fatigue_inputs::UpdateFatigueInputs { new_delta_sigma_mpa: 999.0, new_fatigue_category: 9, new_fatigue_method: "changed".to_string() });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.delta_sigma_mpa, 999.0, "delta_sigma_mpa must take the new value after update-fatigue-inputs");
    assert_eq!(after.fatigue_category, 9, "fatigue_category must take the new value after update-fatigue-inputs");
    assert_eq!(after.fatigue_method, "changed".to_string(), "fatigue_method must take the new value after update-fatigue-inputs");
}

#[semio_framework_async_macros::async_test]
async fn update_through_thickness_inputs_round_trips() {
    let base = En1993Snapshot::default();
    let mutation = En1993Mutation::UpdateThroughThicknessInputs(update_through_thickness_inputs::UpdateThroughThicknessInputs { new_t10_steel_subgrade: "changed".to_string(), new_t10_actual_thickness_mm: 999.0, new_t10_t_ed_c: 999.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.t10_steel_subgrade, "changed".to_string(), "t10_steel_subgrade must take the new value after update-through-thickness-inputs");
    assert_eq!(after.t10_actual_thickness_mm, 999.0, "t10_actual_thickness_mm must take the new value after update-through-thickness-inputs");
    assert_eq!(after.t10_t_ed_c, 999.0, "t10_t_ed_c must take the new value after update-through-thickness-inputs");
}

#[semio_framework_async_macros::async_test]
async fn update_tension_component_inputs_round_trips() {
    let base = En1993Snapshot::default();
    let mutation = En1993Mutation::UpdateTensionComponentInputs(update_tension_component_inputs::UpdateTensionComponentInputs { new_tension_component_f_uk_kn: 999.0, new_tension_component_f_k_kn: 999.0, new_tension_component_n_ed_kn: 999.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.tension_component_f_uk_kn, 999.0, "tension_component_f_uk_kn must take the new value after update-tension-component-inputs");
    assert_eq!(after.tension_component_f_k_kn, 999.0, "tension_component_f_k_kn must take the new value after update-tension-component-inputs");
    assert_eq!(after.tension_component_n_ed_kn, 999.0, "tension_component_n_ed_kn must take the new value after update-tension-component-inputs");
}

#[semio_framework_async_macros::async_test]
async fn update_hss_inputs_round_trips() {
    let base = En1993Snapshot::default();
    let mutation = En1993Mutation::UpdateHssInputs(update_hss_inputs::UpdateHssInputs { new_hss_w_el_mm3: 999.0, new_hss_f_y_mpa: 999.0, new_hss_section_class: 9, new_hss_m_ed_knm: 999.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.hss_w_el_mm3, 999.0, "hss_w_el_mm3 must take the new value after update-hss-inputs");
    assert_eq!(after.hss_f_y_mpa, 999.0, "hss_f_y_mpa must take the new value after update-hss-inputs");
    assert_eq!(after.hss_section_class, 9, "hss_section_class must take the new value after update-hss-inputs");
    assert_eq!(after.hss_m_ed_knm, 999.0, "hss_m_ed_knm must take the new value after update-hss-inputs");
}

#[semio_framework_async_macros::async_test]
async fn update_bridge_inputs_round_trips() {
    let base = En1993Snapshot::default();
    let mutation = En1993Mutation::UpdateBridgeInputs(update_bridge_inputs::UpdateBridgeInputs { new_bridge_lambda: 999.0, new_bridge_phi_2: 999.0, new_bridge_delta_sigma_p_mpa: 999.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.bridge_lambda, 999.0, "bridge_lambda must take the new value after update-bridge-inputs");
    assert_eq!(after.bridge_phi_2, 999.0, "bridge_phi_2 must take the new value after update-bridge-inputs");
    assert_eq!(after.bridge_delta_sigma_p_mpa, 999.0, "bridge_delta_sigma_p_mpa must take the new value after update-bridge-inputs");
}

#[semio_framework_async_macros::async_test]
async fn update_tower_inputs_round_trips() {
    let base = En1993Snapshot::default();
    let mutation = En1993Mutation::UpdateTowerInputs(update_tower_inputs::UpdateTowerInputs { new_tower_wind_factor: 999.0, new_tower_n_ed_kn: 999.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.tower_wind_factor, 999.0, "tower_wind_factor must take the new value after update-tower-inputs");
    assert_eq!(after.tower_n_ed_kn, 999.0, "tower_n_ed_kn must take the new value after update-tower-inputs");
}

#[semio_framework_async_macros::async_test]
async fn update_pile_inputs_round_trips() {
    let base = En1993Snapshot::default();
    let mutation = En1993Mutation::UpdatePileInputs(update_pile_inputs::UpdatePileInputs { new_pile_sigma_mpa: 999.0, new_pile_k_red: 999.0, new_pile_n_ed_kn: 999.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.pile_sigma_mpa, 999.0, "pile_sigma_mpa must take the new value after update-pile-inputs");
    assert_eq!(after.pile_k_red, 999.0, "pile_k_red must take the new value after update-pile-inputs");
    assert_eq!(after.pile_n_ed_kn, 999.0, "pile_n_ed_kn must take the new value after update-pile-inputs");
}

#[semio_framework_async_macros::async_test]
async fn update_crane_inputs_round_trips() {
    let base = En1993Snapshot::default();
    let mutation = En1993Mutation::UpdateCraneInputs(update_crane_inputs::UpdateCraneInputs { new_crane_f_z_ed_kn: 999.0, new_crane_wheel_contact_length_mm: 999.0, new_crane_dispersion_mm: 999.0, new_crane_t_w_mm: 999.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.crane_f_z_ed_kn, 999.0, "crane_f_z_ed_kn must take the new value after update-crane-inputs");
    assert_eq!(after.crane_wheel_contact_length_mm, 999.0, "crane_wheel_contact_length_mm must take the new value after update-crane-inputs");
    assert_eq!(after.crane_dispersion_mm, 999.0, "crane_dispersion_mm must take the new value after update-crane-inputs");
    assert_eq!(after.crane_t_w_mm, 999.0, "crane_t_w_mm must take the new value after update-crane-inputs");
}

#[semio_framework_async_macros::async_test]
async fn change_annex_diff_is_sparse() {
    let base = En1993Snapshot::default();
    let mutation = En1993Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En });
    let outcome = mutation.diff(&base);
    let diff = outcome.diff();
    assert_eq!(diff.annex, Some(crate::document::AnnexChoice::En));
    assert_eq!(diff.n_ed_kn, None, "change-annex must not touch unrelated fields");
    assert_eq!(diff.bolt_f_ed_kn, None, "change-annex must not touch unrelated fields");
    assert_eq!(diff.weld_steel_grade, None, "change-annex must not touch unrelated fields");
}

#[semio_framework_async_macros::async_test]
async fn semantic_kinds_cover_every_variant() {
    assert_eq!(En1993Mutation::kinds().len(), 17);
    let mutation = En1993Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En });
    assert_eq!(mutation.semantics().kind, "change-annex");
    assert_eq!(mutation.semantics().record, "ChangedAnnex");
    assert_eq!(mutation.semantics().verb, "change");

    let bolt = En1993Mutation::UpdateBoltInputs(update_bolt_inputs::UpdateBoltInputs {
        new_bolt_f_ed_kn: 1.0,
        new_bolt_n_bolts: 1,
        new_bolt_a_s_mm2: 1.0,
        new_bolt_e1_mm: 1.0,
        new_bolt_e2_mm: 1.0,
        new_bolt_d0_mm: 1.0,
        new_bolt_d_mm: 1.0,
        new_bolt_t_mm: 1.0,
        new_bolt_f_u_mpa: 1.0,
        new_bolt_f_ub_mpa: 1.0,
    });
    assert_eq!(bolt.semantics().kind, "update-bolt-inputs");
    assert_eq!(bolt.semantics().record, "UpdatedBoltInputs");
    assert_eq!(bolt.semantics().verb, "update");
}
