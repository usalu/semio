//! 📍️ Place a question into an existing page or create the first page atomically.
use crate::{FormMutation, FormQuestion, FormStep};
use crate::schema::definition::FormsDefinition;

/// 🎯️ Resolves the final insertion index after removing a moved question from its original page.
pub fn question_insert_index(definition: &FormsDefinition, step_id: &str, target_id: &str, position: &str, moving_id: Option<&str>) -> Result<usize, String> {
    if !matches!(position, "before" | "after" | "inside") { return Err("invalid-position".into()); }
    let step = definition.steps.iter().find(|step| step.id == step_id).ok_or("missing-step")?;
    let source = moving_id.and_then(|id| step.blocks.iter().position(|question| question.id == id));
    let index = if let Some(target_step) = target_id.strip_prefix("step:") {
        if target_step != step_id { return Err("missing-target".into()); }
        if position == "before" { 0 } else { step.blocks.len() }
    } else {
        let target = step.blocks.iter().position(|question| question.id == target_id).ok_or("missing-target")?;
        if moving_id == Some(target_id) { return Ok(target); }
        match position { "before" => target, "after" => target + 1, _ => step.blocks.len() }
    };
    Ok(index - usize::from(source.is_some_and(|source| source < index)))
}

/// 🌱️ Returns one reversible event and refuses stale page targets and ambiguous identities.
pub fn create_question_event(definition: &FormsDefinition, question: FormQuestion, step_id: Option<&str>, new_step_id: &str) -> Result<FormMutation, String> {
    use crate::mutations::{create_block, create_step};
    if definition.steps.iter().flat_map(|step| &step.blocks).any(|existing| existing.id == question.id) { return Err("duplicate-question".into()); }
    let candidate = FormStep { id: new_step_id.into(), title: "Inputs".into(), description: None, blocks: vec![question.clone()] };
    FormsDefinition { steps: vec![candidate.clone()] }.validate().map_err(|_| "invalid-question".to_string())?;
    let target = match step_id {
        Some(id) => Some(definition.steps.iter().find(|step| step.id == id).ok_or("missing-step")?),
        None => definition.steps.first(),
    };
    Ok(match target {
        Some(step) => FormMutation::CreateBlock(create_block::mutation::CreateBlock { step_id: step.id.clone(), block: question, index: None }),
        None => FormMutation::CreateStep(create_step::mutation::CreateStep { step: candidate, index: None }),
    })
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
