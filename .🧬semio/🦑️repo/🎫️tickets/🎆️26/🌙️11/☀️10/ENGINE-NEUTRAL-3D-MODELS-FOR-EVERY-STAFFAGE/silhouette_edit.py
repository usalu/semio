"""Surgical text edits of catalog JSON for the silhouette ticket: objects and array elements are located by JSON path in the raw text; only the touched spans change, the rest of the file is never re-serialised, and the file is re-read right before the write.
index(raw): list of (start, end, path) for every object and array element (path = tuple of keys and indices).
apply(path, ops): ops = [("insert_after"|"replace"|"delete"|"insert_end", jsonpath, payload)], all located in the same read, written once."""
import json
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
def find(slug):
    import glob
    return glob.glob(S + "*/" + slug + "/🔣️.json")[0]
def index(raw):
    out = {}
    i = 0
    n = len(raw)
    def ws(i):
        while i < n and raw[i] in " \n\r\t": i += 1
        return i
    def string(i):
        j = i + 1
        while raw[j] != '"':
            j += 2 if raw[j] == "\\" else 1
        return j + 1
    def value(i, path):
        i = ws(i)
        c = raw[i]
        if c == "{":
            s = i
            i = ws(i + 1)
            while raw[i] != "}":
                k = string(i)
                key = json.loads(raw[i:k])
                i = ws(k)
                assert raw[i] == ":"
                i = value(i + 1, path + (key,))
                i = ws(i)
                if raw[i] == ",": i = ws(i + 1)
            out[path] = (s, i + 1)
            return i + 1
        if c == "[":
            s = i
            i = ws(i + 1)
            idx = 0
            while raw[i] != "]":
                i = value(i, path + (idx,))
                idx += 1
                i = ws(i)
                if raw[i] == ",": i = ws(i + 1)
            out[path] = (s, i + 1)
            return i + 1
        if c == '"': return string(i)
        j = i
        while j < n and raw[j] not in ",]} \n\r\t": j += 1
        return j
    value(0, ())
    return out
def _scalar(v): return not isinstance(v, (dict, list))
def _inline(v):
    return isinstance(v, list) and (all(_scalar(i) for i in v) or (all(isinstance(i, list) and all(_scalar(l) for l in i) for i in v) and len(json.dumps(v)) <= 160))
def vdump(value, depth):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value: return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(k, ensure_ascii=False) + ": " + vdump(i, depth + 1) for k, i in value.items()) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value: return "[]"
        if _inline(value): return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + vdump(i, depth + 1) for i in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)
def dump(obj, raw, start, unit=2, volume=False):
    ls = raw.rfind("\n", 0, start) + 1
    base = len(raw[ls:]) - len(raw[ls:].lstrip(" "))
    if volume: return vdump(obj, base // 2)
    return json.dumps(obj, indent=unit, ensure_ascii=False).replace("\n", "\n" + " " * base)
def apply(path, ops, expect=None):
    raw = open(path, encoding="utf-8").read()
    if expect is not None: assert raw == expect
    idx = index(raw)
    edits = []
    for op, jp, payload in ops:
        s, e = idx[tuple(jp)]
        vol = tuple(jp[:2]) == ("asset", "volume")
        if op == "replace":
            edits.append((s, e, dump(payload, raw, s, volume=vol)))
        elif op == "insert_after":
            ls = raw.rfind("\n", 0, s) + 1
            base = len(raw[ls:]) - len(raw[ls:].lstrip(" "))
            edits.append((e, e, "".join(",\n" + " " * base + dump(p, raw, s, volume=vol) for p in payload)))
        elif op == "delete_many":
            n = len(json.loads(raw[s:e]))
            gone = sorted(set(payload))
            runs = []
            for k in gone:
                if runs and runs[-1][1] == k - 1: runs[-1][1] = k
                else: runs.append([k, k])
            for a0, b0 in runs:
                if b0 + 1 < n: edits.append((idx[tuple(jp) + (a0,)][0], idx[tuple(jp) + (b0 + 1,)][0], ""))
                else:
                    assert a0 > 0, "cannot empty an array"
                    edits.append((idx[tuple(jp) + (a0 - 1,)][1], idx[tuple(jp) + (b0,)][1], ""))
        elif op == "insert_member_after":
            key, value = payload
            ls = raw.rfind("\n", 0, s) + 1
            base = len(raw[ls:]) - len(raw[ls:].lstrip(" "))
            edits.append((e, e, ",\n" + " " * base + json.dumps(key) + ": " + dump(value, raw, ls + base, volume=vol)))
        elif op == "insert_end":
            ls = raw.rfind("\n", 0, e - 1) + 1
            base = len(raw[ls:]) - len(raw[ls:].lstrip(" "))
            inner = raw[s + 1:e - 1].strip()
            body = ",\n".join(" " * (base + 2) + dump(p, raw, s + 0).replace("\n" + " " * base, "\n" + " " * (base + 2), 1) for p in payload)
            raise NotImplementedError
        elif op == "delete":
            parent = tuple(jp[:-1])
            ps, pe = idx[parent]
            n = jp[-1]
            sibling = idx.get(parent + (n + 1,))
            if sibling is not None:
                edits.append((s, sibling[0], ""))
            else:
                prev = idx[parent + (n - 1,)]
                edits.append((prev[1], e, ""))
    again = open(path, encoding="utf-8").read()
    assert again == raw, "file changed meanwhile"
    for s, e, t in sorted(edits, reverse=True): raw = raw[:s] + t + raw[e:]
    json.loads(raw)
    open(path, "w", encoding="utf-8").write(raw)
    return len(edits)
def load(path):
    raw = open(path, encoding="utf-8").read()
    return raw, json.loads(raw), index(raw)
import copy, os
T = os.path.dirname(os.path.abspath(__file__))
def mirror_path(slug, category):
    return os.path.join(T, "volumes", category, slug + ".json")
def patch_volume(slug, mutate):
    """mutate(solids) edits the list of volume solids in place (change, append, remove); only changed solids are replaced in the catalog file, the mirror volumes/<category>/<slug>.json is rewritten with the same solids."""
    p = find(slug)
    raw, doc, idx = load(p)
    a = doc["asset"]
    old = a["volume"]["solids"]
    new = copy.deepcopy(old)
    mutate(new)
    oldids = [s["id"] for s in old]
    newids = [s["id"] for s in new]
    ops = []
    if [i for i in newids if i in oldids] != [i for i in oldids if i in newids] or any(i not in newids for i in oldids):
        ops.append(("replace", ("asset", "volume", "solids"), new))
    else:
        last = None
        byid = {s["id"]: s for s in old}
        for s in new:
            if s["id"] in byid:
                if s != byid[s["id"]]:
                    ops.append(("replace", ("asset", "volume", "solids", oldids.index(s["id"])), s))
                last = oldids.index(s["id"])
            else:
                ops.append(("insert_after", ("asset", "volume", "solids", last if last is not None else 0), [s]))
    n = apply(p, ops, expect=raw) if ops else 0
    mp = mirror_path(slug, a["category"])
    if os.path.exists(mp):
        m = json.load(open(mp))
        m["volume"]["solids"] = new
        open(mp, "w", encoding="utf-8").write(json.dumps(m, indent=1, ensure_ascii=False) + "\n")
    return n
