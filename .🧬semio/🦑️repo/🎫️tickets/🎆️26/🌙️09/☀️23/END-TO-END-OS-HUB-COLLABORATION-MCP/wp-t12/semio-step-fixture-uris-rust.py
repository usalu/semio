#!/usr/bin/env python3
"""🧫️ Rust twin of `semio-step-fixture-uris.py`: the Rust runner Context answers every fixture URI a scenario's steps
name (`step_fixture_uris`, the platform's one grammar), and the eight `🧿️semio` Rust adapters that scanned for
`asset://` (the features moved their vectors to `shared://`) use it. Usage: semio-step-fixture-uris-rust.py [--write]"""
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
RUNNER = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏃️runner/🦀️.rs"
SUBSETS = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
write = "--write" in sys.argv
edits, problems = {}, []


def text(path):
    return edits.get(path) or path.read_text(encoding="utf-8")


def replace(path, old, new, count=1):
    source = text(path)
    if source.count(old) != count:
        problems.append(f"{path.relative_to(ROOT)}: expected {count}× {old[:70]!r}, found {source.count(old)}")
        return
    edits[path] = source.replace(old, new)


def drop(path, pattern):
    source = text(path)
    match = re.search(pattern, source, re.S | re.M)
    if match is None:
        problems.append(f"{path.relative_to(ROOT)}: /{pattern[:50]}/ not found")
        return
    edits[path] = source[: match.start()] + source[match.end():]


replace(RUNNER, "    /// 🧫️ Bytes of a resolved fixture.\n", '''    /// 🔗️ Every fixture URI the scenario's steps name, in step order and whatever scheme the feature uses — the
    /// platform's one fixture-URI grammar (`FIXTURE_URI_RE`, `🧪️test/🟦️.ts`). The feature is the single place a
    /// vector path is written down.
    pub fn step_fixture_uris(&self) -> Vec<String> {
        self.scenario.steps.iter().flat_map(|(_, text)| fixture_uris_in(text)).collect()
    }

    /// 🧫️ Bytes of a resolved fixture.
''')
replace(RUNNER, "//#region 🔖️Adapter\n", '''//#region 🔖️Adapter
/// 🔗️ The fixture URIs one line of feature text names: `shared://`, `local://`, `asset://` or `schema://` at a word
/// start, up to whitespace or one of `"'`,;)]` — `FIXTURE_URI_RE` of `🧪️test/🟦️.ts`.
pub fn fixture_uris_in(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut at = 0;
    while at < text.len() {
        let rest = &text[at..];
        let Some((offset, scheme)) = ["shared://", "local://", "asset://", "schema://"].iter().filter_map(|scheme| rest.find(scheme).map(|offset| (offset, *scheme))).min() else { break };
        let start = at + offset;
        let word_start = text[..start].chars().next_back().is_none_or(|previous| !(previous.is_alphanumeric() || previous == '_'));
        let body = &text[start + scheme.len()..];
        let length = body.find(|character: char| character.is_whitespace() || "\\"'`,;)]".contains(character)).unwrap_or(body.len());
        if word_start && length > 0 {
            found.push(text[start..start + scheme.len() + length].to_string());
        }
        at = start + scheme.len() + length;
    }
    found
}

''')

for case in ["🧰️kit/🧪️tests/🧰️mutate-semio-kit", "📦️object/🧪️tests/📦️mutate-semio-object", "🕸️graph/🧪️tests/🌳️mutate-semio-graph", "🧊️brep/🧪️tests/🧊️mutate-semio-brep"]:
    path = SUBSETS / case / "🦀️.rs"
    drop(path, r"^/// 🧫️ Every `asset://` URI.*?\n#\[cfg\(feature = \"sut\"\)\]\nfn step_assets\(ctx: &Context\) -> Vec<String> \{\n.*?^\}\n\n")
    replace(path, "let assets = super::step_assets(ctx);", "let assets = ctx.step_fixture_uris();")
for case in ["📊️table/🧪️tests/📊️mutate-semio-table", "🔤️text/🧪️tests/🔤️mutate-semio-text"]:
    path = SUBSETS / case / "🦀️.rs"
    drop(path, r"^    /// 🧫️ Every `asset://` URI.*?\n    fn step_assets\(ctx: &Context\) -> Vec<String> \{\n.*?^    \}\n\n")
    replace(path, "step_assets(ctx).into_iter().nth(position)", "ctx.step_fixture_uris().into_iter().nth(position)")
for case in ["🖊️drawing/🧪️tests/🖊️mutate-semio-drawing", "🖼️image/🧪️tests/🖼️mutate-semio-image"]:
    path = SUBSETS / case / "🦀️.rs"
    source = text(path)
    match = re.search(r"^(    )fn step_uris\(ctx: &Context, scheme: &str\) -> Vec<String> \{\n.*?^    \}\n", source, re.S | re.M)
    if match is None:
        problems.append(f"{case}: step_uris not found")
        continue
    edits[path] = source[: match.start()] + "    fn step_uris(ctx: &Context, prefix: &str) -> Vec<String> {\n        ctx.step_fixture_uris().into_iter().filter(|uri| uri.starts_with(prefix)).collect()\n    }\n" + source[match.end():]
    replace(path, 'step_uris(ctx, "asset://")', "ctx.step_fixture_uris()")

for path, source in edits.items():
    if path != RUNNER and "asset://` URI" in source:
        edits[path] = source.replace("the specification vectors from the `asset://` URIs its steps name", "the specification vectors from the fixture URIs its steps name")
for path in edits:
    print(("write " if write and not problems else "plan  ") + str(path.relative_to(ROOT))[-80:])
print(f"files={len(edits)} problems={len(problems)}")
for problem in problems:
    print("PROBLEM", problem)
if write and not problems:
    for path, source in edits.items():
        path.write_text(source, encoding="utf-8")
