import os

A2 = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "🧰️A2")
p = os.path.join(A2, "lib.py")
s = open(p, encoding="utf-8").read()

helper = '''

def _radial(self, bone, like, stations=14):
    """Half profile (along the bone axis, radius) of a part: the largest distance from the bone axis of the side-view outline `like` at each station, in the bone frame."""
    lk = self.A.like(self.scale, like)
    assert lk["bone"] == bone, (like, bone)
    if lk["kind"] == "poly":
        pts = [tuple(q) for q in lk["points"]]
    elif lk["kind"] == "ellipse":
        a = math.radians(lk.get("rotationDeg", 0))
        pts = []
        for k in range(48):
            t = 2 * math.pi * k / 48
            ex, ey = lk["rx"] * math.cos(t), lk["ry"] * math.sin(t)
            pts.append((lk["cx"] + ex * math.cos(a) - ey * math.sin(a), lk["cy"] + ex * math.sin(a) + ey * math.cos(a)))
    else:
        raise ValueError(lk["kind"])
    x0, x1 = min(x for x, _ in pts), max(x for x, _ in pts)
    prof = []
    for i in range(stations):
        x = x0 + (x1 - x0) * i / (stations - 1)
        ys = []
        for (ax, ay), (bx, by) in zip(pts, pts[1:] + pts[:1]):
            if (ax - x) * (bx - x) <= 0 and ax != bx:
                ys.append(ay + (by - ay) * (x - ax) / (bx - ax))
        r = max((abs(y) for y in ys), default=0.0)
        prof.append((r5(x), r5(max(r, 0.002))))
    return [[prof[0][0], 0.0]] + [list(q) for q in prof] + [[prof[-1][0], 0.0]]


def _rev(self, id, bone, like, **kw):
    """Revolved solid about the bone's own axis with the symmetric envelope of the side-view part `like`: one 3D definition (circular cross-sections) for elevation, front and plan; the elevation shape is rewritten by commit."""
    prof = self._radial(bone, like)
    solid = {"kind": "revolved", "axisBone": bone, "axisX": r5(self.A.rest[bone][0]), "axisZ": 0.0, "mirror": True}
    self.A.evs[(self.scale, like)] = ("rev", bone, prof, solid)
    kw.setdefault("like", like)
    return self.add(id, bone, {"kind": "poly", "points": prof, "closed": True, "solid": solid}, **kw)


View._radial = _radial
View.rev = _rev
'''
s = s.replace("\n\ndef limbs(F, P, rows):", helper + "\n\ndef limbs(F, P, rows):", 1)
s = s.replace(
    "        hit = A.evs.get((scale, sh[\"id\"]))\n        if hit is not None and hit[0] == sh[\"bone\"]:",
    "        hit = A.evs.get((scale, sh[\"id\"]))\n        if hit is not None and hit[0] == \"rev\" and hit[1] == sh[\"bone\"]:\n            for k in [k for k in sh if k in (\"x\", \"y\", \"w\", \"h\", \"r\", \"cx\", \"cy\", \"rx\", \"ry\", \"rotationDeg\", \"points\", \"closed\", \"x0\", \"y0\", \"x1\", \"y1\", \"r0\", \"r1\", \"followTip\", \"d\", \"solid\")]:\n                del sh[k]\n            sh.update({\"kind\": \"poly\", \"points\": hit[2], \"closed\": True, \"solid\": hit[3]})\n        elif hit is not None and len(hit) == 3 and hit[0] == sh[\"bone\"]:",
)
s = s.replace("            sh[\"solid\"] = {\"kind\": \"extruded\", \"z0\": hit[1], \"z1\": hit[2]}", "            sh[\"solid\"] = {\"kind\": \"extruded\", \"z0\": hit[1], \"z1\": hit[2]}", 1)
open(p, "w", encoding="utf-8").write(s)
print("rev" in s)
