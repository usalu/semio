"""🧬️ S20 pass 2 (P2-F1) on the p2 overlay — the framework API of mutation reports by code (spec `📓️fault-localization-api.md`
§6): `MutationCode` (the frozen seven of contract C2, each arm a literal catalogued framework code), `MutationMessage` without
`message` (public constructors take `MutationCode`; `framework_report` carries a catalogued framework code for
framework-internal reports), `MutationOutcome::{error,fatal}(code, target)` + `.info/.warn(code)`, `MutationApplyError {code:
MutationCode, target}`, the `MutationMessageCode` bridge and the census exclusion `FAULT_PASS2_BRIDGE` removed together.
Idempotent (an edit whose result is present is skipped; its anchor must match exactly once otherwise).
Usage: python3 p2-api.py [--dry-run]"""
from __future__ import annotations

import json
import sys
from pathlib import Path

OVERLAY = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults-p2")
MUTATION = "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs"
CENSUS = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts"
CENSUS_FIXTURE = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🧫️fixtures/🧮️source-census/🔣️.json"
CODES = [("TargetMissing", "mutation.target-missing"), ("NoOp", "mutation.no-op"), ("Partial", "mutation.partial"), ("Clamped", "mutation.clamped"),
         ("DuplicateId", "mutation.duplicate-id"), ("Invariant", "mutation.invariant"), ("Cascade", "mutation.cascade")]

CODE_TYPE = """/// 🧊️ The frozen mutation report codes (contract C2 of ticket 26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-
/// CONFLICTS: generic, gate-enforced, no per-plugin codes) — every report ([`MutationMessage`]) and apply rejection
/// ([`MutationApplyError`]) names one; hosts and agents show the text the framework fault catalog declares for its code, and
/// the report's `target` names the element.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MutationCode {
""" + "".join(f"    {variant},\n" for variant, _ in CODES) + """}

impl MutationCode {
    /// 🧊️ Every frozen code, in contract order.
    pub const ALL: [MutationCode; 7] = [""" + ", ".join(f"MutationCode::{variant}" for variant, _ in CODES) + """];

    /// 🏷️ The code's wire name (`mutation.target-missing`, …).
    pub fn as_str(self) -> &'static str {
        match self {
""" + "".join(f'            MutationCode::{variant} => "{code}",\n' for variant, code in CODES) + """        }
    }

    /// 🏷️ The fault code a report carries — one literal per variant, each a catalogued framework code.
    pub fn code(self) -> crate::diagnostic::FaultCode {
        match self {
""" + "".join(f'            MutationCode::{variant} => crate::diagnostic::FaultCode::new("{code}"),\n' for variant, code in CODES) + """        }
    }
}

/// 🌱️ Hand-written, not derived — this crate sits below the derive macro's target crate (see `MutationMessage`).
impl crate::value::ToValue for MutationCode {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::String(self.as_str().to_string())
    }
}
impl crate::value::FromValue for MutationCode {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        match value {
            crate::value::DslValue::String(name) => MutationCode::ALL.into_iter().find(|code| code.as_str() == name).ok_or_else(|| crate::value::ValueError::new(format!("unknown mutation code `{name}`"))),
            other => Err(crate::value::ValueError::new(format!("expected a string, found {other:?}"))),
        }
    }
}

"""

