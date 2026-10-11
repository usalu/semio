//! 📷 Shooting mutation payload — `ReplaceShotCamera`. Overwrites the *saved* camera `shot_id` references with a new pose — a no-op (empty diff) when that shot has no saved camera. The free/live viewport camera is session-only runtime state and never reaches this mutation.

use crate::diff::ShootingDiff;
use crate::mutations::ShootingMutation;
use crate::{ShootingCamera, ShootingSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ReplaceShotCamera {
    pub shot_id: String,
    pub new_camera: ShootingCamera,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ReplaceShotCamera {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "shot-camera", kind: "replace-shot-camera", record: "ReplacedShotCamera" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace shot \"{}\" camera", self.shot_id), &format!("Kamera von Aufnahme \"{}\" ersetzen", self.shot_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.shot_id.clone()]
    }
}
