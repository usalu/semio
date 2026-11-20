"""🚚 Writes the root fixes of `humans/courier-with-hand-truck` from the validated candidate document (`🗑️generated/r3-courier-with-hand-truck/🔣️.json`, built by r3-courier-roots.ts, r3-courier-build.py and r3-courier-draw2d.ts) into the catalog asset: every member of `asset` except `volumes` that differs is replaced in place in the file's own formatting, member order kept, the file re-read right before writing. `volumes` follows with volumes-insert.py. Usage: python3 r3-courier-apply.py [--dry]"""
import json
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/courier-with-hand-truck/🔣️.json"
CANDIDATE = os.path.join(T, "🗑️generated", "r3-courier-with-hand-truck", "🔣️.json")


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160)
    return False


def dump(value, depth):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1) for key, item in value.items()) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


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
    raise SystemExit("unbalanced asset file")


text = open(CATALOG, encoding="utf-8").read()
document = json.loads(text)
candidate = json.load(open(CANDIDATE, encoding="utf-8"))["asset"]
if list(candidate) != list(document["asset"]):
    raise SystemExit("member order of the candidate differs")
result = text
changed = []
for key, value in candidate.items():
    if key == "volumes" or document["asset"][key] == value:
        continue
    found = re.search(r'\n    "' + key + r'"\s*:\s*', result)
    end = member_end(result, found.end())
    if dump(document["asset"][key], 2) != result[found.end():end]:
        print("note: member", key, "was not in the insert tool's formatting; rewritten in it")
    result = result[: found.end()] + dump(value, 2) + result[end:]
    changed.append(key)
check = json.loads(result)
if {k: v for k, v in check["asset"].items() if k != "volumes"} != {k: v for k, v in candidate.items() if k != "volumes"} or check["asset"]["volumes"] != document["asset"]["volumes"] or list(check["asset"]) != list(document["asset"]):
    raise SystemExit("the edit did not round-trip")
print("members replaced:", ", ".join(changed))
if "--dry" not in sys.argv:
    if open(CATALOG, encoding="utf-8").read() != text:
        raise SystemExit("asset changed while editing; retry")
    open(CATALOG, "w", encoding="utf-8").write(result)
    print("written", CATALOG)
