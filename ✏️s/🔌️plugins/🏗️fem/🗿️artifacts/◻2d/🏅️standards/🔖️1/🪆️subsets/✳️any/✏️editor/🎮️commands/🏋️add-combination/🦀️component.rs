//! 🏋️ 🏋️ Fem2d play app commands command — `add-combination`.

use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use crate::artifacts::fem2d::mutations::{add_load, change_load_case_self_weight, create_combination, create_load_case};
use crate::artifacts::fem2d::op::Fem2dMutation;
use crate::artifacts::fem2d::{FemCombination, FemDof, FemLoad, FemLoadCase};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

type Fem2dSnapshot = crate::artifacts::fem2d::Fem2dSnapshot;

/// 🔎️ Resolves the target load case for a load-adding command: the named `case_id` if given and
/// found, else the document's first load case, else `None` — a missing case is not resolved here,
/// the caller decides between `add-load` (existing case) and `create-load-case` (pre-seeded with the
/// new load, synthesized `"case-1"`/`"Load Case 1"`) once it knows which branch it's in.
fn resolve_load_case(doc: &Fem2dSnapshot, case_id: Option<&str>) -> Option<FemLoadCase> {
    case_id.and_then(|id| doc.load_cases.iter().find(|lc| lc.id == id).cloned()).or_else(|| doc.load_cases.first().cloned())
}

/// 🌉️ Shared resolve-or-create gesture behind `add-nodal-load`/`add-member-udl`/`add-area-load`:
/// attaches `load` to the named/first load case via `add-load` if one exists, else synthesizes a
/// fresh `"case-1"`/`"Load Case 1"` case pre-seeded with `load` via `create-load-case`.
fn add_load_mutation(doc: &Fem2dSnapshot, case_id: Option<&str>, load: FemLoad) -> Fem2dMutation {
    match resolve_load_case(doc, case_id) {
        Some(existing) => Fem2dMutation::AddLoad(add_load::mutation::AddLoad { case_id: existing.id, load: Box::new(load) }),
        None => Fem2dMutation::CreateLoadCase(create_load_case::mutation::CreateLoadCase { load_case: FemLoadCase { id: "case-1".into(), name: "Load Case 1".into(), loads: vec![load], self_weight: false } }),
    }
}

/// 🌉️ The load id a new load on the (possibly not-yet-existing) target case should get — reads the
/// existing case's loads for `next_id` continuity, or starts fresh for a synthesized case.
fn next_load_id(doc: &Fem2dSnapshot, case_id: Option<&str>) -> String {
    let loads = resolve_load_case(doc, case_id).map(|lc| lc.loads).unwrap_or_default();
    crate::app_surface::next_id(loads.iter().map(|l| crate::artifacts::fem2d::load_id(l).to_string()), "l")
}

//#region 🔖️AddNodalLoad
//#endregion 🔖️AddNodalLoad

//#region 🔖️AddMemberUdl
//#endregion 🔖️AddMemberUdl

//#region 🔖️AddAreaLoad
//#endregion 🔖️AddAreaLoad

//#region 🔖️AddLoadCase
//#endregion 🔖️AddLoadCase

//#region 🔖️AddCombination
//#endregion 🔖️AddCombination

//#region 🔖️SetSelfWeight
//#endregion 🔖️SetSelfWeight

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-combination")]
pub struct AddCombination {
    pub name: String,
    pub terms: Vec<crate::artifacts::fem2d::FemCombinationTerm>,
}

pub fn handle(payload: &AddCombination, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.combinations.iter().map(|c| c.id.clone()), "c");
    Ok(Emit::mutations(vec![Fem2dMutation::CreateCombination(create_combination::mutation::CreateCombination { combination: FemCombination { id, name: payload.name.clone(), terms: payload.terms.clone() } })]))
}
