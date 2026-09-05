//! 🧬️ GIS terrain artifact schema — every field of the artifact with its state class.

use crate::artifacts::gisterrain::dsl::REUSE_TERRAIN_EXAMPLE_TEXT;
use crate::artifacts::gisterrain::{gis_terrain_mesh_child_handle, gis_terrain_mesh_content_key, GisTerrainSnapshot};
use framework_surface::terrain::tiles;
use schema::ArtifactSchema;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Artifact
/// 🧬️ Full GIS terrain artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.gis.gisterrain")]
pub struct GisTerrainArtifact {
    #[state(artifact)]
    pub exaggeration: f64,
    #[state(artifact)]
    pub imported_features_json: String,
    /// 🕸️ Mirrors `GisTerrainSnapshot.mesh` — see that field's own doc comment. Always re-derived
    /// from `(exaggeration, imported_features_json)` by `to_snapshot`, never independently set.
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.mesh")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>,
    #[state(config)]
    pub camera_json: String,
    #[state(config)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for GisTerrainArtifact {
    fn default() -> Self {
        Self {
            exaggeration: 0.0,
            imported_features_json: String::new(),
            mesh: Some(gis_terrain_mesh_child_handle(&gis_terrain_mesh_content_key(0.0, ""))),
            camera_json: serde_json::json!({ "position": [800.0, -800.0, 600.0], "target": [0.0, 0.0, 0.0], "up": [0.0, 0.0, 1.0], "fov": 45.0 }).to_string(),
            locale: "en-US".into(),
        }
    }
}

