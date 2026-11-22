"""🏚️ Writes the asset document `🔣️.json` of the reclaimed bolted timber W-truss in its stillage: rig, anchors, poses, clips, thermal and structural model, the 2D drawings and the 3D volumes of 1:50, 1:100, 1:200 and 1:500, all from the numbers of `author-geometry.py`.

Run: `python3 author-truss.py` in this folder; it overwrites `🔣️.json` next to it.
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("geometry", os.path.join(HERE, "author-geometry.py"))
g = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(g)

POLYS, PLATES = g.check()
SL = g.sling()
SCALES = (50, 100, 200, 500)
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
AREAS = {"truss-timber": 12.72, "steel-fittings": 6.23, "stillage-timber": 4.25, "chain-sling": 0.25, "round-slings": 0.132}


def T(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def r(x, n=5):
    """🔢️ Rounded number for the document."""
    v = round(x, n)
    return 0.0 if v == 0 else v


def rp(points, n=5):
    """🔢️ Rounded point list."""
    return [[r(p[0], n), r(p[1], n)] for p in points]


BONES = []


def bone(id, parent, title, x, y, angle, length, joint, z=None, mass=None, section=None):
    """🦴️ Appends a rig bone."""
    b = {"id": id, "parent": parent, "title": title, "x": r(x), "y": r(y), "angleDeg": r(angle, 4), "lengthM": r(length), "joint": joint}
    if z is not None:
        b["z"] = r(z)
    if mass is not None:
        b["mass"] = mass
    if section is not None:
        b["section"] = section
    BONES.append(b)


def frames():
    """🧭️ World rest frames (origin x, y, z and angle) of every bone."""
    out = {}
    for b in BONES:
        if b["parent"] is None:
            out[b["id"]] = (b["x"], b["y"], b.get("z", 0.0), b["angleDeg"])
            continue
        px, py, pz, pa = out[b["parent"]]
        c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
        out[b["id"]] = (px + c * b["x"] - s * b["y"], py + s * b["x"] + c * b["y"], pz + b.get("z", 0.0), pa + b["angleDeg"])
    return out


def chord_section(h, material, en, de):
    """📏️ The rectangular section of a timber member 80 mm wide and h deep."""
    return {"areaM2": r(g.B_CHORD * h, 6), "secondMomentM4": float(f"{g.B_CHORD * h ** 3 / 12:.5e}"), "extremeFibreM": r(h / 2), "material": material, "title": T(en, de)}


def timber_mass(polys_key):
    """⚖️ Mass of a timber member polygon, 80 mm thick, at the mean density of C24."""
    return abs(g.area(POLYS[polys_key])) * g.B_CHORD * g.DENSITY["softwood-weathered"]


def plate_mass(node):
    """⚖️ Mass of the pair of 6 mm steel gusset plates of one node."""
    return 2 * abs(g.area(PLATES[node])) * g.PLATE_T * g.DENSITY["steel-primed"]


BOLT_KG = 0.42
BEARING_KG = 2 * 0.30 * 0.12 * 0.010 * 7850
STUB_KG = 0.10 * 0.024 * 0.40 * g.DENSITY["softwood-weathered"]
CHAIN_KG_M = 1.4
BOLTS = g.bolts()
MASS = {
    "truss": timber_mass("bottom") + sum(plate_mass(n) for n in "aefg") + BOLT_KG * sum(len(BOLTS[n]) for n in "aefg") + BEARING_KG,
    "top-chord-l": timber_mass("top-l") + plate_mass("b") + plate_mass("c") / 2 + BOLT_KG * (len(BOLTS["b"]) + len(BOLTS["c"]) / 2) + STUB_KG,
    "top-chord-r": timber_mass("top-r") + plate_mass("d") + plate_mass("c") / 2 + BOLT_KG * (len(BOLTS["d"]) + len(BOLTS["c"]) / 2) + STUB_KG,
    "web-bf": timber_mass("bf"),
    "web-fc": timber_mass("fc"),
    "web-gc": timber_mass("gc"),
    "web-gd": timber_mass("gd"),
    "eye-l": 0.30,
    "eye-r": 0.30,
    "chain-l": CHAIN_KG_M * SL["c"] + 0.45,
    "chain-r": CHAIN_KG_M * SL["c"] + 0.45,
    "master-link": 1.10,
}
TOTAL_KG = sum(MASS.values())


def seg_mass(id, ref, length):
    """⚖️ A uniform segment mass of a bone with its source."""
    return {"kg": r(MASS[id], 3), "comX": r(length / 2), "comY": 0, "distribution": "uniform", "source": "literature" if id.startswith(("truss", "top", "web")) else "authored-estimate", "reference": ref}


def build_rig():
    """🦴️ The rig: yard root, two sliding trestles, the lifting truss with its seven members and the two-leg sling."""
    A, E, F, G = g.A, g.E, g.F, g.G
    L_tc = math.hypot(g.C[0] - A[0], g.C[1] - A[1])
    L_bf = math.hypot(g.B[0] - F[0], g.B[1] - F[1])
    L_fc = math.hypot(g.C[0] - F[0], g.C[1] - F[1])
    bone("yard", None, T("Yard (ground of the component exchange)", "Lagerplatz (Boden der Bauteilbörse)"), 0, 0, 0, 0, {"type": "fixed"})
    bone("trestle-l", "yard", T("Left stillage trestle (slides aside)", "Linker Lagerbock (wird beiseitegeschoben)"), g.TRESTLE_X[0], 0, 0, 0, {"type": "prismatic", "axisDeg": 0, "minM": -g.SLIDE, "maxM": 0})
    bone("trestle-r", "yard", T("Right stillage trestle (slides aside)", "Rechter Lagerbock (wird beiseitegeschoben)"), g.TRESTLE_X[1], 0, 0, 0, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": g.SLIDE})
    bone("truss", "yard", T("Bottom chord (tie beam), carries the whole truss when lifted", "Untergurt (Zugbalken), trägt beim Anheben den ganzen Binder"), 0, g.Y_TRUSS, 0, g.LENGTH, {"type": "prismatic", "axisDeg": 90, "minM": g.SET_DOWN, "maxM": 1.0}, mass=seg_mass("truss", "en-338", g.LENGTH), section=chord_section(g.H_CHORD, "softwood-weathered", "Bottom chord C24 80 x 180 mm", "Untergurt C24 80 x 180 mm"))
    bone("top-chord-l", "truss", T("Left top chord (rafter)", "Linker Obergurt (Sparren)"), A[0], A[1], g.ALPHA, L_tc, {"type": "fixed"}, mass=seg_mass("top-chord-l", "en-338", L_tc), section=chord_section(g.H_CHORD, "softwood-weathered", "Top chord C24 80 x 180 mm", "Obergurt C24 80 x 180 mm"))
    bone("top-chord-r", "truss", T("Right top chord (rafter)", "Rechter Obergurt (Sparren)"), E[0], E[1], 180 - g.ALPHA, L_tc, {"type": "fixed"}, mass=seg_mass("top-chord-r", "en-338", L_tc), section=chord_section(g.H_CHORD, "softwood-weathered", "Top chord C24 80 x 180 mm", "Obergurt C24 80 x 180 mm"))
    web_title = {"web-bf": T("Short left web (strut)", "Kurze linke Diagonale (Druckstab)"), "web-fc": T("Long left web (tie)", "Lange linke Diagonale (Zugstab)"), "web-gc": T("Long right web (tie)", "Lange rechte Diagonale (Zugstab)"), "web-gd": T("Short right web (strut)", "Kurze rechte Diagonale (Druckstab)")}
    for id, origin, key, length in (("web-bf", F, "bf", L_bf), ("web-fc", F, "fc", L_fc), ("web-gc", G, "gc", L_fc), ("web-gd", G, "gd", L_bf)):
        bone(id, "truss", web_title[id], origin[0], origin[1], g.DIR[key], length, {"type": "fixed"}, mass=seg_mass(id, "en-338", length), section=chord_section(g.H_WEB, "softwood-weathered", "Web C24 80 x 120 mm", "Diagonale C24 80 x 120 mm"))
    chain_section = {"areaM2": 1.005e-4, "secondMomentM4": 4.02e-10, "extremeFibreM": 0.004, "material": "chain-steel", "title": T("Chain 8 mm grade 8, two link legs", "Kette 8 mm Güteklasse 8, zwei Gliedschenkel")}
    for side, parent, sign in (("l", "top-chord-l", 1), ("r", "top-chord-r", -1)):
        bone(f"eye-{side}", parent, T(f"Choked round-sling eye, {'left' if side == 'l' else 'right'} leg", f"Geschnürtes Rundschlingenauge, {'linker' if side == 'l' else 'rechter'} Strang"), g.CHOKE_S, sign * g.EYE_V, 180, SL["b"], {"type": "revolute", "minDeg": -180 if side == "l" else 0, "maxDeg": 0 if side == "l" else 180}, z=sign * 0.015, mass=seg_mass(f"eye-{side}", "en-1492-2", SL["b"]))
        bone(f"chain-{side}", f"eye-{side}", T(f"Chain leg, {'left' if side == 'l' else 'right'}", f"Kettenstrang, {'links' if side == 'l' else 'rechts'}"), SL["b"], -sign * g.CHAIN_OFF, 180, SL["c"], {"type": "revolute", "minDeg": 0 if side == "l" else -180, "maxDeg": 180 if side == "l" else 0}, mass=seg_mass(f"chain-{side}", "en-818-4", SL["c"]), section=chain_section)
    bone("master-link", "chain-l", T("Master link (crane hook point)", "Aufhängering (Kranhakenpunkt)"), SL["c"], 0, -2 * g.ALPHA, g.RING[0], {"type": "revolute", "minDeg": -180, "maxDeg": 180}, z=-0.015, mass=seg_mass("master-link", "en-818-4", g.RING[0]))


build_rig()
FR = frames()


def to_local(bone_id, pts):
    """↩️ World (x, y) points into the (u, v) frame of a bone at rest."""
    ox, oy, _, a = FR[bone_id]
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return [((p[0] - ox) * c + (p[1] - oy) * s, -(p[0] - ox) * s + (p[1] - oy) * c) for p in pts]


def to_world(bone_id, pts3):
    """↪️ Bone-local (u, v, w) points into world rest coordinates."""
    ox, oy, oz, a = FR[bone_id]
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return [(ox + c * p[0] - s * p[1], oy + s * p[0] + c * p[1], oz + p[2]) for p in pts3]


def tw(pts):
    """🏗️ Truss-frame points into world points."""
    return [(p[0], p[1] + g.Y_TRUSS) for p in pts]


class Part:
    """🧩️ One physical part of one scale: its body solid, its region and its three 2D shapes."""

    def __init__(self, id, bone, region, material, tone, solid, corners, elevation, role="body", front=None, plan=None, flags=None, label=None):
        self.id, self.bone, self.region, self.material, self.tone, self.role = id, bone, region, material, tone, role
        self.solid = {"id": id, "bone": bone, "material": material, "role": role, "tone": tone, **solid, **(flags or {})}
        self.corners = corners
        self.elevation = elevation
        self.front = front
        self.plan = plan
        self.label = label


def hull_view(corners, view):
    """📐️ Convex outline of world points in a view: front (z, y) or plan (x, z)."""
    pts = [(p[2], p[1]) for p in corners] if view == "front" else [(p[0], p[2]) for p in corners]
    return g.hull(pts)


def extrude_xy(id, bone_id, world_poly, z0, z1, region, material, tone, role="body", flags=None):
    """🧱️ A prism of a world xy polygon between lateral positions z0 and z1 on a bone."""
    local = g.ccw(to_local(bone_id, world_poly))
    oz = FR[bone_id][2]
    solid = {"kind": "extrude", "profile": rp(local), "plane": "xy", "from": r(z0 - oz), "to": r(z1 - oz)}
    corners = [(p[0], p[1], z) for p in world_poly for z in (z0, z1)]
    return Part(id, bone_id, region, material, tone, solid, corners, {"kind": "poly", "points": rp(local), "closed": True}, role, flags=flags)


def box_world(id, bone_id, x0, x1, y0, y1, z0, z1, region, material, tone, role="body", flags=None):
    """📦️ An axis-aligned world box as a prism on a bone whose rest frame is unrotated."""
    return extrude_xy(id, bone_id, [(x0, y0), (x1, y0), (x1, y1), (x0, y1)], z0, z1, region, material, tone, role, flags)


def brace(id, bone_id, x0, x1, zy, region, material, tone):
    """📐️ A raking stillage brace: a (z, y) polygon extruded along x."""
    ox, oy, oz, _ = FR[bone_id]
    profile = g.ccw([(p[1] - oy, p[0] - oz) for p in zy])
    solid = {"kind": "extrude", "profile": rp(profile), "plane": "yz", "from": r(x0 - ox), "to": r(x1 - ox)}
    corners = [(x, p[1], p[0]) for p in zy for x in (x0, x1)]
    ys = [p[1] for p in zy]
    elev = to_local(bone_id, [(x0, min(ys)), (x1, min(ys)), (x1, max(ys)), (x0, max(ys))])
    part = Part(id, bone_id, region, material, tone, solid, corners, {"kind": "poly", "points": rp(elev), "closed": True})
    part.front = {"kind": "poly", "points": rp(g.ccw(list(zy))), "closed": True}
    return part


def bolt(id, bone_id, p_world, side, stepped, region="steel-fittings"):
    """🔩️ Washer with nut (left face) or with bolt head (right face) of one M16 bolt, revolved about the bolt axis."""
    u, v = to_local(bone_id, [p_world])[0]
    sign = 1 if side == "l" else -1
    if stepped:
        profile = [[0, 0], [0, 0.029], [0.006, 0.029], [0.006, 0.0135], [0.019, 0.0135], [0.019, 0.008], [0.027, 0.008], [0.027, 0]] if side == "l" else [[0, 0], [0, 0.029], [0.006, 0.029], [0.006, 0.0135], [0.016, 0.0135], [0.016, 0]]
    else:
        profile = [[0, 0], [0, 0.029], [0.020 if side == "l" else 0.016, 0.029], [0.020 if side == "l" else 0.016, 0]]
    depth = profile[-1][0]
    solid = {"kind": "revolve", "profile": profile, "axis": [0, 0, sign], "origin": [r(u), r(v), r(sign * g.PLATE_Z)]}
    corners = [(p_world[0] + dx, p_world[1] + dy, sign * (g.PLATE_Z + dz)) for dx in (-0.029, 0.029) for dy in (-0.029, 0.029) for dz in (0, depth)]
    elev = {"kind": "ellipse", "cx": r(u), "cy": r(v), "rx": 0.029, "ry": 0.029}
    return Part(id, bone_id, region, "steel-primed", 0.22, solid, corners, elev, "detail")


def capsule(id, bone_id, length, radius, region, material, tone, flags=None):
    """💊️ A chain or eye along its bone from 0 to length."""
    solid = {"kind": "capsule", "p0": [0, 0, 0], "p1": [r(length), 0, 0], "r0": radius, "r1": radius}
    corners = []
    for t in (0.0, length):
        for a in range(8):
            ang = 2 * math.pi * a / 8
            corners += to_world(bone_id, [(t + radius * math.cos(ang), radius * math.sin(ang), 0.0), (t, radius * math.cos(ang), radius * math.sin(ang)), (t + radius * math.cos(ang), 0.0, radius * math.sin(ang))])
    elev = {"kind": "capsule", "x0": 0, "y0": 0, "x1": r(length), "y1": 0, "r0": radius, "r1": radius}
    return Part(id, bone_id, region, material, tone, solid, corners, elev, flags=flags)


def band(id, bone_id, s, width, knot, region="round-slings"):
    """🎗️ The round sling choked around a top chord: a ring of webbing (8 mm) around the 80 x 180 mm section, with the merged eye knot on top at coarse scales."""
    hb, bz = g.H_CHORD / 2, g.B_CHORD / 2
    top = hb + g.BAND_T + (knot if knot else 0.0)
    sign = 1 if bone_id == "top-chord-l" else -1
    outer = [(-hb - g.BAND_T, -bz - g.BAND_T), (-hb - g.BAND_T, bz + g.BAND_T), (top, bz + g.BAND_T), (top, -bz - g.BAND_T)]
    outer = [(sign * p[0], p[1]) for p in outer]
    hole = [(-hb, -bz), (-hb, bz), (hb, bz), (hb, -bz)]
    solid = {"kind": "extrude", "profile": rp(sorted_ring(outer)), "holes": [rp(sorted_ring(hole))], "plane": "yz", "from": r(s - width / 2), "to": r(s + width / 2)}
    corners = to_world(bone_id, [(u, p[0], p[1]) for u in (s - width / 2, s + width / 2) for p in outer])
    vs = [p[0] for p in outer]
    elev = {"kind": "poly", "points": rp([(s - width / 2, min(vs)), (s + width / 2, min(vs)), (s + width / 2, max(vs)), (s - width / 2, max(vs))]), "closed": True}
    return Part(id, bone_id, region, "polyester-sling", 0.42, solid, corners, elev)


def sorted_ring(pts):
    """🔄️ A ring counter-clockwise."""
    return g.ccw(list(pts))


def stub(id, bone_id, s):
    """🪚️ The sawn-off stub of a diagonal wind-brace board (24 x 100 mm) still nailed under a top chord, crossing it at 45 degrees in plan."""
    hb = g.H_CHORD / 2
    q = [0, math.sin(math.pi / 8), 0, math.cos(math.pi / 8)]
    solid = {"kind": "box", "size": [0.10, 0.024, 0.40], "transform": {"rotation": [0, r(q[1], 6), 0, r(q[3], 6)], "translation": [r(s), r(-hb - 0.012), 0]}}
    c45 = math.cos(math.pi / 4)
    local = []
    for du in (-0.05, 0.05):
        for dw in (-0.20, 0.20):
            for dv in (-0.012, 0.012):
                local.append((s + du * c45 + dw * c45, -hb - 0.012 + dv, -du * c45 + dw * c45))
    corners = to_world(bone_id, local)
    us = [p[0] for p in local]
    elev = {"kind": "poly", "points": rp([(min(us), -hb - 0.024), (max(us), -hb - 0.024), (max(us), -hb), (min(us), -hb)]), "closed": True}
    return Part(id, bone_id, "truss-timber", "softwood-weathered", 0.62, solid, corners, elev)


def ring(id, bone_id, holed):
    """⭕️ The master link (oval 120 x 70 mm of 16 mm bar) lying on the right slope at rest, hanging from the crane hook when lifted."""
    L, W, t, li, wi = g.RING
    vc = g.R_CHAIN + t / 2 - 0.0005

    def oval(length, width, u0):
        rad = width / 2
        out = []
        for k in range(9):
            a = math.pi * k / 8
            out.append((-rad * math.cos(a), u0 + rad - rad * math.sin(a)))
        for k in range(9):
            a = math.pi * k / 8
            out.append((rad * math.cos(a), u0 + length - rad + rad * math.sin(a)))
        return g.ccw(out)

    outer = oval(L, W, 0.0)
    solid = {"kind": "extrude", "profile": rp(outer), "plane": "zx", "from": r(vc - t / 2), "to": r(vc + t / 2)}
    if holed:
        solid["holes"] = [rp(oval(li, wi, (L - li) / 2))]
    corners = to_world(bone_id, [(p[1], vc + dv, p[0]) for p in outer for dv in (-t / 2, t / 2)])
    elev = {"kind": "poly", "points": rp([(0, vc - t / 2), (L, vc - t / 2), (L, vc + t / 2), (0, vc + t / 2)]), "closed": True}
    part = Part(id, bone_id, "chain-sling", "chain-steel", 0.12, solid, corners, elev)
    if holed:
        inner = to_world(bone_id, [(p[1], vc + t / 2, p[0]) for p in oval(li, wi, (L - li) / 2)])
        part.plan_hole = g.hull([(p[0], p[2]) for p in inner])
    return part


def truss_parts(scale):
    """🪵️ The parts of the truss at one scale."""
    parts = []
    tone = {"bottom": 0.56, "top-l": 0.6, "top-r": 0.58, "bf": 0.64, "fc": 0.62, "gc": 0.63, "gd": 0.65}
    bone_of = {"bottom": "truss", "top-l": "top-chord-l", "top-r": "top-chord-r", "bf": "web-bf", "fc": "web-fc", "gc": "web-gc", "gd": "web-gd"}
    name = {"bottom": "bottom-chord", "top-l": "top-chord-l", "top-r": "top-chord-r", "bf": "web-bf", "fc": "web-fc", "gc": "web-gc", "gd": "web-gd"}
    bz = g.B_CHORD / 2
    for key in ("bottom", "top-l", "top-r", "bf", "fc", "gc", "gd"):
        parts.append(extrude_xy(name[key], bone_of[key], tw(POLYS[key]), -bz, bz, "truss-timber", "softwood-weathered", tone[key]))
    for node, poly in PLATES.items():
        simple = poly if scale <= 100 else g.hull(poly) if node not in "ae" else poly
        for side, z0, z1 in (("l", bz, g.PLATE_Z), ("r", -g.PLATE_Z, -bz)):
            parts.append(extrude_xy(f"gusset-{node}-{side}", GUSSET_BONE_OF(node), tw(simple), z0, z1, "steel-fittings", "steel-primed", 0.3 if side == "r" else 0.34, flags={"sheet": True}))
    if scale <= 100:
        for node, pts in BOLTS.items():
            for i, p in enumerate(pts):
                for side in ("l", "r"):
                    parts.append(bolt(f"bolt-{node}{i + 1}-{side}", GUSSET_BONE_OF(node), tw([p])[0], side, scale == 50))
    if scale == 50:
        for side, x0 in (("l", 0.05), ("r", g.LENGTH - 0.35)):
            parts.append(box_world(f"bearing-plate-{side}", "truss", x0, x0 + 0.30, g.Y_TRUSS - 0.10, g.Y_TRUSS - 0.09, -0.06, 0.06, "steel-fittings", "steel-primed", 0.26))
    if scale <= 100:
        parts.append(stub("brace-stub-l", "top-chord-l", 1.60))
        parts.append(stub("brace-stub-r", "top-chord-r", 1.60))
    width = 0.075 if scale == 500 else g.BAND_W
    knot = None if scale <= 100 else g.CHAIN_OFF + g.R_EYE - g.BAND_T
    parts.append(band("choke-l", "top-chord-l", g.CHOKE_S, width, knot))
    parts.append(band("choke-r", "top-chord-r", g.CHOKE_S, width, knot))
    if scale <= 100:
        for side in ("l", "r"):
            parts.append(capsule(f"eye-{side}", f"eye-{side}", SL["b"], g.R_EYE if scale == 50 else 0.0075, "round-slings", "polyester-sling", 0.4))
    for side in ("l", "r"):
        parts.append(capsule(f"chain-{side}", f"chain-{side}", SL["c"], g.R_CHAIN, "chain-sling", "chain-steel", 0.12, flags={"slender": True}))
    if scale <= 100:
        parts.append(ring("master-link", "master-link", scale == 50))
    return parts


def GUSSET_BONE_OF(node):
    """🏷️ The bone a node's plates and bolts ride on."""
    return g.GUSSET_BONE[node]


