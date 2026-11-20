"""🏗️ Round 3 shopping trolley: builds the candidate asset document (roots, drawings, volumes) under `🗑️generated/r3-shopping-trolley/`; `--source` writes the volumes source, `--apply` writes the changed catalog members (re-read right before, member order and formatting kept).

Usage: python3 r3-shopping-trolley-build.py [--source] [--apply]
"""
import importlib.util
import json
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/shopping-trolley/🔣️.json"
SOURCE = os.path.join(T, "volumes", "objects", "shopping-trolley.json")
OUT = os.path.join(T, "🗑️generated", "r3-shopping-trolley")
MEMBERS = ["provenance", "rig", "anchors", "representations", "poses", "thermal", "structure"]


def module(name):
    spec = importlib.util.spec_from_file_location("trolley_" + name, os.path.join(T, "r3-shopping-trolley-%s.py" % name))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


def volumes(com):
    out = {}
    for scale in ("50", "100", "200", "500"):
        if os.path.exists(os.path.join(T, "r3-shopping-trolley-s%s.py" % scale)):
            out[scale] = module("s" + scale).volume(com)
    return out


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(i) for i in value) or (all(isinstance(i, list) and all(scalar(x) for x in i) for i in value) and len(json.dumps(value)) <= 160)
    return False


def dump(value, depth):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(k, ensure_ascii=False) + ": " + dump(v, depth + 1) for k, v in value.items()) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(v, depth + 1) for v in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def member_range(text, key):
    found = re.search(r'\n    "' + key + r'"\s*:\s*', text)
    depth, i, quoted = 0, found.end(), False
    while True:
        c = text[i]
        if quoted:
            if c == "\\":
                i += 1
            elif c == '"':
                quoted = False
        elif c == '"':
            quoted = True
        elif c in "{[":
            depth += 1
        elif c in "}]":
            depth -= 1
            if depth == 0:
                return found.end(), i + 1
        i += 1


def build(document):
    asset = document["asset"]
    roots = module("roots")
    areas_path = os.path.join(T, "r3-shopping-trolley-areas.json")
    areas = json.load(open(areas_path, encoding="utf-8")) if os.path.exists(areas_path) else None
    roots.apply(asset, areas)
    module("draw").apply(asset)
    merged = dict(asset["volumes"])
    merged.update(volumes(roots.masses()))
    asset["volumes"] = merged
    return document


def main():
    text = open(CATALOG, encoding="utf-8").read()
    document = build(json.loads(text))
    os.makedirs(OUT, exist_ok=True)
    with open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8") as f:
        f.write(dump(document, 0) + "\n")
    print("candidate", os.path.join(OUT, "🔣️.json"))
    if "--source" in sys.argv:
        with open(SOURCE, "w", encoding="utf-8") as f:
            f.write(dump({"volumes": document["asset"]["volumes"]}, 0) + "\n")
        print("source", SOURCE)
    if "--apply" in sys.argv:
        fresh = open(CATALOG, encoding="utf-8").read()
        if fresh != text:
            raise SystemExit("catalog changed while building; rerun")
        out = fresh
        for key in MEMBERS:
            a, b = member_range(out, key)
            out = out[:a] + dump(document["asset"][key], 2) + out[b:]
        check = json.loads(out)
        for key, value in check["asset"].items():
            if key in MEMBERS:
                assert value == document["asset"][key], key
            else:
                assert value == json.loads(text)["asset"][key], key
        assert list(check["asset"]) == list(json.loads(text)["asset"])
        if open(CATALOG, encoding="utf-8").read() != text:
            raise SystemExit("catalog changed while writing; rerun")
        open(CATALOG, "w", encoding="utf-8").write(out)
        print("catalog members written:", ", ".join(MEMBERS))


if __name__ == "__main__":
    main()
