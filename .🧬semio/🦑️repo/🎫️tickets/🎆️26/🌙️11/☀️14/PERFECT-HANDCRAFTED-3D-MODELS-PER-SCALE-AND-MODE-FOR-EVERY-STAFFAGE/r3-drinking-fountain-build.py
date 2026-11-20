"""🏭 Round 3 cast drinking fountain: builds the candidate asset document (roots, drawings, volumes) under `🗑️generated/r3-drinking-fountain/`, writes the volumes source with `--source`, and writes the changed catalog members with `--apply`.

Usage: python3 r3-drinking-fountain-build.py [--source] [--apply]
"""
import importlib.util
import json
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/drinking-fountain/🔣️.json"
SOURCE = os.path.join(T, "volumes", "objects", "drinking-fountain.json")
OUT = os.path.join(T, "🗑️generated", "r3-drinking-fountain")
MEMBERS = ["summary", "provenance", "rig", "anchors", "representations", "poses", "thermal"]
BASELINE = os.path.join(OUT, "catalog-before-r3.json")


def module(name):
    """📦 Loads one of this asset's scripts."""
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, "r3-drinking-fountain-%s.py" % name))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


def member(source, key):
    """✂️ Start and end of a top-level asset member in the catalog text."""
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
    """🧾 A member value in the file's own two-space style at asset depth."""
    lines = json.dumps(value, indent=2, ensure_ascii=False).split("\n")
    return "\n".join(line if i == 0 else "    " + line for i, line in enumerate(lines))


def build(document):
    """🔨 Applies roots, drawings and volumes to a copy of the baseline catalog document."""
    asset = document["asset"]
    areas_path = os.path.join(T, "r3-drinking-fountain-areas.json")
    areas = json.load(open(areas_path, encoding="utf-8")) if os.path.exists(areas_path) else None
    module("roots").apply(asset, areas)
    module("draw").apply(asset)
    asset["volumes"] = module("model").volumes()
    return document


if __name__ == "__main__":
    os.makedirs(OUT, exist_ok=True)
    if not os.path.exists(BASELINE):
        open(BASELINE, "w", encoding="utf-8").write(open(CATALOG, encoding="utf-8").read())
    document = build(json.loads(open(BASELINE, encoding="utf-8").read()))
    with open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(document, f, ensure_ascii=False, indent=2)
    print("candidate written", os.path.join(OUT, "🔣️.json"))
    if "--source" in sys.argv:
        with open(SOURCE, "w", encoding="utf-8") as f:
            json.dump({"volumes": document["asset"]["volumes"]}, f, ensure_ascii=False, indent=2)
            f.write("\n")
        print("source written", SOURCE)
    if "--apply" in sys.argv:
        text = open(CATALOG, encoding="utf-8").read()
        current = json.loads(text)["asset"]
        for key in MEMBERS:
            start, end = member(text, key)
            text = text[:start] + dumped(document["asset"][key]) + text[end:]
        after = json.loads(text)["asset"]
        for key in after:
            if key not in MEMBERS and key != "volumes" and after[key] != current[key]:
                raise SystemExit("member changed unexpectedly: " + key)
        open(CATALOG, "w", encoding="utf-8").write(text)
        print("catalog members written", ", ".join(MEMBERS))
