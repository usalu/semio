"""🐕 The authoritative script of `used-dog-waste-station`: rig, materials, thermal and structural model, poses, clips and the four scales, written to `🔣️.json`.

Run: `python3 author-used-dog-waste-station.py` in this folder. Measured outer areas of the regions (1:50 model, validator) are in `AREAS`.
See `../../brief.md` and `../../reports/used-dog-waste-station.md`.
"""
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

from author_geometry import BONES, bone_frame, volume_and_centre, loc, r6
import author_scales as S
import author_drawings as D

T = lambda en, de: {"en": en, "de": de}

bone_frame("footing", 0.0, -0.60, 90)
bone_frame("post", 0.0, -0.50, 90)
bone_frame("sign", 0.034, 1.66, 90)
bone_frame("dispenser", 0.035, 1.05, 90)
bone_frame("dispenser-lid", 0.037, 1.355, 0)
bone_frame("bag-roll", 0.10, 1.22, 0)
bone_frame("bag-tab", 0.1375, 1.05, -90)
bone_frame("bin", 0.035, 0.40, 90)
bone_frame("bin-hood", 0.037, 0.8655, 0)
bone_frame("throw-flap", 0.2585, 0.809, -90)
bone_frame("left-bag", 0.09, 0.02, 0)

REFERENCES = [
    {"id": "dog-station-catalogue", "citation": "Catalogue-typical German municipal dog waste station (Hundetoilette, Hundekotbeutelspender mit Abfallbehälter): post-mounted roll-bag dispenser of powder-coated sheet steel about 240 x 130 x 300 mm for 100 to 200 bags, a 25 to 35 l bin with throw-in flap and lockable lid, sign plate about 300 x 250 mm, galvanised square post 60 x 60 mm set in a point foundation; street-furniture catalogues of German municipal suppliers, 2005-2020 (product type, no maker)."},
    {"id": "din-en-10219-2", "citation": "DIN EN 10219-2:2019 Cold formed welded steel structural hollow sections - Part 2: Tolerances, dimensions and sectional properties (SHS 60 x 60 x 3: 5.19 kg/m, outer corner radius 1.5 t to 2 t)."},
    {"id": "din-en-1991-1-1", "citation": "DIN EN 1991-1-1:2010-12 Eurocode 1: Actions on structures - Part 1-1: Densities, self-weight, imposed loads for buildings (Table A.1: normal-weight concrete 24 kN/m3, steel 77-78.5 kN/m3, aluminium 27 kN/m3)."},
    {"id": "din-en-1991-1-4", "citation": "DIN EN 1991-1-4:2010-12 with DIN EN 1991-1-4/NA:2010-12 Wind actions: peak velocity pressure 0.65 kN/m2 for wind zone 2, inland, h up to 10 m (Table NA.B.3); force coefficient 1.80 for signboards (7.4.3) and about 1.5 for short rectangular boxes after the end-effect factor (7.6, 7.13)."},
    {"id": "din-en-12899-1", "citation": "DIN EN 12899-1:2008-02 Fixed, vertical road traffic signs - Part 1: Fixed signs (sign plates of 2 mm aluminium, band clamps on posts, point foundations of posts)."},
    {"id": "din-18040-1", "citation": "DIN 18040-1:2010-10 Barrier-free building design - Part 1: Publicly accessible buildings (operating height of controls and throw-in openings 85 cm, reach range 85 to 105 cm)."},
    {"id": "din-en-206", "citation": "DIN EN 206:2021-06 Concrete - Specification, performance, production and conformity (C20/25 for unreinforced point foundations, exposure XC2/XF1)."},
    {"id": "din-en-iso-1461", "citation": "DIN EN ISO 1461:2022-12 Hot dip galvanized coatings on fabricated iron and steel articles (zinc coating of 55 to 85 micrometres on 1.5 to 3 mm steel; weathering to a dull zinc patina with white rust)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (density, conductivity and specific heat of concrete, steel, aluminium and polymers)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (Table A.11 total normal emissivity and solar absorptance: galvanised sheet new 0.13 / 0.65, weathered and oxidised zinc 0.25-0.28, painted surfaces 0.85-0.95, concrete 0.88-0.93)."},
    {"id": "ashrae-2017", "citation": "ASHRAE Handbook - Fundamentals (2017), chapter 15 and 33: solar absorptance of dark green paint 0.85-0.9, of white or light faded films 0.3-0.45, of weathered concrete 0.6-0.7."},
    {"id": "dog-bag-typical", "citation": "Catalogue-typical dog waste bags on a roll for public dispensers: HDPE film 12 to 15 micrometres, 20 x 30 cm, about 1.7 g per bag, perforated, 100 to 200 bags per roll; a filled bag about 0.1 to 0.25 kg (product type, no maker)."},
]