def stillage_parts(scale):
    """🪵️ The two timber trestles of the stillage at one scale."""
    parts = []
    sx, sy, sz = g.SLEEPER
    pz0, pz1 = g.POST_Z
    for side, bone_id, x in (("l", "trestle-l", g.TRESTLE_X[0]), ("r", "trestle-r", g.TRESTLE_X[1])):
        parts.append(box_world(f"sleeper-{side}", bone_id, x - sx / 2, x + sx / 2, 0, sy, -sz / 2, sz / 2, "stillage-timber", "softwood-sawn", 0.74))
        for k, z0, z1 in (("a", pz0, pz1), ("b", -pz1, -pz0)):
            parts.append(box_world(f"post-{side}{k}", bone_id, x - 0.05, x + 0.05, sy, sy + g.POST[1], z0, z1, "stillage-timber", "softwood-sawn", 0.7 if k == "a" else 0.76))
            if scale <= 200:
                zy = [(pz1, 0.86), (pz1, 0.72), (0.60, sy), (0.74, sy)]
                if k == "b":
                    zy = [(-p[0], p[1]) for p in zy]
                parts.append(brace(f"brace-{side}{k}", bone_id, x - 0.03, x + 0.03, zy, "stillage-timber", "softwood-sawn", 0.72))
            if scale <= 100:
                w0, w1 = g.WEDGE_Z if k == "a" else (-g.WEDGE_Z[1], -g.WEDGE_Z[0])
                ytop = g.Y_TRUSS + g.under_line(x if side == "l" else 2 * g.RIDGE_X - x) + 0.025
                for j, (y0, y1) in enumerate(((g.Y_TRUSS - 0.07, g.Y_TRUSS + 0.07), (ytop, ytop + 0.14))):
                    parts.append(box_world(f"wedge-{side}{k}{j + 1}", bone_id, x - 0.03, x + 0.03, y0, y1, w0, w1, "stillage-timber", "softwood-sawn", 0.8))
    return parts


