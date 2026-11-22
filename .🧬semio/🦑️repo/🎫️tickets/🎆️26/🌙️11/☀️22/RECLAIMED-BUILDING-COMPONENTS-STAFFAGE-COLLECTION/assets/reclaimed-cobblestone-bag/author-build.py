"""🏗️ Writes the asset document `🔣️.json` of `reclaimed-cobblestone-bag` from the hand-chosen numbers of `author-geometry.py` and `author-texts.py`: rig, anchors, poses, clips, thermal and structural models, the four drawn scales and the four handcrafted volumes.

@see ./author-geometry.py — dimensions, field layout and packing
@see ./author-texts.py — every bilingual text
@see ../../brief.md — the asset contract
"""
import importlib.util
import json
import math
import os
from shapely.geometry import Polygon
from shapely.ops import unary_union

HERE = os.path.dirname(os.path.abspath(__file__))


def _load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(HERE, name + ".py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


G = _load("author-geometry")
X = _load("author-texts")
T = X.t
G.tune_shifts()
RUN = os.environ.get("RUN", "")


def sig(v):
    """🔬️ Rounds a section property to four significant digits."""
    return float(f"{v:.4g}")


def r6(v):
    """✂️ Rounds a number or nested list to 0.1 mm precision of the document."""
    if isinstance(v, (list, tuple)):
        return [r6(x) for x in v]
    return round(float(v), 6) + 0.0


SCALES = ["50", "100", "200", "500"]
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}

PALLET_O = (0.0, 0.0, 0.0)
BAG_O = (G.BAG_X, G.BAG_Y0, G.BAG_Z)
LOOPS = {"loop-front-right": (0.90, -1), "loop-back-right": (0.30, -1), "loop-front-left": (0.90, 1), "loop-back-left": (0.30, 1)}
WHEEL_O = (1.45, 0.10, -0.40)


def loop_z(side, hw):
    """↔️ The lateral centre of a loop sewn on the outer face of a wall of half width `hw`."""
    return side * (hw + G.LOOP_T / 2)


def up_local(o, p):
    """⬆️ A world point in the frame of a bone resting with u up, v backwards, w left (bag, loops)."""
    return (p[1] - o[1], -(p[0] - o[0]), p[2] - o[2])


def al_local(o, p):
    """➡️ A world point in the frame of a bone resting with the world axes (pallet, slides, patches)."""
    return (p[0] - o[0], p[1] - o[1], p[2] - o[2])


def patch_o(pid):
    """📍️ The rest origin of a patch bone: its yaw pivot at the bottom of its layer."""
    px, pz = G.pivot(pid)
    return (px, G.layer_y(pid), pz)


def sett_volume():
    """🧊️ Volume of a sett: a 100 mm cube with every arris rounded to 8 mm (worn and chipped)."""
    s, r = G.SETT, G.SETT_R
    c = s - 2 * r
    return c ** 3 + 2 * r * 3 * c * c + math.pi * r * r * 3 * c + 4 / 3 * math.pi * r ** 3


def pallet_mass():
    """⚖️ Mass of the pallet from its boards and blocks."""
    return sum(w * h * d for _, _, _, (w, h, d) in G.pallet_parts()) * G.TIMBER_RHO


def ring_area(hw, t, rc):
    """⭕️ Plan area of a fabric wall ring of half width `hw`."""
    return Polygon(G.rounded_square(hw, rc)).area - Polygon(G.rounded_square(hw - t, rc - t)).area


def bag_mass():
    """⚖️ Mass of the woven bag (walls and bottom) from fabric volume."""
    v = sum(ring_area((a + b) / 2, G.BAG_T, G.BAG_RC) * (u1 - u0) for u0, u1, a, b in G.BAG_BANDS)
    v += Polygon(G.rounded_square(G.BAG_BANDS[0][2] - G.BAG_T, G.BAG_RC - G.BAG_T)).area * G.BAG_T
    return v * G.PP_RHO


def loop_mass():
    """⚖️ Mass of one webbing loop from its band area and 3 mm thickness."""
    return Polygon(G.loop_profile()).area * G.LOOP_T * G.LOOP_RHO


SETT_KG = sett_volume() * G.GRANITE_RHO
PATCH_KG = 48 * SETT_KG
M_PALLET, M_BAG, M_LOOP = pallet_mass(), bag_mass(), loop_mass()
MASS_KG = M_PALLET + M_BAG + 4 * M_LOOP + 8 * PATCH_KG


def rest_centre(pid):
    """🎯️ Centre of the rest footprint of a patch (world x, z)."""
    u = unary_union([Polygon(G.square(x, z, y)) for _, _, x, z, y in G.rest_squares(pid)])
    x0, z0, x1, z1 = u.bounds
    return (x0 + x1) / 2, (z0 + z1) / 2, (x0, x1, z0, z1)


RC = {pid: rest_centre(pid) for pid in G.PATCH_IDS}


