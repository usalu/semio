"""NW-H3 scratch library: scaled human rig from the commuter template, masses, forward kinematics, IK, gait generators, local checks and
sketch rendering. Arithmetic helpers only; every shape, pose intent and thermal parameter is authored in the per-asset scripts."""
import copy, json, math, os, sys
from collections import OrderedDict

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from lib import *  # noqa: E402,F401,F403

BASE = json.load(open(f"{HERE}/base.json"))
DEG = math.pi / 180


def r4(v):
    return round(v, 4)


def asin_deg(x):
    return math.degrees(math.asin(x))


# ------------------------------------------------------------------------------------------------ rig
SEGMENT_FRACTIONS = {
    "f": dict(head=0.0668, trunk=0.4257, pelvis=0.1247, uarm=0.0255, farm=0.0138, hand=0.0056, thigh=0.1478, shank=0.0481, foot=0.0129),
    "m": dict(head=0.0694, trunk=0.4346, pelvis=0.1117, uarm=0.0271, farm=0.0162, hand=0.0061, thigh=0.1416, shank=0.0433, foot=0.0137),
}
COM_FRACTIONS = {
    "f": dict(thigh=0.3612, shank=0.4352, uarm=0.5754, farm=0.4559, hand=0.7474, foot=0.4014, head=0.5894),
    "m": dict(thigh=0.4095, shank=0.4459, uarm=0.5772, farm=0.4574, hand=0.7900, foot=0.4415, head=0.5976),
}


def build_rig(sex, mass, hip_to_top=None, sL=1.0, sT=1.0, sA=1.0, sH=1.0, sF=1.0, hip_x=0.0, stature=None, extra_mass=None):
    """Scaled copy of the commuter rig; returns (rig, anchors, geometry dict of lengths). stature solves sT when given."""
    ankle = 0.075 * sF
    hip_y = (0.435 + 0.425) * sL + ankle
    if stature is not None:
        sT = (stature - hip_y - 0.18 * sH) / 0.665
    L = dict(sL=sL, sT=sT, sA=sA, sH=sH, sF=sF, hip_y=hip_y, ankle=ankle, stature=hip_y + 0.665 * sT + 0.18 * sH)
    alpha = asin_deg(ankle / (0.1858 * sF))
    fr, cf = SEGMENT_FRACTIONS[sex], COM_FRACTIONS[sex]
    bones = []
    for b in BASE["rig"]["bones"]:
        if b["id"] == "bag":
            continue
        b = copy.deepcopy(b)
        i, side = b["id"], b["id"].split("-")[-1]
        base = b["id"].rsplit("-", 1)[0] if b["id"][-2:] in ("-l", "-r") else b["id"]
        if base == "pelvis":
            b.update(x=hip_x, y=hip_y, lengthM=0.1 * sT)
        elif base == "torso":
            b.update(x=0.1 * sT, lengthM=0.465 * sT)
        elif base == "neck":
            b.update(x=0.465 * sT, lengthM=0.1 * sT)
        elif base == "head":
            b.update(x=0.1 * sT, lengthM=0.18 * sH)
        elif base == "arm":
            b.update(x=0.42 * sT, lengthM=0.33 * sA)
        elif base == "forearm":
            b.update(x=0.33 * sA, lengthM=0.26 * sA)
        elif base == "hand":
            b.update(x=0.26 * sA, lengthM=0.19 * sA)
        elif base == "thigh":
            b.update(x=0, lengthM=0.435 * sL)
        elif base == "shank":
            b.update(x=0.435 * sL, lengthM=0.425 * sL)
        elif base == "foot":
            b.update(x=0.425 * sL, lengthM=0.1858 * sF, angleDeg=round(90 - alpha, 3))
        elif base == "toe":
            b.update(x=0.1858 * sF, lengthM=0.065 * sF, angleDeg=round(alpha, 3))
        for k in ("x", "y", "lengthM"):
            b[k] = round(b[k], 5)
        bones.append(b)
    by = {b["id"]: b for b in bones}
    nm = dict(thigh="thigh", shank="shank", foot="foot", hand="hand")
    def setm(bid, kg, frac_com, length=None, dist="point"):
        ln = by[bid]["lengthM"] if length is None else length
        by[bid]["mass"] = {"kg": round(kg, 3), "comX": round(frac_com * ln, 4), "comY": 0, "distribution": dist, "source": "literature", "reference": "de-leva-1996"}
    setm("pelvis", mass * fr["pelvis"], 0.6)
    setm("torso", mass * (fr["trunk"] - fr["pelvis"]), 0.462)
    hn = mass * fr["head"]
    setm("head", hn * 0.86, 0.417)
    setm("neck", hn * 0.14, 0.5)
    by["neck"]["mass"]["source"] = "authored-estimate"
    for s in ("l", "r"):
        setm(f"arm-{s}", mass * fr["uarm"], cf["uarm"])
        setm(f"forearm-{s}", mass * fr["farm"], cf["farm"])
        setm(f"hand-{s}", mass * fr["hand"], cf["hand"])
        by[f"hand-{s}"]["mass"]["reference"] = "winter-2009"
        setm(f"thigh-{s}", mass * fr["thigh"], cf["thigh"])
        setm(f"shank-{s}", mass * fr["shank"], cf["shank"])
        setm(f"foot-{s}", mass * fr["foot"] * 0.92, 0.43 if sex == "f" else 0.43)
        setm(f"toe-{s}", mass * fr["foot"] * 0.08, 0.46)
        by[f"toe-{s}"]["mass"]["source"] = "authored-estimate"
    # trunk fraction above already minus pelvis: the commuter split (pelvis 11-12 %, torso rest) is kept
    anchors = []
    for an in BASE["anchors"]:
        if an["id"] in ("bag-cog", "strap-grip"):
            continue
        an = copy.deepcopy(an)
        if an["bone"].startswith("foot"):
            an["x"], an["y"] = round(an["x"] * sF, 4), round(an["y"] * sF, 4)
            an["footprintLengthM"] = round(an.get("footprintLengthM", 0.07) * sF, 4)
            an["footprintWidthM"] = round(an.get("footprintWidthM", 0.07) * (0.9 if sex == "f" else 1.0), 4)
        elif an["bone"] == "head":
            an["x"], an["y"] = round(an["x"] * sH, 4), round(an["y"] * sH, 4)
        anchors.append(an)
    return {"kind": "anatomical", "root": "pelvis", "bones": bones}, anchors, L


