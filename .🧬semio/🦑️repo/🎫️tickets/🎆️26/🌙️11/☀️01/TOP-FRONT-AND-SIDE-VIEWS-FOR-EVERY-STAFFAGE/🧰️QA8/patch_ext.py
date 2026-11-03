import sys

p = sys.argv[1]
L = open(p, encoding="utf-8").read().split("\n")
i = [k for k, l in enumerate(L) if l.startswith("def _ext(")][0]
j = [k for k, l in enumerate(L) if l.startswith("View._local = _local")][0]
new = '''def _shaped(self, bone, like, world_pts):
    """The profile of an extruded solid in the bone frame: the elevation shape `like` of the same bone when it is a polygon or an ellipse (one shaped 3D definition for all three views), else the bar given in world metres."""
    if like:
        try:
            lk = self.A.like(self.scale, like)
        except KeyError:
            lk = None
        if lk is not None and lk["bone"] == bone:
            if lk["kind"] == "poly" and lk.get("closed", True):
                return [list(pt) for pt in lk["points"]]
            if lk["kind"] == "ellipse":
                a = math.radians(lk.get("rotationDeg", 0))
                pts = []
                for k in range(24):
                    t = 2 * math.pi * k / 24
                    ex, ey = lk["rx"] * math.cos(t), lk["ry"] * math.sin(t)
                    pts.append([r5(lk["cx"] + ex * math.cos(a) - ey * math.sin(a)), r5(lk["cy"] + ex * math.sin(a) + ey * math.cos(a))])
                return pts
    return [[r5(v) for v in self._local(bone, x, y)] for x, y in world_pts]


def _ext(self, id, bone, world_pts, z0, z1, **kw):
    """Extruded solid with a shaped profile (front: lateral z0..z1 by the height of the profile; plan: forward range by z0..z1). A part whose bone is tilted at rest (neck) stays a tapered followTip trapezoid until revolved solids about the bone axis exist."""
    kw.pop("keep_box", None)
    kw.pop("taper", None)
    kw.pop("tippad", None)
    if kw.pop("spread", False) and self.vp == "front":
        pad = world_pts.spec[3] if getattr(world_pts, "spec", None) else 0.0
        _, oy, _, ty = self.A.d[bone]
        lo, hi = min(oy, ty) - pad, max(oy, ty) + pad
        return self.poly(id, bone, [(z0, lo), (z1, lo), (z1, hi), (z0, hi)], True, True, **kw)
    kw.pop("spread", None)
    rx0, ry0, rx1, ry1 = self.A.rest[bone]
    blen = math.hypot(rx1 - rx0, ry1 - ry0)
    spec = getattr(world_pts, "spec", None)
    if spec is not None and self.vp == "front" and abs(ry1 - ry0) >= 0.45 * blen:
        _, a0, a1, half = spec
        ox, oy, tx, ty = self.A.d[bone]
        L = math.hypot(tx - ox, ty - oy)
        uy = (ty - oy) / L
        wmax, zc = (z1 - z0) / 2, (z0 + z1) / 2
        ya, yb = oy + uy * a0, oy + uy * a1
        return self.poly(id, bone, [(zc - wmax, ya), (zc + wmax, ya), (zc + wmax * 0.7, yb), (zc - wmax * 0.7, yb)], True, True, **kw)
    q = self._shaped(bone, kw.get("like"), world_pts)
    return self.add(id, bone, {"kind": "poly", "points": q, "closed": True, "solid": {"kind": "extruded", "z0": r5(z0), "z1": r5(z1)}}, **kw)


'''.split("\n")
L[i:j] = new
s = "\n".join(L).replace("View.ext = _ext", "View.ext = _ext\nView._shaped = _shaped")
open(p, "w", encoding="utf-8").write(s)
