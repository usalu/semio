//! 🧬️ GIS map artifact schema — every field of the artifact with its state class.

use crate::artifacts::gismap::dsl::REUSE_MAP_EXAMPLE_TEXT;
use crate::artifacts::gismap::mutations::{create_position, create_region, create_route, delete_position, delete_region, delete_route, replace_position_data, replace_region_data, replace_route_data};
use crate::artifacts::gismap::op::GisMapMutation;
use crate::artifacts::gismap::{GisMapSnapshot, MapFeature, GIS_MAP_SCHEMA};
use semio_framework_plugin::{ArtifactSerializer, DwgDrawing, DwgGeometry, ErasedComposeSource, IoDirection, IoKey, IoPayload, io_dispatch};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::engine::geometry::{SemioPoint2, SemioRgba, SemioTransform};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::svg::v1_1::any::SemioDrawingToSvg;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot};
use semio_s_plugin_stdio::artifacts::svg::SvgSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashSet};

//#region 🔹Artifact
/// 🧬️ Full GIS map artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.gis.gismap")]
pub struct GisMapArtifact {
    #[state(persistent)] pub positions: Vec<MapFeature>,
    #[state(persistent)] pub routes: Vec<MapFeature>,
    #[state(persistent)] pub regions: Vec<MapFeature>,
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(shared_ui)] pub feature_selection_json: String,
    #[state(shared_ui)] pub layer_visibility: BTreeMap<String, bool>,
    #[state(shared_ui)] pub layer_stroke_scale: BTreeMap<String, f64>,
    #[state(local_ui)] pub camera_json: String,
    #[state(local_ui)] pub render_mode: String,
    #[state(local_ui)] pub vector_style: String,
    #[state(local_ui)] pub lod_mode: String,
    #[state(local_ui)] pub hover_json: String,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub selection_mode: String,
    #[state(local_ui)] pub locale: String,
}
//#endregion 🔹Artifact

//#region 🔹Conversions
impl Default for GisMapArtifact {
    fn default() -> Self {
        Self {
            positions: Vec::new(),
            routes: Vec::new(),
            regions: Vec::new(),
            selected_ids: Vec::new(),
            feature_selection_json: r#"{"positions":[],"routes":[]}"#.into(),
            layer_visibility: BTreeMap::new(),
            layer_stroke_scale: BTreeMap::new(),
            camera_json: r#"{"x":0,"y":0,"zoom":1}"#.into(),
            render_mode: "combined".into(),
            vector_style: "colored".into(),
            lod_mode: "automatic".into(),
            hover_json: "null".into(),
            selection_method: "rectangle".into(),
            selection_mode: "default".into(),
            locale: "en-US".into(),
        }
    }
}

impl GisMapArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::gismap::GisMapSnapshot {
        crate::artifacts::gismap::GisMapSnapshot {
            positions: self.positions.clone(),
            routes: self.routes.clone(),
            regions: self.regions.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::gismap::GisMapSnapshot) -> Self {
        Self {
            positions: snapshot.positions,
            routes: snapshot.routes,
            regions: snapshot.regions,
            ..Self::default()
        }
    }

    /// Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::gismap::GisMapSnapshot) {
        self.positions = snapshot.positions;
        self.routes = snapshot.routes;
        self.regions = snapshot.regions;
    }
}
//#endregion 🔹Conversions

//#region 🔹Descriptor
/// 🧬️ Descriptor for `s.gis.gismap` — twenty handcrafted schema leaves.
pub fn gismap_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.gis.gismap",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔹Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::gismap::{GisMapDiff, GisMapMutation, GisMapSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct GismapBuilderConstruction {
        snapshot: GisMapSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for GismapBuilderConstruction {
        type Snapshot = GisMapSnapshot;
        type Mutation = GisMapMutation;
        type Diff = GisMapDiff;
        fn empty() -> Self { Self { snapshot: GisMapSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<GisMapSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<GisMapSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            crate::artifacts::gismap::schema::mutations::apply_gis_map_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <GisMapDiff as protocol::MutationDiff<GisMapSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::gismap::GisMapSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct GisMapParts {
        pub snapshot: Option<GisMapSnapshot>,
    }

    pub struct GisMapAnalyzerAnalysis;

    impl ArtifactAnalysis for GisMapAnalyzerAnalysis {
        type Parts = GisMapParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.gismap", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = GisMapParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <GisMapSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <GisMapSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec GismapBuilderFacets {
        construction: derived_construction::GismapBuilderConstruction,
        analysis: derived_analysis::GisMapAnalyzerAnalysis,
        composition: super::super::io::derived_composition::GisMapComposerComposition,
    }
    builder: GismapBuilder,
    analyzer: GisMapAnalyzer,
    composer: GisMapComposer,
);
//#endregion 🧬️DerivedArtifactFacets
