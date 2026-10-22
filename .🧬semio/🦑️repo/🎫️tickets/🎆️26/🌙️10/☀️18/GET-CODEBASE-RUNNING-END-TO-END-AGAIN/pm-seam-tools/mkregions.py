#!/usr/bin/env python3
"""📍️ Cuts the pre-seam region fixtures: for every manager-specific position of a neutral schema base (as staged in round 2) the value the pre-seam original schema held there. usage: PM_MIRROR=DIR mkregions.py   (AMENDMENTS are the neutral edits other sets made inside a region text after the pre-seam original was cut; writes DIR/<fixtures>/📥️install.original.json and 🧾️generation.original.json)"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
os.environ.setdefault("PM_ROUND", "3")
import paths, winpath
STAGED2 = os.path.join(paths.HERE, "staged2", "new")
D = paths.DEPENDENCIES
AMENDMENTS = [("install", "/description", "nested semio, array ordering", "nested semio, the install-inert manifest fields named by the adopt protocol, array ordering")]
def staged(rel, suffix):
    return json.loads(open(winpath.resolve(STAGED2, D + "/" + rel + suffix), encoding="utf-8").read())
def isx(node): return isinstance(node, dict) and "$manager" in node
def region(node): return isx(node) or (isinstance(node, str) and "{{" in node)
def walk(base, original, stable, out):
    if region(base):
        out.append([stable, original])
    elif isinstance(base, list):
        for index, item in enumerate(base):
            step = "$" + item["$manager"] if isx(item) else item if region(item) else str(index)
            walk(item, original[index], stable + "/[" + step + "]", out)
    elif isinstance(base, dict):
        for key, value in base.items():
            if key in original:
                walk(value, original[key], stable + "/" + key, out)
for name, rel, target in [("install", "💾️storage/🧬️schema/📥️install.json", "📥️install.original.json"), ("generation", "🧾️generation/🧬️schema/🔣️.json", "🧾️generation.original.json")]:
    rows = []
    walk(staged(rel, ".r2"), staged(rel, ".r1"), "", rows)
    for owner, path, old, new in AMENDMENTS:
        for row in rows:
            if owner == name and row[0] == path:
                assert row[1].count(old) == 1, (path, old)
                row[1] = row[1].replace(old, new)
    paths.seam_write(paths.MANAGERS + "/🧪️tests/🧫️fixtures/" + target, json.dumps(rows, ensure_ascii=False, indent=1) + "\n")
    print(name, len(rows), "regions")
