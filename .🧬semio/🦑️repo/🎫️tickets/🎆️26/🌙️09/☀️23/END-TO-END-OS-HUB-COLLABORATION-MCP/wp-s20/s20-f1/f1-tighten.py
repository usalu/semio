"""🔒️ S20 F1 (faults overlay): tightens the fault constructors so every `FaultCode` a crate can raise is a `&'static str`
the fault census reads — `FaultCode::new(&'static str)`, `Fault::new(origin, FaultCode, message)`, no `From` conversion
into `FaultCode`/`Fault` (the untyped `app.message` fault is gone), `FaultCode::received` for decoded records, and
`Fault.parameters` boxed behind one word (`FaultParameters`) so `Fault` stays inside its inline budget. Idempotent.
Usage: python3 f1-tighten.py <root>"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[1])
F = ROOT / "🧰️framework/🔨️modules/⚠️diagnostic/🦀️.rs"

HUNKS = [
    (
        "impl FaultCode {\n    pub fn new(code: impl Into<String>) -> Self {\n        Self(code.into())\n    }\n}\n\nimpl From<&'static str> for FaultCode {\n    fn from(value: &'static str) -> Self {\n        Self(value.to_string())\n    }\n}\n",
        "impl FaultCode {\n"
        "    /// 🏷️ A code a crate raises: always a string literal (or a `const` bound to one), so the fault census\n"
        "    /// (`bun ./📜️script.ts verify faults`) reads every code a crate can raise and holds it to its declaration.\n"
        "    pub fn new(code: &'static str) -> Self {\n        Self(code.to_string())\n    }\n\n"
        "    /// 🔁️ A code decoded from a record another raise site produced (a typed-operation page, a job detail, a wire\n"
        "    /// frame) — the census checked that raise site; this never raises a code of its own. The census admits only a\n"
        "    /// record's `code` field here.\n"
        "    pub fn received(code: String) -> Self {\n        Self(code)\n    }\n\n"
        "    pub fn as_str(&self) -> &str {\n        &self.0\n    }\n}\n",
    ),
    (
        "/// 🏷️ Stable, greppable diagnostic identifier, e.g. `\"DSL0001\"`.\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub struct DiagnosticCode(pub &'static str);\n\nimpl From<String> for FaultCode {\n    fn from(value: String) -> Self {\n        Self(value)\n    }\n}\n\nimpl From<DiagnosticCode> for FaultCode {\n    fn from(value: DiagnosticCode) -> Self {\n        FaultCode::new(value.0)\n    }\n}\n\n",
        "",
    ),
    (
        "    /// 🧩️ The named values the declared text of `code` shows (`{name}`) — see [`FaultParameter`].\n    pub parameters: Vec<FaultParameter>,\n",
        "    /// 🧩️ The named values the declared text of `code` shows (`{name}`) — see [`FaultParameter`].\n    pub parameters: FaultParameters,\n",
    ),
    (
        "/// 🔗️ One hop in a {@link Fault} cause chain.\n",
        "/// 🧩️ The named values of one fault, boxed behind one word so a fault without values (the common case) keeps\n"
        "/// `Fault` inside its inline budget (`🧫️fixtures/🧯️fault/🔣️.json` `maximumInlineBytes`). Holds at most\n"
        "/// [`FAULT_PARAMETERS_MAXIMUM`].\n"
        "#[derive(Clone, Debug, Default, PartialEq, Eq)]\n"
        "pub struct FaultParameters(Option<Box<Vec<FaultParameter>>>);\n\n"
        "impl FaultParameters {\n"
        "    pub fn as_slice(&self) -> &[FaultParameter] {\n        self.0.as_deref().map_or(&[], Vec::as_slice)\n    }\n\n"
        "    pub fn is_empty(&self) -> bool {\n        self.as_slice().is_empty()\n    }\n\n"
        "    fn push(&mut self, parameter: FaultParameter) {\n"
        "        let parameters = self.0.get_or_insert_with(Box::default);\n"
        "        if parameters.len() < FAULT_PARAMETERS_MAXIMUM {\n            parameters.push(parameter);\n        }\n    }\n}\n\n"
        "impl FromIterator<FaultParameter> for FaultParameters {\n"
        "    fn from_iter<I: IntoIterator<Item = FaultParameter>>(iter: I) -> Self {\n"
        "        let parameters: Vec<FaultParameter> = iter.into_iter().take(FAULT_PARAMETERS_MAXIMUM).collect();\n"
        "        Self((!parameters.is_empty()).then(|| Box::new(parameters)))\n    }\n}\n\n"
        "/// 🔗️ One hop in a {@link Fault} cause chain.\n",
    ),
    (
        "            entries.push((\"parameters\".to_string(), DslValue::Array(self.parameters.iter().map(ToValue::to_value).collect())));\n",
        "            entries.push((\"parameters\".to_string(), DslValue::Array(self.parameters.as_slice().iter().map(ToValue::to_value).collect())));\n",
    ),
    (
        "            parameters: match find(\"parameters\") {\n                None | Some(DslValue::Null) => Vec::new(),\n",
        "            parameters: match find(\"parameters\") {\n                None | Some(DslValue::Null) => FaultParameters::default(),\n",
    ),
    (
        "impl From<&str> for Fault {\n    fn from(value: &str) -> Self {\n        Fault::new(FaultOrigin::App, FaultCode::new(\"app.message\"), value)\n    }\n}\n\nimpl From<String> for Fault {\n    fn from(value: String) -> Self {\n        Fault::new(FaultOrigin::App, FaultCode::new(\"app.message\"), value)\n    }\n}\n\n",
        "",
    ),
    (
        "    pub fn new(origin: FaultOrigin, code: impl Into<FaultCode>, message: impl Into<String>) -> Self {\n        Self { origin, code: code.into(), severity: Severity::Error, message: message.into(), scope: Box::default(), span: None, causes: Vec::new(), parameters: Vec::new(), retryable: false }\n",
        "    pub fn new(origin: FaultOrigin, code: FaultCode, message: impl Into<String>) -> Self {\n        Self { origin, code, severity: Severity::Error, message: message.into(), scope: Box::default(), span: None, causes: Vec::new(), parameters: FaultParameters::default(), retryable: false }\n",
    ),
    (
        "    /// value is clipped to [`FAULT_PARAMETER_VALUE_BYTES`] on a character boundary and its control characters become\n    /// spaces. The fault law (`verify faults`) holds the names to the declared text's placeholders.\n",
        "    /// value is clipped to [`FAULT_PARAMETER_VALUE_BYTES`] on a character boundary and its control characters become\n    /// spaces; values past [`FAULT_PARAMETERS_MAXIMUM`] are not kept. The fault law (`verify faults`) holds the names to\n    /// the declared text's placeholders.\n",
    ),
    (
        "        let parameters: Vec<String> = self.parameters.iter().map(|parameter| format!(\"{}={}\", parameter.name, parameter.value)).collect();\n",
        "        let parameters: Vec<String> = self.parameters.as_slice().iter().map(|parameter| format!(\"{}={}\", parameter.name, parameter.value)).collect();\n",
    ),
    (
        "causes: self.fault_causes(), parameters: Vec::new(), retryable: self.fault_retryable() }\n",
        "causes: self.fault_causes(), parameters: FaultParameters::default(), retryable: self.fault_retryable() }\n",
    ),
    (
        ".unwrap_or_else(|| Fault::new(FaultOrigin::Os, \"os.fault.decode\", String::from_utf8_lossy(bytes)))\n",
        ".unwrap_or_else(|| Fault::new(FaultOrigin::Os, FaultCode::new(\"os.fault.decode\"), String::from_utf8_lossy(bytes)))\n",
    ),
    (
        "    pub fn error(code: &'static str, span: TextSpan, message: impl Into<String>) -> Self {\n        Self { code: FaultCode::new(code), severity: Severity::Error, span, message: message.into(), expected: None, scope: FaultScope::default() }\n    }\n",
        "    /// 🩺️ One finding at `span` with its literal `code` and `severity` — a conformance or parse finding in a report,\n"
        "    /// never a refusal (a refusal is a `Fault`).\n"
        "    pub fn new(code: &'static str, severity: Severity, span: TextSpan, message: impl Into<String>) -> Self {\n        Self { code: FaultCode::new(code), severity, span, message: message.into(), expected: None, scope: FaultScope::default() }\n    }\n\n"
        "    pub fn error(code: &'static str, span: TextSpan, message: impl Into<String>) -> Self {\n        Self::new(code, Severity::Error, span, message)\n    }\n",
    ),
    (
        "pub fn app_fault(code: &'static str) -> Fault {\n    Fault::new(FaultOrigin::App, FaultCode::new(code), String::new())\n}\n",
        "pub fn app_fault(code: &'static str) -> Fault {\n    Fault::new(FaultOrigin::App, FaultCode::new(code), String::new())\n}\n\n"
        "/// 🧯️ An app's refusal through a helper that takes the code as a `FaultCode` — the literal sits where the helper is\n"
        "/// called (`refuse(FaultCode::new(\"…\"))`), so the fault census reads it there; the refusal carries no free text.\n"
        "pub fn app_refusal(code: FaultCode) -> Fault {\n    Fault::new(FaultOrigin::App, code, String::new())\n}\n",
    ),
]


def main() -> None:
    text = F.read_text()
    for before, after in HUNKS:
        if (after != "" and text.count(after) == 1) or (after == "" and before not in text):
            print(f"applied  {before[:60]!r}")
        elif text.count(before) == 1:
            text = text.replace(before, after)
            print(f"apply    {before[:60]!r}")
        else:
            sys.exit(f"CONFLICT {before[:80]!r}")
    F.write_text(text)


main()
