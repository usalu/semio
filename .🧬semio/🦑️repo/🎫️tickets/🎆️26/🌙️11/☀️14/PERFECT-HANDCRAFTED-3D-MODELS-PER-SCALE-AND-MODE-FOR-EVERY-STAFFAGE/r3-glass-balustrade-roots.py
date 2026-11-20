"""🛠️ Root fixes of components/glass-balustrade: the real clamp construction in summary, references, materials, the 2D drawings of
1:50, 1:100 and 1:200 (ruling E1: the true views of the handcrafted body) and the thermal areas of the rebuilt clamp.

Without flag: writes the candidate document to 🗑️generated/r3-glass-balustrade/🔣️.json (with the volumes source) for
`r3-validate.ts --file` and `r3-2d.ts --file`. With --apply: re-reads the catalog file, replaces only the members
listed in MEMBERS, keeps the file layout and verifies nothing else changed.

See: r3-components-glass-balustrade.md, r3-glass-balustrade-kit.py, round3-brief.md
"""

import importlib.util
import json
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
R = os.path.abspath(os.path.join(T, *[".."] * 7))
CATALOG = os.path.join(R, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "🧱️components", "glass-balustrade", "🔣️.json")
spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-glass-balustrade-kit.py"))
k = importlib.util.module_from_spec(spec)
spec.loader.exec_module(k)
r6 = k.r6

MEMBERS = ["summary", "provenance", "materials", "representations", "thermal"]
AREAS = {"panel-glass": 4.27, "clamp-steel": 2.24, "handrail-steel": 0.27}
STEEL, DARK, RUBBER = 0.35, 0.15, 0.2
HEX_R = k.HEX_AF / 3**0.5
RAIL_BOTTOM = r6(k.RAIL_U - k.RAIL_R)
RAIL_TOP = r6(k.RAIL_U + k.RAIL_R)
CAP_PROFILE = [[0.0, k.RAIL_R], [0.003, k.RAIL_R], [0.0055, 0.0185], [0.0072, 0.0125], [k.CAP, 0.005]]


def tx(en, de):
    """🗣️ A bilingual sentence."""
    return {"en": en, "de": de}


def shape(id, role, z, filled, tone, geometry, region, material=None, hatch=None, pen=None):
    """✏️ A shape of the bone `panel` with the member order of the drawing."""
    out = {"id": id, "bone": "panel", "role": role, "z": z, "filled": filled, "tone": tone, "region": region}
    if material:
        out["material"] = material
    if hatch:
        out["hatch"] = hatch
    if pen:
        out["penMm"] = pen
    out.update(geometry)
    return out


def rect(x0, y0, x1, y1):
    """▭ A rectangle between two corners."""
    return {"kind": "rect", "x": r6(x0), "y": r6(y0), "w": r6(x1 - x0), "h": r6(y1 - y0)}


def poly(points, closed=True):
    """📐 A polygon or stroke."""
    return {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in points], "closed": closed}


def disc(cx, cy, radius):
    """⭕ A circle."""
    return {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(radius), "ry": r6(radius)}


def box(u0, v0, u1, v1):
    """🔲 An elevation rectangle as a polygon (x = u up the bone, y = v across)."""
    return poly([[u0, v0], [u1, v0], [u1, v1], [u0, v1]])


def stepped(base0, base1, top0, top1, low, mid, high, swap=False):
    """🔩 A nut with its thread stub seen from the side: a wide step from low to mid and a narrow one to high; swap puts the height on x."""
    pts = [[base0, low], [base1, low], [base1, mid], [top1, mid], [top1, high], [top0, high], [top0, mid], [base0, mid]]
    return poly([[b, a] for a, b in pts] if swap else pts)


def cap_outline(side, centre):
    """🧢 The dished end cap seen from the side on the +w (1) or -w (-1) end; x along the panel, y across the tube axis at `centre`."""
    upper = [[side * (k.RAIL_HALF_LENGTH + a), centre + rad] for a, rad in CAP_PROFILE]
    lower = [[side * (k.RAIL_HALF_LENGTH + a), centre - rad] for a, rad in reversed(CAP_PROFILE)]
    return upper + lower


