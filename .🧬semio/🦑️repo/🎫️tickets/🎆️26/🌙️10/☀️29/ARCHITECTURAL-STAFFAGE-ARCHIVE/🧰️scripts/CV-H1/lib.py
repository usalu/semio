"""CV-H1 scratch library: forward kinematics of a pose, SVG sketch rendering (rsvg-convert), local<->world helpers.
Coordinates helper only; every shape is authored by hand in the per-asset scripts."""
import json, math, subprocess, os

CAT = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans"
GEN = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/CV-H1"


def load(aid):
    return json.load(open(f"{CAT}/{aid}/🔣️.json"))


def save(aid, doc):
    with open(f"{CAT}/{aid}/🔣️.json", "w") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")


def mul(a, b):
    return (a[0]*b[0]+a[2]*b[1], a[1]*b[0]+a[3]*b[1], a[0]*b[2]+a[2]*b[3], a[1]*b[2]+a[3]*b[3],
            a[0]*b[4]+a[2]*b[5]+a[4], a[1]*b[4]+a[3]*b[5]+a[5])


def rot(deg, x=0, y=0):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return (c, s, -s, c, x, y)


def fk(asset, pose=None, bones_override=None):
    """world matrices per bone for a pose (dict of bone offsets); root placement from pose.root"""
    offs = dict(pose["bones"]) if pose else {}
    if bones_override:
        offs.update(bones_override)
    root = (pose or {}).get("root") or {}
    W = {}
    for b in asset["rig"]["bones"]:
        o = offs.get(b["id"], {})
        x, y, ang = b["x"] + o.get("dx", 0), b["y"] + o.get("dy", 0), b["angleDeg"] + o.get("angleDeg", 0)
        if b["parent"] is None:
            x += root.get("x", 0); y += root.get("y", 0); ang += root.get("angleDeg", 0)
            W[b["id"]] = rot(ang, x, y)
        else:
            W[b["id"]] = mul(W[b["parent"]], rot(ang, x, y))
    return W


def apply(m, p):
    return (m[0]*p[0]+m[2]*p[1]+m[4], m[1]*p[0]+m[3]*p[1]+m[5])


def inv(m):
    d = m[0]*m[3]-m[1]*m[2]
    a, b, c, dd = m[3]/d, -m[1]/d, -m[2]/d, m[0]/d
    return (a, b, c, dd, -(a*m[4]+c*m[5]), -(b*m[4]+dd*m[5]))


def to_local(W, bone, pts):
    mi = inv(W[bone])
    return [apply(mi, p) for p in pts]


def f(v):
    return f"{v:.4g}" if abs(v) > 1e-9 else "0"


def spline(pts, closed=True, tension=0.5, corners=()):
    """Catmull-Rom through hand-entered points -> cubic path string (local metres). corners: indices with a sharp corner."""
    n = len(pts)
    d = f"M {f(pts[0][0])} {f(pts[0][1])}"
    segs = n if closed else n-1
    for i in range(segs):
        p0 = pts[(i-1) % n] if (closed or i > 0) else pts[i]
        p1 = pts[i]; p2 = pts[(i+1) % n]
        p3 = pts[(i+2) % n] if (closed or i+2 < n) else p2
        t1 = tension/3*2 if False else 1/6
        k = tension*2/6*1.0
        c1 = (p1[0]+(p2[0]-p0[0])*k, p1[1]+(p2[1]-p0[1])*k)
        c2 = (p2[0]-(p3[0]-p1[0])*k, p2[1]-(p3[1]-p1[1])*k)
        if i in corners:
            c1 = p1
        if (i+1) % n in corners:
            c2 = p2
        d += f" C {f(c1[0])} {f(c1[1])} {f(c2[0])} {f(c2[1])} {f(p2[0])} {f(p2[1])}"
    return d + (" Z" if closed else "")


def poly(pts, closed=True):
    return {"kind": "poly", "points": [[round(x, 4), round(y, 4)] for x, y in pts], "closed": closed}


def shape(id, bone, role, geom, z, filled=True, tone=0.3, region=None, regions=None, material=None, hatch=None, penMm=None):
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geom)
    if region: s["region"] = region
    if regions: s["regions"] = regions
    if material: s["material"] = material
    if hatch: s["hatch"] = hatch
    if penMm: s["penMm"] = penMm
    return s


def path(d):
    return {"kind": "path", "d": d}


