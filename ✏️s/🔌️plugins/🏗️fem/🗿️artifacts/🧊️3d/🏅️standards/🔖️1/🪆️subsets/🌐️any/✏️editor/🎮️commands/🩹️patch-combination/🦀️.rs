//! 🩹️ Fem3d play app command — `patch-combination`: one-field edit of a combination (`name`, a
//! `term:<caseId>` factor, `addTerm` with the case id as value, `removeTerm`) → `ReplaceCombination`.

use crate::standards::v1::subsets::any::schema::mutations::replace_combination;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem3dSnapshot = crate::Fem3dSnapshot;

/// ⚖️ The field prefix a term factor row binds: `term:<caseId>`.
pub const TERM_FIELD_PREFIX: &str = "term:";
/// ➕️ The field the add-term select binds; the chosen case id rides the control's own value.
pub const ADD_TERM_FIELD: &str = "addTerm";
/// ➖️ The field a remove-term button binds; the case id rides the value.
pub const REMOVE_TERM_FIELD: &str = "removeTerm";
/// ⚖️ The factor a freshly added term starts at — the characteristic (1.0) weighting.
pub const ADDED_TERM_FACTOR: f64 = 1.0;

//#region 🔖️PatchCombination
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-combination")]
pub struct PatchCombination {
    pub id: String,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchCombination, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let combination = doc.snapshot.combinations.iter().find(|combination| combination.id == payload.id).ok_or_else(|| Fault::from("fem3d.patch.combination-missing"))?;
    let mut new_combination = combination.clone();
    let value = payload.value.trim();
    match payload.field.as_str() {
        "name" => new_combination.name = payload.value.clone(),
        ADD_TERM_FIELD => {
            if value.is_empty() {
                return Ok(Emit::default());
            }
            new_combination.terms.entry(value.to_string()).or_insert(ADDED_TERM_FACTOR);
        }
        REMOVE_TERM_FIELD => {
            new_combination.terms.remove(value);
        }
        field => {
            let case_id = field.strip_prefix(TERM_FIELD_PREFIX).ok_or_else(|| Fault::from("fem3d.patch.combination-field"))?;
            let factor: f64 = value.parse().map_err(|_| Fault::from("fem3d.patch.combination-value"))?;
            let slot = new_combination.terms.get_mut(case_id).ok_or_else(|| Fault::from("fem3d.patch.combination-term-missing"))?;
            *slot = factor;
        }
    }
    if &new_combination == combination {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem3dMutation::ReplaceCombination(replace_combination::ReplaceCombination { id: payload.id.clone(), new_combination })]))
}
//#endregion 🔖️PatchCombination

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
