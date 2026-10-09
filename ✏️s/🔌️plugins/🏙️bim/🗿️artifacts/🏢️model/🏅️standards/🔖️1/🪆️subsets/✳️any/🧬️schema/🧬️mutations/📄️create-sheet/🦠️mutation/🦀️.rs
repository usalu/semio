//! 📄️ `create-sheet` payload. Brings a new sheet into the drawing set: number, name, paper size, orientation and the authored fields of the title block. The frame, the title block and the revision table it prints are inferred.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Sheet};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateSheet {
    pub id: String,
    pub sheet: Sheet,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateSheet {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "sheet", kind: "create-sheet", record: "CreateSheet" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create sheet \"{}\"", self.sheet.number), &format!("Blatt \"{}\" anlegen", self.sheet.number))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
