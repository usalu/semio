"""NW-A2 scratch library: forward/inverse kinematics, hand-point splines, local<->world conversion, JSON builders and a quick rsvg sketch
renderer. Coordinates helpers only: every outline, pose, number and text is authored by hand in the per-asset scripts."""
import json, math, os, re, subprocess

S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage"
T = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE"
GEN = T + "/🗑️generated/NW-A2"
CAT = S + "/🗂️catalog/🐕️animals"
os.makedirs(GEN, exist_ok=True)


def tx(en, de):
    return {"en": en, "de": de}


# ------------------------------------------------------------------ kinematics (mirrors 🦴️rig)
def mul(a, b):
    return (a[0]*b[0]+a[2]*b[1], a[1]*b[0]+a[3]*b[1], a[0]*b[2]+a[2]*b[3], a[1]*b[2]+a[3]*b[3],
            a[0]*b[4]+a[2]*b[5]+a[4], a[1]*b[4]+a[3]*b[5]+a[5])


def rot(deg, x=0.0, y=0.0):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return (c, s, -s, c, x, y)


def apply(m, p):
    return (m[0]*p[0]+m[2]*p[1]+m[4], m[1]*p[0]+m[3]*p[1]+m[5])


def inv(m):
    d = m[0]*m[3]-m[1]*m[2]
    a, b, c, dd = m[3]/d, -m[1]/d, -m[2]/d, m[0]/d
    return (a, b, c, dd, -(a*m[4]+c*m[5]), -(b*m[4]+dd*m[5]))


def clampv(v, lo, hi):
    if lo is not None: v = max(v, lo)
    if hi is not None: v = min(v, hi)
    return v


def fk(asset, offsets=None, root=None):
    offsets = offsets or {}
    root = root or {}
    W = {}
    for b in asset["rig"]["bones"]:
        o = offsets.get(b["id"], {})
        j = b["joint"]
        a = o.get("angleDeg", 0) if j["type"] == "revolute" else 0
        if j["type"] == "revolute": a = clampv(a, j.get("minDeg"), j.get("maxDeg"))
        if b["parent"] is None:
            W[b["id"]] = rot(b["angleDeg"] + a + root.get("angleDeg", 0), b["x"] + root.get("x", 0), b["y"] + root.get("y", 0))
        else:
            W[b["id"]] = mul(W[b["parent"]], rot(b["angleDeg"] + a, b["x"], b["y"]))
    return W


def anchor_world(asset, W, aid):
    a = next(a for a in asset["anchors"] if a["id"] == aid)
    return apply(W[a["bone"]], (a["x"], a["y"]))


def pose_world(asset, pose, ground=True):
    """world matrices of a pose with the engine's automatic ground (lowest listed contact on y = 0)"""
    off = pose["bones"]
    r = dict(pose.get("root") or {})
    W = fk(asset, off, {"x": r.get("x", 0), "y": 0, "angleDeg": r.get("angleDeg", 0)})
    if pose["ground"] == "auto" and pose["contacts"]:
        low = min(anchor_world(asset, W, c)[1] for c in pose["contacts"])
        y = -low
    else:
        y = r.get("y", 0)
    return fk(asset, off, {"x": r.get("x", 0), "y": y, "angleDeg": r.get("angleDeg", 0)})


def world_angle(m):
    return math.degrees(math.atan2(m[1], m[0]))


def to_local(W, bone, pts):
    mi = inv(W[bone])
    return [apply(mi, p) for p in pts]


# ------------------------------------------------------------------ IK helpers
def two_link(h, a, l1, l2, bend):
    """world angles (deg) of two links from hip h to ankle a; bend = +1 knee on the left of h->a, -1 on the right"""
    dx, dy = a[0]-h[0], a[1]-h[1]
    d = math.hypot(dx, dy)
    d = min(d, l1+l2-1e-6)
    d = max(d, abs(l1-l2)+1e-6)
    base = math.atan2(dy, dx)
    c1 = (l1*l1 + d*d - l2*l2) / (2*l1*d)
    a1 = math.acos(max(-1, min(1, c1)))
    t1 = base + bend*a1
    k = (h[0]+l1*math.cos(t1), h[1]+l1*math.sin(t1))
    t2 = math.atan2(a[1]-k[1], a[0]-k[0])
    return math.degrees(t1), math.degrees(t2)


