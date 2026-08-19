//! 🧬️ Cad artifact schema — every field of the artifact with its state class.

use crate::artifacts::cad::{
    CadCamera, CadDrawingChild, CadModelChild, CadNode, CadReferenceList, CadSnapshot,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️ArtifactHelpers
/// 🎯️ Component-level selection for World3d overlays (artifact-owned mirror of app config).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadSelectionTargets {
    pub mesh: bool,
    pub vertex: bool,
    pub edge: bool,
    pub face: bool,
}

impl Default for CadSelectionTargets {
    fn default() -> Self {
        Self { mesh: true, vertex: false, edge: true, face: false }
    }
}

/// 🎯️ Component selection record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadComponentSelection {
    pub targets: CadSelectionTargets,
    pub mode: String,
    pub ids: Vec<u32>,
}

impl Default for CadComponentSelection {
    fn default() -> Self {
        Self { targets: CadSelectionTargets::default(), mode: "mesh".into(), ids: Vec::new() }
    }
}

/// 🎛️ Per-pane dislocate handle groups.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadDislocateOptions {
    pub move_enabled: bool,
    pub rotate_enabled: bool,
}

impl Default for CadDislocateOptions {
    fn default() -> Self {
        Self { move_enabled: true, rotate_enabled: true }
    }
}
//#endregion 🔖️ArtifactHelpers

//#region 🔖️Artifact
/// 🧬️ Full cad artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.cad.cad")]
pub struct CadArtifact {
    #[state(artifact)] pub schema: String,
    #[state(artifact)] pub id: String,
    #[state(artifact)] #[child(kind = "s.stdio.semio.model")] pub shape_model: Option<CadModelChild>,
    #[state(artifact)] #[child(kind = "s.stdio.semio.model")] pub building_model: Option<CadModelChild>,
    #[state(artifact)] #[child(kind = "s.stdio.semio.model")] pub energy_model: Option<CadModelChild>,
    #[state(artifact)] #[child(kind = "s.stdio.semio.model")] pub structure_classic_model: Option<CadModelChild>,
    #[state(artifact)] #[child(kind = "s.stdio.semio.drawing")] pub drawings: Vec<CadDrawingChild>,
    #[state(artifact)] pub references_by_model_definition_id: BTreeMap<String, CadReferenceList>,
    #[state(artifact)] pub nodes: Vec<CadNode>,
    #[state(artifact)] pub active_model_definition_id: String,
    #[state(presence)] pub selected_object_ids: Vec<String>,
    #[state(presence)] pub selected_node_ids: Vec<String>,
    #[state(presence)] pub active_object_id: Option<String>,
    #[state(presence)] pub component_selection: CadComponentSelection,
    #[state(presence)] pub selected_reference_model_definition_id: Option<String>,
    #[state(presence)] pub selected_reference_id: Option<String>,
    #[state(presence)] pub selected_primitive_id: Option<String>,
    #[state(presence)] pub selected_primitive_kind: Option<String>,
    #[state(presence)] pub active_utility_id: String,
    #[state(presence)] pub active_example_id: Option<String>,
    #[state(config)] pub selection_method: String,
    #[state(config)] pub engagement_input: String,
    #[state(config)] pub engagement_step: String,
    #[state(config)] pub engagement_pane: Option<String>,
    #[state(config)] pub engagement_session_json: Option<String>,
    #[state(config)] pub last_finalized_interaction_id: Option<String>,
    #[state(config)] pub sun_enabled: bool,
    #[state(config)] pub sun_azimuth: f64,
    #[state(config)] pub sun_elevation: f64,
    #[state(config)] pub sun_intensity: f64,
    #[state(config)] pub sun_color: String,
    #[state(config)] pub camera: CadCamera,
    #[state(config)] pub camera_building: CadCamera,
    #[state(config)] pub camera_energy: CadCamera,
    #[state(config)] pub camera_structure_classic: CadCamera,
    #[state(config)] pub dislocate_shape: CadDislocateOptions,
    #[state(config)] pub dislocate_building: CadDislocateOptions,
    #[state(config)] pub dislocate_energy: CadDislocateOptions,
    #[state(config)] pub dislocate_structure_classic: CadDislocateOptions,
    #[state(config)] pub locale: String,
    #[state(config)] pub terminology: String,
    #[state(config)] pub contributions_json: String,
    #[state(artifact)] pub hovered_object_id: Option<String>,
    #[state(artifact)] pub hovered_target_object_id: Option<String>,
    #[state(artifact)] pub hovered_target_mode: Option<String>,
    #[state(artifact)] pub hovered_target_id: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for CadArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::artifacts::cad::empty_cad_snapshot())
    }
}

