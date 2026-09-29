#!/usr/bin/env python3
"""🧭️ LB2 p13 (window 3, framework value + stdio guests): every editor's snapshot is path-editable, and one OPC document has
one encoding.

- Path edits through hand-written value codecs: `FromValue::edit_value_at_path`'s default edits only the ROOT, so a snapshot whose
  codec is hand-written refused every `setSnapshotValue` below the root (`snapshot-edit.path-invalid … value has no child
  `metadata``) — pdf 1.7 (`PdfSnapshot`: defaulted lanes), semio `kit` and `object` (child/link handles). The value crate gains
  `edit_through_value` (edit the value tree the codec emits, decode it back so every invariant `from_value` checks holds for
  the edited document — what a derived `FromValue` does field by field), re-exported where `ToValue`/`FromValue` are
  (`pack::value`, `os_dsl::schema`, the kernel root); the three codecs route `edit_value_at_path` through it. Oracle:
  `serde_json` pointer edits (`🧪️tests/🔬️unit`).
- Deterministic OPC relationships: `OpcPackage::relationships` was a `HashMap` — per-instance iteration order, so the SAME
  xlsx/docx/pptx document encoded to different op bytes in two copies (`xlsx_editor`: `parse_op(print_op(m)).encode_op() !=
  m.encode_op()`, `"relationships": {"": …, "xl/workbook.xml": …}` vs the reverse) and relationship diffs listed owners in
  random order. It is a `BTreeMap` (canonical key order; the JSON object and its contracts are unchanged); the docx/xlsx/pptx
  relationship diff helpers take the same map, and docx's edit-preparation memory bound counts its entries (a B-tree has no
  `capacity`; the existing ×2 node allowance stays).

usage: python3 lb2-p13-value-paths-and-opc-order.py --dry-run | --write | --revert [--root <tree>]
Backups (byte-exact, per root) under `.🧬semio/🌐hub/s14-lb2-backup/p13/<root-hash>/`.
"""
import hashlib, os, re, shutil, sys

TREE = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "/Users/ueli/Documents/semio"
BACKUP = f"/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-backup/p13/{hashlib.sha256(TREE.encode()).hexdigest()[:12]}"
VALUE = "🧰️framework/🔨️modules/🌱️value"
VALUE_CODEC = f"{VALUE}/🔁️codec/🦀️.rs"
VALUE_ROOT = f"{VALUE}/🦀️.rs"
VALUE_TESTS = f"{VALUE}/🧪️tests/🔬️unit/🦀️.rs"
DSL_SCHEMA = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs"
KERNEL_ROOT = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs"
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
PDF17_SNAPSHOT = f"{ART}/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs"
SEMIO_SUBSETS = f"{ART}/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
SEMIO_KIT = f"{SEMIO_SUBSETS}/🧰️kit/🧬️schema/📸️snapshot/🦀️.rs"
SEMIO_OBJECT = f"{SEMIO_SUBSETS}/📦️object/🧬️schema/📸️snapshot/🦀️.rs"
OPC = f"{ART}/🎒️zip/📦️opc/🦀️.rs"
DOCX_PREPARATION = f"{ART}/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/📬️preparation/🦀️.rs"
OPC_DIFFS = [f"{ART}/{kind}/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs" for kind in ("📜️docx", "📕️xlsx", "📽️pptx")]
RELATIONSHIPS = "HashMap<String, Vec<OpcRelationship>>"

problems = []


def once(text, old, new, label):
    count = text.count(old)
    if count != 1:
        problems.append(f"{label}: expected 1 anchor, found {count}")
        return text
    return text.replace(old, new)


#region Value
def value_codec(text):
    return once(
        text,
        "    pub fn under(self, segment: impl std::fmt::Display) -> Self {\n        Self(format!(\"{segment}.{}\", self.0))\n    }\n}\n",
        "    pub fn under(self, segment: impl std::fmt::Display) -> Self {\n        Self(format!(\"{segment}.{}\", self.0))\n    }\n}\n"
        "\n"
        "/// 🧭️ Applies `edit` at `path` of a value whose codec is hand-written: through the value tree the codec emits, decoded back\n"
        "/// so every invariant its `from_value` checks holds for the edited value — the path edit a derived [`FromValue`] performs\n"
        "/// field by field. On failure `target` is unchanged. A hand-written `FromValue` routes its `edit_value_at_path` here.\n"
        "pub fn edit_through_value<T: ToValue + FromValue>(target: &mut T, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {\n"
        "    let mut value = target.to_value();\n"
        "    value.edit_value_at_path(path, edit)?;\n"
        "    *target = T::from_value(value)?;\n"
        "    Ok(())\n"
        "}\n",
        "value: edit_through_value",
    )


