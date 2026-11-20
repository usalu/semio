"""🧷 Assembles and writes the root fixes of `humans/person-with-rollator`.

Without arguments it builds the candidate document `🗑️generated/r3-person-with-rollator/🔣️.json`: the catalog asset with the patches of this round laid over it (`roots.json` from `r3-person-with-rollator-roots.ts`: rig, poses, provenance; anchors and clips stay as the catalog has them since the heel anchors and the stride were calibrated there on 2026-11-20 by `hu-heel.py` and `hu-stride.py`; `geometry.json` from `r3-person-with-rollator-draw2d.ts`: the redrawn 2D shapes; `r3-person-with-rollator-texts.json`: drawing decisions, dimensions, materials, thermal corrections) and with the volumes source merged in. With `--write` the members that differ are replaced textually in the catalog file (re-read right before, the file's own two-space style, every other member byte-identical, member order checked; `volumes` is left to `volumes-insert.py`) and the manifest row follows the dimensions.

Usage: python3 r3-person-with-rollator-candidate.py [--write]
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
OUT = os.path.join(HERE, "🗑️generated", "r3-person-with-rollator")


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


def load(name):
    path = os.path.join(OUT, name) if not name.startswith("r3-") else os.path.join(HERE, name)
    return json.load(open(path, encoding="utf-8")) if os.path.exists(path) else None


def patched(asset):
    roots = load("roots.json")
    if roots:
        asset.update({k: roots[k] for k in ("rig", "poses", "provenance")})
    geometry = load("geometry.json")
    if geometry:
        for scale, views in geometry.items():
            asset["representations"][scale]["geometry"] = views
    texts = load("r3-person-with-rollator-texts.json")
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
