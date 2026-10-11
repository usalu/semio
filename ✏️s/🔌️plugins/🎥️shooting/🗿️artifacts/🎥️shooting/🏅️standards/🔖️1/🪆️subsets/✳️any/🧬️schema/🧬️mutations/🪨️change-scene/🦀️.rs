//! 🪨️ Shooting mutation payload — `ChangeSceneMaterialRoughness`. One of the scene's independently-settable fields.

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
pub struct ChangeSceneMaterialRoughness {
    pub new_roughness: f64,
}

impl MutationKind<ShootingSnapshot, ShootingMutation> for ChangeSceneMaterialRoughness {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "scene-material-roughness", kind: "change-scene-material-roughness", record: "ChangedSceneMaterialRoughness" };
    fn diff(&self, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change material roughness to {}", self.new_roughness), &format!("Materialrauigkeit auf {} ändern", self.new_roughness))
    }
}
