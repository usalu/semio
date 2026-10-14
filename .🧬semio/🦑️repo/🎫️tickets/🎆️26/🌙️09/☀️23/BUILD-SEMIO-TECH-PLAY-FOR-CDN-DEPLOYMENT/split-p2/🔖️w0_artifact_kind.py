#!/usr/bin/env python3
"""🔖️ Wave 0: the OS artifact kind `artifact_kind()` is the artifact's own identity, read by the editor manifest; it moves
from the crate root (now registration glue only) into the schema facet (lane P2)."""
import os
REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
ART = os.path.join(REPO, "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling")
ROOT = os.path.join(ART, "🦀️.rs")
SCHEMA = os.path.join(ART, "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs")
EDITOR = os.path.join(ART, "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs")
root = open(ROOT, encoding="utf-8").read()
a = root.index("//#region 🔖️ArtifactKind\n"); b = root.index("//#endregion 🔖️ArtifactKind\n") + len("//#endregion 🔖️ArtifactKind\n")
block = root[a:b]
root = root[:a] + root[b:]
root = root.replace("use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};\n", "")
open(ROOT, "w", encoding="utf-8").write(root)
block = block.replace("pub fn artifact_kind() -> ArtifactKindSpec {\n    ArtifactKindSpec {", "pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {\n    use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};\n    ArtifactKindSpec {")
schema = open(SCHEMA, encoding="utf-8").read()
anchor = "//#region 🔖️Ids\n"
assert anchor in schema
schema = schema.replace(anchor, block + "\n" + anchor, 1)
open(SCHEMA, "w", encoding="utf-8").write(schema)
editor = open(EDITOR, encoding="utf-8").read()
assert ".artifact_kind(crate::artifact_kind())" in editor
editor = editor.replace(".artifact_kind(crate::artifact_kind())", ".artifact_kind(crate::standards::v1::subsets::any::schema::artifact_kind())")
open(EDITOR, "w", encoding="utf-8").write(editor)
print("ok")
