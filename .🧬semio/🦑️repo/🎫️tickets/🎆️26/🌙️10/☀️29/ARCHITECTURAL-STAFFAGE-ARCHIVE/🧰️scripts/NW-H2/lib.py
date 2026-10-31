"""NW-H2 shared helpers: rig forward kinematics, two-bone IK, smooth outline paths, shape records, thermal builders,
bounds and a quick SVG preview. All shapes, numbers and decisions are authored in the per-asset scripts; this file only computes coordinates."""
import json, math, os, re, subprocess

S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage"
TK = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE"
HERE = os.path.dirname(os.path.abspath(__file__))
GEN = f"{TK}/🗑️generated/NW-H2"
os.makedirs(GEN, exist_ok=True)


def tx(en, de):
    return {"en": en, "de": de}


def r4(x):
    return round(x + 0.0, 4)


def out_path(asset_id):
    d = f"{S}/🗂️catalog/👤️humans/{asset_id}"
    os.makedirs(d, exist_ok=True)
    return f"{d}/🔣️.json"


# ---------------------------------------------------------------- kinematics
def rot(a):
    return math.cos(math.radians(a)), math.sin(math.radians(a))


class Fig:
    """A rig plus a pose under construction (offsets, root)."""

    def __init__(self, bones):
        self.bones = {b["id"]: b for b in bones}
        self.order = [b["id"] for b in bones]
        self.root_id = [b["id"] for b in bones if b["parent"] is None][0]
        self.off = {}
        self.root = {"x": 0.0, "y": 0.0, "angleDeg": 0.0}

    def copy(self):
        f = Fig(list(self.bones.values()))
        f.off = {k: dict(v) for k, v in self.off.items()}
        f.root = dict(self.root)
        return f

    def world_all(self):
        W = {}

        def go(i):
            if i in W:
                return W[i]
            b = self.bones[i]
            o = self.off.get(i, {})
            if b["parent"] is None:
                W[i] = (b["x"] + self.root["x"] + o.get("dx", 0), b["y"] + self.root["y"] + o.get("dy", 0), b["angleDeg"] + self.root["angleDeg"] + o.get("angleDeg", 0))
            else:
                px, py, pa = go(b["parent"])
                c, s = rot(pa)
                lx, ly = b["x"] + o.get("dx", 0), b["y"] + o.get("dy", 0)
                W[i] = (px + c * lx - s * ly, py + s * lx + c * ly, pa + b["angleDeg"] + o.get("angleDeg", 0))
            return W[i]

        for i in self.order:
            go(i)
        return W

    def world(self, i):
        return self.world_all()[i]

    def at(self, i, pt):
        x, y, a = self.world(i)
        c, s = rot(a)
        return (x + c * pt[0] - s * pt[1], y + s * pt[0] + c * pt[1])

    def tip(self, i):
        return self.at(i, (self.bones[i]["lengthM"], 0))

    def to_local(self, i, p):
        x, y, a = self.world(i)
        c, s = rot(a)
        dx, dy = p[0] - x, p[1] - y
        return (c * dx + s * dy, -s * dx + c * dy)

    def aim(self, i, world_deg):
        b = self.bones[i]
        pa = self.world(b["parent"])[2] if b["parent"] else self.root["angleDeg"]
        self.off.setdefault(i, {})["angleDeg"] = norm180(world_deg - pa - b["angleDeg"])

    def rel(self, i, deg):
        self.off.setdefault(i, {})["angleDeg"] = deg

    def place_pelvis(self, x, y, shift="hip-shift", lift="hip-lift"):
        """Prismatic hips: put the pelvis origin at world (x, y) of the unrotated rest frame."""
        self.off.setdefault(shift, {})["dx"] = 0.0
        self.off.setdefault(lift, {})["dy"] = 0.0
        px, py, _ = self.world("pelvis")
        self.off[shift]["dx"] = x - px
        self.off[lift]["dy"] = y - py

    def leg(self, side, ankle, bend=1, thigh="thigh", shank="shank"):
        t, s = f"{thigh}-{side}", f"{shank}-{side}"
        hip = self.world("pelvis")[:2]
        a1, a2 = ik2(hip, ankle, self.bones[t]["lengthM"], self.bones[s]["lengthM"], bend)
        self.aim(t, a1)
        self.aim(s, a2)

    def arm(self, side, wrist, bend=-1, upper="upper-arm", fore="forearm", base="torso", base_pt=None):
        u, f = f"{upper}-{side}", f"{fore}-{side}"
        sh = self.world(u)[:2]
        a1, a2 = ik2(sh, wrist, self.bones[u]["lengthM"], self.bones[f]["lengthM"], bend)
        self.aim(u, a1)
        self.aim(f, a2)

    def pose_dict(self):
        o = {}
        for k in self.order:
            v = self.off.get(k)
            if v is None:
                continue
            e = {}
            if "angleDeg" in v:
                e["angleDeg"] = round(v["angleDeg"], 3)
            if abs(v.get("dx", 0)) > 1e-9:
                e["dx"] = round(v["dx"], 4)
            if abs(v.get("dy", 0)) > 1e-9:
                e["dy"] = round(v["dy"], 4)
            if e:
                o[k] = e
        return o


