"""Format-preserving JSON edits: locate an object/array by key path and append a member/item textually (keeps the file's own formatting elsewhere)."""
import json, re

WS = " \t\r\n"


def skip(t, i):
    while i < len(t) and t[i] in WS:
        i += 1
    return i


def parse(t, i=0):
    """Returns (end, node); node is {'span':(s,e), 'kind':..., 'members': {key:node}|None, 'items': [node]|None}."""
    i = skip(t, i)
    s = i
    c = t[i]
    if c == "{":
        i += 1
        members = {}
        order = []
        while True:
            i = skip(t, i)
            if t[i] == "}":
                return i + 1, {"span": (s, i + 1), "kind": "object", "members": members, "order": order, "items": None}
            if t[i] == ",":
                i += 1
                continue
            ke, _ = parse(t, i)
            key = json.loads(t[i:ke])
            i = skip(t, ke)
            assert t[i] == ":"
            i, node = parse(t, i + 1)
            members[key] = node
            order.append(key)
    if c == "[":
        i += 1
        items = []
        while True:
            i = skip(t, i)
            if t[i] == "]":
                return i + 1, {"span": (s, i + 1), "kind": "array", "members": None, "items": items, "order": []}
            if t[i] == ",":
                i += 1
                continue
            i, node = parse(t, i)
            items.append(node)
    if c == '"':
        i += 1
        while t[i] != '"':
            i += 2 if t[i] == "\\" else 1
        return i + 1, {"span": (s, i + 1), "kind": "scalar", "members": None, "items": None, "order": []}
    m = re.compile(r"-?\d+(\.\d+)?([eE][+-]?\d+)?|true|false|null").match(t, i)
    return m.end(), {"span": (s, m.end()), "kind": "scalar", "members": None, "items": None, "order": []}


def locate(t, path):
    _, root = parse(t)
    node = root
    for k in path:
        node = node["members"][k]
    return node


def indent_of(t, pos):
    ls = t.rfind("\n", 0, pos) + 1
    return t[ls:pos] if t[ls:pos].strip() == "" else re.match(r"\s*", t[ls:]).group(0)


def add_member(t, path, key, value, compact=False):
    node = locate(t, path)
    assert node["kind"] == "object"
    s, e = node["span"]
    if node["order"]:
        last = node["members"][node["order"][-1]]
        ls = t.rfind("\n", 0, t.index('"' + node["order"][-1] + '"', s)) + 1
        ind = re.match(r"\s*", t[ls:]).group(0)
        multiline = "\n" in t[s:e]
        pos = last["span"][1]
    else:
        ind, multiline, pos = indent_of(t, s) + "  ", "\n" in t[s:e], s + 1
    if compact or not multiline:
        body = json.dumps(value, ensure_ascii=False)
    else:
        body = json.dumps(value, indent=2, ensure_ascii=False).replace("\n", "\n" + ind)
    sep = "," if node["order"] else ""
    ins = sep + ("\n" + ind if multiline else " ") + json.dumps(key) + ": " + body
    if not node["order"] and multiline:
        ins += "\n" + indent_of(t, e - 1)
    return t[:pos] + ins + t[pos:]


def add_item(t, path, value):
    node = locate(t, path)
    assert node["kind"] == "array"
    s, e = node["span"]
    multiline = "\n" in t[s:e]
    if node["items"]:
        last = node["items"][-1]
        ind = indent_of(t, last["span"][0]) if multiline else ""
        pos = last["span"][1]
        body = json.dumps(value, ensure_ascii=False) if not multiline else json.dumps(value, indent=2, ensure_ascii=False).replace("\n", "\n" + ind)
        return t[:pos] + "," + ("\n" + ind if multiline else " ") + body + t[pos:]
    return t[: s + 1] + json.dumps(value, ensure_ascii=False) + t[s + 1:]
