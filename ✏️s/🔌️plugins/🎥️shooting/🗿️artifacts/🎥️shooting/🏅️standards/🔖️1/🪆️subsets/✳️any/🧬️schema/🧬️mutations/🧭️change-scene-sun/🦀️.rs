//! 🧭️ Shooting mutation payload — `ChangeSceneSunAzimuth`. One of the scene's independently-settable fields.

use crate::diff::ShootingDiff;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeSceneSunAzimuth {
    pub new_azimuth: f64,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneSunAzimuth {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-sun-azimuth", kind: "change-scene-sun-azimuth", record: "ChangedSceneSunAzimuth" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change sun azimuth to {}", self.new_azimuth), &format!("Sonnenazimut auf {} ändern", self.new_azimuth))
    }
}
