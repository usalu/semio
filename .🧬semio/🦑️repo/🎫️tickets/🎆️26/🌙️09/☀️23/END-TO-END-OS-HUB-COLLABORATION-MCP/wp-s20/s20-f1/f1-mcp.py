"""🌉️ S20 F1 (faults overlay): the MCP gateway answers every fault an agent meets with its code, its parameters and its
declared en/de text (`details.fault`), rendered by code from the plugin's `AppDefinition.faults` or the framework fault
catalog (`semio_framework::fault_text`) — never prose alone. The gateway's own `Fault` gains `parameters` + `texts`
(`..Fault::default()` on every literal), the guest decode reads the parameters and renders the texts against the
channel's descriptor, and re-wrapped guest faults keep them. Idempotent. Usage: python3 f1-mcp.py <root>"""
import re
import sys
from pathlib import Path

ROOT = Path(sys.argv[1])
MCP = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp"
DISPATCH = MCP / "🔀️dispatch/🦀️.rs"
WORKSPACE = MCP / "🏠️workspace/🦀️.rs"
EDITS = [
    (DISPATCH, """#[derive(Clone, Debug, PartialEq)]
pub struct Fault {
    pub code: String,
    pub message: String,
}
""", """#[derive(Clone, Debug, Default, PartialEq)]
pub struct Fault {
    pub code: String,
    pub message: String,
    /// 🧩️ The values the declared text of `code` shows (`Fault.parameters` of the guest's fault).
    pub parameters: Vec<semio_framework::FaultParameter>,
    /// 🗣️ The declared text of `code` in English and German (native terminology) — the plugin's `AppDefinition.faults`
    /// or the framework fault catalog, parameters filled; `None` for a code this gateway raises itself.
    pub texts: Option<FaultTexts>,
}

/// 🗣️ One fault's declared text in both shipped languages.
#[derive(Clone, Debug, PartialEq)]
pub struct FaultTexts {
    pub en: String,
    pub de: String,
}

/// 🗣️ Every gateway error that answers a fault carries it for the agent as `details.fault` — `{code, parameters, texts}`
/// (`texts` = `{en, de}` or `null`) — so an agent branches and renders by code, never by the prose `message`.
fn with_fault_details(fault: &Fault, error: GatewayError) -> GatewayError {
    let record = serde_json::json!({
        "code": fault.code,
        "parameters": fault.parameters.iter().map(|parameter| serde_json::json!({ "name": parameter.name, "value": parameter.value })).collect::<Vec<_>>(),
        "texts": fault.texts.as_ref().map(|texts| serde_json::json!({ "en": texts.en, "de": texts.de })),
    });
    let mut details = match error.details.clone() {
        serde_json::Value::Object(details) => details,
        serde_json::Value::Null => serde_json::Map::new(),
        other => serde_json::Map::from_iter([("detail".to_string(), other)]),
    };
    details.insert("fault".to_string(), record);
    error.with_details(serde_json::Value::Object(details))
}
"""),
    (DISPATCH, """fn map_fault(fault: &Fault) -> GatewayError {
    match fault.code.as_str() {
""", """fn map_fault(fault: &Fault) -> GatewayError {
    with_fault_details(fault, match fault.code.as_str() {
"""),
    (DISPATCH, """        _ => GatewayError::new(GatewayErrorCode::Internal, fault.message.clone()),
    }
}
""", """        _ => GatewayError::new(GatewayErrorCode::Internal, fault.message.clone()),
    })
}
"""),
    (WORKSPACE, """fn decode_guest_fault(bytes: &[u8]) -> Fault {
    let decoded = store::pack_rt::decode_wire_value(bytes).ok();
    match decoded {
        Some(value) => {
            let code = value.get("code").and_then(store::DslValue::as_str).unwrap_or("mutation.rejected").to_string();
            let message = value.get("message").and_then(store::DslValue::as_str).map(str::to_string).unwrap_or_else(|| store::os_pack::json::to_json_string(&value));
            Fault { code, message }
        }
""", """fn decode_guest_fault(bytes: &[u8], apps: &[semio_framework::AppDefinition]) -> Fault {
    let decoded = store::pack_rt::decode_wire_value(bytes).ok();
    match decoded {
        Some(value) => {
            let code = value.get("code").and_then(store::DslValue::as_str).unwrap_or("mutation.rejected").to_string();
            let message = value.get("message").and_then(store::DslValue::as_str).map(str::to_string).unwrap_or_else(|| store::os_pack::json::to_json_string(&value));
            let parameters: Vec<semio_framework::FaultParameter> = match value.get("parameters") {
                Some(store::DslValue::Array(items)) => items.iter().filter_map(|item| Some(semio_framework::FaultParameter { name: item.get("name")?.as_str()?.to_string(), value: item.get("value")?.as_str()?.to_string() })).collect(),
                _ => Vec::new(),
            };
            let app = value.get("origin").and_then(store::DslValue::as_str) == Some("app");
            let declared: Vec<semio_framework::FaultDefinition> = apps.iter().flat_map(|definition| definition.faults.iter().cloned()).collect();
            let text = |locale| semio_framework::fault_text(&code, app, &parameters, &declared, semio_framework::framework_fault_catalog(), semio_framework::Terminology::Native, locale);
            let texts = text(semio_framework::Locale::En).zip(text(semio_framework::Locale::De)).map(|(en, de)| crate::FaultTexts { en, de });
            Fault { code, message, parameters, texts }
        }
"""),
    (WORKSPACE, "fn shell_lane_fault(instance: u32, effect: &semio_framework::kernel::Effect) -> Option<Fault> {", "fn shell_lane_fault(instance: u32, effect: &semio_framework::kernel::Effect, apps: &[semio_framework::AppDefinition]) -> Option<Fault> {"),
    (WORKSPACE, "        Ok(store::AppFrame::Error { fault, .. }) => Some(decode_guest_fault(&fault)),", "        Ok(store::AppFrame::Error { fault, .. }) => Some(decode_guest_fault(&fault, apps)),"),
    (WORKSPACE, "if let Some(fault) = shell_lane_fault(instance, effect) {", "if let Some(fault) = shell_lane_fault(instance, effect, &self.descriptor.manifest.apps) {"),
    (WORKSPACE, "    let decoded = decode_guest_fault(fault);\n", "    let decoded = decode_guest_fault(fault, &[]);\n"),
    (WORKSPACE, ".map_err(|fault| Fault { code: fault.code, message: format!(\"{}; the acknowledging turn published {}\", fault.message, named_shapes(&published)) })?;",
     ".map_err(|fault| Fault { message: format!(\"{}; the acknowledging turn published {}\", fault.message, named_shapes(&published)), ..fault })?;"),
    (WORKSPACE, "return Err(Fault { code: fault.code, message: format!(\"the guest refused a batch its hub document delivered: {}\", fault.message) });",
     "return Err(Fault { message: format!(\"the guest refused a batch its hub document delivered: {}\", fault.message), ..fault });"),
]
DECODE_SELF = re.compile(r"decode_guest_fault\(&(fault|detail|rejection)\)")
LITERAL = re.compile(r"(?<![\w:])Fault \{")


