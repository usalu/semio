//! 🐘 `update-climate` payload — replaces the Din18599 document's parent-owned `climate` facet
//! (monthly external temperature + solar irradiance profile); the derived `climateTable` child follows. Per `📓️derivation-rules.md` rule 1's
//! `update-<facet>` exception: `MonthlyClimate`'s two twelve-month arrays (`theta_e_c`, `g_h_w_m2`)
//! are entered together as one climate dataset (e.g. loaded from a reference climate zone via
//! `MonthlyClimate::german_reference`), never meaningfully edited one month/array at a time from this
//! app's own input surface — an inseparable ≥2-field facet, not independently-set scalars.

use crate::diff::Din18599Diff;
use crate::mutations::Din18599Mutation;
use crate::{Din18599Snapshot, MonthlyClimate};
//#region 🔖️UpdateClimate
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct UpdateClimate {
    pub new_climate: MonthlyClimate,
}

impl protocol::MutationKind<Din18599Snapshot, Din18599Mutation> for UpdateClimate {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "climate", kind: "update-climate", record: "UpdatedClimate" };

    fn diff(&self, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din18599Snapshot) -> Result<Vec<Din18599Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Update monthly climate profile", "Monatsklimaprofil aktualisieren")
    }
}
//#endregion 🔖️UpdateClimate
