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
        set { span_m: f64, spacing_m: f64, support: semio_s_artifact_norm_en1994_model::SupportCondition, construction: semio_s_artifact_norm_en1994_model::ConstructionMethod, steel: semio_s_artifact_norm_en1994_model::SteelSection, slab_thickness_m: f64, concrete_f_ck_pa: f64, concrete_e_cm_pa: f64, transverse_as_m2_per_m: f64, ltb_length_m: f64, as_hogging_m2_per_m: f64, bar_spacing_m: f64, wk_limit_m: f64, n_cycles: f64, adjacent_span_m: f64, patch_bearing_length_m: f64, bridge_lanes: u32, cracked_length_fraction: f64, stud_transverse_spacing_m: f64, stud_edge_distance_m: f64, transverse_as_top_m2_per_m: f64, end_plate_width_m: f64, end_plate_height_m: f64, end_zone_length_m: f64, reinforcement_product: semio_s_artifact_norm_en1994_model::ReinforcementProduct }
        within { steel_height_m => steel . height_m : f64, steel_width_m => steel . width_m : f64, steel_tw_m => steel . tw_m : f64, steel_tf_m => steel . tf_m : f64, steel_a_m2 => steel . a_m2 : f64, steel_w_pl_y_m3 => steel . w_pl_y_m3 : f64, steel_i_y_m4 => steel . i_y_m4 : f64, steel_a_v_m2 => steel . a_v_m2 : f64, steel_r_m => steel . r_m : f64, sheeting_profile => sheeting . profile : semio_s_artifact_norm_en1994_model::SheetingProfile, sheeting_height_m => sheeting . height_m : f64, sheeting_rib_width_m => sheeting . rib_width_m : f64, sheeting_thickness_m => sheeting . thickness_m : f64, sheeting_ribs_parallel_to_beam => sheeting . ribs_parallel_to_beam : bool }
        rows { actions: En1994BeamsActionsRows = "actions" }
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Row delta of the `beams` list (rows keyed by `id`).
    pub En1994BeamsRows { removal: En1994BeamsRemoved, insertion: En1994BeamsInserted, relocation: En1994BeamsMoved, modification: En1994BeamsModified, row: semio_s_artifact_norm_en1994_model::CompositeBeam, patch: En1994BeamsPatch, list: Vec<semio_s_artifact_norm_en1994_model::CompositeBeam>, key: String = by En1994BeamKey }
}
