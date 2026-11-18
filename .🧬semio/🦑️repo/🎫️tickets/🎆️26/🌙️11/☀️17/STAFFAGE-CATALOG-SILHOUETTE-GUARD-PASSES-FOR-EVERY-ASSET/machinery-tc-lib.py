"""Helpers of the tower crane repairs: apply the projected member strokes of machinery-strokes.ts (JSON) to one view of one scale of the asset, replacing named old shapes by the first stroke and inserting the others after the last replaced shape."""
import sys, json, subprocess
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET"
B = "/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
PK = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript"
def strokes(scale, view, regex, tag, mode=None):
    out = f"{T}/🗑️generated/tc-strokes-{scale}-{view}-{tag}.json"
    subprocess.run([B, f"{T}/machinery-strokes.ts", "tower-crane", str(scale), view, out, regex] + ([mode] if mode else []), cwd=PK, check=True, capture_output=True)
    res = json.load(open(out))
    for r in res:
        segs = []
        if mode in ("hull", "copies", "quads", "quadcopies"): continue
        parts = r["d"].split(" M ")
        parts = [p if p.startswith("M ") else "M " + p for p in parts]
        seen = set()
        for p in parts:
            key = p
            if key in seen: continue
            seen.add(key); segs.append(p)
        r["d"] = " ".join(segs)
    return res
def shape_for(r, scale, tmpl):
    hull = r["widthM"] == 0
    o = {"id": r["id"] + ("-hull" if hull else "-stroke"), "bone": r["bone"], "role": "body", "z": tmpl.get("z", 2), "filled": hull, "tone": tmpl.get("tone", 0.3), "kind": "path", "d": r["d"]}
    if not hull: o["penMm"] = round(r["widthM"] * 1000 / scale, 4)
    for k in ("region", "regions", "material"):
        if k in tmpl: o[k] = tmpl[k]
    return o
def apply(d, scale, view, res, replace, tmpl_id):
    tmpl = d.obj(d.shape(scale, view, tmpl_id))
    shapes = [shape_for(r, scale, tmpl) for r in res]
    keep = list(replace)
    for rid, s in zip(keep, shapes):
        n = d.shape(scale, view, rid); o = dict(s); d.replace_obj(n, o)
    rest = shapes[len(keep):]
    last = d.shape(scale, view, keep[-1]) if keep else d.shape(scale, view, tmpl_id)
    for s in reversed(rest): d.insert_after(last, s)
def shift(dstr, dx, dy):
    t = dstr.split(); out = []; i = 0
    while i < len(t):
        if t[i] in ("M", "L"):
            out += [t[i], repr(round(float(t[i + 1]) + dx, 4)), repr(round(float(t[i + 2]) + dy, 4))]; i += 3
        else: i += 1
    return " ".join(out)

def apply_copies(d, scale, view, res, old_ids):
    assert len(res) == len(old_ids), (len(res), len(old_ids))
    for r, oid in zip(res, old_ids):
        n = d.shape(scale, view, oid); o = d.obj(n)
        quad = r["widthM"] == 0
        dpath = r["d"]; pen = None
        if quad and dpath.count("M") == 1:
            v = [float(x) for x in dpath.replace("M", " ").replace("L", " ").replace("Z", " ").split()]
            pts = [(v[i], v[i + 1]) for i in range(0, 8, 2)]
            import math
            a, b = math.dist(pts[0], pts[1]), math.dist(pts[1], pts[2])
            if max(a, b) < 4 * min(a, b):
                w = min(a, b)
                if a >= b: m0 = ((pts[3][0] + pts[0][0]) / 2, (pts[3][1] + pts[0][1]) / 2); m1 = ((pts[1][0] + pts[2][0]) / 2, (pts[1][1] + pts[2][1]) / 2)
                else: m0 = ((pts[0][0] + pts[1][0]) / 2, (pts[0][1] + pts[1][1]) / 2); m1 = ((pts[2][0] + pts[3][0]) / 2, (pts[2][1] + pts[3][1]) / 2)
                L = math.dist(m0, m1) or 1.0
                u = ((m1[0] - m0[0]) / L, (m1[1] - m0[1]) / L)
                q0 = (m0[0] + u[0] * w / 2, m0[1] + u[1] * w / 2); q1 = (m1[0] - u[0] * w / 2, m1[1] - u[1] * w / 2)
                if L < w: q0 = q1 = ((m0[0] + m1[0]) / 2, (m0[1] + m1[1]) / 2)
                dpath = f"M {round(q0[0], 4)} {round(q0[1], 4)} L {round(q1[0], 4)} {round(q1[1], 4)}"; pen = round(w * 1000 / scale, 4); quad = False
        s = {"id": oid, "bone": o["bone"], "role": o["role"], "z": o.get("z", 2), "filled": quad, "tone": o.get("tone", 0.3), "kind": "path", "d": dpath}
        if not quad: s["penMm"] = pen if pen is not None else round(r["widthM"] * 1000 / scale, 4)
        for k in ("region", "regions", "material"):
            if k in o: s[k] = o[k]
        d.replace_obj(n, s)
import math
def classify(r):
    """Returns ("strip", d) for a quad that is at least 4 times as long as wide, else ("pen", d, penM) with the centre segment (or point) of the short quad."""
    v = [float(x) for x in r["d"].replace("M", " ").replace("L", " ").replace("Z", " ").split()]
    pts = [(v[i], v[i + 1]) for i in range(0, 8, 2)]
    a, b = math.dist(pts[0], pts[1]), math.dist(pts[1], pts[2])
    if max(a, b) >= 4 * min(a, b): return ("strip", r["d"])
    w = min(a, b)
    if a >= b: m0 = ((pts[3][0] + pts[0][0]) / 2, (pts[3][1] + pts[0][1]) / 2); m1 = ((pts[1][0] + pts[2][0]) / 2, (pts[1][1] + pts[2][1]) / 2)
    else: m0 = ((pts[0][0] + pts[1][0]) / 2, (pts[0][1] + pts[1][1]) / 2); m1 = ((pts[2][0] + pts[3][0]) / 2, (pts[2][1] + pts[3][1]) / 2)
    L = math.dist(m0, m1) or 1.0
    u = ((m1[0] - m0[0]) / L, (m1[1] - m0[1]) / L)
    q0 = (m0[0] + u[0] * w / 2, m0[1] + u[1] * w / 2); q1 = (m1[0] - u[0] * w / 2, m1[1] - u[1] * w / 2)
    if L < w: q0 = q1 = ((m0[0] + m1[0]) / 2, (m0[1] + m1[1]) / 2)
    return ("pen", f"M {round(q0[0], 4)} {round(q0[1], 4)} L {round(q1[0], 4)} {round(q1[1], 4)}", w)
