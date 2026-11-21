"""🧷 Assembles and writes the root fixes of `humans/person-with-rollator`.

Without arguments it builds the candidate document `🗑️generated/r5-person-with-rollator/🔣️.json`: the catalog asset with the patches of round 5 laid over it (`roots.json` from `r5-person-with-rollator-arms.ts`: the rig with the shoulder joints moved in to the real biacromial breadth, and the poses and clips with the arms re-solved so the hands keep their grips; anchors stay as the catalog has them; `geometry.json` from `r5-person-with-rollator-draw2d.ts`: of its redrawn 2D shapes only those of the parts whose solids round 5 changed (`REDRAWN`: coat, coat seat, skirt halves and the arms from the sleeves to the hands, which moved with the shoulder joints) replace the catalog's, each keeping the catalog's layer `z`, so every other drawing stays as the catalog has it; the print bands are narrowed where the product measures a visible sliver below the 0.2 mm office-laser minimum at a band end (`RANGES`: 1:100 up to 1:130, 1:200 from 1:130 to 1:250, 1:500 from 1:250 to 1:500; beyond 1:500 the 22 mm rollator tubes are thinner than any printable line, so the asset is unavailable there), and the backrest strap of 1:200 is drawn 6 mm taller at each edge (`STRAP_200`) so the part left visible beside the stooping arm keeps a printable width; `r5-person-with-rollator-texts.json`: drawing decisions, dimensions, materials, thermal corrections) and with the volumes source merged in. With `--write` the members that differ are replaced textually in the catalog file (re-read right before, the file's own two-space style, every other member byte-identical, member order checked; `volumes` is left to `volumes-insert.py`) and the manifest row follows the dimensions.

Usage: python3 r5-person-with-rollator-candidate.py [--write]
@see ./r3-humans-person-with-rollator.md
"""
import glob
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
ASSET = glob.glob(CATALOG + "/*/person-with-rollator/🔣️.json")[0]
MANIFEST = CATALOG + "/🔣️.json"
OUT = os.path.join(HERE, "🗑️generated", "r5-person-with-rollator")


def member_range(text, indent, key):
    found = re.search(r"\n" + indent + r'"' + key + r'"\s*:\s*', text)
    depth, index, quoted = 0, found.end(), False
    while index < len(text):
        char = text[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return found.start() + 1, index + 1
        elif depth == 0 and char in ",\n":
            return found.start() + 1, index
        index += 1
    raise SystemExit("unbalanced asset file")


REDRAWN = {"coat", "coat-hip", "skirt-l", "skirt-r", "sleeve-l", "sleeve-r", "forearm-l", "forearm-r", "cardigan-cuff-l", "cardigan-cuff-r", "wrist-l", "wrist-r", "hand-l", "hand-r"}
RANGES = {"100": (75, 130), "200": (130, 250), "500": (250, 500)}
STRAP_200 = 0.006


def load(name):
    path = os.path.join(OUT, name) if not name.startswith("r5-") else os.path.join(HERE, name)
    return json.load(open(path, encoding="utf-8")) if os.path.exists(path) else None


def patched(asset):
    roots = load("roots.json")
    if roots:
        asset.update({k: roots[k] for k in ("rig", "poses", "clips", "provenance") if k in roots})
    geometry = load("geometry.json")
    if geometry:
        for scale, views in geometry.items():
            for view, shapes in views.items():
                drawn = {s["id"]: s for s in shapes}
                kept = asset["representations"][scale]["geometry"][view]
                for i, shape in enumerate(kept):
                    if shape["id"] in REDRAWN and shape["id"] in drawn:
                        kept[i] = {**drawn[shape["id"]], "z": shape["z"]}
    for scale, (lo, hi) in RANGES.items():
        asset["representations"][scale]["validRange"] = {"minDenominator": lo, "maxDenominator": hi}
    for shape in asset["representations"]["200"]["geometry"]["elevation"]:
        ys = [p[1] for p in shape.get("points", [])]
        if shape["id"] in ("backrest-l", "backrest-r") and max(ys) - min(ys) < 0.075:
            lo, hi = min(ys), max(ys)
            shape["points"] = [[p[0], round(p[1] - STRAP_200 if p[1] < (lo + hi) / 2 else p[1] + STRAP_200, 4)] for p in shape["points"]]
    texts = load("r5-person-with-rollator-texts.json")
    if texts:
        for scale, part in texts.get("representations", {}).items():
            asset["representations"][scale].update(part)
        asset["dimensions"].update(texts.get("dimensions", {}))
        have = {m["id"] for m in asset["materials"]}
        asset["materials"].extend(m for m in texts.get("materials", []) if m["id"] not in have)
        for region, values in texts.get("thermalRegions", {}).items():
            next(r for r in asset["thermal"]["regions"] if r["id"] == region).update(values)
        for key in ("summary",):
            if key in texts:
                asset[key] = texts[key]
    return asset


text = open(ASSET, encoding="utf-8").read()
before = json.loads(text)
after = patched(json.loads(text)["asset"])
if "--write" not in sys.argv:
    document = json.loads(text)
    document["asset"] = dict(after)
    document["asset"]["volumes"] = json.load(open(os.path.join(HERE, "volumes", "humans", "person-with-rollator.json"), encoding="utf-8"))["volumes"]
    os.makedirs(OUT, exist_ok=True)
    json.dump(document, open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8"), ensure_ascii=False)
    print("candidate", os.path.join(OUT, "🔣️.json"), "changed:", ", ".join(k for k in before["asset"] if before["asset"][k] != after[k]))
    raise SystemExit(0)
indent = re.search(r'\n( +)"id"\s*:', text).group(1)
depth = len(indent) // 2
changed = [k for k in before["asset"] if k != "volumes" and before["asset"][k] != after[k]]
for key in changed:
    start, end = member_range(text, indent, key)
    body = json.dumps(after[key], ensure_ascii=False, indent=2).replace("\n", "\n" + "  " * depth)
    text = text[:start] + indent + '"%s": ' % key + body + text[end:]
check = json.loads(text)
for key in before["asset"]:
    if key not in changed and check["asset"][key] != before["asset"][key]:
        raise SystemExit("member %s changed unexpectedly" % key)
    if key in changed and check["asset"][key] != after[key]:
        raise SystemExit("member %s not written as meant" % key)
if list(check["asset"].keys()) != list(before["asset"].keys()):
    raise SystemExit("member order changed")
print("changed members:", ", ".join(changed) or "none")
if changed:
    open(ASSET, "w", encoding="utf-8").write(text)
    print("written", ASSET)
manifest = open(MANIFEST, encoding="utf-8").read()
row = re.search(r'\{\s*"id": "person-with-rollator",.*?"poses"', manifest, re.S)
new = row.group(0)
for key in ("heightM", "widthM"):
    new = re.sub(r'"%s": [0-9.]+' % key, '"%s": %s' % (key, json.dumps(after["dimensions"][key])), new)
if new != row.group(0):
    open(MANIFEST, "w", encoding="utf-8").write(manifest[: row.start()] + new + manifest[row.end():])
    print("manifest row updated")
