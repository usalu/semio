//! 📓️ `create-viewport` payload. Places a view on a sheet: a plan, ceiling plan, section or elevation at a scale from 1:1 to 1:1000, with the position of its window on the paper, an optional crop of the drawing and an optional label. The drawing is the inferred linework of the view.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Viewport};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateViewport {
    pub id: String,
    pub viewport: Viewport,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateViewport {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "viewport", kind: "create-viewport", record: "CreateViewport" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Place view \"{}\" on sheet \"{}\"", self.viewport.view, self.viewport.sheet), &format!("Ansicht \"{}\" auf Blatt \"{}\" platzieren", self.viewport.view, self.viewport.sheet))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
