import os, shutil
ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
D = ROOT + "/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🔣️json"
root = D + "/🦀️.rs"
child_dir = D + "/✅️json-validation"
child = child_dir + "/🦀️.rs"
assert not os.path.exists(child)
text = open(root, encoding="utf-8").read()
shutil.copy(root, F + "/lwr-tools/orig/layout-json.rs")
lines = text.split("\n")
derive = "#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]"
a = lines.index("enum JsonRootState {") - 1
assert lines[a] == derive
b = lines.index("enum TopArrayKind {") - 1
assert lines[b] == "#[derive(Clone, Copy, Debug)]" and lines[b - 1] == "" and lines[b - 2] == "}"
block = lines[a:b - 1]
assert block[-1] == "}" and block[0] == derive
assert lines.count("enum JsonRootState {") == 1 and lines.count("enum TopArrayKind {") == 1
imp = "use semio_framework_value_derive::{FromValue, ToValue};"
const_in = "const JSON_INPUT_BYTES_PER_UNIT: usize = 2_048;"
assert lines.count(imp) == 1 and lines.count(const_in) == 1
ci = lines.index(const_in)
assert lines[ci + 1] == ""
body = "\n".join(block)
assert "ToValue" in body and body.count(const_in) == 0
rest_after = "\n".join(lines[:a] + lines[b:])
for forbidden in ("JsonRootState", "JsonValidationCursor::", "JsonContainer", "JsonToken", "JsonLiteral", "JsonNumberState", "JsonStringEscape", "JsonArrayState", "JsonObjectState", "JSON_INPUT_BYTES_PER_UNIT"):
    assert forbidden not in rest_after.replace(const_in, ""), forbidden
assert "ToValue" not in rest_after.replace(imp, "") and "FromValue" not in rest_after.replace(imp, "")
mount_anchor = "pub mod dictionary;"
assert lines.count(mount_anchor) == 1
out = []
for i, l in enumerate(lines):
    if a <= i < b:
        continue
    if l == imp:
        continue
    if i == ci or i == ci + 1:
        continue
    out.append(l)
    if l == mount_anchor:
        out += ['#[path = "✅️json-validation/🦀️.rs"]', "mod json_validation;", "pub use json_validation::JsonValidationCursor;"]
head = [
    "//! ✅️ Layout export JSON validation: the bounded, resumable JSON validation cursor (see [`JsonValidationCursor`]) that checks the package fragments of the export job.",
    "",
    "use super::{MAX_LAYOUT_EXPORT_JSON_NODES, MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES, MAX_LAYOUT_EXPORT_STRING_BYTES};",
    imp,
    "",
    const_in,
    "",
]
os.makedirs(child_dir)
open(child, "w", encoding="utf-8").write("\n".join(head + block) + "\n")
open(root + ".lwr-tmp", "w", encoding="utf-8").write("\n".join(out))
os.replace(root + ".lwr-tmp", root)
print("ok", len(lines), len(out), len(block))