def offsets_from_world(asset, parent_world_deg, chain, world_degs):
    """bone offsets (deg) for a chain [bone ids] given its world angles and the world angle of the chain's parent"""
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    out = {}
    prev = parent_world_deg
    for bid, wd in zip(chain, world_degs):
        out[bid] = {"angleDeg": round(norm_deg(wd - prev - bones[bid]["angleDeg"]), 2)}
        prev = wd
    return out


def norm_deg(d):
    while d > 180: d -= 360
    while d < -180: d += 360
    return d


# ------------------------------------------------------------------ curves
def f(v):
    return f"{v:.4f}".rstrip("0").rstrip(".") if abs(v) > 5e-5 else "0"


def spline_path(pts, closed=True, k=1/6, corners=()):
    """Catmull-Rom (tension k) through hand-entered points -> cubic path; corners = indices that stay sharp"""
    n = len(pts)
    d = f"M {f(pts[0][0])} {f(pts[0][1])}"
    segs = n if closed else n-1
    for i in range(segs):
        p0 = pts[(i-1) % n] if (closed or i > 0) else pts[i]
        p1 = pts[i]
        p2 = pts[(i+1) % n]
        p3 = pts[(i+2) % n] if (closed or i+2 < n) else p2
        c1 = (p1[0]+(p2[0]-p0[0])*k, p1[1]+(p2[1]-p0[1])*k)
        c2 = (p2[0]-(p3[0]-p1[0])*k, p2[1]-(p3[1]-p1[1])*k)
        if i in corners: c1 = p1
        if (i+1) % n in corners: c2 = p2
        d += f" C {f(c1[0])} {f(c1[1])} {f(c2[0])} {f(c2[1])} {f(p2[0])} {f(p2[1])}"
    return d + (" Z" if closed else "")


def ell_pts(cx, cy, rx, ry, rotdeg=0, n=8, start=0):
    c, s = math.cos(math.radians(rotdeg)), math.sin(math.radians(rotdeg))
    out = []
    for i in range(n):
        t = math.radians(start) + 2*math.pi*i/n
        x, y = rx*math.cos(t), ry*math.sin(t)
        out.append((cx + c*x - s*y, cy + s*x + c*y))
    return out


def mirror_x(pts, x0=0.0):
    return [(2*x0 - x, y) for x, y in pts]


class Draw:
    """Collect shapes of one representation; points are given in WORLD metres of the default pose and bound to a bone."""

    def __init__(self, W, local=False):
        self.W = W
        self.local = local
        self.shapes = []

    def _loc(self, bone, pts):
        return [tuple(p) for p in pts] if self.local else to_local(self.W, bone, pts)

    def _add(self, id, bone, role, z, filled, tone, geom, region, regions, material, hatch, pen):
        s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": round(tone, 2)}
        s.update(geom)
        if region: s["region"] = region
        if regions: s["regions"] = regions
        if material: s["material"] = material
        if hatch: s["hatch"] = hatch
        if pen: s["penMm"] = pen
        self.shapes.append(s)
        return s

    def curve(self, id, bone, pts, z, tone=0.5, role="body", closed=True, filled=True, region=None, regions=None, material=None, hatch=None, pen=None, k=1/6, corners=()):
        loc = self._loc(bone, pts)
        return self._add(id, bone, role, z, filled and closed, tone, {"kind": "path", "d": spline_path(loc, closed, k, corners)}, region, regions, material, hatch, pen)

    def poly(self, id, bone, pts, z, tone=0.5, role="body", closed=True, filled=True, region=None, regions=None, material=None, hatch=None, pen=None):
        loc = self._loc(bone, pts)
        return self._add(id, bone, role, z, filled and closed, tone, {"kind": "poly", "points": [[round(x, 4), round(y, 4)] for x, y in loc], "closed": closed}, region, regions, material, hatch, pen)

    def line(self, id, bone, pts, z, tone=0.4, role="detail", region=None, material=None, pen=None, smooth=False, regions=None):
        if smooth:
            return self.curve(id, bone, pts, z, tone, role, closed=False, filled=False, region=region, regions=regions, material=material, pen=pen)
        return self.poly(id, bone, pts, z, tone, role, closed=False, filled=False, region=region, regions=regions, material=material, pen=pen)

    def ellipse(self, id, bone, cx, cy, rx, ry, z, tone=0.5, rot_deg=0, role="body", region=None, regions=None, material=None, pen=None, filled=True):
        loc = self._loc(bone, [(cx, cy)])[0]
        wa = 0 if self.local else world_angle(self.W[bone])
        if self.local: rot_deg = 0 + rot_deg
        g = {"kind": "ellipse", "cx": round(loc[0], 4), "cy": round(loc[1], 4), "rx": rx, "ry": ry}
        r = rot_deg - wa
        if abs(norm_deg(r)) > 0.05: g["rotationDeg"] = round(norm_deg(r), 2)
        return self._add(id, bone, role, z, filled, tone, g, region, regions, material, None, pen)


