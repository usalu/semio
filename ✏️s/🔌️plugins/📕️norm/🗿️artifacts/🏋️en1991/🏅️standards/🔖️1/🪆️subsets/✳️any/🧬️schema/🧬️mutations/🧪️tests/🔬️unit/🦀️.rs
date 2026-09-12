use super::*;
use protocol::{Mutation, MutationDiff, SemanticMutation};

fn round_trip(base: &En1991Snapshot, operation: &En1991Mutation) -> En1991Snapshot {
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
async fn change_area_m2_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeAreaM2(change_area_m2::ChangeAreaM2 { new_area_m2: 77.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.area_m2, 77.0);
}

#[semio_framework_async_macros::async_test]
async fn change_category_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeCategory(change_category::ChangeCategory { new_category: crate::document::ImposedCategory::D });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.category, crate::document::ImposedCategory::D);
}

#[semio_framework_async_macros::async_test]
async fn change_annex_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.annex, crate::document::AnnexChoice::En);
}

#[semio_framework_async_macros::async_test]
async fn change_self_weight_material_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeSelfWeightMaterial(change_self_weight_material::ChangeSelfWeightMaterial { new_self_weight_material: "steel".to_string() });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.self_weight_material, "steel".to_string());
}

#[semio_framework_async_macros::async_test]
async fn change_self_weight_thickness_m_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeSelfWeightThicknessM(change_self_weight_thickness_m::ChangeSelfWeightThicknessM { new_self_weight_thickness_m: 0.3 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.self_weight_thickness_m, 0.3);
}

#[semio_framework_async_macros::async_test]
async fn change_assumed_gk_kn_m2_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeAssumedGKKnM2(change_assumed_gk_kn_m2::ChangeAssumedGKKnM2 { new_assumed_g_k_kn_m2: 7.5 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.assumed_g_k_kn_m2, 7.5);
}

#[semio_framework_async_macros::async_test]
async fn change_fire_curve_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeFireCurve(change_fire_curve::ChangeFireCurve { new_fire_curve: crate::part_1_2::FireCurve::Hydrocarbon });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.fire_curve, crate::part_1_2::FireCurve::Hydrocarbon);
}

#[semio_framework_async_macros::async_test]
async fn change_fire_resistance_min_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeFireResistanceMin(change_fire_resistance_min::ChangeFireResistanceMin { new_fire_resistance_min: 60.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.fire_resistance_min, 60.0);
}

#[semio_framework_async_macros::async_test]
async fn change_fire_member_capacity_c_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeFireMemberCapacityC(change_fire_member_capacity_c::ChangeFireMemberCapacityC { new_fire_member_capacity_c: 1000.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.fire_member_capacity_c, 1000.0);
}

#[semio_framework_async_macros::async_test]
async fn change_snow_zone_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeSnowZone(change_snow_zone::ChangeSnowZone { new_snow_zone: 3 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.snow_zone, 3);
}

#[semio_framework_async_macros::async_test]
async fn change_snow_altitude_m_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeSnowAltitudeM(change_snow_altitude_m::ChangeSnowAltitudeM { new_snow_altitude_m: 300.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.snow_altitude_m, 300.0);
}

#[semio_framework_async_macros::async_test]
async fn change_en_sk_kn_m2_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeEnSKKnM2(change_en_sk_kn_m2::ChangeEnSKKnM2 { new_en_s_k_kn_m2: 1.2 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.en_s_k_kn_m2, 1.2);
}

#[semio_framework_async_macros::async_test]
async fn change_wind_zone_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeWindZone(change_wind_zone::ChangeWindZone { new_wind_zone: 3 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.wind_zone, 3);
}

#[semio_framework_async_macros::async_test]
async fn change_en_vbms_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeEnVBMS(change_en_vbms::ChangeEnVBMS { new_en_v_b_m_s: 28.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.en_v_b_m_s, 28.0);
}

#[semio_framework_async_macros::async_test]
async fn change_delta_tk_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeDeltaTK(change_delta_tk::ChangeDeltaTK { new_delta_t_k: 40.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.delta_t_k, 40.0);
}

#[semio_framework_async_macros::async_test]
async fn change_construction_activity_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeConstructionActivity(change_construction_activity::ChangeConstructionActivity { new_construction_activity: "demolition".to_string() });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.construction_activity, "demolition".to_string());
}

#[semio_framework_async_macros::async_test]
async fn change_accidental_mass_t_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeAccidentalMassT(change_accidental_mass_t::ChangeAccidentalMassT { new_accidental_mass_t: 40.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.accidental_mass_t, 40.0);
}

#[semio_framework_async_macros::async_test]
async fn change_accidental_speed_km_h_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeAccidentalSpeedKmH(change_accidental_speed_km_h::ChangeAccidentalSpeedKmH { new_accidental_speed_km_h: 50.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.accidental_speed_km_h, 50.0);
}

#[semio_framework_async_macros::async_test]
async fn change_bridge_lane_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeBridgeLane(change_bridge_lane::ChangeBridgeLane { new_bridge_lane: 2 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.bridge_lane, 2);
}

#[semio_framework_async_macros::async_test]
async fn change_bridge_span_m_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeBridgeSpanM(change_bridge_span_m::ChangeBridgeSpanM { new_bridge_span_m: 35.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.bridge_span_m, 35.0);
}

