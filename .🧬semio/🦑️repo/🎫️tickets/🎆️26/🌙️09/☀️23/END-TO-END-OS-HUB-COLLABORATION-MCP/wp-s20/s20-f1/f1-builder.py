"""🧯️ S20 F1 (faults overlay): the app builder's `.fault(code, text)` + re-exports (`semio_framework::{app_fault,
FaultParameter}`, hence `semio_framework_plugin::…`). Idempotent. Usage: python3 f1-builder.py <root>"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[1])
SDK = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
FRAMEWORK = ROOT / "🧰️framework/📦️packages/🦀️rust/🦀️.rs"
HUNKS = [
    (FRAMEWORK, "pub use dsl::{Diagnostic, Fault, FaultCause, FaultCode, FaultFrom, FaultOrigin, FaultScope, Severity, TextError, TextSpan};\n",
     "pub use dsl::{app_fault, app_refusal, Diagnostic, Fault, FaultCause, FaultCode, FaultFrom, FaultOrigin, FaultParameter, FaultParameters, FaultScope, Severity, TextError, TextSpan, FAULT_PARAMETERS_MAXIMUM, FAULT_PARAMETER_NAME_BYTES, FAULT_PARAMETER_VALUE_BYTES};\n"),
    (SDK, "        command_grammar: CommandGrammar,\n        io: AppIo,\n    }\n\n    impl AppBuilder {\n",
     "        command_grammar: CommandGrammar,\n        io: AppIo,\n        faults: Vec<semio_framework::FaultDefinition>,\n    }\n\n    impl AppBuilder {\n"),
    (SDK, "                command_grammar: resolve_ready(CommandGrammar::empty()),\n                io: AppIo::default(),\n            }\n        }\n",
     "                command_grammar: resolve_ready(CommandGrammar::empty()),\n                io: AppIo::default(),\n                faults: Vec::new(),\n            }\n        }\n\n"
     "        /// 🧯️ Declares one refusal code this app raises (`app_fault(code)`) with the text a host shows the person\n"
     "        /// in their locale and terminology — `{name}` placeholders take the values the refusal carries\n"
     "        /// (`Fault::with_parameter`). The fault law (`verify faults`) holds every raised code declared, every\n"
     "        /// declared code raised, and both text cells non-empty with the same placeholders.\n"
     "        pub async fn fault(mut self, code: &'static str, text: impl Into<LocalizedLabel>) -> Self {\n"
     "            let text = text.into();\n"
     "            let parameters = semio_framework::fault_text_parameters(text.resolve(semio_framework::Terminology::ALL[0], semio_framework::Locale::ALL[0]));\n"
     "            self.faults.push(semio_framework::FaultDefinition { code: code.to_string(), text, parameters });\n"
     "            self\n"
     "        }\n"),
    (SDK, "                command_grammar: self.command_grammar,\n                io: self.io,\n            };\n",
     "                command_grammar: self.command_grammar,\n                io: self.io,\n                faults: self.faults,\n            };\n"),
    (SDK, "            let classification_result = semio_framework::validate_interactive_job_classification(\n",
     "            semio_framework::validate_fault_definitions(&self.faults).map_err(|message| PluginAssemblyError::new(\"app-definition.invalid\", format!(\"app {}: {message}\", self.id)))?;\n            let classification_result = semio_framework::validate_interactive_job_classification(\n"),
    (SDK, "        action_describe(action_id: impl AsRef<str>, description: impl Into<LocalizedLabel>),\n",
     "        action_describe(action_id: impl AsRef<str>, description: impl Into<LocalizedLabel>),\n        fault(code: &'static str, text: impl Into<LocalizedLabel>),\n"),
]


def main() -> None:
    texts: dict[Path, str] = {}
    for path, before, after in HUNKS:
        text = texts.setdefault(path, path.read_text())
        if text.count(after) == 1:
            continue
        assert text.count(before) == 1, (path.name, before[:70], text.count(before))
        texts[path] = text.replace(before, after)
    for path, text in texts.items():
        path.write_text(text)
    print("builder: F1 applied")


if __name__ == "__main__":
    main()
