//! 🧬️ Equation artifact schema — every field with its state class.

use crate::standards::v1::subsets::any::schema::snapshot::EquationExprSnapshot;
use crate::{EquationComputedChild, EquationNotationChild, EquationResultsChild};
use framework_schema::ArtifactSchema;
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};
//#region 🔖️Artifact
/// 🧬️ Full equation artifact across the artifact and config lanes, mirroring `EquationSnapshot`: the parent-owned `graph`
/// and `geometry`, their derived composed-child handles and the authored `equation`.
#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.mathematical.equation")]
pub struct EquationArtifact {
    #[state(artifact)]
    pub graph: EquationGraph,
    #[state(artifact)]
    pub geometry: EquationGeometry,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub notation: EquationNotationChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub results: EquationResultsChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub computed: EquationComputedChild,
    #[state(artifact)]
    pub equation: EquationExprSnapshot,
}

// 🌱️ Hand-written, not derived — `notation`/`results`/`computed` are `store::ArtifactChild<S>` (fan-out playbook trap #3,
// same shape as `📸️snapshot/🦀️.rs`'s `EquationSnapshot` impl); every other field goes through `ToValue`/`FromValue`.
impl ToValue for EquationArtifact {
    fn to_value(&self) -> DslValue {
        DslValue::object([
            ("graph".to_string(), self.graph.to_value()),
            ("geometry".to_string(), self.geometry.to_value()),
            ("notation".to_string(), semio_framework_value::ToValue::to_value(&self.notation)),
            ("results".to_string(), semio_framework_value::ToValue::to_value(&self.results)),
            ("computed".to_string(), semio_framework_value::ToValue::to_value(&self.computed)),
            ("equation".to_string(), self.equation.to_value()),
        ])
    }
}
impl FromValue for EquationArtifact {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = DslValue::into_object(value)?;
        let field = |key: &str| entries.iter().find(|(k, _)| k == key).map_or(DslValue::Null, |(_, v)| v.clone());
        Ok(Self {
            graph: EquationGraph::from_value(field("graph"))?,
            geometry: EquationGeometry::from_value(field("geometry"))?,
            notation: semio_framework_value::FromValue::from_value(field("notation"))?,
            results: semio_framework_value::FromValue::from_value(field("results"))?,
            computed: semio_framework_value::FromValue::from_value(field("computed"))?,
            equation: EquationExprSnapshot::from_value(field("equation"))?,
        })
    }
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for EquationArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::EquationSnapshot::default())
    }
}

impl EquationArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::EquationSnapshot {
        crate::EquationSnapshot { graph: self.graph.clone(), geometry: self.geometry.clone(), notation: self.notation.clone(), results: self.results.clone(), computed: self.computed.clone(), equation: self.equation.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: crate::EquationSnapshot) -> Self {
        Self { graph: snapshot.graph, geometry: snapshot.geometry, notation: snapshot.notation, results: snapshot.results, computed: snapshot.computed, equation: snapshot.equation }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::EquationSnapshot) {
        self.graph = snapshot.graph;
        self.geometry = snapshot.geometry;
        self.notation = snapshot.notation;
        self.results = snapshot.results;
        self.computed = snapshot.computed;
        self.equation = snapshot.equation;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.mathematical.equation` — twenty handcrafted schema leaves.
pub fn equation_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.mathematical.equation",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::{EquationGeometry, EquationGraph};
//#endregion 🔁️Re-exports
