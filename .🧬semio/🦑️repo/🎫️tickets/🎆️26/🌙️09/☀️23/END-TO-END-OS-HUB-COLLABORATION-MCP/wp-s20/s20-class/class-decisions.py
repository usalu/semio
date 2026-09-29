"""🗂️ S20 fault classes: the calls on the reviewers' ambiguous lists (📓️wp-fh5/6/7.md "Ambiguous → S20"), ONE call per pattern
applied across every family work list (class field only; the lists stay one entry per line):
- a code for work that is closing or being released (`*closing*`) → `unavailable` (the texts say "wait"; once released a new
  command runs — resolves the 31 puzzle `*-closing` retext candidates by class);
- a window or route no longer open (`*window-stale`, `*route-stale`) → `precondition-failed` (FH6 rule, everywhere);
- a stale or unknown host handle → `internal` (a host never reuses a handle after its cancel);
- a request naming another window than its dispatch window (`fem.window.mismatch`) → `input-invalid`;
- states that cannot occur within one revision / from immutable inputs (`drawing.selection.ancestry-changed`,
  `flow-duplicate-source-stale`, `cad.preview.invalid`) → `internal`.
Idempotent. Usage: python3 class-decisions.py"""
import json
import re
from pathlib import Path

LISTS = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-sets/class")
EXACT = {
    "artifact-envelope.ingress-handle": "internal", "load-stale-handle": "internal", "stale-handle": "internal", "plugin.command-driver-stale": "internal",
    "fem.window.mismatch": "input-invalid", "drawing.selection.ancestry-changed": "internal", "flow-duplicate-source-stale": "internal", "cad.preview.invalid": "internal",
}


def decide(code: str, current: str) -> str:
    if "closing" in code:
        return "unavailable"
    if re.search(r"(window|route)-stale$", code):
        return "precondition-failed"
    return EXACT.get(code, current)


for path in sorted(LISTS.glob("family-*.json")):
    entries = json.loads(path.read_text())
    changed = 0
    for entry in entries:
        decided = decide(entry["code"], entry["class"])
        if decided != entry["class"]:
            print(f"{path.stem} {entry['owner']} {entry['code']}: {entry['class']} → {decided}")
            entry["class"] = decided
            changed += 1
    path.write_text("[\n" + ",\n".join(json.dumps(entry, ensure_ascii=False) for entry in entries) + "\n]\n")
    print(f"{path.stem}: {changed} changed")
