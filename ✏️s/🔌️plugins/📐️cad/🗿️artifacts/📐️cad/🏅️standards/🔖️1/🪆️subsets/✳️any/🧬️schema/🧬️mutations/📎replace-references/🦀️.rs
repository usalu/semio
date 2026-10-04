//! 📎️ CAD mutation — `ReplaceReferences` payload + `MutationKind` impl.

use crate::mutations::CadMutation;
use crate::{CadReference, CadSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
/// 📎️ Whole-value swap of one model definition's entire reference-overlay list.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-references")]
pub struct ReplaceReferences {
    pub model_definition_id: String,
    #[dsl(table)]
    pub references: Vec<CadReference>,
}

impl MutationKind<CadSnapshot, CadMutation> for ReplaceReferences {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "references", kind: "replace-references", record: "ReplacedReferences" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Result<Vec<CadMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace references for \"{}\"", self.model_definition_id), &format!("Referenzen für \"{}\" ersetzen", self.model_definition_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.model_definition_id.clone()]
    }
}
//#endregion 🔖️Mutation
