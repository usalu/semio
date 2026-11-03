//! 🏛️ EN 1994 diff lane `columns`: the keyed column rows and their `actions` lane. See [`super`].
use super::*;

scalar_patch! {
    /// 🩹️ Sparse patch of the `actions` row addressed by its key.
    En1994ColumnsActionsPatch for semio_s_artifact_norm_en1994_model::ColumnAction { kind: semio_s_artifact_norm_en1994_model::ActionKind, category: String, stage: semio_s_artifact_norm_en1994_model::ActionStage, n_k_n: f64, m_k_nm: f64 }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Row delta of the `actions` list of a column (rows keyed by `id`).
    pub En1994ColumnsActionsRows { removal: En1994ColumnsActionsRemoved, insertion: En1994ColumnsActionsInserted, relocation: En1994ColumnsActionsMoved, modification: En1994ColumnsActionsModified, row: semio_s_artifact_norm_en1994_model::ColumnAction, patch: En1994ColumnsActionsPatch, list: Vec<semio_s_artifact_norm_en1994_model::ColumnAction>, key: String = by En1994ColumnsActionKey }
}

nested_patch! {
    /// 🩹️ Sparse patch of the `columns` row addressed by its key.
    En1994ColumnsPatch for semio_s_artifact_norm_en1994_model::CompositeColumn {
        set { kind: semio_s_artifact_norm_en1994_model::ColumnKind, length_m: f64, outer_size_m: f64, wall_thickness_m: f64, steel_a_m2: f64, steel_f_y_pa: f64, concrete_a_m2: f64, concrete_f_ck_pa: f64, reinforcement_as_m2: f64, reinforcement_f_yk_pa: f64, i_m4: f64, buckling_curve: semio_s_artifact_norm_en1994_model::BucklingCurve, introduced_force_n: f64, section_height_m: f64, section_width_m: f64, section_tw_m: f64, section_tf_m: f64, rebar_cover_m: f64, reinforcement_product: semio_s_artifact_norm_en1994_model::ReinforcementProduct }
        within {  }
        rows { actions: En1994ColumnsActionsRows = "actions" }
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Row delta of the `columns` list (rows keyed by `id`).
    pub En1994ColumnsRows { removal: En1994ColumnsRemoved, insertion: En1994ColumnsInserted, relocation: En1994ColumnsMoved, modification: En1994ColumnsModified, row: semio_s_artifact_norm_en1994_model::CompositeColumn, patch: En1994ColumnsPatch, list: Vec<semio_s_artifact_norm_en1994_model::CompositeColumn>, key: String = by En1994ColumnKey }
}