EDITS: list[tuple[str, str, str]] = [
    (MUTATION, "//#region 🔖️Mutation\n/// 🚫️ Structured rejection of a diff that cannot be applied to its supplied base.\n/// The shape is protocol-owned and wire-safe: callers never need a technology crate's error type\n/// to preserve the stable machine code, human diagnostic, and outermost-first target address.\n#[derive(Clone, Debug, PartialEq, Eq)]\npub struct MutationApplyError {\n    pub code: String,\n    pub message: String,\n    pub target: Vec<String>,\n}\n",
     "//#region 🔖️Mutation\n" + CODE_TYPE + "/// 🚫️ Structured rejection of a diff that cannot be applied to its supplied base.\n/// The shape is protocol-owned and wire-safe: callers never need a technology crate's error type\n/// to preserve the frozen code and the outermost-first target address.\n#[derive(Clone, Debug, PartialEq, Eq)]\npub struct MutationApplyError {\n    pub code: MutationCode,\n    pub target: Vec<String>,\n}\n"),
    (MUTATION, '        let mut entries = vec![("code".to_string(), crate::value::ToValue::to_value(&self.code)), ("message".to_string(), crate::value::ToValue::to_value(&self.message))];\n',
     '        let mut entries = vec![("code".to_string(), crate::value::ToValue::to_value(&self.code))];\n'),
    (MUTATION, """        let mut code = None;
        let mut message = None;
        let mut target = Vec::new();
        for (key, entry) in fields {
            match key.as_str() {
                "code" => code = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("code"))?),
                "message" => message = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("message"))?),
                "target" => target = <Vec<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("target"))?,
                _ => {}
            }
        }
        Ok(MutationApplyError {
            code: code.ok_or_else(|| crate::value::ValueError::new("MutationApplyError missing code"))?,
            message: message.ok_or_else(|| crate::value::ValueError::new("MutationApplyError missing message"))?,
            target,
        })""",
     """        let mut code = None;
        let mut target = Vec::new();
        for (key, entry) in fields {
            match key.as_str() {
                "code" => code = Some(<MutationCode as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("code"))?),
                "target" => target = <Vec<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("target"))?,
                _ => {}
            }
        }
        Ok(MutationApplyError { code: code.ok_or_else(|| crate::value::ValueError::new("MutationApplyError missing code"))?, target })"""),
    (MUTATION, '        write!(formatter, "{}: {}", self.code, self.message)\n', '        write!(formatter, "{} at {}", self.code.as_str(), self.target.join("/"))\n'),
    (MUTATION, "    /// 🏗️ Builds an untargeted typed rejection.\n    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {\n        Self { code: code.into(), message: message.into(), target: Vec::new() }\n",
     "    /// 🏗️ Builds an untargeted typed rejection.\n    pub fn new(code: MutationCode) -> Self {\n        Self { code, target: Vec::new() }\n"),
    (MUTATION, "/// codes (`.🧬semio/🦑️repo/🎫️tickets/26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/\n/// 📋️contract-freeze.md` §C2 — closed set, no per-plugin codes, ever); `message` is English prose\n/// (UI localizes by `code`, never by parsing `message`); `target` is the address of the offending\n",
     "/// codes ([`MutationCode`], contract C2 — closed set, no per-plugin codes, ever) or a catalogued framework code for a\n/// framework-internal report ([`MutationMessage::framework_report`]); a report carries no prose — hosts and agents show the\n/// catalog's text for `code`; `target` is the address of the offending\n"),
    (MUTATION, "    pub code: crate::diagnostic::FaultCode,\n    pub message: String,\n    pub target: Vec<String>,\n    pub op_index: Option<u32>,\n}\n",
     "    pub code: crate::diagnostic::FaultCode,\n    pub target: Vec<String>,\n    pub op_index: Option<u32>,\n}\n"),
    (MUTATION, '            ("code".to_string(), crate::value::ToValue::to_value(&self.code)),\n            ("message".to_string(), crate::value::ToValue::to_value(&self.message)),\n        ];\n',
     '            ("code".to_string(), crate::value::ToValue::to_value(&self.code)),\n        ];\n'),
    (MUTATION, "        let mut code = None;\n        let mut message = None;\n        let mut target = Vec::new();\n        let mut op_index = None;\n",
     "        let mut code = None;\n        let mut target = Vec::new();\n        let mut op_index = None;\n"),
    (MUTATION, '                "message" => message = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("message"))?),\n                "target" => target = <Vec<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("target"))?,\n                "opIndex"',
     '                "target" => target = <Vec<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("target"))?,\n                "opIndex"'),
    (MUTATION, '            message: message.ok_or_else(|| crate::value::ValueError::new("MutationMessage missing message"))?,\n', ""),
    (MUTATION, """    fn at_level(level: crate::diagnostic::Severity, code: crate::diagnostic::FaultCode, message: impl Into<String>) -> Self {
        Self { level, code, message: message.into(), target: Vec::new(), op_index: None }
    }

    pub fn info(code: impl MutationMessageCode, message: impl Into<String>) -> Self {
        Self::at_level(crate::diagnostic::Severity::Info, code.into_fault_code(), message)
    }
    pub fn warn(code: impl MutationMessageCode, message: impl Into<String>) -> Self {
        Self::at_level(crate::diagnostic::Severity::Warning, code.into_fault_code(), message)
    }
    pub fn error(code: impl MutationMessageCode, message: impl Into<String>) -> Self {
        Self::at_level(crate::diagnostic::Severity::Error, code.into_fault_code(), message)
    }
    pub fn fatal(code: impl MutationMessageCode, message: impl Into<String>) -> Self {
        Self::at_level(crate::diagnostic::Severity::Fatal, code.into_fault_code(), message)
    }
""", """    /// 🧾️ A framework-internal report under a catalogued framework code (document-link status, conflict grading) — app
    /// reports name a [`MutationCode`] through [`Self::info`]/[`Self::warn`]/[`Self::error`]/[`Self::fatal`].
    pub fn framework_report(level: crate::diagnostic::Severity, code: crate::diagnostic::FaultCode) -> Self {
        Self { level, code, target: Vec::new(), op_index: None }
    }

    pub fn info(code: MutationCode) -> Self {
        Self::framework_report(crate::diagnostic::Severity::Info, code.code())
    }
    pub fn warn(code: MutationCode) -> Self {
        Self::framework_report(crate::diagnostic::Severity::Warning, code.code())
    }
    pub fn error(code: MutationCode) -> Self {
        Self::framework_report(crate::diagnostic::Severity::Error, code.code())
    }
    pub fn fatal(code: MutationCode) -> Self {
        Self::framework_report(crate::diagnostic::Severity::Fatal, code.code())
    }
"""),
    (MUTATION, """    pub fn fatal(code: impl MutationMessageCode, message: impl Into<String>, target: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { diff: D::default(), messages: vec![MutationMessage::fatal(code, message).at(target)] }""",
     """    pub fn fatal(code: MutationCode, target: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { diff: D::default(), messages: vec![MutationMessage::fatal(code).at(target)] }"""),
    (MUTATION, """    pub fn error(code: impl MutationMessageCode, message: impl Into<String>, target: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { diff: D::default(), messages: vec![MutationMessage::error(code, message).at(target)] }""",
     """    pub fn error(code: MutationCode, target: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { diff: D::default(), messages: vec![MutationMessage::error(code).at(target)] }"""),
    (MUTATION, "                messages.push(MutationMessage::at_level(crate::diagnostic::Severity::Fatal, crate::diagnostic::FaultCode::received(error.code), error.message).at(error.target));\n",
     "                messages.push(MutationMessage::fatal(error.code).at(error.target));\n"),
    (MUTATION, """    pub fn info(mut self, code: impl MutationMessageCode, message: impl Into<String>) -> Self {
        self.messages.push(MutationMessage::info(code, message));""",
     """    pub fn info(mut self, code: MutationCode) -> Self {
        self.messages.push(MutationMessage::info(code));"""),
    (MUTATION, """    pub fn warn(mut self, code: impl MutationMessageCode, message: impl Into<String>) -> Self {
        self.messages.push(MutationMessage::warn(code, message));""",
     """    pub fn warn(mut self, code: MutationCode) -> Self {
        self.messages.push(MutationMessage::warn(code));"""),
    (MUTATION, "/// instance builders alongside the static `::error(code,msg,target).await`/`::fatal(code,msg,target).await`\n", "/// instance builders alongside the static `::error(code, target)`/`::fatal(code, target)`\n"),
    (MUTATION, "/// whole-outcome-rejecting shortcuts (the common case for a simple non-batch verb) and `info`/`warn`\n/// as the 2-arg chainable instance builders (the fan-out recipe's own `.info(\"mutation.cascade\", ..).await`\n/// example); a leaf needing a TARGETED error/warning alongside a non-empty diff (the batch-verb case).await\n/// builds the message directly (`MutationMessage::error(code, msg).await.at(target).await`) and attaches it via\n",
     "/// whole-outcome-rejecting shortcuts (the common case for a simple non-batch verb) and `info`/`warn`\n/// as the chainable instance builders (`.info(MutationCode::Cascade)`); a leaf needing a TARGETED error/warning\n/// alongside a non-empty diff (the batch-verb case) builds the message directly (`MutationMessage::error(code).at(target)`)\n/// and attaches it via\n"),
    (CENSUS, """/** 🌉️ The ONE exclusion of the fault law, for pass 1 only: the mutation-report bridge turns a report's literal code into a
 * `FaultCode` (`FaultCode::new(self)`). Mutation reports are localized by code in pass 2, whose landing removes the bridge
 * and this exclusion together; nothing else is ever excluded. */
export const FAULT_PASS2_BRIDGE = { path: "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs", item: "impl MutationMessageCode for &'static str" } as const;
""", ""),
    (CENSUS, "  const bridge = path === FAULT_PASS2_BRIDGE.path ? itemBodySpan(masked, FAULT_PASS2_BRIDGE.item) : null;\n", ""),
    (CENSUS, "    else if (!definitions && !(bridge !== null && event > bridge.start && event < bridge.end)) violate(event, \"computed-code\", `FaultCode::new(${argument})`);\n",
     "    else if (!definitions) violate(event, \"computed-code\", `FaultCode::new(${argument})`);\n"),
    (CENSUS, """/** 🧱️ The span of one item's `{…}` body in literal-masked Rust (`null` when the item is absent). */
function itemBodySpan(masked: string, item: string): { start: number; end: number } | null {
  const at = masked.indexOf(item);
  const open = at < 0 ? -1 : masked.indexOf("{", at + item.length);
  if (open < 0) return null;
  let depth = 0;
  for (let index = open; index < masked.length; index += 1) {
    if (masked[index] === "{") depth += 1;
    else if (masked[index] === "}" && --depth === 0) return { start: open, end: index };
  }
  return null;
}

""", ""),
]


