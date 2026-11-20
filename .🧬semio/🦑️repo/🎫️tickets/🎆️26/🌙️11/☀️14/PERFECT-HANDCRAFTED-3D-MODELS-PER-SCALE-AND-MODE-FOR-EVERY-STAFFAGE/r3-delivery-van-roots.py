"""🌱 Root values of the round 3 delivery van outside `volumes`: rig, dimensions, reference, texts, thermal areas and symbols, applied to the parsed asset; the member replacement keeps the file's formatting. Usage: python3 r3-delivery-van-roots.py [--catalog] writes the candidate document or the catalog asset."""
import glob
import importlib
import json
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
ASSET = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*vehicles/delivery-van/*.json")[0]
STAGES = [name for name in ("rig", "roots", "drawings") if os.path.exists(os.path.join(T, f"r3_delivery_van_{name}.py"))]


def member(text, key):
    found = re.search(r'\n  "' + key + r'"\s*:\s*', text)
    depth, index, quoted = 0, found.end(), False
    while True:
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
                return found.end(), index + 1
        index += 1


def dumped(value):
    lines = json.dumps(value, indent=1, ensure_ascii=False).split("\n")
    return "\n".join(line if i == 0 else "  " + line for i, line in enumerate(lines))


def replace(text, key, value):
    start, end = member(text, key)
    return text[:start] + dumped(value) + text[end:]


text = open(ASSET, encoding="utf-8").read()
asset = json.loads(text)["asset"]
before = json.dumps(asset, ensure_ascii=False, sort_keys=True)
members = []
for name in STAGES:
    stage = importlib.import_module(f"r3_delivery_van_{name}")
    stage.apply(asset)
    members += [m for m in stage.MEMBERS if m not in members]
for key in members:
    text = replace(text, key, asset[key])
json.loads(text)
if "--catalog" in sys.argv:
    open(ASSET, "w", encoding="utf-8").write(text)
    print("catalog written:", ", ".join(members))
else:
    out = os.path.join(T, "🗑️generated", "r3-delivery-van", "🔣️.json")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    document = json.loads(text)
    if "--own-volumes" not in sys.argv:
        document["asset"]["volumes"] = json.load(open(os.path.join(T, "volumes", "vehicles", "delivery-van.json"), encoding="utf-8"))["volumes"]
    json.dump(document, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(out)
