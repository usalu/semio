"""CV-C scratch helpers: world-space drawing helpers that emit bone-local v2 shapes, plus thermal/pose helpers.
Every shape, number and text is authored in the per-asset scripts; this module only converts coordinates and assembles JSON."""
import json, math, copy

CAT = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/"


def T(en, de):
    return {"en": en, "de": de}


def r5(v):
    v = round(v, 5)
    return 0.0 if v == 0 else v


class Frame:
    def __init__(self, bones):
        self.b = {x["id"]: x for x in bones}
        self.cache = {}

    def world(self, bid):
        if bid in self.cache:
            return self.cache[bid]
        b = self.b[bid]
        if b["parent"] is None:
            res = (b["x"], b["y"], b["angleDeg"])
        else:
            px, py, pa = self.world(b["parent"])
            c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
            res = (px + c * b["x"] - s * b["y"], py + s * b["x"] + c * b["y"], pa + b["angleDeg"])
        self.cache[bid] = res
        return res

    def loc(self, bid, x, y):
        ox, oy, a = self.world(bid)
        c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
        dx, dy = x - ox, y - oy
        return (r5(c * dx + s * dy), r5(-s * dx + c * dy))

    def angle(self, bid):
        return self.world(bid)[2] % 360


class Draw:
    """Shape factory for one asset: all inputs are world metres of the default pose."""

    def __init__(self, bones):
        self.f = Frame(bones)

    def _base(self, id, bone, role, z, filled, tone, kw):
        d = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
        for k in ("region", "regions", "material", "hatch", "penMm"):
            if k in kw and kw[k] is not None:
                d[k] = kw[k]
        return d

    def poly(self, id, bone, role, z, filled, tone, pts, closed=True, **kw):
        d = self._base(id, bone, role, z, filled, tone, kw)
        d.update({"kind": "poly", "points": [list(self.f.loc(bone, x, y)) for x, y in pts], "closed": closed})
        return d

    def rect(self, id, bone, role, z, filled, tone, x0, y0, x1, y1, r=None, **kw):
        a = self.f.angle(bone)
        if abs(a) < 1e-6 or abs(a - 360) < 1e-6:
            lx, ly = self.f.loc(bone, x0, y0)
            d = self._base(id, bone, role, z, filled, tone, kw)
            d.update({"kind": "rect", "x": lx, "y": ly, "w": r5(x1 - x0), "h": r5(y1 - y0)})
            if r:
                d["r"] = r
            return d
        return self.poly(id, bone, role, z, filled, tone, [(x0, y0), (x1, y0), (x1, y1), (x0, y1)], True, **kw)

    def line(self, id, bone, role, z, tone, pts, **kw):
        return self.poly(id, bone, role, z, False, tone, pts, False, **kw)

    def circ(self, id, bone, role, z, filled, tone, cx, cy, r, **kw):
        lx, ly = self.f.loc(bone, cx, cy)
        d = self._base(id, bone, role, z, filled, tone, kw)
        d.update({"kind": "ellipse", "cx": lx, "cy": ly, "rx": r5(r), "ry": r5(r)})
        return d

    def ell(self, id, bone, role, z, filled, tone, cx, cy, rx, ry, **kw):
        a = self.f.angle(bone)
        assert abs(a) < 1e-6 or abs(rx - ry) < 1e-9, "axis-aligned ellipse only on unrotated bones"
        lx, ly = self.f.loc(bone, cx, cy)
        d = self._base(id, bone, role, z, filled, tone, kw)
        d.update({"kind": "ellipse", "cx": lx, "cy": ly, "rx": r5(rx), "ry": r5(ry)})
        return d

    def capsule(self, id, bone, role, z, filled, tone, x0, y0, x1, y1, r, **kw):
        a = self.f.angle(bone)
        assert abs(a) < 1e-6, "capsules only on unrotated bones"
        lx0, ly0 = self.f.loc(bone, x0, y0)
        lx1, ly1 = self.f.loc(bone, x1, y1)
        d = self._base(id, bone, role, z, filled, tone, kw)
        d.update({"kind": "capsule", "x0": lx0, "y0": ly0, "x1": lx1, "y1": ly1, "r0": r5(r), "r1": r5(r)})
        return d

    def path(self, id, bone, role, z, filled, tone, cmds, **kw):
        out = []
        for c in cmds:
            op = c[0]
            if op == "Z":
                out.append("Z")
                continue
            pts = c[1:]
            s = []
            for i in range(0, len(pts), 2):
                lx, ly = self.f.loc(bone, pts[i], pts[i + 1])
                s.append(f"{lx:g} {ly:g}")
            out.append(op + " " + " ".join(s))
        d = self._base(id, bone, role, z, filled, tone, kw)
        d.update({"kind": "path", "d": " ".join(out)})
        return d


