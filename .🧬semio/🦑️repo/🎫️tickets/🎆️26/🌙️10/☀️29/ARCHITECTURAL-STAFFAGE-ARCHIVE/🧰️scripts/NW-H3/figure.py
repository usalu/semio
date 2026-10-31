"""NW-H3 figure geometry authoring: station tables (fractions of bone length, front/back half widths in metres) for a clothed human, with
per-asset parameters and one function per drawing scale. Shapes are composed here by hand-chosen tables, never simplified automatically."""
from h3lib import *


def poly_fb(L, front, back, s0=None, s1=None):
    """closed outline (s,f) from front rows (frac,f) and back rows (frac,b); b positive = backwards"""
    pts = [(fr * L, f) for fr, f in front] + [(fr * L, -b) for fr, b in reversed(back)]
    return pts


class Fgr:
    def __init__(self, a, pose, P):
        self.a, self.P = a, P
        self.F = Fig(a, pose, signs=dict(P.get("signs", {})))
        self.B = {b["id"]: b for b in a["rig"]["bones"]}
        self.pose = pose

    def Lb(self, b):
        return self.B[b]["lengthM"]

    def sh(self, *a, **k):
        return self.F.sh(*a, **k)

    def poly(self, sid, bone, role, front, back, z, tone, region=None, regions=None, hatch=None, corners=(), pen=None, filled=True, smooth=True):
        pts = poly_fb(self.Lb(bone), front, back)
        return self.F.sh(sid, bone, role, pts, z, tone, region=region, regions=regions, hatch=hatch, corners=corners, penMm=None, filled=filled, smooth=smooth, **({"pen": pen} if pen else {}))
