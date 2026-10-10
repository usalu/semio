//! 📖️ The two carriers of the energy snapshot the bridge writes. The snapshot of `s.energy.model@1` is the schema name, the `model`, and two derived child handles (`energy-value`, `energy-table`: the energy artifact
//! derives their content from the model again on every load, nothing of the model lives in them) and two empty link slots. It is written as the artifact's own DSL text (`semio energy.model.dsl v1`, the `.energy`
//! file its editor opens: a record print of the same members, the model as a value) and as the JSON document the artifact's `s.stdio.json` export writes (members in camel case).
//! 📎 ../../../../../../../../../🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json

use super::target::EnergyModel;
use semio_framework_artifact_reference::{ArtifactDialect, ArtifactRef};
use semio_framework_value::{DslValue, ToValue};

/// 🏷️ The schema name of an energy snapshot.
pub const SCHEMA: &str = "energy.model";

fn handle(subset: &str) -> store::ArtifactChild<()> {
    let target = ArtifactRef { artifact_id: format!("energy-{subset}"), dialect: ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() } };
    store::ArtifactChild::new(target.artifact_id.clone(), target)
}

/// 📖️ The snapshot as the energy artifact's JSON export writes it.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
#[value(rename_all = "camelCase")]
pub struct EnergySnapshot {
    pub schema: String,
    pub model: EnergyModel,
    pub structure: store::ArtifactChild<()>,
    pub zones: store::ArtifactChild<()>,
    pub referenced_model: Option<DslValue>,
    pub weather_link: Option<DslValue>,
}

impl EnergySnapshot {
    /// 📖️ The snapshot of `model`.
    pub fn of(model: EnergyModel) -> Self {
        Self { schema: SCHEMA.to_string(), model, structure: handle("value"), zones: handle("table"), referenced_model: None, weather_link: None }
    }
}

#[derive(semio_framework_dsl_record_derive::DslRecord)]
#[dsl(extension = "energy")]
pub(crate) struct EnergyRecord {
    pub(crate) schema: String,
    pub(crate) model: DslValue,
    pub(crate) structure: store::ArtifactChild<()>,
    pub(crate) zones: store::ArtifactChild<()>,
    pub(crate) referenced_model: Option<store::ArtifactLink>,
    pub(crate) weather_link: Option<store::ArtifactLink>,
}

/// 🖨️ The DSL text of the snapshot of `model`: the preamble `semio energy.model.dsl v1` and the record.
pub fn dsl_text(model: &EnergyModel) -> Result<String, String> {
    let snapshot = EnergySnapshot::of(model.clone());
    let record = EnergyRecord { schema: snapshot.schema, model: snapshot.model.to_value(), structure: snapshot.structure, zones: snapshot.zones, referenced_model: None, weather_link: None };
    let body = semio_framework_dsl_record::print(&record.__dsl_to_record(), &EnergyRecord::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
    let envelope = store::semio_format::SemioEnvelope::from_envelope_id(SCHEMA, store::semio_format::Component::Dsl, 1).map_err(|error| error.to_string())?;
    Ok(store::semio_format::wrap_text(&envelope, &body))
}

/// 🧾️ The JSON text of the snapshot of `model`, two-space indented with a final newline.
pub fn json_text(model: &EnergyModel) -> Result<String, String> {
    let compact = semio_framework_pack_json::to_json_string(&EnergySnapshot::of(model.clone()));
    let value = super::super::json::codec::read_value(&compact)?;
    Ok(super::super::json::codec::document_text(&value))
}

