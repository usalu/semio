"""🪡 Member-wise splice helpers for the jogger asset file: replaces whole members of `asset` textually (same serializer as `volumes-insert.py`), re-reading the file right before writing and leaving every other member byte-identical.

Usage as module: `from importlib import ...` is not needed; run `python3 r3-jogger-splice.py --probe` to see which members the serializer reproduces byte-identically.
"""
import json
import re
import sys

ASSET = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/jogger/🔣️.json"


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (
            all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160
        )
    return False


def dump(value, depth, style):
    if style == "plain":
        return json.dumps(value, ensure_ascii=False, indent=2).replace("\n", "\n" + "  " * depth)
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        rows = [inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1, style) for key, item in value.items()]
        return "{\n" + ",\n".join(rows) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1, style) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def member_end(text, start):
    depth, index, quoted = 0, start, False
    first = text[start]
    if first not in "{[":
        end = re.compile(r"[,\n]").search(text, start)
        return end.start()
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


def member_range(text, indent, key):
    found = re.search(r"\n" + indent + r'"' + key + r'"\s*:\s*', text)
    if not found:
        return None
    return found.start() + 1, member_end(text, found.end())


def styles(text, document, indent, keys):
    depth = len(indent) // 2
    out = {}
    for key in keys:
        r = member_range(text, indent, key)
        current = text[r[0] : r[1]]
        out[key] = next((s for s in ("compact", "plain") if indent + '"%s": ' % key + dump(document["asset"][key], depth, s) == current), None)
    return out


def replace_members(change):
    """Applies `change(asset) -> list of changed member keys` on a fresh read and writes only those members."""
    text = open(ASSET, encoding="utf-8").read()
    document = json.loads(text)
    indent = re.search(r'\n( +)"id"\s*:', text).group(1)
    depth = len(indent) // 2
    before = json.loads(text)
    keys = change(document["asset"])
    style = styles(text, before, indent, keys)
    for key in keys:
        r = member_range(text, indent, key)
        text = text[: r[0]] + indent + '"%s": ' % key + dump(document["asset"][key], depth, style[key] or "plain") + text[r[1] :]
    check = json.loads(text)
    for key in before["asset"]:
        if key not in keys and check["asset"][key] != before["asset"][key]:
            raise SystemExit("member %s changed unexpectedly" % key)
    if list(check["asset"].keys()) != list(before["asset"].keys()):
        raise SystemExit("member order changed")
    open(ASSET, "w", encoding="utf-8").write(text)
    return style


if __name__ == "__main__":
    text = open(ASSET, encoding="utf-8").read()
    document = json.loads(text)
    indent = re.search(r'\n( +)"id"\s*:', text).group(1)
    print(styles(text, document, indent, [k for k in document["asset"] if isinstance(document["asset"][k], (dict, list))]))