def norm180(a):
    return (a + 180) % 360 - 180


def ik2(a, b, l1, l2, bend):
    dx, dy = b[0] - a[0], b[1] - a[1]
    d = math.hypot(dx, dy)
    d = min(d, l1 + l2 - 1e-6)
    d = max(d, abs(l1 - l2) + 1e-6)
    ang = math.atan2(dy, dx)
    cos_a = max(-1, min(1, (l1 * l1 + d * d - l2 * l2) / (2 * l1 * d)))
    th1 = ang + bend * math.acos(cos_a)
    k = (a[0] + l1 * math.cos(th1), a[1] + l1 * math.sin(th1))
    th2 = math.atan2(b[1] - k[1], b[0] - k[0])
    return math.degrees(th1), math.degrees(th2)


def joint_report(fig):
    """Return the list of joint-limit violations of the pose under construction."""
    bad = []
    for i, o in fig.off.items():
        b = fig.bones[i]
        j = b["joint"]
        if j["type"] == "revolute":
            a = o.get("angleDeg", 0)
            if "minDeg" in j and a < j["minDeg"] - 1e-6:
                bad.append(f"{i} {a:.1f} < {j['minDeg']}")
            if "maxDeg" in j and a > j["maxDeg"] + 1e-6:
                bad.append(f"{i} {a:.1f} > {j['maxDeg']}")
        elif j["type"] == "prismatic":
            ax = math.radians(j.get("axisDeg", 0))
            s = o.get("dx", 0) * math.cos(ax) + o.get("dy", 0) * math.sin(ax)
            if "minM" in j and s < j["minM"] - 1e-6:
                bad.append(f"{i} {s:.3f} < {j['minM']}")
            if "maxM" in j and s > j["maxM"] + 1e-6:
                bad.append(f"{i} {s:.3f} > {j['maxM']}")
    return bad


# ---------------------------------------------------------------- outlines
def smooth(pts, closed=True):
    """Catmull-Rom through the points as cubic Beziers; a third tuple element marks a corner."""
    P = [(p[0], p[1]) for p in pts]
    C = [len(p) > 2 for p in pts]
    n = len(P)

    def get(i):
        return P[i % n] if closed else P[min(max(i, 0), n - 1)]

    d = f"M {r4(P[0][0])} {r4(P[0][1])}"
    for i in range(n if closed else n - 1):
        p0, p1, p2, p3 = get(i - 1), get(i), get(i + 1), get(i + 2)
        corner1 = C[i % n] or (not closed and i == 0)
        corner2 = C[(i + 1) % n] or (not closed and i + 1 == n - 1)
        if corner1:
            c1 = (p1[0] + (p2[0] - p1[0]) / 3, p1[1] + (p2[1] - p1[1]) / 3)
        else:
            c1 = (p1[0] + (p2[0] - p0[0]) / 6, p1[1] + (p2[1] - p0[1]) / 6)
        if corner2:
            c2 = (p2[0] - (p2[0] - p1[0]) / 3, p2[1] - (p2[1] - p1[1]) / 3)
        else:
            c2 = (p2[0] - (p3[0] - p1[0]) / 6, p2[1] - (p3[1] - p1[1]) / 6)
        d += f" C {r4(c1[0])} {r4(c1[1])} {r4(c2[0])} {r4(c2[1])} {r4(p2[0])} {r4(p2[1])}"
    return d + (" Z" if closed else "")


