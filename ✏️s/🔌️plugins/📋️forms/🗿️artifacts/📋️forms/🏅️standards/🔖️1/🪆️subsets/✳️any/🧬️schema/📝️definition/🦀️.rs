//! 📝️ The authoritative, durable form definition shared by authoring and answering.
use crate::FormStep;

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
pub struct FormsDefinition {
    pub steps: Vec<FormStep>,
}

impl FormsDefinition {
    /// 🪪️ Rejects ambiguous identities and invalid field constraints before persistence.
    pub fn validate(&self) -> Result<(), String> {
        let mut steps = std::collections::HashSet::new();
        let mut questions = std::collections::HashSet::new();
        for step in &self.steps {
            if step.id.is_empty() || !steps.insert(&step.id) { return Err("invalid or duplicate step id".into()); }
            for question in &step.blocks {
                if question.id.is_empty() || !questions.insert(&question.id) || question.kind.trim().is_empty() { return Err("invalid or duplicate question id or kind".into()); }
                if [question.min, question.max, question.step].into_iter().flatten().any(|value| !value.is_finite()) || question.min.zip(question.max).is_some_and(|(min, max)| min > max) || question.step.is_some_and(|step| step <= 0.0) { return Err("invalid question range".into()); }
                let mut options = std::collections::HashSet::new();
                if question.options.iter().flatten().any(|option| option.value.is_empty() || !options.insert(&option.value)) { return Err("invalid or duplicate option value".into()); }
                let mut fields = std::collections::HashSet::new();
                if question.fields.iter().flatten().any(|field| field.key.is_empty() || !fields.insert(&field.key) || field.value.is_some_and(|value| !value.is_finite())) { return Err("invalid or duplicate vector field".into()); }
                if question.params.as_ref().is_some_and(|value| !matches!(value, semio_framework_value::DslValue::Object(_))) { return Err("invalid extension parameters".into()); }
            }
        }
        Ok(())
    }
}
