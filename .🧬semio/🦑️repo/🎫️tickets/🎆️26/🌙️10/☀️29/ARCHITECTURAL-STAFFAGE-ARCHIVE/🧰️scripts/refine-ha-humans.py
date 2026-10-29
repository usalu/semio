"""Idempotent refinement of the humans catalog: hands with thumbs (all 12) and forefoot toe bones (humans-b).
usage: refine-ha-humans.py rig            # hands + toe bones/shapes (angles 0)
       refine-ha-humans.py angles FILE    # writes toe angles per pose from FILE ({asset: {pose: {l: deg, r: deg}}})"""
import json, math, re, sys
F = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/🔣️.json"
d = json.load(open(F))
R = lambda v: round(v + 0.0, 4)
TOE_ASSETS = ["worker-carrying-box", "parent-with-stroller", "jogger", "person-with-umbrella", "leaning-person", "tour-guide"]

def fmt(v): return f"{R(v):g}"
def hand_path(k):
    def P(x, y): return f"{fmt(x * k)} {fmt(y * k)}"
    fingers = [(0.170, -0.0225, 0.0075), (0.184, -0.0075, 0.0083), (0.192, 0.0085, 0.0085), (0.185, 0.0245, 0.0080)]
    s = f"M {P(-0.015, -0.030)} C {P(0.03, -0.035)} {P(0.075, -0.034)} {P(0.105, -0.031)}"
    for xt, yc, w in fingers:
        s += f" L {P(xt - 0.009, yc - w)} C {P(xt + 0.004, yc - w)} {P(xt + 0.004, yc + w)} {P(xt - 0.009, yc + w)}"
        if (xt, yc, w) != fingers[-1]: s += f" L {P(0.137, yc + w + 0.0004)}"
    s += f" C {P(0.13, 0.034)} {P(0.06, 0.038)} {P(0.0, 0.041)} L {P(-0.015, 0.037)} Z"
    return s
def thumb_path(k):
    def P(x, y): return f"{fmt(x * k)} {fmt(y * k)}"
    return f"M {P(-0.004, 0.024)} C {P(0.026, 0.05)} {P(0.066, 0.072)} {P(0.098, 0.082)} C {P(0.109, 0.085)} {P(0.117, 0.075)} {P(0.109, 0.065)} C {P(0.092, 0.05)} {P(0.07, 0.04)} {P(0.046, 0.022)} Z"
def finger_lines(k):
    return [[[R(0.075 * k), R(y * k)], [R(0.137 * k), R(y * k)]] for y in (-0.0153, 0.0003, 0.0168)]

def hands():
    for a in d["assets"]:
        els = a["geometry"]["elevation"]
        for s in "lr":
            bone = next(b for b in a["rig"]["bones"] if b["id"] == f"hand-{s}")
            k = bone["lengthM"] / 0.19
            main = next(e for e in els if e["bone"] == bone["id"] and e["role"] == "body" and e["lod"] == 0 and e["kind"] == "path" and e["id"] in (f"hand-{s}", f"glove-{s}"))
            fing_tone = next((e["tone"] for e in els if e["id"].startswith(f"fingers-{s}")), None)
            els[:] = [e for e in els if not (e["bone"] == bone["id"] and re.match(r"(fingers|thumb|glove-thumb|finger-tips)-", e["id"]))]
            main["d"] = hand_path(k)
            i = els.index(main)
            base = {kk: main[kk] for kk in ("bone", "region", "material") if kk in main}
            tid = "glove-thumb" if main["id"].startswith("glove") else "thumb"
            thumb = {"id": f"{tid}-{s}", **{"bone": base["bone"]}, "role": "body", "lod": 2, "z": R(main["z"] + 0.02), "filled": True, "tone": R(main["tone"] * 0.97), "kind": "path", "d": thumb_path(k), **{kk: base[kk] for kk in ("region", "material") if kk in base}}
            lines = [{"id": f"fingers-{s}-{n}", "bone": base["bone"], "role": "detail", "lod": 2, "z": R(main["z"] + 0.03), "filled": False, "tone": fing_tone if fing_tone is not None else R(main["tone"] * 0.55), "kind": "poly", "points": pts, "closed": False, **{kk: base[kk] for kk in ("region", "material") if kk in base}} for n, pts in enumerate(finger_lines(k))]
            extra = []
            if a["id"] == "wheelchair-user":
                cut = 0.147 * k
                tips = chain_multi(split_at(parse(main["d"]), cut), cut)
                extra = [{"id": f"finger-tips-{s}", "bone": base["bone"], "role": "detail", "lod": 2, "z": R(main["z"] + 0.025), "filled": True, "tone": 0.72, "kind": "path", "d": to_path(tips, 0, 0), "region": "hand-skin", "material": "skin"}]
            els[i + 1:i + 1] = [thumb] + extra + lines

