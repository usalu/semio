"""🏗️ The four individually authored scales of `used-water-play-table`: every part drawn in elevation, front and plan and modelled in the body, thermal and structure lists for the smallest feature of its scale (7.5, 15, 30, 75 mm).

@see ./author_geometry.py — the hand-chosen numbers
@see ./author_parts.py — shape and solid builders
"""
import math

from author_geometry import (
    APRON_X, APRON_Y, APRON_Z, AXIS_DEG, BEARER, BEARER_HALF_X, BEARER_Z, BONES, BS_CX, BS_FL, BS_FLOOR, BS_RIM, BS_T, BS_WATER, BS_X, BS_Y0, BS_Z,
    CAP, CHUTE_R, CHUTE_T, CLAMP, CRANK, DECK, DRAIN, END_FL_X, END_X, FOOT, FOOT_R, FOOT_Y0, GATE_H, GATE_HOLE, GATE_T, GATE_X, GATE_Z, GRIP, GRIP_UV,
    GUIDE, HELIX, MUD, P, PIN, PLATE, PLATE_Z, PLINTH, POOL_IDS, POOLS, POST_R, POST_X, POST_Y0, POST_Y1, POST_Z, POSTS, SAND_DRIFT, SHAFT, SPOUT,
    TR_FL, TR_FLOOR, TR_IN_X, TR_IN_Z, TR_RIM, TR_T, TR_TOP, TR_X, TR_Y0, TR_Z, TUBE, WATER_D, WEAR, arc, chute_line, chute_outline_world, foot_centre,
    local, quat_axis, quat_w, r5, screw_point,
)
from author_parts import Scale, annulus, box, ellipse, ext, extruded, path, poly, rect, rev, revolved, ring_path, shape

TIMBER_G, TIMBER_F, ROB, STEEL, PC, PE, CONC, WATER, SAND = "douglas-greyed", "douglas-fresh", "robinia-weathered", "stainless-scaled", "polycarbonate-clouded", "pe-grip", "concrete-weathered", "water", "sand-wet"


def posts_and_footings(k):
    """🪵 Four Robinia posts in their concrete point footings: footing below sand level, post from the footing head to the bearer."""
    s = k.s
    hatch = "concrete" if s <= 100 else None
    grain = "timber" if s <= 100 else None
    for bone in POSTS:
        zb = BONES[bone]["z"]
        x = BONES[bone]["origin"][0]
        near = 0.1 if zb < 0 else 0.0
        k.elev.append(shape("footing-" + bone, bone, "body", 1.0 + near, True, 0.62, rect(0, -FOOT_R, -FOOT_Y0, FOOT_R), "footing-concrete", CONC, hatch))
        k.front.append(shape("footing-" + bone, bone, "body", 1.0 + (0.1 if x > 0 else 0), True, 0.62, rect(zb - FOOT_R, FOOT_Y0, zb + FOOT_R, 0.0), "footing-concrete", CONC, hatch))
        prof = [(0, 0), (0, FOOT_R), (-FOOT_Y0, FOOT_R), (-FOOT_Y0, POST_R), (POST_Y0 - FOOT_Y0, POST_R), (POST_Y0 - FOOT_Y0, 0)]
        k.plan.append(shape("footing-" + bone, bone, "body", 0.5, True, 0.62, ellipse(x, zb, FOOT_R, FOOT_R), "footing-concrete", CONC, hatch))
        k.body.append(rev("footing-" + bone, bone, CONC, "body", 0.62, prof, [1, 0, 0], [0, 0, 0]))
        k.th("footing-" + bone, "footing-concrete")
        k.st("footing-" + bone, bone, "member")
        u0, u1 = -FOOT_Y0, POST_Y1 - FOOT_Y0
        k.elev.append(shape("post-" + bone, bone, "body", 1.5 + near, True, 0.46, rect(u0, -POST_R, u1, POST_R), "posts-robinia", ROB, grain))
        k.front.append(shape("post-" + bone, bone, "body", 1.5 + (0.1 if x > 0 else 0), True, 0.46, rect(zb - POST_R, 0.0, zb + POST_R, POST_Y1), "posts-robinia", ROB, grain))
        k.body.append(rev("post-" + bone, bone, ROB, "body", 0.46, [(POST_Y0 - FOOT_Y0, 0), (POST_Y0 - FOOT_Y0, POST_R), (u1, POST_R), (u1, 0)], [1, 0, 0], [0, 0, 0]))
        k.th("post-" + bone, "posts-robinia")
        k.st("post-" + bone, bone, "member")


