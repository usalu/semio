"""🐿️ Red squirrel: shoulder and pelvic girdles on lateral rest placement, rib cage, sternum, sacrum and pubic ring by hand.

Usage: python3 rework-red-squirrel-girdles.py candidate|publish
Builds from the state before this rework kept in rework-red-squirrel-orig.json (rig, poses, structure lists); frames as in rig-lateral-rest.md.
"""
import copy
import json
import math
import re
import sys

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/red-squirrel/🔣️.json"
SOURCE = "volumes/animals/red-squirrel.json"
MM = 0.001
DEFAULT_Q = [0, -0.7071068, 0, 0.7071068]


def sub(a, b): return [a[i] - b[i] for i in range(3)]
def add(a, b): return [a[i] + b[i] for i in range(3)]
def mul(a, s): return [a[i] * s for i in range(3)]
def dot(a, b): return sum(a[i] * b[i] for i in range(3))
def cross(a, b): return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
def norm(a): return math.sqrt(dot(a, a))
def unit(a): return mul(a, 1 / norm(a))
def r6(v): return [round(x, 6) for x in v]


def frame(u, hint):
    u = unit(u)
    w = unit(sub(hint, mul(u, dot(hint, u))))
    return [u, cross(w, u), w]


def to_local(basis, origin, p): return [dot(basis[i], sub(p, origin)) for i in range(3)]
def to_world(basis, origin, p): return add(origin, add(add(mul(basis[0], p[0]), mul(basis[1], p[1])), mul(basis[2], p[2])))


def quat(cols):
    m = [[cols[j][i] for j in range(3)] for i in range(3)]
    t = m[0][0] + m[1][1] + m[2][2]
    if t > 0:
        s = math.sqrt(t + 1) * 2
        q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [s / 4, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / s, s / 4, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, s / 4, (m[1][0] - m[0][1]) / s]
    return q


def qmul(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]


def placement(parent_basis, parent_origin, basis, origin):
    o = to_local(parent_basis, parent_origin, origin)
    m = [[dot(parent_basis[i], basis[j]) for j in range(3)] for i in range(3)]
    b = math.asin(max(-1, min(1, m[2][0])))
    a = math.atan2(m[1][0], m[0][0])
    ca, sa, cb, sb = math.cos(a), math.sin(a), math.cos(b), math.sin(b)
    rab = [[ca * cb, sa * cb, sb], [-sa, ca, 0], [-ca * sb, -sa * sb, cb]]
    v = [m[i][1] for i in range(3)]
    t = math.atan2(-dot(rab[2], v), dot(rab[1], v))
    values = {"x": round(o[0], 6), "y": round(o[1], 6), "z": round(o[2], 6), "angleDeg": round(math.degrees(a), 4), "abductionDeg": round(math.degrees(b), 4), "twistDeg": round(math.degrees(t), 4)}
    return {k: (0 if v == 0 else v) for k, v in values.items() if v != 0 or k in ("x", "y", "angleDeg")}


def mirror(p): return [p[0], p[1], -p[2]]


IDENT = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
SPINE_R = [0, 0, 0]
SPINE_F = [0.072, 0, 0]
SCAPULA_DORSAL = [0.1025, 0.0103, -0.0108]
GLENOID = [0.124, -0.012, -0.015]
STERNOCLAVICULAR = [0.1305, -0.0069, -0.0018]
ACROMION = [0.1260, -0.0095, -0.0172]
SACROILIAC = [0.030, 0.0095, -0.0086]
ACETABULUM = [0.016, 0.002, -0.013]
ARM_BASIS = [[0, -1, 0], [1, 0, 0], [0, 0, 1]]
S60 = math.sqrt(3) / 2
THIGH_BASIS = [[0.5, -S60, 0], [S60, 0.5, 0], [0, 0, 1]]


