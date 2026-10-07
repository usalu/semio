//! 📤️ Response interchange keeps labels, definition revisions, and typed answer values.

use crate::schema::response::{FormsAnswer, FormsResponse};
use semio_s_artifact_stdio_csv::{CsvField, CsvRecord, CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub const RESPONSE_COLUMNS: [&str; 7] = ["responseId", "submittedAt", "definitionVersion", "questionId", "label", "kind", "valueJson"];



/// 📊️ A normalized answer grid supports submissions made against different definitions.
pub fn response_rows(responses: &[FormsResponse]) -> Vec<Vec<String>> {
    std::iter::once(RESPONSE_COLUMNS.into_iter().map(str::to_owned).collect()).chain(responses.iter().flat_map(|response| response.answers.iter().map(move |answer| response_row(response, answer)))).collect()
}

fn csv_record(row: Vec<String>) -> String {
    let records = vec![CsvRecord { fields: row.into_iter().map(|value| CsvField { value, quoted: false }).collect() }];
    semio_s_artifact_stdio_csv::standards::v_rfc4180::subsets::any::io::text::snapshot::encode_csv_with(&CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: false, records }, "\r\n")
}

/// 🧵️ A cancellable exporter advances by one envelope or answer record, retaining only output bytes.
pub struct ResponseExport {
    csv: bool,
    started: bool,
    terminal: bool,
    response: usize,
    answer: Option<usize>,
    output: String,
}

impl ResponseExport {
    pub fn new(format: &str) -> Result<Self, String> {
        if !matches!(format, "json" | "csv") { return Err("forms-export-format-invalid".into()); }
        Ok(Self { csv: format == "csv", started: false, terminal: false, response: 0, answer: None, output: String::new() })
    }

    pub fn work_items(responses: &[FormsResponse]) -> usize {
        responses.iter().fold(2usize, |count, response| count.saturating_add(response.answers.len()).saturating_add(2))
    }

    pub fn advance(&mut self, responses: &[FormsResponse]) -> Option<String> {
        if self.terminal { return None; }
        if !self.started {
            self.output.push_str(&if self.csv { csv_record(RESPONSE_COLUMNS.into_iter().map(str::to_owned).collect()) } else { "[".into() });
            self.started = true;
            return None;
        }
        let Some(response) = responses.get(self.response) else {
            if !self.csv { self.output.push(']'); }
            self.terminal = true;
            return Some(std::mem::take(&mut self.output));
        };
        let Some(index) = self.answer else {
            if !self.csv {
                if self.response > 0 { self.output.push(','); }
                self.output.push_str(&format!("{{\"id\":{},\"submittedAt\":{},\"definitionVersion\":{},\"answers\":[", semio_framework_pack_json::to_json_string(&response.id), response.submitted_at, semio_framework_pack_json::to_json_string(&response.definition_version)));
            }
            self.answer = Some(0);
            return None;
        };
        if let Some(answer) = response.answers.get(index) {
            if self.csv { self.output.push_str(&csv_record(response_row(response, answer))); }
            else {
                if index > 0 { self.output.push(','); }
                self.output.push_str(&semio_framework_pack_json::to_json_string(answer));
            }
            self.answer = Some(index + 1);
            return None;
        }
        if !self.csv { self.output.push_str("]}"); }
        self.response += 1;
        self.answer = None;
        None
    }

    pub fn cancel(&mut self) {
        self.output = String::new();
        self.terminal = true;
    }

    pub fn is_empty(&self) -> bool { self.output.is_empty() }
}

fn collect(responses: &[FormsResponse], format: &str) -> String {
    let mut export = ResponseExport::new(format).expect("built-in response export format");
    loop { if let Some(output) = export.advance(responses) { return output; } }
}

/// 📋️ RFC 4180 records are encoded through the first-party CSV artifact.
pub fn export_responses_csv(responses: &[FormsResponse]) -> String { collect(responses, "csv") }

/// 📦️ JSON retains response boundaries, including submissions with no answer fields.
pub fn export_responses_json(responses: &[FormsResponse]) -> String { collect(responses, "json") }

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;


/// 📋️ One response answer as logical cells before native CSV encoding.
pub fn response_row(response: &FormsResponse, answer: &FormsAnswer) -> Vec<String> {
    vec![response.id.clone(), response.submitted_at.to_string(), response.definition_version.clone(), answer.question_id.clone(), answer.label.clone(), answer.kind.clone(), semio_framework_pack_json::to_json_string(&answer.value)]
}