MATERIALS = [
    {"id": "concrete-weathered", "title": T("Weathered C20/25 concrete", "Verwitterter Beton C20/25"), "emissivity": 0.92, "solarAbsorptivity": 0.65, "densityKgM3": 2400, "conductivityWmK": 2.0, "specificHeatJKgK": 1000, "elasticModulusPa": 30e9, "strengthPa": 20e6, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "steel-galvanised", "title": T("Weathered hot-dip galvanised steel", "Verwitterter feuerverzinkter Stahl"), "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "incropera-2018"},
    {"id": "steel-powder-green", "title": T("Powder-coated steel, moss green, chalked", "Pulverbeschichteter Stahl, moosgrün, ausgekreidet"), "emissivity": 0.9, "solarAbsorptivity": 0.86, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "ashrae-2017"},
    {"id": "steel-corroded", "title": T("Corroded galvanised steel with flaked paint", "Korrodierter verzinkter Stahl mit abgeplatztem Lack"), "emissivity": 0.85, "solarAbsorptivity": 0.8, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "source": "literature", "reference": "incropera-2018"},
    {"id": "aluminium-sign", "title": T("Aluminium sign plate with faded printed film", "Aluminium-Schildplatte mit verblasster Druckfolie"), "emissivity": 0.88, "solarAbsorptivity": 0.4, "densityKgM3": 2700, "conductivityWmK": 160, "specificHeatJKgK": 900, "elasticModulusPa": 70e9, "strengthPa": 190e6, "source": "literature", "reference": "ashrae-2017"},
    {"id": "bag-film-roll", "title": T("Wound HDPE bag film (roll with air)", "Gewickelte HDPE-Beutelfolie (Rolle mit Luft)"), "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": 300, "conductivityWmK": 0.12, "specificHeatJKgK": 1900, "source": "authored-estimate", "reference": "dog-bag-typical"},
    {"id": "filled-bag", "title": T("Filled and knotted dog-waste bag", "Gefüllter, verknoteter Hundekotbeutel"), "emissivity": 0.92, "solarAbsorptivity": 0.92, "densityKgM3": 500, "conductivityWmK": 0.4, "specificHeatJKgK": 3000, "source": "authored-estimate", "reference": "dog-bag-typical"},
]
DENSITY = {m["id"]: m["densityKgM3"] for m in MATERIALS}


def bone(bid, parent, title, x, y, angle, length, joint, mass=None, section=None):
    """🦴 A rig bone."""
    out = {"id": bid, "parent": parent, "title": title, "x": x, "y": y, "angleDeg": angle, "lengthM": length, "joint": joint}
    if mass:
        out["mass"] = mass
    if section:
        out["section"] = section
    return out


def build(d):
    """🏗 One scale: the solids of `author_scales` and the drawings of `author_drawings`."""
    lv = {50: 1, 100: 2, 200: 3, 500: 4}[d]
    sc = S.Scale(d)
    S.footing(sc, lv <= 2)
    S.left_bag(sc, lv == 1)
    S.post(sc, lv == 1)
    sx0, sx1 = S.sign(sc, lv <= 2)
    S.dispenser(sc, lv)
    S.waste_bin(sc, lv)
    D.footing(sc, lv)
    D.left_bag(sc, lv)
    D.post(sc, lv)
    D.sign(sc, lv, sx0, sx1)
    if lv <= 2:
        D.clamps(sc, lv, "sign", (1.70, 1.87), 0.034)
    D.dispenser(sc, lv)
    D.waste_bin(sc, lv)
    return sc


def masses_of(sc):
    """⚖️ Mass and local centre of every bone from the 1:50 body solids; the embedded post envelope is taken out of the footing."""
    out = {}
    for s in sc.body:
        vol, c = volume_and_centre(s)
        m = vol * DENSITY[s["material"]]
        if s["id"] == "footing-block":
            emb_v = 0.06 * 0.06 * (-0.03 + 0.50)
            ec = loc("footing", 0.0, (-0.50 - 0.03) / 2)
            emb = emb_v * DENSITY[s["material"]]
            c = ((c[0] * m - ec[0] * emb) / (m - emb), (c[1] * m - ec[1] * emb) / (m - emb), c[2])
            m -= emb
        if s["id"] == "footing-head":
            emb = 0.06 * 0.06 * 0.05 * DENSITY[s["material"]]
            ec = loc("footing", 0.0, -0.005)
            c = ((c[0] * m - ec[0] * emb) / (m - emb), (c[1] * m - ec[1] * emb) / (m - emb), c[2])
            m -= emb
        b = out.setdefault(s["bone"], [0.0, 0.0, 0.0, 0.0])
        b[0] += m
        b[1] += c[0] * m
        b[2] += c[1] * m
        b[3] += c[2] * m
    return {k: (v[0], v[1] / v[0], v[2] / v[0], v[3] / v[0]) for k, v in out.items()}


SC = {d: build(d) for d in (50, 100, 200, 500)}
MASS = masses_of(SC[50])


def seg_mass(bid, src="literature", ref="din-en-1991-1-1"):
    """⚖️ The SegmentMass of a bone from the 1:50 model."""
    m, cu, cv, _ = MASS[bid]
    return {"kg": round(m, 3), "comX": round(cu, 4), "comY": round(cv, 4), "distribution": "point", "source": src, "reference": ref}


def post_section():
    """📏 Area, second moment and extreme fibre of the modelled SHS 60 x 60 x 3 with its corner radii."""
    outer, bore = S.post_profile(0.03, 0.003, 0.0045, 0.0015)

    def ixx(pts):
        a = 0.0
        for i in range(len(pts)):
            x0, y0 = pts[i]
            x1, y1 = pts[(i + 1) % len(pts)]
            cr = x0 * y1 - x1 * y0
            a += (y0 * y0 + y0 * y1 + y1 * y1) * cr
        return abs(a) / 12

    from author_geometry import area
    A = abs(area(outer)) - abs(area(bore))
    I = ixx(outer) - ixx(bore)
    return {"areaM2": round(A, 8), "secondMomentM4": round(I, 11), "extremeFibreM": 0.03, "material": "steel-powder-green", "title": T("SHS 60 x 60 x 3 (EN 10219-2), corner radii 4.5 / 1.5 mm", "Quadratrohr 60 x 60 x 3 (EN 10219-2), Eckradien 4,5 / 1,5 mm")}


RIG = {"kind": "mechanical", "root": "footing", "bones": [
    bone("footing", None, T("Concrete point foundation, up its axis from the base 0.60 m below ground to its head 20 mm above the lawn", "Punktfundament aus Beton, entlang seiner Achse von der Sohle 0,60 m unter Gelände bis zum Kopf 20 mm über dem Rasen"), 0.0, -0.60, 90, 0.62, {"type": "fixed"}, seg_mass("footing")),
    bone("post", "footing", T("Square hollow post, from its foot 0.50 m below ground in the footing to the cap at 1.93 m", "Quadratrohrpfosten, vom Fuß 0,50 m unter Gelände im Fundament bis zur Kappe auf 1,93 m"), 0.10, 0.0, 0, 2.43, {"type": "fixed"}, seg_mass("post", "literature", "din-en-10219-2"), post_section()),
    bone("sign", "post", T("Sign plate, up its face from 1.66 to 1.91 m", "Schildplatte, entlang der Fläche von 1,66 bis 1,91 m"), 2.16, -0.034, 0, 0.25, {"type": "fixed"}, seg_mass("sign", "literature", "din-en-12899-1")),
    bone("dispenser", "post", T("Bag dispenser housing, up its rear wall from the bottom at 1.05 m to the top edge", "Beutelspendergehäuse, entlang der Rückwand vom Boden auf 1,05 m bis zur Oberkante"), 1.55, -0.035, 0, 0.30, {"type": "fixed"}, seg_mass("dispenser", "authored-estimate", "dog-station-catalogue")),
    bone("dispenser-lid", "dispenser", T("Dispenser lid, from its rear hinge forward", "Spenderdeckel, vom hinteren Scharnier nach vorn"), 0.305, -0.002, -90, 0.12, {"type": "revolute", "minDeg": 0, "maxDeg": 88}, seg_mass("dispenser-lid", "authored-estimate", "dog-station-catalogue")),
    bone("bag-roll", "dispenser", T("Roll of bags on its axle, lifted out upwards to be changed", "Beutelrolle auf ihrer Achse, zum Wechseln nach oben herausgehoben"), 0.17, -0.065, -90, 0.0375, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.32}, seg_mass("bag-roll", "authored-estimate", "dog-bag-typical")),
    bone("bag-tab", "bag-roll", T("Bag tab hanging from the slot, pulled down to take a bag", "Beutelzipfel aus dem Schlitz, zum Entnehmen nach unten gezogen"), 0.0375, -0.17, -90, 0.04, {"type": "prismatic", "axisDeg": -90, "minM": 0, "maxM": 0.18}, seg_mass("bag-tab", "authored-estimate", "dog-bag-typical")),
    bone("bin", "post", T("30 l bin body, up its rear wall from the bottom at 0.40 m", "30-l-Behälter, entlang der Rückwand vom Boden auf 0,40 m"), 0.90, -0.035, 0, 0.46, {"type": "fixed"}, seg_mass("bin", "literature", "din-en-1991-1-1")),
    bone("bin-hood", "bin", T("Rain hood, from its rear hinge forward; lifted 45 degrees onto its stay under the dispenser to empty the bin", "Regenhaube, vom hinteren Scharnier nach vorn; zum Leeren um 45 Grad auf ihren Aufsteller unter dem Spender gehoben"), 0.4655, -0.002, -90, 0.12, {"type": "revolute", "minDeg": 0, "maxDeg": 45}, seg_mass("bin-hood", "literature", "din-en-1991-1-1")),
    bone("throw-flap", "bin", T("Throw-in flap, hanging from its rod and pushed inwards by a bag", "Einwurfklappe, an ihrer Stange hängend und von einem Beutel nach innen gedrückt"), 0.409, -0.2235, -180, 0.105, {"type": "revolute", "minDeg": -70, "maxDeg": 0}, seg_mass("throw-flap", "literature", "din-en-1991-1-1")),
    bone("left-bag", "footing", T("Filled bag left on the footing head", "Auf dem Fundamentkopf abgelegter gefüllter Beutel"), 0.62, -0.09, -90, 0.06, {"type": "fixed"}, seg_mass("left-bag", "authored-estimate", "dog-bag-typical")),
]}
MASS_KG = round(sum(b["mass"]["kg"] for b in RIG["bones"] if "mass" in b), 3)


