"""🎹 Hand-chosen geometry of the used two-manual home organ: rig frames, the parts of every authored scale and their projections.

Coordinates: world x ahead (towards the player), y up, z left of the asset; the rest pose is the stored pose (organ on its dolly).
`OL` points are organ-local (the cabinet base centre on the floor of the organ), world = OL + (0, H_D) at rest.
See README "Front and plan views" and "Volume" of the staffage product for the frames.
"""
import math

H_D = 0.125
P_KEY = 0.0235
W_KEY = 0.0225
OCT = 7 * P_KEY
Z0_LOWER = -0.3878
Z0_UPPER = -0.2233
BLACK_AFTER = [0, 1, 2, 4, 5]
COVER_ANGLE = math.degrees(math.atan2(0.735 - 0.983, 0.275 - (-0.030)))
COVER_LEN = math.hypot(0.275 + 0.030, 0.983 - 0.735)
COVER_OPEN = 90.0 - COVER_ANGLE
KEY_C_INDEX = 11

BONES = {
    "floor": dict(parent=None, o=(0.0, 0.0), th=0.0, length=0.05, joint={"type": "fixed"}),
    "carriage": dict(parent="floor", o=(0.0, 0.0), th=0.0, length=0.05, joint={"type": "prismatic", "axisDeg": 0, "minM": -1.2, "maxM": 0}),
    "dolly": dict(parent="carriage", o=(-0.345, 0.111), th=0.0, length=0.45, joint={"type": "prismatic", "axisDeg": 0, "minM": -1.0, "maxM": 0}),
    "cabinet": dict(parent="carriage", o=(0.0, H_D), th=90.0, length=0.983, joint={"type": "prismatic", "axisDeg": 90, "minM": -H_D, "maxM": 0}),
    "key-cover": dict(parent="cabinet", o=(-0.030, H_D + 0.983), th=COVER_ANGLE, length=COVER_LEN, joint={"type": "revolute", "minDeg": 0, "maxDeg": round(COVER_OPEN, 2)}),
    "pedalboard": dict(parent="cabinet", o=(0.054, H_D), th=90.0, length=0.25, joint={"type": "revolute", "minDeg": -90, "maxDeg": 0}),
    "swell-pedal": dict(parent="cabinet", o=(0.090, H_D + 0.050), th=10.0, length=0.16, joint={"type": "revolute", "minDeg": -14, "maxDeg": 0}),
    "lower-key": dict(parent="cabinet", o=(0.115, H_D + 0.683), th=0.0, length=0.15, joint={"type": "revolute", "minDeg": -4, "maxDeg": 0}),
    "bench": dict(parent="floor", o=(0.490, 0.0), th=90.0, length=0.575, joint={"type": "fixed"}),
}


def rot(p, deg):
    """🔄 Rotates a 2D vector by `deg` degrees counter-clockwise."""
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return (c * p[0] - s * p[1], s * p[0] + c * p[1])


def to_local(bone, p):
    """📐 World rest point (x, y) to the (u, v) frame of a bone."""
    b = BONES[bone]
    return rot((p[0] - b["o"][0], p[1] - b["o"][1]), -b["th"])


def to_world(bone, q):
    """🌍 Bone-local (u, v) to the world rest point (x, y)."""
    b = BONES[bone]
    r = rot(q, b["th"])
    return (b["o"][0] + r[0], b["o"][1] + r[1])


def rel_placement(bone):
    """🦴 Parent-relative x, y, angleDeg of a bone from its world rest frame."""
    b = BONES[bone]
    if b["parent"] is None:
        return b["o"][0], b["o"][1], b["th"]
    p = BONES[b["parent"]]
    x, y = rot((b["o"][0] - p["o"][0], b["o"][1] - p["o"][1]), -p["th"])
    return x, y, b["th"] - p["th"]


def ol(points):
    """🎹 Organ-local points to world rest points."""
    return [(x, y + H_D) for x, y in points]


