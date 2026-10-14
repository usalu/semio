//! 🔧 `change-step-origin` payload — sets an id-keyed [`ProcessStep`]'s `origin` provenance
//! (machine/capability the step was built from) to an explicit new value, or clears it.

use crate::diff::Process3dDiff;
use crate::{Process3dSnapshot, StepOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeStepOrigin
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeStepOrigin {
    pub id: String,
    pub new_origin: Option<StepOrigin>,
}

impl<Op> protocol::MutationKind<Process3dSnapshot, Op> for ChangeStepOrigin
where
    Op: protocol::Mutation<Process3dSnapshot, Diff = crate::standards::v1::subsets::any::schema::diff::Process3dDiff> + From<crate::standards::v1::subsets::any::schema::mutations::change_step_origin::ChangeStepOrigin>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "step", kind: "change-step-origin", record: "ChangedStepOrigin" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change origin of step \"{}\"", self.id), &format!("Ursprung von Schritt \"{}\" ändern", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ChangeStepOrigin