def anchor(aid, bid, x, y, kind, title, **extra):
    """📍 An anchor at world (x, y) of the rest pose on a bone."""
    u, v = loc(bid, x, y)
    out = {"id": aid, "bone": bid, "x": u, "y": v, "kind": kind, "title": title}
    out.update(extra)
    return out


ANCHORS = [
    anchor("footing-base", "footing", 0.0, -0.60, "support", T("Point foundation fixed in the soil at its base", "Punktfundament an der Sohle im Boden eingespannt"), support="fixed", normalDeg=90, footprintLengthM=0.30, footprintWidthM=0.30),
    anchor("bin-floor", "bin", 0.15, 0.4015, "load", T("Centre of the bin floor", "Mitte des Behälterbodens")),
    anchor("bin-centre", "bin", 0.15, 0.63, "load", T("Centre of the bin face, wind", "Mitte der Behälterfläche, Wind")),
    anchor("dispenser-centre", "dispenser", 0.10, 1.20, "load", T("Centre of the dispenser face, wind", "Mitte der Spenderfläche, Wind")),
    anchor("sign-centre", "sign", 0.035, 1.785, "load", T("Centre of the sign plate, wind", "Mitte der Schildplatte, Wind")),
    anchor("roll-hand", "bag-roll", 0.10, 1.22, "load", T("Hand of the depot worker holding the roll", "Hand des Bauhofmitarbeiters an der Rolle")),
    anchor("tab-grip", "bag-tab", 0.1375, 1.01, "load", T("Fingers pulling the bag tab", "Finger am Beutelzipfel")),
    anchor("flap-push", "throw-flap", 0.2535, 0.729, "load", T("Filled bag pushing the flap", "Gefüllter Beutel drückt die Klappe")),
]