def svg_shape(s, W, pen_m, mm_m=None):
    m = W[s["bone"]]
    tr = f'transform="matrix({m[0]:.6g},{m[1]:.6g},{m[2]:.6g},{m[3]:.6g},{m[4]:.6g},{m[5]:.6g})"'
    k = s["kind"]
    pm = (s.get("penMm") and s["penMm"] * mm_m) or pen_m
    tone = int(255*(1-s["tone"]))
    fill = f"rgb({tone},{tone},{tone})" if s["filled"] else "none"
    st = f'fill="{fill}" stroke="black" stroke-width="{pm:.6g}" stroke-linejoin="round" stroke-linecap="round"'
    if s["role"] == "glazing":
        st = f'fill="rgb(200,225,245)" stroke="black" stroke-width="{pm:.6g}"'
    if k == "path":
        return f'<path {tr} {st} d="{s["d"]}"/>'
    if k == "poly":
        pts = " ".join(f"{x},{y}" for x, y in s["points"])
        return f'<{"polygon" if s["closed"] else "polyline"} {tr} {st} points="{pts}"/>'
    if k == "ellipse":
        r = s.get("rotationDeg", 0)
        return f'<ellipse {tr} {st} cx="{s["cx"]}" cy="{s["cy"]}" rx="{s["rx"]}" ry="{s["ry"]}" transform-origin="0 0"/>' if False else \
            f'<g {tr}><ellipse {st} cx="{s["cx"]}" cy="{s["cy"]}" rx="{s["rx"]}" ry="{s["ry"]}" transform="rotate({r} {s["cx"]} {s["cy"]})"/></g>'
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


def render(shapes, W, out_png, den, px_per_mm=40, view=(-0.8, -0.05, 0.8, 2.0), pen_by_role=None, grid=0.1, ground_y=0.0, extra=""):
    """view = (x0,y0,x1,y1) metres; px_per_mm paper pixel density. Paper size = metres*1000/den mm."""
    x0, y0, x1, y1 = view
    pm_per_m = 1000/den*px_per_mm  # px per metre
    w, h = (x1-x0)*pm_per_m, (y1-y0)*pm_per_m
    pen_by_role = pen_by_role or {}
    body = []
    for s in sorted(shapes, key=lambda s: s["z"]):
        pen = pen_by_role.get(s["role"])
        if pen is None:
            continue
        body.append(svg_shape(s, W, pen*den/1000, den/1000))
    g = ""
    if grid:
        gx = math.floor(x0/grid)*grid
        while gx <= x1:
            g += f'<line x1="{gx}" y1="{y0}" x2="{gx}" y2="{y1}" stroke="#ddd" stroke-width="{0.5/pm_per_m}"/>'
            gx += grid
        gy = math.floor(y0/grid)*grid
        while gy <= y1:
            g += f'<line x1="{x0}" y1="{gy}" x2="{x1}" y2="{gy}" stroke="{"#f99" if abs(gy-ground_y) < 1e-6 else "#ddd"}" stroke-width="{0.5/pm_per_m}"/>'
            gy += grid
    svg = (f'<svg xmlns="http://www.w3.org/2000/svg" width="{w:.0f}" height="{h:.0f}" viewBox="0 0 {w:.0f} {h:.0f}">'
           f'<rect width="100%" height="100%" fill="white"/>'
           f'<g transform="scale({pm_per_m},{-pm_per_m}) translate({-x0},{-y1})">{g}{"".join(body)}{extra}</g></svg>')
    svg = svg.replace('transform="scale(', 'transform="scale(', 1)
    p = out_png[:-4] + ".svg"
    open(p, "w").write(svg)
    subprocess.run(["rsvg-convert", "-o", out_png, p], check=True)
    os.remove(p)


def bbox_world(shapes, W, samples=True):
    xs, ys = [], []
    import re
    for s in shapes:
        m = W[s["bone"]]
        pts = []
        k = s["kind"]
        if k == "path":
            nums = [float(v) for v in re.findall(r"-?\d+\.?\d*(?:e-?\d+)?", s["d"])]
            cmds = re.findall(r"[A-Za-z]", s["d"])
            if any(c in "Aa" for c in cmds):
                continue
            pts = [(nums[i], nums[i+1]) for i in range(0, len(nums)-1, 2)]
        elif k == "poly":
            pts = [tuple(p) for p in s["points"]]
        elif k == "ellipse":
            pts = [(s["cx"]+s["rx"]*math.cos(a/8*6.283), s["cy"]+s["ry"]*math.sin(a/8*6.283)) for a in range(8)]
        elif k == "rect":
            pts = [(s["x"], s["y"]), (s["x"]+s["w"], s["y"]+s["h"])]
        elif k == "capsule":
            pts = [(s["x0"]-s["r0"], s["y0"]-s["r0"]), (s["x0"]+s["r0"], s["y0"]+s["r0"]), (s["x1"]-s["r1"], s["y1"]-s["r1"]), (s["x1"]+s["r1"], s["y1"]+s["r1"])]
        for p in pts:
            q = apply(m, p)
            xs.append(q[0]); ys.append(q[1])
    return (min(xs), min(ys), max(xs), max(ys))


