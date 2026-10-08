//! 🧬️ Forms diff schema — sparse positional edits over the artifact.

use crate::schema::mutations::change_block_field::mutation::BlockField;
use crate::schema::response::FormsResponse;
use crate::{forms_children_from_steps, FormQuestion, FormStep, FormsSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;

//#region 🔖️Diff
/// 🔺️ Sparse durable domain edits and their derived child projections: scalar slots, positional step and response row deltas
/// (steps carry nested positional question deltas). The derived `structure` and `results` child handles are never carried: `apply`
/// re-derives them from the steps and responses it leaves behind.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.forms.forms")]
pub struct FormsDiff {
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub title: Option<FormsOptionalText>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub steps: Option<FormsStepsDelta>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub responses: Option<FormsResponsesDelta>,
}

impl semio_framework_value::FromValue for FormsDiff {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let mut result = Self::default();
        let mut seen = 0u8;
        for (key, value) in semio_framework_value::DslValue::into_object(value)? {
            let bit = match key.as_str() {
                "schema" => 1,
                "id" => 2,
                "version" => 4,
                "title" => 8,
                "steps" => 16,
                "responses" => 32,
                _ => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown Forms diff field {key}"))),
            };
            if seen & bit != 0 {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("duplicate Forms diff field {key}")));
            }
            seen |= bit;
            match key.as_str() {
                "schema" => result.schema = Some(semio_framework_value::FromValue::from_value(value)?),
                "id" => result.id = Some(semio_framework_value::FromValue::from_value(value)?),
                "version" => result.version = Some(semio_framework_value::FromValue::from_value(value)?),
                "title" => result.title = Some(semio_framework_value::FromValue::from_value(value)?),
                "steps" => result.steps = Some(semio_framework_value::FromValue::from_value(value)?),
                "responses" => result.responses = Some(semio_framework_value::FromValue::from_value(value)?),
                _ => unreachable!(),
            }
        }
        result.validate().map_err(|message| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message))?;
        Ok(result)
    }
}

impl FormsDiff {
    /// 🪆️ Enforces the document marker.
    pub fn validate(&self) -> Result<(), String> {
        if self.schema.as_deref().is_some_and(|value| value != "forms.form") { return Err("invalid Forms document marker".into()); }
        Ok(())
    }
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🧱️ Carries an optional text as a present slot, so clearing it stays distinct from leaving it untouched on every wire.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct FormsOptionalText {
    pub value: Option<String>,
}

/// 🩹 Field patch of one question: the kind when it changes plus the typed field settings, at most one per field.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct FormsQuestionPatch {
    pub kind: Option<String>,
    pub changes: Vec<BlockField>,
}

/// 🩹 Field patch of one step: title, description and the nested positional question delta.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct FormsStepPatch {
    pub title: Option<String>,
    pub description: Option<FormsOptionalText>,
    pub blocks: Option<FormsQuestionsDelta>,
}


protocol::list_delta! {
    /// 🧩 Positional row delta of the questions of one step.
    pub FormsQuestionsDelta { removal: FormsQuestionRemoval, insertion: FormsQuestionInsertion, relocation: FormsQuestionRelocation, modification: FormsQuestionsModification, row: FormQuestion, patch: FormsQuestionPatch, key: id, values_only }
}

protocol::list_delta! {
    /// 🧩 Positional row delta of the steps.
    pub FormsStepsDelta { removal: FormsStepRemoval, insertion: FormsStepInsertion, relocation: FormsStepRelocation, modification: FormsStepsModification, row: FormStep, patch: FormsStepPatch, key: id, values_only }
}

protocol::plain_list_delta! {
    /// 🧩 Positional row delta of the immutable responses: a response is inserted or removed, never patched.
    pub FormsResponsesDelta { removal: FormsResponseRemoval, insertion: FormsResponseInsertion, relocation: FormsResponseRelocation, row: FormsResponse, key: id }
}

//#endregion 🔖️DeltaHelpers


