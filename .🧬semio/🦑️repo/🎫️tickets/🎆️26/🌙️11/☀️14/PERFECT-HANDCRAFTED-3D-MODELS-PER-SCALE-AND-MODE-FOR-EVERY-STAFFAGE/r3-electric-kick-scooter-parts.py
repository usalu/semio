"""🛴 Round 3 parts of vehicles/electric-kick-scooter shared by the scale files: the hand-stated sections, paths and outlines of the real folding e-scooter (deck extrusion, gooseneck, head tube, rear stays, channel mudguards, tyre and motor sections, cast rear wheel) in world metres and the small writers the kit lacks (channel arcs about an axle, ring drawings with true arcs)."""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


kit = load("r3-electric-kick-scooter-kit")
AF, AR, H, P = kit.AF, kit.AR, kit.H, kit.P
r4, r5 = kit.r4, kit.r5

REGION = {"frame": "deck-battery", "mat": "deck-grip", "bms": "bms-controller", "motor": "hub-motor", "steer": "stem-bars", "guard": "fenders", "tyre": "tyres", "rotor": "brake-disc", "rim": "rear-rim"}

DECK_X = (-0.29, 0.24)
DECK_Y = (0.095, 0.15)
DECK_HALF = 0.075
DECK_SECTION = [[0.095, -0.06], [0.095, 0.06], [0.108, 0.075], [0.146, 0.075], [0.15, 0.071], [0.15, -0.071], [0.146, -0.075], [0.108, -0.075]]
NOSE_PLAN = [[-0.075, 0.232], [-0.075, 0.246], [-0.062, 0.262], [-0.04, 0.268], [0.04, 0.268], [0.062, 0.262], [0.075, 0.246], [0.075, 0.232]]
TAIL_PLAN = [[-0.075, -0.282], [0.075, -0.282], [0.075, -0.294], [0.0545, -0.316], [-0.0545, -0.316], [-0.075, -0.294]]
STAY_Z = (0.0425, 0.0545)
MAT_X = (-0.272, 0.212)
MAT_HALF = 0.066
MAT_Y = (0.1495, 0.158)
NECK = [((0.222, 0.124), 0.06, 0.026), ((0.252, 0.128), 0.052, 0.027), ((0.285, 0.165), 0.037, 0.026), ((0.325, 0.22), 0.028, 0.024), ((0.362, 0.27), 0.024, 0.024), ((0.396, 0.303), 0.022, 0.026)]
HEAD_Y = (0.262, 0.345)
HEAD_R = 0.026
CUP_R = 0.0285
CUP = 0.008
TYRE = [[-0.019, 0.068], [-0.0245, 0.078], [-0.025, 0.09], [-0.022, 0.1], [-0.015, 0.106], [-0.006, 0.108], [0.006, 0.108], [0.015, 0.106], [0.022, 0.1], [0.025, 0.09], [0.0245, 0.078], [0.019, 0.068], [-0.019, 0.068]]
MOTOR = [[-0.033, 0.0], [-0.033, 0.03], [-0.031, 0.058], [-0.0225, 0.066], [-0.0225, 0.077], [-0.018, 0.077], [-0.018, 0.07], [0.018, 0.07], [0.018, 0.077], [0.0225, 0.077], [0.0225, 0.066], [0.031, 0.058], [0.033, 0.03], [0.033, 0.0]]
RIM = [[-0.0225, 0.062], [-0.0225, 0.077], [-0.018, 0.077], [-0.018, 0.07], [0.018, 0.07], [0.018, 0.077], [0.0225, 0.077], [0.0225, 0.062], [-0.0225, 0.062]]
CHANNEL = [[-0.03, 0.115], [-0.03, 0.118], [-0.027, 0.121], [0.027, 0.121], [0.03, 0.118], [0.03, 0.115], [0.027, 0.115], [0.027, 0.1165], [0.0255, 0.118], [-0.0255, 0.118], [-0.027, 0.1165], [-0.027, 0.115], [-0.03, 0.115]]
GUARD_R = (0.115, 0.121)
GUARD_HALF = 0.03
GUARD_REAR = (17.0, 170.0)
GUARD_FRONT = (55.0, 148.0)
ROTOR = (0.035, 0.055)
ROTOR_Z = (0.031, 0.034)
BLADE_Z = (0.036, 0.047)
BAR_U = 0.719
AXIS_V = -kit.PIN_BACK
STEM_R = (0.022, 0.019)
BAR_HALF = 0.2
GRIP = (0.095, 0.215)


def axis_x(y):
    """🧭 The x of the steering axis at the height y."""
    return H[0] + (H[1] - y) * math.tan(math.radians(kit.RAKE))


def axis_pt(y, z=0.0):
    return [axis_x(y), y, z]


def polar(c, r, deg, z=None):
    p = [c[0] + r * math.cos(math.radians(deg)), c[1] + r * math.sin(math.radians(deg))]
    return p if z is None else p + [z]


def arc(c, r, a0, a1, n):
    return [polar(c, r, a0 + (a1 - a0) * k / n) for k in range(n + 1)]


