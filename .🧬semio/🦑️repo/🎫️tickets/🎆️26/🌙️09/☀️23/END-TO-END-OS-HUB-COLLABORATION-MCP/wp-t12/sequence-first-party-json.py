#!/usr/bin/env python3
"""🧾️ Contract rows `testing/dependency` (sequence, session 12): production code of the sequence artifact imported
`serde_json`, the crate the subset registers as its third-party ORACLE (`serde-json-sequence-carrier-reader`), so the
differential test compared the implementation with itself — and AGENTS.md forbids runtime external libraries anyway.
Every production JSON read/write in the crate moves onto the first-party `pack` JSON tree (`dsl::os_pack::json`);
`serde_json` stays a dev-dependency for the oracle and tests.
Usage: sequence-first-party-json.py [--write]"""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence")
S = ROOT / "🏅️standards/🔖️1/🪆️subsets/✳️any"
write = "--write" in sys.argv
problems = []
edits = {}


def replace(path, old, new, count=1):
    text = edits.get(path) or path.read_text(encoding="utf-8")
    if text.count(old) != count:
        problems.append(f"{path.relative_to(ROOT)}: expected {count}× {old[:80]!r}, found {text.count(old)}")
        return
    edits[path] = text.replace(old, new)


editor = S / "✏️editor/🦀️.rs"
replace(editor, "use serde_json::{json, Value};\n", "use dsl::os_pack::json;\nuse dsl::os_pack::json::Value;\n")
replace(editor, "impl From<serde_json::Error> for SequenceCoreError {\n    fn from(error: serde_json::Error) -> Self {\n        Self::Json(error.to_string())\n    }\n}\n", "")
replace(editor, '                self.operations = serde_json::from_str(&payload.operations_json).map_err(|_| Fault::from("sequence-node-graph-json"))?;\n',
        '                let Ok(Value::Array(operations)) = json::parse(&payload.operations_json) else {\n                    return Err(Fault::from("sequence-node-graph-json"));\n                };\n                self.operations = operations;\n')
replace(editor, "        let Ok(value) = serde_json::from_str::<Value>(media_json) else {\n", "        let Ok(value) = json::parse(media_json) else {\n")
replace(editor, '        let params_value = if value.is_object() { value } else { json!({ "value": value }) };\n', '        let params_value = if value.as_object().is_some() { value } else { json!({ "value": value }) };\n', 2)
replace(editor, "        let value: Value = serde_json::from_str(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;\n",
        "        let value = json::parse(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;\n")
replace(editor, '                args: semio_framework_plugin::optional_json_to_dsl(Some(json!({ "id": id }))),\n', '                args: Some(json::to_dsl_value(&json!({ "id": id }))),\n')

node_graph = S / "✏️editor/🎮️commands/🕸️node-graph/🦀️.rs"
replace(node_graph, "use serde_json::Value;\n", "use dsl::os_pack::json::{self, Value};\n")
replace(node_graph, "        let sub_operations: Vec<Value> = serde_json::from_str(&payload.operations_json).unwrap_or_default();\n",
        "        let sub_operations: Vec<Value> = match json::parse(&payload.operations_json) {\n            Ok(Value::Array(operations)) => operations,\n            _ => Vec::new(),\n        };\n")

transient = S / "✏️editor/🎭️modes/✏️edit/🪟️windows/📜️script/🫧️transient/🦀️.rs"
replace(transient, "        let json: serde_json::Value = serde_json::from_str(body).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))?;\n        dsl::FromValue::from_value(json.into()).map_err(",
        "        let json = dsl::os_pack::json::parse(body).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))?;\n        dsl::FromValue::from_value(dsl::os_pack::json::to_dsl_value(&json)).map_err(")
replace(transient, "        let value: serde_json::Value = dsl::ToValue::to_value(self).into();\n        let body = serde_json::to_string_pretty(&value).expect(\"Sequence script transient JSON\");\n",
        "        let body = dsl::os_pack::json::to_string_pretty(&dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(self)));\n")
replace(transient, "        let value: serde_json::Value = dsl::ToValue::to_value(self).into();\n        let body = serde_json::to_vec(&value).map_err(|error| store::PackError::Schema(error.to_string()))?;\n",
        "        let body = dsl::os_pack::json::to_string(&dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(self))).into_bytes();\n")
replace(transient, "        let json: serde_json::Value = serde_json::from_slice(&body).map_err(|error| store::PackError::Schema(error.to_string()))?;\n        dsl::FromValue::from_value(json.into()).map_err(",
        "        let json = dsl::os_pack::json::parse_bytes(&body).map_err(|error| store::PackError::Schema(error.to_string()))?;\n        dsl::FromValue::from_value(dsl::os_pack::json::to_dsl_value(&json)).map_err(")

component = S / "✏️editor/🌉️wasm/🧩️component.rs"
replace(component, "            SEQUENCE_OPERATION_SELECTED_NODES => serde_json::to_vec(&self.host.dag.selected_node_ids()).map_err(domain_error),\n",
        "            SEQUENCE_OPERATION_SELECTED_NODES => Ok(dsl::os_pack::to_json_string(&self.host.dag.selected_node_ids()).into_bytes()),\n")
replace(component, "            SEQUENCE_OPERATION_PRESELECT_NODES => serde_json::to_vec(&serde_json::json!({\n                \"ids\": self.host.dag.preselect_widget_ids(),\n                \"removedIds\": self.host.dag.preselect_removed_widget_ids(),\n            }))\n            .map_err(domain_error),\n",
        "            SEQUENCE_OPERATION_PRESELECT_NODES => Ok(dsl::os_pack::json!({\n                \"ids\": self.host.dag.preselect_widget_ids(),\n                \"removedIds\": self.host.dag.preselect_removed_widget_ids(),\n            })\n            .to_string()\n            .into_bytes()),\n")

csv = S / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"
replace(csv, "neural_engine::Atom::String(serde_json::to_string(&values).unwrap_or_default())", "neural_engine::Atom::String(dsl::os_pack::to_json_string(&values))")

cargo = ROOT / "📦️packages/🦀️rust/Cargo.toml"
replace(cargo, "serde.workspace = true\nserde_json.workspace = true\n\n[dev-dependencies]\n", "serde.workspace = true\n\n[dev-dependencies]\nserde_json.workspace = true\n")

for path, text in edits.items():
    print(("write " if write and not problems else "plan  ") + str(path.relative_to(ROOT)))
print(f"files={len(edits)} problems={len(problems)}")
for problem in problems:
    print("PROBLEM", problem)
if write and not problems:
    for path, text in edits.items():
        path.write_text(text, encoding="utf-8")
