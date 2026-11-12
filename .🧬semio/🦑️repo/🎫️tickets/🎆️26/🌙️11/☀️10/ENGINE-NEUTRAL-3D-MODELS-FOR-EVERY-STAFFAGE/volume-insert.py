"""🧵 Splices an authored volume into a catalog asset file without touching any other byte.

Usage: python3 volume-insert.py <authored-volume.json> <asset 🔣️.json>
The authored file is either a Volume object or { "volume": Volume }.
The asset file is re-read immediately before writing; only the `volume` member of `asset` changes.
"""
import json
import re
import sys


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (
            all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160
        )
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


def main():
    if len(sys.argv) != 3:
        raise SystemExit(__doc__)
    authored = json.load(open(sys.argv[1], encoding="utf-8"))
    volume = authored.get("volume", authored)
    if volume.get("status") != "authored" or not volume.get("solids"):
        raise SystemExit("authored volume with solids required")
    text = open(sys.argv[2], encoding="utf-8").read()
    document = json.loads(text)
    keys = list(document["asset"].keys())
    indent = re.search(r'\n( +)"id"\s*:', text).group(1)
    depth = len(indent) // 2
    body = '"volume": ' + dump(volume, depth)
    existing = re.search(r'\n' + indent + r'"volume"\s*:\s*', text)
    if existing:
        end = member_end(text, existing.end())
        result = text[: existing.start() + 1] + indent + body + text[end:]
    else:
        anchor = "presentation" if "presentation" in keys else None
        if anchor is None:
            raise SystemExit("asset has no presentation member to anchor the volume before")
        place = re.search(r'\n' + indent + r'"presentation"\s*:', text)
        result = text[: place.start() + 1] + indent + body + ",\n" + text[place.start() + 1 :]
    check = json.loads(result)
    if check["asset"]["volume"] != volume:
        raise SystemExit("splice did not round-trip")
    without = dict(check["asset"])
    without.pop("volume")
    before = dict(document["asset"])
    before.pop("volume", None)
    if without != before or {k: v for k, v in check.items() if k != "asset"} != {k: v for k, v in document.items() if k != "asset"}:
        raise SystemExit("splice changed other members")
    open(sys.argv[2], "w", encoding="utf-8").write(result)
    print("volume spliced:", len(volume["solids"]), "solids")


main()