def value_root(text):
    return once(text, "pub use codec::{FromValue, ToValue, ValueEdit, ValueError, ValueShape};\n", "pub use codec::{edit_through_value, FromValue, ToValue, ValueEdit, ValueError, ValueShape};\n", "value: re-export")


def value_tests(text):
    return text + (
        "\n"
        "#[test]\n"
        "fn edit_through_value_matches_a_serde_json_pointer_edit_and_keeps_the_decode_invariant() {\n"
        "    #[derive(Debug, PartialEq)]\n"
        "    struct Placed {\n"
        "        name: String,\n"
        "        offset: DslValue,\n"
        "    }\n"
        "    impl ToValue for Placed {\n"
        "        fn to_value(&self) -> DslValue {\n"
        "            DslValue::object([(\"name\".to_string(), self.name.to_value()), (\"offset\".to_string(), self.offset.clone())])\n"
        "        }\n"
        "    }\n"
        "    impl FromValue for Placed {\n"
        "        fn from_value(value: DslValue) -> Result<Self, ValueError> {\n"
        "            let mut entries = value.into_object()?.into_iter();\n"
        "            let (Some((_, name)), Some((_, offset))) = (entries.next(), entries.next()) else { return Err(ValueError::new(\"two fields\")) };\n"
        "            let name = String::from_value(name)?;\n"
        "            if name.is_empty() {\n"
        "                return Err(ValueError::new(\"a placed value is named\"));\n"
        "            }\n"
        "            Ok(Self { name, offset })\n"
        "        }\n"
        "    }\n"
        "    let mut placed = Placed { name: \"saw\".into(), offset: DslValue::from(&serde_json::json!({\"x\": 1, \"y\": [2, 3]})) };\n"
        "    let mut oracle = serde_json::Value::from(&placed.to_value());\n"
        "    *oracle.pointer_mut(\"/offset/y/1\").unwrap() = serde_json::json!(5);\n"
        "    edit_through_value(&mut placed, &[\"offset\", \"y\", \"1\"], ValueEdit::Set(DslValue::from(&serde_json::json!(5)))).unwrap();\n"
        "    assert_eq!(serde_json::Value::from(&placed.to_value()), oracle);\n"
        "    let before = serde_json::Value::from(&placed.to_value());\n"
        "    assert!(edit_through_value(&mut placed, &[\"name\"], ValueEdit::Set(DslValue::from(&serde_json::json!(\"\")))).is_err());\n"
        "    assert!(edit_through_value(&mut placed, &[\"missing\", \"x\"], ValueEdit::Set(DslValue::from(&serde_json::json!(1)))).is_err());\n"
        "    assert_eq!(serde_json::Value::from(&placed.to_value()), before);\n"
        "}\n"
    )


def dsl_schema(text):
    return once(text, "pub use protocol::value::{from_dsl_value, ordered, to_dsl_value, DslValue,", "pub use protocol::value::{edit_through_value, from_dsl_value, ordered, to_dsl_value, DslValue,", "dsl: re-export")


def kernel_root(text):
    return once(text, "pub use crate::os_dsl::schema::{DslValue, FromValue, ToValue, ValueEdit, ValueError, ValueShape};\n", "pub use crate::os_dsl::schema::{edit_through_value, DslValue, FromValue, ToValue, ValueEdit, ValueError, ValueShape};\n", "kernel: re-export")


def hand_written(text, namespace, snapshot, label):
    head = f"impl {namespace}::FromValue for {snapshot} {{\n"
    method = (
        f"    fn edit_value_at_path(&mut self, path: &[&str], edit: {namespace}::ValueEdit) -> Result<(), {namespace}::ValueError> {{\n"
        f"        {namespace}::edit_through_value(self, path, edit)\n"
        "    }\n"
        "\n"
    )
    return once(text, head, head + method, f"{label}: path edits")


def pdf_snapshot(text):
    text = once(
        text,
        "/// 🔀️ First-party value decoding for the public PDF snapshot. `schema` remains required while\n/// every other field falls back to its default.\n",
        "/// 🔀️ First-party value decoding for the public PDF snapshot. `schema` remains required while\n/// every other field falls back to its default; a path edit goes through the value tree ([`pack::value::edit_through_value`]).\n",
        "pdf: doc",
    )
    return hand_written(text, "pack::value", "PdfSnapshot", "pdf")
