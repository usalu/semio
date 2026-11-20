"""🌱 Round 3, appliances/split-ac-indoor-unit: the root fixes (rig, anchors, materials, thermal regions, poses, provenance, the 2D of 1:50, 1:100 and 1:200) applied to the catalog asset; writes the candidate `🗑️generated/r3-split-ac-indoor-unit/🔣️.json` with the volumes source spliced in.

Usage: python3 r3-split-ac-indoor-unit-roots.py [--area discharge-vane=0.15,casing=0.8]
"""
import copy
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("geo", os.path.join(HERE, "r3-split-ac-indoor-unit-geo.py"))
g = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(g)

CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/split-ac-indoor-unit/🔣️.json"))
SOURCE = os.path.join(HERE, "volumes", "appliances", "split-ac-indoor-unit.json")
OUT = os.path.join(HERE, "🗑️generated", "r3-split-ac-indoor-unit", "🔣️.json")
AREAS = {"discharge-vane": 0.141, "casing": 0.9}


def T(en, de):
    """🌐 A bilingual text."""
    return {"en": en, "de": de}


def shape(id, bone, role, z, filled, tone, kind, **fields):
    """✒️ One 2D shape with the common members first."""
    out = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": kind}
    out.update(fields)
    return out


def front_outline(k, z_rim):
    """🟫 The front-view outline: full height between the rims, the rounded rims stepping in to the scaled profile at the ends."""
    prof = g.scaled(g.outer_d(4), k)
    top = max(p[1] for p in prof)
    bot = min(p[1] for p in prof)
    return [(-g.HALF, bot), (-z_rim, 0.0), (z_rim, 0.0), (g.HALF, bot), (g.HALF, top), (z_rim, g.H), (-z_rim, g.H), (-g.HALF, top)]


def plan_outline(k, z_rim):
    """🟩 The plan outline: full depth between the rims, the rims stepping in at the ends."""
    prof = g.scaled(g.outer_d(4), k)
    back = min(p[0] for p in prof)
    front = max(p[0] for p in prof)
    return [(back, -g.HALF), (front, -g.HALF), (g.D, -z_rim), (g.D, z_rim), (front, g.HALF), (back, g.HALF), (0.0, z_rim), (0.0, -z_rim)]


def vane_shapes(view_id, profile, z):
    """🪽 The flap drawn in the side elevation (and, extruded over its span, in the front view)."""
    d = g.path_d(profile)
    if view_id == "elevation":
        return shape("vane", "vane", "body", z, True, 0.6, "path", d=d, region="discharge-vane", material="abs-grey")
    return shape("discharge-vane", "vane", "body", z, True, 0.6, "path", d=d, region="discharge-vane", material="abs-grey", solid={"kind": "extruded", "sweep": "w", "z0": -g.FLAP, "z1": g.FLAP})


def line(id, pts, z=1.5, tone=0.5):
    """➖ An open detail stroke."""
    return shape(id, "unit", "detail", z, False, tone, "path", d=g.open_d(pts))


def rect(id, role, z, filled, tone, x0, y0, x1, y1, **kw):
    """▭ A rectangle from two corners."""
    return shape(id, "unit", role, z, filled, tone, "rect", x=g.r6(x0), y=g.r6(y0), w=g.r6(x1 - x0), h=g.r6(y1 - y0), **kw)