def path(pts, closed=True):
    return {"kind": "path", "d": smooth(pts, closed)}


def lines(pts, closed=True):
    """Straight-edged path (polygon) with optional corner radius 0."""
    d = "M " + " L ".join(f"{r4(p[0])} {r4(p[1])}" for p in pts)
    return {"kind": "path", "d": d + (" Z" if closed else "")}


def poly(pts, closed=False):
    return {"kind": "poly", "points": [[r4(p[0]), r4(p[1])] for p in pts], "closed": closed}


def ellipse(cx, cy, rx, ry, rot_deg=0):
    g = {"kind": "ellipse", "cx": r4(cx), "cy": r4(cy), "rx": r4(rx), "ry": r4(ry)}
    if rot_deg:
        g["rotationDeg"] = rot_deg
    return g


def rect(x, y, w, h, r=0):
    g = {"kind": "rect", "x": r4(x), "y": r4(y), "w": r4(w), "h": r4(h)}
    if r:
        g["r"] = r4(r)
    return g


def capsule(x0, y0, x1, y1, r0, r1):
    return {"kind": "capsule", "x0": r4(x0), "y0": r4(y0), "x1": r4(x1), "y1": r4(y1), "r0": r4(r0), "r1": r4(r1)}


def limb(L, st, cap0=0.0, cap1=0.0):
    """Outline of a bone-aligned form: st = [(s, wa, wb[, 'c'])] with s metres along x, wa the half width on +y, wb on -y; a fourth
    element 'c' makes both edge points corners. cap0/cap1 push a rounded tip beyond the first/last station (L is unused, kept for symmetry)."""
    top = [(q[0], q[1]) + (("c",) if len(q) > 3 else ()) for q in st]
    bot = [(q[0], -q[2]) + (("c",) if len(q) > 3 else ()) for q in reversed(st)]
    pts = list(top)
    if cap1:
        pts.append((st[-1][0] + cap1, (st[-1][1] - st[-1][2]) / 2))
    pts += bot
    if cap0:
        pts.append((st[0][0] - cap0, (st[0][1] - st[0][2]) / 2))
    return pts


def mirror_y(pts):
    return [(p[0], -p[1]) + tuple(p[2:]) for p in pts]


def scaled(pts, f, ox=0.0, oy=0.0):
    return [((p[0] - ox) * f + ox, (p[1] - oy) * f + oy) + tuple(p[2:]) for p in pts]


def shape(sid, bone, role, z, tone, geom, filled=True, region=None, regions=None, material=None, hatch=None, pen=None):
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geom)
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    if pen:
        s["penMm"] = pen
    return s


# ---------------------------------------------------------------- flatten and bounds
def flatten(geom, n=8):
    k = geom["kind"]
    if k == "path":
        toks = re.findall(r"[MLCQZ]|-?\d*\.?\d+(?:e-?\d+)?", geom["d"])
        out, cur, i, start, cmd = [], (0, 0), 0, (0, 0), None
        sub = []
        while i < len(toks):
            t = toks[i]
            if t in "MLCQZ":
                cmd = t
                i += 1
                if t == "Z":
                    if sub:
                        out.append(sub)
                    sub = []
                    cur = start
                    continue
            if cmd == "M":
                if sub:
                    out.append(sub)
                cur = (float(toks[i]), float(toks[i + 1]))
                start = cur
                sub = [cur]
                i += 2
                cmd = "L"
            elif cmd == "L":
                cur = (float(toks[i]), float(toks[i + 1]))
                sub.append(cur)
                i += 2
            elif cmd == "C":
                v = [float(x) for x in toks[i:i + 6]]
                i += 6
                p0 = cur
                for j in range(1, n + 1):
                    u = j / n
                    a, b, c, d = (1 - u) ** 3, 3 * u * (1 - u) ** 2, 3 * u * u * (1 - u), u ** 3
                    sub.append((a * p0[0] + b * v[0] + c * v[2] + d * v[4], a * p0[1] + b * v[1] + c * v[3] + d * v[5]))
                cur = (v[4], v[5])
            elif cmd == "Q":
                v = [float(x) for x in toks[i:i + 4]]
                i += 4
                p0 = cur
                for j in range(1, n + 1):
                    u = j / n
                    sub.append(((1 - u) ** 2 * p0[0] + 2 * u * (1 - u) * v[0] + u * u * v[2], (1 - u) ** 2 * p0[1] + 2 * u * (1 - u) * v[1] + u * u * v[3]))
                cur = (v[2], v[3])
        if sub:
            out.append(sub)
        return out
    if k == "poly":
        return [[tuple(p) for p in geom["points"]]]
    if k == "ellipse":
        a = math.radians(geom.get("rotationDeg", 0))
        pts = []
        for j in range(24):
            u = 2 * math.pi * j / 24
            x, y = geom["rx"] * math.cos(u), geom["ry"] * math.sin(u)
            pts.append((geom["cx"] + x * math.cos(a) - y * math.sin(a), geom["cy"] + x * math.sin(a) + y * math.cos(a)))
        return [pts]
    if k == "rect":
        x, y, w, h = geom["x"], geom["y"], geom["w"], geom["h"]
        return [[(x, y), (x + w, y), (x + w, y + h), (x, y + h)]]
    if k == "capsule":
        pts = []
        a = math.atan2(geom["y1"] - geom["y0"], geom["x1"] - geom["x0"])
        for j in range(13):
            u = a + math.pi / 2 + math.pi * j / 12
            pts.append((geom["x1"] + geom["r1"] * math.cos(u - math.pi), geom["y1"] + geom["r1"] * math.sin(u - math.pi)))
        for j in range(13):
            u = a - math.pi / 2 + math.pi * j / 12
            pts.append((geom["x0"] + geom["r0"] * math.cos(u - math.pi), geom["y0"] + geom["r0"] * math.sin(u - math.pi)))
        return [pts]
    raise ValueError(k)