#endregion Value


#region Opc
def opc(text):
    text = once(text, "use std::collections::{HashMap, HashSet};\n", "use std::collections::{BTreeMap, HashMap, HashSet};\n", "opc: import")
    text = once(
        text,
        "    /// 🗺️ Owner part path (`\"\"` = package root) -> that owner's relationships.\n    #[value(default)]\n    pub relationships: HashMap<String, Vec<OpcRelationship>>,\n",
        "    /// 🗺️ Owner part path (`\"\"` = package root) -> that owner's relationships, in owner order — one document has one\n    /// encoding (a hash map's per-instance order encoded the same package differently in two copies).\n    #[value(default)]\n    pub relationships: BTreeMap<String, Vec<OpcRelationship>>,\n",
        "opc: field",
    )
    return once(text, "    let mut relationships: HashMap<String, Vec<OpcRelationship>> = HashMap::new();\n", "    let mut relationships: BTreeMap<String, Vec<OpcRelationship>> = BTreeMap::new();\n", "opc: decode")


def opc_diff(text, label):
    text = once(text, "use std::collections::HashMap;\n", "use std::collections::BTreeMap;\n", f"{label}: import")
    if text.count(RELATIONSHIPS) != 4:
        problems.append(f"{label}: {text.count(RELATIONSHIPS)} relationship maps, expected 4")
    text = text.replace(RELATIONSHIPS, "BTreeMap<String, Vec<OpcRelationship>>")
    if re.search(r"\bHashMap\b", text.replace("// 🗺️ Relationships live in a `HashMap<owner, …>`", "")):
        problems.append(f"{label}: HashMap still used")
    return text.replace(
        "        // 🗺️ Relationships live in a `HashMap<owner, …>`, which HAS no order to transport, so this\n",
        "        // 🗺️ Relationships live in an owner-keyed `BTreeMap` whose order IS its key order, so this\n",
    )
def docx_preparation(text):
    return once(text, "    measure.add(snapshot.opc.relationships.capacity().saturating_mul(std::mem::size_of::<(String, Vec<semio_s_artifact_stdio_zip::opc::OpcRelationship>)>() * 2))?;\n", "    measure.add(snapshot.opc.relationships.len().saturating_mul(size_of::<(String, Vec<semio_s_artifact_stdio_zip::opc::OpcRelationship>)>() * 2))?;\n", "docx preparation: relationship map size")
#endregion Opc


def plan():
    edits = {
        VALUE_CODEC: value_codec,
        VALUE_ROOT: value_root,
        VALUE_TESTS: value_tests,
        DSL_SCHEMA: dsl_schema,
        KERNEL_ROOT: kernel_root,
        PDF17_SNAPSHOT: pdf_snapshot,
        SEMIO_KIT: lambda text: hand_written(text, "dsl", "SemioKitSnapshot", "semio kit"),
        SEMIO_OBJECT: lambda text: hand_written(text, "dsl", "SemioObjectSnapshot", "semio object"),
        OPC: opc,
        DOCX_PREPARATION: docx_preparation,
    }
    for path in OPC_DIFFS:
        edits[path] = lambda text, path=path: opc_diff(text, path.split("/")[4])
    return edits


def main():
    mode = next((flag for flag in ("--dry-run", "--write", "--revert") if flag in sys.argv), None)
    if mode is None:
        print(__doc__)
        sys.exit(2)
    edits = plan()
    if mode == "--revert":
        for path in edits:
            source = os.path.join(BACKUP, path)
            if os.path.isfile(source):
                shutil.copyfile(source, os.path.join(TREE, path))
                print("restored", path)
        return
    staged = {}
    for path, edit in edits.items():
        before = open(os.path.join(TREE, path), encoding="utf-8").read()
        after = edit(before)
        if after == before:
            problems.append(f"{path}: unchanged")
        staged[path] = (before, after)
    for problem in problems:
        print("PROBLEM", problem)
    print(f"{len(staged)} files, {len(problems)} problems")
    if mode == "--write" and not problems:
        for path, (before, after) in staged.items():
            backup = os.path.join(BACKUP, path)
            os.makedirs(os.path.dirname(backup), exist_ok=True)
            if not os.path.exists(backup):
                open(backup, "w", encoding="utf-8").write(before)
            open(os.path.join(TREE, path), "w", encoding="utf-8").write(after)
        print("written; backups under", BACKUP)
    sys.exit(1 if problems else 0)


if __name__ == "__main__":
    main()