def geometry50():
    """📐 1:50: true side, front and plan views of the 1:50 body."""
    elevation = [
        shape("casing", "unit", "body", 1, True, 0.88, "path", d=g.path_d(g.outer_d(6)), regions=["casing", "intake-grille"], material="abs-white"),
        shape("end-cap-rim", "unit", "detail", 1.5, False, 0.5, "path", d=g.path_d(g.scaled(g.outer_d(10), 0.92))),
        vane_shapes("elevation", g.vane_profile(0.008, 0.004), 2),
    ]
    front = [
        shape("casing", "unit", "body", 1, True, 0.88, "poly", points=[g.pt(p) for p in front_outline(0.92, 0.41)], closed=True, region="casing", material="abs-white"),
        line("cap-joint-r", [(-g.INNER, 0.0), (-g.INNER, g.H)]),
        line("cap-joint-l", [(g.INNER, 0.0), (g.INNER, g.H)]),
        line("cap-rim-r", [(-0.41, 0.0), (-0.41, g.H)], tone=0.6),
        line("cap-rim-l", [(0.41, 0.0), (0.41, g.H)], tone=0.6),
        rect("front-panel", "detail", 1.6, False, 0.5, -g.INNER, g.PANEL[1], g.INNER, g.PANEL[2]),
        vane_shapes("front", g.vane_profile(0.008, 0.004), 2),
        rect("display-window", "detail", 1.7, True, 0.15, 0.32, 0.12, 0.38, 0.155, r=0.004, material="display"),
    ]
    plan = [
        shape("plan-casing", "unit", "body", 1, True, 0.88, "poly", points=[g.pt(p) for p in plan_outline(0.92, 0.41)], closed=True, region="casing", material="abs-white"),
        rect("plan-grille", "detail", 2, True, 0.3, g.GRILLE[0], -g.INNER, g.GRILLE[1], g.INNER, region="intake-grille", material="abs-grey"),
        line("plan-cap-joint-r", [(0.0, -g.INNER), (g.D, -g.INNER)]),
        line("plan-cap-joint-l", [(0.0, g.INNER), (g.D, g.INNER)]),
        line("plan-cap-rim-r", [(0.0, -0.41), (g.D, -0.41)], tone=0.6),
        line("plan-cap-rim-l", [(0.0, 0.41), (g.D, 0.41)], tone=0.6),
    ]
    for i in range(5):
        x = 0.03675 + 0.021 * i
        plan.append(rect(f"plan-bar-{i + 1}", "detail", 2.2, True, 0.75, x, -g.INNER, x + 0.0075, g.INNER, material="abs-grey"))
    for i in range(9):
        y = -0.36375 + 0.09 * i
        plan.append(rect(f"plan-rib-{i + 1}", "detail", 2.3, True, 0.75, g.GRILLE[0], y, g.GRILLE[1], y + 0.0075, material="abs-grey"))
    return {"elevation": elevation, "plan": plan, "front": front}


def geometry100():
    """📐 1:100: the 1:100 body seen from the side, the front and above."""
    elevation = [
        shape("casing", "unit", "body", 1, True, 0.88, "path", d=g.path_d(g.outer_d(10)), regions=["casing", "intake-grille"], material="abs-white"),
        shape("end-cap-rim", "unit", "detail", 1.5, False, 0.5, "path", d=g.path_d(g.scaled(g.outer_d(15), 0.93))),
        vane_shapes("elevation", g.vane_profile(0.015, 0.01, 6), 2),
    ]
    front = [
        shape("casing", "unit", "body", 1, True, 0.88, "poly", points=[g.pt(p) for p in front_outline(0.93, g.INNER)], closed=True, region="casing", material="abs-white"),
        rect("front-panel", "detail", 1.6, False, 0.5, -g.INNER, g.PANEL[1], g.INNER, g.PANEL[2]),
        vane_shapes("front", g.vane_profile(0.015, 0.01, 6), 2),
        rect("display-window", "detail", 1.7, True, 0.15, 0.32, 0.12, 0.38, 0.155, r=0.004, material="display"),
    ]
    plan = [
        shape("plan-casing", "unit", "body", 1, True, 0.88, "poly", points=[g.pt(p) for p in plan_outline(0.93, g.INNER)], closed=True, region="casing", material="abs-white"),
        rect("plan-grille", "detail", 2, True, 0.3, g.GRILLE[0], -g.INNER, g.GRILLE[1], g.INNER, region="intake-grille", material="abs-grey"),
    ]
    return {"elevation": elevation, "plan": plan, "front": front}


