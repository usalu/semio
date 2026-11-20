"""🧰 Round 3 parts of vehicles/cargo-bike, shared by the four authored scales: the key points of the real long-john cargo bike in world metres and the hand-stated sections, profiles and paths of its assemblies. Every scale file states which of them it draws and how far it merges them."""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


kit = load("r3-cargo-bike-kit")
bp, fp = kit.bar_pt, kit.fork_pt
ALL = ("elevation", "front", "plan")
SIDE = ("elevation",)
R3 = [kit.R[0], kit.R[1], 0.0]
F3 = [kit.F[0], kit.F[1], 0.0]
B3 = [kit.B[0], kit.B[1], 0.0]

ST = (-math.cos(math.radians(68)), math.sin(math.radians(68)))
SEAT_TOP = [kit.B[0] + 0.50 * ST[0], kit.B[1] + 0.50 * ST[1], 0.0]
POST_TOP = [kit.B[0] + 0.71 * ST[0], kit.B[1] + 0.71 * ST[1], 0.0]
STAY_Z = 0.0715

HEAD_R = (0.075, 0.43)
HEAD_F = (0.335, 0.475)
CROWN_F = (0.288, 0.31)
DRUM_F = (0.312, 0.331)
DRUM_R = (0.045, 0.07)
RISER_TOP = 0.844
CLAMP_S = 0.82
CLAMP = bp(CLAMP_S, 0.032)
BAR_CORNER = [CLAMP[0] - 0.035, CLAMP[1] + 0.006, 0.17]
BAR_END = [CLAMP[0] - 0.205, CLAMP[1] - 0.012, 0.298]
GRIP = 0.125

BEAM_Y = 0.255
BEAM_HALF = (0.025, 0.035)
BEAM_X0 = 0.555
BEAM_X1 = 1.68
GOOSE = [(180.0 - 6.0 * k, 0.32 + 0.015 * k / 10.0) for k in range(11)]

BOX_X0, BOX_X1, BOX_XT = 0.86, 1.60, 1.69
BOX_Y0, BOX_YF, BOX_YT = 0.29, 0.302, 0.62
BOX_HW = 0.32
PLY = 0.012
BENCH_X1, BENCH_Y = 1.13, 0.50
CAP = (0.022, 0.016)

CRATE = (0.40, 0.60, 0.27)

CARRIER_Y = 0.748
CARRIER_X = (-0.30, 0.235)
CARRIER_Z = 0.075

FENDER_R = (0.345, 0.272)
FENDER_HALF = 0.0325

SADDLE = [(0.338, 0.986, 0.014, 0.008), (0.325, 0.987, 0.024, 0.014), (0.295, 0.986, 0.038, 0.019), (0.255, 0.983, 0.056, 0.022), (0.205, 0.982, 0.083, 0.024), (0.155, 0.984, 0.100, 0.025), (0.115, 0.987, 0.106, 0.024), (0.088, 0.990, 0.099, 0.020), (0.075, 0.992, 0.085, 0.013), (0.070, 0.993, 0.062, 0.007)]