def add_bone(rig, bid, parent, x, y, angle, length, joint, title, mass=None, section=None):
    b = {"id": bid, "parent": parent, "title": title, "x": x, "y": y, "angleDeg": angle, "lengthM": length, "joint": joint}
    if mass:
        b["mass"] = mass
    if section:
        b["section"] = section
    rig["bones"].append(b)
    return b


def bone(rig, bid):
    return next(b for b in rig["bones"] if b["id"] == bid)


def total_mass(rig):
    return sum(b.get("mass", {}).get("kg", 0) for b in rig["bones"])


def rescale_masses(rig, target, only=None):
    cur = total_mass(rig)
    for b in rig["bones"]:
        if "mass" in b and (only is None or b["id"] in only):
            b["mass"]["kg"] = round(b["mass"]["kg"] * target / cur, 3)


def mass_sum_fix(rig, target):
    """put the rounding residual into the pelvis so that the masses sum exactly"""
    d = round(target - total_mass(rig), 3)
    bone(rig, "pelvis")["mass"]["kg"] = round(bone(rig, "pelvis")["mass"]["kg"] + d, 3)


# ------------------------------------------------------------------------------------------------ kinematics
def asset_stub(rig, anchors):
    return {"rig": rig, "anchors": anchors}


def Wof(a, bones, root=None):
    return fk(a, {"bones": bones, "root": root or {}})


def anchor_world(a, W, aid):
    an = next(x for x in a["anchors"] if x["id"] == aid)
    return apply(W[an["bone"]], (an["x"], an["y"]))


def ground_y(a, bones, contacts):
    """root.y so that the lowest listed contact anchor sits on y = 0 (engine ground auto)"""
    W = Wof(a, bones)
    return -min(anchor_world(a, W, c)[1] for c in contacts)


def wangle(W, b):
    return math.atan2(W[b][1], W[b][0])


