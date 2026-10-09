//! 📋️ `align-elements` payload. Moves placed elements along one axis of the plan so that the lower edge, the middle or the upper edge of the authored extent of each element lies on a target coordinate; arcs count with their extremes, hosted openings follow their host.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct AlignElements {
    pub ids: Vec<String>,
    pub axis: super::super::modify::AlignAxis,
    pub edge: super::super::modify::AlignEdge,
    pub target: f64,
}

impl MutationKind<ModelSnapshot, ModelMutation> for AlignElements {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "elements", kind: "align-elements", record: "AlignElements" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Align {} element(s) to {:?} = {} m", self.ids.len(), self.axis, self.target), &format!("{} Element(e) auf {:?} = {} m ausrichten", self.ids.len(), self.axis, self.target))
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
