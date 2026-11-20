"""🌱 Round 3 pendant lamp: builds the candidate asset document with every root correction (rig hanging straight down with the bulb on the lampholder, 0.6 m cord, masses, emitter direction, areas, texts, the 2D drawings of every scale and view drawn from the kit's profiles) and the model's volumes, writes it to `🗑️generated/r3-pendant-lamp/🔣️.json`; with `--apply` writes the changed members into the catalog file in its own formatting (re-read right before, member order kept, `volumes` untouched).

Usage: python3 r3-pendant-lamp-roots.py [--apply]
"""
import copy
import importlib.util
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/pendant-lamp/🔣️.json"))
CANDIDATE = os.path.join(HERE, "🗑️generated", "r3-pendant-lamp", "🔣️.json")
exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0])


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, path))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


K = load("kit", "r3-pendant-lamp-kit.py")
M = load("model", "r3-pendant-lamp-model.py")
TEXTS = json.load(open(os.path.join(HERE, "r3-pendant-lamp-texts.json"), encoding="utf-8"))
AREAS = {"cord": 0.0124}
REFERENCE = {"id": "iec-60238", "citation": "IEC 60238: Edison screw lampholders (E27 lampholder: thread engagement of the cap, outer dimensions and temperature marking)."}
JOINT_Y = {"rose": K.Y_ROSE, "cord": K.Y_ROSE - K.ROSE_H, "holder": K.Y_HOLDER, "shade": K.Y_SHADE, "bulb": K.Y_BULB}
RIG = {
    "rose": dict(x=0, y=K.Y_ROSE, angleDeg=-90, lengthM=K.ROSE_H, comX=0.012),
    "cord": dict(x=K.ROSE_H, y=0, angleDeg=0, lengthM=K.CORD_L, comX=0.3),
    "holder": dict(x=K.CORD_L, y=0, angleDeg=0, lengthM=K.SOCKET_MOUTH, comX=0.049),
    "shade": dict(x=K.COLLAR_H, y=0, angleDeg=0, lengthM=K.SHADE_H, comX=0.068),
    "bulb": dict(x=K.BULB_AT, y=0, angleDeg=0, lengthM=K.BULB_L, comX=0.053, parent="holder"),
}


def f(x):
    return ("%.5f" % (round(x, 5) + 0.0)).rstrip("0").rstrip(".") or "0"


def path(cmds):
    """✏️ A path string from commands [("M", [(u, v)]), ("C", [p1, p2, p3]), ("L", [p]), ("Z", [])]."""
    out = []
    for c, pts in cmds:
        out.append(" ".join([c] + [f(x) + " " + f(y) for x, y in pts]))
    return " ".join(out)


def mirror(cmds):
    """🪞 The left half of a symmetric outline: the right half's commands walked backwards with v negated (start point of each segment becomes its end)."""
    out = []
    seq = []
    last = None
    for c, ps in cmds:
        if c == "M":
            last = ps[0]
            continue
        seq.append((c, last, ps))
        last = ps[-1]
    for c, start, ps in reversed(seq):
        if c == "L":
            out.append(("L", [(start[0], -start[1])]))
        else:
            out.append(("C", [(ps[1][0], -ps[1][1]), (ps[0][0], -ps[0][1]), (start[0], -start[1])]))
    return out


def symmetric(right_start, right):
    """🔁 Closed outline from the right half (v > 0) starting at right_start on the top and ending on the bottom, mirrored to the left."""
    cmds = [("M", [right_start])] + right
    return cmds + [("L", [(right[-1][1][-1][0], -right[-1][1][-1][1])])] + mirror([("M", [right_start])] + right) + [("Z", [])]


def outline_profile(profile):
    """✏️ Right half of a turned part from its profile (points off the axis, in order) as L commands."""
    pts = [(a, r) for a, r in profile if r > 0]
    return pts[0], [("L", [p]) for p in pts[1:]]


def shade_outline(scale):
    """🔔 The side view of the dome, point for point the outer profile of that scale's turned shade: outer curve, rim (bead at 1:50, flare at 1:100 and 1:200), straight across the opening."""
    if scale == "50":
        te = K.bead_entry()
        pts = K.dome_points(40, te)
        u, r = pts[-1]
        phi_in = math.degrees(math.atan2(r - K.BEAD_C[1], u - K.BEAD_C[0])) % 360.0
        pts += [(K.BEAD_C[0] + K.BEAD_R * math.cos(math.radians(a)), K.BEAD_C[1] + K.BEAD_R * math.sin(math.radians(a))) for a in (phi_in - 50, 90, 45, 0)]
        return path(symmetric(pts[0], [("L", [q]) for q in pts[1:]]))
    pts = K.dome_points(10 if scale == "100" else 6, 0.97) + [(K.SHADE_H, 0.133)]
    right = [("L", [q]) for q in pts[1:]]
    if scale == "100":
        return path(symmetric(pts[0], right))
    neck = [("L", [(-K.COLLAR_H, 0.0155)]), ("L", [(0.0, 0.0155)]), ("L", [pts[0]])] + right
    return path(symmetric((-K.COLLAR_H, 0.0), neck))