def trait_removed(text: str) -> str:
    """🌉️ Removes the `MutationMessageCode` trait and both impls (the pass-1 bridge)."""
    start = text.find("/// 🏷️ What a mutation report message names as its code:")
    if start < 0:
        return text
    end = text.index("impl MutationMessage {", start)
    return text[:start] + text[end:]


def census_fixture(text: str) -> str:
    fixture = json.loads(text)
    name = "the pass-2 mutation-report bridge is the one excluded computed code; the same form elsewhere in its module is not"
    lines = [line for line in text.split("\n") if f'"name": "{name}"' not in line]
    kept = "\n".join(lines)
    if kept != text:
        kept = kept.replace(",\n  ],\n  \"faultLaw\"", "\n  ],\n  \"faultLaw\"")
        json.loads(kept)
    return kept if any(case["name"] == name for case in fixture["faultFacts"]) else text


def main(dry_run: bool) -> None:
    texts: dict[str, str] = {}
    for rel, old, new in EDITS:
        text = texts.setdefault(rel, (OVERLAY / rel).read_text())
        if (new and new in text and (old in new or old not in text)) or (not new and old not in text):
            continue
        assert text.count(old) == 1, (rel, old[:100], text.count(old))
        texts[rel] = text.replace(old, new)
        print("apply", rel.split("/")[-2], (new or old).splitlines()[0][:90])
    texts[MUTATION] = trait_removed(texts[MUTATION])
    fixture = (OVERLAY / CENSUS_FIXTURE).read_text()
    texts[CENSUS_FIXTURE] = census_fixture(fixture)
    print("fixture", "bridge case removed" if texts[CENSUS_FIXTURE] != fixture else "unchanged")
    if not dry_run:
        for rel, text in texts.items():
            if (OVERLAY / rel).read_text() != text:
                (OVERLAY / rel).write_text(text)


if __name__ == "__main__":
    main("--dry-run" in sys.argv[1:])