def parts_of(scale):
    """🧩️ Every part of one scale."""
    return truss_parts(scale) + stillage_parts(scale)


COARSE_STROKE = {200: 0.18, 500: 0.13}
NEAR_SIDE = "r"


def shape(base, part, geometry, order, filled=True, role=None, region=None):
    """✏️ A 2D shape of a part."""
    out = {"id": base, "bone": part.bone, "role": role or part.role, "z": r(order, 4), "filled": filled, "tone": part.tone, "region": region or part.region, "material": part.material}
    out.update(geometry)
    return out


def views(scale, parts):
    """🖼️ The elevation, front and plan shapes of one scale, every part drawn as a closed filled shape of its true outline (chains as strokes at 1:200 and 1:500)."""
    elev, front, plan = [], [], []
    k = [0]

    def order(value):
        k[0] += 1
        return value + k[0] * 1e-4

    bolt_groups = {}
    for p in parts:
        zs = [c[2] for c in p.corners]
        xs = [c[0] for c in p.corners]
        ys = [c[1] for c in p.corners]
        is_bolt = p.id.startswith("bolt-")
        is_chain = p.id.startswith("chain-")
        coarse = scale >= 200
        if is_bolt:
            node_side = (p.id.split("-")[1].rstrip("0123456789"), p.id.split("-")[2])
            grp = bolt_groups.setdefault(node_side, {"part": p, "corners": []})
            grp["corners"] += p.corners
            if node_side[1] == NEAR_SIDE:
                elev.append(shape(p.id, p, p.elevation, order(20 - min(zs) * 10)))
            continue
        if is_chain and coarse:
            L = SL["c"]
            elev.append(shape(p.id, p, {"kind": "poly", "points": [[0, 0], [r(L), 0]], "closed": False, "penMm": COARSE_STROKE[scale]}, order(20 - min(zs) * 10), filled=False))
            ends = to_world(p.bone, [(0, 0, 0), (L, 0, 0)])
            front.append(shape(p.id, p, {"kind": "poly", "points": rp([(e[2], e[1]) for e in ends]), "closed": False, "penMm": COARSE_STROKE[scale]}, order(max(xs)), filled=False))
            plan.append(shape(p.id, p, {"kind": "poly", "points": rp([(e[0], e[2]) for e in ends]), "closed": False, "penMm": COARSE_STROKE[scale]}, order(max(ys)), filled=False))
            continue
        elev.append(shape(p.id, p, p.elevation, order(20 - min(zs) * 10)))
        if coarse and p.id.startswith("gusset-"):
            continue
        fgeom = p.front if p.front is not None else {"kind": "poly", "points": rp(hull_view(p.corners, "front")), "closed": True}
        front.append(shape(p.id, p, fgeom, order(max(xs))))
        if getattr(p, "plan_hole", None) is not None:
            outer = hull_view(p.corners, "plan")
            d = "M " + " L ".join(f"{r(a)} {r(b)}" for a, b in outer) + " Z M " + " L ".join(f"{r(a)} {r(b)}" for a, b in p.plan_hole) + " Z"
            plan.append(shape(p.id, p, {"kind": "path", "d": d}, order(max(ys))))
        else:
            plan.append(shape(p.id, p, {"kind": "poly", "points": rp(hull_view(p.corners, "plan")), "closed": True}, order(max(ys))))
    for (node, side), grp in sorted(bolt_groups.items()):
        p, cs = grp["part"], grp["corners"]
        zs = [c[2] for c in cs]
        ys = [c[1] for c in cs]
        xs = [c[0] for c in cs]
        fb = [(min(zs), min(ys)), (max(zs), min(ys)), (max(zs), max(ys)), (min(zs), max(ys))]
        pb = [(min(xs), min(zs)), (max(xs), min(zs)), (max(xs), max(zs)), (min(xs), max(zs))]
        front.append(shape(f"bolts-{node}-{side}", p, {"kind": "poly", "points": rp(fb), "closed": True}, order(max(xs)), role="body"))
        plan.append(shape(f"bolts-{node}-{side}", p, {"kind": "poly", "points": rp(pb), "closed": True}, order(max(c[1] for c in cs)), role="body"))
    if scale >= 200:
        bz = g.PLATE_Z
        for e in front:
            if e["bone"] in ("truss", "top-chord-l", "top-chord-r") and e["id"] in ("bottom-chord", "top-chord-l", "top-chord-r"):
                ys = [q[1] for q in e["points"]]
                e["points"] = rp([(-bz, min(ys)), (bz, min(ys)), (bz, max(ys)), (-bz, max(ys))])
                e["regions"] = ["truss-timber", "steel-fittings"]
                del e["region"]
        for e in plan:
            if e["id"] in ("bottom-chord", "top-chord-l", "top-chord-r"):
                xs = [q[0] for q in e["points"]]
                e["points"] = rp([(min(xs), -bz), (max(xs), -bz), (max(xs), bz), (min(xs), bz)])
                e["regions"] = ["truss-timber", "steel-fittings"]
                del e["region"]
    if scale == 50:
        elev += textures_elevation()
        plan += textures_plan()
    return elev, front, plan


def textures_elevation():
    """🕳️ 1:50 traces in the side elevation: the empty 18 mm holes of the removed longitudinal bracing bolts in the long webs, the nail rows of the sawn-off brace stubs."""
    out = []
    for web, sign in (("web-fc", 1), ("web-gc", 1)):
        for i, s in enumerate((0.95, 1.10)):
            out.append({"id": f"brace-bolt-hole-{web}-{i + 1}", "bone": web, "role": "texture", "z": r(30 + i * 0.01 + (0.5 if web == "web-gc" else 0), 3), "filled": False, "tone": 0.2, "region": "truss-timber", "kind": "ellipse", "cx": r(s), "cy": 0, "rx": 0.009, "ry": 0.009})
    return out


