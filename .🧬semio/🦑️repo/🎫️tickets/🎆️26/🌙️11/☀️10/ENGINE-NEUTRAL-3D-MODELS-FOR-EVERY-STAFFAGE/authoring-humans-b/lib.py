"""Authoring helpers for the humans-b volumes: world-measured parts turned into explicit bone-local solids.
Usage from an asset script: b = Builder("standing-adult"); b.organic(...); b.save()."""
import json, math, os

T = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
GEN = os.path.join(T, "\U0001F5D1️generated")
R = lambda x: round(x, 4)


def sub(a, b): return [a[i] - b[i] for i in range(3)]
def dot(a, b): return sum(a[i] * b[i] for i in range(3))


def scan(rings, axis, at):
    """Intervals of the closed rings (even-odd) crossed by the line coordinate[axis] == at; returns the list of the other coordinate."""
    vals = []
    o = 1 - axis
    for ring in rings:
        n = len(ring)
        for i in range(n):
            a, b = ring[i], ring[(i + 1) % n]
            if (a[axis] <= at < b[axis]) or (b[axis] <= at < a[axis]):
                t = (at - a[axis]) / (b[axis] - a[axis])
                vals.append(a[o] + t * (b[o] - a[o]))
    vals.sort()
    return vals


def span(rings, axis, at):
    v = scan(rings, axis, at)
    return (v[0], v[-1]) if len(v) >= 2 else None


def simplify(ring, tol):
    pts = list(ring)
    if len(pts) > 1 and pts[0] == pts[-1]: pts.pop()
    def dp(p):
        if len(p) < 3: return p
        a, b = p[0], p[-1]
        dx, dy = b[0] - a[0], b[1] - a[1]
        L = math.hypot(dx, dy) or 1e-12
        best, k = -1, 0
        for i in range(1, len(p) - 1):
            d = abs((p[i][0] - a[0]) * dy - (p[i][1] - a[1]) * dx) / L
            if d > best: best, k = d, i
        if best > tol: return dp(p[:k + 1])[:-1] + dp(p[k:])
        return [a, b]
    h = len(pts) // 2
    out = dp(pts[:h + 1])[:-1] + dp(pts[h:] + [pts[0]])[:-1]
    return out


def _unit(a):
    l = math.sqrt(sum(x * x for x in a)) or 1e-12
    return [x / l for x in a]


