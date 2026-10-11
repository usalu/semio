//! 🔧 `change-machine-icon` payload — changes an id-keyed [`WorkshopMachine`]'s `icon_id`.

use crate::diff::Process3dDiff;
use crate::mutations::Process3dMutation;
use crate::Process3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeMachineIcon
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeMachineIcon {
    pub id: String,
    pub new_icon_id: String,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for ChangeMachineIcon {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "machine", kind: "change-machine-icon", record: "ChangedMachineIcon" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Result<Vec<Process3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change icon of machine \"{}\"", self.id), &format!("Symbol von Maschine \"{}\" ändern", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ChangeMachineIcon