def textures_plan():
    """🔨️ 1:50 traces in plan: the nail holes of the removed roof battens, one pair every 330 mm on the top face of each top chord."""
    out = []
    for bone_id, x0, x1, sign in (("top-chord-l", 0.45, 4.55, 1), ("top-chord-r", 4.85, 8.95, -1)):
        x = x0
        i = 0
        while x <= x1 + 1e-9:
            i += 1
            out.append({"id": f"batten-nails-{bone_id}-{i:02d}", "bone": bone_id, "role": "texture", "z": r(40 + i * 0.001 + (0.5 if sign < 0 else 0), 4), "filled": False, "tone": 0.2, "region": "truss-timber", "kind": "poly", "points": [[r(x), -0.02], [r(x), 0.02]], "closed": False})
            x += 0.33
    return out


REFERENCES = [
    ("din-1052-1969", "DIN 1052 Blatt 1 und 2 (1969): Holzbauwerke, Berechnung und Ausführung; Bolzen- und Stabdübelverbindungen (bolted gusset joints of the period, edge and end distances of M16 bolts, square or round washers)."),
    ("din-4074-1", "DIN 4074-1:2012 Sortierung von Holz nach der Tragfähigkeit, Nadelschnittholz (sorting class S10, the historic Güteklasse II, assigned to C24)."),
    ("en-338", "EN 338:2016 Structural timber, strength classes, Table 1: C24 f_m,k 24 MPa, E0,mean 11.0 GPa, rho_mean 420 kg/m3."),
    ("en-1995-1-1", "EN 1995-1-1:2004+A2:2014 Eurocode 5, design of timber structures, 8.5 bolted connections and 9.2 trusses (system lines through the member axes)."),
    ("en-1991-1-3", "EN 1991-1-3:2003 with DIN EN 1991-1-3/NA:2019, snow loads: zone 2, altitude 400 m, s_k = 0.25 + 1.91 ((A + 140) / 760)^2 = 1.21 kN/m2; roof shape coefficient mu_1 = 0.8 for pitches up to 30 degrees; drifted arrangement 0.5 mu_1 on one slope (5.3.3)."),
    ("en-1991-1-1", "EN 1991-1-1:2002 with DIN EN 1991-1-1/NA:2010, Annex A: self-weight of a concrete tile roof covering with battens and counter-battens, about 0.55 kN/m2 of roof surface."),
    ("en-10025-2", "EN 10025-2:2019 Hot rolled products of structural steels, S235JR plates (gusset and bearing plates)."),
    ("iso-4016-din-440", "ISO 4016 hexagon head bolts M16, ISO 4034 nuts M16 (24 mm across flats, 13 mm high) and DIN 440 round washers for timber, 58 mm outside diameter, 6 mm thick."),
    ("en-818-4", "EN 818-4:1996+A1:2008 Short link chain for lifting purposes, chain slings grade 8: 8 mm chain about 1.4 kg/m, two-leg sling working load limit 2.12 t at leg angles 0 to 45 degrees and 1.5 t at 45 to 60 degrees from the vertical."),
    ("en-1492-2", "EN 1492-2:2000+A1:2008 Textile slings, round slings of polyester (WLL 1 t violet, choked use factor 0.8)."),
    ("dguv-r-109-017", "DGUV Regel 109-017 (2020) Betreiben von Lastaufnahmeeinrichtungen im Hebezeugbetrieb: leg angles above 60 degrees from the vertical are not permitted; slinging at the panel points of trusses."),
    ("bcsi-2018", "SBCA and TPI, BCSI 2018 Guide to Good Practice for Handling, Installing, Restraining and Bracing of Wood Trusses, B1: store trusses upright on blocking at the panel points or bearings, braced against toppling; lift at the top chord panel points with a two-leg sling."),
    ("hillebrandt-2018", "Hillebrandt, A., Riegler-Floors, P., Rosen, A., Seggewies, J.-K. (2018) Atlas Recycling: Gebäude als Materialressource, Detail, Munich (dismantling, storage and reuse of timber trusses through component exchanges)."),
    ("en-iso-10456", "EN ISO 10456:2007 Building materials and products, hygrothermal properties: softwood 0.13 W/mK and 1600 J/kgK, steel 50 W/mK and 450 J/kgK, polyester about 0.2 W/mK."),
    ("en-iso-13786", "EN ISO 13786:2017 Dynamic thermal characteristics: periodic penetration depth; about half of the 24 h depth (36 mm for softwood) is taken as the storing depth of an exposed timber face."),
    ("ashrae-2021-radiation", "ASHRAE Handbook Fundamentals (2021), chapter 4, radiative properties of surfaces: weathered wood emissivity about 0.9 and solar absorptance 0.6 to 0.75 (grey weathered higher than planed new spruce, 0.45 to 0.55); oxidised and primed steel 0.8 to 0.9."),
    ("din-1356-1", "DIN 1356-1:1995 Bauzeichnungen, Arten, Inhalte und Grundregeln der Darstellung (line groups 0.35/0.18/0.1 at 1:50)."),
    ("din-iso-128-50", "DIN ISO 128-50:2002 Technical drawings, basic conventions for areas on cuts and sections."),
]