def rect(x0, x1, y0, y1):
    """⬛ A counter-clockwise rectangle."""
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def circle(cx, cy, r, n=24):
    """⭕ A counter-clockwise polygon circle."""
    return [(round(cx + r * math.cos(2 * math.pi * k / n), 6), round(cy + r * math.sin(2 * math.pi * k / n), 6)) for k in range(n)]


def part(pid, bone, mat, region, tone, prof, z0, z1, frame="world", role="body", views=("elevation", "front", "plan"), rep=None, holes=None, sheet=False, texture=False, hatch=None, draw=None, thermal=True):
    """🧩 One part: a profile extruded along the lateral axis from z0 to z1 on a bone."""
    return dict(id=pid, bone=bone, mat=mat, region=region, tone=tone, prof=prof, z0=z0, z1=z1, frame=frame, role=role, views=set(views), rep=rep, holes=holes or [], sheet=sheet, texture=texture, hatch=hatch or {}, draw=draw or {}, thermal=thermal)


def local_profile(p):
    """🧭 The bone-local profile of a part."""
    if p["frame"] == "local":
        return [tuple(q) for q in p["prof"]], [[tuple(q) for q in h] for h in p["holes"]]
    return [to_local(p["bone"], q) for q in p["prof"]], [[to_local(p["bone"], q) for q in h] for h in p["holes"]]


def local_step(p):
    """➡️ The repeat step of a part in its bone frame."""
    n, (dx, dy, dz) = p["rep"]
    if p["frame"] == "local":
        return n, (dx, dy, dz)
    u, v = rot((dx, dy), -BONES[p["bone"]]["th"])
    return n, (u, v, dz)


def r6(x):
    """✂️ Rounds to 0.1 mm."""
    return round(x + 0.0, 5)


def white_keys(prefix, bone, x0, x1, y0, y1, z0, idx, tone, region="keys", mat="key-acrylic-yellowed", frame="world"):
    """🎼 A run of white keys from index list `idx` (consecutive) as one repeated part."""
    first, n = idx[0], len(idx)
    zs = z0 + first * P_KEY + 0.0005
    return part(prefix, bone, mat, region, tone, rect(x0, x1, y0, y1), r6(zs), r6(zs + W_KEY), frame=frame, role="detail", rep=(n, (0.0, 0.0, P_KEY)) if n > 1 else None)


def black_keys(prefix, bone, x0, x1, y0, y1, z0, whites, tone):
    """🎹 The black keys of a manual: one repeated part per position in the octave."""
    out = []
    names = {0: "fs", 1: "gs", 2: "as", 4: "cs", 5: "ds"}
    for k in BLACK_AFTER:
        count = len([i for i in range(whites - 1) if i % 7 == k])
        zc = z0 + (k + 1) * P_KEY
        out.append(part(f"{prefix}-{names[k]}", bone, "key-black-aged", "keys", tone, rect(x0, x1, y0, y1), r6(zc - 0.0055), r6(zc + 0.0055), role="detail", rep=(count, (0.0, 0.0, OCT)) if count > 1 else None))
    return out


def black_positions(z0, whites):
    """📍 Lateral centres of the black keys of a manual."""
    return [z0 + (i + 1) * P_KEY for i in range(whites - 1) if i % 7 in BLACK_AFTER]


CHEEK_50 = [(-0.275, 0.0), (0.245, 0.0), (0.255, 0.010), (0.255, 0.050), (0.215, 0.090), (0.215, 0.580), (0.275, 0.600), (0.275, 0.735), (-0.030, 0.983), (-0.275, 0.983)]
CHEEK_100 = [(-0.275, 0.0), (0.255, 0.0), (0.255, 0.050), (0.215, 0.090), (0.215, 0.580), (0.275, 0.600), (0.275, 0.735), (-0.030, 0.983), (-0.275, 0.983)]
CHEEK_200 = [(-0.275, 0.0), (0.255, 0.0), (0.255, 0.050), (0.215, 0.090), (0.215, 0.600), (0.275, 0.600), (0.275, 0.735), (-0.030, 0.983), (-0.275, 0.983)]
TAB_PANEL = [(0.014, 0.783), (0.030, 0.783), (-0.030, 0.965), (-0.046, 0.965)]


