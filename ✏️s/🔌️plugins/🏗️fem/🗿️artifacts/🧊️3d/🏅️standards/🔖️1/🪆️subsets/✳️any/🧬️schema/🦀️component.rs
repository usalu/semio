//! 🧬️ Fem3d artifact schema — every field of the artifact with its state class.

use crate::artifacts::fem3d::{FemAnalysisSettings, FemCamera, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemSection, FemSolid, FemSupport};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full fem3d artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.fem.fem3d")]
pub struct Fem3dArtifact {
    #[state(persistent)] pub nodes: Vec<FemNode>,
    #[state(persistent)] pub elements: Vec<FemElement>,
    #[state(persistent)] pub materials: Vec<FemMaterial>,
    #[state(persistent)] pub sections: Vec<FemSection>,
    #[state(persistent)] pub solids: Vec<FemSolid>,
    #[state(persistent)] pub supports: Vec<FemSupport>,
    #[state(persistent)] pub load_cases: Vec<FemLoadCase>,
    #[state(persistent)] pub combinations: Vec<FemCombination>,
    #[state(persistent)] pub analysis: FemAnalysisSettings,
    #[state(shared_ui)] pub result_source_id: Option<String>,
    #[state(shared_ui)] pub result_mode: String,
    #[state(shared_ui)] pub result_mode_index: u32,
    #[state(local_ui)] pub camera: FemCamera,
    #[state(preview)] pub solver_results_json: String,
    #[state(preview)] pub mesh_preview_json: String,
}
//#endregion 🔖️Artifact


//#region 🔖️Conversions
impl Default for Fem3dArtifact {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            elements: Default::default(),
            materials: Default::default(),
            sections: Default::default(),
            solids: Default::default(),
            supports: Default::default(),
            load_cases: Default::default(),
            combinations: Default::default(),
            analysis: Default::default(),
            result_source_id: None,
            result_mode: "static".into(),
            result_mode_index: 0,
            camera: FemCamera::default(),
            solver_results_json: String::new(),
            mesh_preview_json: String::new(),
        }
    }
}

impl Fem3dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::fem3d::Fem3dSnapshot {
        crate::artifacts::fem3d::Fem3dSnapshot {
            nodes: self.nodes.clone(), elements: self.elements.clone(), materials: self.materials.clone(), sections: self.sections.clone(), solids: self.solids.clone(), supports: self.supports.clone(), load_cases: self.load_cases.clone(), combinations: self.combinations.clone(), analysis: self.analysis.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI/preview fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::fem3d::Fem3dSnapshot) -> Self {
        Self {
            nodes: snapshot.nodes, elements: snapshot.elements, materials: snapshot.materials, sections: snapshot.sections, solids: snapshot.solids, supports: snapshot.supports, load_cases: snapshot.load_cases, combinations: snapshot.combinations, analysis: snapshot.analysis,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::fem3d::Fem3dSnapshot) {
        self.nodes = snapshot.nodes;
        self.elements = snapshot.elements;
        self.materials = snapshot.materials;
        self.sections = snapshot.sections;
        self.solids = snapshot.solids;
        self.supports = snapshot.supports;
        self.load_cases = snapshot.load_cases;
        self.combinations = snapshot.combinations;
        self.analysis = snapshot.analysis;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.fem.fem3d` — twenty handcrafted schema leaves.
pub fn fem3d_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.fem.fem3d",
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
