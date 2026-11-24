"""🧾️ Writes the per-person part of the report of SH-2 from the measurements: failing checks before (`gate0`) and after (`final`), the proof lines (`record/proof.txt`), the line of the real test (`record/silhouette-test.txt`) and the number of repair ops with the hand-written note of what was changed (`notes.json`). `python3 report.py > <part.md>`"""
import json
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.normpath(os.path.join(HERE, "..", "..", "🗑️generated", "silhouette", "SH-2"))
WAVE = os.path.normpath(os.path.join(HERE, "..", "..", "..", "STAFFAGE-HUMAN-COLLECTION-WAVE", "🧰️scripts"))
notes = json.load(open(os.path.join(HERE, "notes.json"), encoding="utf8"))
people = [line.split("\t")[1] for line in open(os.path.join(HERE, "..", "SH-2.tsv"), encoding="utf8") if line.strip()]
proof = {line.split(" | ")[0]: line.strip() for line in open(os.path.join(HERE, "record", "proof.txt"), encoding="utf8") if " | " in line}
test = open(os.path.join(HERE, "record", "silhouette-test.txt"), encoding="utf8").read()


def failing(folder, slug):
    """📋️ The failing checks of a gate log, condensed."""
    out = []
    for line in open(os.path.join(GEN, folder, f"{slug}.log"), encoding="utf8"):
        match = re.match(rf"{re.escape(slug)} 1:(\d+) bbox (\S+) (\S+) (\S+)\s+elevation (\S+)\s+front (\S+)\s+plan (\S+)", line)
        if not match:
            continue
        scale, *cells = match.groups()
        parts = [f"{axis} {cell.rstrip('!')}" for axis, cell in zip(("widthM", "heightM", "depthM"), cells[:3]) if cell.endswith("!")]
        for view, cell in zip(("elevation", "front", "plan"), cells[3:]):
            iou, h = cell.split("/")
            if iou.endswith("!"):
                parts.append(f"{view} IoU {iou.rstrip('!')}")
            if h.endswith("!"):
                parts.append(f"{view} {h.rstrip('!')}")
        if parts:
            out.append(f"1:{scale} " + ", ".join(parts))
    return "; ".join(out) or "none"


def ops_file(slug):
    """📁️ The repair ops file of a person."""
    text = open(os.path.join(HERE, "people.ts"), encoding="utf8").read()
    worker = re.search(rf'"{re.escape(slug)}": "(HW-1\d)"', text)
    if slug == "tennis-player":
        return os.path.join(WAVE, "REP-A", "ops", "tennis-player.80-silhouette.json")
    return os.path.join(WAVE, worker.group(1), "repair", f"{slug}.json") if worker else os.path.join(HERE, "ops", f"{slug}.json")


print("| Person | Failing checks before | Failing checks after | Real `🧊️catalog-silhouette` test after | `validateAsset` (now/before), three-views sweep (now/before), check CLI | What was changed |")
print("|---|---|---|---|---|---|")
for slug in people:
    error = re.search(rf"AssertionError: {re.escape(slug)} [^\n]*", test)
    verdict = f"`{error.group(0)[:150]}`" if error else "✓ passes"
    line = proof.get(slug, "").split(" | ")
    count = len(json.load(open(ops_file(slug), encoding="utf8"))) if os.path.exists(ops_file(slug)) else 0
    print(f"| `{slug}` | {failing('gate0', slug)} | {failing('final', slug)} | {verdict} | {', '.join(line[1:])} | {notes.get(slug, '')} ({count} ops in the repair file) |")