def parse(dstr):
    tok = dstr.replace(",", " ").split(); i = 0; segs = []; cur = None; start = None
    while i < len(tok):
        c = tok[i]; i += 1
        if c == "M": cur = (float(tok[i]), float(tok[i + 1])); start = cur; i += 2
        elif c == "L": p = (float(tok[i]), float(tok[i + 1])); i += 2; segs.append(("L", cur, p)); cur = p
        elif c == "C": q = [(float(tok[i + 2 * j]), float(tok[i + 2 * j + 1])) for j in range(3)]; i += 6; segs.append(("C", cur, *q)); cur = q[2]
        elif c == "Z":
            if cur != start: segs.append(("L", cur, start)); cur = start
        else: raise ValueError(c)
    return segs
def bez(s, t):
    _, p0, p1, p2, p3 = s; u = 1 - t
    return tuple(u**3 * p0[i] + 3 * u * u * t * p1[i] + 3 * u * t * t * p2[i] + t**3 * p3[i] for i in (0, 1))
def split_c(s, t):
    _, p0, p1, p2, p3 = s
    l = lambda a, b, t: (a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t)
    a, b, c = l(p0, p1, t), l(p1, p2, t), l(p2, p3, t); ab, bc = l(a, b, t), l(b, c, t); m = l(ab, bc, t)
    return ("C", p0, a, ab, m), ("C", m, bc, c, p3)
def split_at(segs, bx):
    out = []
    for s in segs:
        if s[0] == "L":
            (x0, y0), (x1, y1) = s[1], s[2]
            if (x0 - bx) * (x1 - bx) < 0:
                t = (bx - x0) / (x1 - x0); m = (bx, y0 + (y1 - y0) * t); out += [("L", s[1], m), ("L", m, s[2])]
            else: out.append(s)
        else:
            ts = [j / 200 for j in range(201)]; xs = [bez(s, t)[0] - bx for t in ts]; cuts = []
            for j in range(200):
                if xs[j] * xs[j + 1] < 0:
                    lo, hi = ts[j], ts[j + 1]
                    for _ in range(50):
                        mid = (lo + hi) / 2
                        if (bez(s, mid)[0] - bx) * (bez(s, lo)[0] - bx) <= 0: hi = mid
                        else: lo = mid
                    cuts.append((lo + hi) / 2)
            rest = s; prev = 0.0
            for c in cuts:
                a, b = split_c(rest, (c - prev) / (1 - prev)); out.append(a); rest = b; prev = c
            out.append(rest)
    return out
def side(s, bx): 
    m = bez(s, 0.5) if s[0] == "C" else ((s[1][0] + s[2][0]) / 2, 0)
    return m[0] >= bx
def chain(segs, bx, right):
    sides = [side(s, bx) for s in segs]
    n = len(segs); k0 = next(i for i in range(n) if sides[i] != sides[i - 1]) if len(set(sides)) > 1 else 0
    seq = [segs[(k0 + i) % n] for i in range(n)]; sd = [sides[(k0 + i) % n] for i in range(n)]
    sel = [s for s, q in zip(seq, sd) if q == right]
    flags = [q == right for q in sd]
    changes = sum(1 for i in range(n) if flags[i] != flags[i - 1])
    assert changes == 2, changes
    return sel