# ---------------------------------------------------------------- thermal helpers (arithmetic only; every value is entered by hand)
CLO = 0.155
VAP = 0.0165 * 0.38   # K/Pa Lewis relation x i_m, R_e = R / VAP


def T(en, de):
    return {"en": en, "de": de}


def garment(id, en, de, clo, material, thick, cap, fcl=1.0, kind="clothing", src="authored-estimate", ref="iso-9920-2007"):
    R = round(clo * CLO * fcl, 5)
    return {"id": id, "title": T(en, de), "kind": kind, "material": material, "thicknessM": thick,
            "resistanceM2KW": R, "vapourResistanceM2PaW": round(R / VAP, 1), "heatCapacityJM2K": cap, "source": src, "reference": ref}


def fcl_of(clo):
    return round(1 + 1.97 * clo * CLO, 3)


def body_region(id, en, de, kind, material, skinM2, fcl, segment, share, skinfrac, L, geom, sky, capJ, layers, src="authored-estimate", ref="iso-9920-2007"):
    return {"id": id, "title": T(en, de), "kind": kind, "model": "body", "material": material, "areaM2": round(skinM2 * fcl, 4),
            "characteristicLengthM": L, "geometry": geom, "skyViewFactor": sky, "layers": layers, "heatCapacityJM2K": capJ,
            "body": {"segment": segment, "metabolicShare": share, "skinFraction": skinfrac}, "source": src, "reference": ref}


def dubois(kg, hm):
    return round(0.007184 * kg ** 0.425 * (hm * 100) ** 0.725, 3)


class Fig:
    """figure-coordinate authoring: points as (s along bone, f forward) -> bone-local; sign determined once per bone from a pose."""
    def __init__(self, asset, pose, signs=None):
        self.a = asset; self.W = fk(asset, pose); self.sg = signs or {}

    def sgn(self, bone):
        if bone in self.sg:
            return self.sg[bone]
        m = self.W[bone]
        return 1 if m[2] > 0 else -1

    def pts(self, bone, p, raw=False):
        g = 1 if raw else self.sgn(bone)
        return [(s, g * f) for s, f in p]

    def sh(self, id, bone, role, p, z, tone, region=None, regions=None, closed=True, corners=(), filled=None, raw=False, smooth=True,
           material=None, hatch=None, pen=None):
        q = self.pts(bone, p, raw)
        if closed and smooth:
            g = path(spline(q, True, corners=corners))
        elif (not closed) and smooth and len(q) > 2:
            g = path(spline(q, False, corners=corners))
        else:
            g = poly(q, closed)
        return shape(id, bone, role, g, z, filled if filled is not None else closed, tone, region, regions, material, hatch, pen)

    def ell(self, id, bone, role, s, f, rs, rf, z, tone, region=None, material=None, raw=False):
        g = 1 if raw else self.sgn(bone)
        return shape(id, bone, role, {"kind": "ellipse", "cx": round(s, 4), "cy": round(g * f, 4), "rx": rs, "ry": rf}, z, True, tone, region, None, material)


def v1_to_50(shapes, hatch_map=None, drop=(), region_map=None):
    """1:50 drawing = the v1 detail drawing, re-bound (lod removed); hatch_map {shape id: hatch id}."""
    out = []
    for s in shapes:
        if s["id"] in drop:
            continue
        t = {k: v for k, v in s.items() if k != "lod"}
        if hatch_map and s["id"] in hatch_map:
            t["hatch"] = hatch_map[s["id"]]
        if region_map and s["id"] in region_map:
            t["region"] = region_map[s["id"]]
        out.append(t)
    return out


KEYS = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]


def decisions(en, de):
    assert set(en) == set(KEYS) == set(de), (set(KEYS) - set(en), set(KEYS) - set(de))
    return {k: T(en[k], de[k]) for k in KEYS}


def position_symbol(root, id, en, de, noteen, notede, arrow=True):
    g = [shape("dot", root, "body", {"kind": "ellipse", "cx": 0, "cy": 0, "rx": 0.45, "ry": 0.45}, 1, True, 0.9)]
    if arrow:
        g.append(shape("arrow", root, "detail", poly([(0.45, 0), (2.6, 0)], False), 2, False, 0.0))
        g.append(shape("arrow-head", root, "body", poly([(2.9, 0), (2.1, 0.35), (2.1, -0.35)], True), 3, True, 0.9))
    return {"id": id, "title": T(en[0], de[0]), "description": T(en[1], de[1]), "semantics": "true-position", "units": "paper-mm",
            "denominators": [500], "pens": {"body": 0.15, "detail": 0.15, "texture": None, "glazing": None, "ground": 0.18}, "hatches": [],
            "geometry": {"elevation": g}, "dimensionalNote": T(noteen, notede)}


