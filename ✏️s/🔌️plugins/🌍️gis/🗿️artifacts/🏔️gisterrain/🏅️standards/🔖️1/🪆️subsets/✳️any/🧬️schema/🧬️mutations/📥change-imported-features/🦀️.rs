//! 📥️ Direct `change-imported-features` mutation owner — sets the terrain's last-imported `2d.map`
//! complete typed map (the `map:in` insertion point).
use crate::diff::GisTerrainDiff;
use crate::mutations::GisTerrainMutation;
use crate::GisTerrainSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔹Payload
/// 📥️ Sets `GisTerrainSnapshot::imported_map` to `new_imported_map`. Diff/
/// inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-imported-features")]
pub struct ChangeImportedFeatures {
    #[dsl(key = "new-imported-map")]
    #[value(default, skip_serializing_if="Option::is_none")]
    pub new_imported_map: Option<crate::schema::ImportedMap>,
}

impl MutationKind<GisTerrainSnapshot, GisTerrainMutation> for ChangeImportedFeatures {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "imported-features", kind: "change-imported-features", record: "ChangedImportedFeatures" };

    fn diff(&self, base: &GisTerrainSnapshot) -> protocol::MutationOutcome<GisTerrainDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &GisTerrainSnapshot) -> Result<Vec<GisTerrainMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change imported terrain features", "Importierte Geländemerkmale ändern")
    }
}
//#endregion 🔹Payload
