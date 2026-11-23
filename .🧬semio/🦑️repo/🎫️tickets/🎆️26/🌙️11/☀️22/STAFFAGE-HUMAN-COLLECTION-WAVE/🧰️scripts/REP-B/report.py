"""📝️ Writes the per-person tables of the repair report from the records of this folder: `record/before.json` and `record/after.json` (gate logs condensed by summarize.py), `record/before-silhouette-test.txt` and `record/after-silhouette-test.txt` (the lines of the real test), `record/proof.txt` (validator and check CLI) and the repair ops of every person. Prints Markdown to stdout.
python3 report.py"""
import collections
import json
import os
import re

K = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
HERE = f"{K}/🧰️scripts/REP-B"
WORKER = {}
for line in open(f"{HERE}/people.ts", encoding="utf8"):
    for slug, worker in re.findall(r'"([a-z-]+)": "(HW-\d+)"', line):
        WORKER[slug] = worker


def load(name):
    path = f"{HERE}/record/{name}"
    if not os.path.exists(path):
        return None
    with open(path, encoding="utf8") as f:
        return json.load(f) if name.endswith(".json") else f.read()


def gates(entry):
    if entry is None:
        return "not measured"
    parts = []
    by_scale = collections.defaultdict(list)
    for scale, view, iou, h in entry["silhouette"]:
        by_scale[scale].append(f"{view} {'IoU ' + format(iou, '.3f') if iou < 0.92 else str(h) + ' mm'}")
    for scale in sorted(by_scale):
        parts.append(f"1:{scale} " + ", ".join(by_scale[scale]))
    for scale, axis, value in entry["box"]:
        parts.append(f"1:{scale} box {axis} {value}")
    for rule, n in sorted(entry["rules"].items()):
        parts.append(f"{rule.replace('volumes-', '')} ×{n}")
    return "; ".join(parts) if parts else "none"


def ops_of(slug):
    path = f"{K}/🧰️scripts/{WORKER[slug]}/repair/{slug}.json"
    if not os.path.exists(path):
        return "no ops"
    ops = json.load(open(path, encoding="utf8"))
    c = collections.Counter()
    for o in ops:
        if o["op"] == "region-set":
            c["thermal areas redeclared"] += 1
        elif o["op"] == "dims":
            c["dimensions redeclared"] += len(o["set"])
        elif o["op"] == "solid-add" and o["solid"].get("part") == "joint":
            c["joints added"] += 1
        elif o["op"].startswith("solid"):
            c["solids moved or added"] += 1
        elif o["op"] == "shape-remove":
            c["shapes given way to prisms"] += 1
        elif o["op"] == "shape-add" and str(o.get("group", "")).startswith("prism"):
            c["solids drawn as prisms"] += 1
        elif o["op"] == "shape-add":
            c["shapes added"] += 1
        elif o["op"].startswith("shape"):
            c["shapes redrawn or refitted"] += 1
    return ", ".join(f"{n} {what}" for what, n in c.items()) or "no ops"


before, after = load("before.json") or {}, load("after.json") or {}
test_before = {m.group(1): line.strip() for line in (load("before-silhouette-test.txt") or "").splitlines() if (m := re.match(r"AssertionError: (\S+) ", line))}
test_after = {m.group(1): line.strip() for line in (load("after-silhouette-test.txt") or "").splitlines() if (m := re.match(r"AssertionError: (\S+) ", line))}
proof = {line.split(" | ")[0]: line.strip().split(" | ")[1:] for line in (load("proof.txt") or "").splitlines() if " | " in line}
print("| Worker | Person | Failing gates before | Failing gates after | Real `catalog-silhouette` test after | Validator, check CLI | Repair ops |")
print("|---|---|---|---|---|---|---|")
green = 0
for slug, worker in WORKER.items():
    line = test_after.get(slug)
    verdict = "✓ passes" if line is None and after.get(slug) is not None else f"`{line}`" if line else "not run"
    green += line is None and after.get(slug) is not None
    p = proof.get(slug, ["?", "?"])
    print(f"| {worker} | `{slug}` | {gates(before.get(slug))} | {gates(after.get(slug))} | {verdict} | {p[0]}, {p[1][:40] if len(p) > 1 else '?'} | {ops_of(slug)} |")
print()
print(f"Real test: {len(test_before)} of {len(WORKER)} people failed before, {len(test_after)} fail after, {green} pass.")
print(f"Failing silhouette checks (scale × view, of {len(WORKER) * 12}): {sum(len(v['silhouette']) for v in before.values())} before, {sum(len(v['silhouette']) for v in after.values())} after. `checkVolumes` findings without `volumes-silhouette`: {sum(sum(v['rules'].values()) for v in before.values())} before, {sum(sum(v['rules'].values()) for v in after.values())} after.")