REGION_LIST = [
    ("footing-concrete", T("Concrete footing head and buried block", "Betonfundament, Kopf und eingegrabener Block"), "wall", "concrete-weathered", "vertical-plate", 0.30, 0.1, ("masonry", 0.15, "literature", "iso-10456-2007")),
    ("left-bag", T("Filled bag left on the footing", "Abgelegter gefüllter Beutel"), "surface", "filled-bag", "sphere", 0.1, 0.6, ("polymer", 0.04, "authored-estimate", "dog-bag-typical")),
    ("post-paint", T("Powder-coated post and cap", "Pulverbeschichteter Pfosten mit Kappe"), "surface", "steel-powder-green", "cylinder", 0.06, 0.5, ("metal", 0.003, "literature", "iso-10456-2007")),
    ("post-foot-corroded", T("Corroded post foot above the footing", "Korrodierter Pfostenfuß über dem Fundament"), "surface", "steel-corroded", "cylinder", 0.06, 0.45, ("metal", 0.003, "literature", "iso-10456-2007")),
    ("sign-plate", T("Faded sign plate", "Verblasste Schildplatte"), "surface", "aluminium-sign", "vertical-plate", 0.25, 0.5, ("metal", 0.002, "literature", "iso-10456-2007")),
    ("brackets-galvanised", T("Band clamps and mounting plates", "Schellen und Montageplatten"), "surface", "steel-galvanised", "vertical-plate", 0.1, 0.3, ("metal", 0.003, "literature", "iso-10456-2007")),
    ("dispenser-housing", T("Dispenser housing, outside and inside", "Spendergehäuse, außen und innen"), "surface", "steel-powder-green", "vertical-plate", 0.30, 0.45, ("metal", 0.0015, "literature", "iso-10456-2007")),
    ("dispenser-lid", T("Dispenser lid", "Spenderdeckel"), "surface", "steel-powder-green", "horizontal-plate-up", 0.13, 0.9, ("metal", 0.0015, "literature", "iso-10456-2007")),
    ("bag-roll", T("Roll of bags and bag tab", "Beutelrolle und Beutelzipfel"), "surface", "bag-film-roll", "cylinder", 0.075, 0.05, ("polymer", 0.027, "authored-estimate", "dog-bag-typical")),
    ("bin-body", T("Galvanised bin body, flap and floor, outside and inside", "Verzinkter Behälter, Klappe und Boden, außen und innen"), "surface", "steel-galvanised", "vertical-plate", 0.46, 0.45, ("metal", 0.0015, "literature", "iso-10456-2007")),
    ("bin-hood", T("Galvanised rain hood", "Verzinkte Regenhaube"), "surface", "steel-galvanised", "horizontal-plate-up", 0.25, 0.95, ("metal", 0.0015, "literature", "iso-10456-2007")),
]
AREAS = {"footing-concrete": 0.92, "left-bag": 0.031, "post-paint": 0.421, "post-foot-corroded": 0.0302, "sign-plate": 0.152, "brackets-galvanised": 0.19, "dispenser-housing": 0.50, "dispenser-lid": 0.084, "bag-roll": 0.1225, "bin-body": 1.14, "bin-hood": 0.196}


def regions():
    """🌡 The thermal regions with one layer each; the hidden roll counts its whole surface."""
    mats = {m["id"]: m for m in MATERIALS}
    out = []
    for rid, title, kind, mat, geom, lc, sky, (lkind, th, src, ref) in REGION_LIST:
        m = mats[mat]
        cap = round(m["densityKgM3"] * m["specificHeatJKgK"] * th, 1)
        layer = {"id": f"{rid}-layer", "title": T(f"{m['title']['en']} {th * 1000:g} mm", f"{m['title']['de']} {str(th * 1000).rstrip('0').rstrip('.').replace('.', ',')} mm"), "kind": lkind, "material": mat, "thicknessM": th, "resistanceM2KW": round(th / m["conductivityWmK"], 7), "heatCapacityJM2K": cap, "source": src, "reference": ref}
        r = {"id": rid, "title": title, "kind": kind, "model": "passive", "material": mat, "areaM2": AREAS[rid], "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": sky, "layers": [layer], "heatCapacityJM2K": cap, "source": src, "reference": ref}
        if rid == "bag-roll":
            r["hidden"] = T("The roll and the inner part of the tab lie inside the closed dispenser; only the tab tip shows below the slot.", "Rolle und innerer Teil des Zipfels liegen im geschlossenen Spender; nur die Zipfelspitze zeigt sich unter dem Schlitz.")
        if rid == "dispenser-housing":
            r["ratedPowerW"] = 0
            r["ratedPowerBasis"] = "passive"
        out.append(r)
    return out


