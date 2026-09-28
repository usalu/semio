#!/usr/bin/env python3
"""🎯️ S19 one-off codemod (set `norm-args`): the fifteen norm editors declare the arguments their value-tree verbs
actually read, with the types the handlers decode (`app_surface::{path_arg, index_arg, value_arg_json, check_id_arg}`).
Measured: `setField`/`insertItem`/`removeItem`/`applyRemedy` published `path`/`checkId` as OPTIONAL text and
`index`/`remedyIndex` as TEXT, while the handlers need a path and read the ordinals with `as_u64` — an agent following
the published input schema sent nothing (MCP `INTERNAL "path must not be empty"`, 15 kinds) or a string ordinal (read as
0); `insertItem`'s `value` was read but never declared, so an agent could only insert `null`. Now: `path`/`checkId`
required text, `index`/`remedyIndex` required non-negative integer ordinals, `setField.value` a required typed value,
`insertItem.value` an optional typed value, `setSnapshot.snapshot` required JSON text; plus the law
`🧪️tests/🎯️action-args` over all fifteen editors. Idempotent. usage: s19-norm-args.py <root>"""
import glob
import os
import re
import sys

root = sys.argv[1]
NORM = "✏️s/🔌️plugins/📕️norm"
P = r"((?:semio_framework_plugin::)?ActionArgDef::)"


def rewrite(text):
    text = re.sub(P + r'text\("path", (LocalizedLabel::native\("Path", "Pfad"\))\)(?!\.required\(\))', r'\1text("path", \2).required()', text)
    text = re.sub(P + r'text\("checkId", (LocalizedLabel::native\("Check", "Nachweis"\))\)(?!\.required\(\))', r'\1text("checkId", \2).required()', text)
    text = re.sub(P + r'text\("index", (LocalizedLabel::native\("Index", "Index"\))\)', r'\1index("index", \2).required()', text)
    text = re.sub(P + r'text\("remedyIndex", (LocalizedLabel::native\("Remedy", "Abhilfe"\))\)', r'\1index("remedyIndex", \2).required()', text)
    text = re.sub(P + r'text\("value", (LocalizedLabel::native\("Value", "Wert"\))\)', r'\1any("value", \2).required()', text)
    text = re.sub(P + r'text\("snapshot", (LocalizedLabel::native\("Document JSON", "Dokument-JSON"\))\)', r'\1json_text("snapshot", \2).required()', text)
    insert = re.compile(r'(new_catalog\("insertItem",[^\n]*\n\s*\.with_args\(vec!\[\n(\s*)' + P + r'text\("path"[^\n]*\n\s*' + P + r'index\("index"[^\n]*\n)(?!\s*' + P + r'any\("value")')
    text = insert.sub(lambda m: m.group(1) + f'{m.group(2)}{m.group(3)}any("value", LocalizedLabel::native("Value", "Wert")),\n', text)
    return text


editors = sorted(glob.glob(os.path.join(root, NORM, "🗿️artifacts/*/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs")))
assert len(editors) == 15, len(editors)
for path in editors:
    before = open(path, encoding="utf-8").read()
    after = rewrite(before)
    for needle, count in (('text("path", LocalizedLabel::native("Path", "Pfad")).required()', 3), ('.required()', 9), ('index("index"', 2), ('index("remedyIndex"', 1), ('any("value"', 2), ('json_text("snapshot"', 1)):
        assert after.count(needle) == count, f"{os.path.relpath(path, root)}: {needle} x{after.count(needle)} (want {count})"
    if after != before:
        open(path, "w", encoding="utf-8").write(after)
        print(f"edited {os.path.relpath(path, root)}")