def table_frame(k):
    """🪚 Bearers, deck, the two long aprons with their worn top edges, the greyed head board and the replaced fresh board at the outlet end."""
    s = k.s
    grain = "timber" if s <= 100 else None
    for sign in (1, -1):
        x0, x1 = sign * POST_X - BEARER_HALF_X, sign * POST_X + BEARER_HALF_X
        k.block("bearer-" + ("f" if sign > 0 else "r"), "frame", (x0, x1, BEARER[0], BEARER[1], -BEARER_Z, BEARER_Z), "understructure-timber", TIMBER_G, 0.4, views="ef", struct="member")
    if s < 500:
        k.block("deck", "frame", DECK, "understructure-timber", TIMBER_G, 0.42, views="", struct="member")
    ta = APRON_Z[1] - APRON_Z[0] if s < 500 else 0.075
    zin = APRON_Z[1] - ta
    for side, sg in (("r", -1), ("l", 1)):
        sid = "apron-" + side
        if s == 50:
            cy, cz = APRON_Y[1] - WEAR, APRON_Z[1] - WEAR
            worn = [(cy + WEAR * math.sin(math.radians(a)), cz + WEAR * math.cos(math.radians(a))) for a in range(90, -1, -15)]
            prof = [(APRON_Y[0], zin), (APRON_Y[1], zin)] + worn + [(APRON_Y[0], APRON_Z[1])]
            prof = [(y, sg * z) for y, z in prof]
            k.body.append(ext(sid, "frame", TIMBER_G, "body", 0.5, prof, "yz", -APRON_X, APRON_X))
            k.front.append(shape(sid, "frame", "body", 3.0, True, 0.5, poly([(z, y) for y, z in prof]), "timber-greyed", TIMBER_G, grain))
        else:
            z0, z1 = (zin, APRON_Z[1]) if sg > 0 else (-APRON_Z[1], -zin)
            k.body.append(box(sid, "frame", TIMBER_G, "body", 0.5, [2 * APRON_X, APRON_Y[1] - APRON_Y[0], ta], [0, sum(APRON_Y) / 2, sg * (zin + APRON_Z[1]) / 2]))
            k.front.append(shape(sid, "frame", "body", 3.0, True, 0.5, rect(z0, APRON_Y[0], z1, APRON_Y[1]), "timber-greyed", TIMBER_G, grain))
        k.th(sid, "timber-greyed")
        k.st(sid, "frame", "member")
        zz = (zin, APRON_Z[1]) if sg > 0 else (-APRON_Z[1], -zin)
        k.plan.append(shape(sid, "frame", "body", 2.6, True, 0.5, rect(-APRON_X, zz[0], APRON_X, zz[1]), "timber-greyed", TIMBER_G, grain))
        if sg < 0:
            ap = shape(sid, "frame", "body", 3.0, True, 0.5, rect(-APRON_X, APRON_Y[0], APRON_X, APRON_Y[1]), "timber-greyed" if s < 500 else None, TIMBER_G, grain)
            if s == 500:
                ap["regions"] = ["timber-greyed", "trough-steel", "gates-steel"]
            k.elev.append(ap)
    te = END_X[1] - END_X[0] if s < 500 else 0.075
    for sg, sid, region, mat, tone in ((1, "head-board", "timber-greyed", TIMBER_G, 0.5), (-1, "replaced-board", "timber-fresh", TIMBER_F, 0.78)):
        xa, xb = sorted((sg * APRON_X, sg * (APRON_X - te)))
        k.block(sid, "frame", (xa, xb, APRON_Y[0], APRON_Y[1], -zin, zin), region, mat, tone, views="fp" if sg > 0 else "ep", struct="member", order=(1.2, 3.1, 2.6))


