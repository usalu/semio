//! 🧾️ `s.bim.model@1/*` → `s.stdio.json@rfc8259/*`: the diagnostics of a [`ModelSnapshot`] as one RFC 8259 document. The document has the counts by severity, one object per finding (severity, code, category, storey, element
//! ids, missing ids, the numbers of its message and the message in English and German), the index of the findings by element (worst severity, count, codes) and the counts by storey. Numbers are the shortest text that reads back
//! as the same `f64`, so a consumer measures a clash exactly as the model does; the document is the same in every language of the editor. The values are typed `s.stdio.json` values and the text is written by that artifact, not
//! by a writer of this leaf; [`adjudicated_json`] reads a document back into the table the third-party oracle adjudicates.
//! 🔖 `IoFidelity::Lossy`: the document keeps the findings, not the model they were found in.
//! 📎 https://www.rfc-editor.org/rfc/rfc8259

use crate::standards::v1::subsets::any::schema::inferences::diagnostics::{Diagnostic, DiagnosticCode, DiagnosticIndex, Severity, SeverityCounts, ADJUDICATED};
use crate::standards::v1::subsets::any::schema::inferences::ModelInference;
use crate::ModelSnapshot;
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use semio_s_artifact_stdio_json::schema::snapshot::JsonValue;
use std::collections::BTreeMap;

#[path = "🧱️codec/🦀️.rs"]
pub mod codec;

use codec::{array, count, document_text, items_of, member, number, number_of, object, read_value, string, text_of};

/// 🪪️ The RFC 8259 dialect this leaf writes.
pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🚦️ The token of a severity: `error`, `warning` or `info`.
pub fn severity_token(severity: Severity) -> String {
    format!("{severity:?}").to_lowercase()
}

fn counts_value(counts: &SeverityCounts) -> JsonValue {
    object([("error", count(counts.error)), ("warning", count(counts.warning)), ("info", count(counts.info)), ("total", count(counts.total()))])
}

fn strings(values: &[String]) -> JsonValue {
    array(values.iter().map(|value| string(value.as_str())))
}

/// ⚠️ The object of one finding.
pub fn finding_value(found: &Diagnostic) -> JsonValue {
    let storey = found.storey.as_ref().map_or(JsonValue::Null, |storey| string(storey.as_str()));
    let message = object([("en", string(found.text("en").unwrap_or_default())), ("de", string(found.text("de").unwrap_or_default()))]);
    object([
        ("severity", string(severity_token(found.severity))),
        ("code", string(found.code.slug())),
        ("category", string(found.code.category())),
        ("storey", storey),
        ("elements", strings(&found.elements)),
        ("missing", strings(&found.missing)),
        ("values", object(found.values.iter().map(|(name, value)| (name.as_str(), number(*value))))),
        ("message", message),
    ])
}

fn index_value(index: &DiagnosticIndex) -> (JsonValue, JsonValue) {
    let elements = object(index.elements.iter().map(|(id, entry)| (id.as_str(), object([("severity", string(severity_token(entry.severity))), ("count", count(entry.count)), ("codes", array(entry.codes.iter().map(|code| string(code.slug()))))]))));
    let storeys = object(index.storeys.iter().map(|(id, counts)| (id.as_str(), counts_value(counts))));
    (elements, storeys)
}

/// 🧾️ The document value of the diagnostics of an inference.
pub fn diagnostics_value(inferred: &ModelInference) -> JsonValue {
    let (elements, storeys) = index_value(&inferred.diagnostic_index);
    object([("counts", counts_value(&inferred.diagnostic_index.total)), ("findings", array(inferred.diagnostics.iter().map(finding_value))), ("elements", elements), ("storeys", storeys)])
}

/// 🧾️ The JSON text of the diagnostics of an inference.
pub fn diagnostics_json(inferred: &ModelInference) -> String {
    document_text(&diagnostics_value(inferred))
}

/// ⚖️ The table `"<slug>|<element>+<element>" → { "measure": x }` of the adjudicated findings of a diagnostics document, in the canonical JSON of `diagnostics::table_json`: the table the oracle compares, read from the export.
pub fn adjudicated_json(text: &str) -> Result<String, String> {
    let root = read_value(text)?;
    let findings = member(&root, "findings").ok_or_else(|| "the document has no findings".to_string())?;
    let mut table: BTreeMap<String, BTreeMap<String, f64>> = BTreeMap::new();
    for found in items_of(findings) {
        let code = member(found, "code").and_then(text_of).ok_or_else(|| "a finding has no code".to_string())?;
        if !ADJUDICATED.iter().any(|adjudicated| adjudicated.slug() == code) {
            continue;
        }
        let elements = member(found, "elements").map(items_of).unwrap_or_default().iter().filter_map(text_of).collect::<Vec<_>>().join("+");
        let value = |name: &str| member(found, "values").and_then(|values| member(values, name)).and_then(number_of);
        let measure = if code == DiagnosticCode::StoreyLevelGap.slug() { value("to").zip(value("from")).map_or(0.0, |(to, from)| to - from) } else { value("overlap_area").unwrap_or(0.0) };
        table.insert(format!("{code}|{elements}"), BTreeMap::from([("measure".to_string(), measure)]));
    }
    Ok(semio_framework_pack_json::to_json_string(&table))
}

/// 🧾️ The JSON text of the diagnostics of `model`, inferred through the model graph.
pub fn export_diagnostics(model: &ModelSnapshot) -> Result<String, semio_framework_value::ValueError> {
    use protocol::Inference;
    ModelInference::infer(model).map(|inferred| diagnostics_json(&inferred))
}

//#region 🔖️Serializer
/// 🧾️ The JSON serializer of the BIM model: the document of its diagnostics.
pub struct ModelIntoJson;

impl Serializer<ModelSnapshot> for ModelIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &ModelSnapshot, _: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(export_diagnostics(from).map_err(IoError::from_value_error)?)))
    }
}
//#endregion 🔖️Serializer

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
