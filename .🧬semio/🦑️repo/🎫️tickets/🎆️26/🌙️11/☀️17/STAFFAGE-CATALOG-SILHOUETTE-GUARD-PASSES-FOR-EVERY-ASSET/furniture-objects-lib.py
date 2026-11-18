"""Shared helpers of the furniture and objects silhouette repairs: mvlib D editor plus shape and solid access by id."""
import sys, os, json, importlib.util
R = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS"
sys.path.insert(0, R)
import mvlib
sp = mvlib.sp
D = mvlib.D
def doc(cat, slug): return D(cat + "/" + slug)
def shape_obj(d, scale, view, sid): return d.obj(d.shape(scale, view, sid))
def solid_obj(d, scale, sid, lst="body"): return d.obj(d.solid(scale, sid, lst))
def copy_after(d, scale, view, src, new_id, **ch):
    n = d.shape(scale, view, src); o = d.obj(n); o["id"] = new_id
    for k, v in ch.items(): o[k] = v
    d.insert_after(n, o)
def delete_items(d, arr_node, pred):
    items = arr_node["items"]; t = d.t
    flags = [pred(json.loads(t[sp.child(it, "id")["s"]:sp.child(it, "id")["e"]])) for it in items]
    i = 0
    while i < len(items):
        if not flags[i]: i += 1; continue
        j = i
        while j + 1 < len(items) and flags[j + 1]: j += 1
        first, last = items[i], items[j]
        if j + 1 < len(items):
            a = t.rfind("\n", 0, first["s"]) + 1
            b = t.index(",", last["e"]) + 1
            while t[b] in " \t\r\n": b += 1
            b = t.rfind("\n", 0, b) + 1 if t[b - 1] == "\n" or True else b
            d.edits.append((a, items[j + 1]["s"] if False else t.rfind("\n", 0, items[j + 1]["s"]) + 1, ""))
        else:
            a = t.rfind(",", 0, first["s"]); d.edits.append((a, last["e"], ""))
        i = j + 1
import subprocess
_B = "/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
_HERE = os.path.dirname(os.path.abspath(__file__)) if "__file__" in globals() else "."
def hulls(slug, scale, q, view=None, bone=None):
    cmd = [_B, os.path.join(_HERE, "furniture-objects-hulls.ts"), slug, str(scale), q, view or "all"] + ([bone] if bone else [])
    out = subprocess.run(cmd, capture_output=True, text=True, cwd="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript").stdout
    r = {}
    for line in out.splitlines():
        p = line.split(" ", 2)
        if len(p) == 3 and p[2].startswith("["): r[(p[0], p[1])] = json.loads(p[2])
    return r
def hull_of(points):
    p = sorted(set(map(tuple, points)))
    def cr(o, a, b): return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    lo = []
    for x in p:
        while len(lo) >= 2 and cr(lo[-2], lo[-1], x) <= 0: lo.pop()
        lo.append(x)
    up = []
    for x in reversed(p):
        while len(up) >= 2 and cr(up[-2], up[-1], x) <= 0: up.pop()
        up.append(x)
    return [list(x) for x in lo[:-1] + up[:-1]]