def trough(k):
    """🛁 The stainless channel: a 1.5 mm U with folded flanges over the aprons, end plates and end flanges, the sluice guides, the overflow spout and the silted wet sand at the outlet end."""
    s = k.s
    t = TR_T
    zo = TR_Z if s < 500 else APRON_Z[1] - 0.075
    zi = zo - t
    xo = TR_X if s < 500 else APRON_X - 0.075
    xi = xo - t
    fl = TR_FL if s < 500 else APRON_Z[1] - 0.035
    prof = [(TR_Y0, -zo), (TR_Y0, zo), (TR_RIM, zo), (TR_RIM, fl), (TR_TOP, fl), (TR_TOP, zi), (TR_FLOOR, zi), (TR_FLOOR, -zi), (TR_TOP, -zi), (TR_TOP, -fl), (TR_RIM, -fl), (TR_RIM, -zo)]
    k.body.append(ext("trough", "frame", STEEL, "body", 0.7, prof, "yz", -xo, xo))
    k.th("trough", "trough-steel")
    k.st("trough", "frame", "member")
    for sg in (1, -1):
        e = "f" if sg > 0 else "r"
        xa, xb = sorted((sg * xi, sg * xo))
        k.block("end-plate-" + e, "frame", (xa, xb, TR_FLOOR, TR_TOP, -zi, zi), "trough-steel", STEEL, 0.7, views="", sheet=True)
        if s < 500:
            xa, xb = sorted((sg * xo, sg * END_FL_X))
            k.block("end-flange-" + e, "frame", (xa, xb, TR_RIM, TR_TOP, -fl, fl), "trough-steel", STEEL, 0.7, views="", sheet=True)
    xf = END_FL_X if s < 500 else xo
    k.plan.append(shape("trough-flange", "frame", "body", 2.8, True, 0.72, ring_path((-xf, -fl, xf, fl), (-xi, -zi, xi, zi)), "trough-steel", STEEL))
    k.plan.append(shape("trough-floor", "frame", "body", 2.7, True, 0.8, rect(-xi, -zi, xi, zi), "trough-steel", STEEL))
    if s <= 100:
        k.elev.append(shape("trough-rim", "frame", "detail", 3.2, False, 0.72, poly([(-xf, TR_RIM + t / 2), (xf, TR_RIM + t / 2)], False), "trough-steel", STEEL))
    if s == 50:
        for gid, xg in GATE_X.items():
            for sg in (1, -1):
                gz = (GUIDE[1], TR_IN_Z) if sg > 0 else (-TR_IN_Z, -GUIDE[1])
                k.block(f"guide-{gid}-{'l' if sg > 0 else 'r'}", "frame", (xg - GUIDE[0], xg + GUIDE[0], TR_FLOOR, TR_TOP, gz[0], gz[1]), "trough-steel", STEEL, 0.66, views="p", order=(2.0, 2.0, 3.4))
    if s < 500:
        x0, x1, yc, ro, ri = SPOUT
        if s == 50:
            k.body.append(rev("spout", "frame", STEEL, "body", 0.66, annulus(x0, x1, ri, ro), [1, 0, 0], [0, yc, 0]))
        else:
            k.body.append(rev("spout", "frame", STEEL, "body", 0.66, [(x0, 0), (x0, ro), (x1, ro), (x1, 0)], [1, 0, 0], [0, yc, 0]))
        k.th("spout", "trough-steel")
        k.elev.append(shape("spout", "frame", "body", 3.3, True, 0.66, rect(x0, yc - ro, -APRON_X, yc + ro), "trough-steel", STEEL))
        k.plan.append(shape("spout", "frame", "body", 3.3, True, 0.66, rect(x0, -ro, -APRON_X, ro), "trough-steel", STEEL))
    if s <= 100:
        x0, x1, h = SAND_DRIFT
        k.body.append(ext("sand-drift", "frame", SAND, "body", 0.3, [(x0, TR_FLOOR), (x1, TR_FLOOR), (x0, TR_FLOOR + h)], "xy", -TR_IN_Z, TR_IN_Z))
        k.th("sand-drift", "wet-sand")
        k.plan.append(shape("sand-drift", "frame", "body", 2.75, True, 0.35, rect(x0, -TR_IN_Z, x1, TR_IN_Z), "wet-sand", SAND))


def gates(k):
    """🚪 The two sluice gates: 3 mm stainless plates with a hand hole, sliding up in their guides."""
    s = k.s
    for gid, xg in GATE_X.items():
        h0, h1, hw = GATE_HOLE
        outer = [(-GATE_Z, 0), (GATE_Z, 0), (GATE_Z, GATE_H), (-GATE_Z, GATE_H)]
        hole = [(-hw, h0), (hw, h0), (hw, h1), (-hw, h1)] if s <= 100 else None
        if s < 500:
            k.body.append(ext("gate-" + gid, gid, STEEL, "body", 0.6, outer, "zx", -GATE_T / 2, GATE_T / 2, holes=[hole] if hole else None, sheet=True))
            k.th("gate-" + gid, "gates-steel")
            pen = {50: 0.25, 100: 0.18, 200: 0.13}[s]
            k.elev.append(shape("gate-" + gid, gid, "detail", 3.6, False, 0.6, poly([(0, 0), (GATE_H, 0)], False), "gates-steel", STEEL, pen=pen))
            d = f"M {r5(-GATE_Z)} {r5(TR_FLOOR)} L {r5(GATE_Z)} {r5(TR_FLOOR)} L {r5(GATE_Z)} {r5(TR_FLOOR + GATE_H)} L {r5(-GATE_Z)} {r5(TR_FLOOR + GATE_H)} Z"
            if hole:
                d += f" M {r5(-hw)} {r5(TR_FLOOR + h0)} L {r5(hw)} {r5(TR_FLOOR + h0)} L {r5(hw)} {r5(TR_FLOOR + h1)} L {r5(-hw)} {r5(TR_FLOOR + h1)} Z"
            k.front.append(shape("gate-" + gid, gid, "body", 2.0 + xg, True, 0.6, path(d), "gates-steel", STEEL))
            k.plan.append(shape("gate-" + gid, gid, "detail", 3.6, False, 0.6, poly([(xg, -GATE_Z), (xg, GATE_Z)], False), "gates-steel", STEEL, pen=pen))
        else:
            k.thermal.append(ext("th-gate-" + gid, gid, STEEL, "body", 0.6, outer, "zx", -GATE_T / 2, GATE_T / 2, sheet=True, region="gates-steel"))