impl CadArtifact {
    /// 📸️ Persisted subset.
    pub async fn to_snapshot(&self) -> CadSnapshot {
        CadSnapshot {
            schema: self.schema.clone(),
            id: self.id.clone(),
            shape_model: self.shape_model.clone(),
            building_model: self.building_model.clone(),
            energy_model: self.energy_model.clone(),
            structure_classic_model: self.structure_classic_model.clone(),
            drawings: self.drawings.clone(),
            references_by_model_definition_id: self.references_by_model_definition_id.clone(),
            nodes: self.nodes.clone(),
            active_model_definition_id: self.active_model_definition_id.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub async fn from_snapshot(snapshot: CadSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            shape_model: snapshot.shape_model,
            building_model: snapshot.building_model,
            energy_model: snapshot.energy_model,
            structure_classic_model: snapshot.structure_classic_model,
            drawings: snapshot.drawings,
            references_by_model_definition_id: snapshot.references_by_model_definition_id,
            nodes: snapshot.nodes,
            active_model_definition_id: snapshot.active_model_definition_id,
            selected_object_ids: Vec::new(),
            selected_node_ids: Vec::new(),
            active_object_id: None,
            component_selection: CadComponentSelection::default(),
            selected_reference_model_definition_id: None,
            selected_reference_id: None,
            selected_primitive_id: None,
            selected_primitive_kind: None,
            active_utility_id: "dislocate".into(),
            active_example_id: None,
            selection_method: "rectangle".into(),
            engagement_input: String::new(),
            engagement_step: "Idle".into(),
            engagement_pane: None,
            engagement_session_json: None,
            last_finalized_interaction_id: None,
            sun_enabled: false,
            sun_azimuth: 45.0,
            sun_elevation: 35.0,
            sun_intensity: 0.85,
            sun_color: "#ffffff".into(),
            camera: CadCamera::default(),
            camera_building: CadCamera::default(),
            camera_energy: CadCamera::default(),
            camera_structure_classic: CadCamera::default(),
            dislocate_shape: CadDislocateOptions::default(),
            dislocate_building: CadDislocateOptions::default(),
            dislocate_energy: CadDislocateOptions::default(),
            dislocate_structure_classic: CadDislocateOptions::default(),
            locale: "en-US".into(),
            terminology: "native".into(),
            contributions_json: "[]".into(),
            hovered_object_id: None,
            hovered_target_object_id: None,
            hovered_target_mode: None,
            hovered_target_id: None,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub async fn set_snapshot(&mut self, snapshot: CadSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.shape_model = snapshot.shape_model;
        self.building_model = snapshot.building_model;
        self.energy_model = snapshot.energy_model;
        self.structure_classic_model = snapshot.structure_classic_model;
        self.drawings = snapshot.drawings;
        self.references_by_model_definition_id = snapshot.references_by_model_definition_id;
        self.nodes = snapshot.nodes;
        self.active_model_definition_id = snapshot.active_model_definition_id;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.cad.cad` — twenty handcrafted schema leaves.
pub async fn cad_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.cad.cad",
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
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::cad::diff::schema::CadDiff;
    use crate::artifacts::cad::mutations::CadMutation;
    use crate::artifacts::cad::{CadSnapshot, CAD_PLAY_DOCUMENT_SCHEMA};
    use std::collections::BTreeMap;

    //#region Builder
    async fn empty_snapshot() -> CadSnapshot {
        CadSnapshot {
            schema: CAD_PLAY_DOCUMENT_SCHEMA.into(),
            id: String::new(),
            shape_model: None,
            building_model: None,
            energy_model: None,
            structure_classic_model: None,
            drawings: Vec::new(),
            references_by_model_definition_id: BTreeMap::new(),
            nodes: Vec::new(),
            active_model_definition_id: String::new(),
        }
    }

    /// Builds a `cad` snapshot.
    #[derive(Clone, Debug)]
    pub struct CadBuilderConstruction {
        snapshot: CadSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for CadBuilderConstruction {
        type Snapshot = CadSnapshot;
        type Mutation = CadMutation;
        type Diff = CadDiff;

        async fn empty() -> Self {
            Self { snapshot: empty_snapshot(), diagnostics: Vec::new() }
        }

        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }

        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<CadSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<CadSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <CadMutation as protocol::Mutation<CadSnapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error(
                    "mutation.apply",
                    dsl::TextSpan::at(1, 1),
                    error.to_string(),
                )),
            }
            (self, outcome)
        }

        async fn absorb(
            mut self,
            diff: Self::Diff,
        ) -> protocol::MutationApplyResult<Self> {
            let snapshot = <CadDiff as protocol::MutationDiff<CadSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }

        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
    //#endregion Builder
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::cad::CadSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct CadParts {
        pub snapshot: Option<CadSnapshot>,
    }

    pub struct CadAnalyzerAnalysis;

    impl ArtifactAnalysis for CadAnalyzerAnalysis {
        type Parts = CadParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.cad", standard: StandardId("1"), subset: SubsetId("*") };

        async fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = CadParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <CadSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <CadSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec CadBuilderFacets {
        construction: CadBuilderConstruction,
        analysis: CadAnalyzerAnalysis,
        composition: super::super::io::derived_composition::CadComposerComposition,
    }
    builder: CadBuilder,
    analyzer: CadAnalyzer,
    composer: CadComposer,
);
//#endregion 🧬️DerivedArtifactFacets
