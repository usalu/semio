//! ♻️ ProgramSnapshot mutation — `replace-register-row` leaf: the generic replace over every id-keyed register, typed by the
//! register catalog generated from the `x-semio-collection-catalog` of `📸️snapshot/🔣️.json`.

use crate::standards::v1::subsets::any::schema::registers::RegisterRow;
use crate::standards::v1::subsets::any::schema::{diff::ProgramDiff, mutations::ProgramMutation, snapshot::ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// ♻️ Whole-value swap of one row's non-identity content, addressed by the register and id `row` carries.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ReplaceRegisterRow {
    pub row: RegisterRow,
}

impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceRegisterRow {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "register-row", kind: "replace-register-row", record: "ReplacedRegisterRow" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let label = self.row.key().label();
        let name = &self.row.header().name;
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace {} \"{name}\"", label.en), &format!("{} \"{name}\" ersetzen", label.de))
    }
    fn target(&self) -> Vec<String> {
        vec![self.row.header().id.0.clone()]
    }
}