def girdle(side):
    far = side == "f"
    m = mirror if far else (lambda p: p)
    sc_o, sc_t = m(SCAPULA_DORSAL), m(GLENOID)
    cl_o, cl_t = m(STERNOCLAVICULAR), m(ACROMION)
    hp_o, hp_t = m(SACROILIAC), m(ACETABULUM)
    return {
        "scapula": (frame(sub(sc_t, sc_o), [0, 0, 1]), sc_o, norm(sub(sc_t, sc_o))),
        "clavicle": (frame(sub(cl_t, cl_o), [-1, 0, 0] if far else [1, 0, 0]), cl_o, norm(sub(cl_t, cl_o))),
        "hip": (frame(sub(hp_t, hp_o), [0, 0, 1]), hp_o, norm(sub(hp_t, hp_o))),
        "glenoid": sc_t, "acetabulum": hp_t,
    }


def rig(original):
    bones = {b["id"]: b for b in copy.deepcopy(original["bones"])}
    out = []
    names = {"n": ("Near", "Nahes", "Nahe", "Naher"), "f": ("Far", "Fernes", "Ferne", "Ferner")}
    for b in copy.deepcopy(original["bones"]):
        side = b["id"][-1]
        if b["id"] in ("arm-n", "arm-f"):
            g = girdle(side)
            sb, so, sl = g["scapula"]
            cb, co, cl = g["clavicle"]
            en, de_n, de_f, de_m = names[side]
            scapula = {"id": "scapula-" + side, "parent": "spine-f", "title": {"en": en + " shoulder blade", "de": de_n + " Schulterblatt"}}
            scapula.update(placement(IDENT, SPINE_F, sb, so))
            scapula.update({"lengthM": round(sl, 6), "joint": {"type": "revolute", "minDeg": -20, "maxDeg": 20}})
            clavicle = {"id": "clavicle-" + side, "parent": "spine-f", "title": {"en": en + " collarbone", "de": de_n + " Schlüsselbein"}}
            clavicle.update(placement(IDENT, SPINE_F, cb, co))
            clavicle.update({"lengthM": round(cl, 6), "joint": {"type": "revolute", "minDeg": -15, "maxDeg": 15}})
            arm = {"id": b["id"], "parent": "scapula-" + side, "title": b["title"]}
            arm.update(placement(sb, so, ARM_BASIS, g["glenoid"]))
            arm.update({k: v for k, v in b.items() if k not in ("id", "parent", "title", "x", "y", "z", "angleDeg", "abductionDeg", "twistDeg")})
            out += [clavicle, scapula, arm]
        elif b["id"] in ("thigh-n", "thigh-f"):
            g = girdle(side)
            hb, ho, hl = g["hip"]
            en, de_n, de_f, de_m = names[side]
            hip = {"id": "hip-" + side, "parent": "spine-r", "title": {"en": en + " hip bone", "de": de_n + " Hüftbein"}}
            hip.update(placement(IDENT, SPINE_R, hb, ho))
            hip.update({"lengthM": round(hl, 6), "joint": {"type": "revolute", "minDeg": -2, "maxDeg": 2}})
            thigh = {"id": b["id"], "parent": "hip-" + side, "title": b["title"]}
            thigh.update(placement(hb, ho, THIGH_BASIS, g["acetabulum"]))
            thigh.update({k: v for k, v in b.items() if k not in ("id", "parent", "title", "x", "y", "z", "angleDeg", "abductionDeg", "twistDeg")})
            out += [hip, thigh]
        else:
            out.append(b)
    result = copy.deepcopy(original)
    result["bones"] = out
    return result


def rebind(solid, old_origin, basis, origin, bone, member):
    s = copy.deepcopy(solid)
    q = quat([[basis[j][i] for j in range(3)] for i in range(3)])
    for sec in s["sections"]:
        world = add(old_origin, sec["at"])
        sec["at"] = r6(to_local(basis, origin, world))
        sec["rotation"] = [round(x, 7) for x in qmul(q, sec.get("rotation", DEFAULT_Q))]
    s["bone"] = bone
    s["member"] = member
    return s


def loft(id, bone, member, sections, slender=False, role="body"):
    s = {"id": id, "kind": "loft", "bone": bone, "material": "bone", "role": role, "tone": 0.85, "sections": sections, "capStart": True, "capEnd": True}
    if slender:
        s["slender"] = True
    s.update({"member": member, "part": "member"})
    return s


