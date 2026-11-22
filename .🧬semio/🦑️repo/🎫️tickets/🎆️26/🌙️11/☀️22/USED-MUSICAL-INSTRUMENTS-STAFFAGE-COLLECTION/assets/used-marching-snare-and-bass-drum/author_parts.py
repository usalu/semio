"""🥁 The hand-chosen numbers of the used marching snare and bass drum: rig frames and the parts of every authored scale (1:50, 1:100, 1:200, 1:500), each part a list of primitives in world rest coordinates.

Reference table and sources: `../../reports/used-marching-snare-and-bass-drum.md`.
"""
import math

from author_geometry import Box, Capsule, Ellipsoid, Frame, Prism, Ring, Tube, arc_path, q_axis

XS = 0.764
YB = 0.555
SNARE_LUGS = [9.0 + 36.0 * k for k in range(10)]
BASS_LUGS = [22.5 + 45.0 * k for k in range(8)]
BAR_Z = 0.095
BAR_R = 0.011
PAD_R = 0.022
SNARE_LIFT = (0.40, 0.66)
BASS_LIFT = 0.445
STICK_LIFT = (-0.047, 0.1595)
STICK_STRIKE = -40.0
STICK_RAISED = -22.0

BONES = [
    ("stand", None, Frame(0.0, 0.0, 0.0, 0.0), 0.265, {"type": "fixed"}, ("Storage cradle stand, from its centre along the feet", "Lagerbock, von der Mitte entlang der Füße")),
    ("bass-lift", "stand", Frame(0.0, YB, 0.0, 0.0), 0.0, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.46}, ("Lift of the bass drum off the cradle onto the carrier height", "Hub der großen Trommel vom Bock auf die Tragehöhe")),
    ("bass", "bass-lift", Frame(0.0, YB, 0.0, 0.0), 0.305, {"type": "fixed"}, ("Bass drum, from the axis to the front of the shell", "Große Trommel, von der Achse zur Kesselvorderseite")),
    ("bass-carrier", "bass", Frame(-0.3565, 0.545, 0.0, 90.0), 0.46, {"type": "fixed"}, ("Bass drum carrier, from the vest plate up to the shoulder bends", "Tragegestell der großen Trommel, von der Bauchplatte zu den Schulterbögen")),
    ("snare-lift", "stand", Frame(XS, 0.0, 0.0, 0.0), 0.0, {"type": "prismatic", "axisDeg": round(math.degrees(math.atan2(SNARE_LIFT[1], SNARE_LIFT[0])), 4), "minM": 0, "maxM": 0.8}, ("Lift of the snare drum from the floor onto the carrier height", "Hub der kleinen Trommel vom Boden auf die Tragehöhe")),
    ("snare", "snare-lift", Frame(XS, 0.0, 0.0, 90.0), 0.329, {"type": "fixed"}, ("Snare drum, up its axis from the bottom hoop to the top hoop", "Kleine Trommel, entlang der Achse vom unteren zum oberen Reifen")),
    ("snare-carrier", "snare", Frame(XS - 0.2615, 0.29, 0.0, 90.0), 0.50, {"type": "fixed"}, ("Snare drum carrier, from the vest plate up to the shoulder bends", "Tragegestell der kleinen Trommel, von der Bauchplatte zu den Schulterbögen")),
    ("snare-strainer", "snare", Frame(XS, 0.10, -0.204, -90.0), 0.05, {"type": "revolute", "minDeg": 0, "maxDeg": 180}, ("Throw-off lever of the snare strainer (down: snares off)", "Abhebehebel der Teppichabhebung (unten: Teppich ab)")),
    ("sticks", "snare", Frame(XS - 0.073, 0.3365, 0.0, 0.0), 0.0, {"type": "prismatic", "axisDeg": None, "minM": 0, "maxM": 0.2}, ("Pair of sticks, from lying across the hoop to the hands", "Stockpaar, vom Liegen auf dem Reifen in die Hände")),
    ("stick-r", "sticks", Frame(XS - 0.073, 0.3365, -0.035, 0.0), 0.276, {"type": "revolute", "minDeg": -45, "maxDeg": 25}, ("Right stick, from the fulcrum to the tip", "Rechter Stock, vom Drehpunkt zur Spitze")),
    ("stick-l", "sticks", Frame(XS - 0.073, 0.3365, 0.035, 0.0), 0.276, {"type": "revolute", "minDeg": -45, "maxDeg": 25}, ("Left stick, from the fulcrum to the tip", "Linker Stock, vom Drehpunkt zur Spitze")),
]
FRAMES = {b[0]: b[2] for b in BONES}


