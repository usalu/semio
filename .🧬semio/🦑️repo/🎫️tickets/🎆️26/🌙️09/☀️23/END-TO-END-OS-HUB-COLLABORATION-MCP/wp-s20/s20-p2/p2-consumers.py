"""🧬️ S20 pass 2 (P2-F1) on the p2 overlay — every framework reader and writer of a mutation report's former `message`:
the durable history record (`HistoryMessage` wire drops its message string), the store's history conversions, retirement,
validation and retained sizing, the retire-struct field list, sync's framework-internal reports (document-link status,
conflict events: code + target only), the planner's rejection reason (codes), the run summary (code + target), the MCP
relay (terminal status by code), and both shells' conflict rows (the catalog text of the code in the shell's locale,
never prose). Idempotent (an edit whose result is present is skipped; its anchor must match exactly once otherwise).
Usage: python3 p2-consumers.py [--dry-run]"""
from __future__ import annotations

import re
import sys
from pathlib import Path

OVERLAY = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults-p2/🧰️framework/🛍️products/💻️os/🔨️modules")
HISTORY = "📡️spr/📜️history/🦀️.rs"
COMMAND = "📡️spr/🎮️command/🦀️.rs"
STORE = "🏪️store/🦀️.rs"
RETAINED = "🏪️store/🎚️config/📥️retained/🦀️.rs"
OPEN = "🏪️store/🧩️composition/🚪️open/🦀️.rs"
SYNC = "🏪️store/🔄️sync/🦀️.rs"
RUN = "🏃️run/🦀️.rs"
MCP = "🌉️mcp/🏠️workspace/🦀️.rs"
WGPU = "📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
SHELL_HOST = "📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"

EDITS: list[tuple[str, str, str]] = [
    (HISTORY, "/// `📋️contract-freeze.md` §C2), `message`/`target` are plain strings (English prose / element\n/// address, never interned — they vary per occurrence).",
     "/// `📋️contract-freeze.md` §C2), `target` is plain strings (the element address, never interned — it varies per\n/// occurrence). A report carries no prose: hosts show the catalog's text for `code`."),
    (HISTORY, "    pub code: String,\n    pub message: String,\n    pub target: Vec<String>,\n    pub op_index: Option<u32>,\n}\n", "    pub code: String,\n    pub target: Vec<String>,\n    pub op_index: Option<u32>,\n}\n"),
    (HISTORY, "/// 🎯️ `level u8 | code(idfield, dict-interned) | message(strfield) | target_count varint +", "/// 🎯️ `level u8 | code(idfield, dict-interned) | target_count varint +"),
    (HISTORY, "    write_id_field(out, &message.code, dict, &|_: &str| None).await?;\n    write_str_field(out, &message.message).await;\n", "    write_id_field(out, &message.code, dict, &|_: &str| None).await?;\n"),
    (HISTORY, "    let message = read_str_field(input).await?;\n    let target_count = input.read_varint_u64()?;", "    let target_count = input.read_varint_u64()?;"),
    (HISTORY, "    Ok(HistoryMessage { level, code, message, target, op_index })", "    Ok(HistoryMessage { level, code, target, op_index })"),
    (STORE, "            state.strings[0] = Some(message.code.0);\n            state.strings[1] = Some(message.message);\n", "            state.strings[0] = Some(message.code.0);\n"),
    (STORE, "if let Some(bytes) = Self::take_string(&mut message.message).or_else(|| Self::take_string(&mut message.code.0)) {", "if let Some(bytes) = Self::take_string(&mut message.code.0) {"),
    (STORE, "code: message.code.0.clone(), message: message.message.clone(), target: message.target.clone(), op_index: message.op_index }", "code: message.code.0.clone(), target: message.target.clone(), op_index: message.op_index }"),
    (STORE, "code: crate::os_dsl::FaultCode(message.code), message: message.message, target: message.target, op_index: message.op_index })", "code: crate::os_dsl::FaultCode(message.code), target: message.target, op_index: message.op_index })"),
    (STORE, "if message.level != expected_level || message.message.trim().is_empty() || message.target.iter()", "if message.level != expected_level || message.target.iter()"),
    (RETAINED, "message.code.len() + message.message.len() + message.target.iter()", "message.code.len() + message.target.iter()"),
    (OPEN, "crate::artifact_retire_struct!(crate::os_spr::MutationMessage { level, code, message, target, op_index });", "crate::artifact_retire_struct!(crate::os_spr::MutationMessage { level, code, target, op_index });"),
    (SYNC, "/// 📣️ The message a document actor emits once its link turns terminal: the status code is the fault code a\n/// shell localizes ([`DocumentLinkStatus::text`]), the message the English line for logs.",
     "/// 📣️ The report a document actor emits once its link turns terminal: the status code is the fault code a shell\n/// localizes ([`DocumentLinkStatus::text`]); the report carries no prose."),
    (SYNC, "code: status.fault_code(), message: status.text(false).unwrap_or_default().to_string(), target: vec![document_id.to_string()], op_index: None }",
     "code: status.fault_code(), target: vec![document_id.to_string()], op_index: None }"),
    (COMMAND, ".map(|message| message.message.clone()).collect::<Vec<_>>().join(\"; \");", ".map(|message| message.code.0.clone()).collect::<Vec<_>>().join(\"; \");"),
    (RUN, 'decoded.messages.iter().map(|message| if message.target.is_empty() { format!("{}: {}", message.code.0, message.message) } else { format!("{}: {} [{}]", message.code.0, message.message, message.target.join("/")) })',
     'decoded.messages.iter().map(|message| if message.target.is_empty() { message.code.0.clone() } else { format!("{} [{}]", message.code.0, message.target.join("/")) })'),
    (MCP, "store::sync::ArtifactEvent::Conflict(message) if document_link_terminal_code(&message.code.0) => relay.record_terminal(&message.code.0, &message.message),",
     "store::sync::ArtifactEvent::Conflict(message) if document_link_terminal_code(&message.code.0) => relay.record_terminal(&message.code.0),"),
    (MCP, 'store::sync::ArtifactEvent::Conflict(message) => relay.record(None, Some(format!("{}: {}", message.code.0, message.message))),',
     'store::sync::ArtifactEvent::Conflict(message) => relay.record(None, Some(format!("{} [{}]", message.code.0, message.target.join("/")))),'),
    (MCP, "    fn record_terminal(&self, code: &str, message: &str) {\n        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);\n        state.version += 1;\n        state.fault = Some(format!(\"{code}: {message}\"));",
     "    fn record_terminal(&self, code: &str) {\n        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);\n        state.version += 1;\n        state.fault = Some(code.to_string());"),
    (WGPU, "    /// 📨️ The WORST message's fault code and text (`conflict.messages[0]`, React's own pick).\n    pub code: String,\n    pub message: String,\n",
     "    /// 📨️ The WORST message's fault code (`conflict.messages[0]`, React's own pick) — its text is the catalog's, in the\n    /// shell's locale, rendered where the row is painted.\n    pub code: String,\n"),
    (WGPU, "                    message: worst.map(|message| message.message.clone()).unwrap_or_default(),\n", ""),
    (WGPU, 'label: Label::data(format!("{kind} \\u{2014} {} \\u{2014} {}", conflict.code, conflict.message)),',
     'label: Label::data(format!("{kind} \\u{2014} {} \\u{2014} {}", conflict.code, semio_framework::fault_text(&conflict.code, false, &[], &[], semio_framework::framework_fault_catalog(), semio_framework::Terminology::Native, if is_de { semio_framework::Locale::De } else { semio_framework::Locale::En }).unwrap_or_default())),'),
]


