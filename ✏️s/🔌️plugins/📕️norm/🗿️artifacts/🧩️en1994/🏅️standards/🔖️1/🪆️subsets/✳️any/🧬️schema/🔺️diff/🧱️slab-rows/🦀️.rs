//! 🧱️ EN 1994 diff lane `slabs`: the keyed slab rows and their `actions` lane. See [`super`].
use super::*;

scalar_patch! {
    /// 🩹️ Sparse patch of the `actions` row addressed by its key.
    En1994SlabsActionsPatch for semio_s_artifact_norm_en1994_model::SlabAction { kind: semio_s_artifact_norm_en1994_model::ActionKind, category: String, stage: semio_s_artifact_norm_en1994_model::ActionStage, q_area_pa: f64, f_k_n: f64, delta_sigma_k_pa: f64, delta_tau_k_pa: f64, x_m: f64, y_m: f64 }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Row delta of the `actions` list of a slab (rows keyed by `id`).
    pub En1994SlabsActionsRows { removal: En1994SlabsActionsRemoved, insertion: En1994SlabsActionsInserted, relocation: En1994SlabsActionsMoved, modification: En1994SlabsActionsModified, row: semio_s_artifact_norm_en1994_model::SlabAction, patch: En1994SlabsActionsPatch, list: Vec<semio_s_artifact_norm_en1994_model::SlabAction>, key: String = by En1994SlabsActionKey }
}

nested_patch! {
    /// 🩹️ Sparse patch of the `slabs` row addressed by its key.
    En1994SlabsPatch for semio_s_artifact_norm_en1994_model::CompositeSlab {
        set { span_m: f64, support: semio_s_artifact_norm_en1994_model::SupportCondition, concrete_thickness_m: f64, f_ck_pa: f64, m_factor: f64, k_factor: f64, as_m2_per_m: f64 }
        within { sheeting_profile => sheeting . profile : semio_s_artifact_norm_en1994_model::SheetingProfile, sheeting_height_m => sheeting . height_m : f64, sheeting_rib_width_m => sheeting . rib_width_m : f64, sheeting_thickness_m => sheeting . thickness_m : f64, sheeting_ribs_parallel_to_beam => sheeting . ribs_parallel_to_beam : bool }
        rows { actions: En1994SlabsActionsRows = "actions" }
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Row delta of the `slabs` list (rows keyed by `id`).
    pub En1994SlabsRows { removal: En1994SlabsRemoved, insertion: En1994SlabsInserted, relocation: En1994SlabsMoved, modification: En1994SlabsModified, row: semio_s_artifact_norm_en1994_model::CompositeSlab, patch: En1994SlabsPatch, list: Vec<semio_s_artifact_norm_en1994_model::CompositeSlab>, key: String = by En1994SlabKey }
}