def ik_two(p0, l1, l2, target, bend):
    dx, dy = target[0] - p0[0], target[1] - p0[1]
    d = math.hypot(dx, dy)
    dmax = (l1 + l2) * 0.9995
    clipped = d > dmax
    d = min(max(d, abs(l1 - l2) + 1e-6), dmax)
    base = math.atan2(dy, dx)
    c = max(-1, min(1, (l1 * l1 + d * d - l2 * l2) / (2 * l1 * d)))
    t1 = base + bend * math.acos(c)
    j = (p0[0] + l1 * math.cos(t1), p0[1] + l1 * math.sin(t1))
    t2 = math.atan2(target[1] - j[1], target[0] - j[0])
    return t1, t2, clipped


def rest_angle(a, b):
    return next(x for x in a["rig"]["bones"] if x["id"] == b)["angleDeg"]


def set_world(a, off, W, bid, theta):
    """set offset of bone so that its world angle (rad) is theta; W holds world matrices of the parents (parent must be final)"""
    bn = next(x for x in a["rig"]["bones"] if x["id"] == bid)
    pw = math.degrees(wangle(W, bn["parent"]))
    ang = math.degrees(theta) - pw - bn["angleDeg"]
    ang = (ang + 180) % 360 - 180
    off.setdefault(bid, {})["angleDeg"] = round(ang, 4)


def solve_chain(a, off, root, b1, b2, target, bend):
    """2-link IK for bones b1 (origin at its joint) and b2 whose far end is at target; offsets updated; returns clipped flag"""
    W = Wof(a, off, root)
    p0 = (W[b1][4], W[b1][5])
    l1 = next(x for x in a["rig"]["bones"] if x["id"] == b2)["x"]
    l2 = next(x for x in a["rig"]["bones"] if x["id"] == b2)["lengthM"]
    t1, t2, clipped = ik_two(p0, l1, l2, target, bend)
    set_world(a, off, W, b1, t1)
    W = Wof(a, off, root)
    set_world(a, off, W, b2, t2)
    return clipped


def pose_clean(off):
    return {k: {kk: round(vv, 4) for kk, vv in v.items()} for k, v in off.items() if v}


JOINT_LIMITS = {}


def joint_issues(a, bones):
    out = []
    for b in a["rig"]["bones"]:
        o = bones.get(b["id"], {})
        j = b["joint"]
        if j["type"] == "revolute":
            v = o.get("angleDeg", 0)
            if "minDeg" in j and v < j["minDeg"] - 1e-6:
                out.append((b["id"], v, j["minDeg"], j["maxDeg"]))
            if "maxDeg" in j and v > j["maxDeg"] + 1e-6:
                out.append((b["id"], v, j["minDeg"], j["maxDeg"]))
        elif j["type"] == "fixed":
            if o:
                out.append((b["id"], "fixed bone offset", o))
        elif j["type"] == "prismatic":
            pass
    return out


# ------------------------------------------------------------------------------------------------ gait
def smooth(x):
    x = max(0.0, min(1.0, x))
    return x * x * x * (x * (6 * x - 15) + 10)


def interp(keys, u):
    """piecewise smooth interpolation of (u, value) keys, cyclic over [0,1]"""
    ks = sorted(keys)
    if u < ks[0][0]:
        u += 1
    ks2 = ks + [(ks[0][0] + 1, ks[0][1])]
    for (u0, v0), (u1, v1) in zip(ks2, ks2[1:]):
        if u0 <= u <= u1:
            return v0 + (v1 - v0) * smooth((u - u0) / (u1 - u0))
    return ks[0][1]


def rot_pt(phi, p):
    c, s = math.cos(phi), math.sin(phi)
    return (c * p[0] - s * p[1], s * p[0] + c * p[1])


def yaw_sample(f, N):
    return [f(i / N) for i in range(N)]


def frames_sum(keys):
    return keys


# ------------------------------------------------------------------------------------------------ thermal helper additions
def met_to_w(met, skin):
    return met * 58.15 * skin


def mifflin(sex, kg, cm, age):
    k = 10 * kg + 6.25 * cm - 5 * age + (5 if sex == "m" else -161)
    return round(k * 4184 / 86400, 1)