def world_rings(fig, shp):
    rings = []
    for ring in flatten(shp):
        rings.append([fig.at(shp["bone"], p) for p in ring])
    return rings


def bounds(fig, shapes):
    xs, ys = [], []
    for s in shapes:
        if s["role"] == "ground":
            continue
        for ring in world_rings(fig, s):
            for p in ring:
                xs.append(p[0])
                ys.append(p[1])
    return min(xs), min(ys), max(xs), max(ys)


# ---------------------------------------------------------------- preview
def tone_hex(t):
    v = int(round(255 * max(0, min(1, t))))
    return f"#{v:02x}{v:02x}{v:02x}"


def svg(fig, shapes, pens, denominator, zoom=1.0, ground_y=0.0, extra_w=0.3, ids=False, pad=0.15):
    """A quick paper-mm preview of one drawing in the figure's current pose (not the official preview)."""
    k = 1000.0 / denominator
    x0, y0, x1, y1 = bounds(fig, shapes)
    x0 -= pad
    x1 += pad
    y0 = min(y0, ground_y) - 0.03
    y1 += pad * 0.5
    W, H = (x1 - x0) * k, (y1 - y0) * k
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W:.3f} {H:.3f}" width="{W * zoom * 300 / 25.4:.0f}" height="{H * zoom * 300 / 25.4:.0f}">', '<rect width="100%" height="100%" fill="white"/>']
    gy = (y1 - ground_y) * k
    out.append(f'<line x1="0" x2="{W:.3f}" y1="{gy:.3f}" y2="{gy:.3f}" stroke="#888" stroke-width="{pens["ground"]}"/>')
    for s in sorted(shapes, key=lambda q: q["z"]):
        pen = s.get("penMm", pens.get(s["role"]))
        if pen is None or s["role"] == "ground":
            continue
        rings = world_rings(fig, s)
        d = ""
        for ring in rings:
            d += "M " + " L ".join(f"{(p[0] - x0) * k:.3f},{(y1 - p[1]) * k:.3f}" for p in ring)
            if s["kind"] != "poly" or s.get("closed"):
                if s["filled"] or s["kind"] in ("path", "ellipse", "rect", "capsule") and not s["kind"] == "poly" and "Z" in s.get("d", "Z"):
                    d += " Z"
        fill = tone_hex(s["tone"]) if s["filled"] else "none"
        out.append(f'<path d="{d}" fill="{fill}" stroke="black" stroke-width="{pen}" stroke-linejoin="round" stroke-linecap="round"/>')
    out.append("</svg>")
    return "\n".join(out)