def tab_strip(t0, t1, depth):
    """🎛️ The voice-tab strip proud of the inclined tab panel between the fractions t0 and t1 of its face."""
    a, b = (0.030, 0.783), (-0.030, 0.965)
    d = (b[0] - a[0], b[1] - a[1])
    ln = math.hypot(*d)
    n = (d[1] / ln, -d[0] / ln)
    p0 = (a[0] + t0 * d[0], a[1] + t0 * d[1])
    p1 = (a[0] + t1 * d[0], a[1] + t1 * d[1])
    return [p0, p1, (p1[0] + depth * n[0], p1[1] + depth * n[1]), (p0[0] + depth * n[0], p0[1] + depth * n[1])]


def castor_parts(scale, tag, cx, cz):
    """🛞 One swivel castor of the dolly at world x = cx, z = cz."""
    if scale == 50:
        return [
            part(f"castor-{tag}-wheel", "dolly", "castor-rubber", "dolly", 0.15, circle(cx, 0.0375, 0.0375), cz - 0.0125, cz + 0.0125),
            part(f"castor-{tag}-fork", "dolly", "castor-steel", "dolly", 0.45, rect(cx - 0.022, cx + 0.022, 0.072, 0.092), cz - 0.022, cz + 0.022),
            part(f"castor-{tag}-leg", "dolly", "castor-steel", "dolly", 0.45, rect(cx - 0.012, cx + 0.012, 0.0375, 0.072), cz + 0.0125, cz + 0.0205, rep=(2, (0.0, 0.0, -0.033))),
            part(f"castor-{tag}-plate", "dolly", "castor-steel", "dolly", 0.5, rect(cx - 0.030, cx + 0.030, 0.092, 0.100), cz - 0.030, cz + 0.030),
        ]
    if scale == 100:
        return [
            part(f"castor-{tag}-wheel", "dolly", "castor-rubber", "dolly", 0.15, circle(cx, 0.0375, 0.0375, 16), cz - 0.0125, cz + 0.0125),
            part(f"castor-{tag}-housing", "dolly", "castor-steel", "dolly", 0.45, rect(cx - 0.0225, cx + 0.0225, 0.060, 0.100), cz - 0.0225, cz + 0.0225),
        ]
    return [part(f"castor-{tag}", "dolly", "castor-steel", "dolly", 0.25, rect(cx - 0.0375, cx + 0.0375, 0.0, 0.095), cz - 0.020, cz + 0.020)]


CASTORS = [("bl", -0.300, -0.250), ("br", -0.300, 0.250), ("fl", 0.060, -0.250), ("fr", 0.060, 0.250)]


def dolly_parts(scale):
    """🛒 The four-castor furniture dolly the organ stands on at the dealer."""
    out = []
    if scale in (50, 100):
        top = 0.122 if scale == 50 else 0.125
        out += [
            part("dolly-end-back", "dolly", "dolly-beech", "dolly", 0.6, rect(-0.345, -0.250, 0.100, top), -0.300, 0.300),
            part("dolly-end-front", "dolly", "dolly-beech", "dolly", 0.6, rect(0.010, 0.105, 0.100, top), -0.300, 0.300),
            part("dolly-rail-right", "dolly", "dolly-beech", "dolly", 0.6, rect(-0.250, 0.010, 0.100, top), -0.300, -0.205),
            part("dolly-rail-left", "dolly", "dolly-beech", "dolly", 0.6, rect(-0.250, 0.010, 0.100, top), 0.205, 0.300),
        ]
        if scale == 50:
            out += [
                part("dolly-carpet-back", "dolly", "dolly-carpet", "dolly", 0.35, rect(-0.345, -0.250, 0.122, 0.125), -0.300, 0.300, sheet=True),
                part("dolly-carpet-front", "dolly", "dolly-carpet", "dolly", 0.35, rect(0.010, 0.105, 0.122, 0.125), -0.300, 0.300, sheet=True),
                part("dolly-carpet-right", "dolly", "dolly-carpet", "dolly", 0.35, rect(-0.250, 0.010, 0.122, 0.125), -0.300, -0.205, sheet=True),
                part("dolly-carpet-left", "dolly", "dolly-carpet", "dolly", 0.35, rect(-0.250, 0.010, 0.122, 0.125), 0.205, 0.300, sheet=True),
            ]
    else:
        out.append(part("dolly-deck", "dolly", "dolly-beech", "dolly", 0.6, rect(-0.345, 0.105, 0.095, 0.125), -0.300, 0.300))
    for tag, cx, cz in CASTORS:
        out += castor_parts(scale, tag, cx, cz)
    return out


