//! 🏷️ `rename-storey` payload. Changes a storey's display name.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RenameStorey {
    pub id: String,
    pub name: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for RenameStorey
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<RenameStorey>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "storey", kind: "rename-storey", record: "RenameStorey" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::rename_storey::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::rename_storey::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename storey to \"{}\"", self.name), &format!("Geschoss in \"{}\" umbenennen", self.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
