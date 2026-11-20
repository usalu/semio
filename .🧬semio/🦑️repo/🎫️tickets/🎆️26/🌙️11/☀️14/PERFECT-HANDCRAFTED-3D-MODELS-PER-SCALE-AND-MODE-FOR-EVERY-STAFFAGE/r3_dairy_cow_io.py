"""🧷 Member-level text splicing of the dairy cow catalog file in the file's own formatting style (the style `volumes-insert.py` writes): only the named members of `asset` are replaced, every other byte stays."""
import json
import re

ASSET = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/dairy-cow/🔣️.json"


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
        rows = [inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1) for key, item in value.items()]
        return "{\n" + ",\n".join(rows) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def expanded(value, depth):
    """📜 The fully expanded two-space style every member but `volumes` is written in."""
    return json.dumps(value, ensure_ascii=False, indent=2).replace("\n", "\n" + "  " * depth)


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


def member_range(text, key):
    found = re.search(r'\n    "' + key + r'"\s*:\s*', text)
    if not found:
        raise SystemExit("no member " + key)
    return found.start() + 1, member_end(text, found.end())


def read():
    text = open(ASSET, encoding="utf-8").read()
    return text, json.loads(text)["asset"]


def write(change):
    """✍️ Re-reads the file, lets `change(asset)` return { member: new value }, replaces exactly those members and verifies that nothing else changed."""
    text, asset = read()
    before = json.loads(text)
    members = change(asset)
    for key, value in members.items():
        a, b = member_range(text, key)
        text = text[:a] + '    "' + key + '": ' + (dump if key == "volumes" else expanded)(value, 2) + text[b:]
    after = json.loads(text)
    for key in before["asset"]:
        if key not in members and before["asset"][key] != after["asset"][key]:
            raise SystemExit("unexpected change in " + key)
    if list(before["asset"]) != list(after["asset"]):
        raise SystemExit("member order changed")
    for key, value in members.items():
        if after["asset"][key] != value:
            raise SystemExit("member not written: " + key)
    open(ASSET, "w", encoding="utf-8").write(text)
    return sorted(members)
