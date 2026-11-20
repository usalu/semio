"""🧮 Geometry kit of the market parasol (round 3): the real linkage (crown, eight ribs at 45 degree spacing with an 11 degree plan offset, runner, eight stretchers), the product's bone frame convention (angle, abduction, twist) and world/local transforms.

See ./r3-objects-market-parasol.md and the product evaluator
🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts.
"""
import json
import math
from pathlib import Path

T = Path(__file__).resolve().parent
CATALOG = Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/market-parasol/🔣️.json")
GEN = T / "🗑️generated" / "r3-market-parasol"
SOURCE = T / "volumes" / "objects" / "market-parasol.json"

PIVOT_Y = 0.26
CROWN_Y = 2.35
MAST_L = CROWN_Y - PIVOT_Y
RUNNER_OPEN_H = 0.3926
RUNNER_X = MAST_L - RUNNER_OPEN_H
RIB_EL = -15.2803
RIB_L = 1.3676
STRETCH_L = 0.55
FURL_DX = -0.65381
FLUTTER_DX = 0.04896
RIBS = ["right", "b", "c", "d", "left", "f", "g", "h"]
AZ = {r: 11.0 + 45.0 * i for i, r in enumerate(RIBS)}
REST_RIB = {r: (0.0 if math.cos(math.radians(AZ[r])) > 0 else 180.0) for r in RIBS}
STRETCH_REST_RIGHT = 29.0537
REST_STRETCH = {r: (STRETCH_REST_RIGHT if math.cos(math.radians(AZ[r])) > 0 else 180.0 - STRETCH_REST_RIGHT) for r in RIBS}


def rib_id(r):
    return f"rib-{r}"


def stretch_id(r):
    return f"stretch-{r}"


def nxt(r):
    return RIBS[(RIBS.index(r) + 1) % 8]


def add(a, b):
    return [a[i] + b[i] for i in range(3)]


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def mul(a, s):
    return [x * s for x in a]


def dot(a, b):
    return sum(a[i] * b[i] for i in range(3))


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def unit(a):
    n = math.sqrt(dot(a, a))
    return [x / n for x in a]


def direction(az, el):
    """🧭 World unit vector of plan azimuth az (atan2(-z, x)) and elevation el, in degrees."""
    a, e = math.radians(az), math.radians(el)
    return [math.cos(e) * math.cos(a), math.sin(e), -math.cos(e) * math.sin(a)]


def frame(angle, abduction, twist):
    """🦴 The product's bone frame in its parent's frame: u from the in-plane angle and the abduction, v the in-plane normal turned by the twist about u (v' = cos t v - sin t w)."""
    A, b, t = (math.radians(x) for x in (angle, abduction, twist))
    u = [math.cos(A) * math.cos(b), math.sin(A) * math.cos(b), math.sin(b)]
    v0 = [-math.sin(A), math.cos(A), 0.0]
    w0 = cross(u, v0)
    v = sub(mul(v0, math.cos(t)), mul(w0, math.sin(t)))
    w = add(mul(v0, math.sin(t)), mul(w0, math.cos(t)))
    return u, v, w


def wrap(a):
    while a > 180:
        a -= 360
    while a <= -180:
        a += 360
    return a


def rib_el_for(h, s0):
    """📏 Rib elevation (degrees) that closes the stretcher linkage for a runner h below the crown."""
    s = (STRETCH_L ** 2 - s0 ** 2 - h ** 2) / (2 * s0 * h)
    return math.degrees(math.asin(s))


def stretch_s0():
    """📍 Distance of the stretcher pin along the rib, from the open pose (crown to runner 0.3926 m, rib at -15.2803 degrees)."""
    h, e = RUNNER_OPEN_H, math.radians(RIB_EL)
    b = 2 * h * math.sin(e)
    c = h * h - STRETCH_L ** 2
    return (-b + math.sqrt(b * b - 4 * c)) / 2


S0 = stretch_s0()


def linkage(h, el):
    """🔗 World directions (relative to the crown, resp. the runner) and pose offsets of ribs and stretchers for a runner h below the crown and ribs at el."""
    out = {}
    for r in RIBS:
        d = direction(AZ[r], el)
        dn = direction(AZ[nxt(r)], el)
        n = unit(cross(d, dn))
        if n[1] < 0:
            n = mul(n, -1)
        cb = math.sqrt(d[0] ** 2 + d[1] ** 2)
        A = math.degrees(math.atan2(d[1] / cb, d[0] / cb))
        b = math.degrees(math.asin(d[2]))
        u, v0, w0 = frame(A, b, 0)
        t = math.degrees(math.atan2(-dot(n, w0), dot(n, v0)))
        p = mul(d, S0)
        sd = unit(add(p, [0, h, 0]))
        cs = math.sqrt(sd[0] ** 2 + sd[1] ** 2)
        As = math.degrees(math.atan2(sd[1] / cs, sd[0] / cs))
        bs = math.degrees(math.asin(sd[2]))
        out[r] = {
            "dir": d,
            "normal": n,
            "rib": {"angleDeg": round(wrap(A - REST_RIB[r]), 4), "abductionDeg": round(b, 4), "twistDeg": round(wrap(t), 4)},
            "stretch": {"angleDeg": round(wrap(As - REST_STRETCH[r]), 4), "abductionDeg": round(bs, 4)},
            "frame": frame(A, b, t),
        }
    return out


OPEN = linkage(RUNNER_OPEN_H, RIB_EL)
FURL_H = RUNNER_OPEN_H - FURL_DX
FURL_EL = rib_el_for(FURL_H, S0)
FURLED = linkage(FURL_H, FURL_EL)
FLUTTER_H = RUNNER_OPEN_H - FLUTTER_DX
FLUTTER_EL = rib_el_for(FLUTTER_H, S0)
FLUTTER = linkage(FLUTTER_H, FLUTTER_EL)
CROWN = [0.0, CROWN_Y, 0.0]


def to_local(fr, origin, p):
    """↘️ World point p in the bone frame fr = (u, v, w) at origin."""
    q = sub(p, origin)
    return [dot(q, fr[0]), dot(q, fr[1]), dot(q, fr[2])]


def to_world(fr, origin, q):
    """↗️ Bone-local point q of the frame fr at origin in the world."""
    return add(origin, add(add(mul(fr[0], q[0]), mul(fr[1], q[1])), mul(fr[2], q[2])))


def r6(x):
    return round(x, 6)


def load_json(path):
    return json.loads(Path(path).read_text(encoding="utf-8"))
