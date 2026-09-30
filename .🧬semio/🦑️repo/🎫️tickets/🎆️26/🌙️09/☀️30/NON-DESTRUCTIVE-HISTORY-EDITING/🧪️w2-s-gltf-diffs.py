#!/usr/bin/env python3
"""🔺️ W2-S glTF: replaces every committed `🔺️diff/🔣️.json` of an applied case whose content is not the `GltfDiff` the
production mutation produces (legacy per-leaf shapes, pre-`json_presence` JSON slots) with that produced diff.

Input: the `[DEBUG] <case>\t<message count>\t<diff json>` lines the temporary `debug_dump_produced_diffs` test printed
(`cargo test -p semio-s-artifact-stdio-gltf --lib -- fixture_corpus --nocapture`), passed as the first argument. Only
cases the committed `🎯️outcome` declares `applied` and whose production outcome carries no message are written; a
committed diff that already equals the produced one as JSON (numbers by value) is left untouched.

Run from the repository root: `.venv/bin/python <ticket>/🧪️w2-s-gltf-diffs.py <dump.txt> [--write]`.
"""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
FIXTURES = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧫️fixtures/🧬️mutations"
WRITE = "--write" in sys.argv

written = kept = skipped = 0
for line in Path(sys.argv[1]).read_text().splitlines():
    if not line.startswith("[DEBUG] "):
        continue
    name, messages, produced = line[len("[DEBUG] "):].split("\t", 2)
    case = FIXTURES / name
    if json.loads((case / "🎯️outcome/🔣️.json").read_text())["status"] != "applied" or messages != "0":
        skipped += 1
        continue
    target = case / "🔺️diff/🔣️.json"
    diff = json.loads(produced)
    if target.exists() and json.loads(target.read_text()) == diff:
        kept += 1
        continue
    written += 1
    print(("write " if WRITE else "would write ") + name)
    if WRITE:
        target.write_text(json.dumps(diff, indent=2, ensure_ascii=False) + "\n")
print(f"{written} rewritten, {kept} already the produced diff, {skipped} not applied")