REAR_HUB = [(-0.0675, 0.008), (-0.0675, 0.014), (-0.042, 0.014), (-0.042, 0.030), (-0.032, 0.030), (-0.032, 0.046), (-0.028, 0.046), (-0.028, 0.033), (0.028, 0.033), (0.028, 0.046), (0.032, 0.046), (0.032, 0.030), (0.040, 0.030), (0.040, 0.024), (0.052, 0.024), (0.052, 0.014), (0.0675, 0.014), (0.0675, 0.008)]
FRONT_HUB = [(-0.046, 0.007), (-0.046, 0.012), (-0.034, 0.012), (-0.034, 0.031), (-0.030, 0.031), (-0.030, 0.021), (0.026, 0.021), (0.026, 0.031), (0.030, 0.031), (0.030, 0.024), (0.036, 0.024), (0.036, 0.012), (0.046, 0.012), (0.046, 0.007)]
REAR_RIM = [(-0.015, 0.2875), (-0.015, 0.280), (-0.011, 0.2735), (0.011, 0.2735), (0.015, 0.280), (0.015, 0.2875), (0.0115, 0.2875), (0.0115, 0.2835), (-0.0115, 0.2835), (-0.0115, 0.2875), (-0.015, 0.2875)]
FRONT_RIM = [(-0.015, 0.2115), (-0.015, 0.204), (-0.011, 0.1975), (0.011, 0.1975), (0.015, 0.204), (0.015, 0.2115), (0.0115, 0.2115), (0.0115, 0.2075), (-0.0115, 0.2075), (-0.0115, 0.2115), (-0.015, 0.2115)]
WHEELS = {
    "wheel-r": {"c": R3, "tyre": (0.3065, 0.025, 0.0235), "rim": REAR_RIM, "rim_r": (0.2735, 0.2875), "hub": REAR_HUB, "flanges": ((0.030, 0.044, 0.0), (-0.030, 0.044, 10.0)), "spoke_r": 0.2745, "rotor": (0.052, 0.054, 0.024, 0.090), "axle": 0.087, "nut": 0.0795, "drop": (0.0675, 0.0755), "out": 0.33, "inner": 0.2735},
    "wheel-f": {"c": F3, "tyre": (0.2315, 0.025, 0.0235), "rim": FRONT_RIM, "rim_r": (0.1975, 0.2115), "hub": FRONT_HUB, "flanges": ((0.028, 0.029, 0.0), (-0.032, 0.029, 10.0)), "spoke_r": 0.1985, "rotor": (0.0335, 0.0355, 0.024, 0.090), "axle": 0.066, "nut": 0.058, "drop": (0.046, 0.054), "out": 0.255, "inner": 0.1975},
}
CHAINRING = (0.077, -0.0495)
SPROCKET = 0.0405


def tyre_profile(centre, half, height, n=24):
    return [(half * math.cos(2 * math.pi * k / n), centre + height * math.sin(2 * math.pi * k / n)) for k in range(n)] + [(half, centre)]


def fender_profile(crown, half=FENDER_HALF, skirt=0.012, t=0.0025, n=8):
    """🌧️ The section of a plastic mudguard: a shallow arch of 2.5 mm sheet, [lateral, radial], as a closed loop."""
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


def spoke(flange_r, flange_z, deg, lead, rim_r):
    a = math.radians(deg)
    b = math.radians(deg + (60 if lead else -60))
    return [flange_r * math.cos(a), flange_r * math.sin(a), flange_z], [rim_r * math.cos(b), rim_r * math.sin(b), 0.0]


def beam_path(n_goose=None):
    """🦒 The centre line of the main beam: straight under the box from the motor bracket, a 9 cm fillet behind the front wheel, then the gooseneck that bends up around the wheel at 32 to 33.5 cm from its axle into the front head tube."""
    arc = [(deg, r) for deg, r in GOOSE if deg < 168.0]
    if n_goose is not None:
        arc = [arc[round(k * (len(arc) - 1) / n_goose)] for k in range(n_goose + 1)]
    at = lambda deg, r: [kit.F[0] + r * math.cos(math.radians(deg)), kit.F[1] + r * math.sin(math.radians(deg)), 0.0]
    fillet = kit.bezier([BEAM_X1 - 0.08, BEAM_Y, 0.0], [BEAM_X1, BEAM_Y, 0.0], at(*GOOSE[2]), 4 if n_goose is None else 2)
    return [[BEAM_X0, BEAM_Y, 0.0]] + fillet + [at(deg, r) for deg, r in arc]


def box_side():
    return [[BOX_X0, BOX_Y0], [BOX_X1, BOX_Y0], [BOX_XT, BOX_YT], [BOX_X0, BOX_YT]]


