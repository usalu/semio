"""NW-V2 scratch library: coordinate helpers for the hand-designed plant assets (rest pose in world metres -> bone-local shapes).
Every shape, number and text is designed in the per-asset scripts; this module only converts and serialises."""
import json, math, os
from math import sin, cos, radians, atan2, hypot, pi

OUT = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/NW-V2"
CATALOG = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation"


def T(en, de):
    return {"en": en, "de": de}


def r4(v):
    v = round(float(v), 4)
    return 0.0 if v == 0 else v


class Bones:
    def __init__(s):
        s.list = []
        s.w = {}

    def add(s, id, parent, title, wx, wy, wang, length, joint, mass=None, section=None):
        if parent is None:
            lx, ly, la = wx, wy, wang
        else:
            px, py, pa, _ = s.w[parent]
            dx, dy = wx - px, wy - py
            c, sn = cos(radians(pa)), sin(radians(pa))
            lx, ly, la = c * dx + sn * dy, -sn * dx + c * dy, wang - pa
        s.w[id] = (wx, wy, wang, length)
        b = {"id": id, "parent": parent, "title": title, "x": r4(lx), "y": r4(ly), "angleDeg": r4(la), "lengthM": r4(length), "joint": joint}
        if mass:
            kg, src, ref = mass
            b["mass"] = {"kg": kg, "comX": r4(length / 2), "comY": 0, "distribution": "uniform", "source": src, **({"reference": ref} if ref else {})}
        if section:
            b["section"] = section
        s.list.append(b)

    def end(s, id):
        x, y, a, l = s.w[id]
        return (x + l * cos(radians(a)), y + l * sin(radians(a)))

    def loc(s, id, pts):
        x, y, a, _ = s.w[id]
        c, sn = cos(radians(a)), sin(radians(a))
        out = []
        for p in pts:
            dx, dy = p[0] - x, p[1] - y
            out.append((c * dx + sn * dy, -sn * dx + c * dy) + tuple(p[2:]))
        return out

    def glob(s, id, pts):
        x, y, a, _ = s.w[id]
        c, sn = cos(radians(a)), sin(radians(a))
        return [(x + c * p[0] - sn * p[1], y + sn * p[0] + c * p[1]) for p in pts]

    def anchor_local(s, id, wx, wy):
        return s.loc(id, [(wx, wy)])[0]


def revolute(lo, hi):
    return {"type": "revolute", "minDeg": lo, "maxDeg": hi}


FIXED = {"type": "fixed"}


def fmt(pts):
    return " ".join(f"{r4(p[0])} {r4(p[1])}" for p in pts)


def catmull(pts, closed, k=1.0):
    """pts: (x,y) or (x,y,'s') sharp. Returns SVG path d (absolute, cubic)."""
    n = len(pts)
    P = [(p[0], p[1]) for p in pts]
    sharp = [len(p) > 2 and p[2] == "s" for p in pts]

    def get(i):
        if closed:
            return P[i % n]
        return P[max(0, min(n - 1, i))]

    segs = [f"M{r4(P[0][0])} {r4(P[0][1])}"]
    last = n if closed else n - 1
    for i in range(last):
        p0, p1, p2, p3 = get(i - 1), get(i), get(i + 1), get(i + 2)
        a = (0, 0) if sharp[i % n] else ((p2[0] - p0[0]) / 6 * k, (p2[1] - p0[1]) / 6 * k)
        b = (0, 0) if sharp[(i + 1) % n] else ((p3[0] - p1[0]) / 6 * k, (p3[1] - p1[1]) / 6 * k)
        c1 = (p1[0] + a[0], p1[1] + a[1])
        c2 = (p2[0] - b[0], p2[1] - b[1])
        segs.append(f"C{r4(c1[0])} {r4(c1[1])} {r4(c2[0])} {r4(c2[1])} {r4(p2[0])} {r4(p2[1])}")
    if closed:
        segs.append("Z")
    return "".join(segs)


def shape(id, bone, role, z, filled, tone, geom, region=None, regions=None, material=None, hatch=None, pen=None):
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, **geom}
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


