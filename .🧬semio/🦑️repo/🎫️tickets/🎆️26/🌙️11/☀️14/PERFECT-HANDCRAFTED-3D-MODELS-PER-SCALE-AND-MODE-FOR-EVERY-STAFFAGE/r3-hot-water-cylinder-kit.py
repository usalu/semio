"""🛢️ Round 3 hot-water cylinder kit: the measures of the real 200 l cylinder (EN 12897 class, Vitocell 100-V size, DIN 28011 Kloepper heads) and small writers for the solids. Every section is stated here or in `r3-hot-water-cylinder-model.py`; nothing is guessed from the drawings.

World frame: origin on the floor at the tank axis, y up, x ahead (to the right in the side elevation), z lateral (the side elevation looks from -z, so the dial and the type plate face -z). Every bone has angle 0, so a bone frame is the world frame moved to the bone origin.
"""
import math

ORIGIN = {"base": 0.0, "connections": 0.0, "zone-4": 0.06, "zone-3": 0.39, "zone-2": 0.72, "zone-1": 1.05, "cap": 1.38, "needle": 0.885}
ZONE_H = 0.33
JACKET_R = 0.30
TANK_RO = 0.240
TANK_S = 0.003
TANK_RI = TANK_RO - TANK_S
SHELL_LO = 0.20
SHELL_HI = 1.20
CROWN_R = 0.48
KNUCKLE_R = 0.048
CAP_A = 0.30
CAP_C = 0.10
CAP_SKIN = 0.003
FOOT_PITCH = 0.25
FEET = [(-FOOT_PITCH, 0.0), (0.5 * FOOT_PITCH, -FOOT_PITCH * math.sqrt(3) / 2), (0.5 * FOOT_PITCH, FOOT_PITCH * math.sqrt(3) / 2)]
STUBS = [("stub-0", -1, 1.1977), ("stub-1", -1, 0.7852), ("stub-2", -1, 0.1893), ("stub-3", 1, 0.9227), ("stub-4", 1, 0.5102)]
HOT_X = 0.2039
DIAL_Y = 0.165


def r5(x):
    """🔢 Rounds to 0.01 mm so the source stays readable."""
    return round(x + 0.0, 5)


def pts(seq):
    """📐 Rounds a point list."""
    return [[r5(a), r5(b)] for a, b in seq]


def head_hc():
    """⚖️ Height of the crown centre below the tangent line of a Kloepper head (DIN 28011: crown radius D, knuckle radius 0.1 D)."""
    cx = TANK_RO - KNUCKLE_R
    return -math.sqrt((CROWN_R - KNUCKLE_R) ** 2 - cx ** 2)


def head_depth(s=0.0):
    """📏 Depth of the head surface offset inwards by s above its tangent line."""
    return head_hc() + CROWN_R - s


def head(s=0.0, nk=4, nc=4):
    """🥣 (radial, height above the tangent line) of a Kloepper head surface offset inwards by s, from the rim to the apex."""
    cx = TANK_RO - KNUCKLE_R
    hc = head_hc()
    phi = math.atan2(-hc, cx)
    out = []
    for i in range(nk + 1):
        t = phi * i / nk
        out.append((cx + (KNUCKLE_R - s) * math.cos(t), (KNUCKLE_R - s) * math.sin(t)))
    for i in range(1, nc + 1):
        t = phi + (math.pi / 2 - phi) * i / nc
        out.append(((CROWN_R - s) * math.cos(t), hc + (CROWN_R - s) * math.sin(t)))
    out[-1] = (0.0, out[-1][1])
    return out


def head_height(r):
    """📍 Height of the outer head surface above its tangent line at radius r."""
    cx = TANK_RO - KNUCKLE_R
    hc = head_hc()
    if r >= CROWN_R * math.cos(math.atan2(-hc, cx)):
        return math.sqrt(max(KNUCKLE_R ** 2 - (r - cx) ** 2, 0.0))
    return hc + math.sqrt(CROWN_R ** 2 - r ** 2)


def ellipse(a, c, n, t0=0.0, t1=math.pi / 2):
    """🥚 (radial, height) of a quarter ellipse from the rim (t 0) to the apex (t pi/2)."""
    return [(a * math.cos(t0 + (t1 - t0) * i / n), c * math.sin(t0 + (t1 - t0) * i / n)) for i in range(n + 1)]


def loop(seq):
    """➰ Closes a profile into a loop (first point repeated last)."""
    seq = [list(p) for p in seq]
    return seq + [seq[0]]


def bead(axial, radial, r, n=6):
    """⭕ A closed profile loop of a small round bead (weld, seam) centred at (axial, radial)."""
    return loop([(axial + r * math.cos(2 * math.pi * i / n), radial + r * math.sin(2 * math.pi * i / n)) for i in range(n)])


def solid(kind, sid, bone, material, role, tone, **kw):
    """🧱 One solid with the keys in schema order."""
    s = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    s.update({k: v for k, v in kw.items() if v is not None})
    return s


def rev(sid, bone, material, role, tone, profile, origin=(0, 0, 0), axis=(0, 1, 0), **kw):
    """🌀 A revolve: profile of [axial, radial], open profiles close through the axis, loops are their own body."""
    return solid("revolve", sid, bone, material, role, tone, profile=pts(profile), axis=list(axis), origin=[r5(v) for v in origin], **kw)


def hexagon(cv, cw, rad):
    """⬡ A hexagon with a vertex up (v) around (cv, cw)."""
    return [[r5(cv + rad * math.cos(math.pi / 2 + i * math.pi / 3)), r5(cw + rad * math.sin(math.pi / 2 + i * math.pi / 3))] for i in range(6)]


def local(bone, y):
    """↕️ World height to bone-local height."""
    return y - ORIGIN[bone]


def jacket_point(x, r=JACKET_R):
    """🎯 The point of the jacket at lateral side -z for a given x (where the dial, plate and seam sit)."""
    return -math.sqrt(r * r - x * x)