def render(svg_text, name, width_px=None):
    p = f"{GEN}/{name}"
    with open(p + ".svg", "w") as f:
        f.write(svg_text)
    cmd = ["rsvg-convert", p + ".svg", "-o", p + ".png"]
    if width_px:
        cmd[1:1] = ["-w", str(width_px)]
    subprocess.run(cmd, check=True)
    return p + ".png"


# ---------------------------------------------------------------- JSON output
def dump(obj, path_, depth=4):
    def enc(o, d):
        pad = "  " * d
        if isinstance(o, dict) and d < depth:
            if not o:
                return "{}"
            return "{\n" + ",\n".join(f'{pad}  {json.dumps(k, ensure_ascii=False)}: {enc(v, d + 1)}' for k, v in o.items()) + f"\n{pad}}}"
        if isinstance(o, list) and d < depth:
            if not o:
                return "[]"
            return "[\n" + ",\n".join(f"{pad}  {enc(v, d + 1)}" for v in o) + f"\n{pad}]"
        return json.dumps(o, ensure_ascii=False)

    with open(path_, "w") as f:
        f.write(enc(obj, 0) + "\n")


# ---------------------------------------------------------------- physiology helpers
def dubois(height_m, mass_kg):
    return 0.007184 * mass_kg ** 0.425 * (height_m * 100) ** 0.725


def mifflin_w(male, mass_kg, height_m, age):
    kcal = 10 * mass_kg + 6.25 * height_m * 100 - 5 * age + (5 if male else -161)
    return kcal * 4184 / 86400


def clo_r(clo, fcl):
    return round(clo * 0.155 * fcl, 4)


def vap(r):
    return round(r / (16.5e-3 * 0.38), 2)


def layer(lid, en, de, kind, material, thick, r, cap=None, vapour=True, src="authored-estimate", ref="iso-9920-2007"):
    L = {"id": lid, "title": tx(en, de), "kind": kind, "material": material, "thicknessM": thick, "resistanceM2KW": r}
    if vapour and kind not in ("air-gap",):
        L["vapourResistanceM2PaW"] = vap(r)
    if cap is not None:
        L["heatCapacityJM2K"] = cap
    L["source"] = src
    L["reference"] = ref
    return L


def region(rid, en, de, kind, material, area, charL, geometry, sky, layers, cap, segment, share, skin, subject=None, hidden=None, ref="iso-9920-2007", src="authored-estimate", model="body"):
    r = {"id": rid, "title": tx(en, de), "kind": kind, "model": model, "material": material, "areaM2": round(area, 4), "characteristicLengthM": charL, "geometry": geometry, "skyViewFactor": sky, "layers": layers, "heatCapacityJM2K": cap}
    if model == "body":
        b = {"segment": segment, "metabolicShare": share, "skinFraction": skin}
        if subject:
            b["subject"] = subject
        r["body"] = b
    if hidden:
        r["hidden"] = hidden
    r["source"] = src
    r["reference"] = ref
    return r


def breath_pattern(pid, en, de, rate, vt, i, p1, e, p2, route, ref, src="authored-estimate"):
    assert abs(i + p1 + e + p2 - 1) < 1e-6, pid
    return {"id": pid, "title": tx(en, de), "rateBpm": rate, "tidalVolumeL": vt, "inhaleFraction": i, "pauseAfterInhaleFraction": p1, "exhaleFraction": e, "pauseAfterExhaleFraction": p2, "route": route, "source": src, "reference": ref}


def rbind(exposed=1.0, sun=0.0, wet=None, contact=None):
    b = {"exposedFraction": exposed, "sunlitFraction": sun}
    if wet is not None:
        b["wettedness"] = wet
    if contact is not None:
        b["contact"] = {"fraction": contact[0], "tempC": contact[1]}
    return b