FAMILIES = ["din4108", "din16798", "din18599", "en1990", "en1991", "en1992", "en1993", "en1994", "en1995", "en1996", "en1997", "en1998", "en1999", "iso16757", "vdi3805"]
LAW = '''//! 🎯️ Every norm editor publishes the arguments its value-tree verbs read, typed as the handlers decode them
//! (`app_surface::{path_arg, index_arg, value_arg_json, check_id_arg}`): an agent that follows the published input schema
//! must be able to call `setField`/`insertItem`/`removeItem`/`applyRemedy`/`setSnapshot` (measured over MCP: optional
//! text `path` → `INTERNAL "path must not be empty"` on all fifteen kinds; text `index` read as 0; `insertItem.value`
//! undeclared). Ticket 26/09/23 slice S19.
use semio_framework_plugin::{ActionArgDef, AppDefinition, ArgSchema};

fn argument<'a>(definition: &'a AppDefinition, action: &str, argument: &str) -> &'a ActionArgDef {
    let declared = definition.actions.iter().find(|candidate| candidate.id == action).unwrap_or_else(|| panic!("{} declares no {action}", definition.id));
    declared.args.iter().find(|candidate| candidate.id == argument).unwrap_or_else(|| panic!("{} {action} declares no {argument}", definition.id))
}

fn assert_text(definition: &AppDefinition, action: &str, name: &str, required: bool) {
    let declared = argument(definition, action, name);
    assert!(matches!(declared.schema, ArgSchema::String { .. }), "{} {action}.{name} must be text", definition.id);
    assert_eq!(declared.required, required, "{} {action}.{name} required", definition.id);
}

fn assert_ordinal(definition: &AppDefinition, action: &str, name: &str) {
    let declared = argument(definition, action, name);
    assert!(matches!(declared.schema, ArgSchema::Number { integer: true, min: Some(min), .. } if min == 0.0), "{} {action}.{name} must be a non-negative integer", definition.id);
    assert!(declared.required, "{} {action}.{name} must be required", definition.id);
}

fn assert_value(definition: &AppDefinition, action: &str, required: bool) {
    let declared = argument(definition, action, "value");
    assert!(matches!(declared.schema, ArgSchema::Any), "{} {action}.value must be a typed value", definition.id);
    assert_eq!(declared.required, required, "{} {action}.value required", definition.id);
}

fn assert_norm_value_tree_arguments(definition: AppDefinition) {
    assert_text(&definition, "setField", "path", true);
    assert_value(&definition, "setField", true);
    assert_text(&definition, "insertItem", "path", true);
    assert_ordinal(&definition, "insertItem", "index");
    assert_value(&definition, "insertItem", false);
    assert_text(&definition, "removeItem", "path", true);
    assert_ordinal(&definition, "removeItem", "index");
    assert_text(&definition, "applyRemedy", "checkId", true);
    assert_ordinal(&definition, "applyRemedy", "remedyIndex");
    assert_text(&definition, "setSnapshot", "snapshot", true);
}

''' + "".join(f'''#[test]
fn {family}_declares_its_value_tree_arguments() {{
    assert_norm_value_tree_arguments(semio_s_artifact_norm_{family}::editor::{family}::create_{family}_app());
}}

''' for family in FAMILIES).rstrip("\n") + "\n"
law_path = os.path.join(root, NORM, "🧪️tests/🎯️action-args/🦀️.rs")
os.makedirs(os.path.dirname(law_path), exist_ok=True)
if not os.path.exists(law_path) or open(law_path, encoding="utf-8").read() != LAW:
    open(law_path, "w", encoding="utf-8").write(LAW)
    print(f"wrote {os.path.relpath(law_path, root)}")

plugin = os.path.join(root, NORM, "🦀️.rs")
text = open(plugin, encoding="utf-8").read()
anchor = '#[cfg(test)]\n#[path = "🧪️tests/🔬️surface/🦀️.rs"]\nmod surface_tests;\n'
addition = '#[cfg(test)]\n#[path = "🧪️tests/🎯️action-args/🦀️.rs"]\nmod action_args_tests;\n'
if addition not in text:
    assert text.count(anchor) == 1
    open(plugin, "w", encoding="utf-8").write(text.replace(anchor, anchor + addition))
    print(f"edited {os.path.relpath(plugin, root)}")
