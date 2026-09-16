//! 🩹️ Fem2d play app command — `patch-combination`: renames a combination (`name`), re-weights one term (`term:<caseId>`) or appends one (`addTerm`) → `ReplaceCombination`.

use crate::standards::v1::subsets::any::schema::mutations::replace_combination;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use crate::FemCombinationTerm;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️Constants
/// 🔗️ Field prefix naming the term a factor edit addresses — one inspector number input per term is
/// bound as `term:<caseId>`, which is how a per-term control fits the flat `{id, field, value}`
/// patch shape without a second argument key.
pub const TERM_FIELD_PREFIX: &str = "term:";
/// ➕️ Field an "add a term" select binds: the CHOSEN case id arrives as the control's `value`, so
/// the field name has to stay constant across every option — `term:<caseId>` cannot express it.
pub const ADD_TERM_FIELD: &str = "addTerm";
/// ⚖️ Factor a freshly appended term starts at.
pub const APPENDED_TERM_FACTOR: f64 = 1.0;
//#endregion 🔖️Constants

//#region 🔖️PatchCombination
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-combination")]
pub struct PatchCombination {
    pub id: String,
    pub field: String,
    pub value: String,
}

/// 🩹️ Re-weights, drops or appends one combination term, or renames the combination. A factor of
/// exactly zero REMOVES the term rather than leaving a superposition that contributes nothing:
/// a zero-weighted term still pins the referenced case against deletion, which is not what the
/// person dialing the factor to zero asked for.
pub fn handle(payload: &PatchCombination, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let combination = doc.snapshot.combinations.iter().find(|combination| combination.id == payload.id).ok_or_else(|| Fault::from("fem2d.patch.combination-missing"))?;
    let mut new_combination = combination.clone();
    if let Some(case_id) = payload.field.strip_prefix(TERM_FIELD_PREFIX) {
        if case_id.is_empty() {
            return Err(Fault::from("fem2d.patch.combination-field"));
        }
        let factor: f64 = payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.combination-value"))?;
        if factor == 0.0 {
            new_combination.terms.retain(|term| term.case_id != case_id);
        } else if let Some(term) = new_combination.terms.iter_mut().find(|term| term.case_id == case_id) {
            term.factor = factor;
        } else {
            new_combination.terms.push(FemCombinationTerm { case_id: case_id.to_string(), factor });
        }
    } else {
        match payload.field.as_str() {
            "name" => new_combination.name = payload.value.clone(),
            ADD_TERM_FIELD => {
                let case_id = payload.value.trim();
                if case_id.is_empty() {
                    return Err(Fault::from("fem2d.patch.combination-value"));
                }
                if !new_combination.terms.iter().any(|term| term.case_id == case_id) {
                    new_combination.terms.push(FemCombinationTerm { case_id: case_id.to_string(), factor: APPENDED_TERM_FACTOR });
                }
            }
            _ => return Err(Fault::from("fem2d.patch.combination-field")),
        }
    }
    if &new_combination == combination {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem2dMutation::ReplaceCombination(replace_combination::ReplaceCombination { id: payload.id.clone(), new_combination })]))
}
//#endregion 🔖️PatchCombination

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