ASSET_REFS = {
    "de-leva-1996": "de Leva, P. (1996). Adjustments to Zatsiorsky-Seluyanov's segment inertia parameters. Journal of Biomechanics 29(9), 1223-1230 (segment mass fractions and centre-of-mass positions, male and female).",
    "dubois-1916": "DuBois, D. and DuBois, E. F. (1916). A formula to estimate the approximate surface area if height and weight be known. Archives of Internal Medicine 17, 863-871.",
    "iso-9920-2007": "ISO 9920:2007 Ergonomics of the thermal environment, estimation of thermal insulation and water vapour resistance of a clothing ensemble (1 clo = 0.155 m2K/W; garment clo values here are authored estimates guided by its tables).",
    "ainsworth-2011": "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581 (activity classes; MET values are mapped to thermal met as an estimate).",
    "ashrae-2021": "ASHRAE Handbook Fundamentals (2021), Chapter 9 Thermal Comfort (met unit 58.15 W/m2, respiratory heat loss, skin wettedness).",
    "gagge-1971": "Gagge, A. P., Stolwijk, J. A. J., Nishi, Y. (1971). An effective temperature scale based on a simple model of human physiological regulatory response. ASHRAE Transactions 77(1), 247-262 (two-node model; set points 36.8 and 34.1 degrees Celsius, skin blood flow limits 6.3 and 90 L/(m2 h) after the common implementation).",
    "gupta-2010": "Gupta, J. K., Lin, C.-H., Chen, Q. (2010). Characterizing exhaled airflow from breathing and talking. Indoor Air 20(1), 31-39 (sinusoidal breathing flow; opening areas and jet angles used here are authored estimates guided by it).",
    "hoeppe-1981": "Hoeppe, P. (1981). Temperatures of expired air under varying climatic conditions. International Journal of Biometeorology 25, 127-132 (expired air temperature; the 34 degrees Celsius and 95 % used here are authored estimates).",
    "mifflin-1990": "Mifflin, M. D. et al. (1990). A new predictive equation for resting energy expenditure in healthy individuals. American Journal of Clinical Nutrition 51, 241-247.",
    "incropera-2011": "Incropera, F. P., DeWitt, D. P. et al. (2011). Fundamentals of Heat and Mass Transfer, 7th ed., Wiley (convection correlations, material properties).",
    "winter-2009": "Winter, D. A. (2009). Biomechanics and Motor Control of Human Movement, 4th ed., Wiley (anthropometric tables, joint ranges, gait kinematics).",
    "drillis-contini-1966": "Drillis, R., Contini, R. (1966). Body segment parameters. New York University, School of Engineering and Science, Report 1166-03 (segment lengths as fractions of stature).",
}


def refs(*ids, extra=None):
    out = [{"id": i, "citation": ASSET_REFS[i]} for i in ids]
    for e in extra or []:
        out.append(e)
    return out


def skeleton_svg(figs, labels=None, denominator=20, extra_pts=None, ground_y=0.0, x_range=None, y_range=None):
    """Stick-figure sheet of several poses side by side (debug)."""
    k = 1000.0 / denominator
    cells = []
    for f in figs:
        W = f.world_all()
        cells.append(W)
    x_lo, x_hi = x_range or (-1.0, 1.1)
    y_lo, y_hi = y_range or (-0.1, 2.0)
    cw, ch = (x_hi - x_lo) * k, (y_hi - y_lo) * k
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {cw * len(figs):.1f} {ch:.1f}" width="{cw * len(figs) * 1.2:.0f}" height="{ch * 1.2:.0f}"><rect width="100%" height="100%" fill="white"/>']
    for n, f in enumerate(figs):
        ox = n * cw
        X = lambda x: ox + (x - x_lo) * k
        Y = lambda y: (y_hi - y) * k
        out.append(f'<line x1="{ox}" x2="{ox + cw}" y1="{Y(ground_y)}" y2="{Y(ground_y)}" stroke="#999" stroke-width="0.4"/>')
        if labels:
            out.append(f'<text x="{ox + 4}" y="12" font-size="9">{labels[n]}</text>')
        W = f.world_all()
        for i, b in f.bones.items():
            x, y, a = W[i]
            c, s = rot(a)
            ex, ey = x + c * b["lengthM"], y + s * b["lengthM"]
            col = "#c00" if i.endswith("-l") else "#06c" if i.endswith("-r") else "#222"
            out.append(f'<line x1="{X(x)}" y1="{Y(y)}" x2="{X(ex)}" y2="{Y(ey)}" stroke="{col}" stroke-width="1.6"/><circle cx="{X(x)}" cy="{Y(y)}" r="1.5" fill="{col}"/>')
        for p in (extra_pts or {}).get(n, []):
            out.append(f'<circle cx="{X(p[0])}" cy="{Y(p[1])}" r="2.2" fill="none" stroke="#090"/>')
    out.append("</svg>")
    return "\n".join(out)