#[semio_framework_async_macros::async_test]
async fn change_bridge_lane_width_m_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeBridgeLaneWidthM(change_bridge_lane_width_m::ChangeBridgeLaneWidthM { new_bridge_lane_width_m: 3.5 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.bridge_lane_width_m, 3.5);
}

#[semio_framework_async_macros::async_test]
async fn change_bridge_moment_resistance_knm_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeBridgeMomentResistanceKnm(change_bridge_moment_resistance_knm::ChangeBridgeMomentResistanceKnm { new_bridge_moment_resistance_knm: 3500.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.bridge_moment_resistance_knm, 3500.0);
}

#[semio_framework_async_macros::async_test]
async fn change_crane_class_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeCraneClass(change_crane_class::ChangeCraneClass { new_crane_class: "HC3".to_string() });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.crane_class, "HC3".to_string());
}

#[semio_framework_async_macros::async_test]
async fn change_hoist_class_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeHoistClass(change_hoist_class::ChangeHoistClass { new_hoist_class: "HC3".to_string() });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.hoist_class, "HC3".to_string());
}

#[semio_framework_async_macros::async_test]
async fn change_hoisting_speed_ms_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeHoistingSpeedMS(change_hoisting_speed_ms::ChangeHoistingSpeedMS { new_hoisting_speed_m_s: 0.8 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.hoisting_speed_m_s, 0.8);
}

#[semio_framework_async_macros::async_test]
async fn change_silo_bulk_density_kn_m3_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeSiloBulkDensityKnM3(change_silo_bulk_density_kn_m3::ChangeSiloBulkDensityKnM3 { new_silo_bulk_density_kn_m3: 9.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.silo_bulk_density_kn_m3, 9.0);
}

#[semio_framework_async_macros::async_test]
async fn change_silo_height_m_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeSiloHeightM(change_silo_height_m::ChangeSiloHeightM { new_silo_height_m: 15.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.silo_height_m, 15.0);
}

#[semio_framework_async_macros::async_test]
async fn change_silo_hydraulic_radius_m_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeSiloHydraulicRadiusM(change_silo_hydraulic_radius_m::ChangeSiloHydraulicRadiusM { new_silo_hydraulic_radius_m: 2.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.silo_hydraulic_radius_m, 2.0);
}

#[semio_framework_async_macros::async_test]
async fn change_silo_mu_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeSiloMu(change_silo_mu::ChangeSiloMu { new_silo_mu: 0.5 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.silo_mu, 0.5);
}

#[semio_framework_async_macros::async_test]
async fn change_silo_k_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeSiloK(change_silo_k::ChangeSiloK { new_silo_k: 0.5 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.silo_k, 0.5);
}

#[semio_framework_async_macros::async_test]
async fn change_cs_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeCS(change_cs::ChangeCS { new_c_s: 1.1 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.c_s, 1.1);
}

#[semio_framework_async_macros::async_test]
async fn change_cd_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeCD(change_cd::ChangeCD { new_c_d: 0.9 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.c_d, 0.9);
}

#[semio_framework_async_macros::async_test]
async fn semantic_kinds_cover_every_variant() {
    assert_eq!(En1991Mutation::kinds().len(), 32);
    let mutation = En1991Mutation::ChangeAreaM2(change_area_m2::ChangeAreaM2 { new_area_m2: 99.0 });
    assert_eq!(mutation.semantics().kind, "change-area-m2");
    assert_eq!(mutation.semantics().record, "ChangedAreaM2");
}

#[semio_framework_async_macros::async_test]
async fn change_category_inverse_restores_base_category() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeCategory(change_category::ChangeCategory { new_category: crate::document::ImposedCategory::D });
    let undo = mutation.inverse(&base);
    assert_eq!(undo, vec![En1991Mutation::ChangeCategory(change_category::ChangeCategory { new_category: base.category })]);
}

#[semio_framework_async_macros::async_test]
async fn change_of_a_string_field_undoes_to_default_value() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeCraneClass(change_crane_class::ChangeCraneClass { new_crane_class: "HC4".to_string() });
    let undo = mutation.inverse(&base);
    assert_eq!(undo, vec![En1991Mutation::ChangeCraneClass(change_crane_class::ChangeCraneClass { new_crane_class: base.crane_class.clone() })]);
}

//#region 🔖️OutcomeLaws
/// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
/// this facet is entirely one verb family (root-scoped `change-<field>`) — see en1992's own
/// `🔖️OutcomeLaws` note for why `assert_missing_target_is_error`/`assert_outcome_policy_matrix`
/// don't apply/aren't landed yet.
#[semio_framework_async_macros::async_test]
async fn change_area_m2_non_finite_is_fatal() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeAreaM2(change_area_m2::ChangeAreaM2 { new_area_m2: f64::INFINITY });
    let outcome = mutation.diff(&base);
    protocol::os_spr::protocol_laws::assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn change_category_same_value_is_no_op() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeCategory(change_category::ChangeCategory { new_category: base.category });
    let outcome = mutation.diff(&base);
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
    assert_eq!(outcome.diff(), &En1991Diff::default());
}

#[semio_framework_async_macros::async_test]
async fn change_area_m2_is_deterministic() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeAreaM2(change_area_m2::ChangeAreaM2 { new_area_m2: 77.0 });
    protocol::os_spr::protocol_laws::assert_outcome_deterministic(&base, &mutation).await;
}
//#endregion 🔖️OutcomeLaws
