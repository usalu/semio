"""🦾 Excavator linkage (round 3): the pin geometry of a 20 t crawler excavator with a 5.7 m boom and a 2.9 m stick, the closure of the three cylinders and of the bucket four-bar, and the working envelope it gives. Usage: python3 r3-excavator-kin.py (prints the envelope against the manufacturer ranges)."""
import math

DECK = 1.06
PIN = {
    "A": (0.15, 0.86),
    "C": (0.552, 0.14),
    "D": (2.45, 0.66),
    "E": (3.0, 0.92),
    "F": (-0.794, 0.457),
    "H": (0.25, 0.56),
    "I": (2.44, 0.05),
    "J": (-0.06, 0.47),
}
BOOM, STICK, TIP = 5.7, 2.9, 1.47
IDLER, LINK = 0.64, 0.66
CYL = {
    "boom": {"closed": 1.85, "stroke": 1.30, "bore": 0.120, "rod": 0.085, "barrel": 0.145},
    "stick": {"closed": 2.18, "stroke": 1.50, "bore": 0.140, "rod": 0.100, "barrel": 0.168},
    "bucket": {"closed": 1.70, "stroke": 1.10, "bore": 0.120, "rod": 0.085, "barrel": 0.145},
}


def rot(p, deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return (p[0] * c - p[1] * s, p[0] * s + p[1] * c)


def add(a, b):
    return (a[0] + b[0], a[1] + b[1])


def sub(a, b):
    return (a[0] - b[0], a[1] - b[1])


def dist(a, b):
    return math.hypot(a[0] - b[0], a[1] - b[1])


def ang(a, b):
    return math.degrees(math.atan2(b[1] - a[1], b[0] - a[0]))


def circles(c0, r0, c1, r1, side):
    """The intersection of two circles on the left (side +1) or right (-1) of the line c0 to c1; None when they do not meet."""
    d = dist(c0, c1)
    if d > r0 + r1 or d < abs(r0 - r1) or d == 0:
        return None
    a = (r0 * r0 - r1 * r1 + d * d) / (2 * d)
    h = math.sqrt(max(0.0, r0 * r0 - a * a))
    ex, ey = (c1[0] - c0[0]) / d, (c1[1] - c0[1]) / d
    return (c0[0] + a * ex - side * h * ey, c0[1] + a * ey + side * h * ex)


def chain(theta, sigma, beta):
    """World pins (upper frame, deck at y = 0) for the boom chord angle theta, the stick angle sigma relative to the boom and the bucket angle beta relative to the stick, all in degrees."""
    A = PIN["A"]
    boom = lambda p: add(A, rot(p, theta))
    B = boom((BOOM, 0))
    stick = lambda p: add(B, rot(p, theta + sigma))
    G = stick((STICK, 0))
    bucket = lambda p: add(G, rot(p, theta + sigma + beta))
    out = {"A": A, "C": PIN["C"], "D": boom(PIN["D"]), "E": boom(PIN["E"]), "B": B, "F": stick(PIN["F"]), "H": stick(PIN["H"]), "I": stick(PIN["I"]), "G": G, "J": bucket(PIN["J"]), "T": bucket((TIP, 0))}
    out["K"] = circles(out["I"], IDLER, out["J"], LINK, +1)
    return out


def lengths(theta, sigma, beta):
    p = chain(theta, sigma, beta)
    return {"boom": dist(p["C"], p["D"]), "stick": dist(p["E"], p["F"]), "bucket": None if p["K"] is None else dist(p["H"], p["K"])}, p


def solve(name, target, lo, hi, other):
    """The joint angle in lo..hi at which the cylinder `name` has the pin length `target` (bisection; the length is monotonic over the working range)."""
    f = lambda x: (lengths(*other(x))[0][name] or 1e9) - target
    a, b = lo, hi
    fa = f(a)
    for _ in range(80):
        m = (a + b) / 2
        fm = f(m)
        if (fa < 0) == (fm < 0):
            a, fa = m, fm
        else:
            b = m
    return (a + b) / 2


def limits():
    c = CYL
    t0 = solve("boom", c["boom"]["closed"], -80, 20, lambda x: (x, -90, 0))
    t1 = solve("boom", c["boom"]["closed"] + c["boom"]["stroke"], 20, 100, lambda x: (x, -90, 0))
    s0 = solve("stick", c["stick"]["closed"], -90, -5, lambda x: (0, x, 0))
    s1 = solve("stick", c["stick"]["closed"] + c["stick"]["stroke"], -179, -60, lambda x: (0, x, 0))
    b0 = solve("bucket", c["bucket"]["closed"], -30, 42, lambda x: (0, -90, x))
    b1 = solve("bucket", c["bucket"]["closed"] + c["bucket"]["stroke"], -200, -30, lambda x: (0, -90, x))
    return {"boom": (t0, t1), "stick": (s1, s0), "bucket": (b1, b0)}


def envelope():
    lim = limits()
    best = {"reach": -1e9, "depth": 1e9, "height": -1e9, "dump": -1e9}
    n = 40
    for i in range(n + 1):
        th = lim["boom"][0] + (lim["boom"][1] - lim["boom"][0]) * i / n
        for j in range(n + 1):
            sg = lim["stick"][0] + (lim["stick"][1] - lim["stick"][0]) * j / n
            for k in range(n + 1):
                be = lim["bucket"][0] + (lim["bucket"][1] - lim["bucket"][0]) * k / n
                p = chain(th, sg, be)
                tx, ty = p["T"][0], p["T"][1] + DECK
                if abs(ty) < 0.08:
                    best["reach"] = max(best["reach"], tx)
                best["depth"] = min(best["depth"], ty)
                best["height"] = max(best["height"], ty)
                if th + sg + be < -85 and th + sg + be > -95:
                    best["dump"] = max(best["dump"], ty)
    return lim, best


REST = {"boom": 20.0, "stick": -100.0, "bucket": -40.0}
OLD = {"A": (1.0, 0.65), "deck": 1.0, "boom": 40.0, "stick": -90.0, "bucket": 30.0, "tip": 1.5}


def old_target(off):
    """The tooth tip and the bucket direction of the round 2 rig for its key offsets, in the new upper frame."""
    th = OLD["boom"] + off.get("boom", {}).get("angleDeg", 0)
    sg = OLD["stick"] + off.get("stick", {}).get("angleDeg", 0)
    be = OLD["bucket"] + off.get("bucket", {}).get("angleDeg", 0)
    B = add(OLD["A"], rot((BOOM, 0), th))
    G = add(B, rot((STICK, 0), th + sg))
    T = add(G, rot((OLD["tip"], 0), th + sg + be))
    return (T[0], T[1] + OLD["deck"] - DECK), th + sg + be


def ik(T, phi, lim):
    """Boom, stick and bucket angles that put the tooth tip at T with the bucket pointing along phi (boom above the line from the foot to the bucket pin), each clamped to its cylinder range."""
    G = sub(T, rot((TIP, 0), phi))
    A = PIN["A"]
    d = min(dist(A, G), BOOM + STICK - 1e-6)
    d = max(d, abs(BOOM - STICK) + 1e-6)
    th = ang(A, G) + math.degrees(math.acos((BOOM * BOOM + d * d - STICK * STICK) / (2 * BOOM * d)))
    th = min(max(th, lim["boom"][0]), lim["boom"][1])
    B = add(A, rot((BOOM, 0), th))
    sg = (ang(B, G) - th + 180) % 360 - 180
    sg = min(max(sg, lim["stick"][0]), lim["stick"][1])
    be = (phi - th - sg + 180) % 360 - 180
    if be > 90:
        be -= 360
    be = min(max(be, lim["bucket"][0]), lim["bucket"][1])
    return th, sg, be


def offsets(th, sg, be):
    """Every pose offset of the rig for the three joint angles: the three joints, the cylinder swings, the rod travels and the two links of the bucket four-bar, all relative to the rest pose."""
    rest = chain(REST["boom"], REST["stick"], REST["bucket"])
    now = chain(th, sg, be)
    r3 = lambda v: round(v, 3)
    r4 = lambda v: round(v, 4)
    wrap = lambda v: (v + 180) % 360 - 180
    out = {"boom": {"angleDeg": r3(th - REST["boom"])}, "stick": {"angleDeg": r3(sg - REST["stick"])}, "bucket": {"angleDeg": r3(be - REST["bucket"])}}
    for name, a, b, parent, parent0 in (("boom", "C", "D", 0.0, 0.0), ("stick", "E", "F", th, REST["boom"]), ("bucket", "H", "K", th + sg, REST["boom"] + REST["stick"])):
        swing = wrap((ang(now[a], now[b]) - parent) - (ang(rest[a], rest[b]) - parent0))
        out[name + "-cyl"] = {"angleDeg": r3(swing)}
        out[name + "-rod"] = {"dx": r4(dist(now[a], now[b]) - dist(rest[a], rest[b]))}
    idler = wrap((ang(now["I"], now["K"]) - (th + sg)) - (ang(rest["I"], rest["K"]) - (REST["boom"] + REST["stick"])))
    link = wrap((ang(now["K"], now["J"]) - ang(now["I"], now["K"])) - (ang(rest["K"], rest["J"]) - ang(rest["I"], rest["K"])))
    out["idler-link"] = {"angleDeg": r3(idler)}
    out["bucket-link"] = {"angleDeg": r3(link)}
    return out


if __name__ == "__main__":
    lim, best = envelope()
    print("joint ranges", {k: (round(v[0], 1), round(v[1], 1)) for k, v in lim.items()})
    print("max reach at ground %.2f (9.70 to 9.86)  max depth %.2f (6.62 to 6.72)  max cutting height %.2f (9.37 to 10.00)  max dump height, teeth down %.2f (6.49 to 7.11)" % (best["reach"], -best["depth"], best["height"], best["dump"]))
    for be in range(int(lim["bucket"][0]), int(lim["bucket"][1]) + 1, 20):
        l, p = lengths(0, -90, be)
        K = p["K"]
        tr = None if K is None else (ang(K, p["J"]) - ang(K, p["I"])) % 360
        print("bucket", be, "cyl", None if l["bucket"] is None else round(l["bucket"], 3), "angle at K", None if tr is None else round(tr, 1))
