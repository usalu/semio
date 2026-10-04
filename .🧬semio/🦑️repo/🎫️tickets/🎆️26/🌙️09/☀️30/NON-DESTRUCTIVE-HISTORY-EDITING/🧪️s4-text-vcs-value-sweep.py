#!/usr/bin/env python3
"""🌿️ S4-TEXT (session 4): moves the vcs io (de)serializers off the removed `IoError { message }` shape onto the owned refusal
(`IoError::from_value_error(ValueError::new(InvalidValue, …))`), peer value/io extraction fallout. Every replacement asserts its exact
count; all files are staged, then written together (`--check` = dry run). Run from the repo root."""
import re
import sys
from pathlib import Path

S = Path("✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io")
OLD = r"IoError \{ message: (.+?), diagnostics: Vec::new\(\) \}"
NEW = r"IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, \1))"
FILES = {
    "📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs": 2,
    "📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs": 1,
    "📥️import/🧩️deserializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️.rs": 1,
    "📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs": 1,
    "📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs": 1,
    "📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs": 1,
}
staged = {}
for rel, count in FILES.items():
    path = S / rel
    text = path.read_text()
    found = len(re.findall(OLD, text))
    if found == 0 and "IoError::from_value_error" in text:
        continue
    if found != count:
        sys.exit(f"{path}: expected {count} IoError {{ message }} sites, found {found}")
    staged[path] = re.sub(OLD, NEW, text)
V = S.parent
VE, VK = "semio_framework_value::ValueError", "semio_framework_value::ValueRefusalKind"
EXACT = {
    V / "✏️editor/🦀️.rs": [(f'return Err({VE}::new({VK}::InvariantViolated,"VCS one-item preparation rejected its lane or description envelope"));', 'return Err("VCS one-item preparation rejected its lane or description envelope".into());', 1)],
    V / "✏️editor/🧪️tests/🔬️unit/🦀️.rs": [("fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {", f"fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, {VE}> {{", 1)],
    S / "🧪️tests/🔬️unit/🦀️.rs": [(f"{name}::serialize(&sample())", f"{name}::serialize(&sample(), &semio_framework::io::io_mechanism::ArchiveChildren::empty())", 1) for name in ("VcsIntoCsv", "VcsIntoXlsx", "VcsIntoZip")],
    V / "🧬️schema/⚙️operations/🧪️tests/🔬️unit/🦀️.rs": [("VcsIntoTxt::serialize(&snapshot)", "VcsIntoTxt::serialize(&snapshot, &semio_framework::io::io_mechanism::ArchiveChildren::empty())", 1)],
    V / "🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs": [("&mut dsl::NativeDecodeControl::new(", "&mut semio_framework_value::NativeDecodeControl::new(", 1)],
}
for path, pairs in EXACT.items():
    text = staged.get(path) or path.read_text()
    for old, new, count in pairs:
        if text.count(old) != count:
            sys.exit(f"{path}: expected {count} of {old!r}, found {text.count(old)}")
        text = text.replace(old, new)
    staged[path] = text
if "--check" not in sys.argv:
    for path, text in staged.items():
        path.write_text(text)
print(f"vcs value sweep: {len(staged)} files {'checked' if '--check' in sys.argv else 'written'}")