def zigzag(x0, y0, x1, y1, n, amp, axis="x"):
    """Insulation line (DIN ISO 128-50 / DIN 1356-1 zigzag): n teeth along the long axis between two parallel limits."""
    pts = []
    if axis == "x":
        for i in range(n * 2 + 1):
            x = x0 + (x1 - x0) * i / (n * 2)
            pts.append((x, y0 + amp if i % 2 else y0 - amp))
        return pts
    for i in range(n * 2 + 1):
        y = y0 + (y1 - y0) * i / (n * 2)
        pts.append((x0 + amp if i % 2 else x0 - amp, y))
    return pts


# ---------------------------------------------------------------- print conventions
PENS = {
    "500": {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18},
    "200": {"body": 0.18, "detail": 0.1, "texture": None, "glazing": 0.1, "ground": 0.25},
    "100": {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35},
    "50": {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5},
}
RANGE = {"500": (350, 1000), "200": (150, 350), "100": (75, 150), "50": (20, 75)}


def H(id, en, de, pattern, angle, spacing, pen):
    return {"id": id, "title": T(en, de), "pattern": pattern, "angleDeg": angle, "spacingMm": spacing, "penMm": pen}


def dec(sil, ret, mer, omi, thin, hat, sty, ovl):
    return {"silhouette": T(*sil), "retained": T(*ret), "merged": T(*mer), "omitted": T(*omi), "thinElements": T(*thin),
            "hatching": T(*hat), "styles": T(*sty), "overlays": T(*ovl)}


def rep(den, shapes, decisions, hatches=(), pens=None, rng=None, plan=None):
    k = str(den)
    lo, hi = rng or RANGE[k]
    geom = {"elevation": shapes}
    if plan:
        geom["plan"] = plan
    return {"status": "authored", "denominator": den, "validRange": {"minDenominator": lo, "maxDenominator": hi},
            "pens": pens or PENS[k], "hatches": list(hatches), "geometry": geom, "decisions": decisions}


def unavailable(den, reason, feature, size, alts):
    return {"status": "unavailable", "denominator": den, "reason": T(*reason), "limitingFeature": T(*feature), "limitingSizeM": size,
            "alternatives": alts}


def bbox(shapes, frame):
    """World bounding box of shape geometry (centre lines, no pen width) at the default pose."""
    xs, ys = [], []

    def w(bone, x, y):
        ox, oy, a = frame.world(bone)
        c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
        return ox + c * x - s * y, oy + s * x + c * y

    for sh in shapes:
        b, k = sh["bone"], sh["kind"]
        pts = []
        if k == "poly":
            pts = [w(b, *p) for p in sh["points"]]
        elif k == "rect":
            pts = [w(b, sh["x"] + dx * sh["w"], sh["y"] + dy * sh["h"]) for dx in (0, 1) for dy in (0, 1)]
        elif k == "ellipse":
            pts = [w(b, sh["cx"] + dx * sh["rx"], sh["cy"] + dy * sh["ry"]) for dx in (-1, 0, 1) for dy in (-1, 0, 1)]
        elif k == "capsule":
            pts = [w(b, sh["x0"] + dx * sh["r0"], sh["y0"] + dy * sh["r0"]) for dx in (-1, 1) for dy in (-1, 1)] + [w(b, sh["x1"] + dx * sh["r1"], sh["y1"] + dy * sh["r1"]) for dx in (-1, 1) for dy in (-1, 1)]
        elif k == "path":
            toks = sh["d"].replace(",", " ").split()
            nums, cur = [], None
            i = 0
            while i < len(toks):
                t = toks[i]
                if t.isalpha():
                    i += 1
                    continue
                nums.append(float(t))
                i += 1
            pts = [w(b, nums[j], nums[j + 1]) for j in range(0, len(nums) - 1, 2)]
        xs += [p[0] for p in pts]
        ys += [p[1] for p in pts]
    return min(xs), min(ys), max(xs), max(ys)