def material(id, en, de, eps, alpha_s, rho, k, c, src, ref, E=None, f=None):
    """🧱️ A material entry."""
    m = {"id": id, "title": T(en, de), "emissivity": eps, "solarAbsorptivity": alpha_s, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": c, "source": src, "reference": ref}
    if E:
        m["elasticModulusPa"] = E
    if f:
        m["strengthPa"] = f
    return m


MATERIALS = [
    material("softwood-weathered", "Weathered spruce C24 (truss timber, grey patina)", "Verwitterte Fichte C24 (Binderholz, graue Patina)", 0.9, 0.7, 420, 0.13, 1600, "literature", "en-338", 11.0e9, 24e6),
    material("softwood-sawn", "Sawn spruce (stillage, fresh)", "Fichte sägerau (Lagerbock, frisch)", 0.9, 0.55, 450, 0.13, 1600, "authored-estimate", "en-iso-10456"),
    material("steel-primed", "Steel S235, red-lead primed with rust patina", "Stahl S235, mennigegrundiert mit Rostpatina", 0.85, 0.75, 7850, 50, 450, "literature", "en-10025-2", 210e9, 235e6),
    material("chain-steel", "Alloy chain steel grade 8, red powder coat", "Legierter Kettenstahl Güteklasse 8, rot pulverbeschichtet", 0.9, 0.65, 7850, 40, 460, "literature", "en-818-4", 210e9, 800e6),
    material("polyester-sling", "Polyester round sling, violet", "Polyester-Rundschlinge, violett", 0.9, 0.7, 1380, 0.2, 1300, "authored-estimate", "en-1492-2"),
]


def region(id, en, de, mat, area_m2, char, geom, sky, cap, ref, src="authored-estimate"):
    """🌡️ A passive thermal region."""
    return {"id": id, "title": T(en, de), "kind": "surface", "model": "passive", "material": mat, "areaM2": area_m2, "characteristicLengthM": char, "geometry": geom, "skyViewFactor": sky, "layers": [], "heatCapacityJM2K": cap, "source": src, "reference": ref}


REGIONS = [
    region("truss-timber", "Truss timber faces (weathered)", "Holzflächen des Binders (verwittert)", "softwood-weathered", AREAS["truss-timber"], 0.18, "vertical-plate", 0.5, 24000, "en-iso-13786"),
    region("steel-fittings", "Gusset plates, bolts and bearing plates", "Knotenbleche, Bolzen und Lagerplatten", "steel-primed", AREAS["steel-fittings"], 0.4, "vertical-plate", 0.5, 10600, "en-iso-10456"),
    region("stillage-timber", "Stillage trestles", "Lagerböcke", "softwood-sawn", AREAS["stillage-timber"], 0.12, "vertical-plate", 0.45, 26000, "en-iso-10456"),
    region("chain-sling", "Chain legs and master link", "Kettenstränge und Aufhängering", "chain-steel", AREAS["chain-sling"], 0.018, "cylinder", 0.5, 11300, "en-818-4"),
    region("round-slings", "Choked round slings", "Geschnürte Rundschlingen", "polyester-sling", AREAS["round-slings"], 0.05, "cylinder", 0.5, 14000, "en-1492-2"),
]


def binding(sun_truss, sun_yard):
    """🌞️ The region binding of a pose: the truss in the yard sun or under the roof covering, the stillage always in the yard."""
    return {"regions": {"truss-timber": {"exposedFraction": 1, "sunlitFraction": sun_truss}, "steel-fittings": {"exposedFraction": 1, "sunlitFraction": sun_truss}, "stillage-timber": {"exposedFraction": 1, "sunlitFraction": sun_yard}, "chain-sling": {"exposedFraction": 1, "sunlitFraction": sun_truss}, "round-slings": {"exposedFraction": 1, "sunlitFraction": sun_truss}}, "emitters": {}}


L_TC = math.hypot(g.C[0] - g.A[0], g.C[1] - g.A[1])
SNOW = 0.8 * (0.25 + 1.91 * ((400 + 140) / 760) ** 2) * 1.0 * g.COS * 1000
DEAD = 0.55 * 1.0 * 1000


def load(id, kind, en, de, bone_id, w, ref):
    """🧲️ A load distributed along a top chord."""
    return {"id": id, "kind": kind, "title": T(en, de), "fxN": 0, "fyN": 0, "distributed": {"bone": bone_id, "fromM": 0, "toM": r(L_TC), "wxNPerM": 0, "wyNPerM": r(-w, 1)}, "source": "literature", "reference": ref}


THETA = SL["theta"]
SLUNG = {"eye-l": {"angleDeg": r(-(180 - THETA), 4)}, "chain-l": {"angleDeg": 180}, "master-link": {"angleDeg": r(90 + g.ALPHA - THETA, 4)}, "eye-r": {"angleDeg": r(180 - THETA, 4)}, "chain-r": {"angleDeg": -180}}
OUT = {"trestle-l": {"dx": -g.SLIDE}, "trestle-r": {"dx": g.SLIDE}}


def pose(id, en, de, den, dde, bones, contacts, loads, sun_truss, key=False):
    """🧍️ A pose."""
    p = {"id": id, "title": T(en, de), "description": T(den, dde), "bones": bones, "ground": "authored", "contacts": contacts, "thermal": binding(sun_truss, 0.5), "loads": loads}
    if key:
        p["key"] = True
    return p


DEAD_L = load("roof-dead-l", "carried", "Roof covering, left slope 0.55 kN/m", "Dachdeckung, linke Dachfläche 0,55 kN/m", "top-chord-l", DEAD, "en-1991-1-1")
DEAD_R = load("roof-dead-r", "carried", "Roof covering, right slope 0.55 kN/m", "Dachdeckung, rechte Dachfläche 0,55 kN/m", "top-chord-r", DEAD, "en-1991-1-1")
SNOW_L = load("snow-l", "snow", f"Snow, left slope {SNOW / 1000:.2f} kN/m".replace(".", "."), f"Schnee, linke Dachfläche {SNOW / 1000:.2f} kN/m".replace(".", ","), "top-chord-l", SNOW, "en-1991-1-3")
SNOW_R = load("snow-r", "snow", f"Snow, right slope {SNOW / 1000:.2f} kN/m", f"Schnee, rechte Dachfläche {SNOW / 1000:.2f} kN/m".replace(".", ","), "top-chord-r", SNOW, "en-1991-1-3")
DRIFT_R = load("snow-r", "snow", f"Snow, right slope halved {SNOW / 2000:.2f} kN/m", f"Schnee, rechte Dachfläche halbiert {SNOW / 2000:.2f} kN/m".replace(".", ","), "top-chord-r", SNOW / 2, "en-1991-1-3")
SET = {"truss": {"dy": g.SET_DOWN}, **OUT}

POSES = [
    pose("stored", "Stored upright in the stillage", "Aufrecht im Lagerbock gelagert", "The truss stands upright on the two sleepers of the timber stillage, wedged between the posts; the chain sling stays choked at the top chord panel points, its legs laid along the top chords to the ridge.", "Der Binder steht aufrecht auf den beiden Schwellen des Holzlagerbocks, zwischen den Pfosten verkeilt; das Kettengehänge bleibt an den Obergurtknoten angeschlagen, seine Stränge liegen auf den Obergurten bis zum First.", {}, ["bearer-l", "bearer-r"], [], 0.5),
    pose("slung", "Slung, sling taut", "Angeschlagen, Gehänge gespannt", "Key pose of the lift: the crane hook has taken up the sling, the legs stand at 59 degrees from the vertical, the truss still rests on the sleepers.", "Schlüsselpose des Hubs: der Kranhaken hat das Gehänge gespannt, die Stränge stehen 59 Grad gegen die Senkrechte, der Binder liegt noch auf den Schwellen.", SLUNG, ["bearer-l", "bearer-r", "sling-l", "sling-r"], [], 0.5, key=True),
    pose("lifted", "Lifted by the two-leg sling", "Am Zweistranggehänge angehoben", f"The truss hangs 0.60 m above the stillage from the two chain legs choked at the top chord panel points (leg angle {g.BETA:.0f} degrees from the vertical, below the permitted 60); the stillage stays on the ground.", f"Der Binder hängt 0,60 m über dem Lagerbock an den zwei Kettensträngen, die an den Obergurtknoten angeschnürt sind (Neigungswinkel {g.BETA:.0f} Grad gegen die Senkrechte, unter den zulässigen 60); der Lagerbock bleibt am Boden.", {**SLUNG, "truss": {"dy": g.LIFT}}, ["sling-l", "sling-r"], [], 0.5),
    pose("cleared", "Lifted, stillage moved aside", "Angehoben, Lagerbock beiseite", "Key pose of the set-down: the truss still hangs in the sling, both trestles are pushed out beyond its ends.", "Schlüsselpose des Absetzens: der Binder hängt noch im Gehänge, beide Böcke sind über seine Enden hinaus weggeschoben.", {**SLUNG, "truss": {"dy": g.LIFT}, **OUT}, ["sling-l", "sling-r"], [], 0.5, key=True),
    pose("set-down", "Set down on its bearings, sling still taut", "Auf die Auflager abgesetzt, Gehänge noch gespannt", "Key pose of the set-down: the heel bearing plates stand on the wall plates (the ground line), the sling is still taut.", "Schlüsselpose des Absetzens: die Auflagerplatten der Traufknoten stehen auf den Fußpfetten (der Bodenlinie), das Gehänge ist noch gespannt.", {**SLUNG, **SET}, ["heel-l", "heel-r", "sling-l", "sling-r"], [], 0.0, key=True),
    pose("reinstalled-dead", "Reinstalled, roof covering only", "Wieder eingebaut, nur Dachdeckung", "Back in a roof at 1.0 m spacing: pin bearing at the left heel, roller at the right heel on the wall plates, the 0.55 kN/m2 tile covering on both top chords.", "Wieder in einem Dach mit 1,0 m Binderabstand: Festlager am linken, Rollenlager am rechten Traufknoten auf den Fußpfetten, die Ziegeldeckung 0,55 kN/m2 auf beiden Obergurten.", SET, ["heel-l", "heel-r"], [DEAD_L, DEAD_R], 0.0),
    pose("reinstalled-snow", "Reinstalled under full snow", "Wieder eingebaut unter Vollschnee", "Roof covering plus the snow load of zone 2 at 400 m (s_k 1.21 kN/m2, mu_1 0.8) on both slopes.", "Dachdeckung und Schneelast der Zone 2 auf 400 m (s_k 1,21 kN/m2, mu_1 0,8) auf beiden Dachflächen.", SET, ["heel-l", "heel-r"], [DEAD_L, DEAD_R, SNOW_L, SNOW_R], 0.0),
    pose("reinstalled-drift", "Reinstalled under drifted snow", "Wieder eingebaut unter verwehtem Schnee", "Roof covering, full snow on the left slope and half the snow on the right slope, the unbalanced arrangement of EN 1991-1-3 5.3.3.", "Dachdeckung, volle Schneelast auf der linken und halbe auf der rechten Dachfläche, die unsymmetrische Anordnung nach EN 1991-1-3 5.3.3.", SET, ["heel-l", "heel-r"], [DEAD_L, DEAD_R, SNOW_L, DRIFT_R], 0.0),
]

CLIPS = [
    {"id": "lift", "title": T("Lift out of the stillage", "Aus dem Lagerbock heben"), "description": T("The crane takes up the sling (the choked eyes stand up and the legs straighten), the load passes from the sleepers to the two legs and the truss rises 0.60 m clear of the stillage.", "Der Kran spannt das Gehänge (die geschnürten Augen richten sich auf, die Stränge strecken sich), die Last geht von den Schwellen auf die zwei Stränge über und der Binder hebt sich 0,60 m über den Lagerbock."), "durationS": 5, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 2.0, "pose": "slung", "ease": "smooth"}, {"t": 2.6, "pose": "slung", "ease": "smooth"}, {"t": 5.0, "pose": "lifted", "ease": "smooth"}],
     "contacts": [{"anchor": "bearer-l", "fromS": 0, "toS": 2.6}, {"anchor": "bearer-r", "fromS": 0, "toS": 2.6}, {"anchor": "sling-l", "fromS": 2.0, "toS": 5.0}, {"anchor": "sling-r", "fromS": 2.0, "toS": 5.0}],
     "phases": [{"id": "rigging", "fromS": 0, "toS": 2.0, "title": T("Sling taken up", "Gehänge gespannt")}, {"id": "take-up", "fromS": 2.0, "toS": 2.6, "title": T("Load transfer", "Lastübergabe")}, {"id": "hoist", "fromS": 2.6, "toS": 5.0, "title": T("Hoist", "Heben")}],
     "events": [{"t": 2.0, "kind": "contact-on", "title": T("Sling legs take load", "Stränge nehmen Last auf"), "target": "sling-l"}, {"t": 2.6, "kind": "contact-off", "title": T("Truss leaves the sleepers", "Binder verlässt die Schwellen"), "target": "bearer-l"}], "tracks": []},
    {"id": "set-down", "title": T("Set down on the wall plates", "Auf die Fußpfetten absetzen"), "description": T("Hanging in the sling, the truss waits while both trestles are pushed aside, is lowered onto its heel bearings and the sling goes slack and is laid back on the top chords.", "Im Gehänge hängend wartet der Binder, bis beide Böcke beiseitegeschoben sind, wird auf seine Traufauflager abgesenkt, und das Gehänge wird schlaff und auf die Obergurte zurückgelegt."), "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "lifted", "ease": "smooth"}, {"t": 2.0, "pose": "cleared", "ease": "linear"}, {"t": 4.5, "pose": "set-down", "ease": "smooth"}, {"t": 6.0, "pose": "reinstalled-dead", "ease": "smooth"}],
     "contacts": [{"anchor": "sling-l", "fromS": 0, "toS": 4.6}, {"anchor": "sling-r", "fromS": 0, "toS": 4.6}, {"anchor": "heel-l", "fromS": 4.4, "toS": 6.0}, {"anchor": "heel-r", "fromS": 4.4, "toS": 6.0}],
     "phases": [{"id": "clear", "fromS": 0, "toS": 2.0, "title": T("Stillage pushed aside", "Lagerbock beiseite")}, {"id": "lower", "fromS": 2.0, "toS": 4.5, "title": T("Lowering", "Absenken")}, {"id": "release", "fromS": 4.5, "toS": 6.0, "title": T("Sling released", "Gehänge gelöst")}],
     "events": [{"t": 4.5, "kind": "contact-on", "title": T("Heels on the wall plates", "Traufknoten auf den Fußpfetten"), "target": "heel-l"}, {"t": 4.6, "kind": "contact-off", "title": T("Sling slack", "Gehänge schlaff"), "target": "sling-l"}], "tracks": []},
    {"id": "snow-accumulation", "title": T("Snow accumulates on the reinstalled truss", "Schnee sammelt sich auf dem wieder eingebauten Binder"), "description": T("Over a snowfall the snow load on both slopes grows from nothing to the characteristic value of zone 2 at 400 m and holds.", "Während eines Schneefalls wächst die Schneelast auf beiden Dachflächen von null auf den charakteristischen Wert der Zone 2 auf 400 m und bleibt."), "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "reinstalled-snow", "ease": "smooth"}, {"t": 8, "pose": "reinstalled-snow", "ease": "smooth"}],
     "contacts": [{"anchor": "heel-l", "fromS": 0, "toS": 8}, {"anchor": "heel-r", "fromS": 0, "toS": 8}],
     "phases": [{"id": "snowfall", "fromS": 0, "toS": 6, "title": T("Snowfall", "Schneefall")}, {"id": "hold", "fromS": 6, "toS": 8, "title": T("Full snow", "Vollschnee")}],
     "events": [{"t": 6, "kind": "load-on", "title": T("Snow at its characteristic value", "Schnee auf charakteristischem Wert"), "target": "snow-l"}],
     "tracks": [{"target": "load:snow-l", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 6, "v": 1}, {"t": 8, "v": 1}]}, {"target": "load:snow-r", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 6, "v": 1}, {"t": 8, "v": 1}]}]},
]