# ------------------------------------------------------------------ rendering (rsvg sketches)
def svg_shape(s, W, pen_m, mm_m, hatches):
    m = W[s["bone"]]
    tr = f'transform="matrix({m[0]:.6g},{m[1]:.6g},{m[2]:.6g},{m[3]:.6g},{m[4]:.6g},{m[5]:.6g})"'
    k = s["kind"]
    pm = (s.get("penMm") and s["penMm"] * mm_m) or pen_m
    tone = int(255*(1-s["tone"]))
    fill = f"rgb({tone},{tone},{tone})" if s["filled"] else "none"
    st = f'fill="{fill}" stroke="black" stroke-width="{pm:.6g}" stroke-linejoin="round" stroke-linecap="round"'
    if k == "path":
        return f'<path {tr} {st} d="{s["d"]}"/>'
    if k == "poly":
        pts = " ".join(f"{x},{y}" for x, y in s["points"])
        return f'<{"polygon" if s["closed"] else "polyline"} {tr} {st} points="{pts}"/>'
    if k == "ellipse":
        r = s.get("rotationDeg", 0)
        return f'<g {tr}><ellipse {st} cx="{s["cx"]}" cy="{s["cy"]}" rx="{s["rx"]}" ry="{s["ry"]}" transform="rotate({r} {s["cx"]} {s["cy"]})"/></g>'
    if k == "rect":
        return f'<rect {tr} {st} x="{s["x"]}" y="{s["y"]}" width="{s["w"]}" height="{s["h"]}" rx="{s.get("r", 0)}"/>'
    if k == "capsule":
        x0, y0, x1, y1, r0, r1 = s["x0"], s["y0"], s["x1"], s["y1"], s["r0"], s["r1"]
        a = math.atan2(y1-y0, x1-x0)
        nx, ny = -math.sin(a), math.cos(a)
        d = (f"M {x0+nx*r0} {y0+ny*r0} L {x1+nx*r1} {y1+ny*r1} A {r1} {r1} 0 0 0 {x1-nx*r1} {y1-ny*r1} "
             f"L {x0-nx*r0} {y0-ny*r0} A {r0} {r0} 0 0 0 {x0+nx*r0} {y0+ny*r0} Z")
        return f'<path {tr} {st} d="{d}"/>'
    raise ValueError(k)