def band(c, r0, r1, a0, a1, n):
    """🌈 The closed side outline of an arc band about c between the radii r0 and r1 from a0 to a1 degrees."""
    return arc(c, r1, a0, a1, n) + arc(c, r0, a1, a0, n)


def stay_outline():
    return [[-0.3, 0.096], [-0.3, 0.15], [-0.335, 0.146], [-0.4, 0.132]] + arc(AR, 0.022, 100, 260, 8) + [[-0.4, 0.087], [-0.335, 0.094]]


def blade_outline(top=0.135, wide=0.018, end=0.0185):
    v = kit.FORK_OFFSET
    return [kit.steer_pt(top, v + wide)[:2]] + arc(AF, end, 8, -172, 9) + [kit.steer_pt(top, v - wide)[:2]]


def window(k, r0=0.028, r1=0.058, spoke=0.007, phase=90.0):
    """🪟 One of the five windows between the cast spokes of the rear wheel, about the rear axle."""
    mid = phase + 36 + 72 * k
    h0 = 36 - math.degrees(math.asin(spoke / r0))
    h1 = 36 - math.degrees(math.asin(spoke / r1))
    return arc(AR, r1, mid - h1, mid + h1, 6) + arc(AR, r0, mid + h0, mid - h0, 3)


def channel(S, i, bone, c, profile, a0, a1, material, tone, role="body"):
    """🌙 A channel section revolved about the lateral axis through the world point c between the world angles a0 and a1 (degrees, counter-clockwise in the side view); the kernel sweeps counter-clockwise from the bone axis u, so the solid is turned to start at a0."""
    deg = kit.BONES[bone][1]
    o = kit.loc(bone, c)
    return S.solid(i, "revolve", bone, material, tone, role, profile=[[r5(a), r5(b)] for a, b in profile], axis=[0.0, 0.0, 1.0], origin=[0.0, 0.0, 0.0], sweepRad=round(math.radians(a1 - a0), 6), transform={"translation": kit.vec(o), "rotation": kit.rot_w(a0 - deg)})



def ring2d(S, i, bone, c, r1, r0, z0, z1, role, tone, region, material, views=("elevation", "front", "plan"), pen=None, windows=None):
    """⭕ A ring (or a disc with windows) about a lateral axis drawn with true arcs, pushed from z0 to z1: the annulus in the side view, its bounding bars from ahead and from above."""
    o = kit.loc(bone, c)

    def circle(r):
        return f"M{r4(o[0] + r)} {r4(o[1])} A{r4(r)} {r4(r)} 0 1 0 {r4(o[0] - r)} {r4(o[1])} A{r4(r)} {r4(r)} 0 1 0 {r4(o[0] + r)} {r4(o[1])} Z"

    d = circle(r1)
    if r0 is not None:
        d += " " + circle(r0)
    for w in windows or []:
        q = [kit.loc(bone, p) for p in w]
        d += " M" + " L".join(f"{r4(p[0])} {r4(p[1])}" for p in q) + " Z"
    s = {"kind": "path", "id": i, "bone": bone, "role": role, "z": 0, "filled": True, "tone": tone, "region": region, "material": material, "d": d, "solid": {"kind": "extruded", "sweep": "w", "z0": r4(z0), "z1": r4(z1)}}
    if pen is not None:
        s["penMm"] = pen
    S.shape(views, s, [c[0] + r1, c[1] + r1, z0])


def quad(a, b, h0, h1=None):
    """▱ The outline of a straight bar from a to b in the side view with the half widths h0 and h1."""
    h1 = h0 if h1 is None else h1
    dx, dy = b[0] - a[0], b[1] - a[1]
    L = math.hypot(dx, dy)
    nx, ny = -dy / L, dx / L
    return [[a[0] + nx * h0, a[1] + ny * h0], [a[0] - nx * h0, a[1] - ny * h0], [b[0] - nx * h1, b[1] - ny * h1], [b[0] + nx * h1, b[1] + ny * h1]]


def rounded(x0, x1, half, r, n=4):
    """▢ A rounded rectangle in plan as (w, u) points for an extrusion along v."""
    out = []
    for cx, cz, a0 in ((x1 - r, half - r, 0), (x0 + r, half - r, 90), (x0 + r, -half + r, 180), (x1 - r, -half + r, 270)):
        for k in range(n + 1):
            a = math.radians(a0 + 90 * k / n)
            out.append([r5(cz + r * math.sin(a)), r5(cx + r * math.cos(a))])
    return out


def neck_side(neck=NECK):
    return kit.ribbon([list(p[0]) for p in neck], [p[2] for p in neck])


def neck_plan(neck=NECK):
    top = [[p[0][0], p[1]] for p in neck]
    return top + [[q[0], -q[1]] for q in top[::-1]]


def neck_front(neck=NECK):
    side = [[p[1], p[0][1] - p[2]] for p in neck] + [[neck[-1][1], neck[-1][0][1] + neck[-1][2]]]
    return side + [[-q[0], q[1]] for q in side[::-1]]
