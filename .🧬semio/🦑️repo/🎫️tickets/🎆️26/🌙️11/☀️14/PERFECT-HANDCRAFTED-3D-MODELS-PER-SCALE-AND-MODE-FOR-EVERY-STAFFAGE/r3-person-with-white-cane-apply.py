"""🪡 Root edits of `humans/person-with-white-cane` outside `volumes`: the members computed by `r3-person-with-white-cane-roots.ts` (rig, anchors, poses, clips, summary), the redrawn 2D geometry of `r3-person-with-white-cane-draw2d.ts` and the texts of `r3-person-with-white-cane-texts.json` (drawing decisions, dimensions, materials, references). The file is re-read right before writing and only the named members are replaced textually; every other member stays byte-identical.

Usage: python3 r3-person-with-white-cane-apply.py            writes the candidate `🗑️generated/r3-person-with-white-cane/🔣️.json` (with the volumes source merged in)
       python3 r3-person-with-white-cane-apply.py --write    writes the catalog file
"""
import glob
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ASSET = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/person-with-white-cane/🔣️.json")[0]
OUT = os.path.join(HERE, "🗑️generated", "r3-person-with-white-cane")


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
    for key in ("summary", "rig", "anchors", "poses", "clips"):
        asset[key] = roots[key]
        keys.append(key)
    geometry = load("geometry.json")
    texts = load("r3-person-with-white-cane-texts.json")
    if geometry is not None:
        for scale, views in geometry.items():
            for view, shapes in views.items():
                asset["representations"][scale]["geometry"][view] = shapes
        keys.append("representations")
    if texts is not None:
        for scale, part in texts.get("representations", {}).items():
            asset["representations"][scale].update(part)
        if "representations" not in keys:
            keys.append("representations")
        asset["dimensions"].update(texts.get("dimensions", {}))
        have = {m["id"] for m in asset["materials"]}
        asset["materials"].extend(m for m in texts.get("materials", []) if m["id"] not in have)
        have = {r["id"] for r in asset["provenance"]["references"]}
        asset["provenance"]["references"].extend(r for r in texts.get("references", []) if r["id"] not in have)
        for region in asset["thermal"]["regions"]:
            region.update(texts.get("regions", {}).get(region["id"], {}))
        for emitter in asset["thermal"]["emitters"]:
            emitter.update(texts.get("emitters", {}).get(emitter["id"], {}))
        asset["thermal"]["assumptions"] = texts.get("thermalAssumptions", asset["thermal"]["assumptions"])
        asset["structure"]["assumptions"] = texts.get("structureAssumptions", asset["structure"]["assumptions"])
        if "notes" in texts:
            asset["provenance"]["notes"] = texts["notes"]
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
    manifest_path = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(ASSET))), "🔣️.json")
    manifest = open(manifest_path, encoding="utf-8").read()
    at = manifest.index('"id": "person-with-white-cane",\n      "category": "humans"')
    end = manifest.index('"powerW"', at)
    row = manifest[at:end]
    for key in ("heightM", "widthM"):
        row = re.sub(r'("' + key + r'": )[0-9.]+', lambda m: m.group(1) + json.dumps(document["asset"]["dimensions"][key]), row)
    if row != manifest[at:end]:
        open(manifest_path, "w", encoding="utf-8").write(manifest[:at] + row + manifest[end:])
        print("manifest row updated")
else:
    check["asset"]["volumes"] = json.load(open(os.path.join(HERE, "volumes", "humans", "person-with-white-cane.json"), encoding="utf-8"))["volumes"]
    os.makedirs(OUT, exist_ok=True)
    json.dump(check, open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8"), ensure_ascii=False)
    print("candidate", os.path.join(OUT, "🔣️.json"))