def chain_multi(segs, bx):
    sides = [side(s, bx) for s in segs]; n = len(segs)
    k0 = next(i for i in range(n) if sides[i] != sides[i - 1])
    seq = [segs[(k0 + i) % n] for i in range(n)]; sd = [sides[(k0 + i) % n] for i in range(n)]
    out = []
    for s, q in zip(seq, sd):
        if not q: continue
        if out and out[-1][2] != s[1]: out.append(("L", out[-1][2], s[1]))
        out.append(s)
    return out
def to_path(sel, dx, dy):
    P = lambda p: f"{fmt(p[0] - dx)} {fmt(p[1] - dy)}"
    s = f"M {P(sel[0][1])}"
    for g in sel: s += (f" L {P(g[2])}" if g[0] == "L" else f" C {P(g[2])} {P(g[3])} {P(g[4])}")
    return s + " Z"
def xs_of(sel): return [p[0] for g in sel for p in g[1:]]

def toes():
    for a in d["assets"]:
        if a["id"] not in TOE_ASSETS: continue
        bones = a["rig"]["bones"]; els = a["geometry"]["elevation"]
        for s in "lr":
            if any(b["id"] == f"toe-{s}" for b in bones): continue
            foot = next(b for b in bones if b["id"] == f"foot-{s}")
            ball = next(an for an in a["anchors"] if an["id"] == f"contact-{s}")
            bx, by = ball["x"], ball["y"]
            shoe = next(e for e in els if e["bone"] == foot["id"] and e["role"] == "body" and e["lod"] == 0)
            segs = split_at(parse(shoe["d"]), bx)
            front = chain(segs, bx, True); rear = chain(segs, bx, False)
            tip = max(xs_of(front))
            bones.insert(bones.index(foot) + 1, {"id": f"toe-{s}", "parent": foot["id"], "title": {"en": f"{'Left' if s == 'l' else 'Right'} forefoot", "de": f"{'Linker' if s == 'l' else 'Rechter'} Vorfuß"}, "x": bx, "y": by, "angleDeg": 0, "lengthM": R(tip - bx), "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 70}})
            i = els.index(shoe)
            toe = {**shoe, "id": f"shoe-toe-{s}", "bone": f"toe-{s}", "d": to_path(front, bx, by)}
            shoe["d"] = to_path(rear, 0, 0)
            els.insert(i + 1, toe)
            for e in list(els):
                if e["bone"] != foot["id"] or e["kind"] != "poly" or e is shoe: continue
                px = [p[0] for p in e["points"]]
                if min(px) >= bx - 1e-6:
                    e["bone"] = f"toe-{s}"; e["points"] = [[R(p[0] - bx), R(p[1] - by)] for p in e["points"]]
                elif max(px) > bx + 1e-6 and len(e["points"]) == 2:
                    (x0, y0), (x1, y1) = e["points"]; t = (bx - x0) / (x1 - x0); ym = y0 + (y1 - y0) * t
                    e["points"] = [[x0, y0], [bx, R(ym)]]
                    els.insert(els.index(e) + 1, {**e, "id": e["id"].replace(f"-{s}", f"-toe-{s}") if f"-{s}" in e["id"] else e["id"] + f"-toe-{s}", "bone": f"toe-{s}", "points": [[0, R(ym - by)], [R(x1 - bx), R(y1 - by)]]})

def angles(fn):
    ang = json.load(open(fn))
    for a in d["assets"]:
        if a["id"] not in ang: continue
        for p in a["poses"]:
            for s in "lr":
                v = ang[a["id"]][p["id"]][s]
                p["bones"][f"toe-{s}"] = {"angleDeg": R(v)}

def clipkeys(fn):
    patch = json.load(open(fn))
    for a in d["assets"]:
        for cid, keys in patch.get(a["id"], {}).items():
            c = next(x for x in a["clips"] if x["id"] == cid)
            c["keys"] = [k for k in c["keys"] if not (k.get("bones") and len(k["bones"]) > 8)] + keys
            c["keys"].sort(key=lambda k: k["t"])

if sys.argv[1] == "rig": hands(); toes()
elif sys.argv[1] == "clipkeys": clipkeys(sys.argv[2])
else: angles(sys.argv[2])
open(F, "w").write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")