def binding(**over):
    """🌞 The region binding of a pose: exposed and sunlit shares in the summer afternoon sun, with overrides."""
    base = {
        "footing-concrete": {"exposedFraction": 0.1, "sunlitFraction": 0.05, "contact": {"fraction": 0.88, "tempC": 16}},
        "left-bag": {"exposedFraction": 0.85, "sunlitFraction": 0.6, "contact": {"fraction": 0.15, "tempC": 30}},
        "post-paint": {"exposedFraction": 0.85, "sunlitFraction": 0.4},
        "post-foot-corroded": {"exposedFraction": 1, "sunlitFraction": 0.4},
        "sign-plate": {"exposedFraction": 1, "sunlitFraction": 0.5},
        "brackets-galvanised": {"exposedFraction": 0.5, "sunlitFraction": 0.1},
        "dispenser-housing": {"exposedFraction": 0.55, "sunlitFraction": 0.3},
        "dispenser-lid": {"exposedFraction": 1, "sunlitFraction": 0.9},
        "bag-roll": {"exposedFraction": 0.15, "sunlitFraction": 0.0},
        "bin-body": {"exposedFraction": 0.6, "sunlitFraction": 0.3},
        "bin-hood": {"exposedFraction": 1, "sunlitFraction": 0.95},
    }
    for k, v in over.items():
        base[k.replace("_", "-")] = v
    return {"regions": base, "emitters": {}}


CONTENTS = {"id": "bin-contents", "kind": "live", "title": T("Filled bags in the half-full bin (3.2 kg)", "Gefüllte Beutel im halbvollen Behälter (3,2 kg)"), "anchor": "bin-floor", "fxN": 0, "fyN": -31.4, "source": "authored-estimate", "reference": "dog-bag-typical"}
PULL = {"id": "hand-pull", "kind": "imposed", "title": T("Hand pulling a bag off the roll (15 N)", "Hand zieht einen Beutel von der Rolle (15 N)"), "anchor": "tab-grip", "fxN": 2.0, "fyN": -15.0, "source": "authored-estimate", "reference": "dog-bag-typical"}
PUSH = {"id": "bag-push", "kind": "imposed", "title": T("Filled bag pushed through the flap (10 N)", "Gefüllter Beutel durch die Klappe gedrückt (10 N)"), "anchor": "flap-push", "fxN": -10.0, "fyN": -2.0, "source": "authored-estimate", "reference": "dog-bag-typical"}
LIFT = {"id": "roll-lift", "kind": "carried", "title": T("Worker's hand carrying the roll (its own weight)", "Hand des Mitarbeiters trägt die Rolle (ihr Eigengewicht)"), "anchor": "roll-hand", "fxN": 0, "fyN": round(MASS["bag-roll"][0] * 9.80665, 2), "source": "authored-estimate", "reference": "dog-bag-typical"}
QP = 650
WIND = [
    {"id": "wind-sign", "kind": "wind", "title": T("Gust on the sign plate (cf 1.8)", "Böe auf die Schildplatte (cf 1,8)"), "anchor": "sign-centre", "fxN": -round(QP * 1.8 * 0.0748, 1), "fyN": 0, "source": "literature", "reference": "din-en-1991-1-4"},
    {"id": "wind-dispenser", "kind": "wind", "title": T("Gust on the dispenser (cf 1.5)", "Böe auf den Spender (cf 1,5)"), "anchor": "dispenser-centre", "fxN": -round(QP * 1.5 * 0.247 * 0.3015, 1), "fyN": 0, "source": "literature", "reference": "din-en-1991-1-4"},
    {"id": "wind-bin", "kind": "wind", "title": T("Gust on the bin (cf 1.5)", "Böe auf den Behälter (cf 1,5)"), "anchor": "bin-centre", "fxN": -round(QP * 1.5 * 0.317 * 0.4615, 1), "fyN": 0, "source": "literature", "reference": "din-en-1991-1-4"},
    {"id": "wind-post", "kind": "wind", "title": T("Gust on the post (cf 1.5)", "Böe auf den Pfosten (cf 1,5)"), "distributed": {"bone": "post", "fromM": 0.52, "toM": 2.43, "wxNPerM": -round(QP * 1.5 * 0.06, 1), "wyNPerM": 0}, "fxN": 0, "fyN": 0, "source": "literature", "reference": "din-en-1991-1-4"},
]


def pose(pid, title, desc, bones, th, loads, key=False):
    """🧍 A pose standing on the footing base."""
    out = {"id": pid, "title": title, "description": desc}
    if key:
        out["key"] = True
    out.update({"bones": bones, "ground": "authored", "contacts": ["footing-base"], "thermal": th, "loads": loads})
    return out