def section(at_mm, rx, ry, q=None):
    return {"at": r6(mul(at_mm, MM)), "rx": round(rx * MM, 6), "ry": round(ry * MM, 6), "exponent": 2, "rotation": [round(x, 7) for x in (q or DEFAULT_Q)]}


def spline(points, count):
    p = [points[0]] + points + [points[-1]]
    out = []
    spans = len(points) - 1
    for k in range(count):
        t = k * spans / (count - 1)
        i = min(int(t), spans - 1)
        f = t - i
        a, b, c, d = p[i], p[i + 1], p[i + 2], p[i + 3]
        out.append([0.5 * ((2 * b[j]) + (-a[j] + c[j]) * f + (2 * a[j] - 5 * b[j] + 4 * c[j] - d[j]) * f * f + (-a[j] + 3 * b[j] - 3 * c[j] + d[j]) * f ** 3) for j in range(3)])
    return out


def tube(path, radius, tip=1.0):
    sections = []
    for k, at in enumerate(path):
        t = unit(sub(path[min(k + 1, len(path) - 1)], path[max(k - 1, 0)]))
        seed = [1, 0, 0] if abs(t[0]) < 0.8 else [0, 1, 0]
        ex = unit(cross(seed, t))
        r = radius * (tip if k in (0, len(path) - 1) else 1)
        sections.append(section(at, r, r, quat([ex, cross(t, ex), t])))
    return sections


def interp(table, x):
    if x <= table[0][0]:
        return table[0][1]
    for (x0, y0), (x1, y1) in zip(table, table[1:]):
        if x <= x1:
            return y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    return table[-1][1]


COLUMN_Y = [(0, 14.6), (10, 14.8), (20, 14.8), (30, 14.0), (40, 12.4), (48, 10.9), (56, 8.8), (59, 7.6), (62, 6.4)]
STERNUM = [(59, -6.6, 1.2, 1.2), (56, -8.4, 2.4, 1.3), (53, -10.2, 2.0, 1.3), (48.5, -13.0, 1.5, 1.2), (44, -15.6, 1.5, 1.2), (39.5, -17.9, 1.5, 1.2), (35, -19.8, 1.5, 1.2), (30.5, -21.3, 1.6, 1.2), (26, -22.0, 1.4, 1.0), (21, -21.7, 1.8, 0.7), (16, -21.2, 2.2, 0.5)]
STERNUM_Y = [(x, y) for x, y, _, _ in reversed(STERNUM)]
RIB_HALF_WIDTH = {1: 5.5, 2: 6.8, 3: 8.0, 4: 9.2, 5: 10.4, 6: 11.8, 7: 13.4, 8: 14.8, 9: 15.8, 10: 16.2, 11: 15.8, 12: 14.6}
COSTAL_ARCH = {8: [25.5, -19.5, 6.0], 9: [21.5, -16.5, 10.0], 10: [17.5, -12.5, 13.0]}
XIPHISTERNAL = [27.5, -21.0, 2.0]


def rib_path(i):
    xh = 59.4 - 3.9 * (i - 1)
    yv = interp(COLUMN_Y, xh)
    w = RIB_HALF_WIDTH[i]
    head = [[xh, yv - 0.3, 3.0], [xh - 0.3, yv + 0.8, 0.45 * w + 2.2]]
    if i <= 7:
        xs = 56.5 - 4.6 * (i - 1)
        ys = interp(STERNUM_Y, xs)
        depth = yv - ys
        slant = 0
        return head + [[xh - 0.25 * slant - 0.5, yv - 0.18 * depth, 0.92 * w], [xh - 0.6 * slant - 0.7, yv - 0.40 * depth, w], [xh - slant - 0.8, yv - 0.64 * depth, 0.90 * w], [(xh - slant - 0.8 + xs) / 2 - 0.5, yv - 0.84 * depth, 0.62 * w], [xs, ys + 0.4, 1.6]]
    if i <= 10:
        arch = COSTAL_ARCH[i]
        end = XIPHISTERNAL if i == 8 else COSTAL_ARCH[i - 1]
        low = [xh - 1.8, yv - 12.5, w]
        return head + [[xh - 1.0, yv - 5.5, 0.92 * w], low, [(low[0] + arch[0]) / 2 - 0.4, (low[1] + arch[1]) / 2 - 1.0, (low[2] + arch[2]) / 2 + 2.0], arch, end]
    if i == 11:
        return head + [[xh - 1.0, yv - 5.5, 0.92 * w], [xh - 1.8, yv - 12.0, w], [xh - 3.5, yv - 18.0, 0.86 * w]]
    return head + [[xh - 1.0, yv - 5.0, 0.92 * w], [xh - 1.8, yv - 10.0, w], [xh - 3.5, yv - 14.5, 0.9 * w]]