impl protocol::list_delta::RowPatch<FormQuestion> for FormsQuestionPatch {
    fn commit_into(&self, row: &mut FormQuestion, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(kind) = &self.kind {
            row.kind = kind.clone();
        }
        for change in &self.changes {
            change.set_on(row);
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        for change in later.changes {
            match self.changes.iter_mut().find(|existing| std::mem::discriminant(*existing) == std::mem::discriminant(&change)) {
                Some(slot) => *slot = change,
                None => self.changes.push(change),
            }
        }
        self.kind = later.kind.or(self.kind.take());
    }
    fn inverse(&self, row: &FormQuestion) -> Self {
        Self { kind: self.kind.as_ref().map(|_| row.kind.clone()), changes: self.changes.iter().map(|change| change.read(row)).collect() }
    }
    fn is_empty(&self) -> bool {
        self.kind.is_none() && self.changes.is_empty()
    }
}

impl protocol::list_delta::RowPatch<FormStep> for FormsStepPatch {
    fn commit_into(&self, row: &mut FormStep, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(title) = &self.title {
            row.title = title.clone();
        }
        if let Some(description) = &self.description {
            row.description = description.value.clone();
        }
        if let Some(delta) = &self.blocks {
            row.blocks = delta.commit_onto(&row.blocks, capability).map_err(|error| error.under(["blocks"]))?;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.title = later.title.or(self.title.take());
        self.description = later.description.or(self.description.take());
        self.blocks = match (self.blocks.take(), later.blocks) {
            (Some(mut first), Some(later)) => {
                first.absorb(later);
                Some(first)
            }
            (first, later) => later.or(first),
        };
    }
    fn inverse(&self, row: &FormStep) -> Self {
        Self {
                        title: self.title.as_ref().map(|_| row.title.clone()),
            description: self.description.as_ref().map(|_| FormsOptionalText { value: row.description.clone() }),
            blocks: self.blocks.as_ref().map(|delta| delta.inverse(&row.blocks)),
        }
    }
    fn is_empty(&self) -> bool {
        self.title.is_none() && self.description.is_none() && self.blocks.as_ref().is_none_or(FormsQuestionsDelta::is_empty)
    }
}



//#region 🔖️Apply
impl MutationDiff<FormsSnapshot> for FormsDiff {
    fn apply(&self, snapshot: &FormsSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<FormsSnapshot> {
        self.validate().map_err(|message| protocol::MutationApplyError { code: "mutation.apply.child-identity".into(), message, target: Vec::new() })?;
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(id) = &self.id {
            next.id = id.clone();
        }
        if let Some(version) = &self.version {
            next.version = version.clone();
        }
        if let Some(title) = &self.title {
            next.title = title.value.clone();
        }
        if let Some(delta) = &self.steps {
            next.definition.steps = delta.commit_onto(&next.definition.steps, capability).map_err(|error| error.under(["steps"]))?;
        }
        if let Some(delta) = &self.responses {
            next.responses = delta.commit_onto(&next.responses, capability).map_err(|error| error.under(["responses"]))?;
        }
        if self.steps.is_some() {
            (next.structure, _) = forms_children_from_steps(&next.definition.steps);
        }
        if self.responses.is_some() {
            next.results = crate::forms_results_child(&next.responses);
        }
        next.validate().map_err(|message| protocol::MutationApplyError { code: "mutation.apply.invariant".into(), message, target: Vec::new() })?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(id);
        take!(version);
        take!(title);
        self.steps = match (self.steps.take(), other.steps) {
            (Some(mut first), Some(later)) => {
                first.absorb(later);
                Some(first)
            }
            (first, later) => later.or(first),
        };
        self.responses = match (self.responses.take(), other.responses) {
            (Some(mut first), Some(later)) => {
                first.absorb(later);
                Some(first)
            }
            (first, later) => later.or(first),
        };
    }
}

impl protocol::DiffAlgebra<FormsSnapshot> for FormsDiff {
    fn inverse(&self, base: &FormsSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            id: self.id.as_ref().map(|_| base.id.clone()),
            version: self.version.as_ref().map(|_| base.version.clone()),
            title: self.title.as_ref().map(|_| FormsOptionalText { value: base.title.clone() }),
            steps: self.steps.as_ref().map(|delta| delta.inverse(&base.definition.steps)),
            responses: self.responses.as_ref().map(|delta| delta.inverse(&base.responses)),
        }
    }

    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.id.is_none() && self.version.is_none() && self.title.is_none() && self.steps.as_ref().is_none_or(FormsStepsDelta::is_empty) && self.responses.as_ref().is_none_or(FormsResponsesDelta::is_empty)
    }
}
//#endregion 🔖️Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