LIDS_OPEN = {"dispenser-lid": {"angleDeg": 85}, "bin-hood": {"angleDeg": 45}}
POSES = [
    pose("in-service", T("In service", "Im Einsatz"), T("Standing on its footing in the lawn on a summer afternoon: lid and hood shut, a half-empty roll in the dispenser with the next bag tab hanging from the slot, the bin half full, a filled bag left on the footing head.", "Auf seinem Fundament im Rasen an einem Sommernachmittag: Deckel und Haube zu, eine halb leere Rolle im Spender, der nächste Beutelzipfel hängt aus dem Schlitz, der Behälter halb voll, ein gefüllter Beutel auf dem Fundamentkopf abgelegt."), {}, binding(), [CONTENTS]),
    pose("maintenance", T("Restocking and emptying", "Nachfüllen und Leeren"), T("The depot worker has unlocked the dispenser lid and the rain hood with the triangle key, opened the lid by 85 degrees and set the hood at 45 degrees on its stay, 20 mm below the dispenser, pulled the bin bag out forwards and lifts the old roll out of the dispenser by 0.30 m to put in a new one.", "Der Bauhofmitarbeiter hat Spenderdeckel und Regenhaube mit dem Dreikantschlüssel aufgeschlossen, den Deckel um 85 Grad geöffnet und die Haube 45 Grad auf ihren Aufsteller gestellt, 20 mm unter dem Spender, den Behältersack nach vorn herausgezogen und hebt die alte Rolle 0,30 m aus dem Spender, um eine neue einzusetzen."), dict(LIDS_OPEN, **{"bag-roll": {"dx": 0.30}}), binding(dispenser_lid={"exposedFraction": 1, "sunlitFraction": 0.6}, bin_hood={"exposedFraction": 1, "sunlitFraction": 0.6}, bag_roll={"exposedFraction": 1, "sunlitFraction": 0.5}, bin_body={"exposedFraction": 0.95, "sunlitFraction": 0.45}, dispenser_housing={"exposedFraction": 0.9, "sunlitFraction": 0.35}), [LIFT]),
    pose("in-use", T("Taking a bag", "Beutel entnehmen"), T("A dog walker pulls the bag tab 0.15 m down out of the slot with about 15 N until the perforation tears; the bin stays half full.", "Eine Hundehalterin zieht den Beutelzipfel mit etwa 15 N 0,15 m aus dem Schlitz, bis die Perforation reißt; der Behälter bleibt halb voll."), {"bag-tab": {"dy": -0.15}}, binding(), [CONTENTS, PULL]),
    pose("wind-gust", T("Gust from the front", "Böe von vorn"), T("Closed and half full in a design gust of 0.65 kN/m2 from the front on sign, dispenser, bin and post; the footing carries the overturning moment into the soil.", "Geschlossen und halb voll in einer Bemessungsböe von 0,65 kN/m² von vorn auf Schild, Spender, Behälter und Pfosten; das Fundament leitet das Kippmoment in den Boden."), {}, binding(), [CONTENTS] + WIND),
    pose("lids-open", T("Lid and hood open", "Deckel und Haube offen"), T("Key pose: lid and hood open, the roll still on its axle.", "Schlüsselpose: Deckel und Haube offen, die Rolle noch auf ihrer Achse."), dict(LIDS_OPEN), binding(dispenser_lid={"exposedFraction": 1, "sunlitFraction": 0.6}, bin_hood={"exposedFraction": 1, "sunlitFraction": 0.6}, bin_body={"exposedFraction": 0.95, "sunlitFraction": 0.45}, dispenser_housing={"exposedFraction": 0.9, "sunlitFraction": 0.35}), [CONTENTS], key=True),
    pose("bag-dropped", T("Bag pushed in", "Beutel eingeworfen"), T("Key pose: a filled bag pushes the throw-in flap 55 degrees inwards.", "Schlüsselpose: ein gefüllter Beutel drückt die Einwurfklappe um 55 Grad nach innen."), {"throw-flap": {"angleDeg": -55}}, binding(), [CONTENTS, PUSH], key=True),
]

SPAN = lambda dur: [{"anchor": "footing-base", "fromS": 0, "toS": dur}]
CLIPS = [
    {"id": "restocking", "title": T("Restocking and emptying", "Nachfüllen und Leeren"), "description": T("The depot worker opens lid and hood, lifts the old roll out, sets the new one on the axle and locks both again.", "Der Bauhofmitarbeiter öffnet Deckel und Haube, hebt die alte Rolle heraus, setzt die neue auf die Achse und schließt beides wieder ab."),
     "durationS": 7.0, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 1.5, "pose": "lids-open", "ease": "smooth"}, {"t": 3.5, "pose": "maintenance", "ease": "smooth"}, {"t": 5.5, "pose": "lids-open", "ease": "smooth"}, {"t": 7.0, "pose": "in-service", "ease": "smooth"}],
     "contacts": SPAN(7.0), "phases": [{"id": "open", "fromS": 0, "toS": 1.5, "title": T("Unlocking and opening", "Aufschließen und Öffnen")}, {"id": "change-roll", "fromS": 1.5, "toS": 5.5, "title": T("Emptying and changing the roll", "Leeren und Rolle wechseln")}, {"id": "close", "fromS": 5.5, "toS": 7.0, "title": T("Closing and locking", "Schließen und Abschließen")}],
     "events": [{"t": 1.5, "kind": "motion", "title": T("Lid and hood open", "Deckel und Haube offen")}, {"t": 2.5, "kind": "load-off", "title": T("Bin emptied", "Behälter geleert"), "target": "bin-contents"}, {"t": 3.5, "kind": "motion", "title": T("Old roll lifted out", "Alte Rolle herausgehoben")}, {"t": 7.0, "kind": "motion", "title": T("Locked again", "Wieder abgeschlossen")}],
     "tracks": [{"target": "load:bin-contents", "ease": "smooth", "keys": [{"t": 0, "v": 1}, {"t": 1.5, "v": 1}, {"t": 2.5, "v": 0}, {"t": 7.0, "v": 0}]}, {"target": "load:roll-lift", "ease": "smooth", "keys": [{"t": 1.5, "v": 0}, {"t": 3.5, "v": 1}, {"t": 5.5, "v": 0}]}]},
    {"id": "bag-pull-and-drop", "title": T("Taking a bag and dropping it in", "Beutel nehmen und einwerfen"), "description": T("A dog walker pulls a bag off the roll, the tab springs back to the slot, and a little later the filled bag pushes the throw-in flap inwards, which swings shut again.", "Eine Hundehalterin zieht einen Beutel von der Rolle, der Zipfel schnellt in den Schlitz zurück, und wenig später drückt der gefüllte Beutel die Einwurfklappe nach innen, die wieder zufällt."),
     "durationS": 5.0, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 1.2, "pose": "in-use", "ease": "out"}, {"t": 1.45, "pose": "in-service", "ease": "smooth"}, {"t": 2.4, "pose": "in-service", "ease": "smooth"}, {"t": 3.1, "pose": "bag-dropped", "ease": "out"}, {"t": 4.0, "pose": "in-service", "ease": "smooth"}, {"t": 5.0, "pose": "in-service", "ease": "smooth"}],
     "contacts": SPAN(5.0), "phases": [{"id": "pull", "fromS": 0, "toS": 1.45, "title": T("Pulling a bag", "Beutel ziehen")}, {"id": "walk", "fromS": 1.45, "toS": 2.4, "title": T("Bag used and knotted", "Beutel benutzt und verknotet")}, {"id": "drop", "fromS": 2.4, "toS": 5.0, "title": T("Dropping it in", "Einwerfen")}],
     "events": [{"t": 1.2, "kind": "load-off", "title": T("Perforation tears", "Perforation reißt"), "target": "hand-pull"}, {"t": 3.1, "kind": "load-off", "title": T("Bag falls in", "Beutel fällt hinein"), "target": "bag-push"}],
     "tracks": []},
]

