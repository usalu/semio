//! 🧹 Shooting mutation payload — `DeleteSavedCamera`. Removes a saved camera by id; inverse recreates it.

use crate::diff::ShootingDiff;
use crate::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct DeleteSavedCamera {
    pub id: String,
}

impl<Op> MutationKind<ShootingSnapshot, Op> for DeleteSavedCamera
where
    Op: protocol::Mutation<ShootingSnapshot, Diff = ShootingDiff> + From<DeleteSavedCamera> + From<crate::standards::v1::subsets::any::schema::mutations::create_saved_camera::CreateSavedCamera>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "saved-camera", kind: "delete-saved-camera", record: "DeletedSavedCamera" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete saved camera \"{}\"", self.id), &format!("Gespeicherte Kamera \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ShootingSnapshot, Op> for DeleteSavedCamera
where
    Op: protocol::ColdMutation<ShootingSnapshot, ReferenceDiff = ShootingDiff> + From<DeleteSavedCamera> + From<crate::standards::v1::subsets::any::schema::mutations::create_saved_camera::CreateSavedCamera>,
{
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
