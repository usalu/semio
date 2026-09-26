"""🧩️ D1 one-off codemod helper: inserts agent-facing description chain steps into one app's builder chain.

`describe(path, fn, rows, audiences)` finds `fn <fn>(` in `path`, then the first `.build_definition()` after it,
and inserts one `.action_describe(id, LocalizedLabel::native(en, de))` step per row (plus
`.action_audience(id, CapabilityAudience::X)` per audience row) directly before it, at the chain's indentation.
Refuses: missing anchor, a row whose id already has an `action_describe` in that function, quotes/backslashes/
newlines in text, en == de. Idempotent refusal, never a silent partial edit."""
import os, re, sys

ROOT = "/Users/ueli/Documents/semio"


def rust_str(text):
    if '"' in text or "\\" in text or "\n" in text:
        raise SystemExit(f"refused text with quote/backslash/newline: {text}")
    return f'"{text}"'


def describe(path, fn, rows, audiences=(), sdk="semio_framework_plugin", awaited=False, anchor=".build_definition()", destructive=(), statement=None):
    full = path if path.startswith("/") else os.path.join(ROOT, path)
    source = open(full).read()
    start = source.find(f"fn {fn}(")
    if start < 0:
        raise SystemExit(f"{fn} not found in {path}")
    at = source.find(anchor, start)
    while at >= 0 and source[source.rfind("\n", 0, at) + 1:at].strip():
        at = source.find(anchor, at + 1)
    if at < 0:
        raise SystemExit(f"{anchor} not found after {fn} in {path}")
    body = source[start:at]
    line_start = source.rfind("\n", 0, at) + 1
    indent = source[line_start:at]
    if statement:
        indent = indent[: len(indent) - len(indent.lstrip())]
        at = line_start + len(indent)
    if indent.strip():
        raise SystemExit(f"{anchor} is not at the start of its line in {fn}")
    suffix = ".await" if awaited else ""
    label = "LocalizedLabel" if re.search(r"use [^;]*\bLocalizedLabel\b[^;]*;", source) else f"{sdk}::LocalizedLabel"
    steps = []
    for vid, en, de in rows:
        if f'.action_describe("{vid}"' in body:
            raise SystemExit(f"{fn}: {vid} already described")
        if en.strip() == de.strip():
            raise SystemExit(f"{vid}: en == de")
        steps.append(f'{indent}.action_describe({rust_str(vid)}, {label}::native({rust_str(en)}, {rust_str(de)})){suffix}\n')
    for vid, audience in audiences:
        steps.append(f'{indent}.action_audience({rust_str(vid)}, {sdk}::CapabilityAudience::{audience}){suffix}\n')
    for vid in destructive:
        steps.append(f'{indent}.action_destructive({rust_str(vid)}){suffix}\n')
    if statement:
        steps = [f"{indent}{statement} = {statement}{step.strip()};\n" for step in steps]
    source = source[:line_start] + "".join(steps) + source[line_start:]
    if os.environ.get("D1_DRY_RUN"):
        print(f"[dry-run] {path}::{fn}: would insert {len(steps)} step(s) before line {source.count(chr(10), 0, line_start) + 1}")
        return
    open(full, "w").write(source)
    print(f"{path}::{fn}: +{len(rows)} descriptions, +{len(audiences)} audiences, +{len(destructive)} destructive")
