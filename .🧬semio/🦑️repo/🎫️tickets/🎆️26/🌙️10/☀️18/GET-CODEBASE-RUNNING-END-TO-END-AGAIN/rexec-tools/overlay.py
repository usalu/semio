#!/usr/bin/env python3
"""Materializes the staged tree as an overlay of COPIES under run/overlay: every relative specifier of a staged .ts/.mjs file is re-pointed to the staged sibling when one exists and to the live tree otherwise; JSON and other files are copied verbatim. Also writes <adapter dir>/adapter-scratch.ts (overlay adapter + scratch host part)."""
import os, re, shutil, sys

ROOT = "/Users/ueli/Documents/semio"
C = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN")
STAGED = os.path.join(C, "rexec-tools/staged")
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(STAGED), "..")))
from winpath import iter_logical, resolve
OVERLAY = os.path.join(C, "rexec-tools/run/overlay")
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"
pattern = re.compile(r'(from\s*|import\()(["\'])(\.{1,2}/[^"\']+)\2')

shutil.rmtree(OVERLAY, ignore_errors=True)
staged_files = iter_logical(STAGED)
staged_set = set(staged_files)

def target_for(final_dir_rel, spec, out_dir):
    absolute = os.path.normpath(os.path.join(ROOT, final_dir_rel, spec))
    rel = os.path.relpath(absolute, ROOT)
    where = os.path.join(OVERLAY, rel) if rel in staged_set else absolute
    result = os.path.relpath(where, out_dir)
    return result if result.startswith(".") else "./" + result

for rel in sorted(staged_files):
    source = resolve(STAGED, rel)
    out = os.path.join(OVERLAY, rel)
    os.makedirs(os.path.dirname(out), exist_ok=True)
    if rel.endswith((".ts", ".mjs")):
        text = open(source, encoding="utf-8").read()
        text = pattern.sub(lambda m: m.group(1) + m.group(2) + target_for(os.path.dirname(rel), m.group(3), os.path.dirname(out)) + m.group(2), text)
        open(out, "w", encoding="utf-8").write(text)
    else:
        shutil.copyfile(source, out)

EXTRAS = [os.path.join(B, "🧪️tests", d, "🟦️.ts") for d in ["📤️arguments", "📥️actions", "📥️closure", "📥️control", "📥️delegation", "📥️environment", "📥️fingerprint", "📥️framing", "📥️inventory", "📥️purpose", "📥️sources", "📥️workspace", "📦️snapshot", "🔏️implementation", "🕸️dependencies"]]
for rel in EXTRAS:
    source = os.path.join(ROOT, rel)
    if not os.path.exists(source) or rel in staged_set:
        continue
    out = os.path.join(OVERLAY, rel)
    os.makedirs(os.path.dirname(out), exist_ok=True)
    text = open(source, encoding="utf-8").read()
    text = pattern.sub(lambda m: m.group(1) + m.group(2) + target_for(os.path.dirname(rel), m.group(3), os.path.dirname(out)) + m.group(2), text)
    open(out, "w", encoding="utf-8").write(text)

for rel in [os.path.join(B, "🧬️schema/🔣️.json")]:
    source = os.path.join(ROOT, rel)
    out = os.path.join(OVERLAY, rel)
    if os.path.exists(source) and not os.path.exists(out):
        os.makedirs(os.path.dirname(out), exist_ok=True)
        shutil.copyfile(source, out)

adapter_out = os.path.join(OVERLAY, B, "📜️script.ts")
if os.path.exists(adapter_out):
    text = open(adapter_out, encoding="utf-8").read()
    for old, new in [("async function writeBazelRuntimeV1(", "export async function writeBazelRuntimeV1("), ("async function bazelCancellationControl(", "export async function bazelCancellationControl(")]:
        if old in text and new not in text:
            text = text.replace(old, new)
    text += open(os.path.join(C, "rexec-tools/scratch.part.ts"), encoding="utf-8").read()
    open(os.path.join(OVERLAY, B, "adapter-scratch.ts"), "w", encoding="utf-8").write(text)
print(len(staged_files), "files ->", OVERLAY)