def shape(sid, bone, role, z, filled, tone, geom, region=None, regions=None, material=None, hatch=None, pen=None):
    """✏️ One drawn shape."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
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
    s.update(geom)
    return s


def rect(x0, y0, x1, y1):
    """⬛️ A rect geometry from two corners."""
    return {"kind": "rect", "x": r6(min(x0, x1)), "y": r6(min(y0, y1)), "w": r6(abs(x1 - x0)), "h": r6(abs(y1 - y0))}


def poly(points, closed=True):
    """🔷️ A poly geometry."""
    return {"kind": "poly", "points": [r6(p) for p in points], "closed": closed}


def path(rings):
    """〰️ A path geometry of closed rings."""
    d = " ".join("M " + " L ".join(f"{r6(a)} {r6(b)}" for a, b in ring) + " Z" for ring in rings)
    return {"kind": "path", "d": d}


def bag_bands(scale):
    """👜️ The wall bands (u0, u1, half width at u0, at u1) of the bag at a scale."""
    if scale in ("50", "100"):
        return G.BAG_BANDS
    hw = G.BAG_COARSE_HW
    return [(0.0, G.BAG_H, hw, hw)]


def bag_side_outline(scale):
    """👜️ The side (or front) outline of the bag at a scale: (horizontal offset from the bag axis, world y) points around."""
    bands = bag_bands(scale)
    right = [(bands[0][2], G.BAG_Y0)] + [(b[3], G.BAG_Y0 + b[1]) for b in bands]
    left = [(-h, y) for h, y in reversed(right)]
    return right + left


def top_hw(scale):
    """📏️ Half width of the bag at its rim."""
    return bag_bands(scale)[-1][3]


def max_hw(scale):
    """📏️ Largest half width of the bag (the bulge)."""
    return max(max(b[2], b[3]) for b in bag_bands(scale))


# ---------------------------------------------------------------- rig
def rig():
    """🦴️ The rig: pallet (root), bag on its lifting slide, four loops, eight setts patches on slide and yaw bones, the wheel carrier."""
    bones = [{"id": "pallet", "parent": None, "title": T("pallet-bone"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 1.2, "joint": {"type": "fixed"},
              "mass": {"kg": r6(M_PALLET), "comX": 0.6, "comY": 0.072, "distribution": "uniform", "source": "authored-estimate", "reference": "din-en-13698-1"},
              "section": {"areaM2": r6(0.635 * 0.022), "secondMomentM4": sig(0.635 * 0.022 ** 3 / 12), "extremeFibreM": 0.011, "material": "pallet-softwood", "title": T("pallet-section")}},
             {"id": "bag", "parent": "pallet", "title": T("bag-bone"), "x": G.BAG_X, "y": G.BAG_Y0, "z": G.BAG_Z, "angleDeg": 90, "lengthM": G.BAG_H,
              "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 1.2},
              "mass": {"kg": r6(M_BAG), "comX": 0.4, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "din-en-iso-21898"},
              "section": {"areaM2": r6(4 * 0.95 * G.BAG_T), "secondMomentM4": sig(2 / 3 * G.BAG_T * 0.95 ** 3), "extremeFibreM": 0.475, "material": "pp-fabric", "title": T("bag-section")}}]
    for lid, (xc, side) in LOOPS.items():
        zc = loop_z(side, G.BAG_BANDS[-1][3])
        bones.append({"id": lid, "parent": "bag", "title": T(lid), "x": G.BAG_H, "y": r6(-(xc - G.BAG_X)), "z": r6(zc - G.BAG_Z), "angleDeg": 0, "lengthM": 0.28, "joint": {"type": "fixed"},
                      "mass": {"kg": r6(M_LOOP), "comX": 0.08, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "din-en-iso-21898"},
                      "section": {"areaM2": r6(2 * G.LOOP_BAND * G.LOOP_T), "secondMomentM4": sig(2 * G.LOOP_T * G.LOOP_BAND ** 3 / 12), "extremeFibreM": 0.04, "material": "pp-webbing", "title": T("loop-section")}})
    for pid in G.LAYER_ORDER:
        ly = G.layer_y(pid)
        px, py, pz = patch_o(pid)
        cx, cz, _ = RC[pid]
        bones.append({"id": f"slide-{pid}", "parent": "bag", "title": T("slide", pid), "x": r6(ly - G.BAG_Y0), "y": 0, "z": r6(-G.BAG_Z), "angleDeg": -90, "lengthM": 0,
                      "joint": {"type": "prismatic", "axisDeg": 0, "minM": -1.2, "maxM": 0}})
        bones.append({"id": f"patch-{pid}", "parent": f"slide-{pid}", "title": T("patch", pid), "x": r6(px - G.BAG_X), "y": 0, "z": r6(pz), "angleDeg": 0, "lengthM": 0,
                      "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0},
                      "mass": {"kg": r6(PATCH_KG), "comX": r6(cx - px), "comY": 0.05, "distribution": "point", "source": "literature", "reference": "din-en-1342"}})
    bones.append({"id": "wheel", "parent": "pallet", "title": T("wheel-bone"), "x": WHEEL_O[0], "y": WHEEL_O[1], "z": WHEEL_O[2], "angleDeg": 0, "lengthM": 0,
                  "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 3.0}})
    return {"kind": "mechanical", "root": "pallet", "bones": bones}


def anchors():
    """📍️ Ground contact of the pallet, the four lifting points, the bedding of every patch and the wheel load point."""
    out = [{"id": "pallet-ground", "bone": "pallet", "x": 0.6, "y": 0, "kind": "contact", "title": T("pallet-ground"), "support": "contact", "normalDeg": 90, "footprintLengthM": 1.2, "footprintWidthM": 0.8, "frictionCoefficient": 0.5}]
    for lid in LOOPS:
        out.append({"id": f"sling-{lid[5:]}", "bone": lid, "x": 0.2, "y": 0, "kind": "support", "title": T("sling", lid), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.08, "footprintWidthM": 0.05, "frictionCoefficient": 0.3})
    for pid in G.LAYER_ORDER:
        px, py, pz = patch_o(pid)
        cx, cz, _ = RC[pid]
        out.append({"id": f"bed-{pid}", "bone": f"patch-{pid}", "x": r6(cx - px), "y": 0, "z": r6(cz - pz), "kind": "contact", "title": T("bed", pid), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.7, "footprintWidthM": 0.7, "frictionCoefficient": 0.6})
    out.append({"id": "wheel-load", "bone": "wheel", "x": 0, "y": 0, "kind": "load", "title": T("wheel-load")})
    return out


# ---------------------------------------------------------------- volumes
def solid(sid, bone, material, tone, kind, **kw):
    """💡️ One body solid."""
    s = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": "body", "tone": tone}
    s.update(kw)
    return s


def pallet_solids(scale):
    """🪵️ Pallet solids of a scale: every board and block at 1:50, merged decks and feet coarser."""
    out = []
    parts = G.pallet_parts()
    if scale == "50":
        for pid, kind, c, size in parts:
            out.append(solid(f"p-{pid}", "pallet", "pallet-softwood", {"bottom": 0.42, "block": 0.36, "stringer": 0.46, "deck": 0.52}[kind], "box", size=r6(size), transform={"translation": r6(c)}))
    elif scale == "100":
        for pid, kind, c, size in parts:
            if kind != "deck":
                out.append(solid(f"p-{pid}", "pallet", "pallet-softwood", {"bottom": 0.42, "block": 0.36, "stringer": 0.46}[kind], "box", size=r6(size), transform={"translation": r6(c)}))
        out.append(solid("p-deck", "pallet", "pallet-softwood", 0.52, "box", size=[1.2, 0.022, 0.8], transform={"translation": [0.6, 0.133, 0.0]}))
    elif scale == "200":
        out.append(solid("p-deck", "pallet", "pallet-softwood", 0.5, "box", size=[1.2, 0.044, 0.8], transform={"translation": [0.6, 0.122, 0.0]}))
        for pid, kind, c, size in parts:
            if kind == "block":
                out.append(solid(f"p-{pid}", "pallet", "pallet-softwood", 0.38, "box", size=r6([size[0], 0.1, size[2]]), transform={"translation": r6([c[0], 0.05, c[2]])}))
    else:
        out.append(solid("p-pallet", "pallet", "pallet-softwood", 0.45, "box", size=[1.2, 0.144, 0.8], transform={"translation": [0.6, 0.072, 0.0]}))
    return out


def bag_solids(scale):
    """👜️ The woven bag: hollow wall bands (true 0.4 mm fabric), the bottom sheet, and four webbing loops."""
    out = []
    w0 = -G.BAG_Z
    for n, (u0, u1, a, b) in enumerate(bag_bands(scale)):
        prof = [(r6(-p[0]), r6(p[1] + w0)) for p in G.rounded_square(a, G.BAG_RC)]
        hole = [(r6(-p[0]), r6(p[1] + w0)) for p in G.rounded_square(a - G.BAG_T, G.BAG_RC - G.BAG_T)]
        s = solid(f"bag-wall-{n + 1}", "bag", "pp-fabric", 0.82, "extrude", profile=prof, holes=[hole], plane="yz", **{"from": r6(u0), "to": r6(u1)})
        if abs(a - b) > 1e-9:
            s["taperTo"] = [1.0, r6(b / a)]
        out.append(s)
    hb = bag_bands(scale)[0][2] - G.BAG_T
    out.append(solid("bag-bottom", "bag", "pp-fabric", 0.8, "extrude", profile=[(r6(-p[0]), r6(p[1] + w0)) for p in G.rounded_square(hb, G.BAG_RC - G.BAG_T)], plane="yz", sheet=True, **{"from": 0.0, "to": G.BAG_T}))
    for lid in LOOPS:
        out.append(solid(lid, lid, "pp-webbing", 0.34, "extrude", profile=G.loop_profile(), plane="xy", sheet=True, **{"from": r6(-G.LOOP_T / 2), "to": r6(G.LOOP_T / 2)}))
    return out


def quat_yaw(psi):
    """🧭️ The quaternion that turns u by `psi` towards w about v."""
    return [0.0, r6(math.sin(-psi / 2)), 0.0, r6(math.cos(-psi / 2))]


SETT_TONES = [0.62, 0.55, 0.66, 0.58, 0.6, 0.53, 0.64, 0.57]


def patch_rest_polys(pid, poly_field):
    """↪️ A field polygon of a patch in the local (w, u) plane of its patch bone at rest."""
    px, _, pz = patch_o(pid)
    return [(r6(z - pz), r6(x - px)) for x, z in G.ring_rest(pid, poly_field)]


TOL = {"50": 0.002, "100": 0.004, "200": 0.010, "500": 0.025}


def course_rest(pid, m, scale):
    """🌈️ Course m of a patch as a rest-frame profile (w, u) of its patch bone."""
    side, j = pid[0], int(pid[1:]) - 1
    return patch_rest_polys(pid, G.course_band(G.COURSES_PER_PATCH * j + m, side).simplify(TOL[scale] / 2, preserve_topology=True))


def patch_solids(scale):
    """🪨️ The setts: every sett at 1:50 (eight per half course, repeated over six courses), half courses at 1:100, one block per patch coarser."""
    out = []
    for pid in G.LAYER_ORDER:
        px, _, pz = patch_o(pid)
        bone = f"patch-{pid}"
        if scale == "50":
            for m, i, x, z, yaw in G.rest_squares(pid):
                if m != 0:
                    continue
                out.append(solid(f"sett-{pid}-{i:02d}", bone, "granite", SETT_TONES[i % 8], "box", size=[G.SETT, G.SETT_H, G.SETT], cornerRadius=G.SETT_R,
                                 transform={"rotation": quat_yaw(yaw), "translation": r6([x - px, G.SETT_H / 2, z - pz])},
                                 repeat={"count": G.COURSES_PER_PATCH, "translation": [-G.ROW_PITCH, 0.0, 0.0]}))
        elif scale == "100":
            out.append(solid(f"course-{pid}", bone, "granite", 0.6, "extrude", profile=course_rest(pid, 0, scale), plane="zx", **{"from": 0.0, "to": G.SETT_H},
                             repeat={"count": G.COURSES_PER_PATCH, "translation": [-G.ROW_PITCH, 0.0, 0.0]}))
        else:
            out.append(solid(f"block-{pid}", bone, "granite", 0.6, "extrude", profile=patch_rest_polys(pid, G.patch_outline_field(pid, TOL[scale])), plane="zx", **{"from": 0.0, "to": G.SETT_H}))
    return out


def body(scale):
    """🧱️ The body list of a scale."""
    return pallet_solids(scale) + bag_solids(scale) + patch_solids(scale)


def thermal(scale, b):
    """🌡️ The thermal list: pallet and bag as their body surfaces, the setts of each patch as one envelope (joints filled when laid, setts touching in the bag)."""
    out = []
    for s in b:
        if s["material"] == "pallet-softwood":
            out.append({"id": f"th-{s['id']}", "of": s["id"], "region": "pallet-timber"})
        elif s["material"] in ("pp-fabric", "pp-webbing"):
            out.append({"id": f"th-{s['id']}", "of": s["id"], "region": "bag-fabric"})
        elif scale in ("200", "500"):
            out.append({"id": f"th-{s['id']}", "of": s["id"], "region": "setts"})
    if scale in ("50", "100"):
        for pid in G.LAYER_ORDER:
            prof = patch_rest_polys(pid, G.patch_outline_field(pid, TOL[scale]))
            out.append({**solid(f"th-envelope-{pid}", f"patch-{pid}", "granite", 0.6, "extrude", profile=prof, plane="zx", **{"from": 0.0, "to": G.SETT_H}), "region": "setts"})
    return out


MASS_R = {"50": 0.03, "100": 0.033, "200": 0.035, "500": 0.045}


def structure(scale, b):
    """🏗️ The structure list: pallet boards, bag fabric and loops as members (their body solids), one mass marker per setts patch."""
    out = []
    for s in b:
        if s["material"] == "pallet-softwood":
            out.append({"id": f"st-{s['id']}", "of": s["id"], "member": "pallet", "part": "member"})
        elif s["material"] == "pp-fabric":
            out.append({"id": f"st-{s['id']}", "of": s["id"], "member": "bag", "part": "member"})
        elif s["material"] == "pp-webbing":
            out.append({"id": f"st-{s['id']}", "of": s["id"], "member": s["bone"], "part": "member"})
    for pid in G.LAYER_ORDER:
        px, _, pz = patch_o(pid)
        cx, cz, _ = RC[pid]
        rr = MASS_R[scale]
        out.append({**solid(f"mass-{pid}", f"patch-{pid}", "granite", 0.3, "ellipsoid", radii=[rr, rr, rr], transform={"translation": r6([cx - px, 0.05, cz - pz])}), "member": f"patch-{pid}", "part": "mass"})
    return out


def volumes():
    """🧊️ The four handcrafted volumes."""
    out = {}
    for sc in SCALES:
        b = body(sc)
        out[sc] = {"status": "authored", "denominator": int(sc), "body": b, "thermal": thermal(sc, b), "structure": structure(sc, b), "decisions": X.volume_decisions(sc)}
    return out


# ---------------------------------------------------------------- 2D
PENS = {"50": {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5},
        "100": {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35},
        "200": {"body": 0.18, "detail": 0.1, "texture": None, "glazing": 0.1, "ground": 0.25},
        "500": {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}


def hatches(scale):
    """▤️ Hatches of a scale."""
    if scale == "50":
        return [{"id": "weave", "title": T("hatch-weave"), "pattern": "cross", "angleDeg": 45, "spacingMm": 0.8, "penMm": 0.1},
                {"id": "grain", "title": T("hatch-grain"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.5, "penMm": 0.1},
                {"id": "granite", "title": T("hatch-granite"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.5, "penMm": 0.1}]
    if scale == "100":
        return [{"id": "grain", "title": T("hatch-grain"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.5, "penMm": 0.1}]
    return []


def rings_of(geom):
    """🔁️ Every ring (exteriors and holes) of a shapely polygon or multipolygon."""
    polys = list(geom.geoms) if hasattr(geom, "geoms") else [geom]
    out = []
    for p in polys:
        out.append(list(p.exterior.coords)[:-1])
        out += [list(h.coords)[:-1] for h in p.interiors]
    return out


def pallet_2d(scale):
    """🪵️ Pallet shapes: one union outline per view (no unprintable slivers between touching parts), board joints as detail at 1:50, in plan only what the bag leaves visible at 1:50 and 1:100."""
    from shapely.geometry import box as sb
    el, fr, pl = [], [], []
    reg = dict(region="pallet-timber", material="pallet-softwood")
    hat = {"hatch": "grain"} if scale in ("50", "100") else {}
    sol = pallet_solids(scale)

    def ext(s):
        (cx, cy, cz), (w, h, d) = s["transform"]["translation"], s["size"]
        return cx - w / 2, cx + w / 2, cy - h / 2, cy + h / 2, cz - d / 2, cz + d / 2
    side = unary_union([sb(a[0], a[2], a[1], a[3]) for a in map(ext, sol)])
    front = unary_union([sb(a[4], a[2], a[5], a[3]) for a in map(ext, sol)])
    el.append(shape("el-pallet", "pallet", "body", 1, True, 0.45, path(rings_of(side)), **reg, **hat))
    fr.append(shape("fr-pallet", "pallet", "body", 1, True, 0.45, path(rings_of(front)), **reg))
    if scale == "50":
        for n, (y, x0, x1) in enumerate([(0.122, 0, 1.2), (0.1, 0, 0.145), (0.1, 0.5275, 0.6725), (0.1, 1.055, 1.2)]):
            el.append(shape(f"el-pallet-joint-{n + 1}", "pallet", "detail", 1.1, False, 0.3, poly([(x0, y), (x1, y)], False), region="pallet-timber"))
        fr.append(shape("fr-pallet-joint-1", "pallet", "detail", 1.1, False, 0.3, poly([(-0.4, 0.1), (0.4, 0.1)], False), region="pallet-timber"))
        fr.append(shape("fr-pallet-joint-2", "pallet", "detail", 1.1, False, 0.3, poly([(-0.4, 0.122), (0.4, 0.122)], False), region="pallet-timber"))
    top = unary_union([sb(a[0], a[4], a[1], a[5]) for a in map(ext, sol)])
    if scale in ("50", "100"):
        hw = max_hw(scale)
        top = top.difference(sb(G.BAG_X - hw, -1, G.BAG_X + hw, 1))
    if scale == "50":
        for n, (x0, x1) in enumerate([(0.0, G.BAG_X - hw), (G.BAG_X + hw, 1.2)]):
            for k, z in enumerate([-0.255, -0.21375, -0.11375, -0.0725, 0.0725, 0.11375, 0.21375, 0.255]):
                pl.append(shape(f"pl-pallet-joint-{n + 1}-{k + 1}", "pallet", "detail", 1.1, False, 0.3, poly([(x0, z), (x1, z)], False), region="pallet-timber"))
    pieces = sorted(list(top.geoms) if hasattr(top, "geoms") else [top], key=lambda g: (g.bounds[0], g.bounds[1]))
    for n, piece in enumerate(pieces):
        pl.append(shape(f"pl-pallet-{n + 1}", "pallet", "body", 1, True, 0.45, poly(list(piece.exterior.coords)[:-1]), **reg, **hat))
    return el, fr, pl


def loop_outer():
    """🪢️ The outer edge of a loop (legs and arch without the opening), for the 1:500 tab."""
    pts = G.loop_profile()
    return pts[: len(pts) // 2]


def bag_2d(scale):
    """👜️ Bag and loops: filled side and front outlines; in plan a 16 mm wall ring with the open mouth (1:50), the bulge outline (1:100) or the filled bag (coarser); loops as strokes where they are 3 mm thin."""
    el, fr, pl = [], [], []
    reg = dict(region="bag-fabric", material="pp-fabric")
    hat = {"hatch": "weave"} if scale == "50" else {}
    side = bag_side_outline(scale)
    el.append(shape("el-bag", "bag", "body", 2, True, 0.82, poly([up_local(BAG_O, (G.BAG_X + h, y, 0))[:2] for h, y in side]), **reg, **hat))
    fr.append(shape("fr-bag", "bag", "body", 2, True, 0.82, poly([(h, y) for h, y in side]), **reg, **hat))
    outer = [(G.BAG_X + a, b) for a, b in G.rounded_square(max_hw(scale), G.BAG_RC)]
    if scale == "50":
        inner = [(G.BAG_X + a, b) for a, b in G.rounded_square(top_hw(scale) - 0.002, G.BAG_RC - 0.002)]
        pl.append(shape("pl-bag-wall", "bag", "body", 2, True, 0.8, path([outer, inner]), **reg))
        pl.append(shape("pl-bag-floor", "bag", "body", 1.4, True, 0.88, poly(inner), **reg))
    elif scale == "100":
        pl.append(shape("pl-bag", "bag", "body", 1.4, True, 0.88, poly(outer), **reg))
    else:
        pl.append(shape("pl-bag", "bag", "body", 2, True, 0.8, poly(outer), **reg))
    if scale in ("50", "100"):
        rim = G.BAG_Y0 + G.BAG_H
        for n, sgn in enumerate((-1, 1)):
            pts = []
            for u0, u1, a, b in bag_bands(scale):
                pts += [(G.BAG_X + sgn * (a - G.BAG_RC), G.BAG_Y0 + u0), (G.BAG_X + sgn * (b - G.BAG_RC), G.BAG_Y0 + u1)]
            el.append(shape(f"el-bag-seam-{n + 1}", "bag", "detail", 2.1, False, 0.5, poly([up_local(BAG_O, (x, y, 0))[:2] for x, y in pts], False), region="bag-fabric"))
            fr.append(shape(f"fr-bag-seam-{n + 1}", "bag", "detail", 2.1, False, 0.5, poly([(x - G.BAG_X, y) for x, y in pts], False), region="bag-fabric"))
        hw = top_hw(scale)
        el.append(shape("el-bag-hem", "bag", "detail", 2.1, False, 0.5, poly([up_local(BAG_O, (G.BAG_X + s * hw, rim - 0.03, 0))[:2] for s in (-1, 1)], False), region="bag-fabric"))
        fr.append(shape("fr-bag-hem", "bag", "detail", 2.1, False, 0.5, poly([(s * hw, rim - 0.03) for s in (-1, 1)], False), region="bag-fabric"))
    hw = top_hw(scale)
    role = "detail" if scale != "500" else "body"
    for lid, (xc, sd) in LOOPS.items():
        zc = loop_z(sd, hw)
        if sd < 0:
            geom = poly(loop_outer()) if scale == "500" else poly(G.loop_profile())
            el.append(shape(f"el-{lid}", lid, "body", 3, True, 0.34, geom, region="bag-fabric", material="pp-webbing"))
        y0, y1 = G.BAG_Y0 + G.BAG_H - G.LOOP_SEWN, G.BAG_Y0 + G.BAG_H + G.LOOP_ARCH_C + G.LOOP_HALF_OUT
        if xc > 0.6:
            fr.append(shape(f"fr-{lid}", lid, role, 3, False, 0.34, poly([(zc, y0), (zc, y1)], False), region="bag-fabric"))
        pl.append(shape(f"pl-{lid}", lid, role, 3, False, 0.34, poly([(xc - G.LOOP_HALF_OUT, zc), (xc + G.LOOP_HALF_OUT, zc)], False), region="bag-fabric"))
    return el, fr, pl


def joint_lines(pid, radial):
    """〰️ Course joints (arcs between courses) and, with `radial`, the joints between the setts of a course, carried to the rest place of a patch."""
    side, j = pid[0], int(pid[1:]) - 1
    idx = list(range(0, 8)) if side == "r" else list(range(8, 16))
    lo, hi = G.PHI[idx[0]] - G.DELTA / 2, G.PHI[idx[-1]] + G.DELTA / 2
    if side == "r":
        hi = 0.0
    else:
        lo = 0.0
    out = []
    for m in range(1, G.COURSES_PER_PATCH):
        a = G.COURSES_PER_PATCH * j + m
        c = G.crown_x(a) - G.ARC_R - G.ROW_PITCH / 2
        pts = [G.to_rest(pid, c + G.ARC_R * math.cos(lo + (hi - lo) * k / 10), G.ARC_R * math.sin(lo + (hi - lo) * k / 10)) for k in range(11)]
        out.append(("detail", pts))
    if radial:
        for m in range(G.COURSES_PER_PATCH):
            a = G.COURSES_PER_PATCH * j + m
            c = G.crown_x(a) - G.ARC_R
            for i in idx[:-1]:
                ph = G.PHI[i] + G.DELTA / 2
                pts = [G.to_rest(pid, c + r * math.cos(ph), r * math.sin(ph)) for r in (G.ARC_R - 0.046, G.ARC_R + 0.046)]
                out.append(("texture", pts))
    return out


def patch_2d(scale):
    """🪨️ Setts: elevation and front bands; in plan the outline of each patch with its course joints (and sett joints at 1:50), because the 2.5 to 12 mm joints are no printable gaps."""
    el, fr, pl = [], [], []
    reg = dict(region="setts", material="granite")
    hat = {"hatch": "granite"} if scale == "50" else {}
    for pid in G.LAYER_ORDER:
        px, py, pz = patch_o(pid)
        bone = f"patch-{pid}"
        ring = G.ring_rest(pid, G.patch_outline_field(pid, TOL[scale]))
        xs, zs = [p[0] for p in ring], [p[1] for p in ring]
        x0, x1, z0, z1 = min(xs), max(xs), min(zs), max(zs)
        zl = 1.5 + 0.02 * G.LAYER_ORDER.index(pid)
        ze = 1.5 if pid[0] == "r" else 1.4
        el.append(shape(f"el-{pid}", bone, "body", ze, True, 0.6, rect(x0 - px, 0, x1 - px, G.SETT_H), **reg, **hat))
        fr.append(shape(f"fr-{pid}", bone, "body", zl, True, 0.6, rect(z0, py, z1, py + G.SETT_H), **reg, **hat))
        pl.append(shape(f"pl-{pid}", bone, "body", zl, True, 0.6, path([ring]), **reg, **hat))
        if scale == "50" and pid[0] == "r":
            near = sorted([(x, z, y) for m, i, x, z, y in G.rest_squares(pid) if i == 0])
            for n in range(len(near) - 1):
                xm = (near[n][0] + near[n + 1][0]) / 2 - px
                el.append(shape(f"el-joint-{pid}-{n + 1}", bone, "detail", 1.6, False, 0.3, poly([(xm, 0.004), (xm, G.SETT_H - 0.004)], False), region="setts"))
        if scale in ("50", "100", "200"):
            for n, (role, pts) in enumerate(joint_lines(pid, scale == "50")):
                pl.append(shape(f"pl-joint-{pid}-{n + 1}", bone, role, zl + 0.01, False, 0.3, poly(pts, False), region="setts"))
    return el, fr, pl


def representations():
    """🖨️ The four drawn scales."""
    out = {}
    ranges = {"50": (20, 75), "100": (75, 150), "200": (150, 350), "500": (350, 1000)}
    for sc in SCALES:
        e1, f1, p1 = pallet_2d(sc)
        e2, f2, p2 = bag_2d(sc)
        e3, f3, p3 = patch_2d(sc)
        out[sc] = {"status": "authored", "denominator": int(sc), "validRange": {"minDenominator": ranges[sc][0], "maxDenominator": ranges[sc][1]}, "pens": PENS[sc], "hatches": hatches(sc),
                   "geometry": {"elevation": e1 + e3 + e2, "front": f1 + f3 + f2, "plan": p1 + p3 + p2}, "decisions": X.rep_decisions(sc)}
    return out


# ---------------------------------------------------------------- poses
def bind(setts, bag, pallet):
    """🌬️ A thermal binding of the three regions: (exposed, sunlit) each."""
    return {"regions": {"setts": {"exposedFraction": setts[0], "sunlitFraction": setts[1]}, "bag-fabric": {"exposedFraction": bag[0], "sunlitFraction": bag[1]},
                        "pallet-timber": {"exposedFraction": pallet[0], "sunlitFraction": pallet[1]}}, "emitters": {}}


SLINGS = [f"sling-{lid[5:]}" for lid in LOOPS]
BEDS = [f"bed-{pid}" for pid in G.LAYER_ORDER]
WHEEL_MID = 1.40
REINSTALL = {**{f"slide-{pid}": {"dx": r6(-G.layer_y(pid))} for pid in G.LAYER_ORDER}, **{f"patch-{pid}": {"abductionDeg": 180} for pid in G.LAYER_ORDER}}
LIFT_M = 0.40


def wheel_load():
    """🛞️ The wheel load of a light vehicle (EN 1991-1-1 category F: axle 20 kN, wheel 10 kN on 100 x 100 mm)."""
    return {"id": "wheel", "kind": "live", "title": T("wheel-load-title"), "anchor": "wheel-load", "fxN": 0, "fyN": -10000, "source": "literature", "reference": "en-1991-1-1"}


def poses():
    """🧍️ Stored, lifted and reinstalled."""
    return [{"id": "stored", "title": T("pose-stored"), "description": T("pose-stored-d"), "bones": {}, "ground": "authored", "contacts": ["pallet-ground"], "thermal": bind((0.25, 0.08), (0.75, 0.4), (0.8, 0.1)), "loads": []},
            {"id": "lifted", "title": T("pose-lifted"), "description": T("pose-lifted-d"), "bones": {"bag": {"dy": LIFT_M}}, "ground": "authored", "contacts": SLINGS, "thermal": bind((0.25, 0.08), (0.9, 0.45), (0.85, 0.15)), "loads": []},
            {"id": "reinstalled", "title": T("pose-reinstalled"), "description": T("pose-reinstalled-d"), "bones": {**REINSTALL, "wheel": {"dx": WHEEL_MID}}, "ground": "authored", "contacts": ["pallet-ground"] + BEDS,
             "thermal": bind((0.6, 0.6), (0.85, 0.4), (0.8, 0.1)), "loads": [wheel_load()]}]


def clips():
    """🎞️ The lift and the wheel crossing the relaid field."""
    lift = {"id": "lift", "title": T("clip-lift"), "description": T("clip-lift-d"), "durationS": 6, "fps": 25, "loop": False, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1, "pose": "stored", "ease": "smooth"}, {"t": 5, "pose": "lifted", "ease": "smooth"}, {"t": 6, "pose": "lifted", "ease": "smooth"}],
            "contacts": [{"anchor": "pallet-ground", "fromS": 0, "toS": 1.2}] + [{"anchor": s, "fromS": 0.8, "toS": 6} for s in SLINGS],
            "phases": [{"id": "take-up", "fromS": 0, "toS": 1, "title": T("ph-take-up")}, {"id": "lift", "fromS": 1, "toS": 5, "title": T("ph-lift")}, {"id": "hold", "fromS": 5, "toS": 6, "title": T("ph-hold")}],
            "events": [{"t": 0.8, "kind": "contact-on", "title": T("ev-loops")}, {"t": 1.2, "kind": "contact-off", "title": T("ev-pallet")}], "tracks": []}
    wheel = {"id": "wheel-crossing", "title": T("clip-wheel"), "description": T("clip-wheel-d"), "durationS": 6, "fps": 25, "loop": False, "rootMotion": "none",
             "keys": [{"t": 0, "pose": "reinstalled", "ease": "linear", "bones": {"wheel": {"dx": 0.05}}}, {"t": 6, "pose": "reinstalled", "ease": "linear", "bones": {"wheel": {"dx": 2.75}}}],
             "contacts": [{"anchor": a, "fromS": 0, "toS": 6} for a in ["pallet-ground"] + BEDS],
             "phases": [{"id": "travel", "fromS": 0, "toS": 6, "title": T("ph-travel")}], "events": [{"t": 0, "kind": "load-on", "title": T("ev-wheel")}], "tracks": []}
    return [lift, wheel]


AREA = {"setts": float(os.environ.get("A_SETTS", "3.68")), "bag-fabric": float(os.environ.get("A_BAG", "8.9")), "pallet-timber": float(os.environ.get("A_PALLET", "3.14"))}


def thermal_model():
    """🌡️ Three passive regions with weathered surface data."""
    def region(rid, kind, mat, area, cl, geom, sky, cap, ref):
        return {"id": rid, "title": T(f"reg-{rid}"), "kind": kind, "model": "passive", "material": mat, "areaM2": area, "characteristicLengthM": cl, "geometry": geom, "skyViewFactor": sky, "layers": [], "heatCapacityJM2K": cap, "source": "authored-estimate", "reference": ref}
    return {"environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 1, "windDirectionDeg": 0, "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325},
            "operating": [],
            "regions": [region("setts", "surface", "granite", AREA["setts"], 0.9, "horizontal-plate-up", 0.7, r6(G.GRANITE_RHO * 1000 * 0.05), "en-iso-10456"),
                        region("bag-fabric", "surface", "pp-fabric", AREA["bag-fabric"], 0.9, "vertical-plate", 0.5, r6(G.PP_RHO * 1800 * G.BAG_T), "en-iso-10456"),
                        region("pallet-timber", "surface", "pallet-softwood", AREA["pallet-timber"], 0.15, "vertical-plate", 0.3, r6(G.TIMBER_RHO * 1600 * 0.011), "en-iso-10456")],
            "emitters": [], "assumptions": X.thermal_assumptions(), "passengers": []}


def asset():
    """🏺️ The whole asset."""
    vols = volumes()
    return {"id": "reclaimed-cobblestone-bag", "category": "components", "title": T("title"), "summary": T("summary"),
            "tags": ["component", "reclaimed", "granite", "natural-stone", "paving", "setts", "big-bag", "pallet", "landscaping"],
            "provenance": X.provenance(), "dimensions": {"heightM": r6(G.BAG_Y0 + G.BAG_H + G.LOOP_ARCH_C + G.LOOP_HALF_OUT), "widthM": 1.2, "depthM": r6(2 * max_hw("50")), "massKg": r6(MASS_KG)},
            "materials": X.materials(), "rig": rig(), "volumes": vols, "anchors": anchors(), "representations": representations(), "symbols": [], "poses": poses(), "clips": clips(),
            "thermal": thermal_model(), "structure": {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "assumptions": X.structure_assumptions(), "clinical": False},
            "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
            "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"], "completion": "authored"}


if __name__ == "__main__":
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset()}
    with open(os.path.join(HERE, "🔣️.json"), "w") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
    print("massKg", round(MASS_KG, 2), "sett kg", round(SETT_KG, 3), "pallet", round(M_PALLET, 2), "bag", round(M_BAG, 3), "loop", round(M_LOOP, 3))