def breath_emitters(head="head", nose_anchor="nostrils", mouth_anchor="mouth", plume_anchor="crown", plume_region="coat", plume_d=0.25, noseD=0.0095, mouthD=0.0124):
    es = [{"id": "nose-breath", "kind": "breath", "title": T("Breath from the nostrils", "Atem aus den Nasenöffnungen"), "anchor": nose_anchor,
           "directionDeg": -150, "diameterM": noseD, "source": "authored-estimate", "reference": "gupta-2010"},
          {"id": "mouth-breath", "kind": "breath", "title": T("Breath from the mouth", "Atem aus dem Mund"), "anchor": mouth_anchor,
           "directionDeg": -105, "diameterM": mouthD, "source": "authored-estimate", "reference": "gupta-2010"},
          {"id": "body-plume", "kind": "plume", "title": T("Thermal plume above the body", "Thermische Fahne über dem Körper"), "anchor": plume_anchor,
           "directionDeg": 0, "diameterM": plume_d, "region": plume_region, "source": "authored-estimate", "reference": "gupta-2010"}]
    return es


REFS = {
    "gagge-1971": "Gagge, A. P., Stolwijk, J. A. J. and Nishi, Y. (1971). An effective temperature scale based on a simple model of human physiological regulatory response. ASHRAE Transactions 77(1), 247-262.",
    "mifflin-1990": "Mifflin, M. D., St Jeor, S. T. et al. (1990). A new predictive equation for resting energy expenditure in healthy individuals. American Journal of Clinical Nutrition 51(2), 241-247.",
    "icrp-66-1994": "ICRP (1994). Human respiratory tract model for radiological protection. ICRP Publication 66, Annals of the ICRP 24(1-3); reference ventilation rates by activity and age (used as guidance, values authored).",
    "tartarini-2020": "Tartarini, F. and Schiavon, S. (2020). pythermalcomfort: a Python package for thermal comfort research. SoftwareX 12, 100578; two-node model defaults and clothing conventions (1 clo = 0.155 m2K/W, i_m about 0.38).",
}


def _wsh(self, id, bone, role, wpts, z, tone, region=None, regions=None, closed=True, corners=(), filled=None, smooth=True, material=None, hatch=None, pen=None):
    q = to_local(self.W, bone, wpts)
    if closed and smooth:
        g = path(spline(q, True, corners=corners))
    elif (not closed) and smooth and len(q) > 2:
        g = path(spline(q, False, corners=corners))
    else:
        g = poly(q, closed)
    return shape(id, bone, role, g, z, filled if filled is not None else closed, tone, region, regions, material, hatch, pen)


Fig.wsh = _wsh


def finish(aid, src, a, R, extra, doc_keys=None):
    """assemble ordered v2 asset from converted parts (extra: dict of replaced top-level keys) and save"""
    asset = OrderedDict()
    for k in ("id", "category", "title", "summary", "tags"):
        asset[k] = extra.get(k, a[k])
    for k in ("provenance", "dimensions", "materials", "rig", "anchors"):
        asset[k] = extra.get(k, a[k])
    asset["representations"] = OrderedDict((k, R[k]) for k in ("500", "200", "100", "50"))
    for k in ("symbols", "poses", "clips", "thermal", "structure"):
        asset[k] = extra.get(k, a.get(k))
    asset["presentation"] = extra["presentation"]
    asset["exports"] = a["exports"]
    asset["completion"] = "authored"
    doc = OrderedDict([("$schema", src["$schema"]), ("version", "semio.staffage/v2"), ("asset", asset)])
    save(aid, doc)
    print("wrote", aid, {k: len(v["geometry"]["elevation"]) if v["status"] == "authored" else "unavailable" for k, v in R.items()})


from collections import OrderedDict


def dim_report(aid):
    a = load(aid)["asset"]
    pose = next(p for p in a["poses"] if p["id"] == a["presentation"]["defaultPose"])
    W = fk(a, pose)
    d = a["dimensions"]
    for k, r in a["representations"].items():
        if r["status"] != "authored":
            print(aid, k, "unavailable"); continue
        bb = bbox_world([s for s in r["geometry"]["elevation"] if s["role"] != "ground"], W)
        h, w = bb[3] - max(bb[1], 0), bb[2] - bb[0]
        ok = lambda x, n: abs(x - n) <= max(0.02 * n, 0.015)
        print(f"{aid} {k}: h {h:.3f} vs {d['heightM']} {'ok' if ok(h, d['heightM']) else 'OFF'}  w {w:.3f} vs {d['widthM']} {'ok' if ok(w, d['widthM']) else 'OFF'}")