def water(k):
    """🌊 Standing water: three pools between the sluices and the lower basin; drained, each slab sinks into the timber deck or the concrete plinth beneath it."""
    s = k.s
    if s < 500:
        for pid in POOL_IDS:
            x0, x1 = POOLS[pid]
            cx = (x0 + x1) / 2
            k.body.append(box(pid, pid, WATER, "body", 0.86, [x1 - x0, WATER_D, 2 * TR_IN_Z], [0, WATER_D / 2, 0]))
            k.th(pid, "water-pools")
            k.plan.append(shape(pid, pid, "body", 2.9, True, 0.86, rect(x0, -TR_IN_Z, x1, TR_IN_Z), "water-pools", WATER))
            k.elev.append(shape(pid, pid, "body", 1.2, True, 0.86, rect(x0 - cx, 0, x1 - cx, WATER_D), "water-pools", WATER))
    else:
        zi = APRON_Z[1] - 0.075 - TR_T
        x0, x1 = -(APRON_X - 0.075 - TR_T), APRON_X - 0.075 - TR_T
        cx = BONES["pool-2"]["origin"][0]
        k.body.append(box("pools", "pool-2", WATER, "body", 0.86, [x1 - x0, 0.075, 2 * zi], [(x0 + x1) / 2 - cx, 0.0375, 0]))
        k.th("pools", "water-pools")
        k.plan.append(shape("pools", "pool-2", "body", 2.9, True, 0.86, rect(x0, -zi, x1, zi), "water-pools", WATER))
        k.elev.append(shape("pools", "pool-2", "body", 1.2, True, 0.86, rect(x0 - cx, 0, x1 - cx, 0.075), "water-pools", WATER))
    hi = BS_X[1] - BS_X[0] - 2 * BS_T
    k.body.append(box("basin-water", "basin-water", WATER, "body", 0.86, [hi, BS_WATER, 2 * (BS_Z - BS_T)], [0, BS_WATER / 2, 0]))
    k.th("basin-water", "water-basin")
    k.plan.append(shape("basin-water", "basin-water", "body", 3.0, True, 0.86, rect(BS_X[0] + BS_T, -(BS_Z - BS_T), BS_X[1] - BS_T, BS_Z - BS_T), "water-basin", WATER))
    k.elev.append(shape("basin-water", "basin-water", "body", 1.2, True, 0.86, rect(-hi / 2, 0, hi / 2, BS_WATER), "water-basin", WATER))


def basin(k):
    """🪣 The lower stainless basin on its precast plinth, bedded in the sand: a U pressed along z, two side plates and the side flanges."""
    s = k.s
    hatch = "concrete" if s <= 100 else None
    x0, x1, y0, y1, zp = PLINTH
    k.block("plinth", "basin", (x0, x1, y0, y1, -zp, zp), "footing-concrete", CONC, 0.62, struct="member", hatch=hatch, order=(1.0, 1.0, 1.0))
    a, b = BS_X
    t = BS_T
    fl = BS_FL if s < 500 else 0.0
    top = BS_RIM + t
    prof = [(a, BS_Y0), (b, BS_Y0), (b, BS_RIM), (b + fl, BS_RIM), (b + fl, top), (b - t, top), (b - t, BS_FLOOR), (a + t, BS_FLOOR), (a + t, top), (a - fl, top), (a - fl, BS_RIM), (a, BS_RIM)]
    if s == 500:
        prof = [(a, BS_Y0), (b, BS_Y0), (b, top), (b - t, top), (b - t, BS_FLOOR), (a + t, BS_FLOOR), (a + t, top), (a, top)]
    k.body.append(ext("basin", "basin", STEEL, "body", 0.68, [(x - BS_CX, y) for x, y in prof], "xy", -BS_Z, BS_Z))
    k.th("basin", "basin-steel")
    k.st("basin", "basin", "member")
    for sg in (1, -1):
        e = "l" if sg > 0 else "r"
        za, zb = sorted((sg * (BS_Z - t), sg * BS_Z))
        k.block("basin-side-" + e, "basin", (a + t, b - t, BS_FLOOR, top, za, zb), "basin-steel", STEEL, 0.68, views="", sheet=True)
        if s < 500:
            za, zb = sorted((sg * BS_Z, sg * (BS_Z + fl)))
            k.block("basin-flange-" + e, "basin", (a - fl, b + fl, BS_RIM, top, za, zb), "basin-steel", STEEL, 0.68, views="", sheet=True)
    outline = [(a, BS_Y0), (b, BS_Y0), (b, BS_RIM), (b + fl, BS_RIM), (b + fl, top), (a - fl, top), (a - fl, BS_RIM), (a, BS_RIM)] if fl else [(a, BS_Y0), (b, BS_Y0), (b, top), (a, top)]
    k.elev.append(shape("basin", "basin", "body", 2.5, True, 0.68, poly([(x - BS_CX, y) for x, y in outline]), "basin-steel", STEEL))
    zf = BS_Z + fl
    k.front.append(shape("basin", "basin", "body", 4.0, True, 0.68, poly([(z, y) for y, z in ((BS_Y0, -BS_Z), (BS_Y0, BS_Z), (BS_RIM, BS_Z), (BS_RIM, zf), (top, zf), (top, -zf), (BS_RIM, -zf), (BS_RIM, -BS_Z))]) if fl else rect(-BS_Z, BS_Y0, BS_Z, top), "basin-steel", STEEL))
    k.plan.append(shape("basin-rim", "basin", "body", 2.9, True, 0.68, ring_path((a - fl, -zf, b + fl, zf), (a + t, -(BS_Z - t), b - t, BS_Z - t)), "basin-steel", STEEL))
    k.plan.append(shape("basin-floor", "basin", "body", 2.85, True, 0.78, rect(a + t, -(BS_Z - t), b - t, BS_Z - t), "basin-steel", STEEL))


