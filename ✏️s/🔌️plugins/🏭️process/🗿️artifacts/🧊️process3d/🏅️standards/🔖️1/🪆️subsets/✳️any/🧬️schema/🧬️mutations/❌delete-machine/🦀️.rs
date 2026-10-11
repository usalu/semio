//! 🗑️ `delete-machine` payload — removes an id-keyed [`WorkshopMachine`] from the document's
//! workshop.

use crate::diff::Process3dDiff;
use crate::mutations::Process3dMutation;
use crate::Process3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️DeleteMachine
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteMachine {
    pub id: String,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for DeleteMachine {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "machine", kind: "delete-machine", record: "DeletedMachine" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Result<Vec<Process3dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete machine \"{}\"", self.id), &format!("Maschine \"{}\" löschen", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DeleteMachine