def anchors():
    """📍️ Contacts on the sleepers, the sling points at the chokes and the heel bearings."""
    hb = g.H_CHORD / 2
    lifted = g.ALPHA + THETA
    out = [
        {"id": "bearer-l", "bone": "truss", "x": g.TRESTLE_X[0], "y": -hb, "kind": "contact", "title": T("Bottom chord on the left sleeper", "Untergurt auf der linken Schwelle"), "support": "contact", "normalDeg": 90, "footprintLengthM": g.SLEEPER[0], "footprintWidthM": g.B_CHORD, "frictionCoefficient": 0.5},
        {"id": "bearer-r", "bone": "truss", "x": g.TRESTLE_X[1], "y": -hb, "kind": "contact", "title": T("Bottom chord on the right sleeper", "Untergurt auf der rechten Schwelle"), "support": "contact", "normalDeg": 90, "footprintLengthM": g.SLEEPER[0], "footprintWidthM": g.B_CHORD, "frictionCoefficient": 0.5},
        {"id": "sling-l", "bone": "top-chord-l", "x": r(g.CHOKE_S), "y": r(hb + g.BAND_T), "kind": "support", "title": T("Left sling point (choke at the top chord panel point), along the leg", "Linker Anschlagpunkt (Schnürung am Obergurtknoten), entlang des Strangs"), "support": "roller", "normalDeg": r(lifted, 3)},
        {"id": "sling-r", "bone": "top-chord-r", "x": r(g.CHOKE_S), "y": r(-(hb + g.BAND_T)), "kind": "support", "title": T("Right sling point (choke at the top chord panel point), along the leg", "Rechter Anschlagpunkt (Schnürung am Obergurtknoten), entlang des Strangs"), "support": "roller", "normalDeg": r(180 - lifted, 3)},
        {"id": "heel-l", "bone": "truss", "x": g.HEEL_X, "y": -hb - 0.010, "kind": "support", "title": T("Left heel on the wall plate, pin", "Linker Traufknoten auf der Fußpfette, Festlager"), "support": "pin", "footprintLengthM": 0.30, "footprintWidthM": 0.12},
        {"id": "heel-r", "bone": "truss", "x": r(g.HEEL_X + g.SPAN), "y": -hb - 0.010, "kind": "support", "title": T("Right heel on the wall plate, roller", "Rechter Traufknoten auf der Fußpfette, Rollenlager"), "support": "roller", "normalDeg": 90, "footprintLengthM": 0.30, "footprintWidthM": 0.12},
        {"id": "crane-hook", "bone": "master-link", "x": g.RING[0], "y": 0, "kind": "attachment", "title": T("Crane hook in the master link", "Kranhaken im Aufhängering"), "z": 0},
    ]
    return out


MEMBER_BONE = {"bottom-chord": "truss", "top-chord-l": "top-chord-l", "top-chord-r": "top-chord-r", "web-bf": "web-bf", "web-fc": "web-fc", "web-gc": "web-gc", "web-gd": "web-gd", "chain-l": "chain-l", "chain-r": "chain-r"}


def volume(scale, parts, decisions):
    """🧊️ The authored volume of one scale: body solids, thermal and structural references."""
    body = [p.solid for p in parts]
    thermal = [{"id": f"th-{p.id}", "of": p.id, "region": p.region} for p in parts]
    structure = []
    for p in parts:
        if p.id in MEMBER_BONE:
            structure.append({"id": f"st-{p.id}", "of": p.id, "member": MEMBER_BONE[p.id], "part": "member"})
        elif p.id.startswith(("choke-", "eye-")) or (p.id.startswith("gusset-") and (scale == 50 or p.id.endswith("-r"))):
            structure.append({"id": f"st-{p.id}", "of": p.id, "member": p.bone, "part": "joint"})
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


PENS = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35}, 200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": 0.1, "ground": 0.25}, 500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def mm(x, scale):
    """📏️ Paper millimetres of a real length at a scale, as text."""
    return f"{x * 1000 / scale:.2f}"


def rep_decisions(scale):
    """📝️ The eight drawing decisions of one scale."""
    s = scale
    span = mm(g.LENGTH, s)
    if s == 50:
        return {
            "silhouette": T(f"Side elevation of the whole W-truss ({span} mm long, {mm(2.41, s)} mm high on paper): bottom chord, both top chords and the four webs in their true 80 x 180 and 80 x 120 mm sections, cut at the joints as built; seven pairs of 6 mm steel gusset plates with every M16 bolt as its 58 mm washer; the two heel bearing plates; the choked round slings, eyes and chain legs laid up the top chords to the master link on the ridge; the two trestles of the stillage with sleeper, posts, raking braces and folding wedges. Front view: the truss end-on as a 92 mm strip (132 mm over the nuts) between the posts of the near trestle; plan: the truss strip with the bolt ends at the nodes, the sawn-off wind-brace stubs crossing under the top chords at 45 degrees and the two sleepers.", f"Seitenansicht des ganzen W-Binders ({span} mm lang, {mm(2.41, s)} mm hoch auf dem Papier): Untergurt, beide Obergurte und die vier Diagonalen in ihren wahren Querschnitten 80 x 180 und 80 x 120 mm, an den Knoten so geschnitten wie gebaut; sieben Paare Stahlknotenbleche 6 mm mit jedem M16-Bolzen als seine 58-mm-Scheibe; die beiden Auflagerplatten der Traufknoten; die geschnürten Rundschlingen, Augen und Kettenstränge auf den Obergurten bis zum Aufhängering am First; die beiden Böcke des Lagergestells mit Schwelle, Pfosten, Streben und Keilpaaren. Vorderansicht: der Binder von der Stirn als 92-mm-Streifen (132 mm über den Muttern) zwischen den Pfosten des vorderen Bocks; Draufsicht: der Binderstreifen mit den Bolzenenden an den Knoten, die abgesägten Windrispenstummel unter den Obergurten unter 45 Grad und die beiden Schwellen."),
            "retained": T("Every bolt (38 per face), the empty 18 mm holes of the removed longitudinal bracing bolts in both long webs, the batten nail pairs at 330 mm on the top chords in plan, the sawn wind-brace stubs, the heel bearing plates with their 120 mm width standing 20 mm proud of the chord in plan, the choke eyes (12 mm) and the 18 mm chain legs.", "Jeder Bolzen (38 je Seite), die leeren 18-mm-Löcher der ausgebauten Längsverbandsbolzen in beiden langen Diagonalen, die Lattennagelpaare im Abstand 330 mm auf den Obergurten in der Draufsicht, die abgesägten Windrispenstummel, die Auflagerplatten mit ihrer Breite von 120 mm, die in der Draufsicht 20 mm über den Gurt stehen, die Schnüraugen (12 mm) und die 18-mm-Kettenstränge."),
            "merged": T("Washer, nut and bolt end of each bolt are one circle in elevation and one strip per node and face in front and plan; the chain links are one 18 mm envelope per leg.", "Scheibe, Mutter und Bolzenende jedes Bolzens sind in der Ansicht ein Kreis und in Vorder- und Draufsicht ein Streifen je Knoten und Seite; die Kettenglieder sind eine 18-mm-Hülle je Strang."),
            "omitted": T("Bolts of the far face (hidden behind the near plates), grain, nail heads of the stubs and the slotted holes of the bearing plates (they face the ground).", "Bolzen der hinteren Seite (hinter den vorderen Blechen verdeckt), Maserung, Nagelköpfe der Stummel und die Langlöcher der Auflagerplatten (sie zeigen zum Boden)."),
            "thinElements": T("The 6 mm gusset plates are drawn at their true outline in elevation and at true 6 mm in front and plan (0.12 mm plus the 0.35 mm body pen); the 10 mm bearing plates are true; chain legs true at 18 mm.", "Die 6-mm-Knotenbleche sind in der Ansicht in ihrer wahren Kontur und in Vorder- und Draufsicht mit wahren 6 mm gezeichnet (0,12 mm plus 0,35-mm-Körperstift); die 10-mm-Auflagerplatten sind wahr; die Kettenstränge wahr mit 18 mm."),
            "hatching": T("No hatch: no view is a cut (DIN ISO 128-50); weathering is the darker tone of the truss against the fresh stillage.", "Keine Schraffur: keine Ansicht ist ein Schnitt (DIN ISO 128-50); die Verwitterung ist der dunklere Ton des Binders gegenüber dem frischen Lagerbock."),
            "styles": T("Outlines 0.35 mm, bolts 0.18 mm, holes and nail marks 0.1 mm (DIN 1356-1 line group).", "Konturen 0,35 mm, Bolzen 0,18 mm, Löcher und Nagelspuren 0,1 mm (Liniengruppe DIN 1356-1)."),
            "overlays": T("Members, plates, bolts, slings and trestle parts are separate cells; the textures lie on their members.", "Stäbe, Bleche, Bolzen, Schlingen und Bockteile sind eigene Zellen; die Texturen liegen auf ihren Stäben."),
        }
    if s == 100:
        return {
            "silhouette": T(f"The same truss at {span} mm: true members and gusset outlines, every bolt as a 58 mm dot (0.58 mm), slings, eyes, chain legs and master link, the stillage with braces and wedges; front and plan as at 1:50 without the bearing plates.", f"Derselbe Binder mit {span} mm: wahre Stäbe und Blechkonturen, jeder Bolzen als 58-mm-Punkt (0,58 mm), Schlingen, Augen, Kettenstränge und Aufhängering, der Lagerbock mit Streben und Keilen; Vorder- und Draufsicht wie bei 1:50 ohne die Auflagerplatten."),
            "retained": T("Bolt pattern of every node, the wind-brace stubs (24 mm, 0.24 mm), wedges (35 mm), eyes drawn 15 mm.", "Bolzenbild jedes Knotens, die Windrispenstummel (24 mm, 0,24 mm), Keile (35 mm), Augen 15 mm gezeichnet."),
            "merged": T("Washer, nut and bolt end are one flat cylinder of 20 mm (16 mm on the head side); in front and plan the bolt ends of a node are one strip standing 20 mm (16 mm) proud of the plate, drawn with the body pen because it is the outline of the view there, so it prints to 1:150; the master link is a full oval plate.", "Scheibe, Mutter und Bolzenende sind ein flacher Zylinder von 20 mm (16 mm auf der Kopfseite); in Vorder- und Draufsicht sind die Bolzenenden eines Knotens ein Streifen, der 20 mm (16 mm) über das Blech steht, mit dem Körperstift gezeichnet, weil er dort die Kontur der Ansicht ist, damit er bis 1:150 druckt; der Aufhängering ist eine volle ovale Scheibe."),
            "omitted": T("The 10 mm heel bearing plates (0.1 mm), the bracing bolt holes, the batten nail marks and the ring opening of the master link.", "Die 10-mm-Auflagerplatten (0,1 mm), die Löcher der Verbandsbolzen, die Lattennagelspuren und die Öffnung des Aufhängerings."),
            "thinElements": T("Gusset plates stay true 6 mm in front and plan (0.06 mm plus the 0.25 mm pen); eyes drawn 15 mm thick (one feature) instead of 12 mm.", "Knotenbleche bleiben in Vorder- und Draufsicht wahre 6 mm (0,06 mm plus 0,25-mm-Stift); Augen 15 mm statt 12 mm gezeichnet (ein Merkmal)."),
            "hatching": T("No hatch: no view is a cut.", "Keine Schraffur: keine Ansicht ist ein Schnitt."),
            "styles": T("Outlines 0.25 mm, bolts 0.13 mm.", "Konturen 0,25 mm, Bolzen 0,13 mm."),
            "overlays": T("Members, plates, bolt dots, slings and trestle parts are separate cells.", "Stäbe, Bleche, Bolzenpunkte, Schlingen und Bockteile sind eigene Zellen."),
        }
    if s == 200:
        return {
            "silhouette": T(f"The truss at {span} mm with its seven members in true section, the gusset outlines filled (the steel of every node), the chokes with their eye knot as one small block, the chain legs as 0.18 mm strokes, the trestles with sleeper, posts and braces. Front and plan: the truss as one 92 mm strip including the plates (0.46 mm).", f"Der Binder mit {span} mm, seine sieben Stäbe in wahrem Querschnitt, die Blechkonturen gefüllt (der Stahl jedes Knotens), die Schnürungen mit ihrem Augenknoten als ein kleiner Block, die Kettenstränge als 0,18-mm-Striche, die Böcke mit Schwelle, Pfosten und Streben. Vorder- und Draufsicht: der Binder als ein 92-mm-Streifen einschließlich der Bleche (0,46 mm)."),
            "retained": T("Member widths 180 mm (0.9 mm) and 120 mm (0.6 mm), plate outlines, the leg angle of the sling, the trestle braces (100 mm).", "Stabbreiten 180 mm (0,9 mm) und 120 mm (0,6 mm), Blechkonturen, der Winkel der Stränge, die Bockstreben (100 mm)."),
            "merged": T("The plates of both faces into the truss strip of front and plan; the eye into the choke knot; the convex hull is the outline of the intermediate plates.", "Die Bleche beider Seiten in den Binderstreifen von Vorder- und Draufsicht; das Auge in den Schnürknoten; die konvexe Hülle ist die Kontur der Zwischenbleche."),
            "omitted": T("Bolts (58 mm, 0.29 mm), wedges, wind-brace stubs, master link and bearing plates.", "Bolzen (58 mm, 0,29 mm), Keile, Windrispenstummel, Aufhängering und Auflagerplatten."),
            "thinElements": T("The 18 mm chain legs are strokes of 0.18 mm pen (36 mm), the model keeps them true and slender.", "Die 18-mm-Kettenstränge sind Striche mit 0,18-mm-Stift (36 mm), das Modell behält sie wahr und schlank."),
            "hatching": T("No hatch.", "Keine Schraffur."),
            "styles": T("Outlines 0.18 mm, chains 0.18 mm strokes.", "Konturen 0,18 mm, Ketten als 0,18-mm-Striche."),
            "overlays": T("Members, plates, chokes, chain strokes and trestle parts are separate cells.", "Stäbe, Bleche, Schnürungen, Kettenstriche und Bockteile sind eigene Zellen."),
        }
    return {
        "silhouette": T(f"The truss at {span} mm long and {mm(2.41, s)} mm high: chords and webs as filled bars (0.36 and 0.24 mm), the node plates as filled patches, the chokes as 75 mm blocks, the legs as 0.13 mm strokes, each trestle as sleeper and two posts.", f"Der Binder mit {span} mm Länge und {mm(2.41, s)} mm Höhe: Gurte und Diagonalen als gefüllte Balken (0,36 und 0,24 mm), die Knotenbleche als gefüllte Flecken, die Schnürungen als 75-mm-Blöcke, die Stränge als 0,13-mm-Striche, jeder Bock als Schwelle und zwei Pfosten."),
        "retained": T("The W of the webs, the pitch of 26 degrees, the ridge, both heels, the position of the trestles and the two sling legs.", "Das W der Diagonalen, die Neigung von 26 Grad, der First, beide Traufknoten, die Lage der Böcke und die zwei Stränge."),
        "merged": T("Plates into the truss strip in front and plan; posts and wedges into one post per side; the eye knot into the choke block, widened from 50 to 75 mm along the chord.", "Bleche in den Binderstreifen von Vorder- und Draufsicht; Pfosten und Keile in einen Pfosten je Seite; der Augenknoten in den Schnürblock, längs des Gurts von 50 auf 75 mm verbreitert."),
        "omitted": T("Bolts, braces of the trestles (60 mm thick), wedges, stubs, master link, bearing plates.", "Bolzen, Streben der Böcke (60 mm dick), Keile, Stummel, Aufhängering, Auflagerplatten."),
        "thinElements": T("The 80 mm front width of the members is 0.16 mm plus the 0.13 mm pen (0.29 mm, 0.21 mm at 1:1000); the chain legs are 0.13 mm strokes.", "Die Vorderansichtsbreite der Stäbe von 80 mm ist 0,16 mm plus 0,13-mm-Stift (0,29 mm, bei 1:1000 0,21 mm); die Kettenstränge sind 0,13-mm-Striche."),
        "hatching": T("No hatch.", "Keine Schraffur."),
        "styles": T("Outlines 0.13 mm.", "Konturen 0,13 mm."),
        "overlays": T("Members, plates, chokes, chain strokes and trestle parts are separate cells.", "Stäbe, Bleche, Schnürungen, Kettenstriche und Bockteile sind eigene Zellen."),
    }


