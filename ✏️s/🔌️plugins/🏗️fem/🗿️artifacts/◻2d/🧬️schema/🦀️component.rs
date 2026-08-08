//! 🧬️ Fem2d artifact schema — every field of the artifact with its state class.

use crate::artifacts::fem2d::{FemAnalysisSettings, FemCamera, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full fem2d artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.fem.fem2d")]
pub struct Fem2dArtifact {
    #[state(persistent)] pub nodes: Vec<FemNode>,
    #[state(persistent)] pub elements: Vec<FemElement>,
    #[state(persistent)] pub regions: Vec<FemRegion>,
    #[state(persistent)] pub materials: Vec<FemMaterial>,
    #[state(persistent)] pub sections: Vec<FemSection>,
    #[state(persistent)] pub supports: Vec<FemSupport>,
    #[state(persistent)] pub load_cases: Vec<FemLoadCase>,
    #[state(persistent)] pub combinations: Vec<FemCombination>,
    #[state(persistent)] pub analysis: FemAnalysisSettings,
    #[state(shared_ui)] pub result_source_id: Option<String>,
    #[state(shared_ui)] pub result_mode: String,
    #[state(shared_ui)] pub result_mode_index: u32,
    #[state(local_ui)] pub camera: FemCamera,
    #[state(local_ui)] pub locale: String,
    #[state(preview)] pub solver_results_json: String,
    #[state(preview)] pub mesh_preview_json: String,
}
//#endregion 🔖️Artifact


//#region 🔖️Conversions
impl Default for Fem2dArtifact {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            elements: Default::default(),
            regions: Default::default(),
            materials: Default::default(),
            sections: Default::default(),
            supports: Default::default(),
            load_cases: Default::default(),
            combinations: Default::default(),
            analysis: Default::default(),
            result_source_id: None,
            result_mode: "static".into(),
            result_mode_index: 0,
            camera: FemCamera::default(),
            locale: "en-US".into(),
            solver_results_json: String::new(),
            mesh_preview_json: String::new(),
        }
    }
}

impl Fem2dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::fem2d::Fem2dSnapshot {
        crate::artifacts::fem2d::Fem2dSnapshot {
            nodes: self.nodes.clone(), elements: self.elements.clone(), regions: self.regions.clone(), materials: self.materials.clone(), sections: self.sections.clone(), supports: self.supports.clone(), load_cases: self.load_cases.clone(), combinations: self.combinations.clone(), analysis: self.analysis.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI/preview fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::fem2d::Fem2dSnapshot) -> Self {
        Self {
            nodes: snapshot.nodes, elements: snapshot.elements, regions: snapshot.regions, materials: snapshot.materials, sections: snapshot.sections, supports: snapshot.supports, load_cases: snapshot.load_cases, combinations: snapshot.combinations, analysis: snapshot.analysis,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::fem2d::Fem2dSnapshot) {
        self.nodes = snapshot.nodes;
        self.elements = snapshot.elements;
        self.regions = snapshot.regions;
        self.materials = snapshot.materials;
        self.sections = snapshot.sections;
        self.supports = snapshot.supports;
        self.load_cases = snapshot.load_cases;
        self.combinations = snapshot.combinations;
        self.analysis = snapshot.analysis;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.fem.fem2d` — fifteen handcrafted schema leaves.
pub fn fem2d_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.fem.fem2d",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
