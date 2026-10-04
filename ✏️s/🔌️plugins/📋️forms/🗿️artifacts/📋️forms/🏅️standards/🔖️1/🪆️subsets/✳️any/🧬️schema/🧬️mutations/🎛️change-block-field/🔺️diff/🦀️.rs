//! 🔺️ `change-block-field` — sparse diff construction: locates the question in whichever step holds it, refuses a value that
//! breaks the question's own invariants, and clones only that step's `blocks` Vec.

use super::mutation::{BlockField, ChangeBlockField};
use crate::schema::diff::forms_diff_from_delta;
use crate::schema::diff::{FormsStepPatch, FormsStepPatchEntry, FormsStepsDelta};
use crate::{forms_steps, FormQuestion, FormsDiff, FormsSnapshot};

//#region 🔖️Diff
pub fn diff_change_block_field(payload: &ChangeBlockField, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
    let steps = forms_steps(base);
    let Some(step) = steps.iter().find(|step| step.blocks.iter().any(|block| block.id == payload.block_id)) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Question \"{}\" does not exist.", payload.block_id), [payload.block_id.clone()]);
    };
    let existing = step.blocks.iter().find(|block| block.id == payload.block_id).expect("the step holds the question");
    let next = payload.change.applied(existing);
    if &next == existing {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Question \"{}\" already holds that {}.", payload.block_id, payload.change.labels().0));
    }
    if let Some((code, reason)) = refusal(&next, &payload.change) {
        return protocol::MutationOutcome::fatal(code, reason, vec![payload.block_id.clone()]);
    }
    let blocks: Vec<_> = step.blocks.iter().map(|block| if block.id == payload.block_id { next.clone() } else { block.clone() }).collect();
    let patch = FormsStepPatch { blocks: Some(blocks), ..Default::default() };
    protocol::MutationOutcome::new(forms_diff_from_delta(&FormsStepsDelta { patched: vec![FormsStepPatchEntry { id: step.id.clone(), patch }], ..Default::default() }, base))
}

/// 🛡️ Why `next` (the question with `change` set) breaks an invariant of the field `change` sets: a non-finite or inverted
/// numeric range, a step that is not positive, a default the question kind cannot answer, parameters that are no object,
/// or an option value / vector key that is empty or repeated.
fn refusal(next: &FormQuestion, change: &BlockField) -> Option<(&'static str, String)> {
    let finite = |value: Option<f64>| value.is_none_or(f64::is_finite);
    match change {
        BlockField::Min(_) | BlockField::Max(_) if !finite(next.min) || !finite(next.max) => Some(("mutation.invariant", "a numeric bound is not a finite number".into())),
        BlockField::Min(_) | BlockField::Max(_) => next.min.zip(next.max).filter(|(min, max)| min > max).map(|(min, max)| ("mutation.invariant", format!("the minimum {min} exceeds the maximum {max}"))),
        BlockField::Step(Some(step)) if !step.is_finite() || *step <= 0.0 => Some(("mutation.invariant", format!("the step {step} is not greater than zero"))),
        BlockField::Default(Some(value)) if !default_fits(&next.kind, value) => Some(("mutation.invariant", format!("a {} question cannot default to that value", next.kind))),
        BlockField::Params(Some(value)) if !matches!(value, semio_framework_value::DslValue::Object(_)) => Some(("mutation.invariant", "parameters are one object".into())),
        BlockField::Options(Some(options)) => unique(options.iter().map(|option| option.value.as_str()), "option value"),
        BlockField::Fields(Some(fields)) => unique(fields.iter().map(|field| field.key.as_str()), "vector key"),
        _ => None,
    }
}

/// 🎯️ Whether a question of `kind` can default to `value` (kinds without a typed answer accept any value).
fn default_fits(kind: &str, value: &semio_framework_value::DslValue) -> bool {
    match kind {
        "number" | "slider" => value.as_f64().is_some_and(f64::is_finite),
        "boolean" => value.as_bool().is_some(),
        "text" | "longText" | "date" | "color" | "single" => value.as_str().is_some(),
        "multi" => value.as_array().is_some_and(|items| items.iter().all(|item| item.as_str().is_some())),
        _ => true,
    }
}

/// 🆔️ An empty or a repeated identity among `ids`.
fn unique<'a>(ids: impl Iterator<Item = &'a str>, what: &str) -> Option<(&'static str, String)> {
    let mut seen = std::collections::HashSet::new();
    for id in ids {
        if id.is_empty() {
            return Some(("mutation.invariant", format!("an empty {what}")));
        }
        if !seen.insert(id) {
            return Some(("mutation.duplicate-id", format!("the {what} \"{id}\" is repeated")));
        }
    }
    None
}
//#endregion 🔖️Diff
