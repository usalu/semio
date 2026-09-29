"""🧯️ S20 F1 (faults overlay): `Fault.parameters` + `app_fault` + `with_parameter` in `⚠️diagnostic/🦀️.rs`. Idempotent.
Usage: python3 f1-diagnostic.py <root>"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[1])
F = ROOT / "🧰️framework/🔨️modules/⚠️diagnostic/🦀️.rs"
HUNKS = [
    ("/// 🔗️ One hop in a {@link Fault} cause chain.\n",
     "/// 🧩️ One named runtime value a refusal carries for the text a host renders BY ITS CODE — the declared text's `{name}`\n"
     "/// placeholder (an app's `FaultDefinition`, the framework fault catalog). The only way a refusal carries a value: a\n"
     "/// refusal never carries prose.\n"
     "#[derive(Clone, Debug, PartialEq, Eq)]\n"
     "pub struct FaultParameter {\n    pub name: String,\n    pub value: String,\n}\n\n"
     "/// 📏️ Parameters one refusal carries at most.\npub const FAULT_PARAMETERS_MAXIMUM: usize = 8;\n"
     "/// 📏️ Longest parameter name, in UTF-8 bytes (`[a-z][A-Za-z0-9]*`).\npub const FAULT_PARAMETER_NAME_BYTES: usize = 32;\n"
     "/// 📏️ Longest parameter value, in UTF-8 bytes — clipped on a character boundary, control characters become spaces.\n"
     "pub const FAULT_PARAMETER_VALUE_BYTES: usize = 64;\n\n"
     "/// 🔗️ One hop in a {@link Fault} cause chain.\n"),
    ("    pub causes: Vec<FaultCause>,\n    pub retryable: bool,\n}\n",
     "    pub causes: Vec<FaultCause>,\n    /// 🧩️ The named values the declared text of `code` shows (`{name}`) — see [`FaultParameter`].\n    pub parameters: Vec<FaultParameter>,\n    pub retryable: bool,\n}\n"),
    ("/// 🌉️ Hand-written — see `FaultOrigin` above.\nimpl ToValue for FaultCause {\n",
     "/// 🌉️ Hand-written — see `FaultOrigin` above.\nimpl ToValue for FaultParameter {\n    fn to_value(&self) -> DslValue {\n        DslValue::Object(vec![(\"name\".to_string(), DslValue::String(self.name.clone())), (\"value\".to_string(), DslValue::String(self.value.clone()))])\n    }\n}\n"
     "impl FromValue for FaultParameter {\n    fn from_value(value: DslValue) -> Result<Self, ValueError> {\n        let entries = match value {\n            DslValue::Object(entries) => entries,\n            other => return Err(ValueError::new(format!(\"expected an object for FaultParameter, found {other:?}\"))),\n        };\n"
     "        let text = |key: &str| match entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone()) {\n            Some(DslValue::String(text)) => Ok(text),\n            other => Err(ValueError::new(format!(\"expected a string for FaultParameter.{key}, found {other:?}\"))),\n        };\n"
     "        Ok(FaultParameter { name: text(\"name\")?, value: text(\"value\")? })\n    }\n}\n\n"
     "/// 🌉️ Hand-written — see `FaultOrigin` above.\nimpl ToValue for FaultCause {\n"),
    ("            entries.push((\"causes\".to_string(), DslValue::Array(self.causes.iter().map(ToValue::to_value).collect())));\n        }\n        entries.push((\"retryable\".to_string(), DslValue::Bool(self.retryable)));\n",
     "            entries.push((\"causes\".to_string(), DslValue::Array(self.causes.iter().map(ToValue::to_value).collect())));\n        }\n        if !self.parameters.is_empty() {\n            entries.push((\"parameters\".to_string(), DslValue::Array(self.parameters.iter().map(ToValue::to_value).collect())));\n        }\n        entries.push((\"retryable\".to_string(), DslValue::Bool(self.retryable)));\n"),
    ("                Some(other) => return Err(ValueError::new(format!(\"expected an array for Fault.causes, found {other:?}\"))),\n            },\n",
     "                Some(other) => return Err(ValueError::new(format!(\"expected an array for Fault.causes, found {other:?}\"))),\n            },\n            parameters: match find(\"parameters\") {\n                None | Some(DslValue::Null) => Vec::new(),\n                Some(DslValue::Array(items)) => items.into_iter().map(FaultParameter::from_value).collect::<Result<_, _>>()?,\n                Some(other) => return Err(ValueError::new(format!(\"expected an array for Fault.parameters, found {other:?}\"))),\n            },\n"),
    ("        Self { origin, code: code.into(), severity: Severity::Error, message: message.into(), scope: Box::default(), span: None, causes: Vec::new(), retryable: false }\n",
     "        Self { origin, code: code.into(), severity: Severity::Error, message: message.into(), scope: Box::default(), span: None, causes: Vec::new(), parameters: Vec::new(), retryable: false }\n"),
    ("    pub fn with_retryable(mut self, retryable: bool) -> Self {\n        self.retryable = retryable;\n        self\n    }\n",
     "    pub fn with_retryable(mut self, retryable: bool) -> Self {\n        self.retryable = retryable;\n        self\n    }\n\n"
     "    /// 🧩️ Adds one named value the declared text of this fault's code shows as `{name}` (see [`FaultParameter`]): the\n"
     "    /// value is clipped to [`FAULT_PARAMETER_VALUE_BYTES`] on a character boundary and its control characters become\n"
     "    /// spaces. The fault law (`verify faults`) holds the names to the declared text's placeholders.\n"
     "    pub fn with_parameter(mut self, name: &'static str, value: impl Into<String>) -> Self {\n"
     "        let value: String = value.into().chars().map(|character| if character.is_control() { ' ' } else { character }).collect();\n"
     "        let mut end = value.len().min(FAULT_PARAMETER_VALUE_BYTES);\n"
     "        while !value.is_char_boundary(end) {\n            end -= 1;\n        }\n"
     "        self.parameters.push(FaultParameter { name: name.to_string(), value: value[..end].to_string() });\n"
     "        self\n    }\n"),
    ("    pub fn describe(&self) -> String {\n        format!(\"{}: {}\", self.code.0, self.message)\n    }\n}\n",
     "    pub fn describe(&self) -> String {\n        let parameters: Vec<String> = self.parameters.iter().map(|parameter| format!(\"{}={}\", parameter.name, parameter.value)).collect();\n"
     "        if parameters.is_empty() {\n            format!(\"{}: {}\", self.code.0, self.message)\n        } else {\n            format!(\"{} {{{}}}: {}\", self.code.0, parameters.join(\", \"), self.message)\n        }\n    }\n}\n\n"
     "/// 🧯️ An app's refusal: its frozen `code` and nothing else — the person reads the text the app DECLARES for the code\n"
     "/// (`AppDefinition.faults`, the builder's `.fault(code, text)`), filled with the values [`Fault::with_parameter`] adds.\n"
     "/// A refusal never carries free text. `code` is a string literal so the fault census reads it (`verify faults`).\n"
     "pub fn app_fault(code: &'static str) -> Fault {\n    Fault::new(FaultOrigin::App, FaultCode::new(code), String::new())\n}\n"),
    ("        Fault { origin: self.fault_origin(), code: self.fault_code(), severity: self.fault_severity(), message: self.fault_message(), scope: Box::new(self.fault_scope()), span: self.fault_span(), causes: self.fault_causes(), retryable: self.fault_retryable() }\n",
     "        Fault { origin: self.fault_origin(), code: self.fault_code(), severity: self.fault_severity(), message: self.fault_message(), scope: Box::new(self.fault_scope()), span: self.fault_span(), causes: self.fault_causes(), parameters: Vec::new(), retryable: self.fault_retryable() }\n"),
]


def main() -> None:
    text = F.read_text()
    if "pub struct FaultParameters(" in text:
        print("diagnostic: F1 applied (tightened by f1-tighten.py)")
        return
    for before, after in HUNKS:
        if text.count(after) == 1:
            continue
        assert text.count(before) == 1, before[:80]
        text = text.replace(before, after)
    F.write_text(text)
    print("diagnostic: F1 applied")


if __name__ == "__main__":
    main()
