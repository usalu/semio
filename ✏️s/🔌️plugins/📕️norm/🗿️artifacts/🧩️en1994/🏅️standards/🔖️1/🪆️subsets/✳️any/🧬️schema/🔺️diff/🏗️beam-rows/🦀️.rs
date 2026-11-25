//! 🏗️ EN 1994 diff lane `beams`: the keyed beam rows and their `actions` lane. See [`super`].
use super::*;

scalar_patch! {
    /// 🩹️ Sparse patch of the `actions` row addressed by its key.
    En1994BeamsActionsPatch for semio_s_artifact_norm_en1994_model::CharacteristicAction { kind: semio_s_artifact_norm_en1994_model::ActionKind, category: String, stage: semio_s_artifact_norm_en1994_model::ActionStage, q_area_pa: f64, f_k_n: f64, delta_sigma_k_pa: f64, delta_tau_k_pa: f64, x_m: f64 }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Row delta of the `actions` list of a beam (rows keyed by `id`).
    pub En1994BeamsActionsRows { removal: En1994BeamsActionsRemoved, insertion: En1994BeamsActionsInserted, relocation: En1994BeamsActionsMoved, modification: En1994BeamsActionsModified, row: semio_s_artifact_norm_en1994_model::CharacteristicAction, patch: En1994BeamsActionsPatch, list: Vec<semio_s_artifact_norm_en1994_model::CharacteristicAction>, key: String = by En1994BeamsActionKey }
}

nested_patch! {
    /// 🩹️ Sparse patch of the `beams` row addressed by its key.
    En1994BeamsPatch for semio_s_artifact_norm_en1994_model::CompositeBeam {
        leaf span_m: f64,
        leaf spacing_m: f64,
        leaf support: semio_s_artifact_norm_en1994_model::SupportCondition,
        leaf construction: semio_s_artifact_norm_en1994_model::ConstructionMethod,
        leaf steel: semio_s_artifact_norm_en1994_model::SteelSection,
        nested steel_height_m: f64 = (steel.height_m),
        nested steel_width_m: f64 = (steel.width_m),
        nested steel_tw_m: f64 = (steel.tw_m),
        nested steel_tf_m: f64 = (steel.tf_m),
        nested steel_a_m2: f64 = (steel.a_m2),
        nested steel_w_pl_y_m3: f64 = (steel.w_pl_y_m3),
        nested steel_i_y_m4: f64 = (steel.i_y_m4),
        nested steel_a_v_m2: f64 = (steel.a_v_m2),
        nested steel_r_m: f64 = (steel.r_m),
        leaf slab_thickness_m: f64,
        leaf concrete_f_ck_pa: f64,
        leaf concrete_e_cm_pa: f64,
        nested sheeting_profile: semio_s_artifact_norm_en1994_model::SheetingProfile = (sheeting.profile),
        nested sheeting_height_m: f64 = (sheeting.height_m),
        nested sheeting_rib_width_m: f64 = (sheeting.rib_width_m),
        nested sheeting_thickness_m: f64 = (sheeting.thickness_m),
        nested sheeting_ribs_parallel_to_beam: bool = (sheeting.ribs_parallel_to_beam),
        leaf transverse_as_m2_per_m: f64,
        leaf ltb_length_m: f64,
        leaf as_hogging_m2_per_m: f64,
        leaf bar_spacing_m: f64,
        leaf wk_limit_m: f64,
        leaf n_cycles: f64,
        leaf adjacent_span_m: f64,
        leaf patch_bearing_length_m: f64,
        leaf bridge_lanes: u32,
        leaf cracked_length_fraction: f64,
        leaf stud_transverse_spacing_m: f64,
        leaf stud_edge_distance_m: f64,
        leaf transverse_as_top_m2_per_m: f64,
        leaf end_plate_width_m: f64,
        leaf end_plate_height_m: f64,
        leaf end_zone_length_m: f64,
        leaf reinforcement_product: semio_s_artifact_norm_en1994_model::ReinforcementProduct,
        rows actions: En1994BeamsActionsRows = "actions",
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Row delta of the `beams` list (rows keyed by `id`).
    pub En1994BeamsRows { removal: En1994BeamsRemoved, insertion: En1994BeamsInserted, relocation: En1994BeamsMoved, modification: En1994BeamsModified, row: semio_s_artifact_norm_en1994_model::CompositeBeam, patch: En1994BeamsPatch, list: Vec<semio_s_artifact_norm_en1994_model::CompositeBeam>, key: String = by En1994BeamKey }
}
