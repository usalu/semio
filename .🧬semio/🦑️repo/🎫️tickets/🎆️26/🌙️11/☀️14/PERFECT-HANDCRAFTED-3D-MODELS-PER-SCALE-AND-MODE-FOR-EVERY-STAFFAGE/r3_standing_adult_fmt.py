"""🧷 Formatting helpers of the round-3 standing-adult scripts: the catalog's JSON style (two-space indent, scalar lists inline) and member replacement inside the asset file."""
import json
import re


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
        elif depth == 0 and char in ",\n":
            return index
        index += 1
    raise SystemExit("unbalanced asset file")


def member_range(text, indent, key):
    found = re.search(r"\n" + indent + r'"' + key + r'"\s*:\s*', text)
    if not found:
        return None
    return found.start() + 1, member_end(text, found.end())


def member_text(key, value, depth):
    return "  " * depth + json.dumps(key) + ": " + dump(value, depth)
