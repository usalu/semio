"""🧷 Replaces whole members of the hedgehog asset (`asset.<key>`) in the catalog file, every other byte untouched; the file is re-read right before writing.

Usage as module: put(members: dict) ; load() -> asset dict. Style: two-space indent, scalars and scalar lists inline (the style the file has).
"""
import glob
import json
import re

FILE = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/hedgehog/🔣️.json")[0]


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value, style):
    if not isinstance(value, list):
        return False
    if all(scalar(item) for item in value):
        return style != "expanded" or len(value) == 0
    return style == "compact" and all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160


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


def load():
    return json.load(open(FILE, encoding="utf-8"))["asset"]


def style_of(text, key):
    a, b = span(text, key)
    old = json.loads(text[a:b])
    for style in ("compact", "scalars", "expanded"):
        if dump(old, 2, style) == text[a:b]:
            return style
    return None


def candidate(members, path):
    """🧪 Writes the catalog document with `members` laid over `asset` to `path` (for r3-validate.ts --file and r3-2d.ts --file)."""
    doc = json.load(open(FILE, encoding="utf-8"))
    doc["asset"].update(members)
    json.dump(doc, open(path, "w", encoding="utf-8"), ensure_ascii=False)


def put(members):
    text = open(FILE, encoding="utf-8").read()
    before = json.loads(text)["asset"]
    for key, value in members.items():
        a, b = span(text, key)
        style = style_of(text, key) or "scalars"
        text = text[:a] + dump(value, 2, style) + text[b:]
    after = json.loads(text)["asset"]
    if list(after) != list(before):
        raise SystemExit("member order changed")
    for key in before:
        if key not in members and after[key] != before[key]:
            raise SystemExit("foreign member changed: " + key)
    open(FILE, "w", encoding="utf-8").write(text)


if __name__ == "__main__":
    text = open(FILE, encoding="utf-8").read()
    for key in ("dimensions", "rig", "anchors", "representations", "poses", "thermal", "structure", "summary"):
        print(key, style_of(text, key))