VOL_DECISIONS = {
    50: [T("Body at 1:50: the seven members as prisms of their true cut outlines (80 mm thick, 180 mm chords and 120 mm webs), fourteen 6 mm gusset plates declared as sheets, every M16 bolt as a stepped revolve of washer 58 x 6 mm, nut 27 x 13 mm and bolt end 16 mm (head side: washer and 10 mm head), the two 300 x 120 x 10 mm heel bearing plates, the sawn wind-brace stubs (24 x 100 x 400 mm at 45 degrees), the choked round slings (8 mm webbing around the chord) with their 12 mm eyes, the 18 mm envelopes of the 8 mm chain legs (slender) and the holed master link; the stillage as sleepers, posts, raking braces and folding wedges.", "Körper bei 1:50: die sieben Stäbe als Prismen ihrer wahren geschnittenen Konturen (80 mm dick, Gurte 180 mm, Diagonalen 120 mm), vierzehn 6-mm-Knotenbleche als Bleche erklärt, jeder M16-Bolzen als gestufter Drehkörper aus Scheibe 58 x 6 mm, Mutter 27 x 13 mm und Bolzenende 16 mm (Kopfseite: Scheibe und 10-mm-Kopf), die beiden Auflagerplatten 300 x 120 x 10 mm, die abgesägten Windrispenstummel (24 x 100 x 400 mm unter 45 Grad), die geschnürten Rundschlingen (8-mm-Band um den Gurt) mit ihren 12-mm-Augen, die 18-mm-Hüllen der 8-mm-Kettenstränge (schlank) und der offene Aufhängering; der Lagerbock aus Schwellen, Pfosten, Streben und Keilpaaren."),
         T("Thermal: every body solid by reference, bound to its region (truss timber, steel fittings, stillage timber, chain sling, round slings). Structure: the seven members and the two chain legs as members, the gusset plates, chokes and eyes as joints; the stillage carries no structural solid, the truss stands on contact anchors at the sleepers.", "Thermisch: jeder Körper per Verweis, seiner Region zugeordnet (Binderholz, Stahlteile, Bockholz, Kettengehänge, Rundschlingen). Tragwerk: die sieben Stäbe und die zwei Kettenstränge als Stäbe, Knotenbleche, Schnürungen und Augen als Knoten; der Lagerbock trägt keinen Tragwerkskörper, der Binder steht auf Kontaktankern an den Schwellen.")],
    100: [T("Body at 1:100 (15 mm feature): members and plates as at 1:50; each bolt is one flat cylinder of 58 x 20 mm (16 mm on the head side) since the 6 mm washer step is below the feature; the heel bearing plates (10 mm) are left out; the eyes are 15 mm thick; the master link is a full oval plate of 16 mm without its opening; braces, wedges and stubs stay (24 mm and more).", "Körper bei 1:100 (15-mm-Merkmal): Stäbe und Bleche wie bei 1:50; jeder Bolzen ist ein flacher Zylinder 58 x 20 mm (16 mm auf der Kopfseite), da die 6-mm-Stufe der Scheibe unter dem Merkmal liegt; die Auflagerplatten (10 mm) entfallen; die Augen sind 15 mm dick; der Aufhängering ist eine volle ovale Scheibe von 16 mm ohne Öffnung; Streben, Keile und Stummel bleiben (24 mm und mehr)."),
          T("Thermal by reference as at 1:50. Structure: the members as at 1:50; each node is one joint, the near-face plate, instead of the pair of plates.", "Thermik per Verweis wie bei 1:50. Tragwerk: die Stäbe wie bei 1:50; jeder Knoten ist ein Knoten, das vordere Blech, statt des Blechpaars.")],
    200: [T("Body at 1:200 (30 mm feature): the seven members in true section, the plates as sheets with the outline of the drawing (the convex hull of the intermediate nodes), the chokes with their eye merged into a knot on top so that the chain rests on them, the two chain legs true and slender; bolts, wedges, stubs, eyes, master link and bearing plates are left out (all under 30 mm); trestles with sleeper, posts and braces.", "Körper bei 1:200 (30-mm-Merkmal): die sieben Stäbe in wahrem Querschnitt, die Bleche als Bleche mit der Kontur der Zeichnung (konvexe Hülle der Zwischenknoten), die Schnürungen mit dem Auge als Knoten obenauf vereint, sodass die Kette auf ihnen liegt, die zwei Kettenstränge wahr und schlank; Bolzen, Keile, Stummel, Augen, Aufhängering und Auflagerplatten entfallen (alle unter 30 mm); Böcke mit Schwelle, Pfosten und Streben."),
          T("Thermal and structure by reference; the chokes are the joints between chain and chord.", "Thermik und Tragwerk per Verweis; die Schnürungen sind die Knoten zwischen Kette und Gurt.")],
    500: [T("Body at 1:500 (75 mm feature): the seven members (80 mm thick, the smallest kept extent), the plates as sheets, the chokes widened to 75 mm along the chord with the merged eye knot, the chain legs true and slender, each trestle as sleeper and two posts; the 60 mm braces are left out.", "Körper bei 1:500 (75-mm-Merkmal): die sieben Stäbe (80 mm dick, die kleinste erhaltene Abmessung), die Bleche als Bleche, die Schnürungen längs des Gurts auf 75 mm verbreitert mit dem vereinten Augenknoten, die Kettenstränge wahr und schlank, jeder Bock als Schwelle und zwei Pfosten; die 60-mm-Streben entfallen."),
          T("Minimal: the structure cannot be simpler than at 1:200: seven timber members and two chain legs, one plate per node as its joint and the two chokes that join chain and chord. Thermal by reference.", "Minimal: das Tragwerk kann nicht einfacher sein als bei 1:200: sieben Holzstäbe und zwei Kettenstränge, ein Blech je Knoten als sein Knoten und die zwei Schnürungen, die Kette und Gurt verbinden. Thermik per Verweis.")],
}


