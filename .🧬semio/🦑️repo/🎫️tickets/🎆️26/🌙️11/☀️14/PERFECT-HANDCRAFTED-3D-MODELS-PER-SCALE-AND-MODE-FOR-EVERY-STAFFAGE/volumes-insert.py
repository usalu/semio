"""🧵 Splices the authored `volumes` of a source into a catalog asset file and removes the old `volume` member in the same write; every other member stays byte-identical.

Usage: python3 volumes-insert.py <source.json> <asset 🔣️.json> [--allow-incomplete]
The source is { "volumes": { "50", "100", "200", "500" } }. Authored scales need non-empty body, thermal, structure and decisions (unless --allow-incomplete); unavailable scales only status and denominator.
The asset file is re-read immediately before writing. The new member sits where `volume` was (else where `volumes` was, else before `presentation`).
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


def member_range(text, indent, key):
    found = re.search(r'\n' + indent + r'"' + key + r'"\s*:\s*', text)
    if not found:
        return None
    return found.start() + 1, member_end(text, found.end())


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    if len(args) != 2:
        raise SystemExit(__doc__)
    incomplete = "--allow-incomplete" in sys.argv
    volumes = json.load(open(args[0], encoding="utf-8"))["volumes"]
    if sorted(volumes) != ["100", "200", "50", "500"]:
        raise SystemExit("volumes needs exactly the scales 50, 100, 200, 500")
    for scale, v in volumes.items():
        if v.get("denominator") != int(scale):
            raise SystemExit("scale %s: denominator mismatch" % scale)
        if v.get("status") == "authored" and not incomplete:
            for key in ("body", "thermal", "structure", "decisions"):
                if not v.get(key):
                    raise SystemExit("scale %s: %s is empty (use --allow-incomplete for work in progress)" % (scale, key))
        elif v.get("status") not in ("authored", "unavailable"):
            raise SystemExit("scale %s: bad status" % scale)
    text = open(args[1], encoding="utf-8").read()
    document = json.loads(text)
    indent = re.search(r'\n( +)"id"\s*:', text).group(1)
    depth = len(indent) // 2
    body = '"volumes": ' + dump(volumes, depth)
    old = member_range(text, indent, "volume")
    new = member_range(text, indent, "volumes")
    if old and new:
        raise SystemExit("asset has both volume and volumes")
    if old or new:
        start, end = old or new
        result = text[:start] + indent + body + text[end:]
    else:
        place = re.search(r'\n' + indent + r'"presentation"\s*:', text)
        if not place:
            raise SystemExit("asset has no presentation member to anchor before")
        result = text[: place.start() + 1] + indent + body + ",\n" + text[place.start() + 1 :]
    check = json.loads(result)
    if check["asset"]["volumes"] != volumes or "volume" in check["asset"]:
        raise SystemExit("splice did not round-trip")
    before = {k: v for k, v in document["asset"].items() if k not in ("volume", "volumes")}
    after = {k: v for k, v in check["asset"].items() if k != "volumes"}
    if before != after or {k: v for k, v in check.items() if k != "asset"} != {k: v for k, v in document.items() if k != "asset"}:
        raise SystemExit("splice changed other members")
    if list(before) != [k for k in check["asset"] if k != "volumes"]:
        raise SystemExit("member order changed")
    fresh = open(args[1], encoding="utf-8").read()
    if fresh != text:
        raise SystemExit("asset changed while splicing; retry")
    open(args[1], "w", encoding="utf-8").write(result)
    print("volumes spliced:", ", ".join("1:%s %s" % (k, v["status"]) for k, v in sorted(volumes.items(), key=lambda x: int(x[0]))))


main()
