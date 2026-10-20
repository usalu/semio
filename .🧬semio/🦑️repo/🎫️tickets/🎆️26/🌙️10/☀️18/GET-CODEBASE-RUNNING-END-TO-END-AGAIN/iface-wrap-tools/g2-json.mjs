import { edit, load, set } from "./lib.mjs";
import { HOST, MCP, WS, src } from "./paths.mjs";

const wire = `${MCP}/🧬️schema/📡️artifact-inference-wire/🦀️.rs`;
const port = `${MCP}/🔀️dispatch/🔌️channel-port/🦀️.rs`;
const frames = `${WS}/🧩️artifact-app-frames/🦀️.rs`;
const route = `${WS}/📨️plugin-artifact-channel/💡️inference-route-and-actions/🦀️.rs`;
const cargo = `${MCP}/⚙️plugin-runtime/📦️packages/🦀️rust/Cargo.toml`;
const binding = `${WS}/🧪️tests/🔬️quick/🔗️inference-artifact-binding/🦀️.rs`;

const DERIVE = "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]";
const DERIVE_VALUE = "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, semio_framework_value::ToValue, semio_framework_value::FromValue)]";
const SERDE = `#[serde(rename_all = "camelCase")]`;
const VALUE = `#[value(rename_all = "camelCase")]`;