def render(asset, rep_key, pose_id, out_png, view, px_per_mm=40, bones=False, den=None, grid=0.05, W=None, sym=None):
    rep = asset["representations"][rep_key]
    den = den or int(rep_key)
    pose = next(p for p in asset["poses"] if p["id"] == pose_id)
    W = W or pose_world(asset, pose)
    x0, y0, x1, y1 = view
    ppm = 1000/den*px_per_mm
    w, h = (x1-x0)*ppm, (y1-y0)*ppm
    body = []
    pens = rep["pens"]
    for s in sorted(rep["geometry"]["elevation"], key=lambda s: s["z"]):
        pen = s.get("penMm") or pens.get(s["role"])
        if pen is None:
            continue
        body.append(svg_shape(s, W, pen*den/1000, den/1000, rep["hatches"]))
    g = ""
    gx = math.floor(x0/grid)*grid
    lw = 0.4/ppm
    while gx <= x1:
        g += f'<line x1="{gx:.4f}" y1="{y0}" x2="{gx:.4f}" y2="{y1}" stroke="#e4e4e4" stroke-width="{lw}"/>'
        gx += grid
    gy = math.floor(y0/grid)*grid
    while gy <= y1:
        g += f'<line x1="{x0}" y1="{gy:.4f}" x2="{x1}" y2="{gy:.4f}" stroke="{"#f66" if abs(gy) < 1e-6 else "#e4e4e4"}" stroke-width="{lw}"/>'
        gy += grid
    extra = ""
    if bones:
        for b in asset["rig"]["bones"]:
            m = W[b["id"]]
            p0 = apply(m, (0, 0)); p1 = apply(m, (b["lengthM"], 0))
            extra += f'<line x1="{p0[0]}" y1="{p0[1]}" x2="{p1[0]}" y2="{p1[1]}" stroke="#08f" stroke-width="{1.2/ppm}"/><circle cx="{p0[0]}" cy="{p0[1]}" r="{2/ppm}" fill="#f80"/>'
        for a in asset["anchors"]:
            p = apply(W[a["bone"]], (a["x"], a["y"]))
            extra += f'<circle cx="{p[0]}" cy="{p[1]}" r="{2.5/ppm}" fill="none" stroke="#0a0" stroke-width="{1/ppm}"/>'
    svg = (f'<svg xmlns="http://www.w3.org/2000/svg" width="{w:.0f}" height="{h:.0f}" viewBox="0 0 {w:.0f} {h:.0f}">'
           f'<rect width="100%" height="100%" fill="white"/>'
           f'<g transform="scale({ppm},{-ppm}) translate({-x0},{-y1})">{g}{"".join(body)}{extra}</g></svg>')
    p = out_png[:-4] + ".svg"
    open(p, "w").write(svg)
    subprocess.run(["rsvg-convert", "-o", out_png, p], check=True)
    os.remove(p)


def bounds_world(asset, rep_key, pose_id):
    """extent of the flattened shapes (curves sampled by control points; sufficient for sketches)"""
    rep = asset["representations"][rep_key]
    pose = next(p for p in asset["poses"] if p["id"] == pose_id)
    W = pose_world(asset, pose)
    xs, ys = [], []
    for s in rep["geometry"]["elevation"]:
        if s["role"] == "ground": continue
        pts = []
        k = s["kind"]
        if k == "path":
            nums = [float(v) for v in re.findall(r"-?\d+\.?\d*(?:e-?\d+)?", s["d"])]
            pts = [(nums[i], nums[i+1]) for i in range(0, len(nums)-1, 2)]
        elif k == "poly":
            pts = [tuple(p) for p in s["points"]]
        elif k == "ellipse":
            pts = ell_pts(s["cx"], s["cy"], s["rx"], s["ry"], s.get("rotationDeg", 0), 36)
        for p in pts:
            q = apply(W[s["bone"]], p)
            xs.append(q[0]); ys.append(q[1])
    return (min(xs), min(ys), max(xs), max(ys))


# ------------------------------------------------------------------ builders
def bone(id, parent, x, y, ang, L, lim, title, kg=None, com=None, dist="uniform", ref=None, joint="revolute"):
    b = {"id": id, "parent": parent, "title": tx(*title), "x": x, "y": y, "angleDeg": ang, "lengthM": L}
    if joint == "fixed":
        b["joint"] = {"type": "fixed"}
    else:
        b["joint"] = {"type": "revolute", "minDeg": lim[0], "maxDeg": lim[1]}
    if kg is not None:
        c = com if com is not None else (L/2, 0.0)
        b["mass"] = {"kg": kg, "comX": c[0], "comY": c[1], "distribution": dist, "source": "authored-estimate"}
        if ref: b["mass"]["reference"] = ref
    return b