class Part:
    """🧩 One drawn and modelled part: its bone, primitives, material, region, role, tone and flags; `repeat` copies the first primitive about a drum axis."""

    def __init__(self, pid, bone, prims, material, region, role="body", tone=0.4, flags=None, views=("elevation", "front", "plan"), repeat=None, mass=None, struct=None, density_factor=1.0, solid=True, draw=True):
        self.id, self.bone, self.prims, self.material, self.region = pid, bone, prims, material, region
        self.role, self.tone, self.flags, self.views, self.repeat = role, tone, flags or {}, views, repeat
        self.mass, self.struct, self.density_factor, self.solid, self.draw = mass, struct, density_factor, solid, draw


def snare_box(alpha, r0, r1, a0, a1, tang):
    """📦 A box on the snare: radial r0..r1 from the drum axis at angle alpha (from ahead towards the left), axial a0..a1 above the bottom, tangential width tang."""
    rc = (r0 + r1) / 2
    a = math.radians(alpha)
    return Box((XS + rc * math.cos(a), (a0 + a1) / 2, rc * math.sin(a)), (r1 - r0, a1 - a0, tang), q_axis((0, 1, 0), -alpha))


def bass_box(beta, r0, r1, z0, z1, tang):
    """📦 A box on the bass drum: radial r0..r1 from the drum axis at angle beta (from ahead towards up), lateral z0..z1, tangential width tang."""
    rc = (r0 + r1) / 2
    b = math.radians(beta)
    return Box((rc * math.cos(b), YB + rc * math.sin(b), (z0 + z1) / 2), (r1 - r0, tang, z1 - z0), q_axis((0, 0, 1), beta))


def snare_lug(alpha, r_axis, a0, a1, r):
    """🪵 A tube lug of the snare: a cylinder parallel to the drum axis at radius r_axis and angle alpha."""
    a = math.radians(alpha)
    return Ring("y", (XS + r_axis * math.cos(a), 0.0, r_axis * math.sin(a)), a0, a1, 0.0, r)


def j_path(xp, y0, y_plate_top, y_shoulder, n_arc=12):
    """🪝 The centre line of a J-shaped shoulder bar in the x-y plane: up the front of the vest plate, up the chest, over the shoulder and down the back."""
    pts = [(xp, y0), (xp, y_plate_top), (xp - 0.021, y_shoulder)]
    arc = arc_path(xp - 0.111, y_shoulder, 0.09, 0.0, 180.0, n_arc)
    pts += arc[1:]
    pts.append((xp - 0.201, y_shoulder - 0.12))
    return pts


def pad_path(xp, y_shoulder, n=8):
    """🧽 The centre line of a shoulder pad (sling) over the bend of a bar."""
    return arc_path(xp - 0.111, y_shoulder, 0.09, 15.0, 165.0, n)


SNARE_XP = XS - 0.249
SNARE_JS = 0.70
BASS_XP = -0.344
BASS_JS = YB + 0.36


def stand_geometry():
    """🛠 The numbers of the tubular storage cradle: hub height and arm direction tangent to the bass shell at 40 degrees from its bottom."""
    gamma = 40.0
    t = (0.319 * math.sin(math.radians(gamma)), YB - 0.319 * math.cos(math.radians(gamma)))
    hub = t[1] - t[0] * math.tan(math.radians(gamma))
    d = (math.cos(math.radians(gamma)), math.sin(math.radians(gamma)))
    return hub, d, t


HUB, ARM_DIR, TANGENT = stand_geometry()


def arm_point(sign, s, y_add=0.0):
    """📏 A point at distance s along the front (sign +1) or back (sign -1) arm of the cradle."""
    return (sign * ARM_DIR[0] * s, HUB + ARM_DIR[1] * s + y_add)


