//! ✏️ Forms mutation payloads — the `steps` collection's `rename` and `change` verbs. Physical dir
//! name (`🩹update-step`, wired by `🦀️.rs`) predates the split of the old generic `UpdateStep`
//! (whole-struct patch) into two granular semantic mutations — title is meaningfully set alone
//! (`rename`), and so is description (`change`), so neither is an inseparable `update` facet.

use crate::{FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region 🔖️Mutation
/// 📝️ Sets a step's `description` scalar (a `None` clears it).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeStepDescription {
    pub id: String,
    pub new_description: Option<String>,
}

impl<Op> MutationKind<FormsSnapshot, Op> for ChangeStepDescription
where
    Op: protocol::Mutation<FormsSnapshot, Diff = FormsDiff> + From<ChangeStepDescription>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "step-description", kind: "change-step-description", record: "ChangedStepDescription" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change step \"{}\" description", self.id), &format!("Beschreibung von Schritt \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<FormsSnapshot, Op> for ChangeStepDescription
where
    Op: protocol::ColdMutation<FormsSnapshot, ReferenceDiff = FormsDiff> + From<ChangeStepDescription>,
{
    fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        crate::standards::v1::subsets::any::schema::mutations::change_step_description::diff::diff(self, base)
    }
    fn inverse(&self, base: &FormsSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::standards::v1::subsets::any::schema::mutations::change_step_description::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
