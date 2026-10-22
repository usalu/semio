//! 🏷️ ProgramSnapshot mutation — `rename-register-row` leaf: the generic rename over every id-keyed register, typed by the
//! register catalog generated from the `x-semio-collection-catalog` of `📸️snapshot/🔣️.json`.

use crate::standards::v1::subsets::any::schema::kernel::EntityId;
use crate::standards::v1::subsets::any::schema::registers::catalog::RegisterKey;
use crate::standards::v1::subsets::any::schema::{diff::ProgramDiff, snapshot::ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🏷️ Sets the identity `name` of one row, addressed by register and id.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct RenameRegisterRow {
    pub register: RegisterKey,
    pub id: EntityId,
    pub new_name: String,
}

impl<Op> MutationKind<ProgramSnapshot, Op> for RenameRegisterRow
where
    Op: protocol::Mutation<ProgramSnapshot, Diff = ProgramDiff> + From<RenameRegisterRow>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "register-row", kind: "rename-register-row", record: "RenamedRegisterRow" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let label = self.register.label();
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename {} to \"{}\"", label.en, self.new_name), &format!("{} in \"{}\" umbenennen", label.de, self.new_name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ProgramSnapshot, Op> for RenameRegisterRow
where
    Op: protocol::ColdMutation<ProgramSnapshot, ReferenceDiff = ProgramDiff> + From<RenameRegisterRow>,
{
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
}
