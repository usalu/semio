"""🛵 Round 3 parts of vehicles/motor-scooter: the hand-stated numbers every scale shares (regions, materials, wheel sections, the spoke windows, the arc and channel writers) of the 125 cc steel-monocoque scooter of the Vespa Primavera class (1.85 x 0.74 x 1.15 m, wheelbase 1.34 m, 12 inch wheels)."""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


kit = load("r3-motor-scooter-kit")
AF, AR, H, PIVOT, HINGE = kit.AF, kit.AR, kit.H, kit.PIVOT, kit.HINGE
r4, r5 = kit.r4, kit.r5

PAINT, RUBBER, ALLOY, VINYL, IRON, EXHAUST, ENGINE, GLASS = "paint", "rubber", "alloy", "seat-vinyl", "cast-iron", "exhaust-steel", "engine-metal", "glass"
RG = {"panels": "body-panels", "steer": "steer-assembly", "lamp": "headlamp", "seat": "seat", "engine": "engine", "exhaust": "exhaust-system", "tyre": "tyres", "brake": "brake-disc", "rim": "rims"}
TONE = {"paint": 0.56, "trim": 0.74, "dark": 0.2, "tyre": 0.1, "rim": 0.78, "seat": 0.13, "engine": 0.36, "exhaust": 0.42, "glass": 0.9, "iron": 0.48, "steel": 0.4}

BAR = (0.288, 1.008)
GRIP = (0.213, 0.34)
CRANK = (-0.39, 0.255)
CLUTCH = AR
MUFFLER = ((-0.5, 0.255), (-0.88, 0.272), -0.15, 0.055)
TAILPIPE = (-0.914, 0.262, -0.15)
LINK_PIVOT = (0.745, 0.262)
FORK_Z = -0.078
SHOCK_Z = -0.118
STALK = [(0.288, 1.008, 0.135), (0.275, 1.075, 0.19), (0.272, 1.106, 0.3)]
MIRROR = (0.27, 1.108, 0.314)


def tyre(width):
    """🛞 The closed section [lateral, radius] of a 12 inch scooter tyre of the given width on its rim: beads at the rim flange, upright sidewalls, a rounded crown."""
    h = width / 2
    side = [[-0.047, 0.166], [-h, 0.184], [-h, 0.2], [-h + 0.007, 0.216], [-h + 0.023, 0.2265], [-0.012, 0.23]]
    return side + [[-a, r] for a, r in side[::-1]] + [[0.04, 0.15], [-0.04, 0.15], [-0.047, 0.166]]


RIM = [[-0.047, 0.166], [-0.04, 0.15], [-0.012, 0.143], [0.012, 0.143], [0.04, 0.15], [0.047, 0.166], [0.049, 0.16], [0.045, 0.14], [0.012, 0.134], [-0.012, 0.134], [-0.045, 0.14], [-0.049, 0.16], [-0.047, 0.166]]


def arc(c, r, a0, a1, n):
    return [[c[0] + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)), c[1] + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))] for k in range(n + 1)]


def band(c, r0, r1, a0, a1, n):
    """🌈 The closed side outline of an arc band about c between the radii r0 and r1 from a0 to a1 degrees."""
    return arc(c, r1, a0, a1, n) + arc(c, r0, a1, a0, n)


def window(c, k, count=5, r0=0.062, r1=0.126, spoke=0.02, phase=90.0, n=5):
    """🪟 Window k of a cast wheel with `count` spokes: the opening between two spokes of the given width, between the hub radius r0 and the rim radius r1."""
    step = 360.0 / count
    mid = phase + (k + 0.5) * step
    out = []
    for r, sign in ((r1, 1), (r0, -1)):
        half = step / 2 - math.degrees(math.asin(min(0.99, spoke / 2 / r)))
        pts = arc(c, r, mid - half, mid + half, n)
        out += pts if sign > 0 else pts[::-1]
    return out


def channel(S, i, bone, c, profile, a0, a1, material, tone, role="body"):
    """🌙 A channel section revolved about the lateral axis through the world point c between the world angles a0 and a1 (degrees, counter-clockwise in the side view); the kernel sweeps counter-clockwise from the bone axis u, so the solid is turned to start at a0."""
    deg = kit.BONES[bone][1]
    o = kit.loc(bone, c)
    return S.solid(i, "revolve", bone, material, tone, role, profile=[[r5(a), r5(b)] for a, b in profile], axis=[0.0, 0.0, 1.0], origin=[0.0, 0.0, 0.0], sweepRad=round(math.radians(a1 - a0), 6), transform={"translation": kit.vec(o), "rotation": kit.rot_w(a0 - deg)})


def ring_path(o, r1, r0=None, holes=()):
    """⭕ The path of a disc or ring about the bone-local point o with true arcs and polygonal windows."""

    def circle(r):
        return f"M{r4(o[0] + r)} {r4(o[1])} A{r4(r)} {r4(r)} 0 1 0 {r4(o[0] - r)} {r4(o[1])} A{r4(r)} {r4(r)} 0 1 0 {r4(o[0] + r)} {r4(o[1])} Z"

    d = circle(r1)
    if r0 is not None:
        d += " " + circle(r0)
    for h in holes:
        d += " M" + " L".join(f"{r4(p[0])} {r4(p[1])}" for p in h) + " Z"
    return d


def ring2d(S, i, bone, c, r1, r0, z0, z1, role, tone, region, material, views=("elevation", "front", "plan"), pen=None, holes=(), regions=None):
    """🥏 A disc, ring or spoked disc about a lateral axis from z0 to z1: true arcs in the side view, its bounding bar from ahead and from above."""
    o = kit.loc(bone, c)
    s = {"kind": "path", "id": i, "bone": bone, "role": role, "z": 0, "filled": True, "tone": tone, "d": ring_path(o, r1, r0, [[kit.loc(bone, p)[:2] for p in h] for h in holes])}
    if regions is not None:
        s["regions"] = regions
    else:
        s["region"] = region
    s["material"] = material
    if pen is not None:
        s["penMm"] = pen
    s["solid"] = {"kind": "extruded", "sweep": "w", "z0": r4(min(z0, z1)), "z1": r4(max(z0, z1))}
    S.shape(views, s, [c[0] + r1, c[1] + r1, min(z0, z1)])


def many(S, i, regions):
    """🧩 Binds the drawn shape i to several thermal regions (a merged form)."""
    for shapes in S.shapes.values():
        for s in shapes:
            if s["id"] == i:
                s.pop("region", None)
                s["regions"] = regions