def mud(k):
    """🟤 The splash hollow under the spout: a rim of wet sand kicked up around a trodden hollow, a flat wet patch at the coarse scales."""
    s = k.s
    cx, ri, ro, h = MUD
    if s <= 100:
        prof = [(0, ri), (0.012, ri + 0.02), (h, ri + 0.06), (0.02, ri + 0.10), (0.008, ro - 0.01), (0, ro), (0, ri)]
        k.body.append(rev("mud-rim", "frame", SAND, "body", 0.32, prof, [0, 1, 0], [cx, 0, 0]))
        side = [(cx - ro, 0), (cx - ro + 0.01, 0.008), (cx - ri - 0.10, 0.02), (cx - ri - 0.06, h), (cx + ri + 0.06, h), (cx + ri + 0.10, 0.02), (cx + ro - 0.01, 0.008), (cx + ro, 0)]
        k.elev.append(shape("mud-rim", "frame", "body", 0.9, True, 0.32, poly(side), "wet-sand", SAND))
        k.front.append(shape("mud-rim", "frame", "body", 0.5, True, 0.32, poly([(x - cx, y) for x, y in side]), "wet-sand", SAND))
        n = 32
        outer = " ".join(("M" if i == 0 else "L") + f" {r5(cx + ro * math.cos(2 * math.pi * i / n))} {r5(ro * math.sin(2 * math.pi * i / n))}" for i in range(n)) + " Z"
        inner = " ".join(("M" if i == 0 else "L") + f" {r5(cx + ri * math.cos(2 * math.pi * i / n))} {r5(ri * math.sin(2 * math.pi * i / n))}" for i in range(n)) + " Z"
        k.plan.append(shape("mud-rim", "frame", "body", 0.9, True, 0.32, path(outer + " " + inner), "wet-sand", SAND))
    else:
        th = 0.01
        k.body.append(rev("mud-patch", "frame", SAND, "body", 0.32, [(0, 0), (0, ro), (th, ro), (th, 0)], [0, 1, 0], [cx, 0, 0], sheet=True))
        k.th("mud-patch", "wet-sand")
        k.elev.append(shape("mud-patch", "frame", "body", 0.9, True, 0.32, rect(cx - ro, 0, cx + ro, th), "wet-sand", SAND))
        k.front.append(shape("mud-patch", "frame", "body", 0.5, True, 0.32, rect(-ro, 0, ro, th), "wet-sand", SAND))
        k.plan.append(shape("mud-patch", "frame", "body", 0.9, True, 0.32, ellipse(cx, 0, ro, ro), "wet-sand", SAND))
        return
    k.th("mud-rim", "wet-sand")