def binding(regions, emitters, met, breathing, exposed=None, sun=0.0, contact=None, wet=None, operating=None, extra=None):
    regs = {}
    for r in regions:
        rid = r["id"]
        b = {"exposedFraction": (exposed or {}).get(rid, 1.0), "sunlitFraction": (sun if not isinstance(sun, dict) else sun.get(rid, 0.0))}
        if contact and rid in contact:
            b["contact"] = contact[rid]
        if wet and rid in wet:
            b["wettedness"] = wet[rid]
        regs[rid] = b
    t = {"activityMet": met, "breathing": breathing, "regions": regs, "emitters": dict(emitters)}
    if operating:
        t["operating"] = dict(operating)
    if extra:
        t.update(extra)
    return t


def clip_events(ev):
    return [{"t": t, "kind": k, "title": T(en, de), **({"target": tg} if tg else {})} for t, k, en, de, tg in ev]


# ------------------------------------------------------------------------------------------------ render helpers (sketches)
def frame_view(W, pad=0.2, y0=-0.05, y1=None, x0=None, x1=None):
    return (x0 if x0 is not None else -0.8, y0, x1 if x1 is not None else 0.8, y1 if y1 is not None else 2.0)


def pens_of(rep):
    return {k: v for k, v in rep["pens"].items() if v is not None}


def sketch(asset, pose_id, scale, out, px_per_mm=None, view=None, root=None, bones=None, clip_sample=None):
    pose = next(p for p in asset["poses"] if p["id"] == pose_id) if pose_id else None
    if bones is not None:
        pose = {"bones": bones, "root": root or {}}
    rep = asset["representations"][str(scale)]
    cts = pose.get("contacts") if pose and "contacts" in pose else [x["id"] for x in asset["anchors"] if x["kind"] == "contact"]
    p2 = copy.deepcopy(pose)
    if cts:
        try:
            gy = ground_y(asset, p2["bones"], cts)
            p2["root"] = dict(p2.get("root") or {}, y=gy)
        except Exception:
            pass
    W = fk(asset, p2)
    pen = {k: v for k, v in rep["pens"].items() if v is not None}
    shapes = [s for s in rep["geometry"]["elevation"]]
    render(shapes, W, out, scale, px_per_mm=px_per_mm or 40, view=view or (-0.9, -0.05, 0.9, 2.05), pen_by_role=pen, grid=0.1 if scale <= 100 else 0.5)
    return W


# ------------------------------------------------------------------------------------------------ pose builder (IK for legs and arms)
def make_pose(a, spec, ypelvis=None):
    """spec: pelvis/torso/neck/head angle offsets; legs {side: dict(x=ankle x relative to hip, dy=ankle lift above flat, pitch=deg toes-up, flex=toe flex deg)};
    arms {side: dict(wrist=(x,y world rel. to hip x, absolute y) | angles=(arm,fore,hand))}; extra: {bone: offset dict}. Returns (offsets, root, info)."""
    off = {}
    for b in ("pelvis", "torso", "neck", "head"):
        if b in spec:
            off[b] = {"angleDeg": spec[b]}
    for k, v in spec.get("extra", {}).items():
        off.setdefault(k, {}).update(v)
    rootx = spec.get("rootx", 0.0)
    ankleH = [b for b in a["rig"]["bones"] if b["id"] == "foot-l"][0]
    sF_ankle = 0.075 * (ankleH["lengthM"] / 0.1858)
    foot = {s: next(b for b in a["rig"]["bones"] if b["id"] == f"foot-{s}") for s in "lr"}
    alpha = math.radians(90 - foot["l"]["angleDeg"]) if False else math.radians(foot["l"]["angleDeg"] - 90) * -1
    l1 = next(b for b in a["rig"]["bones"] if b["id"] == "thigh-l")["lengthM"]
    l2 = next(b for b in a["rig"]["bones"] if b["id"] == "shank-l")["lengthM"]
    hip_y0 = next(b for b in a["rig"]["bones"] if b["id"] == "pelvis")["y"]
    if ypelvis is None:
        sup = spec.get("support", "r")
        lg = spec["legs"][sup]
        dx = lg["x"]
        ypelvis = sF_ankle + lg.get("dy", 0) + math.sqrt((spec.get("reach", 0.995) * (l1 + l2)) ** 2 - dx ** 2)
    root = {"x": rootx, "y": ypelvis - hip_y0}
    info = {"clipped": []}
    W = Wof(a, off, root)
    for s in "lr":
        lg = spec["legs"][s]
        hip = (W[f"thigh-{s}"][4], W[f"thigh-{s}"][5])
        ankle = (hip[0] + lg["x"], sF_ankle + lg.get("dy", 0))
        info["clipped"].append(solve_chain(a, off, root, f"thigh-{s}", f"shank-{s}", ankle, +1))
        W = Wof(a, off, root)
        set_world(a, off, W, f"foot-{s}", -alpha + math.radians(lg.get("pitch", 0)))
        W = Wof(a, off, root)
        off.setdefault(f"toe-{s}", {})["angleDeg"] = lg.get("flex", 0)
        W = Wof(a, off, root)
    for s in "lr":
        ar = spec.get("arms", {}).get(s)
        if not ar:
            continue
        if "wrist" in ar:
            wx, wy = ar["wrist"]
            tgt = (a["rig"]["bones"][0]["x"] + rootx + wx, wy)
            info["clipped"].append(solve_chain(a, off, root, f"arm-{s}", f"forearm-{s}", tgt, ar.get("bend", -1)))
            W = Wof(a, off, root)
            if "hand" in ar:
                set_world(a, off, W, f"hand-{s}", math.radians(ar["hand"]))
        else:
            for bid, v in zip((f"arm-{s}", f"forearm-{s}", f"hand-{s}"), ar["angles"]):
                off.setdefault(bid, {})["angleDeg"] = v
        W = Wof(a, off, root)
    return pose_clean(off), root, info


