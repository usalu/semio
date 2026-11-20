"""🚲 Round 3 parts of vehicles/city-bicycle, shared by the three authored scales: the key points of the real Dutch city bicycle in world metres and the hand-stated sections, profiles and paths of its assemblies. Every scale file states which of them it draws and how far it merges them."""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


kit = load("r3-city-bicycle-kit")
R, B, F, S, P = kit.R, kit.B, kit.F, kit.S, kit.P
sp = kit.steer_pt
ALL = ("elevation", "front", "plan")
SIDE = ("elevation",)

ST = (-math.cos(math.radians(68)), math.sin(math.radians(68)))
SEAT_TOP = [B[0] + 0.55 * ST[0], B[1] + 0.55 * ST[1], 0.0]
POST_TOP = [B[0] + 0.675 * ST[0], B[1] + 0.675 * ST[1], 0.0]
LOOP_END = [B[0] + 0.2 * ST[0], B[1] + 0.2 * ST[1], 0.0]
HEAD_TOP, HEAD_BOTTOM = sp(0.012, 0), sp(0.232, 0)
DOWN_HEAD, LOOP_HEAD = sp(0.2, 0), sp(0.055, 0)
B3 = [B[0], B[1], 0.0]
R3 = [R[0], R[1], 0.0]
F3 = [F[0], F[1], 0.0]
DOWN_DIR = kit.unit(kit.sub(B3, DOWN_HEAD))
LOOP_Q1 = kit.add(LOOP_HEAD, DOWN_DIR, 0.36)
LOOP_C = kit.add(LOOP_HEAD, DOWN_DIR, 0.52)
LOOP = [LOOP_HEAD] + kit.bezier(LOOP_Q1, LOOP_C, LOOP_END, 10)
CROWN_U = 0.258
QUILL_TOP = sp(-0.085, 0)
CLAMP = sp(-0.095, 0.045)
BAR_CORNER = [CLAMP[0], 1.046, 0.21]
BAR_END = [0.01, 1.055, 0.292]
GRIP = 0.12
LAMP = sp(0.215, 0.075)
LAMP_AXIS = [math.cos(math.radians(-5)), math.sin(math.radians(-5)), 0.0]
TAIL_DEG = 160.0
CASE_Z = (-0.066, -0.036)
ARM_Z = 0.084
RACK_Y = 0.795
RACK_Z = 0.07
RACK_X = (-0.44, -0.855)
STAY_Z = 0.064
SADDLE = [(-0.22, 0.984, 0.012, 0.008), (-0.228, 0.985, 0.02, 0.012), (-0.25, 0.984, 0.028, 0.016), (-0.3, 0.981, 0.043, 0.019), (-0.35, 0.98, 0.072, 0.021), (-0.4, 0.982, 0.099, 0.022), (-0.44, 0.986, 0.107, 0.022), (-0.47, 0.991, 0.104, 0.02), (-0.485, 0.994, 0.092, 0.015), (-0.492, 0.996, 0.07, 0.008)]
BLADE = [(CROWN_U, 0.0, 0.046, 0.009, 0.013), (0.33, 0.0, 0.0472, 0.0088, 0.0125), (0.4, 0.0, 0.0483, 0.0085, 0.0115), (0.46, 0.0006, 0.0492, 0.0082, 0.0105), (0.5, 0.002, 0.0498, 0.008, 0.0098), (0.53, 0.0048, 0.0503, 0.0078, 0.0092), (0.56, 0.01, 0.0507, 0.0076, 0.0086), (0.585, 0.017, 0.0511, 0.0074, 0.008), (0.61, 0.026, 0.0515, 0.0072, 0.0075), (0.6409, 0.0445, 0.052, 0.007, 0.007)]
REAR_HUB = [(-0.034, 0.012), (-0.034, 0.022), (-0.0305, 0.022), (-0.0305, 0.037), (-0.0275, 0.037), (-0.0275, 0.03), (0.0275, 0.03), (0.0275, 0.037), (0.0305, 0.037), (0.0305, 0.024), (0.036, 0.024), (0.036, 0.045), (0.056, 0.045), (0.056, 0.012)]
FRONT_HUB = [(-0.038, 0.011), (-0.038, 0.02), (-0.0335, 0.02), (-0.0335, 0.034), (-0.0305, 0.034), (-0.0305, 0.045), (0.002, 0.045), (0.01, 0.03), (0.0205, 0.03), (0.0205, 0.034), (0.0235, 0.034), (0.0235, 0.022), (0.026, 0.022), (0.026, 0.042), (0.042, 0.042), (0.042, 0.011)]
RIM = [(-0.0125, 0.3175), (-0.0125, 0.302), (-0.008, 0.297), (0.008, 0.297), (0.0125, 0.302), (0.0125, 0.3175), (0.0095, 0.3175), (0.0095, 0.308), (-0.0095, 0.308), (-0.0095, 0.3175), (-0.0125, 0.3175)]