def _cross(a, b): return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def quat_of(x, y, z):
    m = [[x[0], y[0], z[0]], [x[1], y[1], z[1]], [x[2], y[2], z[2]]]
    t = m[0][0] + m[1][1] + m[2][2]
    if t > 0:
        s = math.sqrt(t + 1) * 2; q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2; q = [s / 4, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2; q = [(m[0][1] + m[1][0]) / s, s / 4, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2; q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, s / 4, (m[1][0] - m[0][1]) / s]
    return [R(c) for c in q]


def with_rotations(secs):
    """fills the path-following section rotation exactly as the staffage solid conversion does (x = lateral axis projected perpendicular to the travel, z = travel), so the data renders the same in every consumer."""
    out = []
    n = len(secs)
    for i, s in enumerate(secs):
        if "rotation" in s: out.append(s); continue
        a, b = secs[min(i, n - 2)]["at"], secs[min(i + 1, n - 1)]["at"]
        tr = [b[k] - a[k] for k in range(3)]
        if math.sqrt(sum(c * c for c in tr)) < 1e-12: out.append(s); continue
        z = _unit(tr)
        def proj(ax):
            d = sum(ax[k] * z[k] for k in range(3)); return [ax[k] - d * z[k] for k in range(3)]
        x = proj([0, 0, 1])
        if math.sqrt(sum(c * c for c in x)) < 1e-6: x = proj([0, 1, 0])
        x = _unit(x); y = _cross(z, x)
        d = dict(s); d["rotation"] = quat_of(x, y, z); out.append(d)
    return out


class Builder:
    def __init__(self, slug, category="humans"):
        self.slug = slug
        self.ctx = json.load(open(os.path.join(GEN, "authoring-humans-b", slug + ".ctx.json")))
        self.draft = json.load(open(os.path.join(GEN, "drafts", category, slug + ".json")))
        self.dmat = {s["id"]: s["material"] for s in self.draft["volume"]["solids"]}
        self.fr = {b["id"]: b["frame"] for b in self.ctx["bones"]}
        self.shapes = {v: {} for v in ("elevation", "front", "plan")}
        for v in self.shapes:
            for s in self.ctx[v]:
                self.shapes[v].setdefault(s["id"], []).append(s)
        self.solids = []
        self.decisions = []
        self.prov = {p["solid"]: p["lateral"]["z"] for p in self.draft["report"]["provenance"] if p.get("lateral") and p["lateral"].get("z")}

    def has(self, view, sid): return sid in self.shapes[view]
    def rings(self, view, sid):
        return [r for s in self.shapes[view][sid] for r in s["rings"]]
    def sh(self, sid, view="elevation"): return self.shapes[view][sid][0]

    def loc(self, bone, p):
        f = self.fr[bone]
        d = sub(p, f["o"])
        return [dot(d, f["u"]), dot(d, f["v"]), dot(d, f["w"])]

    def locd(self, bone, d):
        f = self.fr[bone]
        return [dot(d, f["u"]), dot(d, f["v"]), dot(d, f["w"])]

    def meta(self, sid, material=None, role=None, tone=None, region=None, view="elevation", src=None):
        if sid not in self.dmat and src: sid = src
        import re
        while sid not in self.dmat and re.search(r"-(l|r|\d+)$", sid): sid = re.sub(r"-(l|r|\d+)$", "", sid)
        s = None
        for v in (view, "elevation", "front", "plan"):
            if sid in self.shapes[v]: s = self.shapes[v][sid][0]; break
        m = material or self.dmat.get(sid) or (s and s.get("material"))
        return {"material": m, "role": role or (s["role"] if s else "body"), "tone": tone if tone is not None else (s["tone"] if s and s["tone"] is not None else 0.5), "region": region if region is not None else (s["region"] if s else None)}

    def maxden(self, *ids):
        best = None
        for sid in ids:
            seen = [int(k) for k, v in self.ctx["scales"].items() if sid in v]
            if seen: best = max(seen) if best is None else max(best, max(seen))
        return None if best is None or best >= 500 else best

    def add(self, sid, bone, kind, geom, mt, maxden_ids=None, transform=None, repeat=None, weights=None, maxden=None):
        d = {"id": sid, "kind": kind, "bone": bone, "material": mt["material"], "role": mt["role"], "tone": mt["tone"]}
        if mt.get("region"): d["region"] = mt["region"]
        d.update(geom)
        md = maxden if maxden is not None else (self.maxden(*(maxden_ids or [sid])) if maxden_ids is not False else None)
        if md: d["maxDenominator"] = md
        if transform: d["transform"] = transform
        if repeat: d["repeat"] = repeat
        if weights: d["weights"] = weights
        assert d["material"], sid
        if any(s["id"] == sid for s in self.solids): raise SystemExit("duplicate " + sid)
        self.solids.append(d)
        return d

    def lateral_at(self, bone, sid_f, sid_p, fx, wpt, u_dir):
        """lateral span (lo,hi) at the world point wpt from the front twin sid_f (scan at y) or the plan twin sid_p (scan at x)."""
        vertical = abs(u_dir[1]) >= abs(u_dir[0])
        order = [("front", sid_f, 1, wpt[1]), ("plan", sid_p, 0, wpt[0])] if vertical else [("plan", sid_p, 0, wpt[0]), ("front", sid_f, 1, wpt[1])]
        for view, ids, axis, at in order:
            if not ids: continue
            rs = [r for i in (ids if isinstance(ids, (list, tuple)) else [ids]) if self.has(view, i) for r in self.rings(view, i)]
            if not rs: continue
            sp = span(rs, axis, at)
            if sp: return sp
        for view, ids, axis, at in order:
            if not ids: continue
            rs = [r for i in (ids if isinstance(ids, (list, tuple)) else [ids]) if self.has(view, i) for r in self.rings(view, i)]
            if rs:
                lo = min(p[axis] for r in rs for p in r); hi = max(p[axis] for r in rs for p in r)
                if lo - 0.03 <= at <= hi + 0.03:
                    o = 1 - axis; vals = [p[o] for r in rs for p in r]
                    return (min(vals), max(vals))
        return None

    def organic(self, sid, bone, elev, front=None, plan=None, n=7, exp=2.3, ends=0.55, mt=None, lat=None, inset=0.04, urange=None, rxmin=0.01, wc_fixed=None, latscale=1.0, grow=1.0, xmax=None, ymin=None, maxden_ids=None, exps=None, **kw):
        """loft along the bone: sections measured from the elevation outline(s) `elev` (ry, v centre) and from the front/plan twins (rx, w centre); `lat` = (lo, hi) overrides the lateral span."""
        ids = elev if isinstance(elev, (list, tuple)) else [elev]
        f = self.fr[bone]
        polys = []
        for i in ids:
            for r in self.rings("elevation", i):
                cl = lambda p: self.loc(bone, [min(p[0], xmax) if xmax is not None else p[0], max(p[1], ymin) if ymin is not None else p[1], 0])
                polys.append([[cl(p)[0], cl(p)[1]] for p in r])
        us = [p[0] for r in polys for p in r]
        u0, u1 = (min(us), max(us)) if urange is None else urange
        L = u1 - u0
        secs = []
        raw = []
        for k in range(n):
            t = k / (n - 1)
            u = u0 + L * (inset + (1 - 2 * inset) * t)
            sp = span(polys, 0, u)
            if sp is None:
                raw.append(None); continue
            vc, ry = (sp[0] + sp[1]) / 2, (sp[1] - sp[0]) / 2
            wp = [f["o"][i] + u * f["u"][i] + vc * f["v"][i] for i in range(3)]
            la = lat if lat else self.lateral_at(bone, ids if front is None else front, ids if plan is None else plan, 0, wp, f["u"])
            if la is None and ids[0] in self.prov: la = self.prov[ids[0]]
            raw.append((u, vc, ry, la))
        good = [r for r in raw if r]
        for k, r in enumerate(raw):
            if r is None:
                j = min(range(len(raw)), key=lambda q: abs(q - k) if raw[q] else 99)
                r = raw[j]; raw[k] = (u0 + L * (inset + (1 - 2 * inset) * k / (n - 1)), r[1], r[2], r[3])
        for k, r in enumerate(raw):
            if r[3] is None:
                j = min((q for q in range(len(raw)) if raw[q][3]), key=lambda q: abs(q - k), default=None)
                if j is None: raise SystemExit(f"{sid}: no lateral data; pass lat=")
                raw[k] = (r[0], r[1], r[2], raw[j][3])
        for k, (u, vc, ry, la) in enumerate(raw):
            rx = max(rxmin, (la[1] - la[0]) / 2 * latscale)
            wc = (la[0] + la[1]) / 2 if wc_fixed is None else wc_fixed
            if lat is None and wc_fixed is None:
                wc = (la[0] + la[1]) / 2
            secs.append({"at": [R(u), R(vc), R(self.wloc(bone, wc))], "rx": R(rx * grow), "ry": R(max(0.004, ry) * grow), "exponent": (exps[k] if exps else exp)})
        a = dict(secs[0]); b = dict(secs[-1])
        a["at"] = [R(u0), a["at"][1], a["at"][2]]; a["rx"] = R(a["rx"] * ends); a["ry"] = R(a["ry"] * ends)
        b["at"] = [R(u1), b["at"][1], b["at"][2]]; b["rx"] = R(b["rx"] * ends); b["ry"] = R(b["ry"] * ends)
        if ends < 1 and ends > 0: secs = [a] + secs + [b]
        return self.add(sid, bone, "loft", {"sections": with_rotations(secs), "capStart": True, "capEnd": True}, mt or self.meta(sid, src=ids[0], **kw), maxden_ids)

    def wloc(self, bone, z):
        """world lateral coordinate z (the lateral axis is world z for every planar bone) to the bone-local w coordinate."""
        f = self.fr[bone]
        return z - f["o"][2] if f["w"][2] >= 0 else f["o"][2] - z

    def blob(self, sid, bone, elev, front=None, plan=None, lat=None, mt=None, grow=1.0, rw=None, maxden_ids=None, tilt=True, **kw):
        """ellipsoid fitted to the elevation outline (principal axes) with lateral span from the twins."""
        rs = self.rings("elevation", elev)
        pts = [[self.loc(bone, [p[0], p[1], 0])[0], self.loc(bone, [p[0], p[1], 0])[1]] for r in rs for p in r]
        mx = sum(p[0] for p in pts) / len(pts); my = sum(p[1] for p in pts) / len(pts)
        sxx = sum((p[0] - mx) ** 2 for p in pts); syy = sum((p[1] - my) ** 2 for p in pts); sxy = sum((p[0] - mx) * (p[1] - my) for p in pts)
        th = 0.5 * math.atan2(2 * sxy, sxx - syy) if tilt else 0.0
        c, s = math.cos(th), math.sin(th)
        a = [(p[0] - mx) * c + (p[1] - my) * s for p in pts]; bb = [-(p[0] - mx) * s + (p[1] - my) * c for p in pts]
        ru, rv = (max(a) - min(a)) / 2 * grow, (max(bb) - min(bb)) / 2 * grow
        cu, cv = (max(a) + min(a)) / 2, (max(bb) + min(bb)) / 2
        cx, cy = mx + cu * c - cv * s, my + cu * s + cv * c
        if lat is None:
            fr = self.fr[bone]
            wp = [fr["o"][i] + cx * fr["u"][i] + cy * fr["v"][i] for i in range(3)]
            lat = self.lateral_at(bone, front or [elev], plan or [elev], 0, wp, fr["u"])
        if lat is None and elev in self.prov: lat = self.prov[elev]
        if lat is None and rw is None: raise SystemExit(f"{sid}: no lateral; pass lat=/rw=")
        lo, hi = lat if lat else (None, None)
        rwv = rw if rw is not None else (hi - lo) / 2
        wc = self.wloc(bone, (lo + hi) / 2) if lat else 0
        tr = {"translation": [R(cx), R(cy), R(wc)]}
        if abs(th) > 1e-3: tr["rotation"] = [0, 0, R(math.sin(th / 2)), R(math.cos(th / 2))]
        return self.add(sid, bone, "ellipsoid", {"radii": [R(max(.003, ru)), R(max(.003, rv)), R(max(.003, rwv))]}, mt or self.meta(sid, src=elev, **kw), maxden_ids, transform=tr)

    def extrude(self, sid, bone, elev, lat, tol=0.0015, mt=None, maxden_ids=None, ring=0, **kw):
        r = self.rings("elevation", elev)[ring]
        pts = [[R(self.loc(bone, [p[0], p[1], 0])[0]), R(self.loc(bone, [p[0], p[1], 0])[1])] for p in r]
        pts = [[R(x), R(y)] for x, y in simplify(pts, tol)]
        lo, hi = self.wspan(bone, lat)
        return self.add(sid, bone, "extrude", {"profile": pts, "plane": "xy", "from": R(lo), "to": R(hi)}, mt or self.meta(sid, **kw), maxden_ids)

    def wspan(self, bone, lat):
        a, b = self.wloc(bone, lat[0]), self.wloc(bone, lat[1])
        return (min(a, b), max(a, b))

    def boxl(self, sid, bone, center, size, mt=None, corner=None, maxden_ids=None, rot=None, repeat=None, **kw):
        """box with a bone-local centre (u, v, w) and size."""
        tr = {"translation": [R(x) for x in center]}
        if rot: tr["rotation"] = [0, 0, R(math.sin(rot / 2)), R(math.cos(rot / 2))]
        g = {"size": [R(x) for x in size]}
        if corner: g["cornerRadius"] = R(corner)
        return self.add(sid, bone, "box", g, mt or self.meta(sid, **kw), maxden_ids, transform=tr, repeat=repeat)

    def box(self, sid, bone, center, size, mt=None, rot=None, corner=None, maxden_ids=None, repeat=None, **kw):
        """box from world centre and size given in bone-local axes (u, v, w); rot = angle about w (rad)."""
        c = self.loc(bone, center)
        tr = {"translation": [R(c[0]), R(c[1]), R(c[2])]}
        if rot: tr["rotation"] = [0, 0, R(math.sin(rot / 2)), R(math.cos(rot / 2))]
        g = {"size": [R(x) for x in size]}
        if corner: g["cornerRadius"] = R(corner)
        return self.add(sid, bone, "box", g, mt or self.meta(sid, **kw), maxden_ids, transform=tr, repeat=repeat)

    def capsule(self, sid, bone, p0, p1, r0, r1=None, mt=None, lateralScale=None, maxden_ids=None, repeat=None, **kw):
        g = {"p0": [R(x) for x in self.loc(bone, p0)], "p1": [R(x) for x in self.loc(bone, p1)], "r0": R(r0), "r1": R(r1 if r1 is not None else r0)}
        if lateralScale: g["lateralScale"] = lateralScale
        return self.add(sid, bone, "capsule", g, mt or self.meta(sid, **kw), maxden_ids, repeat=repeat)


    def ell(self, sid, bone, center, radii, mt=None, maxden_ids=None, repeat=None, **kw):
        """ellipsoid with world-axis radii (rx ahead, ry up, rz lateral) centred on a world point; the transform cancels the bone tilt in the sagittal plane."""
        f = self.fr[bone]
        th = math.degrees(math.atan2(f["u"][1], f["u"][0]))
        c = self.loc(bone, center)
        near_y = abs(((th % 180) - 90)) <= 45
        if near_y:
            phi = ((90 - th + 90) % 180) - 90
            ru, rv = radii[1], radii[0]
        else:
            phi = ((-th + 90) % 180) - 90
            ru, rv = radii[0], radii[1]
        tr = {"translation": [R(c[0]), R(c[1]), R(c[2])]}
        if abs(phi) > 0.05:
            a = math.radians(phi)
            tr["rotation"] = [0, 0, R(math.sin(a / 2)), R(math.cos(a / 2))]
        return self.add(sid, bone, "ellipsoid", {"radii": [R(ru), R(rv), R(radii[2])]}, mt or self.meta(sid, **kw), maxden_ids, transform=tr, repeat=repeat)

    def edge(self, view, sid, axis, at, side):
        """extreme coordinate (side +1 max, -1 min) of the other axis of a shape at coordinate[axis] == at."""
        sp = span(self.rings(view, sid), axis, at)
        return None if sp is None else (sp[1] if side > 0 else sp[0])

    def loft(self, sid, bone, sections, mt=None, caps=(True, True), maxden_ids=None, src=None, **kw):
        """loft from explicit sections given in bone-local u, v, w (explicit rotations are filled in)."""
        return self.add(sid, bone, "loft", {"sections": with_rotations(sections), "capStart": caps[0], "capEnd": caps[1]}, mt or self.meta(sid, src=src, **kw), maxden_ids)

    def loftw(self, sid, bone, pts, rx, ry, exp=2.4, mt=None, caps=(True, True), maxden_ids=None, **kw):
        """loft through world centre points; rx (lateral half width) and ry (half thickness in the section plane) are numbers or lists."""
        n = len(pts)
        lst = lambda v: v if isinstance(v, (list, tuple)) else [v] * n
        rxs, rys, es = lst(rx), lst(ry), lst(exp)
        secs = [{"at": [R(c) for c in self.loc(bone, p)], "rx": R(rxs[i]), "ry": R(rys[i]), "exponent": es[i]} for i, p in enumerate(pts)]
        return self.loft(sid, bone, secs, mt=mt, caps=caps, maxden_ids=maxden_ids, **kw)

    def tube(self, sid, bone, elev, z, r=None, mt=None, maxden_ids=None, repeat=None, **kw):
        """capsule along the principal axis of an elongated elevation outline at lateral position z; radius = half the median width unless given."""
        pts = [p for ring in self.rings("elevation", elev) for p in ring]
        mx = sum(p[0] for p in pts) / len(pts); my = sum(p[1] for p in pts) / len(pts)
        sxx = sum((p[0] - mx) ** 2 for p in pts); syy = sum((p[1] - my) ** 2 for p in pts); sxy = sum((p[0] - mx) * (p[1] - my) for p in pts)
        th = 0.5 * math.atan2(2 * sxy, sxx - syy)
        a = (math.cos(th), math.sin(th)); n = (-a[1], a[0])
        t = [(p[0] - mx) * a[0] + (p[1] - my) * a[1] for p in pts]; q = [(p[0] - mx) * n[0] + (p[1] - my) * n[1] for p in pts]
        t0, t1 = min(t), max(t)
        mid = sorted(q[i] for i in range(len(pts)) if t0 + 0.25 * (t1 - t0) <= t[i] <= t1 - 0.25 * (t1 - t0))
        half = (mid[-1] - mid[0]) / 2 if mid else 0.01
        cq = (mid[-1] + mid[0]) / 2 if mid else 0
        rr = r if r is not None else half
        e0, e1 = t0 + rr * 0.95, t1 - rr * 0.95
        P = lambda tt: (mx + tt * a[0] + cq * n[0], my + tt * a[1] + cq * n[1], z)
        return self.capsule(sid, bone, P(e0), P(e1), rr, mt=mt, maxden_ids=maxden_ids or [elev], repeat=repeat, src=elev, **kw)

    def polyline(self, sid, bone, pts, r, r1=None, maxden_ids=None, **kw):
        """round bar through world points as consecutive capsules sid, sid-2, ..."""
        for i in range(len(pts) - 1):
            self.capsule(sid if i == 0 else f"{sid}-{i + 1}", bone, pts[i], pts[i + 1], r, r1 if i == len(pts) - 2 and r1 is not None else r, maxden_ids=maxden_ids, **kw)

    def dec(self, en, de):
        self.decisions.append({"en": en, "de": de})

    def save(self, title_decisions=None):
        os.makedirs(os.path.join(T, "volumes", "humans"), exist_ok=True)
        out = {"volume": {"status": "authored", "solids": self.solids, "decisions": self.decisions}}
        p = os.path.join(T, "volumes", "humans", self.slug + ".json")
        json.dump(out, open(p, "w"), ensure_ascii=False, indent=1)
        print(p, len(self.solids), "solids")


ORGANIC = {"pelvis", "torso", "neck", "head", "arm-l", "arm-r", "upper-arm-l", "upper-arm-r", "forearm-l", "forearm-r", "hand-l", "hand-r", "thigh-l", "thigh-r", "shank-l", "shank-r", "foot-l", "foot-r", "toe-l", "toe-r"}


def _twin_ids(b, view, sid):
    out = []
    for c in (sid, sid + "-l", sid + "-r"):
        if b.has(view, c): out.append(c)
    return out


def auto(b, skip=(), bones=ORGANIC, over=None, exp=2.3):
    """every closed body/detail elevation shape on an organic bone becomes a loft (ellipses become ellipsoids); `over[id]` = dict of keyword overrides (n, exp, front, plan, lat, ends, ...) or None to skip."""
    over = over or {}
    seen = set()
    for s in b.ctx["elevation"]:
        sid = s["id"]
        if sid in seen or sid in skip or s["bone"] not in bones or not s["closed"] or not s["rings"] or s["role"] not in ("body", "detail"): continue
        seen.add(sid)
        if sid in over and over[sid] is None: continue
        o = dict(over.get(sid) or {})
        same = lambda view: [i for i, ss in b.shapes[view].items() if any(x["bone"] == s["bone"] and x["closed"] and x["role"] == "body" for x in ss)]
        fr = o.pop("front") if "front" in o else (_twin_ids(b, "front", sid) or same("front"))
        pl = o.pop("plan") if "plan" in o else (_twin_ids(b, "plan", sid) or same("plan"))
        if s["kind"] == "ellipse":
            pairs = [(sid, fr, pl)]
            if not fr and not pl:
                continue
            if not any(b.has("front", sid) for _ in [0]) and not b.has("plan", sid):
                lr = [(sid + "-l", [sid + "-l"] if b.has("front", sid + "-l") else [], [sid + "-l"] if b.has("plan", sid + "-l") else []), (sid + "-r", [sid + "-r"] if b.has("front", sid + "-r") else [], [sid + "-r"] if b.has("plan", sid + "-r") else [])]
                for nid, f2, p2 in lr: b.blob(nid, s["bone"], sid, front=f2, plan=p2, maxden_ids=[nid, sid], **o)
            else:
                b.blob(sid, s["bone"], sid, front=fr or None, plan=pl or None, **o)
            continue
        ln = max(max(p[0] for r in s["rings"] for p in r) - min(p[0] for r in s["rings"] for p in r), max(p[1] for r in s["rings"] for p in r) - min(p[1] for r in s["rings"] for p in r))
        n = o.pop("n", 3 if ln < 0.06 else 5 if ln < 0.15 else 7 if ln < 0.35 else 9)
        b.organic(sid, s["bone"], sid, front=fr, plan=pl, n=n, exp=o.pop("exp", exp), **o)