def anchor(id, bone_id, x, y, kind, title, support=None, normal=None, fl=None, fw=None, mu=None):
    a = {"id": id, "bone": bone_id, "x": x, "y": y, "kind": kind, "title": tx(*title)}
    if support: a["support"] = support
    if normal is not None: a["normalDeg"] = normal
    if fl: a["footprintLengthM"] = fl
    if fw: a["footprintWidthM"] = fw
    if mu is not None: a["frictionCoefficient"] = mu
    return a


def material(id, title, em, sol=None, rho=None, k=None, cp=None, src="authored-estimate", ref=None):
    m = {"id": id, "title": tx(*title), "emissivity": em}
    if sol is not None: m["solarAbsorptivity"] = sol
    if rho is not None: m["densityKgM3"] = rho
    if k is not None: m["conductivityWmK"] = k
    if cp is not None: m["specificHeatJKgK"] = cp
    m["source"] = src
    if ref: m["reference"] = ref
    return m


def layer(id, title, kind, mat, thick, R, vapR, cap, src="authored-estimate", ref=None):
    l = {"id": id, "title": tx(*title), "kind": kind, "material": mat}
    if thick is not None: l["thicknessM"] = thick
    l["resistanceM2KW"] = R
    if vapR is not None: l["vapourResistanceM2PaW"] = vapR
    if cap is not None: l["heatCapacityJM2K"] = cap
    l["source"] = src
    if ref: l["reference"] = ref
    return l


def region(id, title, kind, model, mat, area, L, geom, sky, layers, cap, body=None, src="authored-estimate", ref=None, hidden=None):
    r = {"id": id, "title": tx(*title), "kind": kind, "model": model, "material": mat, "areaM2": area, "characteristicLengthM": L,
         "geometry": geom, "skyViewFactor": sky, "layers": layers, "heatCapacityJM2K": cap}
    if body: r["body"] = {"segment": body[0], "metabolicShare": body[1], "skinFraction": body[2]}
    if hidden: r["hidden"] = tx(*hidden)
    r["source"] = src
    if ref: r["reference"] = ref
    return r


def rb(e=1.0, s=0.5, contact=None, wet=None):
    d = {"exposedFraction": e, "sunlitFraction": s}
    if wet is not None: d["wettedness"] = wet
    if contact: d["contact"] = {"fraction": contact[0], "tempC": contact[1]}
    return d


def pose(id, title, desc, bones, contacts, regions, emitters, met, breathing, root=None, ground="auto", key=False):
    p = {"id": id, "title": tx(*title), "description": tx(*desc)}
    if key: p["key"] = True
    p["bones"] = bones
    if root: p["root"] = root
    p["ground"] = ground
    p["contacts"] = contacts
    p["thermal"] = {"activityMet": met, "breathing": breathing, "regions": regions, "emitters": emitters}
    p["loads"] = []
    return p


def provenance(refs, notes=None):
    p = {"author": "Semio staffage content worker NW-A2 (animals)", "method": "handcrafted", "created": "2026-10-29",
         "references": [{"id": i, "citation": c, **({"url": u} if u else {})} for i, c, u in refs]}
    if notes: p["notes"] = tx(*notes)
    return p


def hatch(id, title, pattern, angle, spacing, pen):
    return {"id": id, "title": tx(*title), "pattern": pattern, "angleDeg": angle, "spacingMm": spacing, "penMm": pen}


def decisions(sil, ret, mer, omi, thin, hat, sty, ove):
    keys = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]
    return {k: tx(*v) for k, v in zip(keys, [sil, ret, mer, omi, thin, hat, sty, ove])}


def write_asset(aid, asset):
    d = f"{CAT}/{aid}"
    os.makedirs(d, exist_ok=True)
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    with open(f"{d}/🔣️.json", "w") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
        fh.write("\n")


def print_rig(asset, pose_id):
    pose = next(p for p in asset["poses"] if p["id"] == pose_id)
    W = pose_world(asset, pose)
    for b in asset["rig"]["bones"]:
        o = apply(W[b["id"]], (0, 0)); t = apply(W[b["id"]], (b["lengthM"], 0))
        print(f'{b["id"]:14s} origin ({o[0]:7.3f},{o[1]:7.3f}) tip ({t[0]:7.3f},{t[1]:7.3f}) ang {world_angle(W[b["id"]]):7.1f}')
    for a in asset["anchors"]:
        p = apply(W[a["bone"]], (a["x"], a["y"]))
        print(f'  anchor {a["id"]:14s} ({p[0]:7.3f},{p[1]:7.3f})')


