"""🪡 Root edits of `humans/elderly-with-cane` outside `rig`, `anchors`, `poses`, `clips` (those are written by `r3-elderly-with-cane-roots.ts`): the redrawn 2D geometry of every scale and view, the two materials the model needs, the dimensions of the default pose and the drawing decisions. The file is re-read right before writing and only the named members are replaced textually; every other member stays byte-identical.

Usage: python3 r3-elderly-with-cane-edit.py            writes the candidate `🗑️generated/r3-elderly-with-cane/🔣️.json` (with the volumes source merged in)
       python3 r3-elderly-with-cane-edit.py --write    writes the catalog file (members `materials`, `dimensions`, `representations` only)
"""
import glob
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ASSET = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/elderly-with-cane/🔣️.json")[0]
OUT = os.path.join(HERE, "🗑️generated", "r3-elderly-with-cane")
MATERIALS = [
    {"id": "bone", "title": {"en": "Bone", "de": "Knochen"}, "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 1900, "source": "authored-estimate"},
    {"id": "spectacle-frame", "title": {"en": "Metal spectacle frame", "de": "Brillengestell aus Metall"}, "emissivity": 0.3, "solarAbsorptivity": 0.5, "source": "authored-estimate"},
]


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160)
    return False


def dump(value, depth, style):
    if style == "plain":
        return json.dumps(value, ensure_ascii=False, indent=2).replace("\n", "\n" + "  " * depth)
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1, style) for key, item in value.items()) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1, style) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


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
        index += 1
    raise SystemExit("unbalanced asset file")


def change(asset):
    geometry = json.load(open(os.path.join(OUT, "geometry.json"), encoding="utf-8"))
    for scale, views in geometry.items():
        for view, shapes in views.items():
            asset["representations"][scale]["geometry"][view] = shapes
    extra = os.path.join(HERE, "r3-elderly-with-cane-texts.json")
    if os.path.exists(extra):
        texts = json.load(open(extra, encoding="utf-8"))
        for scale, decisions in texts.get("decisions", {}).items():
            asset["representations"][scale]["decisions"] = decisions
        asset["dimensions"].update(texts.get("dimensions", {}))
    have = {m["id"] for m in asset["materials"]}
    asset["materials"].extend(m for m in MATERIALS if m["id"] not in have)
    return ["materials", "dimensions", "representations"]


text = open(ASSET, encoding="utf-8").read()
before = json.loads(text)
document = json.loads(text)
indent = re.search(r'\n( +)"id"\s*:', text).group(1)
depth = len(indent) // 2
keys = change(document["asset"])
for key in keys:
    start, end = member_range(text, indent, key)
    style = next((s for s in ("compact", "plain") if indent + '"%s": ' % key + dump(before["asset"][key], depth, s) == text[start:end]), "plain")
    text = text[:start] + indent + '"%s": ' % key + dump(document["asset"][key], depth, style) + text[end:]
check = json.loads(text)
for key in before["asset"]:
    if key not in keys and check["asset"][key] != before["asset"][key]:
        raise SystemExit("member %s changed unexpectedly" % key)
if list(check["asset"].keys()) != list(before["asset"].keys()):
    raise SystemExit("member order changed")
if "--write" in sys.argv:
    open(ASSET, "w", encoding="utf-8").write(text)
    print("written", ASSET)
else:
    check["asset"]["volumes"] = json.load(open(os.path.join(HERE, "volumes", "humans", "elderly-with-cane.json"), encoding="utf-8"))["volumes"]
    os.makedirs(OUT, exist_ok=True)
    json.dump(check, open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8"), ensure_ascii=False)
    print("candidate", os.path.join(OUT, "🔣️.json"))