def bench_parts(scale):
    """🪑 The matching organ bench, standing where the player sits."""
    out = []
    zr, zl = (-0.410, -0.370), (0.310, 0.350)
    if scale == 50:
        leg_f = [(0.350, 0.0), (0.380, 0.0), (0.385, 0.450), (0.385, 0.520), (0.345, 0.520), (0.345, 0.450)]
        leg_b = [(0.600, 0.0), (0.630, 0.0), (0.635, 0.450), (0.635, 0.520), (0.595, 0.520), (0.595, 0.450)]
        out += [
            part("bench-leg-back", "bench", "bench-beech", "bench", 0.4, leg_f, zr[0], zr[1], rep=(2, (0.0, 0.0, 0.720))),
            part("bench-leg-front", "bench", "bench-beech", "bench", 0.4, leg_b, zr[0], zr[1], rep=(2, (0.0, 0.0, 0.720))),
            part("bench-apron-back", "bench", "bench-beech", "bench", 0.42, rect(0.345, 0.365, 0.450, 0.520), -0.370, 0.310),
            part("bench-apron-front", "bench", "bench-beech", "bench", 0.42, rect(0.615, 0.635, 0.450, 0.520), -0.370, 0.310),
            part("bench-apron-side", "bench", "bench-beech", "bench", 0.42, rect(0.385, 0.595, 0.450, 0.520), -0.410, -0.390, rep=(2, (0.0, 0.0, 0.740))),
            part("bench-seat", "bench", "bench-veneer", "bench", 0.55, rect(0.330, 0.650, 0.520, 0.545), -0.430, 0.370),
            part("bench-cushion", "bench", "bench-vinyl", "bench", 0.22, [(0.340, 0.545), (0.640, 0.545), (0.640, 0.565), (0.630, 0.575), (0.350, 0.575), (0.340, 0.565)], -0.420, 0.360),
        ]
    elif scale == 100:
        out += [
            part("bench-legs-back", "bench", "bench-beech", "bench", 0.4, rect(0.345, 0.385, 0.0, 0.450), zr[0], zr[1], rep=(2, (0.0, 0.0, 0.720))),
            part("bench-legs-front", "bench", "bench-beech", "bench", 0.4, rect(0.595, 0.635, 0.0, 0.450), zr[0], zr[1], rep=(2, (0.0, 0.0, 0.720))),
            part("bench-frame", "bench", "bench-veneer", "bench", 0.5, rect(0.330, 0.650, 0.450, 0.545), -0.430, 0.370),
            part("bench-cushion", "bench", "bench-vinyl", "bench", 0.22, rect(0.340, 0.640, 0.545, 0.575), -0.420, 0.360),
        ]
    else:
        out += [
            part("bench-legs", "bench", "bench-beech", "bench", 0.4, rect(0.345, 0.385, 0.0, 0.450), zr[0], zr[1], rep=(2, (0.0, 0.0, 0.720))),
            part("bench-legs-far", "bench", "bench-beech", "bench", 0.4, rect(0.595, 0.635, 0.0, 0.450), zr[0], zr[1], rep=(2, (0.0, 0.0, 0.720))),
            part("bench-top", "bench", "bench-vinyl", "bench", 0.4, rect(0.330, 0.650, 0.450, 0.575), -0.430, 0.370),
        ]
    return out


