//! 🔺️ Diff constructor for `CreateRule`: one created rule entry. The id must be free across every collection, then the rule must be writable (see `rule_problem`).

use crate::standards::v1::subsets::any::schema::mutations::elements;
use crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule;
use crate::standards::v1::subsets::any::schema::{snapshot::{rule_problem, ModelSnapshot}, diff::{Entry, ModelDiff}};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateRule, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = rule_problem(base, &payload.id, &payload.rule) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, ["rule", problem.field]);
    }
    MutationOutcome::new(ModelDiff::rules(payload.id.clone(), Entry::Created(payload.rule.clone())))
}
