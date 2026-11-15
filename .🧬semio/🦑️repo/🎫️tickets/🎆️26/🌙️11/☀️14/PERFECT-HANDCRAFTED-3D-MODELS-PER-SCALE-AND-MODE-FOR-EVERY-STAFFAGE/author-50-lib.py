import json
def scalar(v): return not isinstance(v, (dict, list))
def inline(v):
    if isinstance(v, list):
        return all(scalar(i) for i in v) or (all(isinstance(i, list) and all(scalar(l) for l in i) for i in v) and len(json.dumps(v)) <= 160)
    return False
def dump(v, depth=0):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(v, dict):
        if not v: return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(k, ensure_ascii=False) + ": " + dump(i, depth + 1) for k, i in v.items()) + "\n" + pad + "}"
    if isinstance(v, list):
        if not v: return "[]"
        if inline(v): return json.dumps(v, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(i, depth + 1) for i in v) + "\n" + pad + "]"
    return json.dumps(v, ensure_ascii=False)
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/%s/🔣️.json"
def load(slug):
    s = open(CAT % slug, encoding="utf-8").read()
    return s, json.loads(s)
def save(slug, d, before):
    cur = open(CAT % slug, encoding="utf-8").read()
    assert cur == before, "file changed meanwhile"
    open(CAT % slug, "w", encoding="utf-8").write(dump(d) + ("\n" if before.endswith("\n") else ""))

def nth_of(d, sc, view, id):
    n = 0
    for s in d["asset"]["representations"]:
        for v in ("elevation", "front", "plan"):
            for sh in d["asset"]["representations"][s]["geometry"][v]:
                if sh["id"] == id:
                    if s == sc and v == view: return n
                    n += 1
    raise KeyError((sc, view, id))

def edit2d(slug, ops):
    """ops: list of (scale, view, id, [new shapes]|None); text-level so that all else stays byte-identical."""
    s, d = load(slug)
    L = s.split("\n")
    jobs = []
    for sc, view, id, new in ops:
        n = nth_of(d, sc, view, id)
        a0 = next(k for k, l in enumerate(L) if l == '    "representations": {')
        a1 = next(k for k, l in enumerate(L) if k > a0 and l.startswith('    "') and not l.startswith('    "representations"'))
        idx = [k for k, l in enumerate(L) if a0 < k < a1 and l.strip() == '"id": %s,' % json.dumps(id)]
        jobs.append((idx[n], new))
    for i, new in sorted(jobs, key=lambda t: -t[0]):
        st = i - 1
        assert L[st] == "            {", L[st]
        e = st
        while L[e] not in ("            }", "            },"): e += 1
        comma = L[e].endswith(",")
        if new is None:
            block = []
        else:
            parts = []
            for sh in new:
                t = json.dumps(sh, ensure_ascii=False, indent=2).split("\n")
                parts.append(["            " + x for x in t])
            block = []
            for k, p in enumerate(parts):
                if k < len(parts) - 1 or comma: p[-1] += ","
                block += p
        L[st:e + 1] = block
    out = "\n".join(L)
    json.loads(out)
    open(CAT % slug, "w", encoding="utf-8").write(out)