def pedal_parts(scale):
    """🦶 The 13-note pedalboard (C to C), drawn in its own frame as if unfolded: u forward from the hinge, v up."""
    out = []
    pitch = 0.0575
    if scale == 50:
        out += [
            part("pedal-base", "pedalboard", "pedal-beech", "pedals", 0.55, rect(0.0, 0.25, 0.0, 0.012), -0.420, 0.080, frame="local"),
            part("pedal-rail", "pedalboard", "pedal-beech", "pedals", 0.5, rect(0.0, 0.06, 0.012, 0.085), -0.420, 0.080, frame="local"),
            part("pedal-cheek", "pedalboard", "pedal-beech", "pedals", 0.5, rect(0.06, 0.25, 0.012, 0.035), -0.420, -0.400, frame="local", rep=(2, (0.0, 0.0, 0.480))),
            part("pedal-felt", "pedalboard", "pedal-felt", "pedals", 0.3, rect(0.06, 0.245, 0.012, 0.020), -0.400, 0.060, frame="local"),
            part("pedal-naturals", "pedalboard", "pedal-natural", "pedals", 0.85, rect(0.06, 0.245, 0.020, 0.045), -0.394, -0.3485, frame="local", rep=(8, (0.0, 0.0, pitch))),
        ]
        for i, b in enumerate([0, 1, 3, 4, 5]):
            zc = -0.400 + (b + 1) * pitch
            out.append(part(f"pedal-sharp-riser-{i}", "pedalboard", "pedal-felt", "pedals", 0.2, rect(0.06, 0.15, 0.020, 0.045), r6(zc - 0.006), r6(zc + 0.006), frame="local", views=()))
            out.append(part(f"pedal-sharp-{i}", "pedalboard", "pedal-sharp", "pedals", 0.12, rect(0.06, 0.15, 0.045, 0.072), r6(zc - 0.015), r6(zc + 0.015), frame="local"))
    elif scale == 100:
        out += [
            part("pedal-rail", "pedalboard", "pedal-beech", "pedals", 0.5, rect(0.0, 0.06, 0.0, 0.085), -0.420, 0.080, frame="local"),
            part("pedal-deck", "pedalboard", "pedal-natural", "pedals", 0.8, rect(0.06, 0.25, 0.0, 0.045), -0.420, 0.080, frame="local"),
        ]
        for i, b in enumerate([0, 1, 3, 4, 5]):
            zc = -0.400 + (b + 1) * pitch
            out.append(part(f"pedal-sharp-{i}", "pedalboard", "pedal-sharp", "pedals", 0.12, rect(0.06, 0.15, 0.045, 0.072), r6(zc - 0.015), r6(zc + 0.015), frame="local"))
    else:
        out += [
            part("pedal-rail", "pedalboard", "pedal-beech", "pedals", 0.5, rect(0.0, 0.06, 0.0, 0.085), -0.420, 0.080, frame="local"),
            part("pedal-deck", "pedalboard", "pedal-natural", "pedals", 0.8, rect(0.06, 0.25, 0.0, 0.045), -0.420, 0.080, frame="local"),
        ]
    return out


def swell_parts(scale):
    """🎚️ The expression (swell) pedal: base on the cabinet, shoe on its pivot bone."""
    shoe_v = 0.022 if scale < 200 else 0.030
    out = [part("swell-base", "cabinet", "pedal-beech", "pedals", 0.35, ol(rect(-0.034, 0.210, 0.0, 0.030)), 0.150, 0.255)]
    if scale < 200:
        out.append(part("swell-bracket", "cabinet", "castor-steel", "pedals", 0.45, ol(rect(0.070, 0.110, 0.030, 0.050)), 0.180, 0.225))
    out.append(part("swell-shoe", "swell-pedal", "pedal-sharp", "pedals", 0.2, rect(-0.11, 0.16, 0.0, shoe_v), 0.155, 0.250, frame="local"))
    return out