THERMAL = {
    "environment": {"airTempC": 26, "meanRadiantTempC": 30, "skyTempC": 12, "groundTempC": 34, "relativeHumidity": 0.5, "windMS": 1.5, "windDirectionDeg": 180, "solarWM2": 800, "diffuseFraction": 0.15, "pressurePa": 101325},
    "referenceEnvironment": "summer-hot-sun",
    "passengers": [], "operating": [], "regions": regions(), "emitters": [],
    "assumptions": [
        T("Passive surfaces in the summer afternoon sun: absorbed solar radiation, long-wave exchange with sky and lawn, convection and heat capacity; nothing in the station draws power (0 W, stated on the housing).", "Passive Flächen in der Sommernachmittagssonne: absorbierte Solarstrahlung, Langwellenaustausch mit Himmel und Rasen, Konvektion und Wärmekapazität; nichts an der Station nimmt Leistung auf (0 W, am Gehäuse angegeben)."),
        T("Weathered surfaces differ from new ones: the galvanised bin and hood have a dull zinc patina with white rust (emissivity 0.28 instead of 0.13 new, Incropera Table A.11), the moss-green powder coat is chalked (solar absorptance 0.86), the sign film has faded to a light grey (0.40 instead of about 0.25 for a new white film, ASHRAE), the corroded post foot is 0.85 / 0.80.", "Verwitterte Flächen unterscheiden sich von neuen: Behälter und Haube haben eine matte Zinkpatina mit Weißrost (Emissionsgrad 0,28 statt 0,13 neu, Incropera Tab. A.11), die moosgrüne Pulverbeschichtung ist ausgekreidet (solarer Absorptionsgrad 0,86), die Schildfolie ist hellgrau verblasst (0,40 statt etwa 0,25 neu, ASHRAE), der korrodierte Pfostenfuß hat 0,85 / 0,80."),
        T("The dark dispenser is the hot part in summer: thin 1.5 mm sheet with a solar absorptance of 0.86 heats within minutes, and the wound bag film inside follows it slowly. The bin and hood are inside and outside air-washed through the throw-in opening, the dispenser through its bag slot, so both faces of their sheet count.", "Der dunkle Spender ist im Sommer das heiße Teil: dünnes 1,5-mm-Blech mit einem solaren Absorptionsgrad von 0,86 erwärmt sich in Minuten, die gewickelte Beutelfolie im Inneren folgt langsam. Behälter und Haube werden durch die Einwurföffnung innen und außen von Luft umspült, der Spender durch den Beutelschlitz, daher zählen beide Seiten ihrer Bleche."),
        T("The footing lies 0.60 m deep in the soil: 88 % of its surface is in contact with soil at about 16 degC; only its 20 mm head is in the air. Exposed and sunlit shares are scenario data of the poses.", "Das Fundament liegt 0,60 m tief im Boden: 88 % seiner Fläche haben Kontakt mit Erdreich von etwa 16 °C; nur sein 20-mm-Kopf steht in der Luft. Freie und besonnte Anteile sind Szenariodaten der Posen."),
    ],
}

