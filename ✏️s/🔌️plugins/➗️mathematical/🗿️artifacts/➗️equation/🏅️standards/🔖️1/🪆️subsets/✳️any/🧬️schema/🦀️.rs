//! 🧬️ Equation artifact schema — every field with its state class.

use crate::artifacts::equation::standards::v1::subsets::any::schema::snapshot::EquationExprSnapshot;
use crate::artifacts::equation::{EquationComputedChild, EquationGeometry, EquationGraph, EquationNotationChild, EquationResultsChild};
use schema::ArtifactSchema;
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
    #[state(config)]
    pub camera_x: f64,
    #[state(config)]
    pub camera_y: f64,
    #[state(config)]
    pub camera_zoom: f64,
    #[state(config)]
    pub locale: String,
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
            ("cameraX".to_string(), self.camera_x.to_value()),
            ("cameraY".to_string(), self.camera_y.to_value()),
            ("cameraZoom".to_string(), self.camera_zoom.to_value()),
            ("locale".to_string(), self.locale.to_value()),
        ])
    }
}
impl FromValue for EquationArtifact {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = DslValue::into_object(value)?;
        let field = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone()).unwrap_or(DslValue::Null);
        Ok(Self {
            notation: from_dsl_value(field("notation")).map_err(ValueError::new)?,
            results: from_dsl_value(field("results")).map_err(ValueError::new)?,
            computed: from_dsl_value(field("computed")).map_err(ValueError::new)?,
            equation: EquationExprSnapshot::from_value(field("equation"))?,
            camera_x: f64::from_value(field("cameraX"))?,
            camera_y: f64::from_value(field("cameraY"))?,
            camera_zoom: f64::from_value(field("cameraZoom"))?,
            locale: String::from_value(field("locale"))?,
        })
    }
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for EquationArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::artifacts::equation::EquationSnapshot::default())
    }
}

impl EquationArtifact {
    /// 📸️ Persisted subset.
    pub async fn to_snapshot(&self) -> crate::artifacts::equation::EquationSnapshot {
        crate::artifacts::equation::EquationSnapshot { notation: self.notation.clone(), results: self.results.clone(), computed: self.computed.clone(), equation: self.equation.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub async fn from_snapshot(snapshot: crate::artifacts::equation::EquationSnapshot) -> Self {
        Self { notation: snapshot.notation, results: snapshot.results, computed: snapshot.computed, equation: snapshot.equation, ..Self::default_ui() }
    }

    async fn default_ui() -> Self {
        let default_snapshot = crate::artifacts::equation::equation_snapshot_with_state(EquationGraph::default(), EquationGeometry::default());
        Self { notation: default_snapshot.notation, results: default_snapshot.results, computed: default_snapshot.computed, equation: default_snapshot.equation, camera_x: 0.0, camera_y: 0.0, camera_zoom: 1.0, locale: "en-US".into() }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub async fn set_snapshot(&mut self, snapshot: crate::artifacts::equation::EquationSnapshot) {
        self.notation = snapshot.notation;
        self.results = snapshot.results;
        self.computed = snapshot.computed;
        self.equation = snapshot.equation;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.mathematical.equation` — twenty handcrafted schema leaves.
pub async fn equation_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.mathematical.equation",
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
