//! 🧬️ Mathematical diff schema — sparse field delta over the artifact.

use crate::artifacts::mathematical::standards::v1::subsets::any::schema::snapshot::EquationSnapshot;
use crate::artifacts::mathematical::{MathematicalComputedChild, MathematicalNotationChild, MathematicalResultsChild};
use schema::ArtifactSchema;
use semio_framework_os_kernel::{from_dsl_value, to_dsl_value, DslValue, FromValue, ToValue, ValueError};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the mathematical artifact. `notation`/`results`/`computed`/`equation`
/// are always-present slots (never absent, only ever replaced) — single-`Option`, matching writer's
/// `document: Option<WriterDocumentChild>` diff shape, not lowpoly's optional-slot double-`Option`.
/// The former `artifact: Option<Box<MathematicalArtifact>>` whole-snapshot-replace slot is REMOVED:
/// it was dead code (never constructed by any app command — `SetArtifact` already routes through
/// the granular `ReplaceGraph`/`ReplacePoints` mutations) and would otherwise be exactly the banned
/// `SetSnapshot` whole-document-replace vocabulary this ticket's `📌️important.md` forbids. `equation`
/// (wave M3a, 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS) is a WHOLE-node
/// replace too — sparse WITHIN the tree happens via label-addressed mutation payloads
/// (`change-coefficient`'s `EquationNodeLabel`), never by diffing two `EquationNode` trees.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.mathematical.mathematical")]
pub struct MathematicalDiff {
    #[state(artifact)]
    pub notation: Option<MathematicalNotationChild>,
    #[state(artifact)]
    pub results: Option<MathematicalResultsChild>,
    #[state(artifact)]
    pub computed: Option<MathematicalComputedChild>,
    #[state(artifact)]
    pub equation: Option<EquationSnapshot>,
    #[state(config)]
    pub camera_x: Option<f64>,
    #[state(config)]
    pub camera_y: Option<f64>,
    #[state(config)]
    pub camera_zoom: Option<f64>,
    #[state(config)]
    pub locale: Option<String>,
}

// 🌱️ Hand-written, not derived — `notation`/`results`/`computed` are `Option<store::ArtifactChild<S>>`,
// and `ArtifactChild<S>` carries a `local_owner: Option<Arc<dyn Any>>` field a
// `#[derive(ToValue, FromValue)]` cannot route through (fan-out playbook trap #3; mirrors
// `📸️snapshot/🦀️component.rs`'s own `MathematicalSnapshot` impl for the non-`Option` version of the
// same three fields). Bridged per composed field through the PRE-EXISTING `to_dsl_value`/
// `from_dsl_value` serde bridge (framework-internal, exempt); every other field goes through
// `ToValue`/`FromValue` directly, relying on the blanket `Option<T: ToValue/FromValue>` impl.
impl ToValue for MathematicalDiff {
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
impl FromValue for MathematicalDiff {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = DslValue::into_object(value)?;
        let field = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone()).unwrap_or(DslValue::Null);
        Ok(Self {
            notation: from_dsl_value(field("notation")).map_err(ValueError::new)?,
            results: from_dsl_value(field("results")).map_err(ValueError::new)?,
            computed: from_dsl_value(field("computed")).map_err(ValueError::new)?,
            equation: Option::from_value(field("equation"))?,
            camera_x: Option::from_value(field("cameraX"))?,
            camera_y: Option::from_value(field("cameraY"))?,
            camera_zoom: Option::from_value(field("cameraZoom"))?,
            locale: Option::from_value(field("locale"))?,
        })
    }
}
//#endregion 🔖️Diff
