"""🧊 Round 3, appliances/mvhr-unit: every body, thermal and structural solid of 1:50, 1:100 and 1:200 stated by hand from the measures of `r3-mvhr-unit-geo.py`.

`volumes()` returns the `volumes` member (1:500 stays unavailable like its representation).
"""
import importlib.util
import math
import os

_spec = importlib.util.spec_from_file_location("geo", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-mvhr-unit-geo.py"))
g = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(g)

R5 = g.R5
WIN_ORIGIN = [0.3, 0.0, 0.0]
ZAX = [0, 0, 1]
UAX = [1, 0, 0]


def P(points):
    """📌 Rounded point list."""
    return [[R5(a), R5(b)] for a, b in points]


def cas(points):
    """🧭 World (X, Y) polygon to the plane xy profile of `casing`."""
    return [g.casing_uv(x, y) for x, y in points]


def loop(points):
    """➰ Closed revolve loop (first point repeated)."""
    pts = P(points)
    return pts + [pts[0]]


def S(id, kind, bone, material, tone, role="body", **kw):
    """🧱 One solid with the common fields first."""
    out = {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(kw)
    return out


def ext(id, bone, material, tone, profile, z0, z1, plane="xy", holes=None, role="body", **kw):
    """🧱 Extrude helper."""
    s = S(id, "extrude", bone, material, tone, role, profile=profile, plane=plane, **{"from": R5(z0), "to": R5(z1)})
    if holes:
        s["holes"] = holes
    s.update(kw)
    return s


def rev(id, bone, material, tone, profile, axis, origin, role="body", **kw):
    """🧱 Revolve helper."""
    s = S(id, "revolve", bone, material, tone, role, profile=P(profile) if profile[0] != profile[-1] else profile, axis=axis, origin=[R5(v) for v in origin])
    s.update(kw)
    return s


def ref(id, of, **kw):
    """🔗 Reference to a body solid of the same scale."""
    out = {"id": id, "of": of}
    out.update(kw)
    return out


def box(id, bone, material, tone, size, at, role="body", **kw):
    """📦 Box helper."""
    s = S(id, "box", bone, material, tone, role, size=[R5(v) for v in size], transform={"translation": [R5(v) for v in at]})
    s.update(kw)
    return s


def ball(id, bone, r, at, member):
    """⚫ Mass solid."""
    return {"id": id, "kind": "ellipsoid", "bone": bone, "material": "painted-steel", "role": "detail", "tone": 0.6, "radii": [r, r, r], "transform": {"translation": [R5(v) for v in at]}, "member": member, "part": "mass"}


RR_OUT = cas(g.rr(0.3, 0.3, g.EDGE_R, 0, 0.3))
RR_SHEET = cas(g.rr(0.299, 0.299, 0.024, 0, 0.3))
RR_LINER = cas(g.rr(0.280, 0.280, 0.005, 0, 0.3, n=4))
WIN_HOLE = lambda r: [cas(g.circle(r, 0, 0.3))[::-1]]
SPIG_A = [0.0, 0.225, 0.07]
SPIG_B = [0.0, 0.075, -0.07]
ROW = {"count": 2, "translation": [0, -0.3, 0]}
TUBE_LOOP_50 = loop([(-0.003, g.TUBE_RI), (g.TUBE_H, g.TUBE_RI), (g.TUBE_H, g.TUBE_RO), (-0.003, g.TUBE_RO)])
SEAL_LOOP = loop([(0.048, 0.049), (0.048, 0.0525), (0.0505, 0.057), (0.0525, 0.0545), (0.0555, 0.0545), (0.0575, 0.057), (0.0600, 0.0525), (0.062, 0.0505), (0.062, 0.049)])


def emitters(t0):
    """💨 The two air outlets as patches in the bore of their spigot tops (D2), from t0 to 3.5 mm above the rim."""
    prof = [(t0, 0), (t0, g.TUBE_RI), (0.0655, g.TUBE_RI), (0.0655, 0)]
    return [
        dict(rev("th-supply-air", "spigots", "galvanised-steel", 0.7, prof, UAX, [0, -0.075, 0.07], role="detail"), region="ducts", tags=["supply-air"]),
        dict(rev("th-extract-air", "spigots", "galvanised-steel", 0.7, prof, UAX, [0, 0.225, 0.07], role="detail"), region="ducts", tags=["extract-air"]),
    ]


def core(z0, z1, id="th-core"):
    """🔷 The counter-flow plate pack: a hexagonal prism, plates in the xy plane stacked along the depth."""
    return dict(ext(id, "casing", "polystyrene-plate", 0.72, cas(g.CORE_HEX), z0, z1), region="core", layer="plates")


def filter_box(id, x0, x1, y=g.FILTER_Y, z=g.FILTER_Z):
    """🧽 One panel filter cassette under its inlet spigot."""
    return dict(box(id, "casing", "filter-media", 0.9, [y[1] - y[0], x1 - x0, z[1] - z[0]], [(y[0] + y[1]) / 2, -(x0 + x1) / 2, (z[0] + z[1]) / 2]), region="filters", layer="media")


def body50():
    """🎨 1:50 body."""
    fil = [(-0.120, 0), (-0.142, 0), (-0.146, 0.00107), (-0.14813, 0.00286), (-0.14925, 0.00462), (-0.14988, 0.00661), (-0.150, 0.008)]
    return [
        ext("wrapper", "casing", "painted-steel", 0.82, RR_OUT, g.Z_COVER, g.Z_BACK),
        S("cover-rim", "loft", "casing", "painted-steel", 0.86, sections=[{"at": [0.3, 0, R5(z)], "rx": R5(0.3 - d), "ry": R5(0.3 - d), "exponent": 28} for z, d in fil], capStart=False, capEnd=False),
        ext("cover-face", "casing", "painted-steel", 0.86, cas(g.rr(0.292, 0.292, 0.0243, 0, 0.3)), -0.150, -0.144, holes=WIN_HOLE(g.BEZEL_R)),
        rev("window-ring", "casing", "painted-steel", 0.6, loop([(-0.144, 0.112), (-0.1515, 0.112), (-0.1525, 0.1135), (-0.1525, 0.1265), (-0.1515, 0.128), (-0.144, 0.128)]), ZAX, WIN_ORIGIN),
        rev("window", "casing", "polycarbonate", 0.3, [(-0.1515, 0), (-0.1515, g.GLASS_R), (-0.1495, g.GLASS_R), (-0.1495, 0)], ZAX, WIN_ORIGIN, role="glazing"),
        rev("fan-chamber", "casing", "epp-foam", 0.22, loop([(-0.144, 0.106), (-0.1215, 0.106), (-0.1215, 0.0), (-0.1195, 0.0), (-0.1195, 0.112), (-0.144, 0.112)]), ZAX, WIN_ORIGIN),
        rev("shroud", "impeller", "pp-impeller", 0.85, loop([(-0.0165, 0.062), (-0.0165, 0.0645), (-0.0110, 0.0665), (-0.0105, 0.095), (-0.0085, 0.095), (-0.0085, 0.062)]), ZAX, [0, 0, 0]),
        rev("back-plate", "impeller", "pp-impeller", 0.55, [(-0.004, 0), (-0.004, 0.035), (0.0045, 0.095), (0.0075, 0.095), (0.0075, 0)], ZAX, [0, 0, 0]),
        rev("hub", "impeller", "painted-steel", 0.35, [(-0.016, 0), (-0.016, 0.030), (-0.0135, 0.035), (-0.004, 0.035), (-0.004, 0)], ZAX, [0, 0, 0]),
        ext("blades", "impeller", "pp-impeller", 0.8, P(g.blade_profile()), -0.0085, 0.0045, texture=True, repeat={"count": 7, "rotationAxis": ZAX, "rotationRad": R5(2 * math.pi / 7), "origin": [0, 0, 0]}),
        rev("led", "casing", "polycarbonate", 0.95, [(-0.153, 0), (-0.153, 0.010), (-0.1515, 0.014), (-0.1495, 0.014), (-0.1495, 0)], ZAX, [g.LED[1], -g.LED[0], 0], role="detail", texture=True),
        S("handle", "loft", "casing", "painted-steel", 0.45, role="detail", sections=[{"at": [0.042, 0, R5(z)], "rx": rx, "ry": ry, "exponent": 4} for z, rx, ry in [(-0.144, 0.012, 0.080), (-0.1535, 0.012, 0.080), (-0.1555, 0.0105, 0.0785), (-0.1565, 0.007, 0.075)]], capStart=True, capEnd=True),
        rev("spigot-a", "spigots", "galvanised-steel", 0.66, TUBE_LOOP_50, UAX, SPIG_A, repeat=ROW),
        rev("spigot-b", "spigots", "galvanised-steel", 0.66, TUBE_LOOP_50, UAX, SPIG_B, repeat=ROW),
        rev("seal-a", "spigots", "epdm-seal", 0.3, SEAL_LOOP, UAX, SPIG_A, role="detail", repeat=ROW),
        rev("seal-b", "spigots", "epdm-seal", 0.3, SEAL_LOOP, UAX, SPIG_B, role="detail", repeat=ROW),
        ext("rail", "casing", "galvanised-steel", 0.6, P([(z, y) for y, z in g.RAIL]), -g.SPAN, g.SPAN, plane="zx"),
        ext("hook", "casing", "painted-steel", 0.5, P([(z, y) for y, z in g.HOOK]), -g.SPAN, g.SPAN, plane="zx"),
        dict(box("rest", "casing", "epdm-seal", 0.25, [0.03, 0.3, 0.015], [0.03, 0, 0.1425]), cornerRadius=0.003),
    ]


def wrapper_panels():
    """🧩 The 1 mm wrapper cut at the 45 degree points of its bent edges into top, right, bottom and left panels, each with the face looking into the casing as its section face (the product has no cavity flood yet, C31)."""
    outer = g.rr(0.3, 0.3, g.EDGE_R, 0, 0.3)
    inner = g.rr(0.299, 0.299, 0.024, 0, 0.3)
    corner = lambda pts, k, i0, i1: pts[k * 11 + i0 : k * 11 + i1 + 1]
    out = []
    for side, (ka, kb), face in [("top", (0, 1), "-u"), ("left", (1, 2), "-v"), ("bottom", (2, 3), "+u"), ("right", (3, 0), "+v")]:
        o = corner(outer, ka, 5, 10) + corner(outer, kb, 0, 5)
        i = corner(inner, ka, 5, 10) + corner(inner, kb, 0, 5)
        out.append((side, cas(o + i[::-1]), face))
    return out


def thermal50():
    """🌡️ 1:50 thermal: the casing as nested steel and EPP shells, the window, impeller, ducts with their duct-section faces, the hidden plate pack, extract-fan scroll and filter cassettes, the two outlets."""
    sheet = dict(region="casing", layer="sheet")
    liner = dict(region="casing", layer="liner")
    return [
        ref("th-sheet-rim", "cover-rim", **sheet),
        ref("th-sheet-front", "cover-face", sectionFaces=["+w"], **sheet),
    ] + [dict(ext(f"th-sheet-{side}", "casing", "painted-steel", 0.82, prof, g.Z_COVER, g.Z_BACK, sectionFaces=[face]), **sheet) for side, prof, face in wrapper_panels()] + [
        dict(ext("th-sheet-back", "casing", "painted-steel", 0.82, RR_SHEET, 0.134, g.Z_BACK, sectionFaces=["-w"]), **sheet),
        ref("th-bezel", "window-ring", sectionFaces=["+w"], **sheet),
        ref("th-handle", "handle", **sheet),
        ref("th-led", "led", **sheet),
        dict(ext("th-liner-wrap", "casing", "epp-foam", 0.3, RR_SHEET, -0.142, 0.134, holes=[RR_LINER[::-1]]), **liner),
        dict(ext("th-liner-front", "casing", "epp-foam", 0.3, cas(g.rr(0.291, 0.291, 0.016, 0, 0.3)), -0.149, -0.130, holes=WIN_HOLE(g.GLASS_R)), **liner),
        dict(ext("th-liner-back", "casing", "epp-foam", 0.3, RR_LINER, 0.115, 0.134), **liner),
        ref("th-fan-chamber", "fan-chamber", **liner),
        ref("th-window", "window", region="window", layer="pane", sectionFaces=["+w"]),
        ref("th-shroud", "shroud", region="impeller", layer="blades"),
        ref("th-back-plate", "back-plate", region="impeller", layer="blades"),
        ref("th-hub", "hub", region="impeller", layer="blades"),
        ref("th-blades", "blades", region="impeller", layer="blades"),
        dict(rev("th-duct-a", "spigots", "galvanised-steel", 0.7, [(0, 0), (0, g.TUBE_RO), (g.TUBE_H, g.TUBE_RO), (g.TUBE_H, 0)], UAX, SPIG_A, repeat=ROW, sectionFaces=["-u", "+u"]), region="ducts", layer="tube"),
        dict(rev("th-duct-b", "spigots", "galvanised-steel", 0.7, [(0, 0), (0, g.TUBE_RO), (g.TUBE_H, g.TUBE_RO), (g.TUBE_H, 0)], UAX, SPIG_B, repeat=ROW, sectionFaces=["-u", "+u"]), region="ducts", layer="tube"),
        ref("th-seal-a", "seal-a", region="ducts", layer="tube"),
        ref("th-seal-b", "seal-b", region="ducts", layer="tube"),
        core(*g.CORE_Z),
        dict(ext("th-extract-fan", "casing", "pp-impeller", 0.5, cas(g.volute()), *g.VOLUTE_Z), region="extract-fan", layer="scroll"),
        filter_box("th-extract-filter", *g.FILTERS[0][1:]),
        filter_box("th-outdoor-filter", *g.FILTERS[1][1:]),
    ] + emitters(0.0575)


def tray(id, z0, zb, member="casing"):
    """🪣 The back panel as a tray: the plate against the wall side with folded top and bottom flanges running forward to z0."""
    prof = P([(z0, 0.001), (zb + 0.001, 0.001), (zb + 0.001, 0.599), (z0, 0.599), (z0, 0.598), (zb, 0.598), (zb, 0.002), (z0, 0.002)])
    return dict(ext(id, "casing", "painted-steel", 0.88, prof, -0.299, 0.299, plane="zx"), member=member, part="member")


def collar(id, origin, h):
    """🔩 The collar where a spigot is pressed through the top panel: the fixed joint of the spigot member."""
    return dict(rev(id, "spigots", "galvanised-steel", 0.4, loop([(-h / 2, 0.049), (h / 2, 0.049), (h / 2, 0.053), (-h / 2, 0.053)]), UAX, origin, repeat=ROW), member="spigots", part="joint")


def structure50():
    """🏗️ 1:50 structure: the 1 mm wrapper and back tray, the hook strip on the rail, the four spigot tubes, the impeller disc on its shaft, collars, supports and masses."""
    return [
        dict(ext("member-wrapper", "casing", "painted-steel", 0.88, RR_OUT, g.Z_COVER, g.Z_BACK, holes=[RR_SHEET[::-1]]), member="casing", part="member"),
        tray("member-back", 0.118, 0.134),
        ref("member-hook", "hook", member="casing", part="member"),
        ref("member-spigot-a", "spigot-a", member="spigots", part="member"),
        ref("member-spigot-b", "spigot-b", member="spigots", part="member"),
        ref("member-impeller", "back-plate", member="impeller", part="member"),
        dict(rev("joint-impeller-shaft", "impeller", "painted-steel", 0.4, [(-0.010, 0), (-0.010, 0.006), (0.0125, 0.006), (0.0125, 0)], ZAX, [0, 0, 0], role="detail"), member="impeller", part="joint"),
        collar("joint-spigot-a", SPIG_A, 0.008),
        collar("joint-spigot-b", SPIG_B, 0.008),
        dict(ref("support-wall-hook", "rail"), member="casing", part="support", anchor="wall-hook"),
        dict(ref("support-wall-rest", "rest"), member="casing", part="support", anchor="wall-rest"),
        ball("mass-casing", "casing", 0.03, [0.3, 0, 0], "casing"),
        dict(ball("mass-spigots-a", "spigots", 0.008, [0.031, 0.225, 0.07], "spigots"), repeat=ROW),
        dict(ball("mass-spigots-b", "spigots", 0.008, [0.031, 0.075, -0.07], "spigots"), repeat=ROW),
        ball("mass-impeller", "impeller", 0.012, [0, 0, 0], "impeller"),
    ]


IMP100 = [(-0.016, 0), (-0.016, 0.035), (-0.004, 0.035), (-0.004, 0.062), (-0.0165, 0.062), (-0.0165, 0.095), (0.0075, 0.095), (0.0075, 0)]


def body100():
    """🎨 1:100 body."""
    return [
        ext("casing", "casing", "painted-steel", 0.82, RR_OUT, g.Z_COVER, g.Z_BACK),
        ext("cover", "casing", "painted-steel", 0.86, RR_OUT, g.Z_FRONT, g.Z_COVER, holes=WIN_HOLE(g.BEZEL_R)),
        rev("window", "casing", "polycarbonate", 0.3, [(-0.1515, 0), (-0.1515, g.BEZEL_R), (-0.1495, g.BEZEL_R), (-0.1495, 0)], ZAX, WIN_ORIGIN, role="glazing"),
        rev("fan-chamber", "casing", "epp-foam", 0.22, loop([(-0.1495, 0.113), (-0.1215, 0.113), (-0.1215, 0.0), (-0.1195, 0.0), (-0.1195, 0.128), (-0.1495, 0.128)]), ZAX, WIN_ORIGIN),
        rev("impeller", "impeller", "pp-impeller", 0.8, IMP100, ZAX, [0, 0, 0]),
        rev("spigot-a", "spigots", "galvanised-steel", 0.66, loop([(-0.003, g.TUBE_RI), (g.TUBE_H, g.TUBE_RI), (g.TUBE_H, 0.057), (0.047, 0.057), (0.047, g.TUBE_RO), (-0.003, g.TUBE_RO)]), UAX, SPIG_A, repeat=ROW),
        rev("spigot-b", "spigots", "galvanised-steel", 0.66, loop([(-0.003, g.TUBE_RI), (g.TUBE_H, g.TUBE_RI), (g.TUBE_H, 0.057), (0.047, 0.057), (0.047, g.TUBE_RO), (-0.003, g.TUBE_RO)]), UAX, SPIG_B, repeat=ROW),
        box("bracket", "casing", "galvanised-steel", 0.55, [0.08, 0.3, 0.015], [0.56, 0, 0.1425]),
        box("rest", "casing", "epdm-seal", 0.25, [0.03, 0.3, 0.015], [0.03, 0, 0.1425]),
    ]


def duct100(id, origin, top=True):
    """🧪 1:100 duct: closed spigot with its seal band; the floor and the top are duct sections (built into the casing, continued by the duct)."""
    prof = [(0, 0), (0, g.TUBE_RO), (0.047, g.TUBE_RO), (0.047, 0.057), (g.TUBE_H, 0.057), (g.TUBE_H, 0)]
    return dict(rev(id, "spigots", "galvanised-steel", 0.7, prof, UAX, origin, repeat=ROW, sectionFaces=["-u", "+u"]), region="ducts")


def thermal100():
    """🌡️ 1:100 thermal: casing and cover with the steel and EPP layers merged into the surface solids, window, impeller, ducts, hidden pack, scroll and filters."""
    return [
        ref("th-casing", "casing", region="casing"),
        ref("th-cover", "cover", region="casing"),
        ref("th-fan-chamber", "fan-chamber", region="casing"),
        dict(rev("th-window", "casing", "polycarbonate", 0.3, [(-0.1515, 0), (-0.1515, g.GLASS_R), (-0.1495, g.GLASS_R), (-0.1495, 0)], ZAX, WIN_ORIGIN, role="glazing", sectionFaces=["+w"]), region="window"),
        ref("th-impeller", "impeller", region="impeller"),
        duct100("th-duct-a", SPIG_A),
        duct100("th-duct-b", SPIG_B),
        dict(core(-0.09, 0.11), layer=None),
        dict(rev("th-extract-fan", "casing", "pp-impeller", 0.5, [(0, 0), (0, 0.07), (0.105, 0.07), (0.105, 0)], ZAX, [0.5, 0.165, 0]), region="extract-fan"),
        dict(filter_box("th-extract-filter", -0.17, 0.02), layer=None),
        dict(filter_box("th-outdoor-filter", 0.13, 0.275), layer=None),
    ] + emitters(0.0505)


def structure100():
    """🏗️ 1:100 structure."""
    return [
        dict(ext("member-wrapper", "casing", "painted-steel", 0.88, RR_OUT, g.Z_COVER, g.Z_BACK, holes=[RR_SHEET[::-1]]), member="casing", part="member"),
        tray("member-back", 0.118, 0.134),
        dict(rev("member-spigot-a", "spigots", "galvanised-steel", 0.66, loop([(0, g.TUBE_RI), (g.TUBE_H, g.TUBE_RI), (g.TUBE_H, g.TUBE_RO), (0, g.TUBE_RO)]), UAX, SPIG_A, repeat=ROW), member="spigots", part="member"),
        dict(rev("member-spigot-b", "spigots", "galvanised-steel", 0.66, loop([(0, g.TUBE_RI), (g.TUBE_H, g.TUBE_RI), (g.TUBE_H, g.TUBE_RO), (0, g.TUBE_RO)]), UAX, SPIG_B, repeat=ROW), member="spigots", part="member"),
        ref("member-impeller", "impeller", member="impeller", part="member"),
        dict(rev("joint-impeller-shaft", "impeller", "painted-steel", 0.4, [(-0.010, 0), (-0.010, 0.0075), (0.0125, 0.0075), (0.0125, 0)], ZAX, [0, 0, 0], role="detail"), member="impeller", part="joint"),
        collar("joint-spigot-a", SPIG_A, 0.015),
        collar("joint-spigot-b", SPIG_B, 0.015),
        dict(ref("support-wall-hook", "bracket"), member="casing", part="support", anchor="wall-hook"),
        dict(ref("support-wall-rest", "rest"), member="casing", part="support", anchor="wall-rest"),
        ball("mass-casing", "casing", 0.035, [0.3, 0, 0], "casing"),
        dict(ball("mass-spigots-a", "spigots", 0.01, [0.031, 0.225, 0.07], "spigots"), repeat=ROW),
        dict(ball("mass-spigots-b", "spigots", 0.01, [0.031, 0.075, -0.07], "spigots"), repeat=ROW),
        ball("mass-impeller", "impeller", 0.0125, [0, 0, 0], "impeller"),
    ]


R200 = 0.045


def body200():
    """🎨 1:200 body."""
    return [
        ext("casing", "casing", "painted-steel", 0.82, RR_OUT, g.Z_COVER, g.Z_WALL),
        ext("cover", "casing", "painted-steel", 0.86, RR_OUT, g.Z_FRONT, g.Z_COVER, holes=WIN_HOLE(g.BEZEL_R)),
        rev("window", "casing", "polycarbonate", 0.3, [(-0.1515, 0), (-0.1515, g.BEZEL_R), (-0.1495, g.BEZEL_R), (-0.1495, 0)], ZAX, WIN_ORIGIN, role="glazing"),
        rev("impeller", "impeller", "pp-impeller", 0.8, [(-0.0165, 0), (-0.0165, 0.095), (0.0135, 0.095), (0.0135, 0)], ZAX, [0, 0, 0]),
        rev("spigot-a", "spigots", "galvanised-steel", 0.66, loop([(-0.003, R200 - 0.0007), (g.TUBE_H, R200 - 0.0007), (g.TUBE_H, R200), (-0.003, R200)]), UAX, SPIG_A, repeat=ROW),
        rev("spigot-b", "spigots", "galvanised-steel", 0.66, loop([(-0.003, R200 - 0.0007), (g.TUBE_H, R200 - 0.0007), (g.TUBE_H, R200), (-0.003, R200)]), UAX, SPIG_B, repeat=ROW),
    ]


def thermal200():
    """🌡️ 1:200 thermal."""
    cyl = [(0, 0), (0, R200), (g.TUBE_H, R200), (g.TUBE_H, 0)]
    return [
        ref("th-casing", "casing", region="casing"),
        ref("th-cover", "cover", region="casing"),
        dict(rev("th-window", "casing", "polycarbonate", 0.3, [(-0.1515, 0), (-0.1515, g.GLASS_R), (-0.1495, g.GLASS_R), (-0.1495, 0)], ZAX, WIN_ORIGIN, role="glazing", sectionFaces=["+w"]), region="window"),
        ref("th-impeller", "impeller", region="impeller"),
        dict(rev("th-duct-a", "spigots", "galvanised-steel", 0.7, cyl, UAX, SPIG_A, repeat=ROW, sectionFaces=["-u", "+u"]), region="ducts"),
        dict(rev("th-duct-b", "spigots", "galvanised-steel", 0.7, cyl, UAX, SPIG_B, repeat=ROW, sectionFaces=["-u", "+u"]), region="ducts"),
        dict(core(-0.08, 0.11), layer=None),
        dict(box("th-extract-fan", "casing", "pp-impeller", 0.5, [0.144, 0.144, 0.105], [0.5, 0.165, 0.0525]), region="extract-fan"),
        dict(filter_box("th-filters", -0.17, 0.275), layer=None),
    ] + emitters(0.0355)


def structure200():
    """🏗️ 1:200 structure."""
    tube = loop([(0, R200 - 0.0007), (g.TUBE_H, R200 - 0.0007), (g.TUBE_H, R200), (0, R200)])
    return [
        dict(ext("member-wrapper", "casing", "painted-steel", 0.88, RR_OUT, g.Z_COVER, g.Z_WALL, holes=[RR_SHEET[::-1]]), member="casing", part="member"),
        tray("member-back", 0.119, 0.149),
        dict(rev("member-spigot-a", "spigots", "galvanised-steel", 0.66, tube, UAX, SPIG_A, repeat=ROW), member="spigots", part="member"),
        dict(rev("member-spigot-b", "spigots", "galvanised-steel", 0.66, tube, UAX, SPIG_B, repeat=ROW), member="spigots", part="member"),
        ref("member-impeller", "impeller", member="impeller", part="member"),
        dict(rev("joint-impeller-shaft", "impeller", "painted-steel", 0.4, [(-0.010, 0), (-0.010, 0.015), (0.020, 0.015), (0.020, 0)], ZAX, [0, 0, 0], role="detail"), member="impeller", part="joint"),
        collar("joint-spigot-a", SPIG_A, 0.03),
        collar("joint-spigot-b", SPIG_B, 0.03),
        dict(box("support-wall-hook", "casing", "galvanised-steel", 0.6, [0.03, 0.3, 0.03], [0.585, 0, 0.1345]), member="casing", part="support", anchor="wall-hook"),
        dict(box("support-wall-rest", "casing", "epdm-seal", 0.25, [0.03, 0.3, 0.03], [0.03, 0, 0.1345]), member="casing", part="support", anchor="wall-rest"),
        ball("mass-casing", "casing", 0.04, [0.3, 0, 0], "casing"),
    ]


def clean(solids):
    """🧹 Drops fields set to None (a coarser scale merges the layers)."""
    return [{k: v for k, v in s.items() if v is not None} for s in solids]


def T(en, de):
    """🌐 Bilingual text."""
    return {"en": en, "de": de}


DECISIONS = {
    50: [
        T("Body: a 1 mm powder-coated steel wrapper 0.6 x 0.6 m with 25 mm bent edges, 255 mm deep, a removable front cover 30 mm deep whose front edge is rounded by 8 mm (a superelliptic loft), its face with a 256 mm bezel and a 224 mm polycarbonate window; behind the glass a black EPP fan chamber holds the supply impeller of an EC radial fan of the R3G190 class: 190 mm shroud with a 124 mm inlet eye, conical back plate, seven backward-curved blades 2.5 mm thick (texture: only their leading edges show in the eye) and the external rotor of the motor in the hub. A 28 mm status light and a moulded pull grip on the cover; four galvanised DN100 spigots, 0.7 mm wall, each with a double-lip EPDM seal ring; on the back a folded hook strip that hangs over a galvanised wall rail, and a rubber rest strip at the lower edge, which keep the casing 15 mm off the wall.", "Körper: ein 1 mm starker pulverbeschichteter Stahlblechmantel 0,6 x 0,6 m mit 25 mm Biegekanten, 255 mm tief, ein abnehmbarer Frontdeckel von 30 mm Tiefe, dessen Vorderkante um 8 mm gerundet ist (superelliptischer Loft), in seiner Fläche ein Ring von 256 mm und ein Polycarbonatfenster von 224 mm; hinter dem Glas eine schwarze EPP-Ventilatorkammer mit dem Zuluftlaufrad eines EC-Radialventilators der Klasse R3G190: Deckscheibe 190 mm mit 124 mm Einlaufauge, konische Tragscheibe, sieben rückwärts gekrümmte Schaufeln von 2,5 mm (Textur: nur ihre Vorderkanten zeigen sich im Auge) und der Außenläufer des Motors in der Nabe. Eine Statusleuchte von 28 mm und eine angeformte Griffleiste am Deckel; vier verzinkte DN100-Stutzen mit 0,7 mm Wand, je mit einem EPDM-Doppellippenring; hinten eine gekantete Hakenleiste, die über eine verzinkte Wandschiene greift, und unten eine Gummi-Auflageleiste, die das Gehäuse 15 mm von der Wand halten."),
        T("Thermal: the casing as two nested shells, 1 mm steel outside (wrapper, filleted front edge as a tapered ring, cover face, back, bezel, grip and light) and the 19 mm EPP lining inside with the fan chamber; the glass, the impeller with its rotor, the four spigots closed at the casing and at the duct (both duct sections) with their seals; hidden inside the lining the counter-flow plate pack (hexagonal, 0.53 x 0.26 x 0.195 m), the scroll of the extract fan rising to the exhaust spigot and the two filter cassettes under the inlet spigots; the supply and exhaust air leave through patches in the bores of their spigots.", "Thermik: das Gehäuse als zwei verschachtelte Schalen, außen 1 mm Stahl (Mantel, gerundete Vorderkante als konischer Ring, Deckelfläche, Rückwand, Ring, Griff und Leuchte), innen die 19-mm-EPP-Auskleidung mit der Ventilatorkammer; das Glas, das Laufrad mit dem Läufer, die vier Stutzen am Gehäuse und am Kanal geschlossen (beides Kanalschnitte) mit ihren Dichtringen; verborgen in der Auskleidung das Gegenstrom-Plattenpaket (sechseckig, 0,53 x 0,26 x 0,195 m), die Spirale des Abluftventilators, die zum Fortluftstutzen steigt, und die zwei Filterkassetten unter den Ansaugstutzen; Zu- und Fortluft verlassen das Gerät durch Flächen in den Bohrungen ihrer Stutzen."),
        T("Structure: the load path of a hung sheet casing: the 1 mm wrapper and the back tray with folded flanges carry the weight to the hook strip, which hangs on the wall rail (pin) while the rubber strip presses on the wall at the lower edge (roller); the four spigot tubes stand in collars in the top panel, the impeller disc turns on its shaft; masses at the centres of the casing, the spigot group and the impeller.", "Tragwerk: der Lastpfad eines eingehängten Blechgehäuses: der 1-mm-Mantel und die Rückwandwanne mit gekanteten Flanschen tragen das Gewicht zur Hakenleiste, die an der Wandschiene hängt (Festlager), während die Gummileiste unten gegen die Wand drückt (Rollenlager); die vier Stutzenrohre stehen in Kragen im Deckblech, die Laufradscheibe dreht auf ihrer Welle; Massen in den Schwerpunkten von Gehäuse, Stutzengruppe und Laufrad."),
    ],
    100: [
        T("Body: the 8 mm cover fillet, the bezel, light, grip and the blades are below the 15 mm of 1:100: the cover is a square-edged 30 mm slab with the 256 mm window opening and one glass disc, the fan chamber a 15 mm cup, the impeller one revolved disc with the inlet eye and the hub, each spigot one tube with its seal as a 15 mm band, hook strip and rail one 15 mm bracket, and the rest strip.", "Körper: die 8-mm-Deckelrundung, Ring, Leuchte, Griff und Schaufeln liegen unter den 15 mm von 1:100: der Deckel ist eine scharfkantige 30-mm-Platte mit der 256-mm-Fensteröffnung und einer Glasscheibe, die Ventilatorkammer ein 15-mm-Topf, das Laufrad eine gedrehte Scheibe mit Einlaufauge und Nabe, jeder Stutzen ein Rohr mit seiner Dichtung als 15-mm-Band, Hakenleiste und Schiene eine 15-mm-Konsole, dazu die Auflageleiste."),
        T("Thermal: steel and EPP layers merged into the casing and cover surfaces; window, impeller and four closed ducts with their seal band; the hidden plate pack, the extract fan as its 140 mm drum and the two filter cassettes; the outlets as 15 mm patches.", "Thermik: Stahl- und EPP-Schicht in die Flächen von Gehäuse und Deckel zusammengeführt; Fenster, Laufrad und vier geschlossene Kanäle mit Dichtband; das verborgene Plattenpaket, der Abluftventilator als seine 140-mm-Trommel und die zwei Filterkassetten; die Auslässe als 15-mm-Flächen."),
        T("Structure: wrapper and back tray, the four spigot tubes in 15 mm collars, the impeller disc on a 15 mm shaft, bracket and rest strip as the two supports, three masses.", "Tragwerk: Mantel und Rückwandwanne, die vier Stutzenrohre in 15-mm-Kragen, die Laufradscheibe auf einer 15-mm-Welle, Konsole und Auflageleiste als die zwei Lager, drei Massen."),
    ],
    200: [
        T("Body: at 1:200 (30 mm) the casing is one rounded block reaching to the wall (bracket and rest strip merged), the cover a 30 mm slab with the window opening and one glass disc, the impeller a plain 30 mm disc, the spigots plain tubes without seals, generalised to 90 mm so that the gaps between them stay printable (0.26 mm instead of 0.21 mm at the 42 mm gap of the side view).", "Körper: bei 1:200 (30 mm) ist das Gehäuse ein gerundeter Block bis zur Wand (Konsole und Auflageleiste zusammengeführt), der Deckel eine 30-mm-Platte mit Fensteröffnung und einer Glasscheibe, das Laufrad eine glatte 30-mm-Scheibe, die Stutzen glatte Rohre ohne Dichtungen, auf 90 mm verallgemeinert, damit die Lücken zwischen ihnen druckbar bleiben (0,26 mm statt 0,21 mm an der 42-mm-Lücke der Vorderansicht)."),
        T("Thermal: casing and cover surfaces, window, impeller, four plain closed ducts; inside the plate pack, the extract fan as a 144 mm block and both filters as one slab; 30 mm outlet patches.", "Thermik: Gehäuse- und Deckelflächen, Fenster, Laufrad, vier glatte geschlossene Kanäle; innen das Plattenpaket, der Abluftventilator als 144-mm-Block und beide Filter als eine Platte; 30-mm-Auslassflächen."),
        T("Structure: wrapper and back tray to the wall, spigot tubes in 30 mm collars, impeller disc on a 30 mm shaft, the hook and the rest as 30 mm bars in the wall plane; only the casing mass, the spigot and impeller masses (7 and 6 percent) are merged into it.", "Tragwerk: Mantel und Rückwandwanne bis zur Wand, Stutzenrohre in 30-mm-Kragen, Laufradscheibe auf einer 30-mm-Welle, Haken und Auflage als 30-mm-Riegel in der Wandebene; nur die Gehäusemasse, die Massen von Stutzen und Laufrad (7 und 6 Prozent) sind in sie eingerechnet."),
    ],
}


def volumes():
    """📦 The `volumes` member."""
    built = {50: (body50, thermal50, structure50), 100: (body100, thermal100, structure100), 200: (body200, thermal200, structure200)}
    out = {}
    for d, (b, t, s) in built.items():
        out[str(d)] = {"status": "authored", "denominator": d, "body": clean(b()), "thermal": clean(t()), "structure": clean(s()), "decisions": DECISIONS[d]}
    out["500"] = {"status": "unavailable", "denominator": 500}
    return out
