"""🎌 Root edits of `humans/tour-guide` outside `volumes`: the members computed by `r3-tour-guide-roots.ts` (rig, anchors, poses), the redrawn 2D geometry of `r3-tour-guide-draw2d.ts` and the texts of `r3-tour-guide-texts.json` (drawing decisions, dimensions, materials, references). The file is re-read right before writing and only the named members are replaced textually; every other member stays byte-identical.

Usage: python3 r3-tour-guide-apply.py            writes the candidate `🗑️generated/r3-tour-guide/🔣️.json` (with the volumes source merged in)
       python3 r3-tour-guide-apply.py --write    writes the catalog file
"""
import glob
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ASSET = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/tour-guide/🔣️.json")[0]
OUT = os.path.join(HERE, "🗑️generated", "r3-tour-guide")


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
    if text[index] not in "{[":
        end = index
        while text[end] != "\n":
            end += 1
        return found.start() + 1, end - (1 if text[end - 1] == "," else 0)
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


def load(name):
    path = os.path.join(OUT, name) if not name.startswith("r3-") else os.path.join(HERE, name)
    return json.load(open(path, encoding="utf-8")) if os.path.exists(path) else None


def change(asset):
    keys = []
    roots = load("roots.json")
    for key in ("rig", "anchors", "poses"):
        asset[key] = roots[key]
        keys.append(key)
    geometry = load("geometry.json")
    texts = load("r3-tour-guide-texts.json")
    if geometry is not None:
        for scale, views in geometry.items():
            for view, shapes in views.items():
                asset["representations"][scale]["geometry"][view] = shapes
        keys.append("representations")
    if texts is not None:
        for scale, part in texts.get("representations", {}).items():
            for key, value in part.items():
                if key == "decisions":
                    asset["representations"][scale]["decisions"].update(value)
                else:
                    asset["representations"][scale][key] = value
        if "representations" not in keys:
            keys.append("representations")
        asset["dimensions"].update(texts.get("dimensions", {}))
        have = {m["id"] for m in asset["materials"]}
        asset["materials"].extend(m for m in texts.get("materials", []) if m["id"] not in have)
        have = {r["id"] for r in asset["provenance"]["references"]}
        asset["provenance"]["references"].extend(r for r in texts.get("references", []) if r["id"] not in have)
        for region in asset["thermal"]["regions"]:
            region.update(texts.get("regions", {}).get(region["id"], {}))
        keys += ["dimensions", "materials", "provenance", "thermal", "structure"]
    return keys


text = open(ASSET, encoding="utf-8").read()
before = json.loads(text)
document = json.loads(text)
indent = re.search(r'\n( +)"id"\s*:', text).group(1)
depth = len(indent) // 2
keys = change(document["asset"])
for key in keys:
    if document["asset"][key] == before["asset"][key]:
        continue
    start, end = member_range(text, indent, key)
    style = next((s for s in ("compact", "plain") if indent + '"%s": ' % key + dump(before["asset"][key], depth, s) == text[start:end]), "plain")
    text = text[:start] + indent + '"%s": ' % key + dump(document["asset"][key], depth, style) + text[end:]
check = json.loads(text)
for key in before["asset"]:
    if key not in keys and check["asset"][key] != before["asset"][key]:
        raise SystemExit("member %s changed unexpectedly" % key)
    if key in keys and check["asset"][key] != document["asset"][key]:
        raise SystemExit("member %s not written as meant" % key)
if list(check["asset"].keys()) != list(before["asset"].keys()):
    raise SystemExit("member order changed")
if "--write" in sys.argv:
    open(ASSET, "w", encoding="utf-8").write(text)
    print("written", ASSET)
else:
    check["asset"]["volumes"] = json.load(open(os.path.join(HERE, "volumes", "humans", "tour-guide.json"), encoding="utf-8"))["volumes"]
    os.makedirs(OUT, exist_ok=True)
    json.dump(check, open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8"), ensure_ascii=False)
    print("candidate", os.path.join(OUT, "🔣️.json"))
