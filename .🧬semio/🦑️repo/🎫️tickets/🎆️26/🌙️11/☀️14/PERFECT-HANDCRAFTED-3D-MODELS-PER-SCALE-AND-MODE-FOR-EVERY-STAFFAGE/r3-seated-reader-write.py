"""💾 Writes the round 3 root fixes of `humans/seated-reader` into the catalog: the members of the candidate document built by `r3-seated-reader-build.ts` (summary, provenance, dimensions, materials, rig, anchors, representations with the redrawn 2D and its decisions, symbols, poses, clips, thermal, structure) replace the same members of the asset file, which is read again right before writing; `volumes` goes in with the same write so that the asset is never half new, and every other member stays byte-identical (`volumes-insert.py` afterwards changes nothing). The manifest row of the asset gets the new height and width.

Usage: python3 r3-seated-reader-write.py
"""
import json
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
ASSET = CATALOG + "/👤️humans/seated-reader/🔣️.json"
MANIFEST = CATALOG + "/🔣️.json"
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-seated-reader", "🔣️.json")
MEMBERS = ["summary", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure", "volumes"]



def member_end(text, start):
    depth, index, quoted = 0, start, False
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
                return index + 1
        index += 1
    raise SystemExit("unbalanced json")


def member_range(text, indent, key):
    found = re.search(r'\n' + indent + r'"' + key + r'"\s*:\s*', text)
    return found.start() + 1, member_end(text, found.end())


def dumped(value, indent):
    return json.dumps(value, indent=2, ensure_ascii=False).replace("\n", "\n" + indent)


def write_asset(candidate):
    text = open(ASSET, encoding="utf-8").read()
    document = json.loads(text)
    indent = re.search(r'\n( +)"id"\s*:', text).group(1)
    result = text
    for key in MEMBERS:
        start, end = member_range(result, indent, key)
        result = result[:start] + indent + '"' + key + '": ' + dumped(candidate["asset"][key], indent) + result[end:]
    check = json.loads(result)
    for key in document["asset"]:
        expected = candidate["asset"][key] if key in MEMBERS else document["asset"][key]
        if check["asset"][key] != expected:
            raise SystemExit("member %s did not round-trip" % key)
    if list(check["asset"]) != list(document["asset"]):
        raise SystemExit("member order changed")
    if open(ASSET, encoding="utf-8").read() != text:
        raise SystemExit("asset changed while writing; retry")
    open(ASSET, "w", encoding="utf-8").write(result)


def write_manifest(dimensions):
    text = open(MANIFEST, encoding="utf-8").read()
    found = re.search(r'\{\s*"id": "seated-reader",\s*"category": "humans"', text)
    end = member_end(text, found.start())
    row = text[found.start():end]
    new = re.sub(r'("heightM": )[0-9.]+', lambda m: m.group(1) + repr(dimensions["heightM"]), row, count=1)
    new = re.sub(r'("widthM": )[0-9.]+', lambda m: m.group(1) + repr(dimensions["widthM"]), new, count=1)
    result = text[:found.start()] + new + text[end:]
    json.loads(result)
    if open(MANIFEST, encoding="utf-8").read() != text:
        raise SystemExit("manifest changed while writing; retry")
    open(MANIFEST, "w", encoding="utf-8").write(result)


candidate = json.load(open(CANDIDATE, encoding="utf-8"))
write_asset(candidate)
write_manifest(candidate["asset"]["dimensions"])
print("written:", ", ".join(MEMBERS), "and the manifest row", candidate["asset"]["dimensions"])
