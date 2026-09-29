"""🔒️ S20 F1 (faults overlay): `FaultCode` has no `From` conversion any more, so the public `MutationMessage`/
`MutationOutcome` constructors take `impl MutationMessageCode` — a `&'static str` literal or an already-built
`FaultCode` (a forwarded refusal), nothing else. Idempotent.
Usage: python3 f1-mutation-message.py <root>"""
import sys
from pathlib import Path

F = Path(sys.argv[1]) / "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs"
INTO = "code: impl Into<crate::diagnostic::FaultCode>"
EDITS = [
    ("    fn at_level(level: crate::diagnostic::Severity, code: impl Into<crate::diagnostic::FaultCode>, message: impl Into<String>) -> Self {\n        Self { level, code: code.into(), message: message.into(), target: Vec::new(), op_index: None }\n    }\n",
     "    fn at_level(level: crate::diagnostic::Severity, code: crate::diagnostic::FaultCode, message: impl Into<String>) -> Self {\n        Self { level, code, message: message.into(), target: Vec::new(), op_index: None }\n    }\n"),
    ("        Self::at_level(crate::diagnostic::Severity::Info, code, message)", "        Self::at_level(crate::diagnostic::Severity::Info, crate::diagnostic::FaultCode::new(code), message)"),
    ("        Self::at_level(crate::diagnostic::Severity::Warning, code, message)", "        Self::at_level(crate::diagnostic::Severity::Warning, crate::diagnostic::FaultCode::new(code), message)"),
    ("        Self::at_level(crate::diagnostic::Severity::Error, code, message)", "        Self::at_level(crate::diagnostic::Severity::Error, crate::diagnostic::FaultCode::new(code), message)"),
    ("        Self::at_level(crate::diagnostic::Severity::Fatal, code, message)", "        Self::at_level(crate::diagnostic::Severity::Fatal, crate::diagnostic::FaultCode::new(code), message)"),
    ("messages.push(MutationMessage::fatal(error.code, error.message).at(error.target));", "messages.push(MutationMessage::at_level(crate::diagnostic::Severity::Fatal, crate::diagnostic::FaultCode::received(error.code), error.message).at(error.target));"),
]


def main() -> None:
    text = F.read_text()
    if "pub trait MutationMessageCode" in text:
        forwarded = "MutationMessage::at_level(crate::diagnostic::Severity::Fatal, error.code, error.message)"
        if forwarded in text:
            F.write_text(text.replace(forwarded, "MutationMessage::at_level(crate::diagnostic::Severity::Fatal, crate::diagnostic::FaultCode::received(error.code), error.message)"))
        print("mutation messages: applied")
        return
    for before, after in EDITS:
        if after in text:
            continue
        if text.count(before) != 1:
            sys.exit(f"CONFLICT {before[:80]} ×{text.count(before)}")
        text = text.replace(before, after)
    text = text.replace("code: &'static str, message: impl Into<String>", "code: impl MutationMessageCode, message: impl Into<String>").replace(INTO, "code: impl MutationMessageCode")
    for level in ("Info", "Warning", "Error", "Fatal"):
        text = text.replace(f"        Self::at_level(crate::diagnostic::Severity::{level}, crate::diagnostic::FaultCode::new(code), message)", f"        Self::at_level(crate::diagnostic::Severity::{level}, code.into_fault_code(), message)")
    trait = (
        "/// 🏷️ What a mutation report message names as its code: a literal (`&'static str`, the census reads it) or a\n"
        "/// `FaultCode` a refusal already carries — never a runtime string.\n"
        "pub trait MutationMessageCode {\n    fn into_fault_code(self) -> crate::diagnostic::FaultCode;\n}\n\n"
        "impl MutationMessageCode for &'static str {\n    fn into_fault_code(self) -> crate::diagnostic::FaultCode {\n        crate::diagnostic::FaultCode::new(self)\n    }\n}\n\n"
        "impl MutationMessageCode for crate::diagnostic::FaultCode {\n    fn into_fault_code(self) -> crate::diagnostic::FaultCode {\n        self\n    }\n}\n\n"
    )
    if "pub trait MutationMessageCode" not in text:
        anchor = "impl MutationMessage {\n    fn at_level("
        if text.count(anchor) != 1:
            sys.exit("CONFLICT trait anchor")
        text = text.replace(anchor, trait + anchor)
    F.write_text(text)
    print(f"mutation messages: applied ({text.count('impl MutationMessageCode')} MutationMessageCode params)")


main()
