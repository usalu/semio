"""🏗️ The one authoritative chisel of the asset `reclaimed-steel-i-beam`: states the hand-chosen numbers and writes `🔣️.json` next to it.

Run: `python3 author-reclaimed-steel-i-beam.py` (prints the [DEBUG] reference measurements, writes the document).
@see ./author-geometry.py — the INP 240 section, cleat and bearer numbers
@see ../../brief.md — the asset contract of the collection
"""
import json
import math
import os
import importlib.util

HERE = os.path.dirname(os.path.abspath(__file__))


def load(name, file):
    """📥️ Loads a sibling author module by its file name."""
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, file))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


G = load("geometry", "author-geometry.py")
TEXT = load("text", "author-text.py").TEXT


def tx(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def shape(id, bone, role, z, filled, tone, geom, region=None, regions=None, material=None, hatch=None, pen=None):
    """✏️ One 2D shape of a representation."""
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    if pen is not None:
        s["penMm"] = pen
    s.update(geom)
    return s


def rect(x, y, w, h):
    """⬛️ Rectangle geometry."""
    return {"kind": "rect", "x": round(x, 6), "y": round(y, 6), "w": round(w, 6), "h": round(h, 6)}


def poly(points, closed=True):
    """🔷️ Polygon geometry."""
    return {"kind": "poly", "points": G.rnd(points), "closed": closed}


def path(rings):
    """〰️ Path geometry of closed rings (even-odd)."""
    d = " ".join("M " + " L ".join(f"{p[0]:.6f} {p[1]:.6f}" for p in ring) + " Z" for ring in rings)
    return {"kind": "path", "d": d}


def ellipse(cx, cy, r):
    """⭕️ Circle geometry."""
    return {"kind": "ellipse", "cx": round(cx, 6), "cy": round(cy, 6), "rx": r, "ry": r}


def solid(id, bone, material, tone, geom, role="body", **extra):
    """🧊️ One volume solid."""
    s = {"id": id, "kind": geom["kind"], "bone": bone, "material": material, "role": role, "tone": tone}
    s.update({k: v for k, v in geom.items() if k != "kind"})
    s.update(extra)
    return s


def box(size, at):
    """📦️ Centred box."""
    return {"kind": "box", "size": [round(a, 6) for a in size], "transform": {"translation": [round(a, 6) for a in at]}}


def extrude(profile, plane, a, b, holes=None):
    """🧱️ Extrude of a profile on a plane between a and b."""
    g = {"kind": "extrude", "profile": G.rnd(profile), "plane": plane, "from": round(a, 6), "to": round(b, 6)}
    if holes:
        g["holes"] = [G.rnd(h) for h in holes]
    return g


def vw(ring):
    """🔁️ (w, v) points to (v, w) profile points of a yz extrude."""
    return [(v, w) for (w, v) in ring]


def flip(ring):
    """🪞️ Mirror (w, v) points to the other side, w -> -w, keeping the ring order valid."""
    return [(-w, v) for (w, v) in reversed(ring)]


STEEL = "steel-rusted"
TIMBER = "softwood-weathered"
SIDES = [("r", -1), ("l", 1)]


def web_profile(t_flange_top, cut):
    """🪟️ The web plate between the flange tangents in the (u, v) plane, with the slanted flame cut at u = 0."""
    top = t_flange_top
    return [(0.0, -top), (G.L, -top), (G.L, top), (cut, top)]


def holes_uv():
    """🕳️ The four old bolt holes of the web in (u, v)."""
    return [G.circle(x, y, G.WEB_HOLE_R) for (x, y) in G.WEB_HOLES]


def strip_cut(y0, y1):
    """✂️ The u position of the flame cut at heights y0, y1 of the web (linear from 0 at -Y_T to CUT at +Y_T)."""
    f = lambda y: G.CUT * (min(max(y, -G.Y_T), G.Y_T) + G.Y_T) / (2 * G.Y_T)
    return f(y0), f(y1)


def rings_bearer_rings():
    """🪵️ Growth-ring arcs of a bearer end grain in bearer-local (u, v), pith outside the section below left."""
    pith = (-0.015, -0.075)
    out = []
    for r in [0.045, 0.065, 0.085, 0.105, 0.125]:
        pts = []
        for k in range(41):
            a = math.radians(0 + 90 * k / 40)
            p = (pith[0] + r * math.cos(a), pith[1] + r * math.sin(a))
            if 0.004 <= p[0] <= 0.096 and -0.046 <= p[1] <= 0.046:
                pts.append(p)
        if len(pts) >= 2:
            out.append(pts)
    return out


def representation_50():
    """🖋️ The 1:50 drawings: true tapered section with fillets, flame-cut end, riveted cleat, old bolt holes, rust stipple, bearers with end grain."""
    hatches = [{"id": "rust", "title": tx("Rust patina", "Rostpatina"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.1}]
    el = []
    toe = G.inner(G.FLANGE)
    for name, sgn, region in [("top", 1, "top-flange"), ("bottom", -1, "bottom-flange")]:
        x0 = G.CUT if sgn > 0 else 0.0
        el.append(shape(f"{name}-flange-edge", "beam", "body", 2.2, True, 0.42, poly([(x0, sgn * toe), (G.L, sgn * toe), (G.L, sgn * G.HALF), (x0, sgn * G.HALF)]), region=region, material=STEEL, pen=0.25))
        el.append(shape(f"{name}-flange-underside", "beam", "body", 2.1, True, 0.26, poly([(x0, sgn * G.Y_T), (G.L, sgn * G.Y_T), (G.L, sgn * toe), (x0, sgn * toe)]), region=region, material=STEEL, pen=0.18))
    outer = [(0.0, -G.Y_T), (G.L, -G.Y_T), (G.L, G.Y_T), (G.CUT, G.Y_T)]
    el.append(shape("web-face", "beam", "body", 2.0, True, 0.55, path([outer] + holes_uv()), region="web", material=STEEL, hatch="rust", pen=0.18))
    el.append(shape("flame-cut-edge", "beam", "detail", 4.0, False, 0.2, poly([(0.0, -G.HALF), (0.0, -G.Y_T), (G.CUT, G.Y_T), (G.CUT, G.HALF)], closed=False), region="web"))
    el.append(shape("heat-tint", "beam", "texture", 4.1, False, 0.3, poly([(0.018, -G.Y_T), (0.028, G.Y_T)], closed=False), region="web"))
    for (x, y) in G.WEB_HOLES:
        el.append(shape(f"bolt-hole-rim-{x:.2f}-{'a' if y < 0 else 'b'}".replace(".", "-"), "beam", "texture", 4.2, False, 0.2, ellipse(x + 0.003, y - 0.003, G.WEB_HOLE_R), region="web"))
    el.append(shape("cleat-leg", "cleat", "body", 3.0, True, 0.40, rect(0.0, -G.CLEAT_LEN / 2, G.CLEAT_END, G.CLEAT_LEN), region="end-cleat", material=STEEL))
    el.append(shape("cleat-heel", "cleat", "detail", 3.2, False, 0.2, poly([(G.CLEAT_LEG, -G.CLEAT_LEN / 2), (G.CLEAT_LEG, G.CLEAT_LEN / 2)], closed=False), region="end-cleat"))
    for k, v in enumerate([-G.RIVET_V, G.RIVET_V]):
        el.append(shape(f"rivet-head-{k}", "cleat", "detail", 3.5, True, 0.28, ellipse(G.RIVET_X, v, G.RIVET_HEAD_R), region="end-cleat", material=STEEL))
    for i, x in enumerate(G.BEARER_X):
        b = f"bearer-{'ab'[i]}"
        el.append(shape(f"{b}-end", b, "body", 1.0, True, 0.72, rect(0.0, -G.BEARER / 2, G.BEARER, G.BEARER), region="bearers", material=TIMBER))
        for j, pts in enumerate(rings_bearer_rings()):
            el.append(shape(f"{b}-ring-{j}", b, "texture", 1.5, False, 0.4, poly(pts, closed=False), region="bearers"))
        el.append(shape(f"{b}-check", b, "detail", 1.6, False, 0.3, poly([(0.05, 0.05), (0.058, 0.02), (0.055, 0.0)], closed=False), region="bearers"))
    fr = []
    sec = [(w, v + G.BEAM_Y) for (w, v) in G.section_ring()]
    fr.append(shape("beam-end", "beam", "body", 2.0, True, 0.48, poly(sec), regions=["top-flange", "web", "bottom-flange"], material=STEEL))
    for name, sgn in SIDES:
        leg = [(sgn * G.WEB, G.BEAM_Y - G.CLEAT_LEN / 2), (sgn * (G.WEB + G.CLEAT_OUT), G.BEAM_Y - G.CLEAT_LEN / 2), (sgn * (G.WEB + G.CLEAT_OUT), G.BEAM_Y + G.CLEAT_LEN / 2), (sgn * G.WEB, G.BEAM_Y + G.CLEAT_LEN / 2)]
        holes = [G.circle(sgn * G.CLEAT_HOLE_W, G.BEAM_Y + v, G.CLEAT_HOLE_R) for v in (-G.RIVET_V, G.RIVET_V)]
        fr.append(shape(f"cleat-outstanding-{name}", "cleat", "body", 3.0, True, 0.38, path([leg] + holes), region="end-cleat", material=STEEL))
    for i in range(2):
        b = f"bearer-{'ab'[i]}"
        fr.append(shape(f"{b}-side", b, "body", 1.0 + i * 0.1, True, 0.70, rect(-G.BEARER_LEN / 2, 0.0, G.BEARER_LEN, G.BEARER), region="bearers", material=TIMBER))
    for k, y in enumerate([0.028, 0.061, 0.083]):
        fr.append(shape(f"bearer-b-grain-{k}", "bearer-b", "texture", 1.5, False, 0.4, poly([(-0.29, y), (-0.12, y + 0.004), (0.06, y - 0.003), (0.29, y + 0.002)], closed=False), region="bearers"))
    pl = []
    pl.append(shape("top-flange-top", "beam", "body", 2.0, True, 0.50, rect(0.0, -G.FLANGE, G.L, G.B), region="top-flange", material=STEEL, hatch="rust"))
    pl.append(shape("flame-cut-step", "beam", "detail", 3.0, False, 0.2, poly([(G.CUT, -G.FLANGE), (G.CUT, G.FLANGE)], closed=False), region="bottom-flange"))
    pl.append(shape("web-line", "beam", "texture", 3.1, False, 0.3, poly([(0.05, 0.0), (G.L - 0.05, 0.0)], closed=False), region="web"))
    pl.append(shape("cleat-outstanding-plan", "cleat", "body", 1.5, True, 0.38, rect(G.CLEAT_X + G.CLEAT_LEG, -(G.WEB + G.CLEAT_OUT), G.CLEAT_T, 2 * (G.WEB + G.CLEAT_OUT)), region="end-cleat", material=STEEL))
    for i, x in enumerate(G.BEARER_X):
        b = f"bearer-{'ab'[i]}"
        pl.append(shape(f"{b}-top", b, "body", 1.0, True, 0.70, rect(x, -G.BEARER_LEN / 2, G.BEARER, G.BEARER_LEN), region="bearers", material=TIMBER))
        for k, dx in enumerate([0.022, 0.061, 0.083]):
            pl.append(shape(f"{b}-grain-{k}", b, "texture", 1.2, False, 0.4, poly([(x + dx, -0.29), (x + dx + 0.003, -0.10), (x + dx - 0.002, 0.10), (x + dx + 0.002, 0.29)], closed=False), region="bearers"))
    return {"status": "authored", "denominator": 50, "validRange": {"minDenominator": 20, "maxDenominator": 75}, "pens": {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5}, "hatches": hatches, "geometry": {"elevation": el, "front": fr, "plan": pl}, "decisions": TEXT["rep50"]}


def i_ring(tf):
    """🅸️ The generalised I outline with parallel flanges of thickness tf, (w, v)."""
    a, w, f = G.HALF, G.WEB, G.FLANGE
    return [(-f, a), (f, a), (f, a - tf), (w, a - tf), (w, -(a - tf)), (f, -(a - tf)), (f, -a), (-f, -a), (-f, -(a - tf)), (-w, -(a - tf)), (-w, a - tf), (-f, a - tf)]


def representation_100():
    """🖋️ The 1:100 drawings: parallel 15 mm flanges, the web with its four holes as dark dots, cleat, bearers."""
    tf = 0.015
    hatches = [{"id": "rust", "title": tx("Rust patina", "Rostpatina"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.1}]
    el = [
        shape("top-flange-edge", "beam", "body", 2.2, True, 0.40, rect(0.0, G.HALF - tf, G.L, tf), region="top-flange", material=STEEL, pen=0.18),
        shape("bottom-flange-edge", "beam", "body", 2.2, True, 0.40, rect(0.0, -G.HALF, G.L, tf), region="bottom-flange", material=STEEL, pen=0.18),
        shape("web-face", "beam", "body", 2.0, True, 0.55, rect(0.0, -(G.HALF - tf), G.L, 2 * (G.HALF - tf)), region="web", material=STEEL, hatch="rust", pen=0.18),
    ]
    for (x, y) in G.WEB_HOLES:
        el.append(shape(f"bolt-hole-{x:.2f}-{'a' if y < 0 else 'b'}".replace(".", "-"), "beam", "detail", 3.0, True, 0.12, ellipse(x, y, G.WEB_HOLE_R), region="web", material=STEEL, pen=0.1))
    el.append(shape("cleat-leg", "cleat", "body", 3.0, True, 0.40, rect(0.0, -G.CLEAT_LEN / 2, G.CLEAT_END, G.CLEAT_LEN), region="end-cleat", material=STEEL, pen=0.18))
    for k, v in enumerate([-G.RIVET_V, G.RIVET_V]):
        el.append(shape(f"rivet-head-{k}", "cleat", "detail", 3.5, False, 0.28, ellipse(G.RIVET_X, v, G.RIVET_HEAD_R), region="end-cleat"))
    for i in range(2):
        b = f"bearer-{'ab'[i]}"
        el.append(shape(f"{b}-end", b, "body", 1.0, True, 0.72, rect(0.0, -G.BEARER / 2, G.BEARER, G.BEARER), region="bearers", material=TIMBER))
        el.append(shape(f"{b}-ring", b, "texture", 1.5, False, 0.4, poly(rings_bearer_rings()[1], closed=False), region="bearers"))
    fr = [shape("beam-end", "beam", "body", 2.0, True, 0.48, poly([(w, v + G.BEAM_Y) for (w, v) in i_ring(tf)]), regions=["top-flange", "web", "bottom-flange"], material=STEEL)]
    for name, sgn in SIDES:
        fr.append(shape(f"cleat-outstanding-{name}", "cleat", "body", 3.0, True, 0.38, rect(min(sgn * G.WEB, sgn * (G.WEB + G.CLEAT_OUT)), G.BEAM_Y - G.CLEAT_LEN / 2, G.CLEAT_OUT, G.CLEAT_LEN), region="end-cleat", material=STEEL, pen=0.18))
        for v in (-G.RIVET_V, G.RIVET_V):
            fr.append(shape(f"cleat-hole-{name}-{'a' if v < 0 else 'b'}", "cleat", "detail", 3.2, False, 0.2, ellipse(sgn * G.CLEAT_HOLE_W, G.BEAM_Y + v, G.CLEAT_HOLE_R), region="end-cleat"))
    for i in range(2):
        b = f"bearer-{'ab'[i]}"
        fr.append(shape(f"{b}-side", b, "body", 1.0 + i * 0.1, True, 0.70, rect(-G.BEARER_LEN / 2, 0.0, G.BEARER_LEN, G.BEARER), region="bearers", material=TIMBER))
    pl = [
        shape("top-flange-top", "beam", "body", 2.0, True, 0.50, rect(0.0, -G.FLANGE, G.L, G.B), regions=["top-flange", "bottom-flange", "web"], material=STEEL, hatch="rust"),
        shape("cleat-outstanding-plan", "cleat", "body", 1.5, True, 0.38, rect(G.CLEAT_X + G.CLEAT_END - 0.015, -(G.WEB + G.CLEAT_OUT), 0.015, 2 * (G.WEB + G.CLEAT_OUT)), region="end-cleat", material=STEEL, pen=0.18),
    ]
    for i, x in enumerate(G.BEARER_X):
        b = f"bearer-{'ab'[i]}"
        pl.append(shape(f"{b}-top", b, "body", 1.0, True, 0.70, rect(x, -G.BEARER_LEN / 2, G.BEARER, G.BEARER_LEN), region="bearers", material=TIMBER))
    return {"status": "authored", "denominator": 100, "validRange": {"minDenominator": 75, "maxDenominator": 150}, "pens": {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35}, "hatches": hatches, "geometry": {"elevation": el, "front": fr, "plan": pl}, "decisions": TEXT["rep100"]}


def representation_200():
    """🖋️ The 1:200 drawings: a bar with 30 mm flange bands, the cleat as one block, bearers."""
    tf = 0.030
    cl = 0.090
    el = [
        shape("beam-side", "beam", "body", 2.0, True, 0.52, rect(0.0, -G.HALF, G.L, G.H), regions=["web", "top-flange", "bottom-flange"], material=STEEL),
        shape("top-flange-line", "beam", "detail", 2.5, False, 0.3, poly([(0.0, G.HALF - tf), (G.CLEAT_X, G.HALF - tf)], closed=False), region="top-flange"),
        shape("bottom-flange-line", "beam", "detail", 2.5, False, 0.3, poly([(0.0, -(G.HALF - tf)), (G.CLEAT_X, -(G.HALF - tf))], closed=False), region="bottom-flange"),
        shape("cleat-block", "cleat", "body", 3.0, True, 0.36, rect(0.0, -cl, G.CLEAT_END, 2 * cl), region="end-cleat", material=STEEL, pen=0.13),
    ]
    for i in range(2):
        b = f"bearer-{'ab'[i]}"
        el.append(shape(f"{b}-end", b, "body", 1.0, True, 0.72, rect(0.0, -G.BEARER / 2, G.BEARER, G.BEARER), region="bearers", material=TIMBER))
    fr = [
        shape("beam-end", "beam", "body", 2.0, True, 0.45, poly([(w, v + G.BEAM_Y) for (w, v) in i_ring(tf)]), regions=["top-flange", "web", "bottom-flange"], material=STEEL),
        shape("cleat-block-end", "cleat", "body", 3.0, True, 0.36, rect(-(G.WEB + G.CLEAT_OUT), G.BEAM_Y - cl, 2 * (G.WEB + G.CLEAT_OUT), 2 * cl), region="end-cleat", material=STEEL, pen=0.13),
    ]
    for i in range(2):
        b = f"bearer-{'ab'[i]}"
        fr.append(shape(f"{b}-side", b, "body", 1.0 + i * 0.1, True, 0.70, rect(-G.BEARER_LEN / 2, 0.0, G.BEARER_LEN, G.BEARER), region="bearers", material=TIMBER))
    pl = [
        shape("top-flange-top", "beam", "body", 2.0, True, 0.50, rect(0.0, -G.FLANGE, G.L, G.B), regions=["top-flange", "bottom-flange", "web"], material=STEEL),
        shape("cleat-block-plan", "cleat", "body", 1.5, True, 0.36, rect(G.CLEAT_X, -(G.WEB + G.CLEAT_OUT), G.CLEAT_END, 2 * (G.WEB + G.CLEAT_OUT)), region="end-cleat", material=STEEL, pen=0.13),
    ]
    for i, x in enumerate(G.BEARER_X):
        b = f"bearer-{'ab'[i]}"
        pl.append(shape(f"{b}-top", b, "body", 1.0, True, 0.70, rect(x, -G.BEARER_LEN / 2, G.BEARER, G.BEARER_LEN), region="bearers", material=TIMBER))
    return {"status": "authored", "denominator": 200, "validRange": {"minDenominator": 150, "maxDenominator": 350}, "pens": {"body": 0.18, "detail": 0.1, "texture": None, "glazing": 0.1, "ground": 0.25}, "hatches": [], "geometry": {"elevation": el, "front": fr, "plan": pl}, "decisions": TEXT["rep200"]}


def representation_500():
    """🖋️ The 1:500 drawings: one bar with the cleat in its outline, two bearer blocks."""
    total = G.CLEAT_X + G.CLEAT_END
    el = [shape("beam", "beam", "body", 2.0, True, 0.40, rect(0.0, -G.HALF, total, G.H), regions=["top-flange", "web", "bottom-flange", "end-cleat"], material=STEEL)]
    fr = [shape("beam-end", "beam", "body", 2.0, True, 0.30, rect(-G.FLANGE, G.BEARER, G.B, G.H), regions=["top-flange", "web", "bottom-flange", "end-cleat"], material=STEEL)]
    pl = [shape("beam-top", "beam", "body", 2.0, True, 0.40, rect(0.0, -G.FLANGE, total, G.B), regions=["top-flange", "end-cleat"], material=STEEL)]
    for i, x in enumerate(G.BEARER_X):
        b = f"bearer-{'ab'[i]}"
        el.append(shape(f"{b}-end", b, "body", 1.0, True, 0.65, rect(0.0, -G.BEARER / 2, G.BEARER, G.BEARER), region="bearers", material=TIMBER))
        fr.append(shape(f"{b}-side", b, "body", 1.0 + i * 0.1, True, 0.62, rect(-G.BEARER_LEN / 2, 0.0, G.BEARER_LEN, G.BEARER), region="bearers", material=TIMBER))
        pl.append(shape(f"{b}-top", b, "body", 1.0, True, 0.65, rect(x, -G.BEARER_LEN / 2, G.BEARER, G.BEARER_LEN), region="bearers", material=TIMBER))
    return {"status": "authored", "denominator": 500, "validRange": {"minDenominator": 350, "maxDenominator": 1000}, "pens": {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}, "hatches": [], "geometry": {"elevation": el, "front": fr, "plan": pl}, "decisions": TEXT["rep500"]}


def bearers_body(tone):
    """🪵️ The two squared-timber bearers, 100 x 100 x 600 mm, on their own bones."""
    return [solid(f"bearer-{'ab'[i]}", f"bearer-{'ab'[i]}", TIMBER, tone, box([G.BEARER, G.BEARER, G.BEARER_LEN], [G.BEARER / 2, 0, 0])) for i in range(2)]


def rivet_heads(name, sgn):
    """🔘️ Two DIN 124 round rivet heads d 16 on the outside of one cleat leg, revolved about w."""
    prof = [[0.0, 0.0], [0.0, G.RIVET_HEAD_R]] + [[round(G.RIVET_HEAD_K * math.sin(math.pi / 2 * k / 6), 6), round(G.RIVET_HEAD_R * math.cos(math.pi / 2 * k / 6), 6)] for k in range(1, 7)]
    return {"kind": "revolve", "profile": prof, "axis": [0, 0, sgn], "origin": [G.RIVET_X, -G.RIVET_V, round(sgn * (G.WEB + G.CLEAT_T), 6)]}


def volume_50():
    """🧊️ 1:50: flanges with taper, fillets and toe radii as extrudes of the true section, the web as a plate with its slanted flame cut and four holes, the riveted cleat angles with fillet, holes and rivet heads, the bearers."""
    fr = G.flange_ring()
    body = [
        solid("top-flange", "beam", STEEL, 0.46, extrude(vw(fr), "yz", G.CUT, G.L)),
        solid("bottom-flange", "beam", STEEL, 0.44, extrude(vw([(w, -v) for (w, v) in reversed(fr)]), "yz", 0.0, G.L)),
        solid("web", "beam", STEEL, 0.52, extrude(web_profile(G.Y_T, G.CUT), "xy", -G.WEB, G.WEB, holes=holes_uv())),
    ]
    for name, sgn in SIDES:
        w0, w1 = sorted([sgn * G.WEB, sgn * (G.WEB + G.CLEAT_T)])
        body.append(solid(f"cleat-web-leg-{name}", "cleat", STEEL, 0.40, box([G.CLEAT_LEG, G.CLEAT_LEN, G.CLEAT_T], [G.CLEAT_LEG / 2, 0, (w0 + w1) / 2])))
        ow0, ow1 = sorted([sgn * G.WEB, sgn * (G.WEB + G.CLEAT_OUT)])
        leg = [(-G.CLEAT_LEN / 2, ow0), (G.CLEAT_LEN / 2, ow0), (G.CLEAT_LEN / 2, ow1), (-G.CLEAT_LEN / 2, ow1)]
        holes = [[(v + G.CLEAT_HOLE_R * math.cos(2 * math.pi * k / 16), sgn * G.CLEAT_HOLE_W + G.CLEAT_HOLE_R * math.sin(2 * math.pi * k / 16)) for k in range(16)] for v in (-G.RIVET_V, G.RIVET_V)]
        body.append(solid(f"cleat-outstanding-{name}", "cleat", STEEL, 0.38, extrude(leg, "yz", G.CLEAT_LEG, G.CLEAT_END, holes=holes)))
        wi = G.WEB + G.CLEAT_T
        ui = G.CLEAT_LEG
        rr = G.CLEAT_ROOT
        fil = [(wi, ui), (wi, ui - rr)] + G.arc((wi + rr, ui - rr), rr, math.pi, math.pi / 2, 5)[1:-1] + [(wi + rr, ui)]
        fil = [(sgn * w, u) for (w, u) in fil]
        if sgn < 0:
            fil = list(reversed(fil))
        body.append(solid(f"cleat-root-{name}", "cleat", STEEL, 0.38, extrude(fil, "zx", -G.CLEAT_LEN / 2, G.CLEAT_LEN / 2)))
        body.append(solid(f"rivet-heads-{name}", "cleat", STEEL, 0.30, rivet_heads(name, sgn), repeat={"count": 2, "translation": [0, round(2 * G.RIVET_V, 6), 0]}))
    body += bearers_body(0.70)
    thermal = [
        {"id": "th-top-flange", "of": "top-flange", "region": "top-flange"},
        {"id": "th-bottom-flange", "of": "bottom-flange", "region": "bottom-flange"},
        {"id": "th-web", "of": "web", "region": "web", "sectionFaces": ["+v", "-v"]},
    ]
    for name, _ in SIDES:
        for part in ["cleat-web-leg", "cleat-outstanding", "cleat-root", "rivet-heads"]:
            thermal.append({"id": f"th-{part}-{name}", "of": f"{part}-{name}", "region": "end-cleat"})
    thermal += [{"id": f"th-bearer-{c}", "of": f"bearer-{c}", "region": "bearers"} for c in "ab"]
    structure = [solid("member-beam", "beam", STEEL, 0.5, extrude(vw(G.section_ring()), "yz", G.CUT / 2, G.L), member="beam", part="member", slender=True)]
    for name, _ in SIDES:
        structure.append({"id": f"member-cleat-leg-{name}", "of": f"cleat-web-leg-{name}", "member": "cleat", "part": "member"})
        structure.append({"id": f"member-cleat-outstanding-{name}", "of": f"cleat-outstanding-{name}", "member": "cleat", "part": "member"})
    structure.append(solid("joint-rivets", "cleat", STEEL, 0.3, {"kind": "capsule", "p0": [G.RIVET_X, -G.RIVET_V, -(G.WEB + G.CLEAT_T)], "p1": [G.RIVET_X, -G.RIVET_V, G.WEB + G.CLEAT_T], "r0": G.RIVET_SHANK_R, "r1": G.RIVET_SHANK_R}, member="cleat", part="joint", repeat={"count": 2, "translation": [0, round(2 * G.RIVET_V, 6), 0]}))
    structure += [{"id": f"member-bearer-{c}", "of": f"bearer-{c}", "member": f"bearer-{c}", "part": "member"} for c in "ab"]
    structure.append(solid("joint-load-carrier", "load-carrier", STEEL, 0.3, box([0.1, 0.0075, G.B], [0, -0.00375, 0]), member="load-carrier", part="joint"))
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": TEXT["vol50"]}


def volume_100():
    """🧊️ 1:100: parallel flanges of 15 mm, the web as a declared 8.7 mm sheet with its four 22 mm holes, cleat angles with 15 mm legs, bearers."""
    tf = 0.015
    a = G.HALF
    body = [
        solid("top-flange", "beam", STEEL, 0.46, box([G.L, tf, G.B], [G.L / 2, a - tf / 2, 0])),
        solid("bottom-flange", "beam", STEEL, 0.44, box([G.L, tf, G.B], [G.L / 2, -(a - tf / 2), 0])),
        solid("web", "beam", STEEL, 0.52, extrude(web_profile(a - tf, 0.0), "xy", -G.WEB, G.WEB, holes=holes_uv()), sheet=True),
    ]
    t = 0.015
    for name, sgn in SIDES:
        ring = [(w, u) for (w, u) in [(G.WEB, 0.0), (G.WEB + t, 0.0), (G.WEB + t, G.CLEAT_END - t), (G.WEB + G.CLEAT_OUT, G.CLEAT_END - t), (G.WEB + G.CLEAT_OUT, G.CLEAT_END), (G.WEB, G.CLEAT_END)]]
        ring = [(sgn * w, u) for (w, u) in ring]
        if sgn < 0:
            ring = list(reversed(ring))
        body.append(solid(f"cleat-angle-{name}", "cleat", STEEL, 0.40, extrude(ring, "zx", -G.CLEAT_LEN / 2, G.CLEAT_LEN / 2)))
    body += bearers_body(0.70)
    thermal = [
        {"id": "th-top-flange", "of": "top-flange", "region": "top-flange"},
        {"id": "th-bottom-flange", "of": "bottom-flange", "region": "bottom-flange"},
        {"id": "th-web", "of": "web", "region": "web", "sectionFaces": ["+v", "-v"]},
    ] + [{"id": f"th-cleat-angle-{n}", "of": f"cleat-angle-{n}", "region": "end-cleat"} for n, _ in SIDES] + [{"id": f"th-bearer-{c}", "of": f"bearer-{c}", "region": "bearers"} for c in "ab"]
    structure = [solid("member-beam", "beam", STEEL, 0.5, extrude(vw(G.section_ring()), "yz", 0.0, G.L), member="beam", part="member", slender=True)]
    structure += [{"id": f"member-cleat-angle-{n}", "of": f"cleat-angle-{n}", "member": "cleat", "part": "member"} for n, _ in SIDES]
    structure.append(solid("joint-rivets", "cleat", STEEL, 0.3, box([0.016, 2 * G.RIVET_V + 0.016, 2 * (G.WEB + t)], [G.RIVET_X, 0, 0]), member="cleat", part="joint"))
    structure += [{"id": f"member-bearer-{c}", "of": f"bearer-{c}", "member": f"bearer-{c}", "part": "member"} for c in "ab"]
    structure.append(solid("joint-load-carrier", "load-carrier", STEEL, 0.3, box([0.1, 0.015, G.B], [0, -0.0075, 0]), member="load-carrier", part="joint"))
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": TEXT["vol100"]}


def volume_200():
    """🧊️ 1:200: 30 mm flange slabs, the web as a declared sheet without holes, the cleat as two blocks that fill the clear web depth, bearers."""
    tf = 0.030
    a = G.HALF
    cl = 0.090
    body = [
        solid("top-flange", "beam", STEEL, 0.46, box([G.L, tf, G.B], [G.L / 2, a - tf / 2, 0])),
        solid("bottom-flange", "beam", STEEL, 0.44, box([G.L, tf, G.B], [G.L / 2, -(a - tf / 2), 0])),
        solid("web", "beam", STEEL, 0.52, box([G.L, 2 * (a - tf), G.S], [G.L / 2, 0, 0]), sheet=True),
    ]
    for name, sgn in SIDES:
        body.append(solid(f"cleat-block-{name}", "cleat", STEEL, 0.38, box([G.CLEAT_END, 2 * cl, G.CLEAT_OUT], [G.CLEAT_END / 2, 0, sgn * (G.WEB + G.CLEAT_OUT / 2)])))
    body += bearers_body(0.70)
    thermal = [
        {"id": "th-top-flange", "of": "top-flange", "region": "top-flange"},
        {"id": "th-bottom-flange", "of": "bottom-flange", "region": "bottom-flange"},
        {"id": "th-web", "of": "web", "region": "web", "sectionFaces": ["+v", "-v"]},
    ] + [{"id": f"th-cleat-block-{n}", "of": f"cleat-block-{n}", "region": "end-cleat"} for n, _ in SIDES] + [{"id": f"th-bearer-{c}", "of": f"bearer-{c}", "region": "bearers"} for c in "ab"]
    structure = [solid("member-beam", "beam", STEEL, 0.5, extrude(vw(G.section_ring()), "yz", 0.0, G.L), member="beam", part="member", slender=True)]
    structure += [{"id": f"member-cleat-block-{n}", "of": f"cleat-block-{n}", "member": "cleat", "part": "member"} for n, _ in SIDES]
    structure += [{"id": f"member-bearer-{c}", "of": f"bearer-{c}", "member": f"bearer-{c}", "part": "member"} for c in "ab"]
    structure.append(solid("joint-load-carrier", "load-carrier", STEEL, 0.3, box([0.1, 0.03, G.B], [0, -0.015, 0]), member="load-carrier", part="joint"))
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": TEXT["vol200"]}


def volume_500():
    """🧊️ 1:500: the beam as one bar of 6008 x 240 x 106 mm with the cleat in its end, two bearer blocks; thermal as three slabs and an end block; the member keeps the true section."""
    total = G.CLEAT_X + G.CLEAT_END
    a = G.HALF
    body = [solid("beam", "beam", STEEL, 0.42, box([total, G.H, G.B], [total / 2, 0, 0]))] + bearers_body(0.66)
    end = 0.075
    xs = total - end
    thermal = [
        solid("th-top-slab", "beam", STEEL, 0.46, box([xs, 0.075, G.B], [xs / 2, a - 0.0375, 0]), region="top-flange", sectionFaces=["-v"]),
        solid("th-web-slab", "beam", STEEL, 0.52, box([xs, G.H - 0.15, G.B], [xs / 2, 0, 0]), region="web", sectionFaces=["+v", "-v"]),
        solid("th-bottom-slab", "beam", STEEL, 0.44, box([xs, 0.075, G.B], [xs / 2, -(a - 0.0375), 0]), region="bottom-flange", sectionFaces=["+v"]),
        solid("th-cleat-end", "cleat", STEEL, 0.38, box([end, G.H, G.B], [G.CLEAT_END - end / 2, 0, 0]), region="end-cleat"),
    ] + [{"id": f"th-bearer-{c}", "of": f"bearer-{c}", "region": "bearers"} for c in "ab"]
    structure = [
        solid("member-beam", "beam", STEEL, 0.5, extrude(vw(G.section_ring()), "yz", 0.0, G.L), member="beam", part="member", slender=True),
        solid("member-cleat", "cleat", STEEL, 0.38, box([G.CLEAT_END, G.CLEAT_LEN, G.B], [G.CLEAT_END / 2, 0, 0]), member="cleat", part="member"),
    ]
    structure += [{"id": f"member-bearer-{c}", "of": f"bearer-{c}", "member": f"bearer-{c}", "part": "member"} for c in "ab"]
    structure.append(solid("joint-load-carrier", "load-carrier", STEEL, 0.3, box([0.1, 0.075, G.B], [0, -0.0375, 0]), member="load-carrier", part="joint"))
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": TEXT["vol500"]}


def masses():
    """⚖️ Masses from the 1:50 solids: section area x length x density, minus holes and the flame-cut wedge; plus the cleat and the bearers."""
    af = abs(G.shoelace(G.flange_ring()))
    web_area = abs(G.shoelace(web_profile(G.Y_T, G.CUT))) - 4 * math.pi * G.WEB_HOLE_R ** 2
    beam_v = af * (G.L - G.CUT) + af * G.L + web_area * G.S
    beam = beam_v * G.RHO_STEEL
    angle = abs(G.shoelace(G.angle_ring(G.CLEAT_T))) * G.CLEAT_LEN - 2 * math.pi * G.CLEAT_HOLE_R ** 2 * G.CLEAT_T
    cap = math.pi * G.RIVET_HEAD_K * (3 * G.RIVET_HEAD_R ** 2 + G.RIVET_HEAD_K ** 2) / 6
    shank = math.pi * G.RIVET_SHANK_R ** 2 * (2 * (G.WEB + G.CLEAT_T))
    cleat = (2 * angle + 4 * cap + 2 * shank) * G.RHO_STEEL
    bearer = G.BEARER * G.BEARER * G.BEARER_LEN * G.RHO_TIMBER
    return {"beam": beam, "cleat": cleat, "bearer": bearer, "beam_v": beam_v, "af": af}


def areas():
    """🌡️ Physical outer areas of the thermal regions from the exact 1:50 geometry (internal contact strips between flange and web excluded)."""
    fr = G.flange_ring()
    pf = G.perimeter(fr)
    af = abs(G.shoelace(fr))
    top = pf * (G.L - G.CUT) + 2 * af
    bottom = pf * G.L + 2 * af
    web_h = 2 * G.Y_T
    holes = 4 * (2 * math.pi * G.WEB_HOLE_R * G.S - 2 * math.pi * G.WEB_HOLE_R ** 2)
    web = 2 * web_h * G.L + 2 * web_h * G.S + holes
    angle_p = G.perimeter(G.angle_ring(G.CLEAT_T))
    angle_a = abs(G.shoelace(G.angle_ring(G.CLEAT_T)))
    cleat_hole = 2 * (2 * math.pi * G.CLEAT_HOLE_R * G.CLEAT_T - 2 * math.pi * G.CLEAT_HOLE_R ** 2)
    cap = 2 * (math.pi * (G.RIVET_HEAD_R ** 2 + G.RIVET_HEAD_K ** 2))
    cleat = 2 * (angle_p * G.CLEAT_LEN + 2 * angle_a) + 2 * cleat_hole + 2 * cap
    bearers = 2 * (4 * G.BEARER * G.BEARER_LEN + 2 * G.BEARER ** 2)
    return {"top-flange": top, "bottom-flange": bottom, "web": web, "end-cleat": cleat, "bearers": bearers}


def main():
    """🏁️ Assembles the asset document and writes it."""
    m = masses()
    ar = areas()
    mass_total = m["beam"] + m["cleat"] + 2 * m["bearer"]
    ring = G.section_ring()
    print("[DEBUG] A_poly cm2", round(abs(G.shoelace(ring)) * 1e4, 2), "Iy cm4", round(G.second_moment(ring) * 1e8, 1), "U m2/m", round(G.perimeter(ring), 4))
    print("[DEBUG] masses", {k: round(v, 3) for k, v in m.items()}, "total", round(mass_total, 2), "beam kg/m", round(m["beam"] / G.L, 2))
    print("[DEBUG] areas", {k: round(v, 4) for k, v in ar.items()})
    steel_c = G.C_STEEL
    heat = {
        "top-flange": round(m["af"] * (G.L - G.CUT) * G.RHO_STEEL * steel_c / ar["top-flange"], -2),
        "bottom-flange": round(m["af"] * G.L * G.RHO_STEEL * steel_c / ar["bottom-flange"], -2),
        "web": round((m["beam"] - 2 * m["af"] * G.L * G.RHO_STEEL) * steel_c / ar["web"], -2),
        "end-cleat": round(m["cleat"] * steel_c / ar["end-cleat"], -2),
        "bearers": round(2 * m["bearer"] * G.C_TIMBER / ar["bearers"], -2),
    }
    print("[DEBUG] heat capacities", heat)
    doc = load("asset", "author-asset.py").document(G, TEXT, tx, m, ar, heat, mass_total, {"50": representation_50(), "100": representation_100(), "200": representation_200(), "500": representation_500()}, {"50": volume_50(), "100": volume_100(), "200": volume_200(), "500": volume_500()})
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print("[DEBUG] written", os.path.join(HERE, "🔣️.json"))


if __name__ == "__main__":
    main()
