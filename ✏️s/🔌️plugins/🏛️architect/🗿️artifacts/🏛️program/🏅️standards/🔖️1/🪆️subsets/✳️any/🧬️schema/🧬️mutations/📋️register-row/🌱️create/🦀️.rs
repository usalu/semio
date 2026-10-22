//! 🌱️ ProgramSnapshot mutation — `create-register-row` leaf: the generic create over every id-keyed register, typed by the
//! register catalog generated from the `x-semio-collection-catalog` of `📸️snapshot/🔣️.json`.

use crate::standards::v1::subsets::any::schema::registers::catalog::RegisterRow;
use crate::standards::v1::subsets::any::schema::{diff::ProgramDiff, snapshot::ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🌱️ Brings a new row into existence in the register its `row` names, appended unless `index` places it.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CreateRegisterRow {
    pub row: RegisterRow,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub index: Option<usize>,
}

impl<Op> MutationKind<ProgramSnapshot, Op> for CreateRegisterRow
where
    Op: protocol::Mutation<ProgramSnapshot, Diff = ProgramDiff> + From<CreateRegisterRow> + From<crate::standards::v1::subsets::any::schema::mutations::delete_register_row::DeleteRegisterRow>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "register-row", kind: "create-register-row", record: "CreatedRegisterRow" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let label = self.row.key().label();
        let name = &self.row.header().name;
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create {} \"{name}\"", label.en), &format!("{} \"{name}\" erstellen", label.de))
    }
    fn target(&self) -> Vec<String> {
        vec![self.row.header().id.0.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ProgramSnapshot, Op> for CreateRegisterRow
where
    Op: protocol::ColdMutation<ProgramSnapshot, ReferenceDiff = ProgramDiff> + From<CreateRegisterRow> + From<crate::standards::v1::subsets::any::schema::mutations::delete_register_row::DeleteRegisterRow>,
{
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
}
