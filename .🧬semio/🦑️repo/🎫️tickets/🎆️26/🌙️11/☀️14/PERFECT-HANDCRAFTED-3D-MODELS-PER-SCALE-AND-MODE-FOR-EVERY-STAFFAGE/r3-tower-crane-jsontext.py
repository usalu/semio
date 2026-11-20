"""🧾 Text surgery on a catalog asset file for `machinery/tower-crane`: finds one member of `asset` by key, serialises a new value in the file's own style (one space per level, arrays of scalars and arrays of scalar arrays inline) and replaces only that member's text.

@see ./volumes-insert.py
"""
import json


def scalar(v):
    return not isinstance(v, (dict, list))


def inline(v):
    return isinstance(v, list) and (all(scalar(i) for i in v) or all(isinstance(i, list) and all(scalar(l) for l in i) for i in v))


def dump(v, d):
    pad, inner = " " * d, " " * (d + 1)
    if isinstance(v, dict):
        if not v:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(k, ensure_ascii=False) + ": " + dump(x, d + 1) for k, x in v.items()) + "\n" + pad + "}"
    if isinstance(v, list):
        if not v:
            return "[]"
        if inline(v):
            return json.dumps(v, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(x, d + 1) for x in v) + "\n" + pad + "]"
    return json.dumps(v, ensure_ascii=False)


def span(text, key):
    head = '\n  "' + key + '": '
    start = text.index(head) + len(head)
    depth, i, quoted = 0, start, False
    while True:
        c = text[i]
        if quoted:
            if c == "\\":
                i += 1
            elif c == '"':
                quoted = False
        elif c == '"':
            quoted = True
        elif c in "[{":
            depth += 1
        elif c in "]}":
            depth -= 1
            if depth == 0:
                return start, i + 1
        elif depth == 0 and c in ",\n":
            return start, i
        i += 1


def replace(text, key, value):
    a, b = span(text, key)
    return text[:a] + dump(value, 2) + text[b:]