def highlight():
    """✨ The highlight on the left flank of the dome: the outer curve 12 mm inside, from t 0.5 to 0.9."""
    pts = []
    for i in range(4):
        t = 0.5 + 0.4 * i / 3
        r, u = K.bez(t)
        dr, du = K.bez_d(t)
        L = math.hypot(dr, du)
        pts.append([round(u + 0.012 * dr / L, 5) + 0.0, round(-(r - 0.012 * du / L), 5) + 0.0])
    return pts


GLASS_XY = [((0.0, 0.0),), ((0.014, 0.0), (0.03, 0.014), (0.03, 0.032)), ((0.03, 0.046), (0.023, 0.054), (0.0185, 0.063)), ((0.016, 0.069), (0.0143, 0.073), (0.0138, 0.078))]


def glass_right():
    """💡 Right half of the A60 glass in the bulb frame from the neck at the cap down to the tip, as exact Beziers (the drawing's own curves)."""
    to_uv = lambda p: (K.BULB_L - p[1], p[0])
    segs = [GLASS_XY[3], GLASS_XY[2], GLASS_XY[1]]
    starts = [(0.0138, 0.078), (0.0185, 0.063), (0.03, 0.032)]
    ends = [(0.0185, 0.063), (0.03, 0.032), (0.0, 0.0)]
    out = []
    for seg, st, en in zip(segs, starts, ends):
        out.append(("C", [to_uv(seg[1]), to_uv(seg[0]), to_uv(en)]))
    return to_uv(starts[0]), out


def glass_path():
    s, r = glass_right()
    cmds = [("M", [s])] + r
    return path(cmds + mirror(cmds) + [("Z", [])])


def cap_path(scale):
    pts = [(0.0, 0.0045), (0.0032, 0.0105), (0.0045, K.CAP_R), (K.CAP_L, K.CAP_R)] if scale == "50" else [(0.0, 0.009), (0.004, K.CAP_R), (K.CAP_L, K.CAP_R)]
    right = [("L", [p]) for p in pts[1:]]
    return path(symmetric(pts[0], right))


def bulb200_path():
    top = K.SOCKET_TOP + 0.0024 - K.BULB_AT
    mouth = K.SOCKET_MOUTH - K.BULB_AT
    s, g = glass_right()
    right = [("L", [(mouth, K.SOCKET_R)]), ("L", [(mouth, K.CAP_R)]), ("L", [s])] + g
    cmds = [("M", [(top, K.SOCKET_R)])] + right
    return path(cmds + mirror(cmds) + [("Z", [])])


def rose_path(scale):
    if scale == "50":
        kk = 0.5523 * 0.008
        right = [("L", [(0.016, 0.05)]), ("C", [(0.016 + kk, 0.05), (0.024, 0.042 + kk), (0.024, 0.042)]), ("L", [(0.024, 0.007)]), ("L", [(0.031, 0.007)]), ("L", [(0.034, 0.0055)])]
        return path(symmetric((0.0, 0.05), right))
    s, right = outline_profile(K.ROSE[scale])
    return path(symmetric(s, right))


def collar_path(scale):
    s, right = outline_profile(K.COLLAR[scale][:-1] if scale == "50" else K.COLLAR[scale])
    return path(symmetric(s, right))


def to_front(d, bone):
    """↔️ The front elevation of an elevation path of a hanging bone: (u, v) -> (v, joint height - u)."""
    toks = d.split()
    out = []
    i = 0
    y0 = JOINT_Y[bone]
    while i < len(toks):
        t = toks[i]
        if t.isalpha():
            out.append(t)
            i += 1
            continue
        u, v = float(toks[i]), float(toks[i + 1])
        out += [f(v), f(y0 - u)]
        i += 2
    return " ".join(out)


def old_to_uv(bone, p):
    """🔄 A point of the old upright bone frame (x ahead, y up from the old joint) in the new hanging frame."""
    old_joint = {"bulb": 0.0, "shade": JOINT_Y["shade"]}[bone]
    return [round(JOINT_Y[bone] - (old_joint + p[1]), 5) + 0.0, round(p[0], 5) + 0.0]


def circle(r, n):
    return [[round(r * math.cos(2 * math.pi * i / n), 5) + 0.0, round(r * math.sin(2 * math.pi * i / n), 5) + 0.0] for i in range(n)]


