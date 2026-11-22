"""🎒️ A daypack built on the back of a torso bone: one superelliptic loft per scale, its three exact orthographic drawings and its structure.

The pack hangs on its own fixed bone placed in the torso frame; its sections follow the back surface of the sweatshirt loft of that scale,
so the inner face lies 4 mm inside the garment. Elevation shapes are the exact side profile of the sections in the pack frame; front and
plan shapes are the convex hulls of the sampled loft in rest world coordinates (the loft is convex in both directions)."""
import math


def superellipse(rx, ry, n, k=48):
    """⭕️ Points of |x/rx|^n + |y/ry|^n = 1."""
    out = []
    for i in range(k):
        t = 2 * math.pi * i / k
        c, s = math.cos(t), math.sin(t)
        out.append((math.copysign(abs(c) ** (2 / n), c) * rx, math.copysign(abs(s) ** (2 / n), s) * ry))
    return out


def hull(points):
    """🧷️ Convex hull (Andrew's monotone chain), counter-clockwise."""
    pts = sorted(set((round(p[0], 6), round(p[1], 6)) for p in points))
    if len(pts) < 3:
        return pts

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    lower, upper = [], []
    for p in pts:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], p) <= 0:
            lower.pop()
        lower.append(p)
    for p in reversed(pts):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], p) <= 0:
            upper.pop()
        upper.append(p)
    return lower[:-1] + upper[:-1]


def back_surface(loft, u, w=0.0):
    """📏️ The back (+v) of a sweatshirt loft whose sections are rotated so that rx is the half width (w) and ry the half depth (v)."""
    secs = loft["sections"]
    for a, b in zip(secs, secs[1:]):
        if a["at"][0] <= u <= b["at"][0] or b["at"][0] <= u <= a["at"][0]:
            t = 0 if b["at"][0] == a["at"][0] else (u - a["at"][0]) / (b["at"][0] - a["at"][0])
            lerp = lambda p, q: p + (q - p) * t
            vc, rx, ry, n = lerp(a["at"][1], b["at"][1]), lerp(a["rx"], b["rx"]), lerp(a["ry"], b["ry"]), lerp(a["exponent"], b["exponent"])
            f = max(0.0, 1 - abs(w / rx) ** n) ** (1 / n)
            return vc + ry * f
    raise ValueError(f"u {u} outside loft")


def world(frame, p):
    """🌍️ A bone-local point (u, v, w) in rest world coordinates."""
    o, U, V, W = frame["o"], frame["u"], frame["v"], frame["w"]
    return [o[i] + p[0] * U[i] + p[1] * V[i] + p[2] * W[i] for i in range(3)]


def build(profile, loft, origin_u, embed, exponent):
    """🧮️ Pack sections in the torso frame: profile rows (u from the bottom, half width, half depth); returns (sections, origin (u, v))."""
    v0 = back_surface(loft, origin_u) - embed
    secs = []
    for du, hw, hd in profile:
        u = origin_u + du
        vin = back_surface(loft, u) - embed
        secs.append({"u": u, "vc": vin + hd, "hw": hw, "hd": hd})
    return secs, (origin_u, v0)


def loft_solid(sid, secs, origin, material, tone, exponent, role="body"):
    """🧊️ The pack loft in the pack bone frame (rx = half depth along v, ry = half width along w)."""
    return {"id": sid, "kind": "loft", "bone": "backpack", "material": material, "role": role, "tone": tone,
            "sections": [{"at": [round(s["u"] - origin[0], 5), round(s["vc"] - origin[1], 5), 0.0], "rx": round(s["hd"], 5), "ry": round(s["hw"], 5),
                          "exponent": exponent, "rotation": [0.5, 0.5, 0.5, 0.5]} for s in secs], "capStart": True, "capEnd": True}


def elevation_poly(secs, origin):
    """✏️ Exact side profile of the loft in the pack frame."""
    front = [(s["u"] - origin[0], s["vc"] - s["hd"] - origin[1]) for s in secs]
    back = [(s["u"] - origin[0], s["vc"] + s["hd"] - origin[1]) for s in reversed(secs)]
    return [[round(x, 5), round(y, 5)] for x, y in front + back]


def views(secs, torso_frame, exponent):
    """🗺️ Front (lateral, up) and plan (ahead, lateral) hulls of the loft in rest world coordinates."""
    pts = []
    for s in secs:
        for dv, dw in superellipse(s["hd"], s["hw"], exponent, 64):
            pts.append(world(torso_frame, (s["u"], s["vc"] + dv, dw)))
    front = hull([(p[2], p[1]) for p in pts])
    plan = hull([(p[0], p[2]) for p in pts])
    return [[round(x, 5), round(y, 5)] for x, y in front], [[round(x, 5), round(y, 5)] for x, y in plan]


def surface_area(secs, exponent, contact_fraction):
    """📐️ Outer area of the loft: side bands between sections plus the two caps, less the share of the inner face resting on the back."""
    def perim(hd, hw):
        p = superellipse(hd, hw, exponent, 96)
        return sum(math.dist(p[i], p[(i + 1) % len(p)]) for i in range(len(p)))

    def area(hd, hw):
        p = superellipse(hd, hw, exponent, 96)
        return abs(sum(p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1] for i in range(len(p)))) / 2

    side = sum((perim(a["hd"], a["hw"]) + perim(b["hd"], b["hw"])) / 2 * math.hypot(b["u"] - a["u"], b["hd"] - a["hd"]) for a, b in zip(secs, secs[1:]))
    caps = area(secs[0]["hd"], secs[0]["hw"]) + area(secs[-1]["hd"], secs[-1]["hw"])
    inner = sum((2 * a["hw"] + 2 * b["hw"]) / 2 * (b["u"] - a["u"]) for a, b in zip(secs, secs[1:]))
    return side + caps - contact_fraction * inner