def parts_50():
    """🔍 Every part of the asset at 1:50 (smallest feature 7.5 mm)."""
    P = []
    for side, z in (("r", -0.08), ("l", 0.08)):
        P.append(Part(f"foot-{side}", "stand", [Capsule((-0.262, 0.011, z), (0.262, 0.011, z), 0.010)], "steel-coated", "stand-frame", tone=0.25, density_factor=0.2775, struct="member"))
        for end, x in (("rear", -0.262), ("front", 0.262)):
            P.append(Part(f"cap-{end}-{side}", "stand", [Capsule((x - 0.012 if x < 0 else x - 0.004, 0.011, z), (x + 0.004 if x < 0 else x + 0.012, 0.011, z), 0.011)], "rubber", "stand-frame", role="detail", tone=0.12, struct="joint"))
        P.append(Part(f"upright-{side}", "stand", [Capsule((0.0, 0.011, z), (0.0, HUB, z), 0.010)], "steel-coated", "stand-frame", tone=0.25, density_factor=0.2775, struct="member"))
        for arm, sign in (("front", 1), ("rear", -1)):
            a = arm_point(sign, 0.0)
            b = arm_point(sign, 0.30)
            P.append(Part(f"arm-{arm}-{side}", "stand", [Capsule((a[0], a[1], z), (b[0], b[1], z), 0.010)], "steel-coated", "stand-frame", tone=0.25, density_factor=0.2775, struct="member"))
            s0, s1 = arm_point(sign, 0.215), arm_point(sign, 0.293)
            P.append(Part(f"sleeve-{arm}-{side}", "stand", [Capsule((s0[0], s0[1], z), (s1[0], s1[1], z), 0.0145)], "rubber", "stand-frame", role="detail", tone=0.12, struct="joint"))
    P.append(Part("foot-cross", "stand", [Capsule((0.0, 0.011, -0.08), (0.0, 0.011, 0.08), 0.010)], "steel-coated", "stand-frame", tone=0.25, density_factor=0.2775, struct="member"))
    P.append(Part("hub-cross", "stand", [Capsule((0.0, HUB, -0.08), (0.0, HUB, 0.08), 0.010)], "steel-coated", "stand-frame", tone=0.25, density_factor=0.2775, struct="member"))

    P.append(Part("bass-shell", "bass", [Ring("z", (0.0, YB, 0.0), -0.178, 0.178, 0.297, 0.305)], "birch-lacquer", "bass-shell", tone=0.62, struct="member"))
    for side, s in (("r", -1), ("l", 1)):
        P.append(Part(f"bass-head-{side}", "bass", [Ring("z", (0.0, YB, 0.0), (0.178 if s > 0 else -0.1785), (0.1785 if s > 0 else -0.178), 0.0, 0.3055)], "calfskin", "bass-heads", tone=0.82, flags={"sheet": True}, views=("elevation",)))
        P.append(Part(f"bass-hoop-{side}", "bass", [Ring("z", (0.0, YB, 0.0), (0.158 if s > 0 else -0.200), (0.200 if s > 0 else -0.158), 0.3065, 0.3245)], "beech-lacquer", "bass-hoops", tone=0.5, struct="member"))
        zl0, zl1 = (0.108, 0.155) if s > 0 else (-0.155, -0.108)
        zr0, zr1 = (0.153, 0.190) if s > 0 else (-0.190, -0.153)
        zc0, zc1 = (0.188, 0.206) if s > 0 else (-0.206, -0.188)
        zt0, zt1 = (0.206, 0.216) if s > 0 else (-0.216, -0.206)
        P.append(Part(f"bass-lugs-{side}", "bass", [bass_box(BASS_LUGS[0], 0.3045, 0.339, zl0, zl1, 0.020)], "zinc-nickel", "bass-hardware", role="detail", tone=0.7, repeat=("z", (0.0, YB, 0.0), 8, 45.0), density_factor=0.5))
        P.append(Part(f"bass-rods-{side}", "bass", [bass_box(BASS_LUGS[0], 0.331, 0.339, zr0, zr1, 0.008)], "steel-nickel", "bass-hardware", role="detail", tone=0.75, repeat=("z", (0.0, YB, 0.0), 8, 45.0)))
        P.append(Part(f"bass-claws-{side}", "bass", [bass_box(BASS_LUGS[0], 0.320, 0.342, zc0, zc1, 0.018)], "steel-nickel", "bass-hardware", role="detail", tone=0.75, repeat=("z", (0.0, YB, 0.0), 8, 45.0)))
        P.append(Part(f"bass-handles-{side}", "bass", [bass_box(BASS_LUGS[0], 0.329, 0.341, zt0, zt1, 0.044)], "steel-nickel", "bass-hardware", role="detail", tone=0.75, repeat=("z", (0.0, YB, 0.0), 8, 45.0)))
    P.append(Part("bass-cleat", "bass", [bass_box(120.0, 0.3240, 0.3335, -0.196, -0.162, 0.060)], "beech-lacquer", "bass-hoops", role="detail", tone=0.42))
    P.append(Part("bass-mount", "bass", [Box((-0.308, YB, 0.0), (0.010, 0.060, 0.080))], "aluminium", "bass-hardware", role="detail", tone=0.55, struct="joint"))

    P.append(Part("bass-plate", "bass-carrier", [Box((-0.3565, 0.545, 0.0), (0.003, 0.22, 0.24))], "aluminium", "carrier-frames", tone=0.2, flags={"sheet": True}, struct="member"))
    P.append(Part("bass-belly-pad", "bass-carrier", [Box((-0.364, 0.545, 0.0), (0.012, 0.20, 0.22))], "foam-pad", "carrier-pads", role="detail", tone=0.1))
    P.append(Part("bass-arm", "bass-carrier", [Box((-0.334, YB, 0.0), (0.044, 0.012, 0.050))], "aluminium", "carrier-frames", role="detail", tone=0.2, struct="joint"))
    for side, z in (("r", -BAR_Z), ("l", BAR_Z)):
        P.append(Part(f"bass-bar-{side}", "bass-carrier", Tube(j_path(BASS_XP, 0.545, 0.655, BASS_JS), z, BAR_R).segments(), "aluminium", "carrier-frames", tone=0.2, density_factor=0.331, struct="member"))
        P.append(Part(f"bass-sling-{side}", "bass-carrier", Tube(pad_path(BASS_XP, BASS_JS), z, PAD_R).segments(), "foam-pad", "carrier-pads", role="detail", tone=0.1, density_factor=0.6))

    P.append(Part("snare-shell", "snare", [Ring("y", (XS, 0.0, 0.0), 0.012, 0.317, 0.172, 0.178)], "birch-ply", "snare-wrap", tone=0.78, struct="member", density_factor=1.069))
    P.append(Part("snare-batter", "snare", [Ring("y", (XS, 0.0, 0.0), 0.317, 0.31725, 0.0, 0.1785)], "pet-coated", "snare-heads", tone=0.9, flags={"sheet": True}, views=("elevation", "plan")))
    P.append(Part("snare-reso", "snare", [Ring("y", (XS, 0.0, 0.0), 0.01175, 0.012, 0.0, 0.1785)], "pet-coated", "snare-heads", tone=0.9, flags={"sheet": True}, views=("elevation", "plan")))
    top = [(0.305, 0.1795), (0.329, 0.1795), (0.329, 0.1905), (0.325, 0.1905), (0.325, 0.1855), (0.305, 0.1855)]
    bot = [(0.0, 0.1795), (0.024, 0.1795), (0.024, 0.1855), (0.004, 0.1855), (0.004, 0.1905), (0.0, 0.1905)]
    P.append(Part("snare-hoop-top", "snare", [Ring("y", (XS, 0.0, 0.0), 0.305, 0.329, 0.1795, 0.1905, profile=top)], "chrome-steel", "snare-hardware", tone=0.85, density_factor=0.38, struct="member"))
    P.append(Part("snare-hoop-bottom", "snare", [Ring("y", (XS, 0.0, 0.0), 0.0, 0.024, 0.1795, 0.1905, profile=bot)], "chrome-steel", "snare-hardware", tone=0.85, density_factor=0.38, struct="member"))
    P.append(Part("snare-lugs", "snare", [snare_lug(SNARE_LUGS[0], 0.1845, 0.034, 0.295, 0.007)], "chrome-brass", "snare-hardware", role="detail", tone=0.88, repeat=("y", (XS, 0.0, 0.0), 10, 36.0), density_factor=0.265))
    P.append(Part("snare-ears-top", "snare", [snare_box(SNARE_LUGS[0], 0.1850, 0.2000, 0.295, 0.3245, 0.012)], "chrome-steel", "snare-hardware", role="detail", tone=0.88, repeat=("y", (XS, 0.0, 0.0), 10, 36.0)))
    P.append(Part("snare-ears-bottom", "snare", [snare_box(SNARE_LUGS[0], 0.1850, 0.2000, 0.0045, 0.034, 0.012)], "chrome-steel", "snare-hardware", role="detail", tone=0.88, repeat=("y", (XS, 0.0, 0.0), 10, 36.0)))
    P.append(Part("snare-strainer-body", "snare", [snare_box(270.0, 0.1775, 0.204, 0.03, 0.10, 0.036)], "chrome-steel", "snare-hardware", role="detail", tone=0.8))
    P.append(Part("snare-butt", "snare", [snare_box(90.0, 0.1775, 0.198, 0.03, 0.07, 0.030)], "chrome-steel", "snare-hardware", role="detail", tone=0.8))
    P.append(Part("snare-mount", "snare", [snare_box(180.0, 0.1775, 0.1865, 0.17, 0.23, 0.040)], "aluminium", "snare-hardware", role="detail", tone=0.55, struct="joint"))
    P.append(Part("snare-wires", "snare", [Box((XS, 0.011, 0.0), (0.33, 0.001, 0.08))], "steel-wire", "snare-hardware", role="detail", tone=0.6, flags={"sheet": True}, draw=False))
    P.append(Part("strainer-lever", "snare-strainer", [Box((XS, 0.075, -0.208), (0.010, 0.050, 0.008))], "chrome-steel", "snare-hardware", role="detail", tone=0.8))

    P.append(Part("snare-plate", "snare-carrier", [Box((XS - 0.2615, 0.29, 0.0), (0.003, 0.22, 0.24))], "aluminium", "carrier-frames", tone=0.2, flags={"sheet": True}, struct="member"))
    P.append(Part("snare-belly-pad", "snare-carrier", [Box((XS - 0.269, 0.29, 0.0), (0.012, 0.20, 0.22))], "foam-pad", "carrier-pads", role="detail", tone=0.1))
    P.append(Part("snare-arm", "snare-carrier", [Box((XS - 0.2235, 0.20, 0.0), (0.0765, 0.012, 0.040))], "aluminium", "carrier-frames", role="detail", tone=0.2, struct="joint"))
    for side, z in (("r", -BAR_Z), ("l", BAR_Z)):
        P.append(Part(f"snare-bar-{side}", "snare-carrier", Tube(j_path(SNARE_XP, 0.29, 0.40, SNARE_JS), z, BAR_R).segments(), "aluminium", "carrier-frames", tone=0.2, density_factor=0.331, struct="member"))
        P.append(Part(f"snare-sling-{side}", "snare-carrier", Tube(pad_path(SNARE_XP, SNARE_JS), z, PAD_R).segments(), "foam-pad", "carrier-pads", role="detail", tone=0.1, density_factor=0.6))

    for side, z in (("r", -0.035), ("l", 0.035)):
        f = (XS - 0.073, 0.3365)
        P.append(Part(f"stick-{side}", f"stick-{side}", [Capsule((f[0] - 0.1225, f[1], z), (f[0] + 0.18, f[1], z), 0.0075, 0.0072), Capsule((f[0] + 0.18, f[1], z), (f[0] + 0.262, f[1], z), 0.0072, 0.004), Ellipsoid((f[0] + 0.2705, f[1], z), (0.0055, 0.0045, 0.0045))], "hickory", "sticks", tone=0.68, struct="member"))
    return P