impl GisTerrainArtifact {
    /// 📸️ Persisted subset. `mesh` is always re-derived here (never carried verbatim off `self`) so
    /// it can never drift from what `(exaggeration, imported_features_json)` actually determine.
    pub fn to_snapshot(&self) -> GisTerrainSnapshot {
        GisTerrainSnapshot { exaggeration: self.exaggeration, imported_features_json: self.imported_features_json.clone(), mesh: Some(gis_terrain_mesh_child_handle(&gis_terrain_mesh_content_key(self.exaggeration, &self.imported_features_json))) }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: GisTerrainSnapshot) -> Self {
        Self { exaggeration: snapshot.exaggeration, imported_features_json: snapshot.imported_features_json, mesh: snapshot.mesh, ..Self::default() }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: GisTerrainSnapshot) {
        self.exaggeration = snapshot.exaggeration;
        self.imported_features_json = snapshot.imported_features_json;
        self.mesh = snapshot.mesh;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.gis.gisterrain` — twenty handcrafted schema leaves.
pub fn gisterrain_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.gis.gisterrain",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::gisterrain::{GisTerrainDiff, GisTerrainMutation, GisTerrainSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct GisterrainBuilderConstruction {
        snapshot: GisTerrainSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for GisterrainBuilderConstruction {
        type Snapshot = GisTerrainSnapshot;
        type Mutation = GisTerrainMutation;
        type Diff = GisTerrainDiff;
        fn empty() -> Self {
            Self { snapshot: GisTerrainSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<GisTerrainSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<GisTerrainSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <GisTerrainDiff as protocol::MutationDiff<GisTerrainSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::gisterrain::GisTerrainSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct GisTerrainParts {
        pub snapshot: Option<GisTerrainSnapshot>,
    }

    pub struct GisTerrainAnalyzerAnalysis;

    impl ArtifactAnalysis for GisTerrainAnalyzerAnalysis {
        type Parts = GisTerrainParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.gis.gisterrain", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = GisTerrainParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <GisTerrainSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <GisTerrainSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec GisterrainBuilderFacets {
        construction: GisterrainBuilderConstruction,
        analysis: GisTerrainAnalyzerAnalysis,
        composition: super::super::io::derived_composition::GisTerrainComposerComposition,
    }
    builder: GisterrainBuilder,
    analyzer: GisTerrainAnalyzer,
    composer: GisTerrainComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🧭️ Relocated from the artifact's `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): pure document helpers over
/// `GisTerrainSnapshot`, no app-state dependency — an artifact must never depend on an app.
pub fn empty_gis_terrain_snapshot() -> GisTerrainSnapshot {
    let exaggeration = 1.0;
    let imported_features_json = String::new();
    let mesh = Some(gis_terrain_mesh_child_handle(&gis_terrain_mesh_content_key(exaggeration, &imported_features_json)));
    GisTerrainSnapshot { exaggeration, imported_features_json, mesh }
}

/// 🗺️ The default terrain document, seeded from the bundled reuse example's `gisterrain
/// exaggeration=...` header (see `crate::artifacts::gisterrain::GisTerrainSnapshot`'s
/// derive-generated `.gisterrain` DSL).
pub fn default_terrain_document() -> GisTerrainSnapshot {
    <GisTerrainSnapshot as store::ArtifactDsl>::parse_dsl(REUSE_TERRAIN_EXAMPLE_TEXT).unwrap_or_else(|_| empty_gis_terrain_snapshot())
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️TerrainDescriptor
/// 🧭️ Relocated from `⚙️engine/terrain/🦀️.rs` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): pure DTOs + a pure formatter, neither
/// snapshot-derived nor stateful — the `.gis.json` shape a `gis/3d` example is authored in, and the
/// `World3dScene.terrain_json` payload built from it for the `gis3d-play` app's terrain window.
///
/// 🧭️ Originally relocated out of the generic `framework_surface_terrain` engine (audit finding A5:
/// the framework must not know ✏️s — these DTOs and `build_terrain_scene_json` name gis-specific
/// concepts). The DEM-tile decode/session/mesh engine itself
/// (`framework_surface::terrain::{tiles, projection, TerrainSessionCore}`) stays in the framework:
/// it is also path-mounted directly into `framework/os/infinite`'s `World3dState` (to dodge a
/// surface↔infinite cargo cycle) to drive the generic `World3d` terrain layer, so it is genuinely
/// shared rendering engine code, not gis-specific — only this descriptor/DTO layer belonged here.
#[derive(Clone, Debug, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct TerrainProjectOrigin {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Clone, Debug, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct TerrainPositionData {
    pub id: String,
    pub lon: f64,
    pub lat: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[derive(Clone, Debug, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct TerrainDescriptorJson {
    pub schema: String,
    pub project_origin: TerrainProjectOrigin,
    #[value(default)]
    pub positions: Vec<TerrainPositionData>,
    #[value(default = "default_exaggeration")]
    pub exaggeration: f64,
}

fn default_exaggeration() -> f64 {
    1.0
}

pub const GIS_3D_TERRAIN_TILE_URL_TEMPLATE: &str = "/dem/{z}/{x}/{y}.png";

#[derive(ToValue)]
#[value(rename_all = "camelCase")]
struct TerrainSceneStyleJson<'a> {
    tile_url_template: &'a str,
    project_origin_lon: f64,
    project_origin_lat: f64,
    exaggeration: f64,
    color_ramp: &'a str,
    min_zoom: u32,
    max_zoom: u32,
}

/// 🏔️ Builds the `World3dScene.terrain_json` payload for a descriptor — the one place gis needs to
/// reach into `framework_surface::terrain` beyond the wasm session itself (for the generic engine's
/// tile zoom bounds).
pub fn build_terrain_scene_json(descriptor: &TerrainDescriptorJson) -> String {
    let style = TerrainSceneStyleJson {
        tile_url_template: GIS_3D_TERRAIN_TILE_URL_TEMPLATE,
        project_origin_lon: descriptor.project_origin.lon,
        project_origin_lat: descriptor.project_origin.lat,
        exaggeration: descriptor.exaggeration,
        color_ramp: "hypsometric",
        min_zoom: tiles::TERRAIN_TILE_MIN_ZOOM,
        max_zoom: tiles::TERRAIN_TILE_MAX_ZOOM,
    };
    dsl::os_pack::json::to_json_string(&style)
}
//#endregion 🔖️TerrainDescriptor

//#region 🧪️Tests
#[cfg(test)]
mod relocated_engine_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn build_terrain_scene_json_roundtrips_descriptor_fields() {
        let descriptor = TerrainDescriptorJson {
            schema: "gis.terrain".to_string(),
            project_origin: TerrainProjectOrigin { lon: 9.7382, lat: 52.3759 },
            positions: vec![TerrainPositionData { id: "p1".to_string(), lon: 9.74, lat: 52.38, label: Some("Site".to_string()), icon: None }],
            exaggeration: 1.5,
        };
        let json = build_terrain_scene_json(&descriptor);
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
        assert_eq!(value["projectOriginLon"], 9.7382);
        assert_eq!(value["exaggeration"], 1.5);
        assert_eq!(value["tileUrlTemplate"], GIS_3D_TERRAIN_TILE_URL_TEMPLATE);
    }

    #[semio_framework_async_macros::async_test]
    async fn terrain_descriptor_json_defaults_exaggeration_and_positions_when_absent() {
        let json = r#"{"schema":"gis.terrain","projectOrigin":{"lon":1.0,"lat":2.0}}"#;
        let descriptor: TerrainDescriptorJson = serde_json::from_str(json).expect("valid descriptor json");
        assert_eq!(descriptor.exaggeration, 1.0);
        assert!(descriptor.positions.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn terrain_position_data_omits_none_fields_when_serialized() {
        let position = TerrainPositionData { id: "p2".to_string(), lon: 1.0, lat: 2.0, label: None, icon: Some("pin".to_string()) };
        let json = serde_json::to_string(&position).expect("serializes");
        assert!(!json.contains("label"));
        assert!(json.contains("\"icon\":\"pin\""));
    }

    /// 🧭️ Relocated from the artifact's `⚙️engine` tests alongside `default_terrain_document`/
    /// `empty_gis_terrain_snapshot` (`DocumentHelpers` above).
    #[semio_framework_async_macros::async_test]
    async fn default_terrain_document_seeds_the_fixture_exaggeration() {
        assert_eq!(default_terrain_document().exaggeration, 1.5);
        assert_eq!(empty_gis_terrain_snapshot().exaggeration, 1.0);
    }
}
//#endregion 🧪️Tests
