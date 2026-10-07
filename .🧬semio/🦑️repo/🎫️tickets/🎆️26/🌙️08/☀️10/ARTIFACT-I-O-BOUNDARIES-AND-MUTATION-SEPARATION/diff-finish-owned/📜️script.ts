import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
const ticket = dirname(dirname(import.meta.path));
const report = join(ticket, "semantic-diff-final-implementation.md");
const files = [...readFileSync(report, "utf8").matchAll(/^- (.+\/🔺️diff\/🦀️\.rs)$/gm)].map(row => row[1]!);
for (const file of files) {
  const source = readFileSync(file, "utf8");
  const match = source.match(/semio_framework_os_kernel::diff_(text|binary)!\(([^)]+)\);/);
  if (!match) throw new Error(`Missing physical macro ${file}`);
  const [, facet, canonical] = match;
  const implementation = facet === "text" ? `impl semio_framework_os_kernel::DiffText for ${canonical} {
    fn print_diff(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
    fn parse_diff(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}` : `impl semio_framework_os_kernel::DiffBinary for ${canonical} {
    fn encode_diff(&self) -> Result<Vec<u8>, semio_framework_os_kernel::ProtocolError> {
        Ok(semio_framework_os_kernel::os_store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(self)))
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, semio_framework_os_kernel::ProtocolError> {
        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })?;
        semio_framework_value::FromValue::from_value(value).map_err(|error: semio_framework_value::ValueError| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })
    }
}`;
  writeFileSync(file, source.replace(match[0], implementation));
}
writeFileSync(report, readFileSync(report, "utf8").replace("using the existing kernel macros", "using direct JSON and binary owned-value implementations. These canonical types do not expose generated DSL record methods, so the record macros are inapplicable"));
console.log(`[DEBUG] paired owned-value physical implementations=${files.length}`);