def cover_part(scale):
    """🪵 The hinged key cover: closed it is the hood over both manuals, open it stands as the music rest."""
    if scale == 200:
        return part("key-cover", "key-cover", "veneer-aged", "cabinet-veneer", 0.42, rect(0.0, COVER_LEN, 0.0, 0.030), -0.550, 0.550, frame="local")
    return part("key-cover", "key-cover", "veneer-aged", "cabinet-veneer", 0.42, rect(0.0, COVER_LEN, 0.0, 0.012), -0.550, 0.550, frame="local", sheet=True, hatch={"elevation": "veneer"} if scale == 50 else {})


def cabinet_parts(scale):
    """🗄️ The veneered cabinet with both manuals, panels and the wear of thirty years."""
    out = []
    zi = 0.532
    if scale in (50, 100):
        cheek = CHEEK_50 if scale == 50 else CHEEK_100
        hatch = {"elevation": "veneer"} if scale == 50 else {}
        out += [
            part("cheek-right", "cabinet", "veneer-aged", "cabinet-veneer", 0.45, ol(cheek), -0.550, -zi, hatch=hatch),
            part("cheek-left", "cabinet", "veneer-aged", "cabinet-veneer", 0.45, ol(cheek), zi, 0.550),
            part("back-panel", "cabinet", "hardboard-aged", "back-panel", 0.3, ol(rect(-0.2782, -0.275, 0.0, 0.983)), -0.550, 0.550, sheet=True),
            part("lid", "cabinet", "veneer-aged", "cabinet-veneer", 0.42, ol(rect(-0.275, -0.030, 0.965, 0.983)), -zi, zi),
            part("tab-panel", "cabinet", "veneer-aged", "cabinet-veneer", 0.38, ol(TAB_PANEL), -zi, zi),
            part("bottom-board", "cabinet", "veneer-aged", "cabinet-veneer", 0.4, ol(rect(-0.275, -0.050, 0.0, 0.018)), -zi, zi),
            part("key-bed", "cabinet", "veneer-aged", "cabinet-veneer", 0.4, ol(rect(-0.050, 0.275, 0.600, 0.620)), -zi, zi),
            part("name-board", "cabinet", "veneer-aged", "cabinet-veneer", 0.35, ol(rect(0.100, 0.115, 0.620, 0.765)), -zi, zi),
            part("upper-key-bed", "cabinet", "veneer-aged", "cabinet-veneer", 0.4, ol(rect(-0.005, 0.172 if scale == 50 else 0.035, 0.765, 0.783)), -zi, zi),
            part("lower-panel-right", "cabinet", "panel-faded", "control-panels", 0.62, ol(rect(0.115, 0.265 if scale == 50 else 0.275, 0.620, 0.690)), -zi, Z0_LOWER - 0.001),
            part("lower-panel-left", "cabinet", "panel-faded", "control-panels", 0.62, ol(rect(0.115, 0.265 if scale == 50 else 0.275, 0.620, 0.690)), -Z0_UPPER + 0.001, zi),
            part("upper-panel-rhythm", "cabinet", "panel-faded", "control-panels", 0.66, ol(rect(0.035, 0.170, 0.783, 0.800) if scale == 50 else rect(0.035, 0.172, 0.765, 0.800)), -zi, Z0_UPPER - 0.001),
            part("upper-panel-master", "cabinet", "panel-faded", "control-panels", 0.62, ol(rect(0.035, 0.170, 0.783, 0.800) if scale == 50 else rect(0.035, 0.172, 0.765, 0.800)), -Z0_LOWER + 0.001, zi),
        ]
        if scale == 50:
            out += [
                part("lower-action", "cabinet", "key-frame-steel", "keys", 0.12, ol(rect(0.115, 0.265, 0.620, 0.683)), Z0_LOWER, -Z0_UPPER),
                part("knee-panel", "cabinet", "veneer-aged", "grille-panel", 0.35, ol(rect(-0.050, -0.034, 0.018, 0.600)), -zi, zi),
                part("grille-cloth", "cabinet", "grille-cloth", "grille-panel", 0.25, ol(rect(-0.034, -0.031, 0.060, 0.560)), -0.500, 0.500, sheet=True, hatch={"front": "cloth"}),
                part("key-slip", "cabinet", "veneer-aged", "cabinet-veneer", 0.4, ol(rect(0.265, 0.275, 0.620, 0.695)), -zi, zi),
                part("tab-strip", "cabinet", "panel-faded", "control-panels", 0.7, ol(tab_strip(0.30, 0.62, 0.008)), -0.400, -0.352, rep=(16, (0.0, 0.0, 0.053))),
                part("rhythm-buttons", "cabinet", "key-black-aged", "control-panels", 0.25, ol(rect(0.075, 0.100, 0.800, 0.808)), -0.505, -0.485, rep=(6, (0.0, 0.0, 0.045))),
            ]
            lw = [i for i in range(26)]
            out += [
                white_keys("lower-whites-bass", "cabinet", *[q for q in (0.115, 0.265)], *(H_D + 0.683, H_D + 0.705), Z0_LOWER, lw[:KEY_C_INDEX], 0.9),
                white_keys("lower-whites-treble", "cabinet", 0.115, 0.265, H_D + 0.683, H_D + 0.705, Z0_LOWER, lw[KEY_C_INDEX + 1:], 0.9),
                white_keys("upper-whites", "cabinet", 0.035, 0.170, H_D + 0.783, H_D + 0.805, Z0_UPPER, lw, 0.9),
            ]
            out += black_keys("lower-black", "cabinet", 0.115, 0.205, H_D + 0.705, H_D + 0.717, Z0_LOWER, 26, 0.08)
            out += black_keys("upper-black", "cabinet", 0.035, 0.115, H_D + 0.805, H_D + 0.817, Z0_UPPER, 26, 0.08)
            zs = Z0_LOWER + KEY_C_INDEX * P_KEY + 0.0005
            out.append(part("key-c", "lower-key", "key-acrylic-yellowed", "keys", 0.88, rect(0.0, 0.15, 0.0, 0.022), r6(zs), r6(zs + W_KEY), frame="local", role="detail"))
            out += [
                part("wear-cheek-right", "cabinet", "veneer-worn", "cabinet-veneer", 0.62, ol([(0.275, 0.640), (0.275, 0.735), (0.215, 0.784), (0.235, 0.690)]), -0.5505, -0.550, texture=True, thermal=False, views=("elevation",)),
                part("sticker", "cabinet", "paper-sticker", "cabinet-veneer", 0.85, ol(rect(-0.200, -0.120, 0.300, 0.350)), -0.5505, -0.550, texture=True, thermal=False, views=("elevation",)),
                part("wear-lid", "key-cover", "veneer-worn", "cabinet-veneer", 0.6, rect(0.30, COVER_LEN, 0.012, 0.0125), -0.300, 0.250, frame="local", texture=True, thermal=False, views=("plan",)),
            ]
        else:
            out += [
                part("knee-panel", "cabinet", "grille-cloth", "grille-panel", 0.3, ol(rect(-0.050, -0.031, 0.018, 0.600)), -zi, zi, hatch={"front": "cloth"}),
                part("lower-manual-bass", "cabinet", "key-acrylic-yellowed", "keys", 0.88, ol(rect(0.115, 0.275, 0.620, 0.705)), Z0_LOWER, r6(Z0_LOWER + KEY_C_INDEX * P_KEY), role="detail", draw={"views": ("elevation", "front", "plan"), "span": (Z0_LOWER, -Z0_UPPER)}),
                part("lower-manual-treble", "cabinet", "key-acrylic-yellowed", "keys", 0.88, ol(rect(0.115, 0.275, 0.620, 0.705)), r6(Z0_LOWER + (KEY_C_INDEX + 1) * P_KEY), -Z0_UPPER, role="detail", draw={"views": ()}),
                part("upper-whites", "cabinet", "key-acrylic-yellowed", "keys", 0.88, ol(rect(0.035, 0.172, 0.765, 0.805)), Z0_UPPER, -Z0_LOWER, role="detail"),
                part("key-c", "lower-key", "key-acrylic-yellowed", "keys", 0.88, rect(0.0, 0.16, -0.063, 0.022), r6(Z0_LOWER + KEY_C_INDEX * P_KEY), r6(Z0_LOWER + (KEY_C_INDEX + 1) * P_KEY), frame="local", role="detail", draw={"views": ()}),
                part("sticker", "cabinet", "paper-sticker", "cabinet-veneer", 0.85, ol(rect(-0.200, -0.120, 0.300, 0.350)), -0.551, -0.550, texture=True, thermal=False, views=("elevation",)),
            ]
    else:
        console = [(-0.061, 0.600), (0.275, 0.600), (0.275, 0.620), (0.115, 0.620), (0.115, 0.765), (0.030, 0.765), (0.030, 0.783), (-0.030, 0.953), (-0.061, 0.953)]
        out += [
            part("cheek-right", "cabinet", "veneer-aged", "cabinet-veneer", 0.45, ol(CHEEK_200), -0.550, -0.520),
            part("cheek-left", "cabinet", "veneer-aged", "cabinet-veneer", 0.45, ol(CHEEK_200), 0.520, 0.550),
            part("back-panel", "cabinet", "hardboard-aged", "back-panel", 0.3, ol(rect(-0.2782, -0.275, 0.0, 0.983)), -0.550, 0.550, sheet=True),
            part("cabinet-bottom", "cabinet", "veneer-aged", "cabinet-veneer", 0.4, ol(rect(-0.275, -0.061, 0.0, 0.030)), -0.520, 0.520),
            part("cabinet-console", "cabinet", "veneer-aged", "cabinet-veneer", 0.4, ol(console), -0.520, 0.520),
            part("cabinet-lid", "cabinet", "veneer-aged", "cabinet-veneer", 0.42, ol(rect(-0.275, -0.030, 0.953, 0.983)), -0.520, 0.520),
            part("knee-panel", "cabinet", "grille-cloth", "grille-panel", 0.3, ol(rect(-0.061, -0.031, 0.018, 0.600)), -0.520, 0.520),
            part("lower-manual", "cabinet", "key-acrylic-yellowed", "keys", 0.85, ol(rect(0.115, 0.275, 0.620, 0.705)), Z0_LOWER, -Z0_UPPER, role="detail"),
            part("lower-panel-right", "cabinet", "panel-faded", "control-panels", 0.62, ol(rect(0.115, 0.275, 0.620, 0.690)), -0.520, Z0_LOWER),
            part("lower-panel-left", "cabinet", "panel-faded", "control-panels", 0.62, ol(rect(0.115, 0.275, 0.620, 0.690)), -Z0_UPPER, 0.520),
            part("upper-manual", "cabinet", "key-acrylic-yellowed", "keys", 0.85, ol(rect(0.030, 0.172, 0.765, 0.805)), Z0_UPPER, -Z0_LOWER, role="detail"),
            part("upper-panels", "cabinet", "panel-faded", "control-panels", 0.62, ol(rect(0.030, 0.172, 0.765, 0.800)), -0.520, Z0_UPPER, rep=None),
            part("upper-panel-master", "cabinet", "panel-faded", "control-panels", 0.62, ol(rect(0.030, 0.172, 0.765, 0.800)), -Z0_LOWER, 0.520),
        ]
    return out


def parts_of(scale):
    """📦 Every part of one authored scale."""
    return cabinet_parts(scale) + [cover_part(scale)] + swell_parts(scale) + pedal_parts(scale) + dolly_parts(scale) + bench_parts(scale)