export function build() {
  edit(wire, `${DERIVE}\n${SERDE}\npub struct ArtifactInferenceBudgetV1 {`, `${DERIVE_VALUE}\n${SERDE}\n${VALUE}\npub struct ArtifactInferenceBudgetV1 {`);
  edit(wire, `#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]\n${SERDE}\npub enum ArtifactInferenceCacheModeV1 {`, `#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, semio_framework_value::ToValue, semio_framework_value::FromValue)]\n${SERDE}\n${VALUE}\npub enum ArtifactInferenceCacheModeV1 {`);
  edit(wire, `${DERIVE}\n${SERDE}\npub struct ArtifactInferenceRequestV1 {`, `${DERIVE_VALUE}\n${SERDE}\n${VALUE}\npub struct ArtifactInferenceRequestV1 {`);
  edit(wire, `${DERIVE}\n${SERDE}\npub struct ArtifactInferenceResultV1 {`, `${DERIVE_VALUE}\n${SERDE}\n${VALUE}\npub struct ArtifactInferenceResultV1 {`);
  edit(wire, `    pub canonical_payload: Option<Vec<u8>>,\n}`, `    #[value(default)]\n    pub canonical_payload: Option<Vec<u8>>,\n}`);

  edit(port, `fn dsl_to_json_value(value: DslValue) -> Result<serde_json::Value, ValueError> {\n    Ok(serde_json::Value::from(value))\n}\n`, `fn dsl_to_json_value(value: DslValue) -> Result<serde_json::Value, ValueError> {\n    Ok(serde_json::Value::from(value))\n}\n` + src("json/port-fn.rs"));

  edit(frames, `use semio_framework_os_mcp_port::channel_port::{ActivationScope, AppCommand, AppFrame, ArtifactChannel, Fault, InferCommand, PreparedOps};`, `use semio_framework_os_mcp_port::channel_port::{ActivationScope, AppCommand, AppFrame, ArtifactChannel, Fault, InferCommand, PreparedOps, action_input_arguments};`);
  edit(frames, `    let mut body: serde_json::Value = if command.canonical_payload.is_empty() {\n        serde_json::json!({})\n    } else {\n        serde_json::from_slice(&command.canonical_payload).map_err(|error| Fault { code: "app.command.rejected".to_string(), message: format!("\`{}\` payload is not a JSON object: {error}", declared.inference_schema) })?\n    };`, `    let mut body = if command.canonical_payload.is_empty() {\n        semio_framework_pack_json::json!({})\n    } else {\n        semio_framework_pack_json::parse_bytes(&command.canonical_payload, semio_framework_pack_json::JsonMemberPolicy::Replace).map_err(|error| Fault { code: "app.command.rejected".to_string(), message: format!("\`{}\` payload is not a JSON object: {error}", declared.inference_schema) })?\n    };`);
  edit(frames, `        return serde_json::to_vec(&body).map_err(|error| Fault { code: "plugin.internal".to_string(), message: format!("encoding the inference payload: {error}") });`, `        return Ok(semio_framework_pack_json::to_string(&body).into_bytes());`);
  edit(frames, `serde_json::json!({ "pack": encode_base64(&document.pack), "spr": encode_base64(&document.spr) })`, `semio_framework_pack_json::json!({ "pack": encode_base64(&document.pack), "spr": encode_base64(&document.spr) })`);
  edit(frames, `    }\n    serde_json::to_vec(&body).map_err(|error| Fault { code: "plugin.internal".to_string(), message: format!("encoding the inference payload: {error}") })\n}`, `    }\n    Ok(semio_framework_pack_json::to_string(&body).into_bytes())\n}`);
  edit(frames, `let invocation = self.addressed_action_invocation(&capability_id, &input)?;`, `let invocation = self.addressed_action_invocation(&capability_id, || action_input_arguments(&input))?;`);

  edit(route, `    /// with the plugin's own declared roster — byte-for-byte the \`serde_json\` roster\n    /// \`🏃️run/🦀️.rs\` builds (\`contributions.inference_services\` chained with every\n    /// \`artifact_contributions[].inferences\`).`, `    /// with the plugin's own declared roster — the same rows \`🏃️run/🦀️.rs\` builds\n    /// (\`contributions.inference_services\` chained with every\n    /// \`artifact_contributions[].inferences\`), written by the owned JSON codec.`);
  edit(route, `            let roster: Vec<serde_json::Value> = self\n                .descriptor\n                .contributions\n                .inference_services\n                .iter()\n                .chain(self.descriptor.contributions.artifact_contributions.iter().flat_map(|contribution| contribution.inferences.iter()))\n                .map(serde_json::to_value)\n                .collect::<Result<Vec<_>, _>>()\n                .map_err(|error| Self::not_wired("inference roster", error))?;`, `            let roster: Vec<_> = self\n                .descriptor\n                .contributions\n                .inference_services\n                .iter()\n                .chain(self.descriptor.contributions.artifact_contributions.iter().flat_map(|contribution| contribution.inferences.iter()))\n                .cloned()\n                .collect();`);
  edit(route, `            let roster_bytes = serde_json::to_vec(&roster).map_err(|error| Self::not_wired("inference roster", error))?;`, `            let roster_bytes = semio_framework_pack_json::to_json_string(&roster).into_bytes();`);
  edit(route, `        let request_bytes = serde_json::to_vec(&request).map_err(|error| Self::not_wired("encoding the inference request", error))?;`, `        let request_bytes = semio_framework_pack_json::to_json_string(&request).into_bytes();`);
  edit(route, `        serde_json::from_slice(&result_bytes).map_err(|error| Self::not_wired("decoding the inference result", error))\n    }`, `        let result_text = std::str::from_utf8(&result_bytes).map_err(|error| Self::not_wired("decoding the inference result", error))?;\n        semio_framework_pack_json::from_json_str(result_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| Self::not_wired("decoding the inference result", error))\n    }`);
  edit(route, `    pub(crate) fn addressed_action_invocation(&self, capability_id: &str, input: &serde_json::Value) -> Result<semio_framework::manifest::ActionInvocation, Fault> {`, `    pub(crate) fn addressed_action_invocation(&self, capability_id: &str, arguments: impl FnOnce() -> Result<std::collections::BTreeMap<String, semio_framework_value::DslValue>, String>) -> Result<semio_framework::manifest::ActionInvocation, Fault> {`);
  edit(route, `        let arguments = match input {\n            serde_json::Value::Object(map) => map.iter().map(|(key, value)| (key.clone(), semio_framework_value::DslValue::from(value))).collect(),\n            serde_json::Value::Null => std::collections::BTreeMap::new(),\n            other => return Err(Self::not_wired("PureCommand", format!("an action's input is a named argument object; this call carried {other}"))),\n        };`, `        let arguments = arguments().map_err(|detail| Self::not_wired("PureCommand", detail))?;`);

  const manifest = load(cargo);
  if (!manifest.includes("serde_json = { workspace = true }\n")) throw Error("serde_json line missing");
  edit(cargo, `serde_json = { workspace = true }\n`, ``);
  edit(cargo, `[dev-dependencies]\n`, `[dev-dependencies]\nserde_json = { workspace = true }\n`);

  const imports = `${HOST}/📥️imports/🦀️.rs`;
  edit(imports, `async fn decode_json<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Option<T> {\n    if bytes.is_empty() {\n        return None;\n    }\n    serde_json::from_slice(bytes).ok()\n}`, `async fn decode_json<T: semio_framework_value::FromValue>(bytes: &[u8]) -> Option<T> {\n    if bytes.is_empty() {\n        return None;\n    }\n    semio_framework_pack_json::from_json_str(std::str::from_utf8(bytes).ok()?, semio_framework_pack_json::JsonMemberPolicy::Replace).ok()\n}`);
  const importsCargo = `${HOST}/📥️imports/📦️packages/🦀️rust/Cargo.toml`;
  edit(importsCargo, `serde = { version = "1.0.219", features = ["derive"] }\nserde_json = "1.0.140"\n`, ``);
  edit(importsCargo, `semio-framework-value = { path = "../../../../../../../../🔨️modules/🌱️value/📦️packages/🦀️rust" }\n`, `semio-framework-value = { path = "../../../../../../../../🔨️modules/🌱️value/📦️packages/🦀️rust" }\nsemio-framework-pack-json = { path = "../../../../../../../../🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust" }\n`);
  const dispatch = `${HOST}/🎭️guest-runtime/🗂️wasmtime-guest-runtime-and-codec-dispatch/🦀️.rs`;
  edit(dispatch, `if let Ok(update) = serde_json::from_value::<semio_framework::kernel::PresenceUpdate>(serde_json::Value::from(&value)) {`, `if let Ok(update) = <semio_framework::kernel::PresenceUpdate as semio_framework_value::FromValue>::from_value(value) {`);
  const runtimeCargo = `${HOST}/🎭️guest-runtime/📦️packages/🦀️rust/Cargo.toml`;
  edit(runtimeCargo, `serde_json = "1.0.140"\nsemio-framework-plugin-host-engine = { path = "../../../🔧️engine/📦️packages/🦀️rust" }`, `semio-framework-plugin-host-engine = { path = "../../../🔧️engine/📦️packages/🦀️rust" }`);
  edit(runtimeCargo, `[dev-dependencies]\n`, `[dev-dependencies]\nserde_json = "1.0.140"\n`);
  const operationCargo = `${HOST}/🚪️io/🎛️operation/📦️packages/🦀️rust/Cargo.toml`;
  edit(operationCargo, `serde_json = "1.0.140"\nsemio-framework-plugin-host-wit = { path = "../../../../🔗️wit-bindings/📦️packages/🦀️rust" }`, `semio-framework-plugin-host-wit = { path = "../../../../🔗️wit-bindings/📦️packages/🦀️rust" }`);
  edit(operationCargo, `[dev-dependencies]\n`, `[dev-dependencies]\nserde_json = "1.0.140"\n`);

  const text = load(binding);
  const tail = "\n//#endregion 🔗️InferenceArtifactBinding";
  if (!text.endsWith(tail + "\n") && !text.endsWith(tail)) throw Error("binding test tail moved");
  set(binding, text.replace(tail, src("json/tests-append.rs") + tail));
}
