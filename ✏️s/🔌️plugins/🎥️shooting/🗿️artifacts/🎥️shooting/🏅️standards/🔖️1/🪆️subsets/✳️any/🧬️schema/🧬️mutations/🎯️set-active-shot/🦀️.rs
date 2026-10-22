//! 🎯 Shooting mutation payload — `SetActiveShot`. A narrow addressed single-field setter on the document root (taxonomy's `set` verb; NOT the banned whole-document `set-snapshot`).

use crate::diff::ShootingDiff;
use crate::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SetActiveShot {
    pub shot_id: Option<String>,
}

impl<Op> MutationKind<ShootingSnapshot, Op> for SetActiveShot
where
    Op: protocol::Mutation<ShootingSnapshot, Diff = ShootingDiff> + From<SetActiveShot>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "active-shot", kind: "set-active-shot", record: "SetActiveShot" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&{
        match &self.shot_id {
            Some(id) => format!("Set active shot to \"{id}\""),
            None => "Clear active shot".into(),
        }
        }, &{
        match &self.shot_id {
            Some(id) => format!("Aktive Aufnahme auf \"{id}\" setzen"),
            None => "Aktive Aufnahme entfernen".into(),
        }
        })
    }
}

impl<Op> protocol::ColdMutationKind<ShootingSnapshot, Op> for SetActiveShot
where
    Op: protocol::ColdMutation<ShootingSnapshot, ReferenceDiff = ShootingDiff> + From<SetActiveShot>,
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