def drop_message_fields(text: str) -> tuple[str, int]:
    """✂️ Removes the `message: …,` field from every `MutationMessage { … }` struct literal (single- or multi-line, the value
    parsed to its top-level comma)."""
    out, cursor, count = [], 0, 0
    for match in re.finditer(r"\bMutationMessage\s*\{", text):
        start = match.end()
        depth, at, field_start = 1, start, None
        while depth > 0:
            char = text[at]
            if char == '"':
                at += 1
                while text[at] != '"':
                    at += 2 if text[at] == "\\" else 1
            elif char in "([{":
                depth += 1
            elif char in ")]}":
                depth -= 1
            at += 1
        body = text[start:at - 1]
        field = re.search(r"(^|,)(\s*)message\s*:", body)
        if field is None or field.start() < 0:
            continue
        value_start = field.end()
        depth, index = 0, value_start
        while index < len(body):
            char = body[index]
            if char == '"':
                index += 1
                while body[index] != '"':
                    index += 2 if body[index] == "\\" else 1
            elif char in "([{":
                depth += 1
            elif char in ")]}":
                depth -= 1
            elif char == "," and depth == 0:
                break
            index += 1
        removal_start = start + field.start() + (1 if field.group(1) == "," else 0)
        removal_end = start + index + (1 if index < len(body) else 0)
        out.append(text[cursor:removal_start])
        cursor = removal_end
        count += 1
    out.append(text[cursor:])
    return "".join(out), count


def main(dry_run: bool) -> None:
    texts: dict[str, str] = {}
    for rel, old, new in EDITS:
        text = texts.setdefault(rel, (OVERLAY / rel).read_text())
        if (new and new in text and (old in new or old not in text)) or (not new and old not in text):
            continue
        assert text.count(old) == 1, (rel, old[:100], text.count(old))
        texts[rel] = text.replace(old, new)
        print("apply", rel.split("/")[-2], (new or old).strip().splitlines()[0][:90])
    sync = texts.setdefault(SYNC, (OVERLAY / SYNC).read_text())
    texts[SYNC], dropped = drop_message_fields(sync)
    print(f"sync MutationMessage literals: {dropped} message fields dropped")
    host = texts.setdefault(SHELL_HOST, (OVERLAY / SHELL_HOST).read_text())
    print("ShellHost worst.message uses:", len(re.findall(r"worst\.message", host)))
    if not dry_run:
        for rel, text in texts.items():
            if (OVERLAY / rel).read_text() != text:
                (OVERLAY / rel).write_text(text)


if __name__ == "__main__":
    main("--dry-run" in sys.argv[1:])