def default_literals(text: str) -> str:
    out: list[str] = []
    at = 0
    for match in LITERAL.finditer(text):
        start = match.end()
        head = text[start:start + 40].lstrip()
        if not re.match(r"code\s*[:,]", head):
            continue
        depth, index, in_string = 1, start, False
        while depth > 0 and index < len(text):
            char = text[index]
            if in_string:
                if char == "\\":
                    index += 1
                elif char == '"':
                    in_string = False
            elif char == '"':
                in_string = True
            elif char in "{([":
                depth += 1
            elif char in "})]":
                depth -= 1
            index += 1
        close = index - 1
        body = text[start:close]
        if ".." in re.sub(r'"(?:\\.|[^"\\])*"', '""', body) or re.search(r"\btexts\b", body):
            continue
        stripped = body.rstrip()
        trailing = body[len(stripped):]
        insert = ("" if stripped.endswith(",") else ",") + (" " if trailing == " " or trailing == "" else trailing if "\n" not in trailing else "\n" + trailing.split("\n")[-1] + "    ") + "..Fault::default()"
        if "\n" in trailing:
            insert = ("" if stripped.endswith(",") else ",") + "\n" + trailing.split("\n")[-1] + "    ..Fault::default()"
        out.append(text[at:start + len(stripped)] + insert)
        at = start + len(stripped)
    out.append(text[at:])
    return "".join(out)


def main() -> None:
    texts: dict[Path, str] = {}
    for path, before, after in EDITS:
        text = texts.setdefault(path, path.read_text())
        if after in text:
            continue
        if text.count(before) != 1:
            sys.exit(f"CONFLICT {path.name}: {before[:80]} ×{text.count(before)}")
        texts[path] = text.replace(before, after)
    texts[WORKSPACE] = DECODE_SELF.sub(lambda match: f"decode_guest_fault(&{match.group(1)}, &self.descriptor.manifest.apps)", texts[WORKSPACE])
    for path in MCP.rglob("*.rs"):
        text = texts.get(path, path.read_text())
        texts[path] = default_literals(text).replace("..Fault::default(),\n", "..Fault::default()\n")
    tests = MCP / "🏠️workspace/🧪️tests/🔬️quick/🦀️.rs"
    texts[tests] = texts[tests].replace("decode_guest_fault(&store::pack_rt::encode_wire_value(&wire))", "decode_guest_fault(&store::pack_rt::encode_wire_value(&wire), &[])")
    changed = 0
    for path, text in texts.items():
        if text != path.read_text():
            path.write_text(text)
            changed += 1
    print(f"mcp: {changed} files changed")


main()
