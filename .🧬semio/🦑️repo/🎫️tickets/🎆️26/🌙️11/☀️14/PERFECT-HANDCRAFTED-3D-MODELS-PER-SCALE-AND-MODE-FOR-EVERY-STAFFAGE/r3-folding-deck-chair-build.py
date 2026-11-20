"""🏗️ Builds round 3 of `furniture/folding-deck-chair`: the volumes source `volumes/furniture/folding-deck-chair.json` and the candidate asset document `🗑️generated/r3-folding-deck-chair/🔣️.json` (catalog asset with the corrected root members and the new volumes). `--apply` writes the root members into the catalog asset file and its index row, member by member, re-reading the file right before.

Usage: python3 r3-folding-deck-chair-build.py [--apply]
"""
import importlib.util
import io
import contextlib
import json
import os
import re
import sys

from r3_folding_deck_chair_roots import DIMENSIONS, members
from r3_folding_deck_chair_scales import SCALES

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(T, "..", "..", "..", "..", "..", "..", "..", "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog"))
ASSET = os.path.join(CATALOG, "🛋️furniture", "folding-deck-chair", "🔣️.json")
INDEX = os.path.join(CATALOG, "🔣️.json")


def insert_tool():
    source = open(os.path.join(T, "volumes-insert.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0]
    scope = {}
    exec(compile(source, "volumes-insert.py", "exec"), scope)
    return scope


def main():
    tool = insert_tool()
    scales = {key: build() for key, build in SCALES.items()}
    volumes = {key: scales[key].volume() for key in ("50", "100", "200", "500")}
    with open(os.path.join(T, "volumes", "furniture", "folding-deck-chair.json"), "w", encoding="utf-8") as out:
        json.dump({"volumes": volumes}, out, ensure_ascii=False, indent=1)
        out.write("\n")
    text = open(ASSET, encoding="utf-8").read()
    document = json.loads(text)
    new = members(document["asset"], scales)
    candidate = json.loads(text)
    candidate["asset"].update(new)
    candidate["asset"]["volumes"] = volumes
    folder = os.path.join(T, "🗑️generated", "r3-folding-deck-chair")
    os.makedirs(folder, exist_ok=True)
    candidate["$schema"] = os.path.relpath(os.path.join(CATALOG, "..", "🧬️schema", "🔣️.json"), folder).replace(os.sep, "/") + "#/$defs/AssetDocument"
    with open(os.path.join(folder, "🔣️.json"), "w", encoding="utf-8") as out:
        json.dump(candidate, out, ensure_ascii=False, indent=2)
    print("volumes:", {k: [len(v[l]) for l in ("body", "thermal", "structure")] for k, v in volumes.items()}, "dimensions:", DIMENSIONS)
    if "--apply" not in sys.argv:
        return
    indent = re.search(r'\n( +)"id"\s*:', text).group(1)
    result = text
    for key, value in new.items():
        start, end = tool["member_range"](result, indent, key)
        result = result[:start] + indent + json.dumps(key) + ": " + tool["dump"](value, len(indent) // 2) + result[end:]
    check = json.loads(result)
    if list(check["asset"]) != list(document["asset"]):
        raise SystemExit("member order changed")
    for key in document["asset"]:
        if key not in new and check["asset"][key] != document["asset"][key]:
            raise SystemExit("member %s changed" % key)
    if open(ASSET, encoding="utf-8").read() != text:
        raise SystemExit("asset changed while writing; retry")
    open(ASSET, "w", encoding="utf-8").write(result)
    index = open(INDEX, encoding="utf-8").read()
    at = index.index('"id": "folding-deck-chair"')
    row_end = index.index('"poses"', at)
    row = index[at:row_end]
    row = re.sub(r'"heightM": [0-9.]+', '"heightM": %s' % DIMENSIONS["heightM"], row)
    row = re.sub(r'"widthM": [0-9.]+', '"widthM": %s' % DIMENSIONS["widthM"], row)
    if open(INDEX, encoding="utf-8").read() != index:
        raise SystemExit("index changed while writing; retry")
    if row != index[at:row_end]:
        open(INDEX, "w", encoding="utf-8").write(index[:at] + row + index[row_end:])
    print("applied root members:", ", ".join(new))


main()