def document():
    """📄️ The whole asset document."""
    reps, vols = {}, {}
    for s in SCALES:
        parts = parts_of(s)
        elev, front, plan = views(s, parts)
        reps[str(s)] = {"status": "authored", "denominator": s, "validRange": {"minDenominator": RANGES[s][0], "maxDenominator": RANGES[s][1]}, "pens": PENS[s], "hatches": [], "geometry": {"elevation": elev, "front": front, "plan": plan}, "decisions": rep_decisions(s)}
        vols[str(s)] = volume(s, parts, VOL_DECISIONS[s])
    top = max(c[1] for p in parts_of(50) for c in p.corners)
    asset = {
        "id": "reclaimed-timber-roof-truss",
        "category": "components",
        "title": T("Reclaimed Timber Roof Truss", "Gebrauchter Holz-Dachbinder"),
        "summary": T(f"A bolted softwood W-truss (Fink truss) of 9.00 m span and 2.20 m system rise, C24 chords 80 x 180 mm and webs 80 x 120 mm joined by 6 mm steel gusset plates with M16 bolts, taken out of a roof and stored upright in a timber stillage at a component exchange: bearing plates still under the heels, the empty holes of the removed bracing bolts, batten nail holes on the top chords and the sawn-off stubs of the wind braces show its first life. It is lifted by a two-leg chain sling choked at the top chord panel points and reinstalled on two wall plates under roof covering and snow.", f"Ein geschraubter Nadelholz-W-Binder (Fink-Binder) mit 9,00 m Spannweite und 2,20 m Systemhöhe, Gurte C24 80 x 180 mm und Diagonalen 80 x 120 mm, verbunden durch 6-mm-Stahlknotenbleche mit M16-Bolzen, aus einem Dach ausgebaut und bei einer Bauteilbörse aufrecht in einem Holzlagerbock gelagert: die Auflagerplatten noch unter den Traufknoten, die leeren Löcher der ausgebauten Verbandsbolzen, Lattennagellöcher auf den Obergurten und die abgesägten Stummel der Windrispen zeigen sein erstes Leben. Er wird an einem Zweistrang-Kettengehänge gehoben, das an den Obergurtknoten angeschnürt ist, und auf zwei Fußpfetten unter Dachdeckung und Schnee wieder eingebaut."),
        "tags": ["component", "reclaimed", "timber", "softwood", "steel", "truss", "roof-truss", "w-truss", "bolted", "gusset-plate", "stillage", "chain-sling", "structural"],
        "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFERENCES], "notes": T("System lines: span 9.00 m between the heel nodes, rise 2.20 m from the bottom chord axis to the ridge node, pitch 26.05 degrees, Fink web pattern with the bottom chord divided in thirds and the top chords in halves; bottom chord 9.40 m long. All dimensions, masses and loads are literature values or authored estimates, nothing is measured.", "Systemlinien: Spannweite 9,00 m zwischen den Traufknoten, Systemhöhe 2,20 m von der Untergurtachse zum Firstknoten, Neigung 26,05 Grad, Fink-Ausfachung mit dem Untergurt in Dritteln und den Obergurten in Hälften geteilt; Untergurt 9,40 m lang. Alle Maße, Massen und Lasten sind Literaturwerte oder Schätzungen, nichts ist gemessen.")},
        "dimensions": {"heightM": r(top, 3), "widthM": g.LENGTH, "depthM": g.SLEEPER[2], "massKg": r(TOTAL_KG, 2)},
        "materials": MATERIALS,
        "rig": {"kind": "structural", "root": "yard", "bones": BONES},
        "volumes": vols,
        "anchors": anchors(),
        "representations": reps,
        "symbols": [],
        "poses": POSES,
        "clips": CLIPS,
        "thermal": {"environment": {"airTempC": 15, "meanRadiantTempC": 15, "skyTempC": 2, "groundTempC": 14, "relativeHumidity": 0.6, "windMS": 2, "windDirectionDeg": 90, "solarWM2": 600, "diffuseFraction": 0.25, "pressurePa": 101325}, "passengers": [], "operating": [], "regions": REGIONS, "emitters": [],
                    "assumptions": [T("Every surface is passive: the truss, its fittings, the stillage and the sling are inert objects heated by sun and air; nothing draws power (total power 0 W).", "Jede Fläche ist passiv: Binder, Beschläge, Lagerbock und Gehänge sind träge Körper, die von Sonne und Luft erwärmt werden; nichts nimmt Leistung auf (Gesamtleistung 0 W)."),
                                    T("Weathered grey spruce absorbs more sun (0.70) than the fresh sawn stillage (0.55); the primed steel with rust patina 0.75 (ASHRAE Fundamentals, chapter 4, authored estimates).", "Verwitterte graue Fichte absorbiert mehr Sonne (0,70) als der frisch gesägte Lagerbock (0,55); der mennigegrundierte Stahl mit Rostpatina 0,75 (ASHRAE Fundamentals, Kapitel 4, Schätzungen)."),
                                    T("Reinstalled, the truss lies in the shade of the roof covering (sunlit fraction 0); in the yard half of its faces see the sun.", "Wieder eingebaut liegt der Binder im Schatten der Dachdeckung (besonnter Anteil 0); auf dem Lagerplatz sieht die Hälfte seiner Flächen die Sonne."),
                                    T("Reference case: a mild sunny day on the yard, 15 degC, 600 W/m2.", "Referenzfall: ein milder sonniger Tag auf dem Lagerplatz, 15 °C, 600 W/m2.")]},
        "structure": {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [],
                      "assumptions": [T("The supports change with the pose: two contacts on the sleepers when stored, two rollers along the chain legs at the chokes when lifted (the legs meet at the crane hook, so the reactions point along them), pin and roller under the heel bearing plates when reinstalled.", "Die Auflager wechseln mit der Pose: zwei Kontakte auf den Schwellen in der Lagerung, zwei Rollenlager entlang der Kettenstränge an den Schnürungen beim Heben (die Stränge treffen sich im Kranhaken, die Reaktionen zeigen daher entlang der Stränge), Fest- und Rollenlager unter den Auflagerplatten der Traufknoten im Wiedereinbau."),
                                      T("The rig is a tree: the top chords and webs are cantilevered from the bottom chord at their lower joints and the closed triangles of the truss are not solved as a lattice; reactions, the free body and the load path between the supports are exact, the member diagrams are the free-body cuts of the tree and not the axial forces of a pin-jointed truss.", "Das Rig ist ein Baum: Obergurte und Diagonalen hängen an ihren unteren Knoten am Untergurt, die geschlossenen Dreiecke des Binders werden nicht als Fachwerk gelöst; Reaktionen, freigeschnittener Körper und Lastpfad zwischen den Auflagern sind genau, die Stabverläufe sind die Freischnitte des Baums und nicht die Normalkräfte eines gelenkigen Fachwerks."),
                                      T(f"Masses from section times density: C24 timber 420 kg/m3, steel plates 7850 kg/m3, 0.42 kg per M16 bolt set, chain 1.4 kg/m; truss and sling {TOTAL_KG:.1f} kg. The two trestles of the stillage (about 48 kg of sawn spruce) are yard equipment that stays on the ground; they carry no mass in the analysis and are not part of dimensions.massKg.", f"Massen aus Querschnitt mal Dichte: Holz C24 420 kg/m3, Stahlbleche 7850 kg/m3, 0,42 kg je M16-Bolzensatz, Kette 1,4 kg/m; Binder und Gehänge {TOTAL_KG:.1f} kg. Die beiden Böcke des Lagerbocks (etwa 48 kg sägeraue Fichte) sind Lagerplatzausrüstung, die am Boden bleibt; sie tragen in der Berechnung keine Masse und gehören nicht zu dimensions.massKg."),
                                      T(f"Reinstalled at 1.0 m truss spacing: tile roof 0.55 kN/m2 of roof surface ({DEAD / 1000:.2f} kN/m along each top chord), snow zone 2 at 400 m s = 0.8 x 1.21 = 0.97 kN/m2 on plan ({SNOW / 1000:.3f} kN/m along the chord), drifted case with half the snow on the right slope.", f"Wieder eingebaut bei 1,0 m Binderabstand: Ziegeldach 0,55 kN/m2 Dachfläche ({DEAD / 1000:.2f} kN/m entlang jedes Obergurts), Schnee Zone 2 auf 400 m s = 0,8 x 1,21 = 0,97 kN/m2 Grundfläche ({SNOW / 1000:.3f} kN/m entlang des Gurts), verwehter Fall mit halbem Schnee auf der rechten Dachfläche.".replace(".", ",", 0))],
                      "clinical": False},
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 100},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    doc = document()
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
    print("[DEBUG] written", sum(len(v["body"]) for v in doc["asset"]["volumes"].values()), "solids, mass", round(TOTAL_KG, 2), "height", doc["asset"]["dimensions"]["heightM"])
