"""Drawing helpers shared by the species files; every call is parameterised by hand-set values in the species file."""
import math
from lib import *
from common import PENS, RANGES, HATCHES, decisions_texts


def perimeter(m):
    return 2 * math.pi * math.sqrt((m.rx ** 2 + m.ry ** 2) / 2)


def mass_shape(S, m, scale, lobe, amp, vary, z, phase=0.0, under=0.0, pointed=0.6, n=6, hatch=None, rx_s=1.0, ry_s=1.0, suffix="mass", material=None, tilt=0.0, regions=None):
    k = max(5, round(perimeter(m) / lobe))
    pts = blob(m.c[0], m.c[1], m.rx * rx_s, m.ry * ry_s, m.rot, k=k, amp=amp, phase=phase, vary=vary, n=n, under=under, pointed=pointed, tilt=tilt)
    return Shape("%s-%s" % (m.id, suffix), S.rig.bones[m.id], "body", cr_closed(pts), True, S.tone[m.kind], z, region=None if regions else m.region, regions=regions, material=S.mat_of[m.region], hatch=hatch), pts


def mass_arcs(S, m, scale, specs, role, z, prefix="arc"):
    """specs: list of (radius fraction x, radius fraction y, a0, a1, rot extra) hand-set arcs inside the mass."""
    out = []
    for i, (fx, fy, a0, a1, dr) in enumerate(specs):
        pts = arc(m.c[0], m.c[1], m.rx * fx, m.ry * fy, a0, a1, n=6, rot=m.rot + dr)
        out.append(Shape("%s-%s%d" % (m.id, prefix, i + 1), S.rig.bones[m.id], role, cr_open(pts), False, S.tone_line, z + 0.01 * i))
    return out


def stem_pts(bone, stations, asym=None):
    """stations: (s along the bone from its base, half width); right edge up, left edge down; `asym` shifts the axis per station."""
    asym = asym or [0.0]
    right, left = [], []
    for i, (s, w) in enumerate(stations):
        a = asym[i % len(asym)]
        right.append(bone.to_world((s, -w + a)))
        left.append(bone.to_world((s, w + a)))
    return right + left[::-1]


def tapered(S, b, scale, stations, wob, z, region=None, extra_regions=None, id=None, penMm=None, s1=None):
    pts = limb_outline(b, stations=stations, wob=wob, s1=s1)
    return Shape(id or "%s-wood" % b.id, b, "body", cr_closed(pts), True, S.tone["wood"], z, region=region or b.region, material=S.mat_of[region or b.region])


def centre_stroke(S, b, scale, role, z, bend=0.0, id=None, region=None, n=4, s0=0.0, s1=None):
    s1 = b.L if s1 is None else s1
    pts = [b.to_world((s0 + (s1 - s0) * i / (n - 1), bend * 4 * (i / (n - 1)) * (1 - i / (n - 1)))) for i in range(n)]
    sh = Shape(id or "%s-twig" % b.id, b, role, cr_open(pts), False, S.tone_line, z, region=region or b.region)
    return sh


def wavy(bone, s0, s1, off, amp, periods, n=7, phase=0.0):
    pts = []
    for i in range(n):
        t = i / (n - 1)
        s = s0 + (s1 - s0) * t
        pts.append(bone.to_world((s, off + amp * math.sin(2 * math.pi * periods * t + phase))))
    return pts


def stroke(S, id, bone, pts, role, z, region=None, closed=False):
    return Shape(id, bone, role, cr_closed(pts) if closed else cr_open(pts), False, S.tone_line, z, region=region)


def ground_ok(shapes):
    return shapes


def to_json(shapes):
    return [s.json() for s in sorted(shapes, key=lambda s: s.z)]


def world_bbox(shapes, roles=("body",)):
    pts = []
    for s in shapes:
        if s.role in roles or True:
            for ring in flat(s.cmds, 4):
                pts += ring
    return bbox(pts)


PRUNABLE = ("arc", "leaf", "scar", "patch", "fissure", "ridge", "fruit", "spur", "twig", "bud", "vein", "seam", "scale", "knot", "crack", "line", "lenticel")


def prune(S, shapes, scale, drop_body=("-b", "-twig")):
    from probe import pose_gaps
    for _ in range(12):
        g = [(a, b, c) for a, b, c, _ in pose_gaps(S, shapes, scale)]
        victims = set()
        by = {s.id: s for s in shapes}
        for gp, a, b in g:
            cand = [x for x in (a, b) if any(k in x for k in PRUNABLE)]
            if not cand:
                cand = [x for x in (a, b) if any(x.endswith(k) or k in x for k in drop_body) and "shoot" in x or "branch" in x and x.endswith("-b")]
            if cand:
                victims.add(cand[0])
        if not victims:
            break
        shapes[:] = [s for s in shapes if s.id not in victims]
        print("  pruned", scale, sorted(victims))
    return shapes


def rep_json(S, scale, shapes, dec):
    prune(S, shapes, scale)
    return {"status": "authored", "denominator": int(scale), "validRange": {"minDenominator": RANGES[scale][0], "maxDenominator": RANGES[scale][1]},
            "pens": PENS[scale], "hatches": HATCHES[scale], "geometry": {"elevation": to_json(shapes)}, "decisions": decisions_texts(dec)}
