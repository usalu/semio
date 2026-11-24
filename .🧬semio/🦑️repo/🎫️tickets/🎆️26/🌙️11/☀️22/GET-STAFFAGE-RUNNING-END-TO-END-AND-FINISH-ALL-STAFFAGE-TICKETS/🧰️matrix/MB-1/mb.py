"""🧷️ Reading and writing of a asset file for the repairs of batch MB-1 (the reader and writer of the silhouette batch SC, for every category): the file is parsed with the text span of every value, and a save writes the original text of every value that did not change, so that a repair changes only the lines it means to change whatever layout the file has."""
import json, os, re, sys

import glob
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
_WS = re.compile(r"[ \t\n\r]*")
_STR = re.compile(r'"(?:[^"\\]|\\.)*"')
_NUM = re.compile(r"-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?")


class Node:
    """🌿️ A parsed value with its span in the text and its children."""
    __slots__ = ("start", "end", "value", "items")

    def __init__(self, start, end, value, items=None):
        self.start, self.end, self.value, self.items = start, end, value, items


def _parse(t, i):
    i = _WS.match(t, i).end()
    c = t[i]
    if c == "{":
        start, items, value = i, [], {}
        i = _WS.match(t, i + 1).end()
        if t[i] == "}":
            return Node(start, i + 1, value, items), i + 1
        while True:
            i = _WS.match(t, i).end()
            m = _STR.match(t, i)
            key = json.loads(m.group())
            i = _WS.match(t, m.end()).end() + 1
            node, i = _parse(t, i)
            items.append((key, node))
            value[key] = node.value
            i = _WS.match(t, i).end()
            if t[i] == "}":
                return Node(start, i + 1, value, items), i + 1
            i += 1
    if c == "[":
        start, items, value = i, [], []
        i = _WS.match(t, i + 1).end()
        if t[i] == "]":
            return Node(start, i + 1, value, items), i + 1
        while True:
            node, i = _parse(t, i)
            items.append(node)
            value.append(node.value)
            i = _WS.match(t, i).end()
            if t[i] == "]":
                return Node(start, i + 1, value, items), i + 1
            i += 1
    if c == '"':
        m = _STR.match(t, i)
        return Node(i, m.end(), json.loads(m.group())), m.end()
    for word, v in (("true", True), ("false", False), ("null", None)):
        if t.startswith(word, i):
            return Node(i, i + len(word), v), i + len(word)
    m = _NUM.match(t, i)
    s = m.group()
    return Node(i, m.end(), float(s) if any(x in s for x in ".eE") else int(s)), m.end()


def _same(a, b):
    if isinstance(a, bool) or isinstance(b, bool) or a is None or b is None:
        return a is b
    if isinstance(a, (int, float)) and isinstance(b, (int, float)):
        return a == b
    if type(a) is not type(b):
        return False
    if isinstance(a, dict):
        return list(a.keys()) == list(b.keys()) and all(_same(a[k], b[k]) for k in a)
    if isinstance(a, list):
        return len(a) == len(b) and all(_same(x, y) for x, y in zip(a, b))
    return a == b


def _fresh(value, multiline, indent, unit, colon):
    if not multiline:
        return json.dumps(value, ensure_ascii=False, separators=(",", colon)) if colon == ":" else json.dumps(value, ensure_ascii=False)
    return json.dumps(value, ensure_ascii=False, indent=unit).replace("\n", "\n" + indent)