def drawing50():
    """📄 1:50: the section across the clamp, the face of the fixed clamp plate and the view from above."""
    top = k.PLATE_T
    nut_top = top + k.NUT_M
    stud_top = top + k.ANCHOR_STUD
    elevation = [
        shape("base-plate", "body", 2, True, STEEL, box(0, -k.PLATE_HALF, top, k.PLATE_HALF), "clamp-steel", "steel", "steel", pen=0.13),
        shape("cheek-fixed", "body", 2, True, STEEL, box(top, -k.CLAMP_HALF, k.SHOE_TOP, -k.SLOT_HALF), "clamp-steel", "steel", "steel", pen=0.13),
        shape("cheek-clamp", "body", 2, True, STEEL, box(top, k.SLOT_HALF, k.SHOE_TOP, k.CLAMP_HALF), "clamp-steel", "steel", "steel", pen=0.13),
        shape("weld-outer", "detail", 2.2, True, DARK, poly([[top, -k.CLAMP_HALF], [top, -k.CLAMP_HALF - k.WELD], [top + k.WELD, -k.CLAMP_HALF]]), "clamp-steel", "steel", pen=0.13),
        shape("weld-inner", "detail", 2.2, True, DARK, poly([[top, -k.SLOT_HALF], [top + k.WELD, -k.SLOT_HALF], [top, -k.SLOT_HALF + k.WELD]]), "clamp-steel", "steel", pen=0.13),
        shape("pad-fixed", "body", 2.5, True, RUBBER, box(k.GLASS_BOTTOM, -k.SLOT_HALF, k.SHOE_TOP, -k.GLASS_HALF), "clamp-steel", "elastomer", pen=0.1),
        shape("pad-clamp", "body", 2.5, True, RUBBER, box(k.GLASS_BOTTOM, k.GLASS_HALF, k.SHOE_TOP, k.SLOT_HALF), "clamp-steel", "elastomer", pen=0.1),
        shape("setting-block", "detail", 2.5, True, RUBBER, box(top, -k.GLASS_HALF, k.GLASS_BOTTOM, k.GLASS_HALF), "clamp-steel", "elastomer", pen=0.1),
        shape("glass", "glazing", 3, True, 0.92, box(k.GLASS_BOTTOM, -k.GLASS_HALF, k.GLASS_TOP, k.GLASS_HALF), "panel-glass", "glass", pen=0.1),
        shape("bolt-head", "detail", 3, True, DARK, box(k.BOLT_U - k.HEX_AF / 2, -k.CLAMP_HALF - k.HEAD_K, k.BOLT_U + k.HEX_AF / 2, -k.CLAMP_HALF), "clamp-steel", "steel", pen=0.13),
        shape(
            "bolt-nut",
            "detail",
            3,
            True,
            DARK,
            stepped(k.BOLT_U - k.HEX_AF / 2, k.BOLT_U + k.HEX_AF / 2, k.BOLT_U - k.BOLT_R, k.BOLT_U + k.BOLT_R, k.CLAMP_HALF, k.CLAMP_HALF + k.NUT_M, k.CLAMP_HALF + k.NUT_M + k.STUB),
            "clamp-steel",
            "steel",
            pen=0.13,
        ),
        shape("anchor-fixed", "detail", 3, True, DARK, stepped(-k.ANCHOR_V - HEX_R, -k.ANCHOR_V + HEX_R, -k.ANCHOR_V - k.BOLT_R, -k.ANCHOR_V + k.BOLT_R, top, nut_top, stud_top, swap=True), "clamp-steel", "steel", pen=0.13),
        shape("anchor-clamp", "detail", 3, True, DARK, stepped(k.ANCHOR_V - HEX_R, k.ANCHOR_V + HEX_R, k.ANCHOR_V - k.BOLT_R, k.ANCHOR_V + k.BOLT_R, top, nut_top, stud_top, swap=True), "clamp-steel", "steel", pen=0.13),
        shape("handrail", "body", 4, True, STEEL, disc(k.RAIL_U, 0, k.RAIL_R), "handrail-steel", "steel", "steel"),
    ]
    front = [
        shape("base-plate-front", "body", 2, True, STEEL, rect(-1, 0, 1, top), "clamp-steel", "steel", pen=0.13),
        shape("cheek-front", "body", 2, True, STEEL, rect(-1, top, 1, k.SHOE_TOP), "clamp-steel", "steel"),
        shape("weld-front", "detail", 2.5, True, 0.25, rect(-1, top, 1, top + k.WELD), "clamp-steel", "steel", pen=0.13),
    ]
    for i, w in enumerate(k.BOLT_W):
        front.append(shape(f"bolt-front-{i}", "detail", 3.5, True, DARK, poly(k.hexagon(w, k.BOLT_U)), "clamp-steel", "steel"))
    for i, w in enumerate(k.ANCHOR_W):
        front.append(shape(f"anchor-front-{i}", "detail", 3.6, True, DARK, stepped(w - k.HEX_AF / 2, w + k.HEX_AF / 2, w - k.BOLT_R, w + k.BOLT_R, top, nut_top, stud_top), "clamp-steel", "steel"))
    front += [
        shape("glass-front", "glazing", 3, True, 0.92, rect(-k.GLASS_HALF_LENGTH, k.SHOE_TOP, k.GLASS_HALF_LENGTH, RAIL_BOTTOM), "panel-glass", "glass"),
        shape("glass-reflex-a", "glazing", 3.5, False, 0.2, poly([[-0.62, 0.3], [-0.34, 0.86]], False), "panel-glass"),
        shape("glass-reflex-b", "glazing", 3.5, False, 0.2, poly([[-0.5, 0.3], [-0.22, 0.86]], False), "panel-glass"),
        shape("handrail-front", "body", 4, True, STEEL, rect(-k.RAIL_HALF_LENGTH, RAIL_BOTTOM, k.RAIL_HALF_LENGTH, RAIL_TOP), "handrail-steel", "steel"),
        shape("handrail-cap-left", "detail", 4.5, True, 0.2, poly(cap_outline(-1, k.RAIL_U)), "handrail-steel", "steel"),
        shape("handrail-cap-right", "detail", 4.5, True, 0.2, poly(cap_outline(1, k.RAIL_U)), "handrail-steel", "steel"),
    ]
    plan = [
        shape("base-plate-plan", "body", 2, True, STEEL, rect(-k.PLATE_HALF, -1, k.PLATE_HALF, 1), "clamp-steel", "steel"),
        shape("weld-plan", "detail", 2.2, True, 0.25, rect(k.CLAMP_HALF, -1, k.CLAMP_HALF + k.WELD, 1), "clamp-steel", "steel", pen=0.13),
        shape("cheek-fixed-plan", "body", 2.5, True, 0.3, rect(k.SLOT_HALF, -1, k.CLAMP_HALF, 1), "clamp-steel", "steel", pen=0.13),
        shape("cheek-clamp-plan", "body", 2.5, True, 0.3, rect(-k.CLAMP_HALF, -1, -k.SLOT_HALF, 1), "clamp-steel", "steel", pen=0.13),
        shape("pad-fixed-plan", "body", 2.6, True, RUBBER, rect(k.GLASS_HALF, -k.GLASS_HALF_LENGTH, k.SLOT_HALF, k.GLASS_HALF_LENGTH), "clamp-steel", "elastomer", pen=0.1),
        shape("pad-clamp-plan", "body", 2.6, True, RUBBER, rect(-k.SLOT_HALF, -k.GLASS_HALF_LENGTH, -k.GLASS_HALF, k.GLASS_HALF_LENGTH), "clamp-steel", "elastomer", pen=0.1),
        shape("glass-plan", "glazing", 3, True, 0.92, rect(-k.GLASS_HALF, -k.GLASS_HALF_LENGTH, k.GLASS_HALF, k.GLASS_HALF_LENGTH), "panel-glass", "glass", pen=0.1),
    ]
    for i, w in enumerate(k.BOLT_W):
        plan.append(shape(f"bolt-head-plan-{i}", "detail", 3.5, True, DARK, rect(k.CLAMP_HALF, w - HEX_R, k.CLAMP_HALF + k.HEAD_K, w + HEX_R), "clamp-steel", "steel"))
        plan.append(
            shape(
                f"bolt-nut-plan-{i}",
                "detail",
                3.5,
                True,
                DARK,
                stepped(w - HEX_R, w + HEX_R, w - k.BOLT_R, w + k.BOLT_R, -k.CLAMP_HALF, -k.CLAMP_HALF - k.NUT_M, -k.CLAMP_HALF - k.NUT_M - k.STUB, swap=True),
                "clamp-steel",
                "steel",
                pen=0.13,
            )
        )
    for i, w in enumerate(k.ANCHOR_W):
        plan.append(shape(f"anchor-fixed-plan-{i}", "detail", 3.5, True, DARK, poly([[-a, b] for a, b in k.hexagon(-k.ANCHOR_V, w)]), "clamp-steel", "steel"))
        plan.append(shape(f"anchor-clamp-plan-{i}", "detail", 3.5, True, DARK, poly([[-a, b] for a, b in k.hexagon(k.ANCHOR_V, w)]), "clamp-steel", "steel"))
    plan += [
        shape("handrail-plan", "body", 4, True, STEEL, rect(-k.RAIL_R, -k.RAIL_HALF_LENGTH, k.RAIL_R, k.RAIL_HALF_LENGTH), "handrail-steel", "steel"),
        shape("handrail-cap-left-plan", "detail", 4.5, True, 0.2, poly([[b, a] for a, b in cap_outline(-1, 0)]), "handrail-steel", "steel"),
        shape("handrail-cap-right-plan", "detail", 4.5, True, 0.2, poly([[b, a] for a, b in cap_outline(1, 0)]), "handrail-steel", "steel"),
        shape("handrail-axis-plan", "texture", 5, False, 0.9, poly([[0, -0.98], [0, 0.98]], False), "handrail-steel", pen=0.1),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def drawing100():
    """📄 1:100: plate, two clamp bars, the glass stroke, plain bolts and studs, the tube."""
    top = k.PLATE_T
    r = 0.01
    stud_top = top + k.ANCHOR_STUD
    elevation = [
        shape("base-plate", "body", 2, True, STEEL, box(0, -k.PLATE_HALF, top, k.PLATE_HALF), "clamp-steel", "steel"),
        shape("clamp-bar-fixed", "body", 2, True, STEEL, box(top, -k.CLAMP_HALF, k.SHOE_TOP, -k.GLASS_HALF), "clamp-steel", "steel"),
        shape("clamp-bar-clamp", "body", 2, True, STEEL, box(top, k.GLASS_HALF, k.SHOE_TOP, k.CLAMP_HALF), "clamp-steel", "steel"),
        shape("clamp-bolt", "detail", 1.8, True, DARK, box(k.BOLT_U - r, -k.CLAMP_HALF - 0.01, k.BOLT_U + r, k.CLAMP_HALF + 0.01), "clamp-steel", "steel"),
        shape("anchor-fixed", "detail", 3, True, DARK, box(top, -k.ANCHOR_V - r, stud_top, -k.ANCHOR_V + r), "clamp-steel", "steel"),
        shape("anchor-clamp", "detail", 3, True, DARK, box(top, k.ANCHOR_V - r, stud_top, k.ANCHOR_V + r), "clamp-steel", "steel"),
        shape("glass-sheet", "glazing", 3, False, 0.92, poly([[top, 0], [k.GLASS_TOP, 0]], False), "panel-glass", "glass", pen=0.2),
        shape("handrail-tube", "body", 4, True, STEEL, disc(k.RAIL_U, 0, k.RAIL_R), "handrail-steel", "steel"),
    ]
    front = [shape("shoe-base-front", "body", 2, True, STEEL, rect(-1, 0, 1, k.SHOE_TOP), "clamp-steel", "steel")]
    for i, w in enumerate(k.BOLT_W):
        front.append(shape(f"clamp-bolt-front-{i}", "detail", 3.5, True, DARK, disc(w, k.BOLT_U, r), "clamp-steel", "steel"))
    for i, w in enumerate(k.ANCHOR_W):
        front.append(shape(f"anchor-front-{i}", "detail", 3.6, True, DARK, rect(w - r, top, w + r, stud_top), "clamp-steel", "steel"))
    front += [
        shape("glass-sheet-front", "glazing", 3, True, 0.92, rect(-k.GLASS_HALF_LENGTH, k.SHOE_TOP, k.GLASS_HALF_LENGTH, RAIL_BOTTOM), "panel-glass", "glass", pen=0.2),
        shape("glass-reflex-a", "glazing", 3.5, False, 0.2, poly([[-0.62, 0.3], [-0.34, 0.86]], False), "panel-glass", pen=0.1),
        shape("glass-reflex-b", "glazing", 3.5, False, 0.2, poly([[-0.5, 0.3], [-0.22, 0.86]], False), "panel-glass", pen=0.1),
        shape("handrail-tube-front", "body", 4, True, STEEL, rect(-1, RAIL_BOTTOM, 1, RAIL_TOP), "handrail-steel", "steel"),
    ]
    plan = [
        shape("shoe-base-plan", "body", 2, True, STEEL, rect(-k.PLATE_HALF, -1, k.PLATE_HALF, 1), "clamp-steel", "steel"),
        shape("clamp-plan", "body", 2.5, True, 0.3, rect(-k.CLAMP_HALF, -1, k.CLAMP_HALF, 1), "clamp-steel", "steel"),
    ]
    for i, w in enumerate(k.BOLT_W):
        plan.append(shape(f"clamp-bolt-plan-{i}", "detail", 2.2, True, DARK, rect(-k.CLAMP_HALF - 0.01, w - r, k.CLAMP_HALF + 0.01, w + r), "clamp-steel", "steel"))
    for i, w in enumerate(k.ANCHOR_W):
        plan.append(shape(f"anchor-fixed-plan-{i}", "detail", 3.5, True, DARK, disc(k.ANCHOR_V, w, r), "clamp-steel", "steel"))
        plan.append(shape(f"anchor-clamp-plan-{i}", "detail", 3.5, True, DARK, disc(-k.ANCHOR_V, w, r), "clamp-steel", "steel"))
    plan.append(shape("handrail-tube-plan", "body", 4, True, STEEL, rect(-k.RAIL_R, -1, k.RAIL_R, 1), "handrail-steel", "steel"))
    return {"elevation": elevation, "front": front, "plan": plan}


def drawing200():
    """📄 1:200: the inverted T of the shoe, the glass line, the round handrail."""
    c, f = k.CLAMP_HALF, 0.03
    tee = [[0, -k.PLATE_HALF], [0, k.PLATE_HALF], [f, k.PLATE_HALF], [f, c], [k.SHOE_TOP, c], [k.SHOE_TOP, -c], [f, -c], [f, -k.PLATE_HALF]]
    return {
        "elevation": [
            shape("shoe-block", "body", 2, True, STEEL, poly(tee), "clamp-steel", "steel", pen=0.1),
            shape("glass-line", "glazing", 3, False, 0.92, poly([[k.SHOE_TOP, 0], [RAIL_BOTTOM, 0]], False), "panel-glass", "glass", pen=0.1),
            shape("handrail-disc", "body", 4, True, STEEL, disc(k.RAIL_U, 0, k.RAIL_R), "handrail-steel", "steel", pen=0.1),
        ],
        "front": [
            shape("shoe-front", "body", 2, True, STEEL, rect(-1, 0, 1, k.SHOE_TOP), "clamp-steel", "steel", pen=0.1),
            shape("glass-front", "glazing", 3, True, 0.92, rect(-1, k.SHOE_TOP, 1, RAIL_BOTTOM), "panel-glass", "glass", pen=0.1),
            shape("handrail-front", "body", 4, True, STEEL, rect(-1, RAIL_BOTTOM, 1, RAIL_TOP), "handrail-steel", "steel", pen=0.1),
        ],
        "plan": [
            shape("shoe-plan", "body", 2, True, STEEL, rect(-k.PLATE_HALF, -1, k.PLATE_HALF, 1), "clamp-steel", "steel", pen=0.1),
            shape("clamp-plan", "body", 2.5, True, 0.3, rect(-c, -1, c, 1), "clamp-steel", "steel", pen=0.1),
            shape("handrail-plan", "body", 4, True, STEEL, rect(-k.RAIL_R, -1, k.RAIL_R, 1), "handrail-steel", "steel", pen=0.1),
        ],
    }


def decisions(rep, texts):
    """📝 Replace the named decision texts of a representation."""
    for key, value in texts.items():
        rep["decisions"][key] = value


def apply(asset):
    """🔧 Apply every root fix to the asset in place."""
    asset["summary"] = tx(
        "A 1.1 m high laminated glass balustrade panel (2 x 10 mm glass with PVB interlayer, 21.5 mm) clamped over 100 mm between two 15 mm steel clamp plates with elastomer strips and M12 through-bolts at 300 mm on an anchored 300 x 15 mm base plate (DIN 18008-4 category B), topped by a slotted 42.4 mm tube handrail; loaded by a handrail line load, a crowd impact and wind.",
        "Ein 1,1 m hohes Brüstungsfeld aus Verbundglas (2 x 10 mm Glas mit PVB-Folie, 21,5 mm), über 100 mm zwischen zwei 15-mm-Stahlklemmblechen mit Elastomerstreifen und Durchgangsschrauben M12 im Abstand von 300 mm auf einer verankerten Fußplatte 300 x 15 mm eingespannt (DIN 18008-4 Kategorie B), bekrönt von einem geschlitzten Rohrhandlauf 42,4 mm; belastet durch Handlauf-Linienlast, Menschenanprall und Wind.",
    )
    refs = asset["provenance"]["references"]
    new_refs = [
        {
            "id": "din-18008-4",
            "citation": "DIN 18008-4:2013 Glass in building, design and construction rules, part 4: additional requirements for barrier glazing; category B (glazing clamped along its base with a continuous handrail), annex B: laminated safety glass 2 x 10 mm with 1.52 mm PVB, clamping height at least 100 mm, steel clamp plates at least 12 mm, bolts at no more than 300 mm, elastomer interlayers and sleeves.",
        },
        {"id": "iso-4014", "citation": "ISO 4014:2011 and ISO 4032:2012 Hexagon head bolts and hexagon nuts, product grades A and B (M12: head height 7.5 mm, nut height 10.8 mm; width across flats 19 mm as in DIN 931 and DIN 934)."},
        {"id": "en-10219-2", "citation": "EN 10219-2:2019 Cold formed welded steel structural hollow sections, part 2: tolerances, dimensions and sectional properties (circular hollow section 42.4 x 2 mm)."},
    ]
    asset["provenance"]["references"] = [x for x in refs if x["id"] not in {n["id"] for n in new_refs}] + new_refs
    asset["provenance"]["notes"] = tx(
        "The side elevation looks along the panel at its end: the glass is a one-metre strip, cantilevered from the clamp, and its normal load acts in that plane. The panel is 2.0 m long (handrail and base plate; the glass 1.96 m); the manifest width 0.3 m is the base plate.",
        "Die Seitenansicht blickt längs des Feldes auf sein Ende: das Glas ist ein 1 m breiter Streifen, in der Klemmung eingespannt; seine Normallast wirkt in dieser Ebene. Das Feld ist 2,0 m lang (Handlauf und Fußplatte; das Glas 1,96 m); die Manifestbreite 0,3 m ist die Fußplatte.",
    )
    extra = [
        {
            "id": "elastomer",
            "title": tx("Elastomer and plastic interlayers (EPDM strips, setting blocks, gasket, sleeves)", "Elastomer- und Kunststoffzwischenlagen (EPDM-Streifen, Klotzungen, Dichtung, Hülsen)"),
            "emissivity": 0.9,
            "solarAbsorptivity": 0.9,
            "densityKgM3": 1150,
            "source": "literature",
            "reference": "en-iso-10456",
            "conductivityWmK": 0.25,
            "specificHeatJKgK": 1000,
        },
        {
            "id": "pvb",
            "title": tx("PVB interlayer 1.52 mm", "PVB-Folie 1,52 mm"),
            "emissivity": 0.9,
            "solarAbsorptivity": 0.08,
            "densityKgM3": 1070,
            "source": "literature",
            "reference": "din-18008-4",
            "conductivityWmK": 0.2,
            "specificHeatJKgK": 1850,
        },
    ]
    asset["materials"] = [m for m in asset["materials"] if m["id"] not in {"elastomer", "pvb"}] + extra

    reps = asset["representations"]
    reps["50"]["geometry"] = drawing50()
    decisions(
        reps["50"],
        {
            "silhouette": tx(
                "The end view of a clamped glass balustrade (DIN 18008-4 category B): the hatched base plate with its anchor nuts, the two hatched clamp plates with the fillet welds of the fixed one, the elastomer strips and the setting block around the laminated glass, the through-bolt with head and nut, and the hatched tube handrail. Front view (2.0 m panel): the fixed clamp plate on the base plate with seven hexagon bolt heads and five anchor nuts, the glass sheet with two reflex strokes and the tube handrail with its dished end caps. Plan view: the base plate with ten anchor nuts, the two clamp plates with bolt heads and nuts, closed by the handrail tube over the glass.",
                "Die Stirnansicht einer eingespannten Glasbrüstung (DIN 18008-4 Kategorie B): die schraffierte Fußplatte mit ihren Ankermuttern, die beiden schraffierten Klemmbleche mit den Kehlnähten des festen, die Elastomerstreifen und die Klotzung um das Verbundglas, die Durchgangsschraube mit Kopf und Mutter und der schraffierte Rohrhandlauf. Vorderansicht (2,0 m Feld): das feste Klemmblech auf der Fußplatte mit sieben Sechskantköpfen und fünf Ankermuttern, die Glasscheibe mit zwei Reflexstrichen und der Rohrhandlauf mit gewölbten Endkappen. Draufsicht: die Fußplatte mit zehn Ankermuttern, die beiden Klemmbleche mit Schraubenköpfen und Muttern, überdeckt vom Handlaufrohr über dem Glas.",
            ),
            "retained": tx(
                "Base plate 300 x 15 mm (6 x 0.3 mm), clamp plates 15 x 110 mm (0.3 x 2.2 mm), elastomer strips 10 x 100 mm (0.2 x 2 mm), setting block, glass (0.43 x 21 mm), bolt head and nut (0.38 mm), anchor nuts, welds, handrail (0.85 mm). Front: plate and clamp plate 40 x 2.5 mm, bolt heads 0.38 mm at 6 mm, glass 39.2 x 18.7 mm, handrail 40 x 0.85 mm. Plan: plate 6 x 40 mm, clamp plates 0.3 mm, handrail 0.85 mm.",
                "Fußplatte 300 x 15 mm (6 x 0,3 mm), Klemmbleche 15 x 110 mm (0,3 x 2,2 mm), Elastomerstreifen 10 x 100 mm (0,2 x 2 mm), Klotzung, Glas (0,43 x 21 mm), Schraubenkopf und Mutter (0,38 mm), Ankermuttern, Nähte, Handlauf (0,85 mm). Vorderansicht: Platte und Klemmblech 40 x 2,5 mm, Schraubenköpfe 0,38 mm im Abstand von 6 mm, Glas 39,2 x 18,7 mm, Handlauf 40 x 0,85 mm. Draufsicht: Platte 6 x 40 mm, Klemmbleche 0,3 mm, Handlauf 0,85 mm.",
            ),
            "omitted": tx(
                "The 2 mm tube wall with its slot and gasket (0.04 mm), washers, the bolt sleeves in the glass holes and the anchors inside the slab. Front: the loose clamp plate behind the glass. Plan: elastomer strips and glass edge lie below the handrail tube.",
                "Die 2-mm-Rohrwand mit Schlitz und Dichtung (0,04 mm), Unterlegscheiben, die Schraubenhülsen in den Glasbohrungen und die Anker in der Decke. Vorderansicht: das lose Klemmblech hinter dem Glas. Draufsicht: Elastomerstreifen und Glaskante liegen unter dem Handlaufrohr.",
            ),
            "thinElements": tx(
                "The 1.5 mm PVB layer is omitted: inside a 0.43 mm glass body a line would leave white channels of 0.115 mm, below the printable gap; the elastomer strips are drawn at their true 10 mm (0.2 mm); the handrail wall is not resolved and the tube is drawn solid with hatch.",
                "Die 1,5-mm-PVB-Schicht entfällt: in einem 0,43 mm breiten Glaskörper ließe eine Linie weiße Kanäle von 0,115 mm, unter der druckbaren Lücke; die Elastomerstreifen sind in wahrer Dicke 10 mm (0,2 mm) gezeichnet; die Handlaufwand wird nicht aufgelöst, das Rohr ist voll mit Schraffur gezeichnet.",
            ),
            "overlays": tx(
                "Clamp group (plate, clamp plates, strips, bolts), glass and handrail are separate overlay cells; the PVB is part of the glass cell.",
                "Klemmgruppe (Platte, Klemmbleche, Streifen, Schrauben), Glas und Handlauf sind getrennte Überlagerungszellen; das PVB gehört zur Glaszelle.",
            ),
        },
    )
    reps["100"]["geometry"] = drawing100()
    decisions(
        reps["100"],
        {
            "silhouette": tx(
                "A light glass strip rising between two dark clamp bars on a base plate to a round handrail. Front view: the dark clamp bar with seven bolt dots, a light glass sheet with two reflex strokes and the round handrail as a strip. Plan view: the base plate as a 3 mm wide bar with the clamp and the handrail strip on its centre line.",
                "Ein heller Glasstreifen, der zwischen zwei dunklen Klemmleisten auf einer Fußplatte zu einem runden Handlauf aufsteigt. Vorderansicht: die dunkle Klemmleiste mit sieben Schraubenpunkten, eine helle Glasscheibe mit zwei Reflexstrichen und der runde Handlauf als Streifen. Draufsicht: die Fußplatte als 3 mm breiter Balken mit Klemmung und Handlaufstreifen auf der Mittellinie.",
            ),
            "retained": tx(
                "Base plate (3 x 0.15 mm), clamp bars (0.25 x 1.1 mm), glass strip (0.22 x 10.6 mm), bolts and anchor studs (0.2 mm) and handrail (0.42 mm). Front: shoe 20 x 1.25 mm, glass 19.6 x 9.3 mm, handrail 20 x 0.42 mm. Plan: plate 3 x 20 mm, clamp 0.7 x 20 mm, handrail 0.42 x 20 mm.",
                "Fußplatte (3 x 0,15 mm), Klemmleisten (0,25 x 1,1 mm), Glasstreifen (0,22 x 10,6 mm), Schrauben und Ankerbolzen (0,2 mm) und Handlauf (0,42 mm). Vorderansicht: Schuh 20 x 1,25 mm, Glas 19,6 x 9,3 mm, Handlauf 20 x 0,42 mm. Draufsicht: Platte 3 x 20 mm, Klemmung 0,7 x 20 mm, Handlauf 0,42 x 20 mm.",
            ),
            "merged": tx(
                "Laminate layers are merged into the glass strip, each elastomer strip into its clamp bar, bolt head, shank and nut into one rod.",
                "Verbundschichten sind im Glasstreifen zusammengefasst, jeder Elastomerstreifen in seiner Klemmleiste, Schraubenkopf, Schaft und Mutter in einem Stab.",
            ),
            "omitted": tx(
                "Welds, setting blocks, sleeves, the handrail gasket and end caps and the PVB layer. Plan: the glass edge lies below the 42 mm handrail.",
                "Nähte, Klotzungen, Hülsen, Handlaufdichtung und Endkappen sowie die PVB-Schicht. Draufsicht: die Glaskante liegt unter dem 42-mm-Handlauf.",
            ),
            "hatching": tx("No hatch: the clamp bars are 0.25 mm wide, the glass 0.2 mm.", "Keine Schraffur: die Klemmleisten sind 0,25 mm breit, das Glas 0,2 mm."),
        },
    )
    reps["200"]["geometry"] = drawing200()
    decisions(
        reps["200"],
        {
            "silhouette": tx(
                "A glass line standing in a small inverted-T steel shoe with a round handrail on top; 5.5 mm tall on paper. Front view: the 2.0 m long panel as a glass sheet between a steel shoe strip and the handrail strip. Plan view: the 0.30 m wide base plate from above with the clamp and the handrail strip on its centre line.",
                "Eine Glaslinie in einem kleinen Stahlschuh in Form eines umgekehrten T mit rundem Handlauf darauf; 5,5 mm hoch auf dem Papier. Vorderansicht: das 2,0 m lange Feld als Glasscheibe zwischen Schuhstreifen aus Stahl und Handlaufstreifen. Draufsicht: die 0,30 m breite Fußplatte von oben mit Klemmung und Handlaufstreifen auf der Mittellinie.",
            ),
            "retained": tx(
                "Shoe (flange 1.5 x 0.15 mm, upstand 0.36 x 0.48 mm), the glass line (4.7 mm) and the handrail disc (0.21 mm). Front: shoe strip (10 x 0.625 mm), glass sheet (10 x 4.7 mm) and handrail strip (10 x 0.21 mm). Plan: plate (1.5 x 10 mm), clamp (0.36 x 10 mm) and handrail (0.21 x 10 mm).",
                "Schuh (Flansch 1,5 x 0,15 mm, Steg 0,36 x 0,48 mm), die Glaslinie (4,7 mm) und die Handlaufscheibe (0,21 mm). Vorderansicht: Schuhstreifen (10 x 0,625 mm), Glasscheibe (10 x 4,7 mm) und Handlaufstreifen (10 x 0,21 mm). Draufsicht: Platte (1,5 x 10 mm), Klemmung (0,36 x 10 mm) und Handlauf (0,21 x 10 mm).",
            ),
            "merged": tx(
                "The two glass panes with the PVB layer are one glass line; base plate, clamp plates, elastomer strips and bolts are one inverted-T shoe.",
                "Die zwei Glasscheiben mit der PVB-Schicht sind eine Glaslinie; Fußplatte, Klemmbleche, Elastomerstreifen und Schrauben sind ein Schuh in Form eines umgekehrten T.",
            ),
            "omitted": tx(
                "Laminate layers, elastomer strips, bolts, anchors, welds and the handrail wall. In plan the glass edge lies below the handrail (42 mm wide) and is not drawn.",
                "Verbundschichten, Elastomerstreifen, Schrauben, Anker, Nähte und die Handlaufwand. In der Draufsicht liegt die Glaskante unter dem Handlauf (42 mm breit) und wird nicht gezeichnet.",
            ),
            "thinElements": tx(
                "The 21.5 mm glass is a 0.1 mm pen line (20 mm); the 42.4 mm handrail is a filled disc at its true size (0.21 mm); the 15 mm base plate is drawn 30 mm thick (0.15 mm), the smallest feature of the scale, and the clamp at its true 125 mm.",
                "Das 21,5-mm-Glas ist eine Linie mit 0,1 mm Feder (20 mm); der 42,4-mm-Handlauf ist eine gefüllte Scheibe in wahrer Größe (0,21 mm); die 15-mm-Fußplatte ist 30 mm dick gezeichnet (0,15 mm), die kleinste Einzelheit des Maßstabs, die Klemmung in wahrer Höhe 125 mm.",
            ),
        },
    )
    reps["500"]["reason"] = tx(
        "A balustrade end view is 0.30 x 1.10 m, i.e. 0.6 x 2.2 mm at 1:500; the 21.5 mm glass is 0.04 mm and the 42 mm handrail 0.08 mm thick, far below every printable stroke; a line would overstate the glass by a factor of three.",
        "Eine Brüstungs-Stirnansicht ist 0,30 x 1,10 m groß, bei 1:500 also 0,6 x 2,2 mm; das 21,5-mm-Glas ist 0,04 mm und der 42-mm-Handlauf 0,08 mm dick, weit unter jeder druckbaren Strichstärke; eine Linie würde das Glas um das Dreifache überzeichnen.",
    )

    for region in asset["thermal"]["regions"]:
        region["areaM2"] = AREAS[region["id"]]
        if region["id"] == "clamp-steel":
            region["title"] = tx("Base plate, clamp plates, elastomer strips and bolts", "Fußplatte, Klemmbleche, Elastomerstreifen und Schrauben")
    return asset


def number(value):
    """🔢 A number as the catalog writes it."""
    if isinstance(value, float) and value != 0 and abs(value) < 1e-4:
        return ("%.12f" % value).rstrip("0")
    return json.dumps(value)


def dumps(value, indent):
    """🖨️ The fully expanded layout of the catalog file."""
    pad = "  " * indent
    if isinstance(value, dict):
        if not value:
            return "{}"
        inner = ",\n".join("%s  %s: %s" % (pad, json.dumps(key, ensure_ascii=False), dumps(v, indent + 1)) for key, v in value.items())
        return "{\n%s\n%s}" % (inner, pad)
    if isinstance(value, list):
        if not value:
            return "[]"
        inner = ",\n".join("%s  %s" % (pad, dumps(v, indent + 1)) for v in value)
        return "[\n%s\n%s]" % (inner, pad)
    if isinstance(value, float):
        return number(value)
    return json.dumps(value, ensure_ascii=False)


def member(source, key):
    """📍 The span of the value of a top-level asset member in the file text."""
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


if __name__ == "__main__":
    source = open(CATALOG, encoding="utf-8").read()
    document = json.loads(source)
    before = json.loads(source)
    apply(document["asset"])
    if "--apply" in sys.argv:
        for key in MEMBERS:
            start, end = member(source, key)
            if json.loads(source[start:end]) != before["asset"][key]:
                raise SystemExit("member %s not located" % key)
            source = source[:start] + dumps(document["asset"][key], 2) + source[end:]
        check = json.loads(source)
        for key in check["asset"]:
            if key not in MEMBERS and check["asset"][key] != before["asset"][key]:
                raise SystemExit("member %s changed unexpectedly" % key)
            if key in MEMBERS and check["asset"][key] != document["asset"][key]:
                raise SystemExit("member %s not written as intended" % key)
        with open(CATALOG, "w", encoding="utf-8") as f:
            f.write(source)
        print("catalog members written:", ", ".join(MEMBERS))
    else:
        document["asset"]["volumes"] = json.load(open(os.path.join(T, "volumes", "components", "glass-balustrade.json"), encoding="utf-8"))["volumes"]
        folder = os.path.join(T, "🗑️generated", "r3-glass-balustrade")
        os.makedirs(folder, exist_ok=True)
        with open(os.path.join(folder, "🔣️.json"), "w", encoding="utf-8") as f:
            json.dump(document, f, ensure_ascii=False, indent=2)
        print(os.path.join(folder, "🔣️.json"))
