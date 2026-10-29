# [scratch] forward kinematics and two-unknown limb IK against an assembled asset JSON (mirrors 🦴️rig/🟦️.ts)
import json, math

def load(path, asset_id):
    d = json.load(open(path))
    for a in d.get("assets", [d]):
        if a["id"] == asset_id:
            return Asset(a)

def mat(x, y, deg):
    r = math.radians(deg); c, s = math.cos(r), math.sin(r)
    return (c, s, -s, c, x, y)

def mul(p, l):
    return (p[0]*l[0]+p[2]*l[1], p[1]*l[0]+p[3]*l[1], p[0]*l[2]+p[2]*l[3], p[1]*l[2]+p[3]*l[3],
            p[0]*l[4]+p[2]*l[5]+p[4], p[1]*l[4]+p[3]*l[5]+p[5])

def app(m, x, y):
    return (m[0]*x+m[2]*y+m[4], m[1]*x+m[3]*y+m[5])

def ang(m):
    return math.degrees(math.atan2(m[1], m[0]))

class Asset:
    def __init__(self, a):
        self.a = a
        self.bones = {b["id"]: b for b in a["rig"]["bones"]}
        self.anchors = {x["id"]: x for x in a["anchors"]}
        order, seen = [], set()
        def visit(b):
            if b["id"] in seen: return
            if b["parent"]: visit(self.bones[b["parent"]])
            seen.add(b["id"]); order.append(b["id"])
        for b in a["rig"]["bones"]: visit(b)
        self.order = order
    def clamp(self, bid, v):
        j = self.bones[bid]["joint"]
        return min(max(v, j.get("minDeg", -1e9)), j.get("maxDeg", 1e9))
    def fk(self, offsets, root=(0, 0, 0)):
        w = {}
        o = mat(*root)
        for bid in self.order:
            b = self.bones[bid]
            off = self.clamp(bid, offsets.get(bid, 0))
            loc = mat(b["x"], b["y"], b["angleDeg"] + off)
            w[bid] = mul(o if b["parent"] is None else w[b["parent"]], loc)
        return w
    def anchor(self, w, aid):
        a = self.anchors[aid]
        return app(w[a["bone"]], a["x"], a["y"])
    def rest_abs(self, bid):
        t = 0
        while bid:
            b = self.bones[bid]; t += b["angleDeg"]; bid = b["parent"]
        return t
    def limits(self, bid):
        j = self.bones[bid]["joint"]
        return j.get("minDeg", -180), j.get("maxDeg", 180)
    def solve_limb(self, base, chain, aid, target, spec, root=(0, 0, 0), prefer=None):
        """chain: bone ids; spec: dict bone -> ('solve',) | ('off', v) | ('abs', v); returns offsets for chain bones and error."""
        solve = [b for b in chain if spec[b][0] == "solve"]
        assert len(solve) == 2, solve
        wb = self.fk(base, root)
        parent = self.bones[chain[0]]["parent"]
        pm = wb[parent]
        def evaluate(v1, v2):
            offs = {}; m = pm; vals = {solve[0]: v1, solve[1]: v2}
            for bid in chain:
                b = self.bones[bid]; kind = spec[bid]
                if kind[0] == "solve": off = vals[bid]
                elif kind[0] == "off": off = kind[1]
                else:
                    off = kind[1] - ang(m) - b["angleDeg"]
                    off = (off + 180) % 360 - 180
                offs[bid] = off
                m = mul(m, mat(b["x"], b["y"], b["angleDeg"] + off))
            a = self.anchors[aid]
            # recompute anchor with chain matrices
            mm = pm
            for bid in chain:
                b = self.bones[bid]
                mm = mul(mm, mat(b["x"], b["y"], b["angleDeg"] + offs[bid]))
                if bid == a["bone"]:
                    p = app(mm, a["x"], a["y"]); break
            return offs, p
        def ok(offs):
            return all(self.limits(b)[0] - 1e-6 <= offs[b] <= self.limits(b)[1] + 1e-6 for b in chain)
        lo1, hi1 = self.limits(solve[0]); lo2, hi2 = self.limits(solve[1])
        best = None
        def score(offs, p):
            e = math.hypot(p[0]-target[0], p[1]-target[1])
            pen = 0 if prefer is None else sum(abs(offs[k]-v) for k, v in prefer.items()) * 1e-5
            return e + pen
        v1 = lo1
        while v1 <= hi1 + 1e-9:
            v2 = lo2
            while v2 <= hi2 + 1e-9:
                offs, p = evaluate(v1, v2)
                if ok(offs):
                    s = score(offs, p)
                    if best is None or s < best[0]: best = (s, v1, v2)
                v2 += 1.0
            v1 += 1.0
        _, v1, v2 = best
        step = 0.5
        for _ in range(60):
            improved = False
            for d1, d2 in ((step,0),(-step,0),(0,step),(0,-step),(step,step),(-step,-step),(step,-step),(-step,step)):
                n1, n2 = min(max(v1+d1, lo1), hi1), min(max(v2+d2, lo2), hi2)
                offs, p = evaluate(n1, n2)
                if ok(offs) and score(offs, p) < best[0] - 1e-12:
                    best = (score(offs, p), n1, n2); v1, v2 = n1, n2; improved = True
            if not improved: step /= 2
            if step < 1e-5: break
        offs, p = evaluate(v1, v2)
        return offs, math.hypot(p[0]-target[0], p[1]-target[1])
