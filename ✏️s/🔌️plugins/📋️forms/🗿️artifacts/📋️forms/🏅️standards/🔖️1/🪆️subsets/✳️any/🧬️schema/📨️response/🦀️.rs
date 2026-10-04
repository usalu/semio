//! 📨️ An immutable submission keeps the labels and kinds that accompanied its answers.

#[path = "📤️export/🦀️.rs"]
pub mod export;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct FormsAnswer {
    pub question_id: String,
    pub label: String,
    pub kind: String,
    pub value: semio_framework_value::DslValue,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct FormsResponse {
    pub id: String,
    pub submitted_at: u64,
    pub definition_version: String,
    pub answers: Vec<FormsAnswer>,
}

impl FormsResponse {
    /// 🪪️ Submission and answer identities must remain unambiguous after later form edits.
    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() || self.definition_version.is_empty() { return Err("invalid response identity".into()); }
        if self.submitted_at > 9_007_199_254_740_991 { return Err("invalid response timestamp".into()); }
        let mut ids = std::collections::HashSet::new();
        if self.answers.iter().any(|answer| answer.question_id.is_empty() || !ids.insert(&answer.question_id)) { return Err("invalid or duplicate response answer".into()); }
        Ok(())
    }
}

/// ✅️ Validates all visible answers before atomically admitting an immutable response.
pub fn prepare_response(
    definition: &super::definition::FormsDefinition,
    values: &crate::playbook::PlaybookValues,
    id: String,
    submitted_at: u64,
    definition_version: String,
) -> Result<FormsResponse, Vec<super::validation::FormsAnswerError>> {
    let errors: Vec<_> = definition.steps.iter().flat_map(|step| super::step_errors(step, values)).collect();
    if !errors.is_empty() { return Err(errors); }
    let answers = definition.steps.iter().flat_map(|step| super::visible_questions(step, values)).filter(|question| !matches!(question.kind.as_str(), "note" | "image")).map(|question| FormsAnswer {
        question_id: question.id.clone(),
        label: question.label.clone(),
        kind: question.kind.clone(),
        value: values.get(&question.id).cloned().unwrap_or(semio_framework_value::DslValue::Null),
    }).collect();
    Ok(FormsResponse { id, submitted_at, definition_version, answers })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️events/🦀️.rs"]
mod tests;