def ribs(numbers, count, radius):
    out = []
    for i in numbers:
        path = spline(rib_path(i), count)
        out.append(loft("rib-n-%d" % i, "spine-f", "spine-f", tube([[p[0], p[1], -p[2]] for p in path], radius, 0.7), True))
        out.append(loft("rib-f-%d" % i, "spine-f", "spine-f", tube(path, radius, 0.7), True))
    return out


def sternum(rows, slender):
    return loft("sternum", "spine-f", "spine-f", [section([x, y, 0], rx, ry) for x, y, rx, ry in rows], slender)


SACRUM = [(13, 13.9, 4.2, 3.4), (18, 14.2, 5.6, 3.6), (23, 13.2, 8.0, 4.0), (28, 11.6, 10.2, 4.6), (33, 11.6, 10.2, 4.6), (36, 13.8, 6.8, 4.0), (38.5, 15.4, 4.2, 3.6)]
ILIUM_COARSE = [([40, 11.2, -8.0], 1.4, 2.6), ([30, 8.6, -9.2], 1.9, 4.4), ([16, 2, -10.5], 3.6, 5.0), ([5, -2.2, -10.9], 2.0, 5.2), ([-4.2, -5.3, -11.0], 1.4, 3.0)]
PUBIS = [[14.0, -1.0, -10.6], [10.5, -5.2, -8.2], [6.5, -8.4, -4.2], [3.4, -9.6, -0.5]]
ISCHIOPUBIC = [[3.4, -9.6, -0.5], [0.2, -9.2, -4.6], [-2.6, -7.6, -8.6], [-4.4, -5.6, -10.8]]
SCAPULAR_SPINE = [([5, 0.6, -0.9], 0.5, 0.5), ([10, 0.6, -1.6], 1.1, 0.5), ([16, 0.7, -2.2], 1.7, 0.55), ([22, 0.9, -2.7], 2.2, 0.6), ([26, 1.4, -3.0], 1.9, 0.7), ([29.5, 2.6, -3.0], 1.2, 0.9), ([32.2, 3.6, -2.6], 0.9, 0.9)]
CLAVICLE = [([-0.4, 0, 0], 1.1), ([3, 0.5, 0], 0.8), ([8, 0.9, 0], 0.75), ([13, 0.4, 0], 0.8), ([16.6, 0, 0], 1.0)]


def side_solids(side, scale, original):
    far = side == "f"
    zs = -1 if far else 1
    g = girdle(side)
    sb, so, _ = g["scapula"]
    hb, ho, _ = g["hip"]
    name = {"n": "near", "f": "far"}[side]
    out = []
    if scale == "50":
        out.append(rebind(original["scapula-" + side], SPINE_F, sb, so, "scapula-" + side, "scapula-" + side))
        out.append(loft("scapular-spine-" + side, "scapula-" + side, "scapula-" + side, [section([at[0], at[1], at[2] * zs], rx, ry) for at, rx, ry in SCAPULAR_SPINE], True))
        hip = rebind(original["pelvis-" + side], SPINE_R, hb, ho, "hip-" + side, "hip-" + side)
        hip["id"] = "hip-" + side
        out.append(hip)
        world = [mul([p[0], p[1], p[2] * zs], MM) for p in spline(PUBIS + ISCHIOPUBIC[1:], 13)]
        local = [mul(to_local(hb, ho, p), 1 / MM) for p in world]
        out.append(loft("pubis-" + side, "hip-" + side, "hip-" + side, tube(local, 1.1, 0.9), True))
    else:
        out.append(loft("scapula-" + side, "scapula-" + side, "scapula-" + side, [section([x, 0, 0], 0.75, 0.75) for x in (-0.3, 15.5, 31.5)], True))
        world = [mul([at[0], at[1], at[2] * zs], MM) for at, _, _ in ILIUM_COARSE]
        local = [mul(to_local(hb, ho, p), 1 / MM) for p in (world if scale == "100" else [world[0], world[2], world[4]])]
        out.append(loft("hip-" + side, "hip-" + side, "hip-" + side, tube(local, 1.0), True))
    rows = CLAVICLE if scale == "50" else [(at, 0.4) for at, _ in (CLAVICLE[0], CLAVICLE[2], CLAVICLE[4])]
    out.append(loft("clavicle-" + side, "clavicle-" + side, "clavicle-" + side, [section(at, r, r) for at, r in rows], True))
    return out


