#!/usr/bin/env python3
"""🧾️ T13 item 5b, phase A + B1 (T14): a first-party `dsl_value!` literal macro replaces `serde_json::json!` +
`optional_json_to_dsl` at action-argument sites.

Phase A — the macro in the value module (`🌱️value/🦀️.rs`, `#[macro_export]`, crate `protocol`), re-exported by the kernel
root and `semio_framework` (and so by `semio_framework_plugin`'s glob), plus the law in the value module's unit tests:
`dsl_value!` equals `serde_json::json!` (third-party oracle) on every literal form and keeps written entry order.
Phase B1 — every `optional_json_to_dsl(Some(json!(L)))` / `(Some(serde_json::json!(L)))` becomes `Some(<path>dsl_value!(L))`;
a `use serde_json::json;` left without a `json!` use is dropped. Every other `optional_json_to_dsl` site (wrapper parameters,
programmatic maps, fixture-driven tests, the renderer's 16) is REPORTED, not rewritten — phase B2 changes their types.

usage: dsl-value.py [--write] [--root <tree>]   (default: dry run on the live tree)"""
import re
import subprocess
import sys
from pathlib import Path

args = sys.argv[1:]
LIVE = Path("/Users/ueli/Documents/semio")
ROOT = Path(args[args.index("--root") + 1]) if "--root" in args else LIVE
WRITE = "--write" in args
HERE = Path(__file__).resolve().parent
problems, edits, manual = [], {}, []


def text_of(path):
    return edits.get(path) or path.read_text(encoding="utf-8")


def exact(path, old, new):
    text = text_of(path)
    if new.strip() and new in text and old not in text:
        return
    if text.count(old) != 1:
        problems.append(f"{path.relative_to(ROOT)}: anchor found {text.count(old)} times: {old.strip()[:80]!r}")
        return
    edits[path] = text.replace(old, new)


VALUE = ROOT / "🧰️framework/🔨️modules/🌱️value/🦀️.rs"
MACRO = (HERE / "macro.rs.txt").read_text(encoding="utf-8")
if "macro_rules! dsl_value" not in text_of(VALUE):
    exact(VALUE, "//#region 🔖️SerDe\n", MACRO + "//#region 🔖️SerDe\n")
LAW = ROOT / "🧰️framework/🔨️modules/🌱️value/🧪️tests/🔬️unit/🦀️.rs"
if "dsl_value_literal_matches_serde_json_and_keeps_written_order" not in text_of(LAW):
    edits[LAW] = text_of(LAW).rstrip("\n") + '''

/// 🧾️ `dsl_value!` equals `serde_json::json!` (third-party oracle) on every literal form — null, booleans, signed, unsigned
/// and fractional numbers, nested arrays and objects, `const` and parenthesized expression keys, trailing commas — and,
/// unlike `json!` without `preserve_order`, keeps object entries in written order.
#[test]
fn dsl_value_literal_matches_serde_json_and_keeps_written_order() {
    const KEY: &str = "constKey";
    let count = 3u32;
    let names = vec![String::from("a"), String::from("b")];
    let flag = false;
    let literal = crate::dsl_value!({ "zeta": null, "alpha": [1, -2, 2.5, true, false, null, [], {}], KEY: count, ("expr".to_string()): !flag, "nested": { "names": names, "text": "ä€😀" }, });
    let oracle = serde_json::json!({ "zeta": null, "alpha": [1, -2, 2.5, true, false, null, [], {}], KEY: count, ("expr".to_string()): !flag, "nested": { "names": names, "text": "ä€😀" }, });
    assert_eq!(serde_json::Value::from(&literal), oracle);
    let DslValue::Object(entries) = &literal else { panic!("an object literal builds an object") };
    assert_eq!(entries.iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>(), ["zeta", "alpha", "constKey", "expr", "nested"]);
    assert_eq!(crate::dsl_value!(7u64), DslValue::uint(7));
    assert_eq!(crate::dsl_value!([]), DslValue::Array(Vec::new()));
    assert_eq!(crate::dsl_value!({}), DslValue::Object(Vec::new()));
    assert_eq!(serde_json::Value::from(&crate::dsl_value!([[1, [2]], { "k": [3] }])), serde_json::json!([[1, [2]], { "k": [3] }]));
}
'''
KERNEL = ROOT / "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs"
exact(KERNEL, "pub use crate::os_dsl::schema::{DslValue, FromValue, ToValue, ValueEdit, ValueError, ValueShape};\n",
      "pub use crate::os_dsl::schema::{DslValue, FromValue, ToValue, ValueEdit, ValueError, ValueShape};\npub use protocol::dsl_value;\n")
FRAMEWORK = ROOT / "🧰️framework/📦️packages/🦀️rust/🦀️.rs"
exact(FRAMEWORK, "pub use dsl::{from_dsl_value, to_dsl_value, DslValue};\n", "pub use dsl::{dsl_value, from_dsl_value, to_dsl_value, DslValue};\n")

SITE = re.compile(r"((?:semio_framework_plugin|semio_framework)::)?optional_json_to_dsl\(")
listing = subprocess.run(["git", "grep", "-l", "optional_json_to_dsl", "--", "*.rs"], cwd=LIVE, capture_output=True, text=True).stdout.split()
rewritten = 0
for rel in listing:
    if "🎫️tickets" in rel:
        continue
    path = ROOT / rel
    text = text_of(path)
    out, cursor = [], 0
    for match in SITE.finditer(text):
        if match.start() < cursor:
            continue
        depth, index = 1, match.end()
        while depth and index < len(text):
            depth += {"(": 1, ")": -1}.get(text[index], 0)
            index += 1
        argument = text[match.end():index - 1]
        literal = re.fullmatch(r"\s*Some\(\s*(?:serde_json::)?json!\((.*)\)\s*\)\s*", argument, re.S)
        line = text.count("\n", 0, match.start()) + 1
        if not literal:
            manual.append(f"{rel}:{line}: {argument.strip()[:110]}")
            continue
        prefix = match.group(1) or "semio_framework::"
        out.append(text[cursor:match.start()])
        out.append(f"Some({prefix}dsl_value!({literal.group(1)}))")
        cursor = index
        rewritten += 1
    if cursor:
        out.append(text[cursor:])
        updated = "".join(out)
        if "json!(" not in updated:
            updated = re.sub(r"^[ \t]*use serde_json::json;\n", "", updated, flags=re.M)
        if "optional_json_to_dsl" not in updated.replace("use semio_framework::optional_json_to_dsl;", ""):
            updated = updated.replace("use semio_framework::optional_json_to_dsl;\n", "")
        edits[path] = updated

for path in sorted(edits, key=str):
    print(("write " if WRITE else "dry-run ") + str(path.relative_to(ROOT)))
print(f"B1 literal sites rewritten: {rewritten}; sites left for phase B2: {len(manual)}")
for row in manual:
    print("  B2", row)
for problem in problems:
    print("problem:", problem)
print(f"{len(edits)} files, {len(problems)} problems")
if problems:
    sys.exit(1)
if WRITE:
    for path, text in edits.items():
        path.write_text(text, encoding="utf-8")