def pose_thermal(regions, emitters=None):
    return {"regions": regions, "emitters": emitters or {}}


def RB(exposed=1.0, sun=0.0, **kw):
    d = {"exposedFraction": exposed, "sunlitFraction": sun}
    d.update(kw)
    return d


def layer(id, en, de, kind, material, d, R, src="literature", ref=None, c=None, vap=None):
    o = {"id": id, "title": T(en, de), "kind": kind, "material": material, "resistanceM2KW": R, "source": src}
    if d is not None:
        o["thicknessM"] = d
    if vap is not None:
        o["vapourResistanceM2PaW"] = vap
    if c is not None:
        o["heatCapacityJM2K"] = c
    if ref:
        o["reference"] = ref
    return o


def region(id, en, de, kind, model, material, area, L, geom, sky, layers, cap, src, ref=None, conditioned=None):
    o = {"id": id, "title": T(en, de), "kind": kind, "model": model, "material": material, "areaM2": area,
         "characteristicLengthM": L, "geometry": geom, "skyViewFactor": sky, "layers": layers, "heatCapacityJM2K": cap}
    if conditioned is not None:
        o["conditioned"] = conditioned
    o["source"] = src
    if ref:
        o["reference"] = ref
    return o


V1 = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/CVF-VMC/c/v1/"


def load_doc(aid):
    return json.load(open(V1 + aid + ".json"))


def save_doc(aid, doc):
    s = json.dumps(doc, ensure_ascii=False, indent=2)
    open(CAT + aid + "/🔣️.json", "w").write(s + "\n")


