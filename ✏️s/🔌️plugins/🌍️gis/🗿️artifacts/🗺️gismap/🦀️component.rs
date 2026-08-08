//! 🗺️ GIS map artifact — the document entity the 2d app edits (constitutional: general).

use protocol::{Identified, Patchable};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability, OsMediaFormat};
use serde::{Deserialize, Serialize};

//#region 🔹Constants
pub const GIS_MAP_SCHEMA: &str = "gis.map";
//#endregion 🔹Constants

//#region 🔹Types
/// 🗺️ One id-keyed spatial feature carried as its full opaque descriptor payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MapFeature {
    #[dsl(positional)]
    pub id: String,
    /// 🧬️ Deliberately untyped: binds through the engine's `Shape::Value` escape hatch.
    pub data: dsl::DslValue,
}

impl Identified<String> for MapFeature {
    fn id(&self) -> &String {
        &self.id
    }
}

/// Whole-payload replacement patch (features are opaque JSON); inverts to the prior payload.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MapFeaturePatch {
    pub data: Option<dsl::DslValue>,
}

impl Patchable<MapFeaturePatch> for MapFeature {
    fn apply_patch(&mut self, patch: &MapFeaturePatch) {
        if let Some(data) = &patch.data {
            self.data = data.clone();
        }
    }

    fn diff_patch(&self, other: &Self) -> Option<MapFeaturePatch> {
        (self.data != other.data).then(|| MapFeaturePatch { data: Some(other.data.clone()) })
    }
}

/// 📸️ Persisted GIS map snapshot — defined in `📸️ snapshot/🧬️ schema`, re-exported here.
pub use crate::artifacts::gismap::snapshot::schema::GisMapSnapshot;
//#endregion 🔹Types

//#region 🔹ArtifactKind
/// The `2d.map` artifact kind declaration.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.map".into(),
        name: "2D Map".into(),
        source_format: GIS_MAP_SCHEMA.into(),
        component_kind: "gismap".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        schema: GIS_MAP_SCHEMA.into(),
        export_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
        import_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
    }
}
//#endregion 🔹ArtifactKind

//#region 🔹Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_artifact_kind_matches_the_map_out_interchange_kind() {
        let kind = artifact_kind();
        assert_eq!(kind.id, "2d.map");
        assert_eq!(kind.schema, GIS_MAP_SCHEMA);
    }

    #[test]
    fn the_map_snapshot_defaults_to_empty_feature_collections() {
        let document = GisMapSnapshot::default();
        assert!(document.positions.is_empty());
        assert!(document.routes.is_empty());
        assert!(document.regions.is_empty());
    }
}
//#endregion 🔹Tests
