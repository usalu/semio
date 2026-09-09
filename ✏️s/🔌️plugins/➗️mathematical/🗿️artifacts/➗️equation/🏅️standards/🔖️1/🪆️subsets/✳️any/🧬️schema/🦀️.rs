//! 🧬️ Equation artifact schema — every field with its state class.

use crate::standards::v1::subsets::any::schema::snapshot::EquationExprSnapshot;
use crate::{EquationComputedChild, EquationNotationChild, EquationResultsChild};
use framework_schema::ArtifactSchema;
use semio_framework_os_kernel::{from_dsl_value, to_dsl_value, DslValue, FromValue, ToValue, ValueError};
//#region 🔖️Artifact
/// 🧬️ Full equation artifact across the artifact and config lanes. `notation`/`results`/
/// `computed` mirror `EquationSnapshot`'s own composed-child slots (ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM, `equation→C:text,table,value`); `equation`
/// mirrors its plain (non-`#[child]`) persistent sibling added in wave M3a of
/// 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.mathematical.equation")]
pub struct EquationArtifact {
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.text")]
    pub notation: EquationNotationChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.table")]
    pub results: EquationResultsChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.value")]
    pub computed: EquationComputedChild,
    #[state(artifact)]
    pub equation: EquationExprSnapshot,
}

// 🌱️ Hand-written, not derived — `notation`/`results`/`computed` are `store::ArtifactChild<S>`
// (fan-out playbook trap #3, same shape as `📸️snapshot/🦀️.rs`'s `EquationSnapshot`
// impl). Bridged per composed field through the PRE-EXISTING `to_dsl_value`/`from_dsl_value` serde
// bridge (framework-internal, exempt); every other field goes through `ToValue`/`FromValue` directly.
impl ToValue for EquationArtifact {
    fn to_value(&self) -> DslValue {
        DslValue::object([
            ("notation".to_string(), to_dsl_value(&self.notation).unwrap_or(DslValue::Null)),
            ("results".to_string(), to_dsl_value(&self.results).unwrap_or(DslValue::Null)),
            ("computed".to_string(), to_dsl_value(&self.computed).unwrap_or(DslValue::Null)),
            ("equation".to_string(), self.equation.to_value()),
        ])
    }
}
impl FromValue for EquationArtifact {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = DslValue::into_object(value)?;
        let field = |key: &str| entries.iter().find(|(k, _)| k == key).map_or(DslValue::Null, |(_, v)| v.clone());
        Ok(Self {
            notation: from_dsl_value(field("notation")).map_err(ValueError::new)?,
            results: from_dsl_value(field("results")).map_err(ValueError::new)?,
            computed: from_dsl_value(field("computed")).map_err(ValueError::new)?,
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
        crate::EquationSnapshot { notation: self.notation.clone(), results: self.results.clone(), computed: self.computed.clone(), equation: self.equation.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: crate::EquationSnapshot) -> Self {
        Self { notation: snapshot.notation, results: snapshot.results, computed: snapshot.computed, equation: snapshot.equation, ..Self::default_ui() }
    }

    fn default_ui() -> Self {
        let default_snapshot = crate::equation_snapshot_with_state(EquationGraph::default(), EquationGeometry::default());
        Self { notation: default_snapshot.notation, results: default_snapshot.results, computed: default_snapshot.computed, equation: default_snapshot.equation }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::EquationSnapshot) {
        self.notation = snapshot.notation;
        self.results = snapshot.results;
        self.computed = snapshot.computed;
        self.equation = snapshot.equation;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.mathematical.equation` — twenty handcrafted schema leaves.
pub fn equation_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.mathematical.equation",
        artifact: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: framework_schema::FacetLeaves {
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
pub use crate::EquationGeometry;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::EquationGraph;
//#endregion 🔁️Re-exports