class Draw:
    """Collects the shapes of one representation; points are given in world rest metres."""

    def __init__(s, bones):
        s.b = bones
        s.shapes = []

    def poly(s, id, bone, role, z, filled, tone, pts, closed=True, **kw):
        l = s.b.loc(bone, pts)
        s.shapes.append(shape(id, bone, role, z, filled, tone, {"kind": "poly", "points": [[r4(p[0]), r4(p[1])] for p in l], "closed": closed}, **kw))

    def smooth(s, id, bone, role, z, filled, tone, pts, closed=True, k=1.0, **kw):
        l = s.b.loc(bone, pts)
        s.shapes.append(shape(id, bone, role, z, filled, tone, {"kind": "path", "d": catmull(l, closed, k)}, **kw))

    def ellipse(s, id, bone, role, z, filled, tone, cx, cy, rx, ry, rot=0, **kw):
        (lx, ly), = s.b.loc(bone, [(cx, cy)])
        g = {"kind": "ellipse", "cx": r4(lx), "cy": r4(ly), "rx": r4(rx), "ry": r4(ry)}
        a = s.b.w[bone][2]
        if rot - a:
            g["rotationDeg"] = r4(rot - a)
        s.shapes.append(shape(id, bone, role, z, filled, tone, g, **kw))


def bounds_world(bones, shapes):
    xs, ys = [], []
    for sh in shapes:
        ox, oy, a, _ = bones.w[sh["bone"]]
        c, sn = cos(radians(a)), sin(radians(a))
        pts = []
        if sh["kind"] == "poly":
            pts = sh["points"]
        elif sh["kind"] == "path":
            pts = path_points(sh["d"])
        elif sh["kind"] == "ellipse":
            rot = radians(sh.get("rotationDeg", 0))
            for i in range(48):
                t = 2 * pi * i / 48
                ex, ey = sh["rx"] * cos(t), sh["ry"] * sin(t)
                pts.append((sh["cx"] + ex * cos(rot) - ey * sin(rot), sh["cy"] + ex * sin(rot) + ey * cos(rot)))
        elif sh["kind"] == "rect":
            pts = [(sh["x"], sh["y"]), (sh["x"] + sh["w"], sh["y"] + sh["h"])]
        for p in pts:
            xs.append(ox + c * p[0] - sn * p[1])
            ys.append(oy + sn * p[0] + c * p[1])
    return min(xs), max(xs), min(ys), max(ys)


def path_points(d):
    import re
    toks = re.findall(r"[MCLZ]|-?\d*\.?\d+(?:e-?\d+)?", d)
    out, i, cur = [], 0, (0, 0)
    while i < len(toks):
        t = toks[i]
        if t in "ML":
            cur = (float(toks[i + 1]), float(toks[i + 2]))
            out.append(cur)
            i += 3
        elif t == "C":
            v = [float(x) for x in toks[i + 1:i + 7]]
            p0, p1, p2, p3 = cur, (v[0], v[1]), (v[2], v[3]), (v[4], v[5])
            for k in range(1, 9):
                u = k / 8
                out.append(tuple((1 - u) ** 3 * p0[j] + 3 * (1 - u) ** 2 * u * p1[j] + 3 * (1 - u) * u * u * p2[j] + u ** 3 * p3[j] for j in range(2)))
            cur = p3
            i += 7
        else:
            i += 1
    return out


def lancet(base, tip, width, n=6, widest=0.4, bend=0.0, sharp_base=True):
    """Closed point loop of a lanceolate leaf from base to tip (world metres); bend = sideways bow of the midrib as a fraction of the length."""
    bx, by = base
    tx, ty = tip
    L = hypot(tx - bx, ty - by)
    ux, uy = (tx - bx) / L, (ty - by) / L
    nx, ny = -uy, ux
    p = math.log(0.5) / math.log(widest)
    left, right = [], []
    for i in range(n + 1):
        t = i / n
        hw = width / 2 * (sin(pi * t ** p)) ** 0.85
        off = bend * L * sin(pi * t)
        cx, cy = bx + ux * L * t + nx * off, by + uy * L * t + ny * off
        left.append((cx + nx * hw, cy + ny * hw))
        right.append((cx - nx * hw, cy - ny * hw))
    pts = [(bx, by, "s") if sharp_base else (bx, by)]
    pts += left[1:-1]
    pts.append((left[-1][0], left[-1][1], "s"))
    pts += right[-2:0:-1]
    return pts


