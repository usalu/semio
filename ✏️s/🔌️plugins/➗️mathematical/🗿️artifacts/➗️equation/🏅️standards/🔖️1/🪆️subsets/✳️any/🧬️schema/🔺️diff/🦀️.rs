//! 🧬️ Equation diff schema — sparse field delta over the artifact.

use crate::standards::v1::subsets::any::schema::snapshot::EquationExprSnapshot;
use crate::{EquationComputedChild, EquationGeometry, EquationGraph, EquationNotationChild, EquationResultsChild};
use framework_schema::ArtifactSchema;
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the equation artifact. Every field is an always-present slot (never absent, only ever replaced):
/// a state leaf replaces `graph`/`geometry` together with the derived handles it re-mints (`crate::equation_state_diff`).
/// The former `artifact: Option<Box<EquationArtifact>>` whole-snapshot-replace slot is REMOVED:
/// it was dead code (never constructed by any app command — `SetArtifact` already routes through
/// the granular `ReplaceGraph`/`ReplacePoints` mutations) and would otherwise be exactly the banned
/// `SetSnapshot` whole-document-replace vocabulary this ticket's `📌️important.md` forbids. `equation`
/// (wave M3a, 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS) is a WHOLE-node
/// replace too — sparse WITHIN the tree happens via label-addressed mutation payloads
/// (`change-coefficient`'s `EquationNodeLabel`), never by diffing two `EquationNode` trees.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.mathematical.equation")]
pub struct EquationDiff {
    #[state(artifact)]
    pub graph: Option<EquationGraph>,
    #[state(artifact)]
    pub geometry: Option<EquationGeometry>,
    #[state(artifact)]
    pub notation: Option<EquationNotationChild>,
    #[state(artifact)]
    pub results: Option<EquationResultsChild>,
    #[state(artifact)]
    pub computed: Option<EquationComputedChild>,
    #[state(artifact)]
    pub equation: Option<EquationExprSnapshot>,
}

// 🌱️ Hand-written, not derived — `notation`/`results`/`computed` are `Option<store::ArtifactChild<S>>`, a generic framework
// handle `#[derive(ToValue, FromValue)]` cannot route through (fan-out playbook trap #3); every other field goes through the
// blanket `Option<T: ToValue/FromValue>` impl.
impl ToValue for EquationDiff {
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
impl FromValue for EquationDiff {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = DslValue::into_object(value)?;
        let field = |key: &str| entries.iter().find(|(k, _)| k == key).map_or(DslValue::Null, |(_, v)| v.clone());
        Ok(Self {
            graph: Option::from_value(field("graph"))?,
            geometry: Option::from_value(field("geometry"))?,
            notation: semio_framework_value::FromValue::from_value(field("notation"))?,
            results: semio_framework_value::FromValue::from_value(field("results"))?,
            computed: semio_framework_value::FromValue::from_value(field("computed"))?,
            equation: Option::from_value(field("equation"))?,
        })
    }
}
//#endregion 🔖️Diff
