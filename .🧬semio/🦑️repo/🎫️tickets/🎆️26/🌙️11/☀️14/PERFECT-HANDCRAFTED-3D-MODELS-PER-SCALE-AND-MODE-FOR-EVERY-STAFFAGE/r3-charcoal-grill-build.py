"""🔥 Round 3 charcoal kettle grill: builds the candidate asset document (roots, drawings, volumes) under `🗑️generated/r3-charcoal-grill/`, writes the volumes source with `--source`, and writes the changed catalog members with `--apply`.

Usage: python3 r3-charcoal-grill-build.py [--source] [--apply]
"""
import importlib.util
import json
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/charcoal-grill/🔣️.json"
SOURCE = os.path.join(T, "volumes", "objects", "charcoal-grill.json")
OUT = os.path.join(T, "🗑️generated", "r3-charcoal-grill")
MEMBERS = ["provenance", "dimensions", "rig", "anchors", "representations", "poses", "clips", "thermal", "structure"]


def module(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, "r3-charcoal-grill-%s.py" % name))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


def volumes():
    out = {}
    for scale in ("50", "100", "200", "500"):
        path = os.path.join(T, "r3-charcoal-grill-s%s.py" % scale)
        if os.path.exists(path):
            out[scale] = module("s%s" % scale).volume()
    return out


def member(source, key):
    found = re.search(r'\n    "' + key + r'"\s*:\s*', source)
    depth, index, quoted = 0, found.end(), False
    while True:
        char = source[index]
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
                return found.end(), index + 1
        index += 1


def dumped(value):
    lines = json.dumps(value, indent=2, ensure_ascii=False).split("\n")
    return "\n".join(line if i == 0 else "    " + line for i, line in enumerate(lines))


def build(document):
    asset = document["asset"]
    areas_path = os.path.join(T, "r3-charcoal-grill-areas.json")
    areas = json.load(open(areas_path, encoding="utf-8")) if os.path.exists(areas_path) else None
    module("roots").apply(asset, areas)
    if os.path.exists(os.path.join(T, "r3-charcoal-grill-draw.py")):
        module("draw").apply(asset)
    vols = volumes()
    merged = dict(asset["volumes"])
    merged.update(vols)
    asset["volumes"] = merged
    return document


if __name__ == "__main__":
    source = open(CATALOG, encoding="utf-8").read()
    document = build(json.loads(source))
    os.makedirs(OUT, exist_ok=True)
    with open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(document, f, ensure_ascii=False, indent=2)
    print("candidate written", os.path.join(OUT, "🔣️.json"))
    if "--source" in sys.argv:
        with open(SOURCE, "w", encoding="utf-8") as f:
            json.dump({"volumes": document["asset"]["volumes"]}, f, ensure_ascii=False, indent=2)
            f.write("\n")
        print("volumes source written", SOURCE)
    if "--apply" in sys.argv:
        fresh = open(CATALOG, encoding="utf-8").read()
        before = json.loads(fresh)
        for key in MEMBERS:
            start, end = member(fresh, key)
            if dumped(before["asset"][key]) != fresh[start:end]:
                raise SystemExit("member %s is not in the plain layout, not written" % key)
            fresh = fresh[:start] + dumped(document["asset"][key]) + fresh[end:]
        check = json.loads(fresh)
        for key in check["asset"]:
            if key not in MEMBERS and check["asset"][key] != before["asset"][key]:
                raise SystemExit("member %s changed unexpectedly" % key)
        with open(CATALOG, "w", encoding="utf-8") as f:
            f.write(fresh)
        print("catalog members written:", ", ".join(MEMBERS))
