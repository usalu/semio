//! 🗑️ ProgramSnapshot mutation — `delete-register-row` leaf: the generic delete over every id-keyed register, typed by the
//! register catalog generated from the `x-semio-collection-catalog` of `📸️snapshot/🔣️.json`.

use crate::standards::v1::subsets::any::schema::kernel::EntityId;
use crate::standards::v1::subsets::any::schema::registers::catalog::RegisterKey;
use crate::standards::v1::subsets::any::schema::{diff::ProgramDiff, snapshot::ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🗑️ Removes one row, addressed by register and id; `↩️inverse` recreates it at its position.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DeleteRegisterRow {
    pub register: RegisterKey,
    pub id: EntityId,
}

impl<Op> MutationKind<ProgramSnapshot, Op> for DeleteRegisterRow
where
    Op: protocol::Mutation<ProgramSnapshot, Diff = ProgramDiff> + From<DeleteRegisterRow> + From<crate::standards::v1::subsets::any::schema::mutations::create_register_row::CreateRegisterRow>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "register-row", kind: "delete-register-row", record: "DeletedRegisterRow" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let label = self.register.label();
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete {} \"{}\"", label.en, self.id.0), &format!("{} \"{}\" löschen", label.de, self.id.0))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
