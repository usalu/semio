//! 🎚️ Direct `change-exaggeration` mutation owner.
use crate::diff::GisTerrainDiff;
use crate::mutations::GisTerrainMutation;
use crate::GisTerrainSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 🎚️ Sets `GisTerrainSnapshot::exaggeration` to `new_exaggeration`. Diff/inverse delegate to the
/// sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-exaggeration")]
pub struct ChangeExaggeration {
    pub new_exaggeration: f64,
}

impl MutationKind<GisTerrainSnapshot, GisTerrainMutation> for ChangeExaggeration {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "exaggeration", kind: "change-exaggeration", record: "ChangedExaggeration" };

    fn diff(&self, base: &GisTerrainSnapshot) -> protocol::MutationOutcome<GisTerrainDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &GisTerrainSnapshot) -> Result<Vec<GisTerrainMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change terrain exaggeration to {}", self.new_exaggeration), &format!("Geländeüberhöhung auf {} ändern", self.new_exaggeration))
    }
}
//#endregion 🔹Payload
