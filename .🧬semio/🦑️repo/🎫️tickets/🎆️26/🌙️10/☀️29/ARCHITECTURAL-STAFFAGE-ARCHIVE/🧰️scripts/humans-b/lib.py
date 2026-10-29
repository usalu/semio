"""Authoring aid for the humans-b staffage assets (scratch, ticket folder).

Everything physical and geometric is authored by hand as data in the asset scripts: width tables, paths, poses, clips. This module only
(1) converts intuitive profile coordinates (u forward, v up / d down the bone) to bone-local metres, (2) fits a Catmull-Rom spline through
hand-entered width tables, (3) mirrors/boilerplates (Text, mass fractions, DuBois area) and (4) offers a forward-kinematics mirror of the
engine so poses can be checked before the real check CLI runs. It never decides content.
"""
import json, math

# --------------------------------------------------------------------------------------------------------------- text and rounding
def T(en, de): return {"en": en, "de": de}
def r(x, n=4): return round(x + 0.0, n)

# --------------------------------------------------------------------------------------------------------------- references
REFS = {
    "de-leva-1996": {"id": "de-leva-1996", "citation": "de Leva, P. (1996). Adjustments to Zatsiorsky-Seluyanov's segment inertia parameters. Journal of Biomechanics 29(9), 1223-1230 (male segment mass fractions and centre-of-mass positions)."},
    "winter-2009": {"id": "winter-2009", "citation": "Winter, D. A. (2009). Biomechanics and Motor Control of Human Movement, 4th ed., Wiley. Table 4.1, segment lengths and masses."},
    "dubois-1916": {"id": "dubois-1916", "citation": "DuBois, D. and DuBois, E. F. (1916). A formula to estimate the approximate surface area if height and weight be known. Archives of Internal Medicine 17, 863-871."},
    "iso-9920-2007": {"id": "iso-9920-2007", "citation": "ISO 9920:2007 Ergonomics of the thermal environment, estimation of thermal insulation and water vapour resistance of a clothing ensemble."},
    "iso-8996-2021": {"id": "iso-8996-2021", "citation": "ISO 8996:2021 Ergonomics of the thermal environment, determination of metabolic rate (activity classes and met values)."},
    "ainsworth-2011": {"id": "ainsworth-2011", "citation": "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities. Medicine and Science in Sports and Exercise 43(8), 1575-1581."},
    "ashrae-2021": {"id": "ashrae-2021", "citation": "ASHRAE Handbook Fundamentals (2021), Chapter 9 Thermal Comfort (met unit 58.15 W/m2, respiration, skin wettedness)."},
    "en-1991-1-1": {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1, actions on structures, general actions (densities, self-weight, imposed loads)."},
    "en-1991-1-4": {"id": "en-1991-1-4", "citation": "EN 1991-1-4:2005 Eurocode 1, wind actions: F = q_p c_f A_ref with q_p = 0.5 rho v^2 (rho = 1.25 kg/m3)."},
    "anthropometry-din-33402": {"id": "anthropometry-din-33402", "citation": "DIN 33402-2:2020 Ergonomics, human body dimensions, values (stature, shoulder height, hand height)."},
    "niosh-1994": {"id": "niosh-1994", "citation": "Waters, T. R. et al. (1993/1994). Revised NIOSH equation for the design and evaluation of manual lifting tasks. Ergonomics 36(7), 749-776 (load constant 23 kg)."},
    "iso-11228-1": {"id": "iso-11228-1", "citation": "ISO 11228-1:2021 Ergonomics, manual handling, lifting, lowering and carrying."},
    "iso-20471": {"id": "iso-20471", "citation": "ISO 20471:2013 High-visibility clothing, test methods and requirements (class 2 retroreflective and fluorescent areas)."},
    "en-397": {"id": "en-397", "citation": "EN 397:2012 Industrial safety helmets, requirements (mass about 0.35-0.45 kg)."},
    "hoerner-1965": {"id": "hoerner-1965", "citation": "Hoerner, S. F. (1965). Fluid-Dynamic Drag. Hoerner Fluid Dynamics (drag coefficients: open hollow hemisphere facing the flow cd about 1.4)."},
    "en-1888": {"id": "en-1888", "citation": "EN 1888-1:2018 Child care articles, wheeled child conveyances, pushchairs and prams (typical mass 8-12 kg, handle height 0.95-1.1 m)."},
    "cavanagh-1982": {"id": "cavanagh-1982", "citation": "Cavanagh, P. R. and Lafortune, M. A. (1980). Ground reaction forces in distance running. Journal of Biomechanics 13(5), 397-406 (flight phase, stride length, peak vertical force)."},
    "winter-1983": {"id": "winter-1983", "citation": "Winter, D. A. (1983). Moments of force and mechanical power in jogging. Journal of Biomechanics 16(1), 91-97."},
    "mcmahon-1986": {"id": "mcmahon-1986", "citation": "Hoerner/McMahon-type running data: Dalleau, G. et al. (1998). The spring-mass model and the energy cost of treadmill running. Eur J Appl Physiol 77, 257-263 (running 2.5-3.3 m/s, step frequency about 2.7-3 Hz)."},
    "bohannon-1997": {"id": "bohannon-1997", "citation": "Bohannon, R. W. (1997). Comfortable and maximum walking speed of adults aged 20-79 years. Age and Ageing 26, 15-19."},
    "rain-wetting": {"id": "rain-wetting", "citation": "ISO 15496/ISO 11092 style water-vapour resistance of coated shell fabrics; here: ISO 9920 Table for raincoat over light clothing (0.9-1.2 clo)."},
}

def refs(*ids): return [REFS[i] for i in ids]

# --------------------------------------------------------------------------------------------------------------- anthropometry
def dubois_m2(height_m, mass_kg): return 0.007184 * mass_kg ** 0.425 * (height_m * 100) ** 0.725

# de Leva (1996), male: share of body mass per segment (one side for limbs)
DE_LEVA = {"pelvis": 0.1117, "torso": 0.3229, "head": 0.0694, "upper-arm": 0.0271, "forearm": 0.0162, "hand": 0.0061, "thigh": 0.1416, "shank": 0.0433, "foot": 0.0137}

def seg_mass(kind, total_kg):
    return r(DE_LEVA[kind] * total_kg, 3)

# --------------------------------------------------------------------------------------------------------------- profile -> local
# kind "up": bone points up in the neutral pose; profile (u forward, v up along the bone) -> local (x = v, y = -u)
# kind "down": bone points down; profile (u forward, d down along the bone) -> local (x = d, y = u)
# kind "fwd": bone points forward; profile (u along the bone, v up) -> local (x = u, y = v)
class Frame:
    def __init__(self, kind, s=1.0, wf=1.0):
        self.kind, self.s, self.wf = kind, s, wf
    def pt(self, a, b, wide=True):
        s = self.s
        if self.kind == "up": return (b * s, -a * s * (self.wf if wide else 1))
        if self.kind == "down": return (b * s, a * s * (self.wf if wide else 1))
        return (a * s, b * s)
    def lens(self, v): return v * self.s

def fmt(p): return f"{r(p[0])} {r(p[1])}"

def path_d(frame, cmds, wide=True):
    """cmds: ('M',a,b) ('L',a,b) ('C',a1,b1,a2,b2,a,b) ('Z',) in profile coordinates."""
    out = []
    for c in cmds:
        if c[0] == "Z": out.append("Z")
        elif c[0] in "ML": out.append(f"{c[0]} {fmt(frame.pt(c[1], c[2], wide))}")
        else: out.append("C " + " ".join(fmt(frame.pt(c[i], c[i + 1], wide)) for i in (1, 3, 5)))
    return " ".join(out)

def poly_pts(frame, pts, wide=True): return [[r(frame.pt(a, b, wide)[0]), r(frame.pt(a, b, wide)[1])] for a, b in pts]

# --------------------------------------------------------------------------------------------------------------- spline through width tables
def _interp(tab, d):
    """tab: [(d, front, back)], linear interpolation of front/back at d (clamped)."""
    if d <= tab[0][0]: return tab[0][1], tab[0][2]
    for (d0, f0, b0), (d1, f1, b1) in zip(tab, tab[1:]):
        if d <= d1:
            w = (d - d0) / (d1 - d0)
            return f0 + (f1 - f0) * w, b0 + (b1 - b0) * w
    return tab[-1][1], tab[-1][2]

def _cr(points, closed=False):
    """Catmull-Rom (centripetal-free, tension 0.5) through points -> list of cubic segments (c1, c2, p)."""
    n = len(points)
    segs = []
    for i in range(n - 1):
        p0 = points[i - 1] if i > 0 else points[i]
        p1, p2 = points[i], points[i + 1]
        p3 = points[i + 2] if i + 2 < n else points[i + 1]
        c1 = (p1[0] + (p2[0] - p0[0]) / 6, p1[1] + (p2[1] - p0[1]) / 6)
        c2 = (p2[0] - (p3[0] - p1[0]) / 6, p2[1] - (p3[1] - p1[1]) / 6)
        segs.append((c1, c2, p2))
    return segs

def table_cmds(tab, d0=None, d1=None, cap0=0.0, cap1=0.0, grow=0.0):
    """Closed outline (profile coords) of a limb table between d0 and d1; front runs d0->d1, back returns d1->d0.
    cap0/cap1 add a rounded end beyond d0/d1 (metres along the bone). grow offsets the outline outwards."""
    d0 = tab[0][0] if d0 is None else d0
    d1 = tab[-1][0] if d1 is None else d1
    ds = sorted({d0, d1, *[row[0] for row in tab if d0 < row[0] < d1]})
    front, back = [], []
    for d in ds:
        f, b = _interp(tab, d)
        front.append((f + grow, d))
        back.append((b - grow, d))
    if cap0:
        f, b = front[0][0], back[0][0]
        front.insert(0, (f * 0.45, d0 - cap0)); back.insert(0, (b * 0.45, d0 - cap0))
    if cap1:
        f, b = front[-1][0], back[-1][0]
        front.append((f * 0.6, d1 + cap1)); back.append((b * 0.6, d1 + cap1))
    cmds = [("M", front[0][0], front[0][1])]
    for c1, c2, p in _cr(front): cmds.append(("C", c1[0], c1[1], c2[0], c2[1], p[0], p[1]))
    cmds.append(("L", back[-1][0], back[-1][1]))
    rev = list(reversed(back))
    for c1, c2, p in _cr(rev): cmds.append(("C", c1[0], c1[1], c2[0], c2[1], p[0], p[1]))
    cmds.append(("Z",))
    return cmds

# --------------------------------------------------------------------------------------------------------------- shapes
def shape(id, bone, role, lod, z, tone, geom, region=None, material=None, filled=True):
    out = {"id": id, "bone": bone, "role": role, "lod": lod, "z": z, "filled": filled, "tone": tone}
    out.update(geom)
    if region: out["region"] = region
    if material: out["material"] = material
    return out

def g_path(frame, cmds, wide=True): return {"kind": "path", "d": path_d(frame, cmds, wide)}
def g_poly(frame, pts, closed, wide=True): return {"kind": "poly", "points": poly_pts(frame, pts, wide), "closed": closed}
def g_ellipse(frame, a, b, ra, rb, rot=0.0):
    cx, cy = frame.pt(a, b)
    if frame.kind == "up": rx, ry, rd = rb * frame.s, ra * frame.s * frame.wf, rot
    elif frame.kind == "down": rx, ry, rd = rb * frame.s, ra * frame.s * frame.wf, rot
    else: rx, ry, rd = ra * frame.s, rb * frame.s, rot
    g = {"kind": "ellipse", "cx": r(cx), "cy": r(cy), "rx": r(rx), "ry": r(ry)}
    if rot: g["rotationDeg"] = rot
    return g
def g_capsule(frame, a0, b0, a1, b1, r0, r1):
    p0, p1 = frame.pt(a0, b0), frame.pt(a1, b1)
    return {"kind": "capsule", "x0": r(p0[0]), "y0": r(p0[1]), "x1": r(p1[0]), "y1": r(p1[1]), "r0": r(r0 * frame.s), "r1": r(r1 * frame.s)}
def g_rect(x, y, w, h, rr=0.0):
    g = {"kind": "rect", "x": r(x), "y": r(y), "w": r(w), "h": r(h)}
    if rr: g["r"] = r(rr)
    return g

# --------------------------------------------------------------------------------------------------------------- tables (neutral 1.76 m, metres): (d or v, front u, back u)
UPPER_ARM = [(0.0, 0.047, -0.046), (0.08, 0.049, -0.045), (0.20, 0.041, -0.038), (0.31, 0.037, -0.034)]
FOREARM = [(0.0, 0.040, -0.038), (0.08, 0.043, -0.037), (0.18, 0.033, -0.029), (0.27, 0.027, -0.025)]
THIGH = [(0.0, 0.098, -0.108), (0.07, 0.098, -0.108), (0.17, 0.090, -0.097), (0.29, 0.074, -0.076), (0.38, 0.062, -0.062), (0.43, 0.058, -0.058)]
SHANK = [(0.0, 0.060, -0.060), (0.09, 0.054, -0.076), (0.20, 0.048, -0.070), (0.32, 0.041, -0.049), (0.42, 0.037, -0.039)]
TORSO = [(-0.06, 0.108, -0.106), (0.04, 0.119, -0.108), (0.14, 0.125, -0.114), (0.24, 0.131, -0.122), (0.32, 0.137, -0.130), (0.40, 0.118, -0.128), (0.44, 0.084, -0.100), (0.46, 0.058, -0.066)]
PELVIS = [(-0.095, 0.092, -0.110), (-0.04, 0.118, -0.135), (0.03, 0.122, -0.128), (0.10, 0.116, -0.110)]

# --------------------------------------------------------------------------------------------------------------- forward kinematics (mirror of the engine)
def placement(x, y, deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return (c, s, -s, c, x, y)
def compose(p, l):
    return (p[0] * l[0] + p[2] * l[1], p[1] * l[0] + p[3] * l[1], p[0] * l[2] + p[2] * l[3], p[1] * l[2] + p[3] * l[3], p[0] * l[4] + p[2] * l[5] + p[4], p[1] * l[4] + p[3] * l[5] + p[5])
def apply(m, x, y): return (m[0] * x + m[2] * y + m[4], m[1] * x + m[3] * y + m[5])

class Rig:
    def __init__(self, bones, anchors):
        self.bones = {b["id"]: b for b in bones}
        self.order = [b["id"] for b in bones]
        self.anchors = {a["id"]: a for a in anchors}
    def solve(self, offsets, root=(0, 0, 0)):
        world = {}
        for id in self._ordered():
            b = self.bones[id]
            o = offsets.get(id, {})
            j = b["joint"]
            ang, dx, dy = 0.0, 0.0, 0.0
            if j["type"] == "revolute":
                a = o.get("angleDeg", 0)
                lo, hi = j.get("minDeg", -1e9), j.get("maxDeg", 1e9)
                ang = min(max(a, lo), hi)
            elif j["type"] == "prismatic":
                ax = math.radians(j.get("axisDeg", 0))
                sv = o.get("dx", 0) * math.cos(ax) + o.get("dy", 0) * math.sin(ax)
                sv = min(max(sv, j.get("minM", -1e9)), j.get("maxM", 1e9))
                dx, dy = sv * math.cos(ax), sv * math.sin(ax)
            local = placement(b["x"] + dx, b["y"] + dy, b["angleDeg"] + ang)
            parent = placement(*root) if b["parent"] is None else world[b["parent"]]
            world[id] = compose(parent, local)
        return world
    def _ordered(self):
        seen, out = set(), []
        def visit(i):
            if i in seen: return
            p = self.bones[i]["parent"]
            if p is not None: visit(p)
            seen.add(i); out.append(i)
        for i in self.order: visit(i)
        return out
    def anchor(self, world, id):
        a = self.anchors[id]
        return apply(world[a["bone"]], a["x"], a["y"])
    def angle(self, world, bone): return math.degrees(math.atan2(world[bone][1], world[bone][0]))
    def origin(self, world, bone): return (world[bone][4], world[bone][5])

def norm(a):
    while a > 180: a -= 360
    while a <= -180: a += 360
    return a

def two_link(p0, target, l1, l2, bend):
    """World directions (deg) of two segments from p0 reaching target; bend=+1 or -1 picks the knee/elbow side."""
    dx, dy = target[0] - p0[0], target[1] - p0[1]
    d = math.hypot(dx, dy)
    d = min(d, l1 + l2 - 1e-6)
    base = math.atan2(dy, dx)
    a = math.acos(max(-1, min(1, (l1 * l1 + d * d - l2 * l2) / (2 * l1 * d))))
    th1 = base + bend * a
    p1 = (p0[0] + l1 * math.cos(th1), p0[1] + l1 * math.sin(th1))
    th2 = math.atan2(target[1] - p1[1], target[0] - p1[0])
    return math.degrees(th1), math.degrees(th2)

def jd(obj):
    return json.dumps(obj, ensure_ascii=False, separators=(",", ":"))

def root_y(rig, offsets, contacts, root_angle=0.0, root_x=0.0):
    w = rig.solve(offsets, (root_x, 0.0, root_angle))
    return -min(rig.anchor(w, c)[1] for c in contacts)

def report(rig, offsets, contacts, root_angle=0.0, names=("pelvis", "torso", "head", "upper-arm-r", "forearm-r", "hand-r", "thigh-r", "shank-r", "foot-r")):
    y = root_y(rig, offsets, contacts, root_angle)
    w = rig.solve(offsets, (0, y, root_angle))
    print(f"root y {y:.3f}")
    for n in names:
        o = rig.origin(w, n); print(f"  {n:12s} origin ({o[0]:.3f},{o[1]:.3f}) angle {rig.angle(w, n):7.1f}")
    for a in rig.anchors:
        if a.startswith(("contact", "grip", "mouth")):
            p = rig.anchor(w, a); print(f"  anchor {a:10s} ({p[0]:.3f},{p[1]:.3f})")
    return y, w

def arm_offsets(rig, w, side, wrist_target, hand_world_deg, bend=-1):
    """Offsets of upper-arm/forearm/hand so the wrist reaches wrist_target (world). w: world solved with torso/pelvis already set."""
    sh = rig.origin(w, f"upper-arm-{side}")
    l1, l2 = rig.bones[f"forearm-{side}"]["x"], rig.bones[f"hand-{side}"]["x"]
    th1, th2 = two_link(sh, wrist_target, l1, l2, bend)
    tors = rig.angle(w, "torso")
    o1 = norm(th1 - tors + 180)
    o2 = norm(th2 - th1)
    if -1.0 < o2 < 0: o2 = 0.0
    o3 = norm(hand_world_deg - th2)
    return {f"upper-arm-{side}": {"angleDeg": r(o1, 2)}, f"forearm-{side}": {"angleDeg": r(o2, 2)}, f"hand-{side}": {"angleDeg": r(o3, 2)}}

ENV_STD = {"airTempC": 20, "meanRadiantTempC": 20, "relativeHumidity": 0.5, "windMS": 0.2, "windDirectionDeg": 180, "solarWM2": 0, "pressurePa": 101325}

def material(id, en, de, emis, src="literature", ref=None, solar=None, rho=None, k=None):
    m = {"id": id, "title": T(en, de), "emissivity": emis}
    if solar is not None: m["solarAbsorptivity"] = solar
    if rho is not None: m["densityKgM3"] = rho
    if k is not None: m["conductivityWmK"] = k
    m["source"] = src
    if ref: m["reference"] = ref
    return m

def region(id, en, de, kind, mat, area, lc, model, core=None, ins=None, share=None, src="literature", ref="iso-9920-2007", wet=None, surf=None):
    d = {"id": id, "title": T(en, de), "kind": kind, "material": mat, "model": model, "areaM2": r(area, 3), "characteristicLengthM": lc}
    if surf is not None: d["surfaceTempC"] = surf
    if core is not None: d["coreTempC"] = core
    if ins is not None: d["insulationM2KW"] = ins
    if wet is not None: d["wettedness"] = wet
    if share is not None: d["metabolicShare"] = share
    d["source"] = src
    d["reference"] = ref
    return d

def emitter(id, anchor, period, vel=1.5, diam=0.03, temp=34, vap=0.034, dfrom=0.5, dto=1.0, dirdeg=-90):
    return {"id": id, "kind": "breath", "title": T("Exhaled breath", "Ausgeatmete Luft"), "anchor": anchor, "directionDeg": dirdeg, "carries": "vapour", "temperatureC": temp, "velocityMS": vel, "diameterM": diam, "vapourKgPerKg": vap, "periodS": period, "dutyFrom": dfrom, "dutyTo": dto, "source": "authored-estimate", "reference": "ashrae-2021"}

def pose(id, en, de, den, dde, bones, contacts, met, emit, loads=None, ground="auto", root=None, regions=None, key=False):
    th = {"activityMet": met, "emitters": emit}
    if regions: th["regions"] = regions
    p = {"id": id, "title": T(en, de), "description": T(den, dde), "bones": bones, "ground": ground, "contacts": contacts, "thermal": th, "loads": loads or []}
    if root: p["root"] = root
    if key: p["key"] = True
    return p

def key(t, pose, ease="smooth", bones=None):
    k = {"t": t, "pose": pose, "ease": ease}
    if bones: k["bones"] = bones
    return k

def track(target, keys, ease="smooth"): return {"target": target, "ease": ease, "keys": [{"t": t, "v": v} for t, v in keys]}
def ev(t, kind, en, de, target=None):
    e = {"t": t, "kind": kind, "title": T(en, de)}
    if target: e["target"] = target
    return e
def phase(id, a, b, en, de, ballistic=False):
    p = {"id": id, "fromS": a, "toS": b, "title": T(en, de)}
    if ballistic: p["ballistic"] = True
    return p
def span(anchor, a, b): return {"anchor": anchor, "fromS": a, "toS": b}
def clip(id, en, de, den, dde, dur, loop, root, keys, contacts, phases, events, tracks, advance=None, fps=30):
    c = {"id": id, "title": T(en, de), "description": T(den, dde), "durationS": dur, "fps": fps, "loop": loop, "rootMotion": root}
    if advance is not None: c["advanceM"] = advance
    c.update({"keys": keys, "contacts": contacts, "phases": phases, "events": events, "tracks": tracks})
    return c

def merge_offsets(*ds):
    out = {}
    for d in ds:
        for k, v in d.items(): out[k] = {**out.get(k, {}), **v}
    return out

def legs(th_l, kn_l, ft_l, th_r, kn_r, ft_r):
    return {"thigh-l": {"angleDeg": th_l}, "shank-l": {"angleDeg": kn_l}, "foot-l": {"angleDeg": ft_l}, "thigh-r": {"angleDeg": th_r}, "shank-r": {"angleDeg": kn_r}, "foot-r": {"angleDeg": ft_r}}

def assemble(id, title, summary, tags, ref_ids, notes, dims, mats, bones, anchors, elev, poses, clips, regions, emitters, thermal_notes, structure_notes, default_pose, default_clip, base_met=1.5, dynamics="inertial", env=None, plan=None):
    g = {"elevation": elev}
    if plan: g["plan"] = plan
    return {"id": id, "category": "humans", "title": title, "summary": summary, "tags": tags,
            "provenance": {"author": "Semio staffage humans-b content worker", "method": "handcrafted", "created": "2026-10-29", "references": refs(*ref_ids), "notes": notes},
            "dimensions": dims, "materials": mats, "rig": {"kind": "anatomical", "root": "pelvis", "bones": bones}, "anchors": anchors, "geometry": g, "poses": poses, "clips": clips,
            "thermal": {"environment": env or ENV_STD, "baseMet": base_met, "regions": regions, "emitters": emitters, "assumptions": thermal_notes},
            "structure": {"gravityMS2": 9.80665, "dynamics": dynamics, "supports": [], "assumptions": structure_notes, "clinical": False},
            "presentation": {"defaultPose": default_pose, "defaultClip": default_clip, "viewpoints": ["elevation"]},
            "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"], "completion": "authored"}

def std_materials():
    return [material("skin", "Skin", "Haut", 0.98, "literature", "ashrae-2021", 0.6),
            material("cotton-clothing", "Cotton clothing", "Baumwollkleidung", 0.95, "literature", "iso-9920-2007", 0.5),
            material("leather-shoe", "Shoe", "Schuh", 0.90, "authored-estimate", None, 0.7),
            material("hair", "Hair", "Haar", 0.95, "authored-estimate", None, 0.85)]

def com_x(rig, offsets, root):
    w = rig.solve(offsets, root)
    m = x = y = 0.0
    for id, b in rig.bones.items():
        ms = b.get("mass")
        if ms:
            px, py = apply(w[id], ms["comX"], ms["comY"])
            m += ms["kg"]; x += px * ms["kg"]; y += py * ms["kg"]
    return x / m, y / m, m
