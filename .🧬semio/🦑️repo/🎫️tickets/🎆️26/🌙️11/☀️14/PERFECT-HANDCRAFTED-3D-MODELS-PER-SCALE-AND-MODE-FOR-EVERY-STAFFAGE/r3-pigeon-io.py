"""🧷 Round 3 pigeon: replaces whole members of the pigeon asset (`asset.<key>`) in a JSON document, every other byte untouched, in the style each member already has; the target is re-read right before writing.

Usage as module: text = put(text, members) ; CATALOG is the catalog file, CANDIDATE the trial document under 🗑️generated/r3-pigeon.
"""
import glob
import json
import re

CATALOG = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/pigeon/🔣️.json")[0]
T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
CANDIDATE = T + "/🗑️generated/r3-pigeon/🔣️.json"
ORIG = T + "/r3-pigeon-orig.json"


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value, style):
    if not isinstance(value, list):
        return False
    if all(scalar(item) for item in value):
        return style != "expanded" or len(value) == 0
    return False


def dump(value, depth, style):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        rows = [inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1, style) for key, item in value.items()]
        return "{\n" + ",\n".join(rows) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value, style):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1, style) for item in value) + "\n" + pad + "]"
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


def span(text, key):
    found = re.search(r'\n    "' + key + r'"\s*:\s*', text)
    if not found:
        raise SystemExit("no member " + key)
    return found.end(), member_end(text, found.end())


def style_of(text, key):
    a, b = span(text, key)
    old = json.loads(text[a:b])
    for style in ("scalars", "expanded"):
        if dump(old, 2, style) == text[a:b]:
            return style
    return None


def put(text, members):
    before = json.loads(text)["asset"]
    for key, value in members.items():
        a, b = span(text, key)
        style = style_of(text, key)
        if style is None:
            raise SystemExit("unknown style of member " + key)
        text = text[:a] + dump(value, 2, style) + text[b:]
    after = json.loads(text)["asset"]
    if list(after) != list(before):
        raise SystemExit("member order changed")
    for key in before:
        if key not in members and after[key] != before[key]:
            raise SystemExit("foreign member changed: " + key)
    return text


if __name__ == "__main__":
    text = open(CATALOG, encoding="utf-8").read()
    for key in ("summary", "provenance", "dimensions", "rig", "anchors", "representations", "poses", "thermal", "structure"):
        print(key, style_of(text, key))