def skeleton_png(a, bones, root, out, den=20, view=(-0.9, -0.05, 0.9, 2.05), extra_pts=(), contacts=None, px=30):
    """debug: stick figure + bone names + anchors"""
    r = dict(root); 
    if contacts:
        r["y"] = -min(anchor_world(a, Wof(a, bones, {}), c)[1] for c in contacts) if "y" not in r else r["y"]
    W = Wof(a, bones, r)
    x0, y0, x1, y1 = view
    k = px * 1000 / den
    w, h = (x1 - x0) * k, (y1 - y0) * k
    el = []
    for b in a["rig"]["bones"]:
        m = W[b["id"]]
        p0 = (m[4], m[5]); p1 = apply(m, (b["lengthM"], 0))
        el.append(f'<line x1="{p0[0]}" y1="{p0[1]}" x2="{p1[0]}" y2="{p1[1]}" stroke="#06c" stroke-width="{0.006}" />')
        el.append(f'<circle cx="{p0[0]}" cy="{p0[1]}" r="0.01" fill="#c00"/>')
    for an in a["anchors"]:
        p = apply(W[an["bone"]], (an["x"], an["y"]))
        el.append(f'<circle cx="{p[0]}" cy="{p[1]}" r="0.007" fill="#0a0"/>')
    for p in extra_pts:
        el.append(f'<circle cx="{p[0]}" cy="{p[1]}" r="0.012" fill="#f80"/>')
    el.append(f'<line x1="{x0}" y1="0" x2="{x1}" y2="0" stroke="#f55" stroke-width="0.004"/>')
    svg = (f'<svg xmlns="http://www.w3.org/2000/svg" width="{w:.0f}" height="{h:.0f}"><rect width="100%" height="100%" fill="white"/>'
           f'<g transform="scale({k},{-k}) translate({-x0},{-y1})">{"".join(el)}</g></svg>')
    p = out[:-4] + ".svg"
    open(p, "w").write(svg)
    subprocess.run(["rsvg-convert", "-o", out, p], check=True)
    os.remove(p)


def fb(front, back):
    """closed outline (s, f) from front rows (s, f>0) and back rows (s, b>0 backwards), s ascending in both"""
    return [(s, f) for s, f in front] + [(s, -b) for s, b in reversed(back)]


def mirror_head_free(pts):
    return pts


def scl(pts, k, kw=None):
    kw = kw or k
    return [(round(s * k, 4), round(f * kw, 4)) for s, f in pts]


def pose_ground(a, off, contacts):
    return ground_y(a, off, contacts)