LUMBAR_REAR = [-4.0, 4.5, 13.0, 21.5, 30.0, 38.5, 45.1, 51.7, 58.3, 64.9, 71.8]
THORACOLUMBAR = [0.2, 6.6, 13.2] + [round(13.2 + 3.9 * k, 1) for k in range(1, 13)] + [63.6]
CERVICAL = [-1.6, 2.2, 5.9, 9.6, 13.3, 17.0, 20.7, 24.6]


def beads(original, joints, widen=None):
    rows = [(s["at"][0] / MM, s["at"][1] / MM, s["rx"] / MM, s["ry"] / MM) for s in original["sections"]]
    at = lambda x, k: interp([(r[0], r[k]) for r in rows], x)
    out = [section([rows[0][0], rows[0][1], 0], rows[0][2] * 0.8, rows[0][3] * 0.8)]
    for index, (a, b) in enumerate(zip(joints, joints[1:])):
        length = b - a
        grow = (widen or {}).get(index, 1.0)
        for f, scale in ((0.12, 1.0), (0.5, 0.74), (0.88, 1.0)):
            x = a + f * length
            out.append(section([x, at(x, 1), 0], at(x, 2) * scale * grow, at(x, 3) * scale))
        out.append(section([b, at(b, 1), 0], at(b, 2) * 0.84, at(b, 3) * 0.84))
    out.append(section([rows[-1][0], rows[-1][1], 0], rows[-1][2] * 0.7, rows[-1][3] * 0.7))
    result = copy.deepcopy(original)
    result["sections"] = out
    return result


def crest(id, bone, original, joints, lift, height):
    rows = [(s["at"][0] / MM, s["at"][1] / MM, s["ry"] / MM) for s in original["sections"]]
    top = lambda x: interp([(r[0], r[1] + r[2]) for r in rows], x)
    out = []
    for a, b in zip(joints, joints[1:]):
        for f, h in ((0.15, 0.5), (0.55, height), (0.9, 0.5)):
            x = a + f * (b - a)
            out.append(section([x, top(x) + lift + h - 0.6, 0], 0.55, h + 0.6))
    return loft(id, bone, bone, out, True)


