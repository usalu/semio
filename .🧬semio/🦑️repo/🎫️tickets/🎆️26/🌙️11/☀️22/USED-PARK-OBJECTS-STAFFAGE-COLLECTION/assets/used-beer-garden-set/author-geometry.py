"""🍺 Geometry kernel of the used beer garden set: rest frames of the rig, prism parts and their translation into body solids and 2D `solid` shapes.

Every part of the set is a prism: a profile in two bone axes swept along the third, so the 3D body solid and the 2D drawings of all three viewpoints are one definition.
`sw` parts lie on sagittal bones (table, benches: u = x, v = y, w = z) with the profile in (u, v) swept along w; `sv` parts lie on the trestle and brace bones,
whose joint axis w is the world x axis, with the profile in (u, w) swept along v.

@see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — "Solid placements", "Rest placement of a bone", "Volume"
"""
import math

FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}


def rot_w(a):
    """🔄 Rotation about the joint axis w by `a` degrees: u towards v."""
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return [[c, -s, 0], [s, c, 0], [0, 0, 1]]


def rot_v(a):
    """↪️ Rotation about the normal v by `a` degrees: u towards w."""
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return [[c, 0, -s], [0, 1, 0], [s, 0, c]]


def rot_u(a):
    """🌀 Rotation about the bone axis u by `a` degrees: v towards -w."""
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return [[1, 0, 0], [0, c, s], [0, -s, c]]


def mul(a, b):
    """✖️ 3x3 matrix product."""
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def apply(m, p):
    """➡️ Matrix times vector."""
    return [sum(m[i][k] * p[k] for k in range(3)) for i in range(3)]


def rest_frames(bones):
    """🦴 World rest frames {id: (origin, R)} with R holding the bone axes u, v, w as columns."""
    out = {}
    for b in bones:
        local = mul(mul(rot_w(b["angleDeg"]), rot_v(b.get("abductionDeg", 0))), rot_u(b.get("twistDeg", 0)))
        if b["parent"] is None:
            out[b["id"]] = ([b["x"], b["y"], b.get("z", 0)], local)
            continue
        o, r = out[b["parent"]]
        d = apply(r, [b["x"], b["y"], b.get("z", 0)])
        out[b["id"]] = ([o[i] + d[i] for i in range(3)], mul(r, local))
    return out


def to_world(frames, bone, u, v, w):
    """🌍 A bone-local rest point in world metres."""
    o, r = frames[bone]
    d = apply(r, [u, v, w])
    return [o[i] + d[i] for i in range(3)]


def signed_area(ring):
    """📐 Shoelace area, positive counter-clockwise."""
    return 0.5 * sum(ring[i][0] * ring[(i + 1) % len(ring)][1] - ring[(i + 1) % len(ring)][0] * ring[i][1] for i in range(len(ring)))


def ccw(ring):
    """🔁 The ring counter-clockwise."""
    return ring if signed_area(ring) > 0 else list(reversed(ring))


def rect(a0, a1, b0, b1):
    """⬛ A rectangle ring in the two profile coordinates."""
    return [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]


def r6(x):
    """🔢 Six decimals."""
    return round(x + 0.0, 6)


def part(pid, bone, kind, prof, lo, hi, material, region, tone, **extra):
    """🧩 One prism part: `kind` sw (profile u, v along w) or sv (profile u, w along v)."""
    return {"id": pid, "bone": bone, "kind": kind, "prof": ccw([(r6(a), r6(b)) for a, b in prof]), "lo": r6(lo), "hi": r6(hi), "material": material, "region": region, "tone": tone, **extra}


def body_solid(p):
    """🧊 The body solid of a part, in its bone frame."""
    s = {"id": p["id"], "kind": "extrude", "bone": p["bone"], "material": p["material"], "role": p.get("role", "body"), "tone": p["tone"]}
    if p["kind"] == "sw":
        s.update({"profile": [[a, b] for a, b in p["prof"]], "plane": "xy", "from": p["lo"], "to": p["hi"]})
    else:
        s.update({"profile": [[b, a] for a, b in reversed(p["prof"])], "plane": "zx", "from": p["lo"], "to": p["hi"]})
    for flag in ("slender", "sheet", "envelope", "texture"):
        if p.get(flag):
            s[flag] = True
    return s


def shape(p, frames, z, hatch=None, pen=None, view=None):
    """✏️ The 2D shape of a part in any viewpoint: its profile with the extruded `solid` that places it by the full bone frame."""
    sh = {"id": p["id"], "bone": p["bone"], "role": p.get("role", "body"), "z": z, "filled": True, "tone": p["tone"], "region": p["region"], "material": p["material"]}
    if hatch:
        sh["hatch"] = hatch
    if pen:
        sh["penMm"] = pen
    sh.update({"kind": "poly", "points": [[a, b] for a, b in p["prof"]], "closed": True})
    if p["kind"] == "sw":
        base = frames[p["bone"]][0][2]
        sh["solid"] = {"kind": "extruded", "sweep": "w", "z0": r6(base + p["lo"]), "z1": r6(base + p["hi"])}
    else:
        sh["solid"] = {"kind": "extruded", "sweep": "v", "z0": p["lo"], "z1": p["hi"]}
    return sh


def prism_volume(p):
    """⚖️ Volume of the prism in cubic metres."""
    return abs(signed_area(p["prof"])) * (p["hi"] - p["lo"])


def prism_centroid(p):
    """🎯 Centroid of the prism in its bone frame (u, v, w)."""
    ring = p["prof"]
    a = signed_area(ring)
    cx = sum((ring[i][0] + ring[(i + 1) % len(ring)][0]) * (ring[i][0] * ring[(i + 1) % len(ring)][1] - ring[(i + 1) % len(ring)][0] * ring[i][1]) for i in range(len(ring))) / (6 * a)
    cy = sum((ring[i][1] + ring[(i + 1) % len(ring)][1]) * (ring[i][0] * ring[(i + 1) % len(ring)][1] - ring[(i + 1) % len(ring)][0] * ring[i][1]) for i in range(len(ring))) / (6 * a)
    m = (p["lo"] + p["hi"]) / 2
    return (cx, cy, m) if p["kind"] == "sw" else (cx, m, cy)


def prism_area(p):
    """🧾 Whole surface of the prism (sides and both caps) in square metres."""
    ring = p["prof"]
    per = sum(math.dist(ring[i], ring[(i + 1) % len(ring)]) for i in range(len(ring)))
    return per * (p["hi"] - p["lo"]) + 2 * abs(signed_area(ring))


def world_box(p, frames):
    """📦 World axis-aligned box of a part at rest."""
    pts = []
    for a, b in p["prof"]:
        for c in (p["lo"], p["hi"]):
            pts.append(to_world(frames, p["bone"], a, b, c) if p["kind"] == "sw" else to_world(frames, p["bone"], a, c, b))
    return [min(q[i] for q in pts) for i in range(3)], [max(q[i] for q in pts) for i in range(3)]