def geometry200():
    """📐 1:200: the 1:200 body as three filled forms per view."""
    elevation = [
        shape("casing", "unit", "body", 1, True, 0.88, "path", d=g.path_d(g.outer_d(15)), regions=["casing", "intake-grille"], material="abs-white"),
        vane_shapes("elevation", g.vane_profile(0.03, 0.03, 8), 2),
    ]
    front = [
        rect("casing", "body", 1, True, 0.88, -g.HALF, 0.0, g.HALF, g.H, region="casing", material="abs-white"),
        vane_shapes("front", g.vane_profile(0.03, 0.03, 8), 2),
    ]
    plan = [
        rect("plan-casing", "body", 1, True, 0.88, 0.0, -g.HALF, g.D, g.HALF, region="casing", material="abs-white"),
        rect("plan-grille", "body", 2, True, 0.3, g.GRILLE[0], -g.INNER, g.GRILLE[1], g.INNER, region="intake-grille", material="abs-grey"),
    ]
    return {"elevation": elevation, "plan": plan, "front": front}


DECISIONS = {
    "50": {
        "silhouette": T("The true views of the 1:50 model, 5.8 mm by 4.2 mm on paper. Side elevation: the side cover as a D (flat back on the wall, flat top, front panel, lower quarter circle of 0.115 m radius) with the line where its rounded rim begins, and the curved discharge flap that closes the lower quarter flush and swings down about its rear edge. Front: the 0.85 m casing with rims stepping in at both ends, the side-cover joints, the front panel outline, the flap band along the bottom (the side profile extruded over 0.78 m, so it drops with the flap angle) and the display window. Plan: the outline with its rims, the recessed intake grille with five bars and nine ribs and the side-cover joints.", "Die wahren Ansichten des Modells 1:50, auf Papier 5,8 mm mal 4,2 mm. Seitenansicht: der Seitendeckel als D (ebene Rückseite an der Wand, ebene Oberseite, Frontblende, unterer Viertelkreis mit 0,115 m Radius) mit der Linie, an der sein gerundeter Rand beginnt, und die gewölbte Ausblasklappe, die das untere Viertel bündig schließt und um ihre Hinterkante nach unten schwenkt. Vorderansicht: das 0,85 m lange Gehäuse mit an beiden Enden einspringenden Rändern, die Fugen der Seitendeckel, die Kontur der Frontblende, das Klappenband entlang der Unterkante (das Seitenprofil über 0,78 m ausgezogen, es senkt sich mit dem Klappenwinkel) und das Anzeigefenster. Draufsicht: die Kontur mit ihren Rändern, das vertiefte Ansauggitter mit fünf Stäben und neun Rippen und die Fugen der Seitendeckel."),
        "retained": T("Side-cover rims and joints, front panel, flap, display window, grille bars and ribs.", "Ränder und Fugen der Seitendeckel, Frontblende, Klappe, Anzeigefenster, Gitterstäbe und -rippen."),
        "merged": T("Casing, panel and side covers form one outline per view; the grille is one field under its bars; in the side elevation the grille on top is seen edge-on and merged into the casing outline (regions casing and intake grille).", "Gehäuse, Blende und Seitendeckel bilden je Ansicht eine Kontur; das Gitter ist ein Feld unter seinen Stäben; in der Seitenansicht wird das Gitter oben auf der Kante gesehen und geht in der Gehäusekontur auf (Regionen Gehäuse und Ansauggitter)."),
        "omitted": T("The swing louvres behind the flap, the filter mesh, the evaporator, the fan and the wall plate are hidden in every true view; refrigerant pipes and the drain hose leave through the wall and are not part of the unit.", "Die Schwenklamellen hinter der Klappe, das Filtergewebe, der Verdampfer, der Lüfter und die Wandplatte sind in jeder wahren Ansicht verdeckt; Kältemittelleitungen und Kondensatschlauch gehen durch die Wand und gehören nicht zum Gerät."),
        "thinElements": T("The flap is 8 mm (0.16 mm) at its pivot and 4 mm at its front edge; grille bars and ribs 7.5 mm (0.15 mm).", "Die Klappe ist am Drehpunkt 8 mm (0,16 mm) und an der Vorderkante 4 mm dick; Gitterstäbe und -rippen 7,5 mm (0,15 mm)."),
        "hatching": T("None; tone only: casing light, grille dark with light bars, flap mid-grey, display dark.", "Keine; nur Ton: Gehäuse hell, Gitter dunkel mit hellen Stäben, Klappe mittelgrau, Anzeige dunkel."),
        "styles": T("Outlines 0.18 mm, joints, rims and panel 0.13 mm.", "Konturen 0,18 mm, Fugen, Ränder und Blende 0,13 mm."),
        "overlays": T("Casing, grille and flap carry their regions; the cold jet is drawn by the transport view at the discharge slot.", "Gehäuse, Gitter und Klappe tragen ihre Regionen; der kalte Strahl wird von der Strömungsansicht am Ausblasschlitz gezeichnet."),
    },
    "100": {
        "silhouette": T("The true views of the 1:100 model, 2.9 mm by 2.1 mm: the side cover as a D with its rim line and the 15 mm flap; front: the casing with tapered side covers, the panel outline, the flap band and the display; plan: the outline and the grille field.", "Die wahren Ansichten des Modells 1:100, 2,9 mm mal 2,1 mm: der Seitendeckel als D mit Randlinie und die 15 mm dicke Klappe; Vorderansicht: das Gehäuse mit verjüngten Seitendeckeln, die Blendenkontur, das Klappenband und die Anzeige; Draufsicht: die Kontur und das Gitterfeld."),
        "retained": T("Rim line, front panel, flap, display window and the grille as one field.", "Randlinie, Frontblende, Klappe, Anzeigefenster und das Gitter als ein Feld."),
        "merged": T("Grille bars and ribs merge into one dark field; side-cover joints merge into the rim taper.", "Gitterstäbe und -rippen verschmelzen zu einem dunklen Feld; die Fugen der Seitendeckel gehen in der Randverjüngung auf."),
        "omitted": T("Bars, ribs and the joint lines (below 15 mm = 0.15 mm); everything hidden as at 1:50.", "Stäbe, Rippen und Fugenlinien (unter 15 mm = 0,15 mm); alles Verdeckte wie bei 1:50."),
        "thinElements": T("The flap is generalised to 15 mm (0.15 mm), twice its real thickness.", "Die Klappe ist auf 15 mm (0,15 mm) verallgemeinert, doppelt so dick wie in Wirklichkeit."),
        "hatching": T("None.", "Keine."),
        "styles": T("Outlines 0.18 mm, detail 0.13 mm.", "Konturen 0,18 mm, Details 0,13 mm."),
        "overlays": T("Casing, grille and flap carry their regions.", "Gehäuse, Gitter und Klappe tragen ihre Regionen."),
    },
    "200": {
        "silhouette": T("The 1:200 model, 1.45 mm by 1.05 mm: the casing D with the flap in the side elevation, a plain 0.85 m block with the flap band in front and the outline with the grille field in plan.", "Das Modell 1:200, 1,45 mm mal 1,05 mm: das Gehäuse-D mit der Klappe in der Seitenansicht, ein schlichter Block von 0,85 m mit dem Klappenband in der Vorderansicht und die Kontur mit dem Gitterfeld in der Draufsicht."),
        "retained": T("Casing, flap and grille field.", "Gehäuse, Klappe und Gitterfeld."),
        "merged": T("Panel, rims and side covers merge into the casing outline.", "Blende, Ränder und Seitendeckel gehen in der Gehäusekontur auf."),
        "omitted": T("Panel line, display, rims (below 30 mm = 0.15 mm).", "Blendenlinie, Anzeige, Ränder (unter 30 mm = 0,15 mm)."),
        "thinElements": T("The flap is generalised to 30 mm (0.15 mm).", "Die Klappe ist auf 30 mm (0,15 mm) verallgemeinert."),
        "hatching": T("None.", "Keine."),
        "styles": T("Outline 0.15 mm.", "Kontur 0,15 mm."),
        "overlays": T("Casing, grille and flap carry their regions.", "Gehäuse, Gitter und Klappe tragen ihre Regionen."),
    },
}

