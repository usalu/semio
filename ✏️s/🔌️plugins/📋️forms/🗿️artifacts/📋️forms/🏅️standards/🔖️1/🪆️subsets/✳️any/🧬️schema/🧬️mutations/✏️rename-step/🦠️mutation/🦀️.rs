//! ✏️ Forms mutation payloads — the `steps` collection's `rename` and `change` verbs. Physical dir
//! name (`🩹update-step`, wired by `🦀️.rs`) predates the split of the old generic `UpdateStep`
//! (whole-struct patch) into two granular semantic mutations — title is meaningfully set alone
//! (`rename`), and so is description (`change`), so neither is an inseparable `update` facet.

use crate::{FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region 🏷️RenameStep
/// 🏷️ Changes a step's identity `title` field.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RenameStep {
    pub id: String,
    pub new_title: String,
}

impl<Op> MutationKind<FormsSnapshot, Op> for RenameStep
where
    Op: protocol::Mutation<FormsSnapshot, Diff = FormsDiff> + From<RenameStep>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "step", kind: "rename-step", record: "RenamedStep" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename step to \"{}\"", self.new_title), &format!("Schritt in \"{}\" umbenennen", self.new_title))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<FormsSnapshot, Op> for RenameStep
where
    Op: protocol::ColdMutation<FormsSnapshot, ReferenceDiff = FormsDiff> + From<RenameStep>,
{
    fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        crate::standards::v1::subsets::any::schema::mutations::rename_step::diff::diff(self, base)
    }
    fn inverse(&self, base: &FormsSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::standards::v1::subsets::any::schema::mutations::rename_step::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🏷️RenameStep

//#region 📝️ChangeStepDescription

//#endregion 📝️ChangeStepDescription