def quad(p0, p1, w0, w1):
    dx, dy = p1[0] - p0[0], p1[1] - p0[1]
    L = hypot(dx, dy)
    nx, ny = -dy / L, dx / L
    return [(p0[0] + nx * w0, p0[1] + ny * w0), (p1[0] + nx * w1, p1[1] + ny * w1), (p1[0] - nx * w1, p1[1] - ny * w1), (p0[0] - nx * w0, p0[1] - ny * w0)]


def lerp(a, b, t):
    return (a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t)


def along(poly_pts, t):
    """Point at fraction t of a polyline."""
    segs = [hypot(poly_pts[i + 1][0] - poly_pts[i][0], poly_pts[i + 1][1] - poly_pts[i][1]) for i in range(len(poly_pts) - 1)]
    tot = sum(segs)
    d = t * tot
    for i, sl in enumerate(segs):
        if d <= sl or i == len(segs) - 1:
            u = d / sl if sl else 0
            return lerp(poly_pts[i], poly_pts[i + 1], min(1, u)), math.degrees(atan2(poly_pts[i + 1][1] - poly_pts[i][1], poly_pts[i + 1][0] - poly_pts[i][0]))
        d -= sl


def dump(doc, name):
    os.makedirs(OUT, exist_ok=True)
    path = os.path.join(OUT, name + ".json")
    with open(path, "w", encoding="utf8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
    return path


def install(doc, asset_id):
    folder = os.path.join(CATALOG, asset_id)
    os.makedirs(folder, exist_ok=True)
    path = os.path.join(folder, "🔣️.json")
    with open(path, "w", encoding="utf8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=2)
        f.write("\n")
    return path


def document(asset):
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


REFERENCES = {
    "jones-2014": "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, boundary-layer and stomatal conductance.",
    "monteith-unsworth-2013": "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration.",
    "en-1991-1-4": "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Dynamic pressure q = 0.5 rho v^2 with air density 1.25 kg/m3, force F = cf q A.",
    "en-1991-1-3": "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads (order of magnitude of snow and ice accretion on plan area).",
    "vogel-1989": "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948.",
    "liese-1985": "Liese, W. (1985). Bamboos - Biology, Silvics, Properties, Utilization. GTZ Schriftenreihe 180, Eschborn. Culm anatomy, density 600-800 kg/m3, nodes and internodes.",
    "liese-koehl-2015": "Liese, W. and Koehl, M. (eds.) (2015). Bamboo - The Plant and its Uses. Springer, Tropical Forestry 10.",
    "nobel-1988": "Nobel, P. S. (1988). Environmental Biology of Agaves and Cacti. Cambridge University Press. Crassulacean acid metabolism, nocturnal stomatal opening, rosette geometry of Agave species.",
    "nobel-hartsock-1979": "Nobel, P. S. and Hartsock, T. L. (1979). Environmental influences on open stomates of a crassulacean acid metabolism plant, Agave deserti. Plant Physiology 63, 63-66.",
    "metcalfe-2005": "Metcalfe, D. J. (2005). Hedera helix L. Biological Flora of the British Isles no. 233. Journal of Ecology 93(3), 632-648.",
    "schmid-1991": "Schmid, W. G. (1991). The Genus Hosta: Giboshi Zoku. Timber Press, Portland.",
    "en-338": "EN 338:2016 Structural timber, strength classes (C24: mean density 420 kg/m3, E0,mean 11 GPa, f_m,k 24 MPa).",
    "en-10025-2": "EN 10025-2:2019 Hot rolled products of structural steels, non-alloy steels (S235: f_y 235 MPa, E 210 GPa, density 7850 kg/m3).",
    "en-iso-6946": "EN ISO 6946:2017 Building components and building elements, thermal resistance and thermal transmittance (surface resistances, material conductivities).",
    "wood-handbook": "Forest Products Laboratory (2021). Wood Handbook, Wood as an Engineering Material, General Technical Report FPL-GTR-282. Specific heat and thermal conductivity of wood.",
}


def provenance(ids, notes):
    return {"author": "Semio staffage vegetation content worker NW-V2", "method": "handcrafted", "created": "2026-10-29", "references": [{"id": i, "citation": REFERENCES[i]} for i in ids], "notes": notes}


def wind_force(cf, area, v):
    return round(0.5 * 1.25 * cf * area * v * v, 3)


def clamp(v, lo, hi):
    return max(lo, min(hi, v))


def sway_clip(bones_list, id, title, desc, duration, base, specs, n=12, fps=30, extra_tracks=None, events=None):
    """specs: bone -> (amplitude deg, phase deg, harmonic). base: bone -> base angle (pose offset). Periodic by construction (integer harmonics)."""
    lim = {b["id"]: (b["joint"].get("minDeg", -1e9), b["joint"].get("maxDeg", 1e9)) for b in bones_list}
    keys = []
    for k in range(n + 1):
        t = round(duration * k / n, 4)
        kk = k % n
        ov = {}
        for b, (amp, ph, h) in specs.items():
            a = base.get(b, 0) + amp * sin(2 * pi * h * kk / n + radians(ph))
            ov[b] = {"angleDeg": r4(clamp(a, *lim[b]))}
        keys.append({"t": t, "pose": "__POSE__", "ease": "smooth", "bones": ov})
    return {"id": id, "title": title, "description": desc, "durationS": duration, "fps": fps, "loop": True, "rootMotion": "none", "keys": keys, "contacts": [], "phases": [], "events": events or [], "tracks": extra_tracks or []}


def world_rings(bones, sh):
    ox, oy, a, _ = bones.w[sh["bone"]]
    c, sn = cos(radians(a)), sin(radians(a))
    if sh["kind"] == "poly":
        pts = [tuple(p) for p in sh["points"]]
    elif sh["kind"] == "path":
        pts = path_points(sh["d"])
    elif sh["kind"] == "ellipse":
        rot = radians(sh.get("rotationDeg", 0))
        pts = []
        for i in range(48):
            t = 2 * pi * i / 48
            ex, ey = sh["rx"] * cos(t), sh["ry"] * sin(t)
            pts.append((sh["cx"] + ex * cos(rot) - ey * sin(rot), sh["cy"] + ex * sin(rot) + ey * cos(rot)))
    elif sh["kind"] == "rect":
        x, y, w, h = sh["x"], sh["y"], sh["w"], sh["h"]
        pts = [(x, y), (x + w, y), (x + w, y + h), (x, y + h)]
    else:
        pts = []
    return [(ox + c * p[0] - sn * p[1], oy + sn * p[0] + c * p[1]) for p in pts]


def quick_svg(bones, shapes, pens, scale, name, view=None, mm_scale=None):
    """Rest-pose sketch at paper scale 1:scale (px per mm = ppm)."""
    ppm = mm_scale or 8
    k = 1000 / scale * ppm
    rings = [(sh, world_rings(bones, sh)) for sh in shapes]
    xs = [p[0] for _, r in rings for p in r]
    ys = [p[1] for _, r in rings for p in r]
    x0, x1, y0, y1 = (view or (min(xs) - 0.1, max(xs) + 0.1, -0.05, max(ys) + 0.1))
    W, H = (x1 - x0) * k, (y1 - y0) * k
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W:.0f}" height="{H:.0f}" viewBox="0 0 {W:.1f} {H:.1f}"><rect width="100%" height="100%" fill="white"/>']
    for sh, r in sorted(rings, key=lambda t: t[0]["z"]):
        pen = sh.get("penMm") or pens.get(sh["role"]) or 0.1
        pts = " ".join(f"{(p[0]-x0)*k:.1f},{(y1-p[1])*k:.1f}" for p in r)
        closed = sh.get("filled") is not None and (sh["kind"] != "poly" or sh["closed"]) and not (sh["kind"] == "path" and not sh["d"].endswith("Z"))
        fill = f'rgb({int(255*(1-sh["tone"]))},{int(255*(1-sh["tone"]))},{int(255*(1-sh["tone"]))})' if sh["filled"] else "none"
        tag = "polygon" if closed else "polyline"
        out.append(f'<{tag} points="{pts}" fill="{fill}" stroke="black" stroke-width="{pen*ppm:.2f}" stroke-linejoin="round"/>')
    out.append("</svg>")
    p = os.path.join(OUT, name + ".svg")
    open(p, "w").write("\n".join(out))
    os.system(f'rsvg-convert -f png -o "{os.path.join(OUT, name)}.png" "{p}"')
    return p
