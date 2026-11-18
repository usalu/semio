"""📝 Applies an ops file (shape, shape-set, shape-add, solid-set) to a humans asset file: every touched geometry array is re-dumped in the file style (two-space indent, byte-stable for untouched shapes, checked), solid keys are replaced in place. python3 humansb-B-apply.py <slug> <ops.json> [--dry]"""
import importlib.util, json, re, sys
spec = importlib.util.spec_from_file_location('s', '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/sil-span.py'); s = importlib.util.module_from_spec(spec); spec.loader.exec_module(s)
def ind_of(t, n):
    ls = t.rfind("\n", 0, n["s"]) + 1
    return len(t[ls:]) - len(t[ls:].lstrip(" "))
def dump(v, ind): return json.dumps(v, indent=2, ensure_ascii=False).replace("\n", "\n" + " " * ind)
def sync(node, old, new):
    if isinstance(old, dict) and isinstance(new, dict):
        for k in new:
            if k not in old: raise SystemExit(f"new key {k} needs structured edit")
            if old[k] != new[k]: sync(s.child(node, k), old[k], new[k])
    elif isinstance(old, list) and isinstance(new, list) and len(old) == len(new):
        for i, (a, b) in enumerate(zip(old, new)):
            if a != b: sync(s.child(node, str(i)), a, b)
    elif not isinstance(old, (dict, list)) and not isinstance(new, (dict, list)): d.setv(node, new)
    else: raise SystemExit("structure change needs structured edit")
slug, opsf = sys.argv[1], sys.argv[2]
ops = json.load(open(opsf))
d = s.Doc(slug)
arrays = {}
for o in ops:
    if o["op"].startswith("shape"):
        key = (o["scale"], o["view"])
        if key not in arrays:
            n = d.at("asset", "representations", o["scale"], "geometry", o["view"])
            txt = d.t[n["s"]:n["e"]]
            lst = json.loads(txt)
            ind = ind_of(d.t, n)
            originals = {}
            for it in n["items"]:
                t = d.t[it["s"]:it["e"]]
                originals[json.loads(t)["id"]] = (t, json.loads(t))
            arrays[key] = (n, lst, ind, originals)
        lst = arrays[key][1]
        ids = [x["id"] for x in lst]
        if o["op"] == "shape-add": lst.insert(ids.index(o["after"]) + 1, o["shape"])
        elif o["op"] == "shape-set": lst[ids.index(o["id"])].update(o["set"])
        else: lst[ids.index(o["id"])] = o["shape"]
    else:
        n = d.find(o["scale"], o["id"])
        cur = json.loads(d.t[n["s"]:n["e"]])
        sync(n, cur, {**cur, **o["set"]})
for (n, lst, ind, originals) in arrays.values():
    parts = []
    for x in lst:
        o = originals.get(x["id"])
        parts.append(o[0] if o is not None and o[1] == x else dump(x, ind + 2))
    pad = " " * (ind + 2)
    d.rep(n, "[\n" + pad + (",\n" + pad).join(parts) + "\n" + " " * ind + "]")
if "--dry" in sys.argv: print("edits", len(d.edits)); sys.exit()
d.commit(); print("written", slug, len(d.edits), "edits")