def _emit(t, node, new, unit):
    if _same(node.value, new):
        return t[node.start:node.end]
    body = t[node.start:node.end]
    container = node.items is not None and type(node.value) is type(new) and len(node.items) > 0
    if not container:
        line = t.rfind("\n", 0, node.start) + 1
        indent = re.match(r"[ \t]*", t[line:]).group()
        return _fresh(new, isinstance(new, (dict, list)) and unit is not None and len(new) > 0, indent, unit, ": ")
    first = node.items[0][1] if isinstance(new, dict) else node.items[0]
    lead = t[node.start + 1:(node.start + 1 + len(_WS.match(t, node.start + 1).group()))]
    multiline = "\n" in lead
    child_indent = lead.split("\n")[-1] if multiline else ""
    tail = t[node.items[-1][1].end if isinstance(new, dict) else node.items[-1].end:node.end - 1]
    colon = ": "
    if isinstance(new, dict):
        k0 = node.items[0]
        between = t[node.start + 1 + len(lead):k0[1].start]
        colon = between[between.index(":", len(json.dumps(k0[0], ensure_ascii=False)) - 1 if False else between.rindex(":")):]
    comma = ("," + lead) if multiline else ("," if (len(node.items) > 1 and t[(node.items[0][1].end if isinstance(new, dict) else node.items[0].end):][:2] not in (", ",)) else ", ")
    parts = []
    if isinstance(new, dict):
        old = dict(node.items)
        for k, v in new.items():
            text = _emit(t, old[k], v, unit) if k in old else _fresh(v, multiline and isinstance(v, (dict, list)) and len(v) > 0, child_indent, unit, colon)
            parts.append(json.dumps(k, ensure_ascii=False) + colon + text)
    else:
        used = [False] * len(node.items)
        for j, v in enumerate(new):
            pick = j if j < len(node.items) and not used[j] and _same(node.items[j].value, v) else next((n for n, x in enumerate(node.items) if not used[n] and _same(x.value, v)), None)
            if pick is None and j < len(node.items) and not used[j] and type(node.items[j].value) is type(v) and isinstance(v, (dict, list)) and not any(_same(x.value, v) for x in node.items):
                idv = v.get("id") if isinstance(v, dict) else None
                if idv is None or (isinstance(node.items[j].value, dict) and node.items[j].value.get("id") == idv):
                    used[j] = True
                    parts.append(_emit(t, node.items[j], v, unit))
                    continue
            if pick is None and isinstance(v, dict) and "id" in v:
                pick = next((n for n, x in enumerate(node.items) if not used[n] and isinstance(x.value, dict) and x.value.get("id") == v["id"]), None)
            if pick is not None:
                used[pick] = True
                parts.append(_emit(t, node.items[pick], v, unit))
            else:
                parts.append(_fresh(v, multiline and isinstance(v, (dict, list)) and len(v) > 0, child_indent, unit, colon))
    return body[0] + lead + comma.join(parts) + tail + body[-1]


def path_of(name):
    """📄️ The catalog file of a component id, or the given json path."""
    return name if name.endswith(".json") else glob.glob(f"{CATALOG}/*/{name}/🔣️.json")[0]


def load(name):
    """📥️ The text of the file and its parsed document."""
    text = open(path_of(name), encoding="utf-8").read()
    return text, json.loads(text)


def save(name, text, doc):
    """📤️ Writes the document over the text it was read from, keeping the text of every unchanged value; re-reads the file first and refuses when it changed meanwhile. Returns whether the file changed."""
    path = path_of(name)
    if open(path, encoding="utf-8").read() != text:
        raise SystemExit(f"{name}: the file changed since it was read; run the repair again")
    root, _ = _parse(text, 0)
    lines = text.split("\n")
    unit = None if len(lines) < 3 else len(lines[1]) - len(lines[1].lstrip())
    out = text[:root.start] + _emit(text, root, doc, unit) + text[root.end:]
    if not _same(json.loads(out), doc):
        raise SystemExit(f"{name}: the written text does not parse back to the document")
    if out == text:
        print(f"{name}: already repaired, nothing to write")
        return False
    tmp = path + ".mb1-tmp"
    open(tmp, "w", encoding="utf-8").write(out)
    os.replace(tmp, path)
    print(f"{name}: written ({len(text)} -> {len(out)} bytes)")
    return True


def r6(x):
    """🔢️ A coordinate rounded to the micrometre."""
    return round(float(x), 6)


def shapes(doc, scale, view):
    """✏️ The drawn shapes of a view at a scale."""
    return doc["asset"]["representations"][str(scale)]["geometry"][view]


def shape(doc, scale, view, sid):
    """✏️ One drawn shape by id."""
    return next(s for s in shapes(doc, scale, view) if s["id"] == sid)


def solids(doc, scale, which="body"):
    """🧊️ The solids of a volume list at a scale."""
    return doc["asset"]["volumes"][str(scale)][which]


def solid(doc, scale, sid, which="body"):
    """🧊️ One solid by id."""
    return next(s for s in solids(doc, scale, which) if s["id"] == sid)


def put(items, item, after=None, before=None):
    """➕️ Adds an item with an id to a list, or replaces the one of the same id in place (idempotent); a new one goes after or before the named id, else to the end."""
    for n, x in enumerate(items):
        if x.get("id") == item["id"]:
            items[n] = item
            return
    if after is not None:
        items.insert(next(n for n, x in enumerate(items) if x.get("id") == after) + 1, item)
    elif before is not None:
        items.insert(next(n for n, x in enumerate(items) if x.get("id") == before), item)
    else:
        items.append(item)


def drop(items, sid):
    """➖️ Removes the item of an id when it is there (idempotent)."""
    items[:] = [x for x in items if x.get("id") != sid]