def bracket_and_chute(k):
    """🔩 The two bracket plates bolted to the head board, the pivot pins, the cross strap and the outlet chute that leads the lifted water over the head board into pool 1."""
    s = k.s
    (xa, ya), (xb, yb) = chute_line()
    if s == 500:
        x_ts = P[0] - TUBE[3] * math.sqrt(0.5)
        y_ts = P[1] - TUBE[3] * math.sqrt(0.5)
        blk = [(APRON_X - 0.12, TR_RIM), (0.95, TR_RIM), (0.95, y_ts - (0.95 - x_ts)), (x_ts, y_ts), (APRON_X - 0.12, 0.75)]
        k.body.append(ext("outlet-block", "frame", STEEL, "body", 0.55, blk, "xy", -0.0375, 0.0375))
        k.th("outlet-block", "screw-steel")
        k.st("outlet-block", "frame", "joint")
        k.elev.append(shape("outlet-block", "frame", "body", 3.4, True, 0.55, poly(blk), "screw-steel", STEEL))
        ys = [y for _, y in blk]
        xs = [x for x, _ in blk]
        k.front.append(shape("outlet-block", "frame", "body", 3.4, True, 0.55, rect(-0.0375, min(ys), 0.0375, max(ys)), "screw-steel", STEEL))
        k.plan.append(shape("outlet-block", "frame", "body", 3.4, True, 0.55, rect(min(xs), -0.0375, max(xs), 0.0375), "screw-steel", STEEL))
        return
    pt = {50: PLATE_Z, 100: (0.084, 0.099), 200: (0.084, 0.114)}[s]
    for sg in (1, -1):
        e = "l" if sg > 0 else "r"
        z0, z1 = sorted((sg * pt[0], sg * pt[1]))
        k.body.append(ext("plate-" + e, "frame", STEEL, "body", 0.5, PLATE, "xy", z0, z1))
        k.th("plate-" + e, "screw-steel")
        k.st("plate-" + e, "frame", "joint")
        if sg < 0:
            k.elev.append(shape("plate-" + e, "frame", "body", 4.6, True, 0.5, poly(PLATE), "screw-steel", STEEL))
        k.front.append(shape("plate-" + e, "frame", "body", 4.6, True, 0.5, rect(z0, min(y for _, y in PLATE), z1, max(y for _, y in PLATE)), "screw-steel", STEEL))
        k.plan.append(shape("plate-" + e, "frame", "body", 4.6, True, 0.5, rect(APRON_X, z0, max(x for x, _ in PLATE), z1), "screw-steel", STEEL))
        if s == 50:
            r, za, zb = PIN
            pz = sorted((sg * za, sg * zb))
            k.body.append(rev("pin-" + e, "frame", STEEL, "detail", 0.3, [(pz[0], 0), (pz[0], r), (pz[1], r), (pz[1], 0)], [0, 0, 1], [P[0], P[1], 0]))
            k.th("pin-" + e, "screw-steel")
            k.st("pin-" + e, "screw-mount", "joint")
            if sg < 0:
                k.elev.append(shape("pin-" + e, "frame", "detail", 4.7, True, 0.3, ellipse(P[0], P[1], r, r), "screw-steel", STEEL))
            k.front.append(shape("pin-" + e, "frame", "detail", 4.7, True, 0.3, rect(pz[0], P[1] - r, pz[1], P[1] + r), "screw-steel", STEEL))
            k.plan.append(shape("pin-" + e, "frame", "detail", 4.7, True, 0.3, rect(P[0] - r, pz[0], P[0] + r, pz[1]), "screw-steel", STEEL))
    c = 1 / math.hypot(1, 0.2)
    xs = 0.85
    ybot = ya - 0.2 * (xa - (xs - CHUTE_R * 0.2 * c)) - CHUTE_R * c
    hs = {50: 0.008, 100: 0.015, 200: 0.03}[s]
    ws = {50: 0.03, 100: 0.03, 200: 0.04}[s]
    zs = pt[1]
    k.block("chute-strap", "frame", (xs - ws / 2, xs + ws / 2, ybot - hs, ybot, -zs, zs), "screw-steel", STEEL, 0.5, views="ef", struct="joint", order=(4.5, 4.5, 4.5))
    t = CHUTE_T if s <= 100 else 0.0
    n = 12
    outer = [(CHUTE_R * math.sin(math.pi * i / n), CHUTE_R * math.cos(math.pi * i / n)) for i in range(n + 1)]
    prof = outer + ([((CHUTE_R - t) * math.sin(math.pi * i / n), (CHUTE_R - t) * math.cos(math.pi * i / n)) for i in range(n, -1, -1)] if t else [])
    theta = math.degrees(math.atan2(yb - ya, xb - xa))
    length = math.hypot(xb - xa, yb - ya)
    k.body.append(solid_chute(prof, length, theta, (xa, ya)))
    k.th("chute", "screw-steel")
    k.elev.append(shape("chute", "frame", "body", 3.9, True, 0.62, poly(chute_outline_world()), "screw-steel", STEEL))
    arc_pts = [(CHUTE_R * math.cos(math.pi + math.pi * i / n), yb - CHUTE_R * c * math.sin(math.pi * i / n)) for i in range(n + 1)]
    k.front.append(shape("chute", "frame", "body", 3.9, True, 0.62, poly([(-CHUTE_R, ya), (CHUTE_R, ya), (CHUTE_R, yb)] + list(reversed(arc_pts))[1:]), "screw-steel", STEEL))
    k.plan.append(shape("chute", "frame", "body", 3.9, True, 0.62, rect(xb, -CHUTE_R, xa + CHUTE_R * 0.2 * c, CHUTE_R), "screw-steel", STEEL))


def solid_chute(prof, length, theta, a):
    """🛝 The chute as a half-round gutter extruded along u of a turned frame and moved to its upper end A."""
    s = ext("chute", "frame", STEEL, "body", 0.62, prof, "yz", 0.0, length)
    s["transform"] = {"rotation": quat_w(theta), "translation": [r5(a[0]), r5(a[1]), 0.0]}
    return s


