"""🪡 Round 3 English oak: writes the root members `rig`, `anchors` and `representations` of `🗑️generated/r3-english-oak/roots.json` into the catalog asset file in the file's own formatting; every other member stays byte-identical and the file is re-read right before writing.

Usage: python3 r3-english-oak-splice.py <roots.json> <asset 🔣️.json> [--check]
"""
import json
import re
import sys


def scalar(value):
    return not isinstance(value, (dict, list))


def number(value):
    text = json.dumps(value)
    if isinstance(value, float) and "e" in text and 1e-7 <= abs(value) < 1:
        from decimal import Decimal

        return format(Decimal(text), "f")
    return text


def flat(value):
    if scalar(value):
        return number(value) if isinstance(value, (int, float)) and not isinstance(value, bool) else json.dumps(value, ensure_ascii=False)
    return "[" + ", ".join(flat(item) for item in value) + "]"


def inline(value):
    return isinstance(value, list) and (all(scalar(item) for item in value) or all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value))


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
            return flat(value)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1) for item in value) + "\n" + pad + "]"
    return flat(value)


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
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    roots = json.load(open(args[0], encoding="utf-8"))
    text = open(args[1], encoding="utf-8").read()
    document = json.loads(text)
    indent = re.search(r'\n( +)"id"\s*:', text).group(1)
    result = text
    for key, value in roots.items():
        found = re.search(r'\n' + indent + r'"' + key + r'"\s*:\s*', result)
        if not found:
            raise SystemExit("asset has no member " + key)
        end = member_end(result, found.end())
        if "--check" in sys.argv:
            same = result[found.end():end] == dump(document["asset"][key], len(indent) // 2)
            print(key, "formatting round-trips" if same else "FORMATTING DIFFERS")
        result = result[: found.end()] + dump(value, len(indent) // 2) + result[end:]
    check = json.loads(result)
    for key in check["asset"]:
        if check["asset"][key] != (roots[key] if key in roots else document["asset"][key]):
            raise SystemExit("splice changed member " + key)
    if list(check["asset"]) != list(document["asset"]):
        raise SystemExit("member order changed")
    if "--check" in sys.argv:
        return
    if open(args[1], encoding="utf-8").read() != text:
        raise SystemExit("asset changed while splicing; retry")
    open(args[1], "w", encoding="utf-8").write(result)
    print("spliced:", ", ".join(roots))


main()
