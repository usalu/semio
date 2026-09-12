//! 🧬️ Forms snapshot schema — artifact-lane fields only.

use crate::{forms_snapshot_with_state, FormsResultsChild, FormsStructureChild, FORMS_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted forms document snapshot (persistent fields of the artifact). Ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`forms→C:value,table`): the inline
/// `steps: Vec<FormStep>` field is replaced by two fixed composed CHILD slots — this plugin no
/// longer defines its own bespoke document tree, it composes stdio's `value`/`table` subsets
/// instead. See `crate::🔖️Composition` (`🗿️artifacts/📋️forms/🦀️.rs`)
/// for the converters/working-scene this slot pair is built and read through. `#[child(...)]`
/// drives `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
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
    #[child(kind = "s.stdio.semio")]
    pub structure: FormsStructureChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub results: FormsResultsChild,
}

impl dsl::FromValue for FormsSnapshot {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let mut schema = None;
        let mut id = None;
        let mut version = None;
        let mut title = None;
        let mut structure = None;
        let mut results = None;
        for (key, value) in dsl::DslValue::into_object(value)? {
            match key.as_str() {
                "schema" if schema.is_none() => schema = Some(dsl::FromValue::from_value(value)?),
                "id" if id.is_none() => id = Some(dsl::FromValue::from_value(value)?),
                "version" if version.is_none() => version = Some(dsl::FromValue::from_value(value)?),
                "title" if title.is_none() => title = Some(dsl::FromValue::from_value(value)?),
                "structure" if structure.is_none() => structure = Some(dsl::FromValue::from_value(value)?),
                "results" if results.is_none() => results = Some(dsl::FromValue::from_value(value)?),
                _ => return Err(dsl::ValueError::new(format!("unknown or duplicate Forms field {key}"))),
            }
        }
        let result = Self { schema: schema.ok_or_else(|| dsl::ValueError::new("missing Forms schema"))?, id: id.ok_or_else(|| dsl::ValueError::new("missing Forms id"))?, version: version.ok_or_else(|| dsl::ValueError::new("missing Forms version"))?, title: title.unwrap_or(None), structure: structure.ok_or_else(|| dsl::ValueError::new("missing Forms structure"))?, results: results.ok_or_else(|| dsl::ValueError::new("missing Forms results"))? };
        result.validate().map_err(dsl::ValueError::new)?;
        Ok(result)
    }
}

impl FormsSnapshot {
    /// 🪆️ Enforces the document marker and exact owned-child coordinates.
    pub fn validate(&self) -> Result<(), String> {
        use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::child::validate_semio_child_identity;
        if self.schema != "forms.form" { return Err("invalid Forms document marker".into()); }
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