def structure(scale, original_list, reference):
    drop = lambda s: s["id"].startswith("rib-") or s["id"] in ("sternum", "scapula-n", "scapula-f", "pelvis-n", "pelvis-f", "pelvis")
    kept = [s for s in original_list if not drop(s)]
    if scale == "50":
        column = {"m-spine-r": (LUMBAR_REAR, None), "m-spine-f": (THORACOLUMBAR, None), "m-neck": (CERVICAL, {6: 1.35, 5: 1.15})}
        kept = [beads(s, *column[s["id"]]) if s["id"] in column else s for s in kept]
    new = []
    if scale == "50":
        new.append(loft("sacrum", "spine-r", "spine-r", [section([x, y, 0], rx, ry) for x, y, rx, ry in SACRUM]))
        new.append(sternum(STERNUM, True))
        new.append(crest("spinous-processes-r", "spine-r", reference["m-spine-r"], LUMBAR_REAR[5:], -0.3, 2.4))
        new.append(crest("spinous-processes-f", "spine-f", reference["m-spine-f"], THORACOLUMBAR[:15], -0.3, 2.2))
        new += ribs(range(1, 13), 11, 0.75)
    elif scale == "100":
        new.append(loft("sacrum", "spine-r", "spine-r", [section([x, y, 0], rx, ry) for x, y, rx, ry in (SACRUM[0], SACRUM[2], (28, 11.6, 10.2, 5.4), (33, 11.6, 10.2, 5.4), SACRUM[6])]))
        new.append(sternum([STERNUM[k] for k in (0, 1, 4, 7, 10)], True))
        new += ribs((3, 5, 7, 9, 11), 8, 0.7)
    elif scale == "200":
        new.append(loft("sacrum", "spine-r", "spine-r", [section([x, y, 0], rx, ry) for x, y, rx, ry in (SACRUM[0], SACRUM[3], SACRUM[6])]))
        new.append(sternum([(x, y, 0.95, 0.95) for x, y, _, _ in (STERNUM[0], STERNUM[4], STERNUM[8])], True))
        new += ribs((5, 8, 11), 6, 0.7)
    else:
        new.append(loft("sacrum", "spine-r", "spine-r", [section([x, y, 0], rx, ry) for x, y, rx, ry in (SACRUM[0], SACRUM[3], SACRUM[6])]))
        new.append(sternum([(x, y, 0.95, 0.95) for x, y, _, _ in (STERNUM[0], STERNUM[4], STERNUM[8])], True))
        new += ribs((5, 9), 5, 0.7)
    for side in ("n", "f"):
        new += side_solids(side, scale, reference)
    index = max(k for k, s in enumerate(kept) if s["bone"] in ("spine-r", "spine-f", "neck", "head")) + 1
    return kept[:index] + new + kept[index:]


def scalar(value): return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160)
    return False


def dump(value, depth):
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1) for key, item in value.items()) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def member_end(text, start):
    depth, index, quoted = 0, start, False
    while True:
        char = text[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return index + 1
        index += 1


def main():
    mode = sys.argv[1]
    original = json.load(open("rework-red-squirrel-orig.json", encoding="utf-8"))
    current_source = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    new_rig = rig(original["rig"])
    reference = {s["id"]: s for s in original["structure"]["50"]}
    volumes = current_source
    for scale in ("50", "100", "200", "500"):
        volumes[scale]["structure"] = structure(scale, original["structure"][scale], reference)
    if mode == "candidate":
        document = json.load(open(CATALOG, encoding="utf-8"))
        document["asset"]["rig"] = new_rig
        document["asset"]["volumes"] = volumes
        import os
        os.makedirs("🗑️generated/r3-red-squirrel", exist_ok=True)
        json.dump(document, open("🗑️generated/r3-red-squirrel/🔣️.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        print("candidate written", len(new_rig["bones"]), "bones", {k: len(v["structure"]) for k, v in volumes.items()})
        for b in new_rig["bones"]:
            if b["id"].split("-")[0] in ("scapula", "clavicle", "hip", "arm", "thigh"):
                print(json.dumps({k: v for k, v in b.items() if k not in ("title", "mass", "joint")}))
    elif mode == "publish":
        open(SOURCE, "w", encoding="utf-8").write('{"volumes": ' + dump(volumes, 0) + "}\n")
        text = open(CATALOG, encoding="utf-8").read()
        found = re.search(r'\n    "rig"\s*:\s*', text)
        end = member_end(text, found.end())
        body = json.dumps(new_rig, ensure_ascii=False, indent=2).replace("\n", "\n    ")
        result = text[: found.end()] + body + text[end:]
        check = json.loads(result)
        before = json.loads(text)
        if check["asset"]["rig"] != new_rig or {k: v for k, v in check["asset"].items() if k != "rig"} != {k: v for k, v in before["asset"].items() if k != "rig"}:
            raise SystemExit("rig splice did not round-trip")
        if open(CATALOG, encoding="utf-8").read() != text:
            raise SystemExit("catalog changed while writing; retry")
        open(CATALOG, "w", encoding="utf-8").write(result)
        print("source and catalog rig written")


main()