COMMON_REFS = [
    {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, general principles of presentation, part 50: basic conventions for areas on sections and cuts (hatching)."},
    {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, part 1: types of drawings, contents and basic principles of presentation (scales, line groups, symbols for doors, windows and stairs)."},
    {"id": "en-iso-6946", "citation": "EN ISO 6946:2017 Building components and building elements, thermal resistance and thermal transmittance (surface resistances R_si 0.13 and R_se 0.04 m2K/W for horizontal heat flow)."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (design thermal conductivity, specific heat capacity: concrete 2.3 W/mK, 1000 J/kgK; mineral wool 0.035 W/mK; softwood 0.13 W/mK, 1600 J/kgK; steel 50 W/mK, 450 J/kgK; flat glass 1.0 W/mK, 750 J/kgK)."},
    {"id": "en-iso-10077-1", "citation": "EN ISO 10077-1:2017 Thermal performance of windows, doors and shutters, calculation of thermal transmittance, part 1 (U_g, U_f, edge-of-glass linear transmittance psi_g)."},
    {"id": "en-iso-10211", "citation": "EN ISO 10211:2017 Thermal bridges in building construction, heat flows and surface temperatures, detailed calculations (linear transmittance psi)."},
    {"id": "en-12898", "citation": "EN 12898:2019 Glass in building, determination of the emissivity (uncoated float glass, corrected emissivity 0.837)."},
]


def merge_refs(doc_asset, extra=()):
    have = {r["id"] for r in doc_asset["provenance"]["references"]}
    for r in list(COMMON_REFS) + list(extra):
        if r["id"] not in have:
            doc_asset["provenance"]["references"].append(r)
            have.add(r["id"])


DEFAULT_SCALE = {"cantilever-balcony": 50, "glulam-beam": 100, "steel-column": 50, "stair-flight": 50, "sliding-door": 50, "tilt-window": 50, "glass-balustrade": 50, "tied-canopy": 50}


def finish(a):
    """Strip every v1-only field and order the keys of the asset like the v2 type."""
    a.pop("geometry", None)
    a["thermal"].setdefault("passengers", [])
    a["presentation"]["defaultScale"] = DEFAULT_SCALE[a["id"]]
    for rg in a["thermal"]["regions"]:
        rg.pop("lod", None)
    order = ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "representations",
             "symbols", "poses", "clips", "thermal", "structure", "presentation", "exports", "completion"]
    return {k: a[k] for k in order}


EXTRA_REFS = {
    "en-iso-13786": {"id": "en-iso-13786", "citation": "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics: periodic penetration depth delta = sqrt(lambda T / (pi rho c)); about half of it is taken as the effective storage depth of an exposed face."},
}


def set_material(a, mid, **props):
    for m in a["materials"]:
        if m["id"] == mid:
            m.update(props)
            return m
    m = {"id": mid}
    m.update(props)
    a["materials"].append(m)
    return m


def bind_poses(a, base, overrides=None):
    """base: {region: (exposed, sunlit)}; overrides: {pose-id: {region: (exposed, sunlit) or dict}}; applies to every pose."""
    overrides = overrides or {}
    for p in a["poses"]:
        b = {}
        for rid, v in {**base, **overrides.get(p["id"], {})}.items():
            b[rid] = v if isinstance(v, dict) else RB(v[0], v[1])
        p["thermal"] = {"regions": b, "emitters": {}}


def validate(a):
    """Own light validation (the catalog check CLI is the authority): bboxes, region/hatch/bone ids, pens, symbols."""
    fr = Frame(a["rig"]["bones"])
    bones = set(fr.b)
    regs = {r["id"] for r in a["thermal"]["regions"]}
    d = a["dimensions"]
    ok = True
    for key, rp in a["representations"].items():
        if rp["status"] != "authored":
            continue
        sh = rp["geometry"]["elevation"]
        x0, y0, x1, y1 = bbox(sh, fr)
        w, h = x1 - x0, y1 - y0
        tol_h = max(0.02 * d["heightM"], 0.015)
        tol_w = max(0.02 * d["widthM"], 0.015)
        flag = "" if abs(h - d["heightM"]) <= tol_h and abs(w - d["widthM"]) <= tol_w else "  <-- DIMENSION MISMATCH"
        if flag:
            ok = False
        print(f"  {key}: {len(sh)} shapes bbox w={w:.3f} h={h:.3f} (dims {d['widthM']} x {d['heightM']}) x[{x0:.3f},{x1:.3f}] y[{y0:.3f},{y1:.3f}]{flag}")
        used = set()
        hat = {h_["id"] for h_ in rp["hatches"]}
        ids = set()
        for s in sh:
            if s["id"] in ids:
                print("  DUP shape id", s["id"]); ok = False
            ids.add(s["id"])
            if s["bone"] not in bones:
                print("  BAD bone", s["id"]); ok = False
            for r in ([s["region"]] if "region" in s else []) + s.get("regions", []):
                if r not in regs:
                    print("  BAD region", s["id"], r); ok = False
                used.add(r)
            if "hatch" in s and s["hatch"] not in hat:
                print("  BAD hatch", s["id"]); ok = False
            if rp["pens"][s["role"]] is None and "penMm" not in s:
                print("  ROLE NOT DRAWN", s["id"], s["role"]); ok = False
            if s["role"] not in ("texture", "detail") and s["filled"] and not ("region" in s or "regions" in s):
                print("  FILLED SHAPE WITHOUT REGION", s["id"]); ok = False
        miss = regs - used
        if miss:
            print("  REGIONS NOT DRAWN", sorted(miss)); ok = False
    for p in a["poses"]:
        miss = regs - set(p["thermal"]["regions"])
        if miss:
            print("  POSE missing regions", p["id"], miss); ok = False
    print("  VALID" if ok else "  PROBLEMS")
    return ok
