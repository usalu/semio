//! 📍️ EN 1994 diff lane `placements`: the keyed placement rows. See [`super`].
use super::*;

scalar_patch! {
    /// 🩹️ Sparse patch of the `placements` row addressed by its key.
    En1994PlacementsPatch for semio_s_artifact_norm_en1994_model::Placement { x: f64, y: f64, z: f64, yaw: f64 }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Row delta of the `placements` list (rows keyed by `id`).
    pub En1994PlacementsRows { removal: En1994PlacementsRemoved, insertion: En1994PlacementsInserted, relocation: En1994PlacementsMoved, modification: En1994PlacementsModified, row: semio_s_artifact_norm_en1994_model::Placement, patch: En1994PlacementsPatch, list: Vec<semio_s_artifact_norm_en1994_model::Placement>, key: String = by En1994PlacementKey }
}
