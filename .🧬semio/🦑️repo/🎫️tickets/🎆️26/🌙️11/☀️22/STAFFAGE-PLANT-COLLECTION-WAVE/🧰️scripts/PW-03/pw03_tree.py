"""🌳️ PW-03 tree skeleton kit on top of the NW-V1 vegetation library: limbs are authored as 3D polylines (world metres, x ahead, y up, z left),
the planar NW-V1 rig takes their elevation projection and the lateral offset of their origin, and the 3D skeleton is written beside the document for the volume step.
@see ../../../../../🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/NW-V1/common.py
@see ./pw03_build.py"""
import math, os, sys

NWV1 = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "../../../../../🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/NW-V1"))
sys.path.insert(0, NWV1)
from lib import T, D2R, Rig
from common import Mass

GREEN_HC = 2.7e6


def direction(az, el):
    return (math.cos(el * D2R) * math.cos(az * D2R), math.sin(el * D2R), math.cos(el * D2R) * math.sin(az * D2R))


class Tree:
    def __init__(self, density):
        self.rig = Rig()
        self.density = density
        self.skeleton = {}
        self.zabs = {}
        self.levels = {}
        self.wood_mass = {}

    def point_on(self, id, f):
        pts = self.skeleton[id]["points"]
        seg = [math.dist(pts[i], pts[i + 1]) for i in range(len(pts) - 1)]
        target = f * sum(seg)
        for i, s in enumerate(seg):
            if target <= s or i == len(seg) - 1:
                t = min(1.0, target / s if s else 0)
                return tuple(pts[i][k] + (pts[i + 1][k] - pts[i][k]) * t for k in range(3))
            target -= s

    def stem(self, id, parent, rows, title, limit, material, region, level, group):
        """A vertical member from rows (y, r, dx, dz) of its centre line."""
        pts = [(dx, y, dz) for y, r, dx, dz in rows]
        radii = [r for y, r, dx, dz in rows]
        return self._add(id, parent, pts, radii, title, limit, material, region, level, group)

    def limb(self, id, parent, start, az, e0, e1, L, r0, r1, title, limit, material, region, level, group, bend=0.0, n=8):
        """A limb from `start` (world point, or (parent id, fraction)) at azimuth `az` (degrees from +x towards +z), elevation `e0` turning to `e1`, azimuth turning by `bend`."""
        p = self.point_on(*start) if isinstance(start[0], str) else tuple(start)
        pts, radii = [p], [r0]
        for i in range(1, n + 1):
            s = (i - 0.5) / n
            d = direction(az + bend * s, e0 + (e1 - e0) * s)
            p = tuple(p[k] + d[k] * L / n for k in range(3))
            pts.append(p)
            radii.append(r0 + (r1 - r0) * i / n)
        return self._add(id, parent, pts, radii, title, limit, material, region, level, group)

    def _add(self, id, parent, pts, radii, title, limit, material, region, level, group):
        a, b = pts[0], pts[-1]
        dx, dy = b[0] - a[0], b[1] - a[1]
        ang = math.degrees(math.atan2(dy, dx))
        lp = math.hypot(dx, dy)
        vol = sum(math.pi / 3 * math.dist(pts[i], pts[i + 1]) * (radii[i] ** 2 + radii[i] * radii[i + 1] + radii[i + 1] ** 2) for i in range(len(pts) - 1))
        kg = self.density * vol
        bone = self.rig.add(id, parent, (a[0], a[1]), ang, lp, title, 2 * radii[0], 2 * radii[-1], limit=limit, material=material, kg=kg)
        bone.region = region
        self.levels[id] = level
        self.zabs[id] = a[2]
        self.wood_mass[id] = kg
        self.skeleton[id] = {"points": [list(map(lambda v: round(v, 5), q)) for q in pts], "radii": [round(r, 5) for r in radii], "material": material, "region": region, "group": group, "parent": parent}
        return bone

    def mass(self, id, parent, c, rx, ry, region, kind, title, s=None):
        """A foliage mass: `c` is its world centre (x, y, z); the NW-V1 bone takes the elevation part, the lateral part goes into the bone's z."""
        m = Mass(id, parent, (c[0], c[1]), rx, ry, 0, region, kind, title, s=s)
        m.cz = c[2]
        return m

    def areas(self):
        """Lateral bark area per region of the 3D skeleton (frustum mantles)."""
        out = {}
        for k in self.skeleton.values():
            p, r = k["points"], k["radii"]
            a = sum(math.pi * (r[i] + r[i + 1]) * math.hypot(math.dist(p[i], p[i + 1]), r[i] - r[i + 1]) for i in range(len(p) - 1))
            out[k["region"]] = out.get(k["region"], 0.0) + a
        return out


def finish_doc(S, tree, doc):
    """Adds the lateral offsets of the bones (relative to their parent), renames the sway clip, sets provenance and presentation, and returns the skeleton sidecar."""
    a = doc["asset"]
    z = dict(tree.zabs)
    for m in S.masses:
        z[m.id] = m.cz
    for b in a["rig"]["bones"]:
        zz = z.get(b["id"], 0.0) - (z.get(b["parent"], 0.0) if b["parent"] else 0.0)
        if abs(zz) > 1e-4:
            b["z"] = round(zz, 4)
    for c in a["clips"]:
        if c["id"] == "breeze-sway":
            c["id"] = "wind-sway"
    a["presentation"] = {"defaultPose": S.default_pose, "defaultClip": "wind-sway", "defaultScale": S.default_scale}
    a["exports"] = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"]
    a["provenance"]["author"] = "Semio staffage plant worker PW-03"
    a["provenance"]["created"] = "2026-11-22"
    for r in a["thermal"]["regions"]:
        if r["model"] == "leaf" or r["id"] in S.exchange_regions:
            r["areaBasis"] = "exchange"
    return {"skeleton": tree.skeleton, "zabs": z, "masses": {m.id: {"parent": m.parent, "c": [m.c[0], m.c[1], m.cz], "rx": m.rx, "ry": m.ry, "region": m.region, "kind": m.kind} for m in S.masses}}


def skeleton_reps(S):
    """Representation frames without geometry: pens, hatches and ranges of the NW-V1 library; the volume step draws every view as the true view of each scale's body."""
    from common import PENS, RANGES, HATCHES
    reps = {sc: {"status": "authored", "denominator": int(sc), "validRange": {"minDenominator": RANGES[sc][0], "maxDenominator": RANGES[sc][1]}, "pens": PENS[sc], "hatches": HATCHES[sc], "geometry": {"elevation": [], "front": [], "plan": []}, "decisions": {}} for sc in ["500", "200", "100", "50"]}
    xs = [m.c[0] - m.rx for m in S.masses] + [m.c[0] + m.rx for m in S.masses]
    ys = [0.0] + [m.c[1] + m.ry for m in S.masses]
    return reps, (min(xs), min(ys), max(xs), max(ys))
