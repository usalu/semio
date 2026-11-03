//! 📍️ EN 1994 diff lane `studs`: the keyed stud rows. See [`super`].
use super::*;

scalar_patch! {
    /// 🩹️ Sparse patch of the `studs` row addressed by its key.
    En1994StudsPatch for semio_s_artifact_norm_en1994_model::Stud { beam_id: String, x_m: f64, diameter_m: f64, height_m: f64, f_u_pa: f64 }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Row delta of the `studs` list (rows keyed by `id`).
    pub En1994StudsRows { removal: En1994StudsRemoved, insertion: En1994StudsInserted, relocation: En1994StudsMoved, modification: En1994StudsModified, row: semio_s_artifact_norm_en1994_model::Stud, patch: En1994StudsPatch, list: Vec<semio_s_artifact_norm_en1994_model::Stud>, key: String = by En1994StudKey }
}
