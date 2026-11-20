"""🪡 Member-wise splice helpers for the asset file of the runner with blade prosthesis: replaces whole members of `asset` textually (same serializer as `volumes-insert.py`), re-reading the file right before writing and leaving every other member byte-identical.

Usage: `python3 r3-runner-splice.py <member>...` copies those members from the candidate document `🗑️generated/r3-runner-with-blade-prosthesis/🔣️.json`; `--manifest <powerW>` states dimensions, power, poses and distinction in the catalog manifest row; `--probe` shows which members the serializer reproduces byte-identically.
"""
import json
import re
import sys

ASSET = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/runner-with-blade-prosthesis/🔣️.json"


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


CANDIDATE = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/🗑️generated/r3-runner-with-blade-prosthesis/🔣️.json"
MANIFEST = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json"


def from_candidate(keys):
    """Copies the named members of the candidate document into the catalog asset file."""
    candidate = json.load(open(CANDIDATE, encoding="utf-8"))["asset"]

    def change(asset):
        for key in keys:
            asset[key] = candidate[key]
        return keys

    return replace_members(change)


def manifest_row(power):
    """States dimensions, power, poses and the distinction text of the asset in its row of the catalog manifest; nothing else in the manifest changes."""
    asset = json.load(open(ASSET, encoding="utf-8"))["asset"]
    text = open(MANIFEST, encoding="utf-8").read()
    at = text.index('"id": "runner-with-blade-prosthesis"')
    end = text.index('"completion"', at)
    row = text[at:end]
    for key in ("heightM", "widthM"):
        row = re.sub(r'("%s"\s*:\s*)[0-9.]+' % key, lambda m: m.group(1) + json.dumps(asset["dimensions"][key]), row)
    row = re.sub(r'("powerW"\s*:\s*)[0-9.]+', lambda m: m.group(1) + json.dumps(power), row)
    poses = [pose["id"] for pose in asset["poses"] if not pose.get("key")]
    row = re.sub(r'"poses": \[[^\]]*\]', lambda m: '"poses": [\n' + ",\n".join("        " + json.dumps(p) for p in poses) + "\n      ]", row)
    for lang in ("en", "de"):
        row = re.sub(r'("distinction": \{.*?"%s": )"(?:[^"\\]|\\.)*"' % lang, lambda m: m.group(1) + json.dumps(asset["summary"][lang], ensure_ascii=False), row, flags=re.S)
    open(MANIFEST, "w", encoding="utf-8").write(text[:at] + row + text[end:])
    return row


if __name__ == "__main__":
    if sys.argv[1:] == ["--probe"]:
        text = open(ASSET, encoding="utf-8").read()
        document = json.loads(text)
        indent = re.search(r'\n( +)"id"\s*:', text).group(1)
        print(styles(text, document, indent, [k for k in document["asset"] if isinstance(document["asset"][k], (dict, list))]))
    elif sys.argv[1:2] == ["--manifest"]:
        print(manifest_row(float(sys.argv[2])))
    else:
        print(from_candidate(sys.argv[1:]))