def screw_assembly(k):
    """🌀 The Archimedes screw: the clouded polycarbonate tube with its clamp ring, cap and foot bearing on the swinging mount, and the turning shaft, flight, crank and grip."""
    s = k.s
    t0, t1, ri, ro = TUBE
    if s < 500:
        k.body.append(rev("tube", "screw-mount", PC, "glazing", 0.9, annulus(t0, t1, ri, ro), [1, 0, 0], [0, 0, 0]))
    else:
        k.body.append(rev("tube", "screw-mount", PC, "glazing", 0.9, [(t0, 0), (t0, ro), (t1, ro), (t1, 0)], [1, 0, 0], [0, 0, 0]))
    k.th("tube", "tube-pc")
    k.st("tube", "screw-mount", "member")
    tube_shape = poly([(t0, 0), (t0, ro), (t1, ro), (t1, 0)])
    for lst, o in ((k.elev, 3.5), (k.front, 5.0), (k.plan, 5.0)):
        lst.append(shape("tube", "screw-mount", "glazing", o, True, 0.9, tube_shape, "tube-pc", PC, solid=revolved("screw-mount")))
    if s <= 200:
        a0, a1, rc = CLAMP
        c0 = CAP[0] if s == 50 else (-0.035 if s == 100 else -0.05)
        if s == 50:
            k.body.append(rev("clamp", "screw-mount", STEEL, "body", 0.45, annulus(a0, a1, ro, rc), [1, 0, 0], [0, 0, 0]))
            k.body.append(rev("cap", "screw-mount", STEEL, "body", 0.45, annulus(CAP[0], CAP[1], CAP[2], CAP[3]), [1, 0, 0], [0, 0, 0]))
            collar = [(CAP[0], 0), (CAP[0], CAP[3]), (a0, CAP[3]), (a0, rc), (a1, rc), (a1, 0)]
            k.th("clamp", "screw-steel")
            k.th("cap", "screw-steel")
            k.st("clamp", "screw-mount", "joint")
        else:
            k.body.append(rev("collar", "screw-mount", STEEL, "body", 0.45, [(c0, SHAFT[2] + 0.0005), (a1, SHAFT[2] + 0.0005), (a1, rc), (c0, rc), (c0, SHAFT[2] + 0.0005)], [1, 0, 0], [0, 0, 0]))
            collar = [(c0, 0), (c0, rc), (a1, rc), (a1, 0)]
            k.th("collar", "screw-steel")
            k.st("collar", "screw-mount", "joint")
        for lst, o in ((k.elev, 3.7), (k.front, 5.2), (k.plan, 5.2)):
            lst.append(shape("collar", "screw-mount", "body", o, True, 0.45, poly(collar), "screw-steel", STEEL, solid=revolved("screw-mount")))
    fx, fy0, fy1 = foot_centre()
    fw = FOOT[0] if s <= 200 else 0.075
    fu, fv = local("screw-mount", fx, (fy0 + fy1) / 2)
    k.body.append(box("foot", "screw-mount", STEEL, "body", 0.5, [fw, fy1 - fy0, fw], [fu, fv, 0], rotation=quat_w(-AXIS_DEG)))
    k.th("foot", "screw-steel")
    k.st("foot", "screw-mount", "joint")
    corners = [(fx - fw / 2, fy0), (fx + fw / 2, fy0), (fx + fw / 2, fy1), (fx - fw / 2, fy1)]
    k.elev.append(shape("foot", "screw-mount", "body", 2.4, True, 0.5, poly([local("screw-mount", x, y) for x, y in corners]), "screw-steel", STEEL))
    k.front.append(shape("foot", "screw-mount", "body", 3.9, True, 0.5, rect(-fw / 2, fy0, fw / 2, fy1), "screw-steel", STEEL))
    if s <= 100:
        lines = []
        for i in range(10 if s == 50 else 5):
            u = HELIX[0] + (0.04 if s == 50 else 0.08) + i * (HELIX[1] - HELIX[0] - 0.06) / (10 if s == 50 else 5)
            if u + 0.05 < t1:
                lines.append(f"M {r5(u)} {r5(-HELIX[3])} C {r5(u + 0.02)} {r5(-0.03)} {r5(u + 0.03)} {r5(0.03)} {r5(u + 0.05)} {r5(HELIX[3])}")
        k.elev.append(shape("flight-lines", "screw-mount", "detail", 3.6, False, 0.5, path(" ".join(lines)), "screw-steel", STEEL))
    u0, u1, rs = SHAFT
    if s < 500:
        k.body.append(rev("shaft", "screw", STEEL, "body", 0.55, [(u0, 0), (u0, rs), (u1, rs), (u1, 0)], [1, 0, 0], [0, 0, 0]))
        k.th("shaft", "screw-steel")
        k.st("shaft", "screw", "member")
        shaft_out = poly([(u0, 0), (u0, rs), (CAP[0] if s == 50 else -0.035, rs), (CAP[0] if s == 50 else -0.035, 0)])
        for lst, o in ((k.elev, 3.8), (k.front, 5.3), (k.plan, 5.3)):
            lst.append(shape("shaft", "screw", "body", o, True, 0.55, shaft_out, "screw-steel", STEEL, solid=revolved("screw")))
    else:
        k.thermal.append(rev("th-shaft", "screw", STEEL, "body", 0.55, [(u0, 0), (u0, rs), (u1, rs), (u1, 0)], [1, 0, 0], [0, 0, 0], slender=True, region="screw-steel"))
        k.structure.append(rev("st-shaft", "screw", STEEL, "body", 0.55, [(u0, 0), (u0, rs), (u1, rs), (u1, 0)], [1, 0, 0], [0, 0, 0], slender=True, member="screw", part="member"))
    if s <= 100:
        h0, h1, hr0, hr1, pitch = HELIX
        per = 6 if s == 50 else 4
        sweep = 2 * math.pi / per
        n = 6
        blade = [(hr1 * math.cos(sweep * i / n), hr1 * math.sin(sweep * i / n)) for i in range(n + 1)] + [(hr0 * math.cos(sweep * i / n), hr0 * math.sin(sweep * i / n)) for i in range(n, -1, -1)]
        mid = sweep / 2
        step = pitch / per
        mean_r = (hr0 + hr1) / 2
        tilt = math.degrees(math.atan2(step, mean_r * sweep))
        count = int((h1 - h0) / step)
        th = 0.002
        k.body.append({**ext("flight", "screw", STEEL, "body", 0.5, blade, "yz", -th / 2, th / 2, sheet=True), "transform": {"rotation": quat_axis((0, math.cos(mid), math.sin(mid)), tilt), "translation": [r5(h0 + step / 2), 0, 0]}, "repeat": {"count": count, "translation": [r5(step), 0, 0], "rotationAxis": [1, 0, 0], "rotationRad": r5(sweep), "origin": [0, 0, 0]}})
        k.th("flight", "screw-steel")
    c0, c1, v0, v1, cw = CRANK
    if s == 200:
        c0, c1, cw = -0.095, -0.065, 0.015
    if s < 500:
        k.body.append(box("crank", "screw", STEEL, "body", 0.4, [c1 - c0, v1 - v0, 2 * cw], [(c0 + c1) / 2, (v0 + v1) / 2, 0]))
        k.th("crank", "screw-steel")
        k.st("crank", "screw", "joint")
        for lst, o in ((k.elev, 4.0), (k.front, 5.5), (k.plan, 5.5)):
            lst.append(shape("crank", "screw", "body", o, True, 0.4, rect(c0, v0, c1, v1), "screw-steel", STEEL, solid=extruded(-cw, cw)))
        g0, g1, gr, g2, kr = GRIP
        prof = [(g0, 0), (g0, gr), (g1, gr), (g1, kr), (g2, kr), (g2, 0)] if s <= 100 else [(g0, 0), (g0, gr), (g2, gr), (g2, 0)]
        k.body.append(rev("grip", "grip", PE, "body", 0.2, prof, [1, 0, 0], [0, 0, 0]))
        k.th("grip", "grip-pe")
        k.st("grip", "grip", "member")
        for lst, o in ((k.elev, 4.1), (k.front, 5.6), (k.plan, 5.6)):
            lst.append(shape("grip", "grip", "body", o, True, 0.2, poly(prof), "grip-pe", PE, solid=revolved("grip")))
    else:
        kr = 0.0375
        stub = [(-0.12, 0), (-0.12, kr), (TUBE[0] + 0.005, kr), (TUBE[0] + 0.005, 0)]
        k.body.append(rev("crank-stub", "screw", STEEL, "body", 0.4, stub, [1, 0, 0], [0, 0, 0]))
        k.th("crank-stub", "screw-steel")
        k.st("crank-stub", "screw", "joint")
        k.body.append(box("crank", "screw", STEEL, "body", 0.4, [0.075, 0.21, 2 * kr], [-0.0825, 0.065, 0]))
        k.th("crank", "screw-steel")
        k.st("crank", "screw", "joint")
        gp = [(0, 0), (0, kr), (GRIP[3], kr), (GRIP[3], 0)]
        k.body.append(rev("grip", "grip", PE, "body", 0.2, gp, [1, 0, 0], [0, 0, 0]))
        k.th("grip", "grip-pe")
        k.st("grip", "grip", "member")
        for lst, o in ((k.elev, 4.0), (k.front, 5.5), (k.plan, 5.5)):
            lst.append(shape("crank-stub", "screw", "body", o, True, 0.4, poly(stub), "screw-steel", STEEL, solid=revolved("screw")))
            lst.append(shape("crank", "screw", "body", o + 0.05, True, 0.4, rect(-0.12, -0.04, -0.045, 0.17), "screw-steel", STEEL, solid=extruded(-kr, kr)))
            lst.append(shape("grip", "grip", "body", o + 0.1, True, 0.2, poly(gp), "grip-pe", PE, solid=revolved("grip")))
