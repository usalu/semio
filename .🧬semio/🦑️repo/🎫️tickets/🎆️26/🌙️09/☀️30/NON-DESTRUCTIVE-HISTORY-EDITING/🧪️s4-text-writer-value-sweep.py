#!/usr/bin/env python3
"""🧹️ S4-TEXT (session 4): moves the writer crate's non-sqlite files onto the owned `semio_framework_value::ValueError` refusal
(peer value/DSL extraction fallout): `IoError { message }` → `IoError::from_value_error`, retirement `close_step` and store-initializer
pumps → `ValueError`, the generated language-kind codec → the generator's current `semio_framework_value` template, `TextError::new`
with its refusal kind, `dsl::JsonValue`/`dsl::DslValue` → their owners, serializer tests with `ArchiveChildren::empty()`.
The sqlite-snapshot files are S4-INFRA's and untouched. Every replacement asserts its exact count. Run from the repo root."""
import re
import sys
from pathlib import Path

W = Path("✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer")
S = W / "🏅️standards/🔖️1/🪆️subsets/✳️any"
VE = "semio_framework_value::ValueError"
VK = "semio_framework_value::ValueRefusalKind"


STAGED: dict = {}


def edit(path: Path, pairs, regex=False):
    text = STAGED.get(path) or path.read_text()
    for old, new, count in pairs:
        found = len(re.findall(old, text)) if regex else text.count(old)
        if found != count:
            sys.exit(f"{path}: expected {count} of {old!r}, found {found}")
        text = re.sub(old, new, text) if regex else text.replace(old, new)
    STAGED[path] = text


io_error = (r"IoError \{ message: (.+?), diagnostics: Vec::new\(\) \}", rf"IoError::from_value_error({VE}::new({VK}::InvalidValue, \1))")
de = S / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts"
for leaf, count in [("📝️md/🔖️commonmark/✳️any", 2), ("🔣️json/🔖️rfc8259/✳️any", 3), ("📜️docx/🔖️ecma-376/✳️any", 3), ("📖️pdf/🔖️1.4/🧱️base", 2), ("🔤️txt/🔖️utf-8/✳️any", 2)]:
    edit(de / leaf / "🦀️.rs", [(io_error[0], io_error[1], count)], regex=True)
edit(S / "🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs", [(io_error[0], io_error[1], 1)], regex=True)

se = S / "🚪️io/📤️export/🧵️serializers/🗿️artifacts"
for leaf, name, arg in [("🔣️json/🔖️rfc8259/✳️any", "WriterIntoJson", "original"), ("🔤️txt/🔖️utf-8/✳️any", "WriterIntoTxt", "snapshot"), ("📖️pdf/🔖️1.4/🧱️base", "WriterIntoPdf", "snapshot"), ("📜️docx/🔖️ecma-376/✳️any", "WriterIntoDocx", "snapshot"), ("📝️md/🔖️commonmark/✳️any", "WriterIntoMd", "snapshot")]:
    edit(se / leaf / "🧪️tests/🔬️unit/🦀️.rs", [(f"{name}::serialize(&{arg})", f"{name}::serialize(&{arg}, &semio_framework::io::io_mechanism::ArchiveChildren::empty())", 1)])

binary = S / "🚪️io/🧬️mutations/💾️binary/🦀️.rs"
edit(binary, [
    ("fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {", f"fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, {VE}> {{", 3),
    ('Err("Writer snapshot root retirement reported Complete without terminal-empty authority".into())', f'Err({VE}::new({VK}::InvariantViolated, "Writer snapshot root retirement reported Complete without terminal-empty authority"))', 1),
    ("fn pump_active(&mut self) -> Result<bool, String> {", f"fn pump_active(&mut self) -> Result<bool, {VE}> {{", 1),
    ("fn pump_terminal_retirement(&mut self) -> Result<bool, String> {", f"fn pump_terminal_retirement(&mut self) -> Result<bool, {VE}> {{", 1),
    ('Err("Writer store initializer retirement exceeded its exact grant".into())', f'Err({VE}::new({VK}::InvariantViolated, "Writer store initializer retirement exceeded its exact grant"))', 1),
    ('Err("Writer store initializer retirement reported a false terminal".into())', f'Err({VE}::new({VK}::InvariantViolated, "Writer store initializer retirement reported a false terminal"))', 1),
    ('Err("Writer initialization runtime reported a false terminal".into())', f'Err({VE}::new({VK}::InvariantViolated, "Writer initialization runtime reported a false terminal"))', 1),
    ('Err("Writer initialization envelope retirement reported a false terminal".into())', f'Err({VE}::new({VK}::InvariantViolated, "Writer initialization envelope retirement reported a false terminal"))', 1),
    ("if let Err(error) = self.pump_active() {\n            self.fault = Some(error.into_bytes());", "if let Err(error) = self.pump_active() {\n            self.fault = Some(error.into_message().into_bytes());", 1),
    ("Err(error) => {\n                    self.fault = Some(error.into_bytes());\n                    semio_framework_job::StepOutcome::Yield", "Err(error) => {\n                    self.fault = Some(error.into_message().into_bytes());\n                    semio_framework_job::StepOutcome::Yield", 1),
])

edit(S / "✏️editor/🦀️.rs", [
    ("fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {", f"fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, {VE}> {{", 1),
    ('return Err("Writer Artifact preparation could not return its exact base root".into());', f'return Err({VE}::new({VK}::InvariantViolated, "Writer Artifact preparation could not return its exact base root"));', 1),
])
edit(S / "✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🫧️transient/📢️publication/🦀️.rs", [
    ("fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String> {", f"fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, {VE}> {{", 1),
])
edit(S / "🧬️schema/💡️inferences/🦀️.rs", [("Vec<dsl::JsonValue>", "Vec<semio_framework_pack_json::Value>", 1)])
edit(S / "🧬️schema/🦀️.rs", [
    ("semio_framework_diagnostic::TextError::new(e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1))", f"semio_framework_diagnostic::TextError::new({VK}::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1))", 1),
])
edit(S / "✏️editor/🎮️commands/📝️text-edit/🧪️tests/🔬️unit/🦀️.rs", [("dsl::DslValue::String", "semio_framework_value::DslValue::String", 1)])

languages = W / "🤖️generated/🗣️writer-languages/🦀️.rs"
edit(languages, [
    ("Self::parse(&id).map_err(semio_framework_os_kernel::ValueError::new)", f"Self::parse(&id).map_err(|message| {VE}::new({VK}::InvalidValue, message))", 1),
    ('Err(semio_framework_os_kernel::ValueError::new(format!("expected WriterLanguagesLanguageKind string, found {other:?}")))', f'Err({VE}::new({VK}::InvalidValue, format!("expected WriterLanguagesLanguageKind string, found {{other:?}}")))', 1),
    ("semio_framework_os_kernel::", "semio_framework_value::", 14),
])
if "--check" not in sys.argv:
    for path, text in STAGED.items():
        path.write_text(text)
print(f"writer value sweep: {len(STAGED)} files {'checked' if '--check' in sys.argv else 'written'}")