MATERIALS = [
    {"id": "aluminium-fin", "title": T("Aluminium fins on copper tubes", "Aluminiumlamellen auf Kupferrohren"), "emissivity": 0.3, "solarAbsorptivity": 0.4, "densityKgM3": 2700, "conductivityWmK": 237, "specificHeatJKgK": 900, "source": "literature", "reference": "incropera-2018"},
    {"id": "san-gf", "title": T("Glass-filled SAN fan rotor", "Glasfaserverstärkter SAN-Lüfterläufer"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 1250, "conductivityWmK": 0.25, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "split-ac-class-data"},
    {"id": "galvanised-steel", "title": T("Galvanised steel wall plate", "Verzinkte Stahl-Wandplatte"), "emissivity": 0.25, "solarAbsorptivity": 0.6, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 470, "elasticModulusPa": 210000000000, "strengthPa": 280000000, "source": "literature", "reference": "incropera-2018"},
]

REGIONS = [
    {"id": "evaporator", "title": T("Evaporator coil", "Verdampfer"), "kind": "surface", "model": "hydronic", "material": "aluminium-fin", "areaM2": 9.6, "areaBasis": "exchange", "characteristicLengthM": 0.025, "geometry": "vertical-plate", "skyViewFactor": 0, "layers": [{"id": "fins", "title": T("Aluminium fins 0.1 mm", "Aluminiumlamellen 0,1 mm"), "kind": "metal", "material": "aluminium-fin", "thicknessM": 0.0001, "resistanceM2KW": 0.0000004, "heatCapacityJM2K": 243, "source": "literature", "reference": "incropera-2018"}], "heatCapacityJM2K": 243, "source": "authored-estimate", "reference": "split-ac-class-data", "hidden": T("The folded fin-and-tube coil (two rows of 7 mm tubes, 25 mm deep, fins at 1.3 mm pitch) wraps the fan behind the grille and the front panel; no drawing shows it. Its area is the wetted fin area of about 540 fins of 0.35 m by 25 mm on both sides (9.6 m2), an exchange area. Refrigerant evaporates at about 7 degC at rated duty and 12 degC at part load (isothermal two-phase flow); the heat it takes from the air is carried by the cold jet.", "Der gefaltete Lamellen-Rohr-Verdampfer (zwei Reihen 7-mm-Rohre, 25 mm tief, Lamellenteilung 1,3 mm) umgibt den Lüfter hinter Gitter und Frontblende; keine Zeichnung zeigt ihn. Seine Fläche ist die benetzte Lamellenfläche von etwa 540 Lamellen zu 0,35 m mal 25 mm auf beiden Seiten (9,6 m2), eine Austauschfläche. Das Kältemittel verdampft bei Nennlast bei etwa 7 degC und bei Teillast bei 12 degC (isothermer Zweiphasenstrom); die der Luft entzogene Wärme trägt der kalte Strahl."), "hydronic": {"key": "cooling", "rows": [{"v": 0.4, "supplyTempC": 12, "returnTempC": 12}, {"v": 1, "supplyTempC": 7, "returnTempC": 7}], "flowPosition": 0.5, "filmM2KW": 0.0005}},
    {"id": "fan", "title": T("Cross-flow fan and motor", "Querstromlüfter und Motor"), "kind": "surface", "model": "passive", "material": "san-gf", "areaM2": 0.69, "areaBasis": "exchange", "characteristicLengthM": 0.1, "geometry": "cylinder", "skyViewFactor": 0, "layers": [{"id": "blades", "title": T("SAN-GF blades 1.5 mm", "SAN-GF-Schaufeln 1,5 mm"), "kind": "polymer", "material": "san-gf", "thicknessM": 0.0015, "resistanceM2KW": 0.006, "heatCapacityJM2K": 2438, "source": "authored-estimate", "reference": "split-ac-class-data"}], "heatCapacityJM2K": 2438, "source": "authored-estimate", "reference": "split-ac-class-data", "hidden": T("The 100 mm cross-flow rotor of 0.66 m with its motor at the right end turns between coil and discharge mouth; no drawing shows it. Its area is the blade area of 35 blades of 0.66 m by 15 mm on both sides (0.69 m2), an exchange area; its 30 W electric input is part of the 700 W of the casing region and its heat part of the air stream.", "Der Querstromläufer von 100 mm und 0,66 m mit dem Motor am rechten Ende dreht zwischen Verdampfer und Ausblasmund; keine Zeichnung zeigt ihn. Seine Fläche ist die Schaufelfläche von 35 Schaufeln zu 0,66 m mal 15 mm auf beiden Seiten (0,69 m2), eine Austauschfläche; seine 30 W elektrische Aufnahme sind Teil der 700 W der Gehäuseregion, seine Wärme Teil des Luftstroms.")},
]

REFERENCES = [
    {"id": "mitsubishi-msz-ap25vg", "citation": "Mitsubishi Electric MSZ-AP25VG(D) wall-mounted indoor unit, specification sheet: 299 mm high, 798 mm wide, 219 mm deep, 10.5 kg (retrieved 2026-11-20).", "url": "https://www.mitsubishielectric.com.au/product/msz-ap25-80vgd-split-system-air-conditioner/"},
    {"id": "daikin-ftxm25n", "citation": "Daikin FTXM25N Perfera wall-mounted indoor unit, technical data table: 294 mm high, 811 mm wide, 272 mm deep, 10 kg (retrieved 2026-11-20).", "url": "https://www.daikin-ce.com/en_us/products/product.table.html/FTXM-N.html"},
]


def main():
    """🚀 Applies the root fixes to a fresh read of the catalog asset and writes the candidate."""
    areas = dict(AREAS)
    for arg in sys.argv[1:]:
        if arg.startswith("--area"):
            for kv in sys.argv[sys.argv.index(arg) + 1].split(","):
                k, v = kv.split("=")
                areas[k] = float(v)
    doc = json.load(open(CATALOG, encoding="utf-8"))
    a = doc["asset"]
    a["summary"] = T("A wall-mounted split air-conditioner indoor unit (0.85 m wide, 0.29 m high, 0.21 m deep, 9 kg) hung on a steel wall plate: top intake grille, front panel, folded evaporator coil around a cross-flow fan, and a curved discharge flap that swings open under the front: rated cooling of 2.5 kW at 0.7 kW electric input, a jet of 450 m3/h cooled by 13 K.", "Ein wandhängendes Innenteil eines Split-Klimageräts (0,85 m breit, 0,29 m hoch, 0,21 m tief, 9 kg) an einer Stahl-Wandplatte: Ansauggitter oben, Frontblende, gefalteter Verdampfer um einen Querstromlüfter und eine gewölbte Ausblasklappe, die unter der Front aufschwenkt: Nennkühlleistung 2,5 kW bei 0,7 kW elektrischer Aufnahme, ein Strahl von 450 m3/h, um 13 K abgekühlt.")
    refs = [r for r in a["provenance"]["references"] if r["id"] not in {x["id"] for x in REFERENCES}]
    a["provenance"]["references"] = refs + REFERENCES
    a["provenance"]["notes"] = T("Class data of 2.5 kW high-wall split indoor units, checked against the Mitsubishi MSZ-AP25VG (0.299 x 0.798 x 0.219 m, 10.5 kg) and the Daikin FTXM25N (0.294 x 0.811 x 0.272 m, 10 kg); the profile is an original drawing of that type: a D-shaped casing with the discharge flap on its lower quarter circle, hung on a wall plate. The origin is the bottom of the casing on the wall plane, the hook sits 0.27 m above it: place the origin 0.29 m below the mounting height. Rated cooling follows the EN 14511 and ISO 5151 test point (35 C outdoor, 27 C / 19 C indoor); every number is an authored estimate of that class.", "Klassendaten von 2,5-kW-Hochwand-Innengeräten, geprüft an Mitsubishi MSZ-AP25VG (0,299 x 0,798 x 0,219 m, 10,5 kg) und Daikin FTXM25N (0,294 x 0,811 x 0,272 m, 10 kg); das Profil ist eine eigene Zeichnung dieses Typs: ein D-förmiges Gehäuse mit der Ausblasklappe auf seinem unteren Viertelkreis, an einer Wandplatte eingehängt. Der Ursprung liegt an der Unterkante des Gehäuses in der Wandebene, der Haken sitzt 0,27 m darüber: den Ursprung 0,29 m unter die Montagehöhe setzen. Die Nennkühlung folgt dem Prüfpunkt von EN 14511 und ISO 5151 (außen 35 C, innen 27 C / 19 C); jede Zahl ist eine Schätzung dieser Klasse.")
    a["materials"] = [m for m in a["materials"] if m["id"] not in {x["id"] for x in MATERIALS}] + MATERIALS
    unit, vane = a["rig"]["bones"]
    unit["title"] = T("Casing with coil, fan and chassis on the wall plate", "Gehäuse mit Verdampfer, Lüfter und Chassis an der Wandplatte")
    vane["title"] = T("Discharge flap", "Ausblasklappe")
    vane.update({"x": g.PIVOT[0], "y": g.PIVOT[1], "angleDeg": g.VANE_ANGLE, "lengthM": g.VANE_LENGTH})
    vane["mass"]["comX"] = 0.033
    vane["mass"]["comY"] = -0.005
    for an in a["anchors"]:
        if an["id"] == "jet-outlet":
            an["x"], an["y"] = g.JET
            an["title"] = T("Centre of the discharge mouth behind the flap", "Mitte des Ausblasmunds hinter der Klappe")
        if an["id"] == "wall-hook":
            an["title"] = T("Top hook of the chassis on the wall plate", "Oberer Haken des Chassis an der Wandplatte")
        if an["id"] == "wall-rest":
            an["title"] = T("Lower edge of the chassis against the wall plate", "Unterkante des Chassis an der Wandplatte")
    th = a["thermal"]
    th["regions"] = [r for r in th["regions"] if r["id"] not in {x["id"] for x in REGIONS}] + copy.deepcopy(REGIONS)
    for r in th["regions"]:
        if r["id"] in areas:
            r["areaM2"] = areas[r["id"]]
        if r["id"] == "discharge-vane":
            r["title"] = T("Discharge flap in the cold jet", "Ausblasklappe im kalten Strahl")
    extra = T("The evaporator coil and the cross-flow fan are hidden regions inside the casing: the coil follows the refrigerant temperature of the cooling duty (circuit rows: evaporation at 7 degC at rated duty and 12 degC at part load), the fan is passive in the air stream.", "Verdampfer und Querstromlüfter sind verborgene Regionen im Gehäuse: Der Verdampfer folgt der Kältemitteltemperatur des Kühlbetriebs (Kreiszeilen: Verdampfung bei 7 degC bei Nennlast und 12 degC bei Teillast), der Lüfter ist passiv im Luftstrom.")
    th["assumptions"] = [x for x in th["assumptions"] if x["en"] != extra["en"]] + [extra]
    for p in a["poses"]:
        regs = p["thermal"]["regions"]
        for rid in ("evaporator", "fan"):
            regs[rid] = {"exposedFraction": 1, "sunlitFraction": 0}
    a["structure"]["assumptions"] = [T("The 9 kg unit hangs with its chassis on the top hooks of a galvanised wall plate (pin) and rests with its lower edge against the plate (roller): the hook carries the weight, the pair resists the overturning moment of the centre of mass 0.1 m in front of the wall.", "Das Gerät von 9 kg hängt mit seinem Chassis an den oberen Haken einer verzinkten Wandplatte (Festlager) und liegt mit der Unterkante an der Platte an (Rollenlager): Der Haken trägt das Gewicht, das Paar nimmt das Kippmoment des Schwerpunkts 0,1 m vor der Wand auf.")]
    reps = a["representations"]
    for key, build in (("50", geometry50), ("100", geometry100), ("200", geometry200)):
        reps[key]["geometry"] = build()
        reps[key]["decisions"] = DECISIONS[key]
    reps["500"]["reason"] = T("At 1:500 the unit is 0.58 mm tall and 0.42 mm deep and its 8 mm discharge flap 0.016 mm thick; its form shrinks to a dot below every print profile.", "Bei 1:500 ist das Gerät 0,58 mm hoch und 0,42 mm tief und seine 8 mm dicke Ausblasklappe 0,016 mm; seine Form schrumpft unter jedem Druckprofil zu einem Punkt.")
    reps["500"]["limitingFeature"] = T("Discharge flap thickness of 0.008 m", "Dicke der Ausblasklappe von 0,008 m")
    reps["500"]["limitingSizeM"] = 0.008
    a["volumes"] = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    json.dump(doc, open(OUT, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print("written", OUT)


if __name__ == "__main__":
    main()