def skel(asset, pose_id, out, view=(-0.4, -0.25, 0.4, 0.55), den=10, px_per_mm=8):
    asset = dict(asset)
    asset["representations"] = {"1": {"pens": {"body": 0.2}, "hatches": [], "geometry": {"elevation": []}}}
    render(asset, "1", pose_id, out, view, px_per_mm=px_per_mm, bones=True, den=den, grid=0.05)


class Legs:
    """leg inverse kinematics for a trunk-hung four-bone leg (thigh, shank, tarsus, foot) with bone ids <name>-<side>"""

    def __init__(self, asset, root_x, hip_local, lens, cif, trunk="trunk"):
        self.a, self.rx, self.hip, self.l, self.cif, self.trunk = asset, root_x, hip_local, lens, cif, trunk

    def hip_world(self, pitch, ry):
        c, s = math.cos(math.radians(pitch)), math.sin(math.radians(pitch))
        return (self.rx + c*self.hip[0] - s*self.hip[1], ry + s*self.hip[0] + c*self.hip[1])

    def hip_x(self, pitch):
        return self.hip_world(pitch, 0)[0]

    def offsets(self, side, pitch, ry, target, tarsus_w, foot_w, bend=1):
        hip = self.hip_world(pitch, ry)
        c, s = math.cos(math.radians(foot_w)), math.sin(math.radians(foot_w))
        mtp = (target[0] - (c*self.cif[0] - s*self.cif[1]), target[1] - (s*self.cif[0] + c*self.cif[1]))
        ankle = (mtp[0] - self.l[2]*math.cos(math.radians(tarsus_w)), mtp[1] - self.l[2]*math.sin(math.radians(tarsus_w)))
        t1, t2 = two_link(hip, ankle, self.l[0], self.l[1], bend)
        return offsets_from_world(self.a, pitch, [f"{n}-{side}" for n in ("thigh", "shank", "tarsus", "foot")], [t1, t2, tarsus_w, foot_w])


def check_limits(asset):
    bad = []
    for p in asset["poses"]:
        for b, o in p["bones"].items():
            bb = next(x for x in asset["rig"]["bones"] if x["id"] == b)
            j = bb["joint"]
            if j["type"] == "revolute" and not (j["minDeg"] <= o.get("angleDeg", 0) <= j["maxDeg"]):
                bad.append((p["id"], b, o))
    return bad


def thermal_common(pats_ref):
    return None


def scale_asset(asset, k):
    """uniformly scale every length of the rig, anchors, drawings and authored roots (angles unchanged)"""
    num = re.compile(r"-?\d+\.?\d*(?:e-?\d+)?")
    for b in asset["rig"]["bones"]:
        for key in ("x", "y", "lengthM"): b[key] = round(b[key]*k, 5)
        if "mass" in b:
            b["mass"]["comX"] = round(b["mass"]["comX"]*k, 5); b["mass"]["comY"] = round(b["mass"]["comY"]*k, 5)
    for a in asset["anchors"]:
        a["x"] = round(a["x"]*k, 5); a["y"] = round(a["y"]*k, 5)
        for key in ("footprintLengthM", "footprintWidthM"):
            if key in a: a[key] = round(a[key]*k, 5)
    for p in asset["poses"]:
        r = p.get("root")
        if r:
            for key in ("x", "y"):
                if key in r: r[key] = round(r[key]*k, 5)
    for rep in asset["representations"].values():
        if rep["status"] != "authored": continue
        for s in rep["geometry"]["elevation"]:
            if s["kind"] == "path": s["d"] = re.sub(num, lambda m: f(float(m.group(0))*k), s["d"])
            elif s["kind"] == "poly": s["points"] = [[round(x*k, 5), round(y*k, 5)] for x, y in s["points"]]
            elif s["kind"] == "ellipse":
                for key in ("cx", "cy", "rx", "ry"): s[key] = round(s[key]*k, 5)
