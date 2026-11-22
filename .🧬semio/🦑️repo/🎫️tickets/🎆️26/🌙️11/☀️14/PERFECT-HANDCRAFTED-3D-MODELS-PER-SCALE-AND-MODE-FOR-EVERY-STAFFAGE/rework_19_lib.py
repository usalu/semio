"""🧰 Rework 19 helpers: read a volumes source and a catalog asset, and write single root members of the catalog asset back as a textual splice (every other member stays byte-identical, the file is re-read right before the write)."""
import json
import re

T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components"


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(i) for i in value) or (all(isinstance(i, list) and all(scalar(x) for x in i) for i in value) and len(json.dumps(value)) <= 160)
    return False


def dump(value, depth):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(k, ensure_ascii=False) + ": " + dump(v, depth + 1) for k, v in value.items()) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(i, depth + 1) for i in value) + "\n" + pad + "]"
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


def source_path(slug):
    return T + "/volumes/components/" + slug + ".json"


def catalog_path(slug):
    return CATALOG + "/" + slug + "/🔣️.json"


def load_source(slug):
    return json.load(open(source_path(slug), encoding="utf-8"))


def save_source(slug, doc):
    json.dump(doc, open(source_path(slug), "w", encoding="utf-8"), ensure_ascii=False, indent=1)


def load_asset(slug):
    return json.load(open(catalog_path(slug), encoding="utf-8"))["asset"]


def write_members(slug, members):
    """✍️ Replaces the named root members of the catalog asset (objects or arrays) in the file text."""
    path = catalog_path(slug)
    text = open(path, encoding="utf-8").read()
    indent = re.search(r'\n( +)"id"\s*:', text).group(1)
    depth = len(indent) // 2
    result = text
    for key, value in members.items():
        found = re.search(r'\n' + indent + r'"' + key + r'"\s*:\s*', result)
        if not found:
            raise SystemExit("no member " + key)
        end = member_end(result, found.end())
        result = result[: found.end()] + dump(value, depth) + result[end:]
    check = json.loads(result)["asset"]
    before = json.loads(text)["asset"]
    for key in before:
        if key in members:
            if check[key] != members[key]:
                raise SystemExit("member did not round-trip: " + key)
        elif check[key] != before[key]:
            raise SystemExit("other member changed: " + key)
    if open(path, encoding="utf-8").read() != text:
        raise SystemExit("asset changed while writing; retry")
    open(path, "w", encoding="utf-8").write(result)
    print("written:", ", ".join(members))