def redraw(scale, geometry):
    """🖊️ Every shape of one scale redrawn from the kit, ids, roles, z, tones, regions and materials kept."""
    g = copy.deepcopy(geometry)
    el = {s["id"]: s for s in g["elevation"]}
    paths = {"rose": rose_path(scale), "shade": shade_outline(scale)}
    if scale in ("50", "100"):
        paths.update({"collar": collar_path(scale), "bulb-cap": cap_path(scale), "bulb-glass": glass_path()})
    else:
        paths["bulb"] = bulb200_path()
    width = {"50": 0.0066, "100": 0.009, "200": 0.014}[scale]
    for view in ("elevation", "front"):
        for s in g[view]:
            sid = s["id"]
            if sid in paths:
                s["d"] = paths[sid] if view == "elevation" else to_front(paths[sid], s["bone"])
            elif sid == "cord":
                if view == "elevation":
                    s.update(x=0, y=-width / 2, w=K.CORD_L, h=width)
                else:
                    s.update(x=-width / 2, y=round(K.Y_HOLDER, 5), w=width, h=K.CORD_L)
            elif sid == "shade-light":
                uv = highlight()
                s["points"] = uv if view == "elevation" else [[q[1], round(JOINT_Y["shade"] - q[0], 5) + 0.0] for q in uv]
            elif "points" in s:
                src = [q for q in geometry["elevation"] if q["id"] == sid][0]["points"]
                uv = [old_to_uv(s["bone"], p) for p in src]
                s["points"] = uv if view == "elevation" else [[q[1], round(JOINT_Y[s["bone"]] - q[0], 5) + 0.0] for q in uv]
    keep = []
    for s in g["plan"]:
        if s["id"] in ("plan-shade-rim", "plan-rose-face"):
            continue
        if s["id"] == "plan-shade":
            s["points"] = circle(0.133, len(s["points"]))
        if s["id"] == "plan-rose":
            s["points"] = circle(0.05, len(s["points"]))
        keep.append(s)
    g["plan"] = keep
    return g


def roots(asset):
    """🌱 Every root correction in one place, applied to a copy of the catalog asset."""
    a = copy.deepcopy(asset)
    a["summary"] = TEXTS["summary"]
    a["provenance"]["notes"] = TEXTS["notes"]
    if not any(r["id"] == REFERENCE["id"] for r in a["provenance"]["references"]):
        a["provenance"]["references"].append(REFERENCE)
    for b in a["rig"]["bones"]:
        r = RIG[b["id"]]
        b["x"], b["y"], b["angleDeg"], b["lengthM"] = r["x"], r["y"], r["angleDeg"], round(r["lengthM"], 5)
        if "parent" in r:
            b["parent"] = r["parent"]
        b["mass"]["comX"], b["mass"]["comY"] = r["comX"], 0
        if b["id"] == "holder":
            b["title"] = {"en": "Brass collar and E27 lampholder", "de": "Messinghülse und E27-Fassung"}
    for e in a["thermal"]["emitters"]:
        if e["id"] == "shade-plume":
            e["directionDeg"] = 180
    for region in a["thermal"]["regions"]:
        region["areaM2"] = AREAS.get(region["id"], region["areaM2"])
    for p in a["poses"]:
        if p["id"] == "on":
            p["description"] = TEXTS["poseOn"]
    for den, rep in a["representations"].items():
        if rep["status"] != "authored":
            continue
        rep["geometry"] = redraw(den, rep["geometry"])
        rep["decisions"] = TEXTS["representations"][den]
    return a


def dump(key, value):
    s = "    " + json.dumps(key) + ": " + json.dumps(value, indent=2, ensure_ascii=False).replace("\n", "\n    ")
    return s.replace("6e-06", "0.000006").replace("1.2e-05", "0.000012")


def apply(want):
    text = open(CATALOG, encoding="utf-8").read()
    have = json.loads(text)["asset"]
    changed = [key for key in have if key != "volumes" and have[key] != want[key]]
    for key in changed:
        start, end = member_range(text, "    ", key)
        text = text[:start] + dump(key, want[key]) + text[end:]
    got = json.loads(text)["asset"]
    if list(got) != list(have) or any(got[key] != want[key] for key in want if key != "volumes") or got["volumes"] != have["volumes"]:
        raise SystemExit("the edited text does not match the candidate")
    open(CATALOG, "w", encoding="utf-8").write(text)
    return changed


def main():
    doc = json.load(open(CATALOG, encoding="utf-8"))
    want = roots(doc["asset"])
    cand = copy.deepcopy(doc)
    cand["asset"] = copy.deepcopy(want)
    cand["asset"]["volumes"] = M.volumes()
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    with open(CANDIDATE, "w", encoding="utf-8") as fh:
        json.dump(cand, fh, indent=2, ensure_ascii=False)
        fh.write("\n")
    print("candidate", CANDIDATE)
    if "--apply" in sys.argv:
        print("written", CATALOG, "members:", ", ".join(apply(want)) or "none")


if __name__ == "__main__":
    main()