STRUCTURE = {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": ["footing-base"], "assumptions": [
    T("The post is a cantilever set 0.52 m into the unreinforced C20/25 point foundation; the soil restraint of the 0.30 x 0.30 x 0.62 m block is idealised as a fixed support at its base. Sign, dispenser and bin are rigid bodies clamped to the post by band clamps and mounting plates.", "Der Pfosten ist ein Kragarm, 0,52 m tief in das unbewehrte Punktfundament C20/25 eingesetzt; die Bodeneinspannung des Blocks 0,30 x 0,30 x 0,62 m ist als feste Einspannung an seiner Sohle idealisiert. Schild, Spender und Behälter sind starre Körper, mit Schellen und Montageplatten am Pfosten befestigt."),
    T("Masses from the modelled sections and densities (concrete 2400, steel 7850, aluminium 2700 kg/m3, DIN EN 1991-1-1); the half-empty roll and the left bag are authored estimates. The bin contents (3.2 kg of filled bags) are a live load, not a mass.", "Massen aus den modellierten Querschnitten und Dichten (Beton 2400, Stahl 7850, Aluminium 2700 kg/m³, DIN EN 1991-1-1); die halb leere Rolle und der abgelegte Beutel sind Schätzwerte. Der Behälterinhalt (3,2 kg gefüllte Beutel) ist eine Nutzlast, keine Masse."),
    T("Design gust: peak velocity pressure 0.65 kN/m2 (wind zone 2 inland, DIN EN 1991-1-4/NA), force coefficients 1.8 for the sign and 1.5 for dispenser, bin and post; the bending moment at the footing head is about 0.45 kNm, 38 MPa in the SHS 60 x 60 x 3.", "Bemessungsböe: Böengeschwindigkeitsdruck 0,65 kN/m² (Windzone 2 Binnenland, DIN EN 1991-1-4/NA), Kraftbeiwerte 1,8 für das Schild und 1,5 für Spender, Behälter und Pfosten; das Biegemoment am Fundamentkopf beträgt etwa 0,45 kNm, 38 MPa im Quadratrohr 60 x 60 x 3."),
    T("Hand loads (15 N pull on the bag tab, 10 N push on the flap, the worker carrying the roll) are authored estimates; lids and flap carry only their own weight on their hinges.", "Handlasten (15 N Zug am Beutelzipfel, 10 N Druck auf die Klappe, der Mitarbeiter trägt die Rolle) sind Schätzwerte; Deckel und Klappe tragen an ihren Scharnieren nur ihr Eigengewicht."),
], "clinical": False}

exec(open(os.path.join(HERE, "author-texts.py"), encoding="utf8").read())

PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, 500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
HATCHES = {50: [{"id": "concrete-stipple", "title": T("Concrete stipple", "Betonpunktierung"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.13}], 100: [{"id": "concrete-stipple", "title": T("Concrete stipple", "Betonpunktierung"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.13}], 200: [], 500: []}

reps, vols = {}, {}
for d in (500, 200, 100, 50):
    sc = SC[d]
    reps[str(d)] = {"status": "authored", "denominator": d, "validRange": {"minDenominator": RANGES[d][0], "maxDenominator": RANGES[d][1]}, "pens": PENS[d], "hatches": HATCHES[d], "geometry": sc.finish(), "decisions": REP_DECISIONS[d]}
    vols[str(d)] = {"status": "authored", "denominator": d, "body": sc.body, "thermal": sc.thermal, "structure": sc.structure, "decisions": VOL_DECISIONS[d]}

for d in (50, 100, 200, 500):
    sc = SC[d]
    ms = {}
    for bid in ("footing", "dispenser", "bin"):
        m, cu, cv, cw = MASS[bid]
        ox, oy, t = BONES[bid]
        ms[bid] = (r6(ox - cv), r6(oy + cu), r6(cw))
    S.support_and_masses(sc, ms)

asset = {
    "id": "used-dog-waste-station", "category": "objects",
    "title": T("Used Dog Waste Bag Station", "Gebrauchte Hundekotbeutelstation"),
    "summary": SUMMARY,
    "tags": ["used", "park", "public-space", "waste", "dog-waste", "bag-dispenser", "litter-bin", "signage", "galvanised", "powder-coated", "steel", "street-furniture"],
    "provenance": {"author": "Semio staffage content (used park objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": NOTES},
    "dimensions": {"heightM": 2.54, "widthM": 0.4285, "depthM": 0.317, "massKg": MASS_KG},
    "materials": MATERIALS, "rig": RIG, "anchors": ANCHORS,
    "representations": reps, "symbols": [], "poses": POSES, "clips": CLIPS,
    "thermal": THERMAL, "structure": STRUCTURE, "volumes": vols,
    "presentation": {"defaultPose": "in-service", "defaultClip": "bag-pull-and-drop", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}
order = ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure", "volumes", "presentation", "exports", "completion"]
doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": {k: asset[k] for k in order}}
with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf8") as f:
    json.dump(doc, f, ensure_ascii=False, indent=1)
    f.write("\n")
print(f"[DEBUG] mass {MASS_KG} kg; bones " + ", ".join(f"{k} {v[0]:.3f}" for k, v in MASS.items()))
for d in (50, 100, 200, 500):
    print(f"[DEBUG] 1:{d} body {len(SC[d].body)} thermal {len(SC[d].thermal)} structure {len(SC[d].structure)} shapes " + "/".join(str(len(v)) for v in reps[str(d)]["geometry"].values()))


if __name__ == "__main__":
    # the silhouette repair of batch SO, the last step: ops/used-dog-waste-station.json of the umbrella ticket of 2026-11-22
    import sys as _sys
    from pathlib import Path as _Path
    _sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/🧰️silhouette/SO")
    from lib import repair_file as _repair_file
    _repair_file(_Path(__file__).resolve().parent / "🔣️.json", "used-dog-waste-station")
