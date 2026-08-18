//! 🧬️ Mathematical artifact schema — every field with its state class.

use crate::artifacts::mathematical::{MathematicalComputedChild, MathematicalGeometry, MathematicalGraph, MathematicalNotationChild, MathematicalResultsChild};
use crate::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::EquationSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full mathematical artifact across the artifact and config lanes. `notation`/`results`/
/// `computed` mirror `MathematicalSnapshot`'s own composed-child slots (ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM, `mathematical→C:text,table,value`); `equation`
/// mirrors its plain (non-`#[child]`) persistent sibling added in wave M3a of
/// 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.mathematical.mathematical")]
pub struct MathematicalArtifact {
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.text")]
    pub notation: MathematicalNotationChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.table")]
    pub results: MathematicalResultsChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.value")]
    pub computed: MathematicalComputedChild,
    #[state(artifact)]
    pub equation: EquationSnapshot,
    #[state(config)]
    pub camera_x: f64,
    #[state(config)]
    pub camera_y: f64,
    #[state(config)]
    pub camera_zoom: f64,
    #[state(config)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for MathematicalArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::artifacts::mathematical::MathematicalSnapshot::default())
    }
}

impl MathematicalArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::mathematical::MathematicalSnapshot {
        crate::artifacts::mathematical::MathematicalSnapshot { notation: self.notation.clone(), results: self.results.clone(), computed: self.computed.clone(), equation: self.equation.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::mathematical::MathematicalSnapshot) -> Self {
        Self { notation: snapshot.notation, results: snapshot.results, computed: snapshot.computed, equation: snapshot.equation, ..Self::default_ui() }
    }

    fn default_ui() -> Self {
        let default_snapshot = crate::artifacts::mathematical::mathematical_snapshot_with_state(MathematicalGraph::default(), MathematicalGeometry::default());
        Self {
            notation: default_snapshot.notation,
            results: default_snapshot.results,
            computed: default_snapshot.computed,
            equation: default_snapshot.equation,
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: 1.0,
            locale: "en-US".into(),
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::mathematical::MathematicalSnapshot) {
        self.notation = snapshot.notation;
        self.results = snapshot.results;
        self.computed = snapshot.computed;
        self.equation = snapshot.equation;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.mathematical.mathematical` — twenty handcrafted schema leaves.
pub fn mathematical_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.mathematical.mathematical",
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