def tyre_profile(n=24, half=0.0185, height=0.0185, centre=0.3265):
    return [(half * math.cos(2 * math.pi * k / n), centre + height * math.sin(2 * math.pi * k / n)) for k in range(n)] + [(half, centre)]


def fender_profile(half=0.024, crown=kit.FENDER_R, skirt=0.01, t=0.00072, n=8):
    """🌧️ The section of a steel mudguard: a shallow arch of sheet, [lateral, radial], as a closed loop."""
    outer = [(-half + 2 * half * k / n, crown + t / 2 - skirt * (2 * k / n - 1) ** 2) for k in range(n + 1)]
    inner = [(z, r - t) for z, r in reversed(outer)]
    return outer + inner + [outer[0]]


def ring(c, r_out, r_in, n=48):
    return [[c[0] + r_out * math.cos(2 * math.pi * k / n), c[1] + r_out * math.sin(2 * math.pi * k / n)] for k in range(n)], [[c[0] + r_in * math.cos(2 * math.pi * k / n), c[1] + r_in * math.sin(2 * math.pi * k / n)] for k in range(n)]


def half_rings(c, r_out, r_in, n=24):
    """🌗 A ring as two half rings (upper, lower): simple outlines without a hole, so the ring keeps its opening in front and plan when the wheel is steered."""
    out = []
    for a0 in (0.0, 180.0):
        out.append(kit.arc(c, r_out, a0, a0 + 180.0, n) + kit.arc(c, r_in, a0 + 180.0, a0, n))
    return out


def case_outline(n=10, grow=0.0):
    """⛓️ The side outline of the closed chain case: a housing of 94 mm radius about the chainring, one of 52 mm about the sprocket and the two tangent runs."""
    r0, r1 = 0.094 + grow, 0.052 + grow
    d = math.hypot(R[0] - B[0], R[1] - B[1])
    base = math.degrees(math.atan2(R[1] - B[1], R[0] - B[0]))
    a = math.degrees(math.acos((r0 - r1) / d))
    front = kit.arc(B, r0, base + a, base + 360 - a, 2 * n)
    rear = kit.arc(R, r1, base - a, base + a, n)
    return front + rear


def stay_points():
    """🔩 The four stays: (start, end, start radius, end radius) for the left side; the right side mirrors z."""
    lug = [SEAT_TOP[0] - 0.02 * ST[0], SEAT_TOP[1] - 0.02 * ST[1]]
    return {
        "chainstay": ([B[0] - 0.02, B[1] + 0.002, 0.03], [R[0], R[1], STAY_Z], 0.011, 0.008),
        "seatstay": ([lug[0], lug[1], 0.022], [R[0] + 0.004, R[1] + 0.014, STAY_Z], 0.008, 0.0065),
    }


def spoke(flange_r, flange_z, deg, lead, rim_r=0.2975):
    a = math.radians(deg)
    b = math.radians(deg + (60 if lead else -60))
    return [flange_r * math.cos(a), flange_r * math.sin(a), flange_z], [rim_r * math.cos(b), rim_r * math.sin(b), 0.0]


def bar_path(side, n=6):
    """🤲 The centre line of one half of the swept-back handlebar from the stem clamp to the grip end."""
    pts = kit.bent([CLAMP, BAR_CORNER, BAR_END], [0.085], n)
    return [[p[0], p[1], p[2] * side] for p in pts]


def grip_start(side):
    d = kit.unit(kit.sub(BAR_CORNER, BAR_END))
    g = kit.add(BAR_END, d, GRIP)
    return [g[0], g[1], g[2] * side]


def mirror(p, side):
    return [p[0], p[1], p[2] * side]


def frame_com():
    """⚖️ The centre of mass of the frame bone from a hand list of its parts (kg, x, y): seat tube and post, down tube, loop tube, head tube with headset, stays and dropouts, bottom bracket, carrier, saddle, chain case with chain and ring, rear mudguard, tail lamp, stand bracket."""
    parts = [(1.25, -0.22, 0.6), (0.85, 0.13, 0.52), (0.8, 0.1, 0.68), (0.45, 0.34, 0.82), (0.95, -0.4, 0.42), (0.55, -0.1, 0.3), (1.15, -0.64, 0.68), (1.0, -0.36, 0.97), (0.85, -0.33, 0.32), (0.45, -0.62, 0.6), (0.1, -0.94, 0.48), (0.1, -0.21, 0.26)]
    m = sum(p[0] for p in parts)
    return round(sum(p[0] * p[1] for p in parts) / m, 3), round(sum(p[0] * p[2] for p in parts) / m, 3), round(m, 2)
