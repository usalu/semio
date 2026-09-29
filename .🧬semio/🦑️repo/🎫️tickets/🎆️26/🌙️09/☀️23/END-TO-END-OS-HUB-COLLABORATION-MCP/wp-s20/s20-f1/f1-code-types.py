"""🔒️ S20 F1 (faults overlay): the framework sites that handed `FaultCode::new` a runtime `String`/`&str` take a
`&'static str` (callers already pass literals), and the one `format!` code becomes a literal code with a parameter —
the type half of `FaultCode::new(&'static str)`. Idempotent. Usage: python3 f1-code-types.py <root>"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[1])
M = "🧰️framework/🛍️products/💻️os/🔨️modules"
EDITS = [
    (f"{M}/🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs", "fn fault(code: &str, message: impl Into<String>) -> semio_framework::Fault {", "fn fault(code: &'static str, message: impl Into<String>) -> semio_framework::Fault {"),
    (f"{M}/🔌️plugin/🖥️host/⚡️effects/🦀️.rs", "async fn fault_bytes(code: impl Into<String>, message: impl Into<String>) -> Vec<u8> {", "async fn fault_bytes(code: &'static str, message: impl Into<String>) -> Vec<u8> {"),
    (f"{M}/🔌️plugin/🖥️host/📥️imports/🦀️.rs", "async fn fault_bytes(code: impl Into<String>, message: impl Into<String>) -> Vec<u8> {", "async fn fault_bytes(code: &'static str, message: impl Into<String>) -> Vec<u8> {"),
    (f"{M}/🔌️plugin/🪟️window/🎚️config/🦀️.rs", "let reject = |typed: Box<O::Mutation>, code: &str, message: &str| RejectedWindowConfigEmission {", "let reject = |typed: Box<O::Mutation>, code: &'static str, message: &str| RejectedWindowConfigEmission {"),
    (f"{M}/🔌️plugin/🦀️.rs", "async fn push_os_fault(frames: &mut Vec<protocol::AppFrame>, in_reply_to: Option<u64>, code: &str, message: String) {", "async fn push_os_fault(frames: &mut Vec<protocol::AppFrame>, in_reply_to: Option<u64>, code: &'static str, message: String) {"),
    (f"{M}/🔌️plugin/🦀️.rs", "    pub struct PluginAssemblyError {\n        pub code: String,\n", "    pub struct PluginAssemblyError {\n        pub code: &'static str,\n"),
    (f"{M}/🔌️plugin/🦀️.rs", "        pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {\n            Self { code: code.into(), message: message.into() }\n        }\n\n        /// 🚫️async: E1 pure error constructor consumed pervasively by `.map_err(PluginAssemblyError::definition)`",
     "        pub fn new(code: &'static str, message: impl Into<String>) -> Self {\n            Self { code, message: message.into() }\n        }\n\n        /// 🚫️async: E1 pure error constructor consumed pervasively by `.map_err(PluginAssemblyError::definition)`"),
    (f"{M}/🔌️plugin/🦀️.rs", '            let leg = self.leg();\n            Fault::new(FaultOrigin::Plugin, FaultCode::new(format!("plugin.internal.document-archive-replacement.{}", self.slug())), format!("document archive replacement failed its {} leg: {}", leg.slug(), self.detail()))\n',
     '            let leg = self.leg();\n            let stage = self.slug();\n            Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal.document-archive-replacement"), format!("document archive replacement failed its {} leg: {}", leg.slug(), self.detail())).with_parameter("stage", stage)\n'),
    (f"{M}/🏪️store/🔄️sync/🦀️.rs", "code: crate::os_dsl::FaultCode::new(code), message, target: vec![self.hub_base_url.clone()", "code: crate::os_dsl::FaultCode::received(code), message, target: vec![self.hub_base_url.clone()"),
    (f"{M}/🏪️store/🔄️sync/🦀️.rs", "                        code: crate::os_dsl::FaultCode::new(code),\n", "                        code: crate::os_dsl::FaultCode::received(code),\n"),
    (f"{M}/🏃️run/🦀️.rs", "fn run_fault_bytes(code: impl Into<String>, message: impl Into<String>) -> Vec<u8> {\n    dsl::encode_fault_bytes(&dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new(code.into()), message))",
     "fn run_fault_bytes(code: &'static str, message: impl Into<String>) -> Vec<u8> {\n    dsl::encode_fault_bytes(&dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new(code), message))"),
]


def main() -> None:
    texts: dict[str, str] = {}
    for rel, before, after in EDITS:
        text = texts.setdefault(rel, (ROOT / rel).read_text())
        if text.count(after) >= 1 and before not in text:
            continue
        if text.count(before) != 1:
            sys.exit(f"CONFLICT {rel}: {before[:80]} ×{text.count(before)}")
        texts[rel] = text.replace(before, after)
    for rel, text in texts.items():
        if text != (ROOT / rel).read_text():
            (ROOT / rel).write_text(text)
            print(f"changed {rel.split('/')[-2]}/{rel.split('/')[-1]}")
    print("code types: applied")


main()