def front_wall():
    k = (BOX_XT - BOX_X1) / (BOX_YT - BOX_Y0)
    d = PLY * math.sqrt(1 + k * k)
    x0 = BOX_X1 + k * (BOX_YF - BOX_Y0)
    return [[x0 - d, BOX_YF], [x0, BOX_YF], [BOX_XT, BOX_YT], [BOX_XT - d, BOX_YT]]


def bar_path(side, n=6):
    """🤲 The centre line of one half of the swept-back handlebar from the stem clamp to the grip end."""
    pts = kit.bent([CLAMP, BAR_CORNER, BAR_END], [0.09], n)
    return [[p[0], p[1], p[2] * side] for p in pts]


def grip_start(side):
    d = kit.unit(kit.sub(BAR_CORNER, BAR_END))
    g = kit.add(BAR_END, d, GRIP)
    return [g[0], g[1], g[2] * side]


def mirror(p, side):
    return [p[0], p[1], p[2] * side]


def chain_runs():
    """⛓️ The upper and lower runs of the chain: the outer tangents of the 38-tooth ring (r 77 mm) and the 20-tooth sprocket (r 40.5 mm)."""
    r0, r1 = CHAINRING[0], SPROCKET
    bx, by = kit.B
    rx, ry = kit.R
    d = math.hypot(bx - rx, by - ry)
    base = math.atan2(ry - by, rx - bx)
    a = math.acos((r0 - r1) / d)
    out = []
    for s in (1, -1):
        q = base + s * a
        out.append(([bx + r0 * math.cos(q), by + r0 * math.sin(q)], [rx + r1 * math.cos(q), ry + r1 * math.sin(q)]))
    return out


def cable_path(side):
    """🪢 The housed steering cable of one side: from the rear drum under the handlebar column down beside the beam, along it under the box and up the gooseneck to the front drum on the fork crown."""
    z = side * 0.034
    a = bp(sum(DRUM_R) / 2, 0.0)
    f = fp(sum(DRUM_F) / 2, 0.0)
    pts = [[a[0] + 0.012, a[1], z], [a[0] + 0.05, 0.300, z], [0.93, 0.272, z], [BEAM_X1, 0.272, z]]
    for deg in (170.0, 158.0, 146.0, 134.0):
        r = 0.32 - 0.004
        pts.append([kit.F[0] + r * math.cos(math.radians(deg)), kit.F[1] + r * math.sin(math.radians(deg)) + 0.017 * math.sin(math.radians(180 - deg)), z])
    pts.append([f[0] - 0.03, f[1] - 0.012, z])
    pts.append([f[0] - 0.006, f[1], z])
    return kit.bent(pts, [0.03, 0.06, 0.06, 0.03, 0.03, 0.03, 0.03, 0.02], 3)


def frame_com():
    """⚖️ The centre of mass of the frame bone from a hand list of its parts (kg, x, y): beam with gooseneck, front head tube with headset, rear head tube with headset and strut, down tube, seat tube with post and clamp, chain- and seatstays with dropouts, motor bracket, box cross members, stand bracket, rear carrier with tail lamp, rear mudguard, saddle, chain and cables, rear brake caliper."""
    parts = [(4.55, 1.15, 0.29), (0.55, 1.83, 0.62), (0.75, 0.79, 0.53), (0.75, 0.64, 0.52), (1.25, 0.33, 0.68), (1.25, 0.23, 0.45), (0.7, 0.48, 0.30), (0.9, 1.27, 0.28), (0.2, 1.55, 0.21), (1.3, -0.03, 0.70), (0.45, -0.05, 0.45), (0.75, 0.20, 0.99), (0.45, 0.23, 0.30), (0.15, 0.06, 0.37)]
    m = sum(p[0] for p in parts)
    return round(sum(p[0] * p[1] for p in parts) / m, 3), round(sum(p[0] * p[2] for p in parts) / m, 3), round(m, 2)
