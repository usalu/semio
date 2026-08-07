//! 🗺️ GIS map artifact — the document entity the ◻2d app edits (constitutional: general).

use protocol::{Identified, Patchable};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability, OsMediaFormat};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
pub const GIS_MAP_SCHEMA: &str = "gis.map";
//#endregion 🔖️Constants

//#region 🔖️Types
/// 🗺️ One id-keyed spatial feature (a position pin, route polyline, or region ring) carried as its full
/// opaque descriptor payload — id-keyed so two authors editing different features converge granularly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MapFeature {
    #[dsl(positional)]
    pub id: String,
    /// 🧬️ Deliberately untyped: binds through the engine's `Shape::Value` escape hatch because the key
    /// set genuinely varies by collection — positions carry `{lon, lat, icon, kind, label, name,
    /// sourceUrl?}`, routes carry `{points: [[f64; 2]]}`, regions carry `{ring: [[f64; 2]]}` — no single
    /// fixed schema fits all three, so a typed `dsl::DslDocument` derive doesn't apply here.
    pub data: dsl::DslValue,
}

impl Identified<String> for MapFeature {
    fn id(&self) -> &String {
        &self.id
    }
}

/// 🩹️ Whole-payload replacement patch (features are opaque JSON); inverts to the prior payload.
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

/// 🗺️ The editable map document: three id-keyed feature collections. All view/config state (camera,
/// render mode, vector style, LOD, selection, layer visibility, stroke weights) is plugin runtime, not
/// document state, so panning and styling never enter undo history.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "gis.gismap", layout = "lines")]
pub struct GisMapDocument {
    #[serde(default)]
    #[dsl(table)]
    pub positions: Vec<MapFeature>,
    #[serde(default)]
    #[dsl(table)]
    pub routes: Vec<MapFeature>,
    #[serde(default)]
    #[dsl(table)]
    pub regions: Vec<MapFeature>,
}

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for GisMapDocument {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted DocumentPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::DocumentPack for GisMapDocument {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️DocumentCodec

//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — the canonical `2d.map` declaration, stitched into BOTH app
/// manifests (`crate::apps::gis2d::create_gis2d_app` owns it; `crate::apps::gis3d::create_gis3d_app`
/// re-declares the identical shape for clarity on the `map:in` edge — the registry dedupes by id).
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.map".into(),
        name: "2D Map".into(),
        source_format: "gis.map".into(),
        component_kind: "gismap".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        schema: "gis.map".into(),
        export_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
        import_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_kind_declares_the_2d_map_interchange_kind() {
        let kind = artifact_kind();
        assert_eq!(kind.id, "2d.map");
        assert_eq!(kind.schema, GIS_MAP_SCHEMA);
        assert_eq!(kind.component_kind, "gismap");
    }

    #[test]
    fn map_feature_patch_applies_and_diffs_the_whole_payload() {
        let mut feature = MapFeature { id: "p1".into(), data: dsl::DslValue::Null };
        let next = MapFeature { id: "p1".into(), data: dsl::DslValue::String("x".into()) };
        let patch = feature.diff_patch(&next).expect("payload changed");
        feature.apply_patch(&patch);
        assert_eq!(feature, next);
        assert!(feature.diff_patch(&next).is_none(), "identical payloads produce no patch");
    }
}
//#endregion 🧪️Tests
