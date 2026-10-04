//! 🧬️ Forms snapshot schema — artifact-lane fields only.

use crate::{forms_snapshot_with_state, FormsResultsChild, FormsStructureChild, FORMS_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
#[path="🪶️sqlite/🦀️.rs"]
pub mod sqlite;
#[path="📦️pack/🦀️.rs"]pub(crate) mod native_pack;

#[cfg(test)]
#[path="🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;

//#region 🔖️Snapshot
/// 📸️ Durable form definition and immutable responses with derived value/table child projections.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(extension = "forms")]
#[artifact_schema(id = "s.forms.forms")]
pub struct FormsSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub version: String,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[state(artifact)]
    pub definition: crate::schema::definition::FormsDefinition,
    #[state(artifact)]
    pub responses: Vec<crate::schema::response::FormsResponse>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub structure: FormsStructureChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub results: FormsResultsChild,
}

impl semio_framework_value::FromValue for FormsSnapshot {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let mut schema = None;
        let mut id = None;
        let mut version = None;
        let mut title = None;
        let mut definition = None;
        let mut responses = None;
        let mut structure = None;
        let mut results = None;
        for (key, value) in semio_framework_value::DslValue::into_object(value)? {
            match key.as_str() {
                "schema" if schema.is_none() => schema = Some(semio_framework_value::FromValue::from_value(value)?),
                "id" if id.is_none() => id = Some(semio_framework_value::FromValue::from_value(value)?),
                "version" if version.is_none() => version = Some(semio_framework_value::FromValue::from_value(value)?),
                "title" if title.is_none() => title = Some(semio_framework_value::FromValue::from_value(value)?),
                "definition" if definition.is_none() => definition = Some(semio_framework_value::FromValue::from_value(value)?),
                "responses" if responses.is_none() => responses = Some(semio_framework_value::FromValue::from_value(value)?),
                "structure" if structure.is_none() => structure = Some(semio_framework_value::FromValue::from_value(value)?),
                "results" if results.is_none() => results = Some(semio_framework_value::FromValue::from_value(value)?),
                _ => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown or duplicate Forms field {key}"))),
            }
        }
        let result = Self { schema: schema.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Forms schema"))?, id: id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Forms id"))?, version: version.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Forms version"))?, title: title.unwrap_or(None), definition: definition.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Forms definition"))?, responses: responses.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Forms responses"))?, structure: structure.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Forms structure"))?, results: results.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Forms results"))? };
        result.validate().map_err(|message| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message))?;
        Ok(result)
    }
}

impl FormsSnapshot {
    /// 🪆️ Enforces the document marker and exact owned-child coordinates.
    pub fn validate(&self) -> Result<(), String> {
        use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::child::validate_semio_child_identity;
        if self.schema != "forms.form" { return Err("invalid Forms document marker".into()); }
        self.definition.validate()?;
        let mut responses = std::collections::HashSet::new();
        for response in &self.responses { response.validate()?; if !responses.insert(&response.id) { return Err("duplicate response id".into()); } }
        validate_semio_child_identity(&self.structure.child_id, &self.structure.target, "value")?;
        validate_semio_child_identity(&self.results.child_id, &self.results.target, "table")?;
        Ok(())
    }
}


impl Default for FormsSnapshot {
    fn default() -> Self {
        forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms".into(), "1".into(), None, &[])
    }
}
//#endregion 🔖️Snapshot

// 🧬️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (design.md §1 CORRECTION): the native
// `store::ArtifactDsl`/`store::ArtifactPack` codec impls (and their hex/LEB128 primitives) moved to
// `🚪️io/📸️snapshot/{📝️text,💾️binary}` — this facet root keeps only the struct + pure defaults, no
// codecs (design.md rule: `🧬️schema` is types + pure transforms only). Their round-trip tests moved
// with them.

#[cfg(test)]
#[path = "../🧪️tests/💾️persistence/🦀️.rs"]
mod persistence_tests;
