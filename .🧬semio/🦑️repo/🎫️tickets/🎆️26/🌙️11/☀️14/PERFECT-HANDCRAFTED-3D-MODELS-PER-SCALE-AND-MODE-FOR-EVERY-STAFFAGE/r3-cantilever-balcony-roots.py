"""🌱 Round 3 cantilever balcony: the root values outside `volumes` (rig, masses, areas, references and the 2D drawings of every scale as true views of the handcrafted body).

Usage: python3 r3-cantilever-balcony-roots.py            writes the candidate document to 🗑️generated/r3-cantilever-balcony/🔣️.json
       python3 r3-cantilever-balcony-roots.py --apply    re-reads the catalog file and replaces only the changed members in place
"""
import importlib.util
import json
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-cantilever-balcony-kit.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)
shape, rect, poly, disc, text, top, r5 = K.shape, K.rect, K.poly, K.disc, K.text, K.top, K.r5

CATALOG = os.path.join(T, "..", "..", "..", "..", "..", "..", "..", "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "🧱️components", "cantilever-balcony", "🔣️.json")
MEMBERS = ["provenance", "dimensions", "rig", "representations", "thermal", "structure"]
SL, RA = "slab", "railing"
BASE = 0.1 + K.POST_V
HAND_LOW = BASE + K.HAND_U - K.HAND_R
HAND_HIGH = BASE + K.RAIL_H
STUB_TOP = K.HAND_U - K.HAND_R

SLAB_AREA, SLAB_CX, SLAB_CY = K.polygon(K.SLAB_PROFILE)
SLAB_KG = round(SLAB_AREA * 2500, 1)
RAILING_PARTS = [
    ("post 50 x 50 x 4, 5.45 kg/m", 5.45 * (K.POST_TOP - K.PLATE_U1), (K.POST_TOP + K.PLATE_U1) / 2),
    ("base plate 150 x 150 x 12", 0.15 * 0.15 * 0.012 * 7850, (K.PLATE_U0 + K.PLATE_U1) / 2),
    ("cap 50 x 50 x 8", 0.05 * 0.05 * 0.008 * 7850, K.POST_TOP + 0.004),
    ("bracket d 20", 3.14159 * 0.01**2 * (STUB_TOP - K.POST_TOP - K.CAP_T) * 7850, (STUB_TOP + K.POST_TOP + K.CAP_T) / 2),
    ("handrail 60.3 x 2.9, 4.11 kg/m", 4.11, K.HAND_U),
    ("two chords 40 x 10, 3.14 kg/m", 2 * 3.14 * 0.95, (K.CHORD_TOP + K.CHORD_LOW) / 2),
    ("eight balusters d 12, 0.888 kg/m", 8 * 0.888 * (K.CHORD_TOP - K.CHORD_LOW - 0.01), (K.CHORD_TOP + K.CHORD_LOW) / 2),
    ("four M10 anchors with nuts", 4 * 0.07, 0.0),
]
RAILING_KG = round(sum(p[1] for p in RAILING_PARTS), 2)
RAILING_COM = round(sum(p[1] * p[2] for p in RAILING_PARTS) / sum(p[1] for p in RAILING_PARTS), 3)
AREAS = {"slab-edge": 0.16, "railing-steel": 0.88}

REFERENCES = [
    {"id": "schoeck-isokorb-xt-k", "citation": "Schöck Bauteile GmbH, Technical Information Schöck Isokorb XT type K and type K-U (load-bearing thermal-break element for cantilevered balconies, connection to a slab or to a wall below): insulation body 120 mm, element heights 160 to 250 mm (180 to 250 mm with concrete cover CV50), tension bars in the top layer with concrete cover CV35 or CV50, HTE-Compact concrete pressure bearings at the bottom, diagonal shear bars; values taken as typical of the product family, the asset is not a specific module.", "url": "https://www.schoeck.com/en-gb/isokorb/xt-type-k"},
    {"id": "din-18065", "citation": "DIN 18065:2020 Stairs in buildings, terminology, measuring rules, main dimensions: clear openings in guards at most 120 mm, graspable handrail sections of 25 to 60 mm; guard height at least 0.90 m and at least 1.10 m above 12 m fall height after the German model building code (Musterbauordnung, section 38)."},
    {"id": "din-18531-5", "citation": "DIN 18531-5:2017 Waterproofing of roofs, balconies, loggias and arcades, part 5: balconies, loggias and arcades (surfaces are to drain with a fall, 1.5 % is the usual planning value)."},
]
NOTES = text(
    "Strip model: one metre of balcony width, cut at both strip ends and at the top and bottom of the wall. The wall stub shows the three layers of an externally insulated wall (reinforced concrete 180 mm, mineral wool 100 mm, render 20 mm); a load-bearing thermal-break module of 120 mm replaces the insulation over the slab depth and connects the slab to the wall with tension bars bent down into the wall, pressure bearings and shear bars. The slab is 200 mm at the wall and falls 1.5 % to the edge (177.5 mm) with chamfered arrises and a drip groove. The railing is 1.10 m high with one post per metre at the strip centre, top-mounted 120 mm inside the edge on a grouted 150 mm base plate with four M10 anchors (65 mm edge distance), a 60.3 mm handrail tube and a welded baluster infill with 95 mm clear spacing. The crowd point load rides a massless load-carrier bone.",
    "Streifenmodell: ein Meter Balkonbreite, an beiden Streifenenden sowie oben und unten an der Wand geschnitten. Das Wandstück zeigt die drei Schichten einer außengedämmten Wand (Stahlbeton 180 mm, Mineralwolle 100 mm, Putz 20 mm); ein tragendes Dämmelement von 120 mm ersetzt die Dämmung auf Plattenhöhe und schließt die Platte mit in die Wand abgebogenen Zugstäben, Drucklagern und Querkraftstäben an. Die Platte ist an der Wand 200 mm dick und fällt mit 1,5 % zur Kante (177,5 mm), mit gefasten Kanten und Tropfnut. Das Geländer ist 1,10 m hoch mit einem Pfosten je Meter in Streifenmitte, 120 mm hinter der Kante auf einer untergossenen Fußplatte von 150 mm mit vier Ankern M10 aufgesetzt (65 mm Randabstand), mit Handlaufrohr 60,3 mm und geschweißter Stabfüllung mit 95 mm lichtem Abstand. Die Einzellast läuft auf einem masselosen Lastträger-Knochen.",
)


def bars50():
    """🪡 The reinforcement of the 1:50 section as pen lines and cut bar dots."""
    out = [
        shape("tension-bar-hook", SL, "detail", 4, False, 0.1, poly([p for p in K.TENSION_PATH if p[0] <= -0.12] + [[-0.12, K.TENSION_V]], False), region="wall-face", pen=0.25),
        shape("break-bar-top", SL, "detail", 4, False, 0.1, poly([[-0.12, K.TENSION_V], [0, K.TENSION_V]], False), region="thermal-break", pen=0.25),
        shape("tension-bar-lap", SL, "detail", 4, False, 0.1, poly([[0, K.TENSION_V], [0.7, K.TENSION_V]], False), region="slab-top", pen=0.25),
        shape("rebar-top", SL, "detail", 4, False, 0.1, poly(K.TOP_BAR, False), region="slab-top", pen=0.25),
        shape("rebar-edge", SL, "detail", 4, False, 0.1, poly(K.EDGE_PATH, False), region="slab-edge", pen=0.16),
        shape("rebar-bottom", SL, "detail", 4, False, 0.1, poly(K.BOTTOM_BAR, False), region="slab-soffit", pen=0.16),
        shape("break-shear-bar", SL, "detail", 4, False, 0.1, poly(K.SHEAR_PATH, False), region="thermal-break", pen=0.16),
    ]
    for k, x in enumerate(K.CROSS_TOP_X):
        out.append(shape(f"rebar-cross-top-{k}", SL, "detail", 4.1, True, 0.1, disc(x, top(x) - K.CROSS_TOP_DROP, 0.004), region="slab-top", pen=0.1))
    for k, x in enumerate(K.CROSS_BOTTOM_X):
        out.append(shape(f"rebar-cross-bottom-{k}", SL, "detail", 4.1, True, 0.1, disc(x, K.CROSS_BOTTOM_V, 0.004), region="slab-soffit", pen=0.1))
    return out


def zigzag(x0, x1, a, b, n):
    """〰️ The insulation zigzag between x0 and x1, running from a to b in n strokes."""
    return [[x0 if k % 2 == 0 else x1, a + (b - a) * k / n] for k in range(n + 1)]


def wall_plan(merged):
    out = [shape("wall-core", SL, "body", 1, True, 0.6, rect(-0.3, -0.5, -0.12, 0.5), region="wall-face", material="concrete", hatch="concrete")]
    if not merged:
        for name, x in (("i", -0.27), ("o", -0.15)):
            for k in range(5):
                out.append(shape(f"wall-rebar-{name}{k}", SL, "detail", 1.1, True, 0.2, disc(x, -0.4 + 0.2 * k, 0.006), region="wall-face", pen=0.1))
        out.append(shape("wall-insulation", SL, "body", 1, True, 0.9, rect(-0.12, -0.5, -0.02, 0.5), region="wall-face", material="mineral-wool"))
        out.append(shape("wall-insulation-zigzag", SL, "texture", 1.2, False, 0.3, poly(zigzag(-0.11, -0.03, -0.5, 0.5, 24), False), region="wall-face"))
        out.append(shape("wall-render", SL, "body", 1, True, 0.55, rect(-0.02, -0.5, 0, 0.5), region="wall-face", material="render"))
    else:
        out.append(shape("wall-insulation-render", SL, "body", 1, True, 0.9, rect(-0.12, -0.5, 0, 0.5), region="wall-face", material="mineral-wool"))
        out.append(shape("wall-insulation-zigzag", SL, "texture", 1.2, False, 0.3, poly(zigzag(-0.11, -0.01, -0.5, 0.5, 20), False), region="wall-face"))
    return out


def drawing50():
    elevation = [
        shape("wall-core", SL, "body", 1, True, 0.6, rect(-0.3, -0.1, -0.12, 1.2), region="wall-face", material="concrete", hatch="concrete"),
        shape("wall-rebar-inner", SL, "detail", 1.1, False, 0.2, poly([[-0.27, 0.15], [-0.27, 1.15]], False), region="wall-face", pen=0.2),
        shape("wall-rebar-outer", SL, "detail", 1.1, False, 0.2, poly([[-0.15, 0.15], [-0.15, 1.15]], False), region="wall-face", pen=0.2),
        shape("wall-insulation", SL, "body", 1, True, 0.9, rect(-0.12, 0.1, -0.02, 1.2), region="wall-face", material="mineral-wool"),
        shape("wall-insulation-zigzag", SL, "texture", 1.2, False, 0.3, poly(zigzag(-0.11, -0.03, 0.1, 1.2, 28), False), region="wall-face"),
        shape("wall-render", SL, "body", 1, True, 0.55, rect(-0.02, 0.1, 0, 1.2), region="wall-face", material="render"),
        shape("thermal-break-block", SL, "body", 2, True, 0.3, rect(-0.12, -0.1, 0, 0.1), region="thermal-break", material="break-module", hatch="break"),
        shape("slab", SL, "body", 3, True, 0.72, poly(K.SLAB_PROFILE), regions=["slab-top", "slab-soffit", "slab-edge"], material="concrete", hatch="concrete"),
        shape("break-bearing", SL, "detail", 3.5, True, 0.15, rect(-0.13, -0.08, 0.01, -0.05), region="thermal-break", material="concrete", pen=0.13),
    ]
    elevation += bars50()
    elevation += [
        shape("railing-grout-pad", SL, "detail", 4.5, True, 0.6, poly(K.PAD_PROFILE), region="slab-top", material="concrete"),
        shape("railing-base-plate", RA, "body", 5, True, 0.3, rect(K.PLATE_U0, -0.075, K.PLATE_U1, 0.075), region="railing-steel", material="steel", pen=0.18),
    ]
    for name, sign in (("a", 1), ("b", -1)):
        elevation.append(shape(f"railing-anchor-nut-{name}", RA, "detail", 5.3, True, 0.1, rect(K.PLATE_U1, sign * K.ANCHOR - 0.0085, K.PLATE_U1 + 0.01, sign * K.ANCHOR + 0.0085), region="railing-steel", material="steel"))
        elevation.append(shape(f"railing-anchor-stud-{name}", RA, "detail", 5.3, True, 0.1, rect(K.PLATE_U1 + 0.01, sign * K.ANCHOR - 0.005, K.PLATE_U1 + 0.018, sign * K.ANCHOR + 0.005), region="railing-steel", material="steel"))
    elevation += [
        shape("railing-post", RA, "body", 5, True, 0.45, rect(K.PLATE_U1, -0.025, K.POST_TOP, 0.025), region="railing-steel", material="steel"),
        shape("railing-post-cap", RA, "body", 5, True, 0.35, rect(K.POST_TOP, -0.025, K.POST_TOP + K.CAP_T, 0.025), region="railing-steel", material="steel", pen=0.18),
        shape("railing-handrail-stub", RA, "body", 5, True, 0.35, rect(K.POST_TOP + K.CAP_T, -0.01, STUB_TOP + 0.002, 0.01), region="railing-steel", material="steel", pen=0.18),
        shape("railing-handrail", RA, "body", 5.1, True, 0.35, disc(K.HAND_U, 0, K.HAND_R), region="railing-steel", material="steel"),
        shape("railing-balusters", RA, "detail", 5.2, True, 0.3, rect(K.CHORD_LOW + 0.005, -K.BALUSTER_R, K.CHORD_TOP - 0.005, K.BALUSTER_R), region="railing-steel", material="steel"),
        shape("railing-top-chord", RA, "detail", 5.3, True, 0.2, rect(K.CHORD_TOP - 0.005, -0.02, K.CHORD_TOP + 0.005, 0.02), region="railing-steel", material="steel"),
        shape("railing-bottom-chord", RA, "detail", 5.3, True, 0.2, rect(K.CHORD_LOW - 0.005, -0.02, K.CHORD_LOW + 0.005, 0.02), region="railing-steel", material="steel"),
    ]
    edge = 0.1 + top(1.5)
    front = [
        shape("wall", SL, "body", 1, True, 0.55, rect(-0.5, 0, 0.5, 1.3), region="wall-face", material="render"),
        shape("slab-top-fall", SL, "body", 2.9, True, 0.8, rect(-0.5, edge, 0.5, 0.2), regions=["slab-top"], material="concrete", pen=0.18),
        shape("slab", SL, "body", 3, True, 0.72, rect(-0.5, 0, 0.5, edge), regions=["slab-edge"], material="concrete"),
        shape("slab-arris", SL, "texture", 3.5, False, 0.3, poly([[-0.5, edge - 0.01], [0.5, edge - 0.01]], False), region="slab-edge"),
        shape("slab-arris-low", SL, "texture", 3.5, False, 0.3, poly([[-0.5, 0.01], [0.5, 0.01]], False), region="slab-edge"),
        shape("railing-grout-pad", SL, "detail", 4.5, True, 0.6, rect(-0.085, 0.1 + top(1.465), 0.085, 0.1 + K.PAD_TOP), region="slab-top", material="concrete"),
        shape("railing-base-plate", RA, "body", 5, True, 0.3, rect(-0.075, BASE + K.PLATE_U0, 0.075, BASE + K.PLATE_U1), region="railing-steel", material="steel", pen=0.18),
    ]
    for name, sign in (("l", 1), ("r", -1)):
        front.append(shape(f"railing-anchor-nut-{name}", RA, "detail", 5.3, True, 0.1, rect(sign * K.ANCHOR - 0.0098, BASE + K.PLATE_U1, sign * K.ANCHOR + 0.0098, BASE + K.PLATE_U1 + 0.01), region="railing-steel", material="steel"))
        front.append(shape(f"railing-anchor-stud-{name}", RA, "detail", 5.3, True, 0.1, rect(sign * K.ANCHOR - 0.005, BASE + K.PLATE_U1 + 0.01, sign * K.ANCHOR + 0.005, BASE + K.PLATE_U1 + 0.018), region="railing-steel", material="steel"))
    front += [
        shape("railing-post", RA, "body", 5, True, 0.45, rect(-0.025, BASE + K.PLATE_U1, 0.025, BASE + K.POST_TOP), region="railing-steel", material="steel"),
        shape("railing-post-cap", RA, "body", 5, True, 0.35, rect(-0.025, BASE + K.POST_TOP, 0.025, BASE + K.POST_TOP + K.CAP_T), region="railing-steel", material="steel", pen=0.18),
        shape("railing-handrail-stub", RA, "body", 5, True, 0.35, rect(-0.01, BASE + K.POST_TOP + K.CAP_T, 0.01, HAND_LOW + 0.002), region="railing-steel", material="steel", pen=0.18),
    ]
    for chord, u in (("top", K.CHORD_TOP), ("bottom", K.CHORD_LOW)):
        for name, sign in (("l", 1), ("r", -1)):
            front.append(shape(f"railing-{chord}-chord-{name}", RA, "detail", 5.3, True, 0.2, rect(sign * 0.025, BASE + u - 0.005, sign * 0.5, BASE + u + 0.005), region="railing-steel", material="steel"))
    for k, w in enumerate([s * b for s in (1, -1) for b in K.BALUSTER_W]):
        front.append(shape(f"railing-baluster-{k}", RA, "detail", 5.2, True, 0.3, rect(w - K.BALUSTER_R, BASE + K.CHORD_LOW + 0.005, w + K.BALUSTER_R, BASE + K.CHORD_TOP - 0.005), region="railing-steel", material="steel"))
    front.append(shape("railing-handrail", RA, "body", 5.1, True, 0.35, rect(-0.5, HAND_LOW, 0.5, HAND_HIGH), region="railing-steel", material="steel"))
    plan = wall_plan(False) + [
        shape("slab", SL, "body", 3, True, 0.72, rect(0, -0.5, 1.5, 0.5), regions=["slab-top"], material="concrete"),
        shape("slab-arris", SL, "texture", 3.5, False, 0.3, poly([[1.49, -0.5], [1.49, 0.5]], False), region="slab-top"),
        shape("railing-grout-pad", SL, "detail", 4.5, True, 0.6, rect(1.295, -0.085, 1.465, 0.085), region="slab-top", material="concrete"),
        shape("railing-base-plate", RA, "body", 5, True, 0.3, rect(K.POST_X - 0.075, -0.075, K.POST_X + 0.075, 0.075), region="railing-steel", material="steel", pen=0.18),
    ]
    for kx, dx in enumerate((-K.ANCHOR, K.ANCHOR)):
        for ky, dy in enumerate((-K.ANCHOR, K.ANCHOR)):
            plan.append(shape(f"railing-anchor-nut-{kx}{ky}", RA, "detail", 5.3, True, 0.1, disc(K.POST_X + dx, dy, 0.0098), region="railing-steel", material="steel"))
    plan.append(shape("railing-handrail", RA, "body", 5.4, True, 0.35, rect(K.POST_X - K.HAND_R, -0.5, K.POST_X + K.HAND_R, 0.5), region="railing-steel", material="steel"))
    return {"elevation": elevation, "front": front, "plan": plan}


def drawing100():
    foot_u = K.PLATE_U1
    elevation = [
        shape("wall-core", SL, "body", 1, True, 0.6, rect(-0.3, -0.1, -0.12, 1.2), region="wall-face", material="concrete", hatch="concrete"),
        shape("wall-insulation-render", SL, "body", 1, True, 0.9, rect(-0.12, 0.1, 0, 1.2), region="wall-face", material="mineral-wool"),
        shape("wall-insulation-zigzag", SL, "texture", 1.2, False, 0.3, poly(zigzag(-0.11, -0.01, 0.1, 1.2, 22), False), region="wall-face"),
        shape("thermal-break-block", SL, "body", 2, True, 0.3, rect(-0.12, -0.1, 0, 0.1), region="thermal-break", material="break-module", hatch="break"),
        shape("slab", SL, "body", 3, True, 0.72, poly(K.SLAB_WEDGE), regions=["slab-top", "slab-soffit", "slab-edge"], material="concrete", hatch="concrete"),
        shape("railing-foot", RA, "body", 5, True, 0.3, rect(-0.002, -0.075, foot_u, 0.075), region="railing-steel", material="steel", pen=0.13),
        shape("railing-post", RA, "body", 5, True, 0.45, rect(foot_u, -0.025, STUB_TOP + 0.002, 0.025), region="railing-steel", material="steel"),
        shape("railing-handrail", RA, "body", 5.1, True, 0.35, disc(K.HAND_U, 0, K.HAND_R), region="railing-steel", material="steel"),
    ]
    front = [
        shape("wall", SL, "body", 1, True, 0.55, rect(-0.5, 0, 0.5, 1.3), region="wall-face", material="render"),
        shape("slab", SL, "body", 3, True, 0.72, rect(-0.5, 0, 0.5, 0.2), regions=["slab-edge"], material="concrete"),
        shape("railing-foot", RA, "body", 5, True, 0.3, rect(-0.075, BASE - 0.002, 0.075, BASE + foot_u), region="railing-steel", material="steel", pen=0.13),
        shape("railing-post", RA, "body", 5, True, 0.45, rect(-0.025, BASE + foot_u, 0.025, HAND_LOW + 0.002), region="railing-steel", material="steel"),
        shape("railing-top-chord", RA, "detail", 5.3, False, 0.2, poly([[-0.5, BASE + K.CHORD_TOP], [0.5, BASE + K.CHORD_TOP]], False), region="railing-steel", material="steel", pen=0.1),
        shape("railing-bottom-chord", RA, "detail", 5.3, False, 0.2, poly([[-0.5, BASE + K.CHORD_LOW], [0.5, BASE + K.CHORD_LOW]], False), region="railing-steel", material="steel", pen=0.1),
    ]
    for k, w in enumerate([s * b for s in (1, -1) for b in K.BALUSTER_W]):
        front.append(shape(f"railing-baluster-{k}", RA, "detail", 5.2, False, 0.3, poly([[w, BASE + K.CHORD_LOW], [w, BASE + K.CHORD_TOP]], False), region="railing-steel", material="steel", pen=0.12))
    front.append(shape("railing-handrail", RA, "body", 5.1, True, 0.35, rect(-0.5, HAND_LOW, 0.5, HAND_HIGH), region="railing-steel", material="steel"))
    plan = wall_plan(True) + [
        shape("slab", SL, "body", 3, True, 0.72, rect(0, -0.5, 1.5, 0.5), regions=["slab-top"], material="concrete"),
        shape("railing-foot", RA, "body", 5, True, 0.3, rect(K.POST_X - 0.075, -0.075, K.POST_X + 0.075, 0.075), region="railing-steel", material="steel", pen=0.13),
        shape("railing-handrail", RA, "body", 5.1, True, 0.35, rect(K.POST_X - K.HAND_R, -0.5, K.POST_X + K.HAND_R, 0.5), region="railing-steel", material="steel"),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def drawing200():
    wall = [[-0.3, -0.1], [-0.12, -0.1], [-0.12, 0.1], [0, 0.1], [0, 1.2], [-0.3, 1.2]]
    elevation = [
        shape("wall", SL, "body", 1, True, 0.6, poly(wall), region="wall-face", material="concrete"),
        shape("wall-core-edge", SL, "detail", 1.5, False, 0.3, poly([[-0.12, 0.1], [-0.12, 1.2]], False)),
        shape("thermal-break-block", SL, "body", 2, True, 0.3, rect(-0.12, -0.1, 0, 0.1), region="thermal-break", material="break-module"),
        shape("slab", SL, "body", 3, True, 0.72, poly(K.SLAB_WEDGE), regions=["slab-top", "slab-soffit", "slab-edge"], material="concrete"),
        shape("railing-post", RA, "body", 5, True, 0.45, rect(-0.002, -0.025, K.HAND_U, 0.025), region="railing-steel", material="steel"),
        shape("railing-handrail", RA, "body", 5.1, True, 0.35, disc(K.HAND_U, 0, K.HAND_R), region="railing-steel", material="steel"),
    ]
    front = [
        shape("wall", SL, "body", 1, True, 0.6, rect(-0.5, 0, 0.5, 1.3), region="wall-face", material="render"),
        shape("slab", SL, "body", 3, True, 0.72, rect(-0.5, 0, 0.5, 0.2), regions=["slab-edge"], material="concrete"),
        shape("railing-infill", RA, "detail", 4.9, True, 0.8, rect(-0.5, BASE + K.CHORD_LOW - 0.005, 0.5, BASE + K.CHORD_TOP + 0.005), region="railing-steel", material="steel"),
        shape("railing-post", RA, "body", 5, True, 0.45, rect(-0.025, BASE - 0.002, 0.025, HAND_LOW + 0.002), region="railing-steel", material="steel"),
        shape("railing-handrail", RA, "body", 5.1, True, 0.35, rect(-0.5, HAND_LOW, 0.5, HAND_HIGH), region="railing-steel", material="steel"),
    ]
    plan = [
        shape("wall", SL, "body", 1, True, 0.6, rect(-0.3, -0.5, 0, 0.5), region="wall-face", material="concrete"),
        shape("wall-core-edge", SL, "detail", 1.5, False, 0.3, poly([[-0.12, -0.5], [-0.12, 0.5]], False)),
        shape("slab", SL, "body", 3, True, 0.72, rect(0, -0.5, 1.5, 0.5), regions=["slab-top"], material="concrete"),
        shape("railing-handrail", RA, "body", 5.1, True, 0.35, rect(K.POST_X - K.HAND_R, -0.5, K.POST_X + K.HAND_R, 0.5), region="railing-steel", material="steel"),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def drawing500():
    mass = [[-0.3, -0.1], [1.5, -0.1], [1.5, 0.1], [0, 0.1], [0, 1.2], [-0.3, 1.2]]
    rail_y = HAND_HIGH - 0.0375
    elevation = [
        shape("balcony-solid", SL, "body", 1, True, 0.25, poly(mass), regions=["wall-face", "thermal-break", "slab-top", "slab-soffit", "slab-edge"]),
        shape("railing-line", RA, "body", 2, False, 0.45, poly([[0, 0], [K.RAIL_H - 0.0375, 0]], False), region="railing-steel", pen=0.15),
    ]
    front = [
        shape("balcony-solid", SL, "body", 1, True, 0.25, rect(-0.5, 0, 0.5, 1.3), regions=["wall-face", "slab-edge"]),
        shape("railing-post-line", RA, "body", 2, False, 0.45, poly([[0, BASE], [0, rail_y]], False), region="railing-steel", pen=0.15),
        shape("railing-line", RA, "body", 2, False, 0.45, poly([[-0.4625, rail_y], [0.4625, rail_y]], False), region="railing-steel", pen=0.15),
    ]
    plan = [
        shape("balcony-solid", SL, "body", 1, True, 0.25, rect(-0.3, -0.5, 1.5, 0.5), regions=["wall-face", "slab-top"]),
        shape("railing-line", RA, "body", 2, False, 0.45, poly([[K.POST_X, -0.4625], [K.POST_X, 0.4625]], False), region="railing-steel", pen=0.15),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


TEXTS = {
    "50": {
        "silhouette": text(
            "True views of the 1:50 body. Section: wall layers, the break module, the slab with its 1.5 % fall, chamfers and drip groove, and the railing 120 mm inside the edge on its grout bed and base plate with post, cap, bracket and handrail tube; the nearest baluster and the cut chords stand in front of the post. Front view from the free edge: slab edge with both chamfer lines and the falling top behind it, the single post on its plate with two nuts, eight balusters between two chords and the handrail across the strip. Plan: the wall top cut through its layers, the slab top with the chamfer line, grout bed, base plate with four nuts and the handrail over post and infill.",
            "Wahre Ansichten des Körpers 1:50. Schnitt: Wandschichten, Dämmelement, die Platte mit 1,5 % Gefälle, Fasen und Tropfnut und das Geländer 120 mm hinter der Kante auf Mörtelbett und Fußplatte mit Pfosten, Deckel, Träger und Handlaufrohr; der vorderste Füllstab und die geschnittenen Gurte stehen vor dem Pfosten. Vorderansicht von der freien Kante: Plattenstirn mit beiden Fasenlinien und der fallenden Oberseite dahinter, der eine Pfosten auf seiner Platte mit zwei Muttern, acht Füllstäbe zwischen zwei Gurten und der Handlauf über den Streifen. Draufsicht: der Wandkopf durch seine Schichten geschnitten, die Plattenoberseite mit Fasenlinie, Mörtelbett, Fußplatte mit vier Muttern und der Handlauf über Pfosten und Füllung.",
        ),
        "retained": text(
            "Concrete core with two vertical bars, 100 mm mineral wool, 20 mm render, break module with tension bar, pressure bearing and diagonal shear bar, slab with top bars along the fall, the lapped tension bar with its hook into the wall, bottom bars, edge hairpin and cut cross bars, drip groove, grout bed, plate, anchors, post, cap, bracket, handrail, chords, balusters.",
            "Betonkern mit zwei senkrechten Stäben, 100 mm Mineralwolle, 20 mm Putz, Dämmelement mit Zugstab, Drucklager und schrägem Querkraftstab, Platte mit oberen Stäben im Gefälle, dem übergreifenden Zugstab mit Haken in die Wand, unteren Stäben, Steckbügel und geschnittenen Querstäben, Tropfnut, Mörtelbett, Platte, Anker, Pfosten, Deckel, Träger, Handlauf, Gurte, Füllstäbe.",
        ),
        "merged": text(
            "Slab top, soffit and edge share one slab shape in the section (the regions are bound through the bar details); the eight balusters stand behind each other in the section and are one strip.",
            "Plattenoberseite, -unterseite und -stirn teilen sich im Schnitt eine Plattenform (die Regionen sind über die Stabdetails gebunden); die acht Füllstäbe stehen im Schnitt hintereinander und sind ein Streifen.",
        ),
        "omitted": text(
            "Weld seams, washers, the corner radii of the post and the tube walls (4 and 2.9 mm, below the pen). The thermal break and the reinforcement lie behind the slab edge and under the wall top and show only in the section; post and infill lie under the handrail in plan.",
            "Schweißnähte, Scheiben, die Eckradien des Pfostens und die Rohrwandungen (4 und 2,9 mm, unter der Feder). Dämmelement und Bewehrung liegen hinter der Plattenstirn und unter dem Wandkopf und zeigen sich nur im Schnitt; Pfosten und Füllung liegen in der Draufsicht unter dem Handlauf.",
        ),
        "thinElements": text(
            "12 mm bars are pen lines of 0.25 mm and 8 mm bars of 0.16 mm (exact); the 12 mm base plate (0.24 mm), the 10 mm chords (0.2 mm) and the 12 mm balusters (0.24 mm) are drawn at true size.",
            "Stäbe von 12 mm sind Federlinien von 0,25 mm und Stäbe von 8 mm von 0,16 mm (exakt); die Fußplatte von 12 mm (0,24 mm), die Gurte von 10 mm (0,2 mm) und die Füllstäbe von 12 mm (0,24 mm) sind in wahrer Größe gezeichnet.",
        ),
    },
    "100": {
        "silhouette": text(
            "True views of the 1:100 body. Section: wall core, merged insulation and render, break module, the slab wedge with its fall, the railing foot, the post and the handrail circle. Front view: render face, slab edge, one post on its foot, the two chords and eight balusters as pen lines and the handrail. Plan: the wall top cut in its layers, the slab top, the foot and the handrail strip.",
            "Wahre Ansichten des Körpers 1:100. Schnitt: Wandkern, zusammengefasste Dämmung mit Putz, Dämmelement, der Plattenkeil mit Gefälle, der Geländerfuß, der Pfosten und der Handlaufkreis. Vorderansicht: Putzfläche, Plattenstirn, ein Pfosten auf seinem Fuß, die zwei Gurte und acht Füllstäbe als Federlinien und der Handlauf. Draufsicht: der Wandkopf in seinen Schichten geschnitten, die Plattenoberseite, der Fuß und der Handlaufstreifen.",
        ),
        "retained": text(
            "Concrete core (1.8 mm), insulation (1.2 mm) with zigzag line, thermal-break block (1.2 x 2 mm), slab 15 x 2 mm with 0.2 mm fall, foot, post, handrail circle, chords and balusters.",
            "Betonkern (1,8 mm), Dämmung (1,2 mm) mit Zickzacklinie, Dämmelement (1,2 x 2 mm), Platte 15 x 2 mm mit 0,2 mm Gefälle, Fuß, Pfosten, Handlaufkreis, Gurte und Füllstäbe.",
        ),
        "merged": text(
            "The 20 mm render is merged into the insulation shape; slab top, soffit and edge are one slab shape; grout bed and base plate are one foot; cap and bracket are part of the post.",
            "Der Putz von 20 mm ist in der Dämmungsform zusammengefasst; Plattenoberseite, -unterseite und -stirn bilden eine Plattenform; Mörtelbett und Fußplatte sind ein Fuß; Deckel und Träger gehören zum Pfosten.",
        ),
        "omitted": text(
            "Reinforcement, chamfers, drip groove and anchors. The thermal break is hidden behind the slab edge and below the wall top; chords and balusters stand in front of the post in the section and are not drawn there. No hatch on the uncut slab edge and slab top.",
            "Bewehrung, Fasen, Tropfnut und Anker. Das Dämmelement liegt verdeckt hinter der Plattenstirn und unter dem Wandkopf; Gurte und Füllstäbe stehen im Schnitt vor dem Pfosten und werden dort nicht gezeichnet. Keine Schraffur auf der ungeschnittenen Plattenstirn und -oberseite.",
        ),
        "thinElements": text(
            "The 50 mm post is a 0.5 mm filled bar and the handrail tube a 0.6 mm circle (exact); the 12 mm balusters are pen lines of 0.12 mm and the 10 mm chords of 0.1 mm (exact).",
            "Der Pfosten von 50 mm ist ein gefüllter Balken von 0,5 mm und das Handlaufrohr ein Kreis von 0,6 mm (exakt); die Füllstäbe von 12 mm sind Federlinien von 0,12 mm und die Gurte von 10 mm von 0,1 mm (exakt).",
        ),
    },
    "200": {
        "silhouette": text(
            "Wall, break element and slab are three separate masses; the L shape stays readable and the slab keeps its fall. The railing is one post with the handrail circle on top. Front view: wall face, slab edge (5 x 1 mm), the post, the baluster panel as one light tone and the handrail strip. Plan: wall top with its concrete and insulation edge, slab top (7.5 x 5 mm) and the handrail strip.",
            "Wand, Dämmelement und Platte sind drei getrennte Massen; die L-Form bleibt lesbar und die Platte behält ihr Gefälle. Das Geländer ist ein Pfosten mit dem Handlaufkreis darauf. Vorderansicht: Wandfläche, Plattenstirn (5 x 1 mm), der Pfosten, das Stabfeld als ein heller Ton und der Handlaufstreifen. Draufsicht: Wandkopf mit seiner Kante Beton/Dämmung, Plattenoberseite (7,5 x 5 mm) und der Handlaufstreifen.",
        ),
        "retained": text(
            "Wall (0.30 m, 1.5 mm), break module (0.6 x 1.0 mm), slab (7.5 x 1.0 mm), post (0.25 mm), handrail (0.3 mm), the baluster panel as a tone and the concrete/insulation edge inside the wall.",
            "Wand (0,30 m, 1,5 mm), Dämmelement (0,6 x 1,0 mm), Platte (7,5 x 1,0 mm), Pfosten (0,25 mm), Handlauf (0,3 mm), das Stabfeld als Ton und die Kante Beton/Dämmung in der Wand.",
        ),
        "merged": text(
            "Mineral wool and render are merged into the wall mass; slab top, soffit and edge are one slab shape; chords and balusters are one panel tone.",
            "Mineralwolle und Putz sind in der Wandmasse zusammengefasst; Plattenoberseite, -unterseite und -stirn bilden eine Plattenform; Gurte und Füllstäbe sind ein Feldton.",
        ),
        "omitted": text(
            "Reinforcement, chamfers, drip groove, foot, anchors and the 20 mm render line. The thermal break module is hidden behind the slab edge and under the wall top.",
            "Bewehrung, Fasen, Tropfnut, Fuß, Anker und die Putzlinie von 20 mm. Das Dämmelement liegt verdeckt hinter der Plattenstirn und unter dem Wandkopf.",
        ),
        "thinElements": text(
            "The 50 mm post is a 0.25 mm filled bar and the 60 mm handrail a 0.3 mm circle (exact); single balusters (0.06 mm) cannot be printed and are the panel tone.",
            "Der Pfosten von 50 mm ist ein gefüllter Balken von 0,25 mm und der Handlauf von 60 mm ein Kreis von 0,3 mm (exakt); einzelne Füllstäbe (0,06 mm) sind nicht druckbar und sind der Feldton.",
        ),
    },
    "500": {
        "silhouette": text(
            "One closed poché L merges wall stub, thermal break and slab (0.30 m wall plus 1.50 m slab, 1.30 m high); the railing is a single pen line 120 mm inside the edge. Front view (from the free edge towards the wall, strip of 1.0 m): one filled block of wall and slab edge (2 x 2.6 mm) with the post and the rail as two pen lines. Plan view: one filled block of wall top and slab (3.6 x 2 mm) with the railing as a line across the strip.",
            "Eine geschlossene Poché-L-Kontur fasst Wandscheibe, Dämmelement und Platte zusammen (0,30 m Wand plus 1,50 m Platte, 1,30 m hoch); das Geländer ist eine einzige Federlinie 120 mm hinter der Kante. Vorderansicht (von der freien Kante zur Wand, Streifen von 1,0 m): ein gefüllter Block aus Wand und Plattenstirn (2 x 2,6 mm) mit Pfosten und Holm als zwei Federlinien. Draufsicht: ein gefüllter Block aus Wandkopf und Platte (3,6 x 2 mm) mit dem Geländer als Linie über den Streifen.",
        ),
        "omitted": text(
            "Reinforcement, fall, chamfers, drip groove, foot, anchors, baluster infill and every layer line. The thermal break lies behind the slab edge and below the wall top and is not visible in either view.",
            "Bewehrung, Gefälle, Fasen, Tropfnut, Fuß, Anker, Stabfüllung und alle Schichtlinien. Das Dämmelement liegt hinter der Plattenstirn und unter dem Wandkopf und ist in keiner Ansicht sichtbar.",
        ),
        "thinElements": text(
            "Post and rail are 0.15 mm pen lines (75 mm real, the smallest printable feature, for the 50 mm post and the 60 mm tube).",
            "Pfosten und Holm sind Federlinien von 0,15 mm (75 mm real, die kleinste druckbare Einzelheit, für den Pfosten von 50 mm und das Rohr von 60 mm).",
        ),
    },
}


def apply(asset):
    refs = asset["provenance"]["references"]
    for reference in REFERENCES:
        if all(r["id"] != reference["id"] for r in refs):
            refs.append(reference)
    for r in refs:
        if r["id"] == "en-10219":
            r["citation"] = "EN 10219-2:2019 Cold formed welded structural hollow sections, tolerances, dimensions and sectional properties: square 50 x 50 x 4 (A 6.95 cm2, I 23.7 cm4, 5.45 kg/m, outer corner radius 2 t), circular 60.3 x 2.9 (4.11 kg/m)."
    asset["provenance"]["notes"] = NOTES
    asset["dimensions"]["massKg"] = round(SLAB_KG + RAILING_KG, 2)
    for bone in asset["rig"]["bones"]:
        if bone["id"] == "slab":
            bone["mass"].update({"kg": SLAB_KG, "comX": round(SLAB_CX, 3), "comY": round(SLAB_CY, 4)})
            bone["section"]["title"] = text("1 m strip, 200 mm slab at the wall (177.5 mm at the edge)", "1 m Streifen, Platte 200 mm an der Wand (177,5 mm an der Kante)")
        if bone["id"] == "railing":
            bone["title"] = text("Railing (one post per metre with handrail and baluster infill)", "Geländer (ein Pfosten je Meter mit Handlauf und Stabfüllung)")
            bone["x"], bone["y"] = K.POST_X, r5(K.POST_V)
            bone["mass"].update({"kg": RAILING_KG, "comX": RAILING_COM, "comY": 0, "source": "authored-estimate", "reference": "en-10219"})
            bone["section"].update({"areaM2": 0.000695, "secondMomentM4": 2.37e-07})
    for region in asset["thermal"]["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
        if region["id"] == "slab-edge":
            region["characteristicLengthM"] = 0.18
        if region["id"] == "railing-steel":
            region["title"] = text("Railing steel (post, handrail, chords and balusters)", "Geländerstahl (Pfosten, Handlauf, Gurte und Füllstäbe)")
    asset["structure"]["assumptions"][0] = text(
        "Rigid root in a fixed wall connection; the slab is a 1 m strip of concrete, 200 mm at the wall and 177.5 mm at the edge (1.5 % fall), E = 33 GPa; the stiffness is taken from the 200 mm root section and the self-weight from the real tapered section.",
        "Starre Wurzel mit eingespannter Wandverbindung; die Platte ist ein 1 m Streifen aus Beton, 200 mm an der Wand und 177,5 mm an der Kante (1,5 % Gefälle), E = 33 GPa; die Steifigkeit folgt dem Wurzelquerschnitt von 200 mm, das Eigengewicht dem wahren verjüngten Querschnitt.",
    )
    asset["structure"]["assumptions"][1] = text(
        "Crowd and barrier loads are per metre of balcony width; the railing has one rigid fixed post per metre that carries the barrier load at its top, and its mass is the whole railing of that metre (post, plate, handrail, chords and balusters).",
        "Personen- und Geländerlasten beziehen sich auf den Meter Balkonbreite; das Geländer hat je Meter einen starr eingespannten Pfosten, der die Geländerlast an der Oberkante trägt, und seine Masse ist das ganze Geländer dieses Meters (Pfosten, Platte, Handlauf, Gurte und Füllstäbe).",
    )
    drawings = {"50": drawing50(), "100": drawing100(), "200": drawing200(), "500": drawing500()}
    for scale, geometry in drawings.items():
        representation = asset["representations"][scale]
        representation["geometry"] = geometry
        representation["decisions"].update(TEXTS[scale])
    return asset


def member(source, key):
    found = re.search(r'\n    "' + key + r'"\s*:\s*', source)
    depth, index, quoted = 0, found.end(), False
    while True:
        char = source[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return found.end(), index + 1
        index += 1


def dumped(value):
    lines = re.sub(r"(\d)e-0(\d)(?=,?$)", r"\1e-\2", json.dumps(value, indent=2, ensure_ascii=False), flags=re.M).split("\n")
    return "\n".join(line if i == 0 else "    " + line for i, line in enumerate(lines))


if __name__ == "__main__":
    source = open(CATALOG, encoding="utf-8").read()
    document = json.loads(source)
    before = json.loads(source)
    apply(document["asset"])
    if "--apply" in sys.argv:
        for key in MEMBERS:
            start, end = member(source, key)
            if dumped(before["asset"][key]) != source[start:end]:
                raise SystemExit("member %s is not in the plain layout, not written" % key)
            source = source[:start] + dumped(document["asset"][key]) + source[end:]
        check = json.loads(source)
        for key in check["asset"]:
            if key not in MEMBERS and check["asset"][key] != before["asset"][key]:
                raise SystemExit("member %s changed unexpectedly" % key)
        with open(CATALOG, "w", encoding="utf-8") as f:
            f.write(source)
        print("catalog members written:", ", ".join(MEMBERS))
    else:
        document["asset"]["volumes"] = json.load(open(os.path.join(T, "volumes", "components", "cantilever-balcony.json"), encoding="utf-8"))["volumes"]
        folder = os.path.join(T, "🗑️generated", "r3-cantilever-balcony")
        os.makedirs(folder, exist_ok=True)
        with open(os.path.join(folder, "🔣️.json"), "w", encoding="utf-8") as f:
            json.dump(document, f, ensure_ascii=False, indent=2)
        print("candidate written; slab kg", SLAB_KG, "com", round(SLAB_CX, 3), round(SLAB_CY, 4), "railing kg", RAILING_KG, "com", RAILING_COM)
        for name, kg, at in RAILING_PARTS:
            print("  %-36s %6.2f kg at %.3f" % (name, kg, at))
