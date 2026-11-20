"""🖋️ Writes the root fixes of `humans/person-on-crutches` into the catalog file: the members `dimensions`, `materials`, `rig`, `anchors`, `representations`, `poses`, `clips` are taken from the validated candidate `🗑️generated/r3-person-on-crutches/🔣️.json` (built by `r3-person-on-crutches-run.sh`). The catalog file is re-read right before writing, only those members are replaced textually in the file's own indent-2 style, every other member (`volumes` included, spliced by `volumes-insert.py`) stays byte-identical, and the member order is checked.

Usage: python3 r3-person-on-crutches-write.py [--write]   (without --write: dry run that prints the changed members)
@see ./r3-humans-person-on-crutches.md
"""
import glob
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ASSET = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/person-on-crutches/🔣️.json")[0]
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-person-on-crutches", "🔣️.json")
KEYS = ["dimensions", "materials", "rig", "anchors", "representations", "poses", "clips"]


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


candidate = json.load(open(CANDIDATE, encoding="utf-8"))["asset"]
text = open(ASSET, encoding="utf-8").read()
before = json.loads(text)
indent = re.search(r'\n( +)"id"\s*:', text).group(1)
depth = len(indent) // 2
changed = [k for k in KEYS if before["asset"][k] != candidate[k]]
for key in changed:
    start, end = member_range(text, indent, key)
    body = json.dumps(candidate[key], ensure_ascii=False, indent=2).replace("\n", "\n" + "  " * depth)
    text = text[:start] + indent + '"%s": ' % key + body + text[end:]
check = json.loads(text)
for key in before["asset"]:
    if key not in changed and check["asset"][key] != before["asset"][key]:
        raise SystemExit("member %s changed unexpectedly" % key)
if list(check["asset"].keys()) != list(before["asset"].keys()):
    raise SystemExit("member order changed")
print("changed members:", ", ".join(changed) or "none")
if "--write" in sys.argv and changed:
    open(ASSET, "w", encoding="utf-8").write(text)
    print("written", ASSET)
