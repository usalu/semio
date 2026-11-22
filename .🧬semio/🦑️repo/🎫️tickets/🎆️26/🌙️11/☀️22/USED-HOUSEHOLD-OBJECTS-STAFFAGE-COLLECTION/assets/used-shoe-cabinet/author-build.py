"""🏗️ The authoritative build of the used hallway shoe cabinet: writes 🔣️.json next to this script from author_kit (numbers) and author_parts (solids and 2D).

Run: python3 author-build.py [--report]   (--report prints the masses, areas and tipping numbers the report quotes)"""
import json
import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).parent))
from author_kit import *
from author_parts import scale_50, scale_100, scale_200, scale_500, flap_bone, ELEV, FRONT, PLAN
from author_texts import TEXTS, decisions_rep, decisions_vol, REFERENCES, NOTES

HERE = pathlib.Path(__file__).parent


def t(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


MATERIALS = [
    {"id": "chipboard-foil", "title": t("Chipboard P2, 16 mm, white decor foil (aged, yellowed and scuffed)", "Spanplatte P2, 16 mm, weiße Dekorfolie (gealtert, vergilbt und verkratzt)"), "emissivity": 0.9, "solarAbsorptivity": 0.35, "densityKgM3": RHO["chipboard-foil"], "conductivityWmK": 0.13, "specificHeatJKgK": 1700, "elasticModulusPa": 1.6e9, "strengthPa": 11e6, "source": "literature", "reference": "en-312-2010"},
    {"id": "hdf-back", "title": t("HDF back panel 3 mm, raw brown side outwards", "HDF-Rückwand 3 mm, rohe braune Seite außen"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": RHO["hdf-back"], "conductivityWmK": 0.17, "specificHeatJKgK": 1700, "elasticModulusPa": 3.0e9, "strengthPa": 30e6, "source": "literature", "reference": "en-622-5-2009"},
    {"id": "aluminium-handle", "title": t("Anodised aluminium bar handle (original)", "Eloxierter Aluminium-Stangengriff (original)"), "emissivity": 0.77, "solarAbsorptivity": 0.4, "densityKgM3": RHO["aluminium-handle"], "conductivityWmK": 160, "specificHeatJKgK": 900, "elasticModulusPa": 70e9, "strengthPa": 160e6, "source": "literature", "reference": "incropera-2018"},
    {"id": "brushed-steel", "title": t("Brushed steel bar handle (replacement, longer)", "Gebürsteter Stahl-Stangengriff (Ersatz, länger)"), "emissivity": 0.3, "solarAbsorptivity": 0.5, "densityKgM3": RHO["brushed-steel"], "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "incropera-2018"},
    {"id": "zinc-plated-steel", "title": t("Zinc-plated steel angle bracket of the wall fixing, 2 mm", "Verzinkter Stahlwinkel der Wandbefestigung, 2 mm"), "emissivity": 0.25, "solarAbsorptivity": 0.5, "densityKgM3": RHO["zinc-plated-steel"], "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "polyester-nonwoven", "title": t("Moving blanket, polyester nonwoven (folded, five layers)", "Umzugsdecke, Polyestervlies (gefaltet, fünf Lagen)"), "emissivity": 0.9, "solarAbsorptivity": 0.75, "densityKgM3": RHO["polyester-nonwoven"], "conductivityWmK": 0.045, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "iso-10456-2007"},
]


def solid_volume(s):
    """📦 Volume and bone-local centre of a 1:50 body solid (extrudes only; handles and boards are all extrudes)."""
    if s["kind"] != "extrude":
        raise ValueError(s["id"])
    a = area(s["profile"])
    for h in s.get("holes", []):
        a -= area(h)
    depth = s["to"] - s["from"]
    c = centroid(s["profile"])
    if s["plane"] == "xy":
        return a * depth, (c[0], c[1])
    if s["plane"] == "zx":
        return a * depth, (c[1], (s["from"] + s["to"]) / 2)
    return a * depth, ((s["from"] + s["to"]) / 2, c[0])


def masses(body):
    """⚖️ Mass and centre of every bone from the 1:50 solids (section x density); the plastic tip-out fittings add 0.10 kg at each pivot."""
    out = {}
    for s in body:
        if s["bone"] in ("blanket",):
            continue
        vol, (cu, cv) = solid_volume(s)
        m = vol * RHO[s["material"]]
        e = out.setdefault(s["bone"], [0.0, 0.0, 0.0])
        e[0] += m
        e[1] += m * cu
        e[2] += m * cv
    for i in range(3):
        e = out[flap_bone(i)]
        e[0] += 0.10
        e[1] += 0.10 * 0.02
        e[2] += 0.10 * 0.03
    return {b: (m, mu / m, mv / m) for b, (m, mu, mv) in out.items()}


def rig(mass):
    """🦴 Floor, blanket (slides out), lift (raises the cabinet), carcass (tilts about its back-bottom edge) and the three flaps (tip out about their front-bottom edges)."""
    def mass_of(b, src="authored-estimate", ref="en-312-2010"):
        m, cu, cv = mass[b]
        return {"kg": round(m, 3), "comX": round(cu, 4), "comY": round(cv, 4), "distribution": "point", "source": src, "reference": ref}
    i_flap = 2 * FLAP_HW * T ** 3 / 12
    bones = [
        {"id": "floor", "parent": None, "title": t("Floor of the depot or the hallway", "Boden des Lagers oder des Flurs"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "blanket", "parent": "floor", "title": t("Folded moving blanket (stays behind, slides out)", "Gefaltete Umzugsdecke (bleibt liegen, wird herausgezogen)"), "x": BLANKET_X0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.30}},
        {"id": "lift", "parent": "floor", "title": t("Lift of the cabinet off the blanket", "Anheben des Schranks von der Decke"), "x": 0, "y": BLANKET_T, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -BLANKET_T, "maxM": 0.70}},
        {"id": "carcass", "parent": "lift", "title": t("Carcass with back panel, tilting about its back-bottom edge", "Korpus mit Rückwand, kippt um die hintere Unterkante"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": D, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 90}, "mass": mass_of("carcass")},
    ]
    for i in range(3):
        bones.append({"id": flap_bone(i), "parent": "carcass", "title": t(f"Tip-out flap {i + 1} with its shoe tray (from below)", f"Kippklappe {i + 1} mit Schuhfach (von unten)"), "x": D, "y": FLAP_BOTTOMS[i], "angleDeg": 90, "lengthM": FLAP_H, "joint": {"type": "revolute", "minDeg": -42, "maxDeg": 0}, "mass": mass_of(flap_bone(i)), "section": {"areaM2": round(2 * FLAP_HW * T, 6), "secondMomentM4": float(f"{i_flap:.4e}"), "extremeFibreM": T / 2, "material": "chipboard-foil", "title": t("Flap panel 796 x 16 mm", "Klappenplatte 796 x 16 mm")}})
    return {"kind": "mechanical", "root": "floor", "bones": bones}


def anchors():
    """📍 Floor contacts of blanket and side panels, the four hands of the two carriers, the remaining wall bracket and the shoe loads."""
    out = []
    for side, z in (("r", -0.30), ("l", 0.30)):
        out.append({"id": f"blanket-floor-{side}", "bone": "blanket", "x": BLANKET_L / 2, "y": 0.0, "z": z, "kind": "contact", "title": t(f"Blanket on the floor, {'right' if side == 'r' else 'left'} half", f"Decke auf dem Boden, {'rechte' if side == 'r' else 'linke'} Hälfte"), "support": "contact", "normalDeg": 90, "footprintLengthM": BLANKET_L, "footprintWidthM": 0.45, "frictionCoefficient": 0.5})
    for side, z in (("r", -(HW - T / 2)), ("l", HW - T / 2)):
        out.append({"id": f"foot-{side}", "bone": "carcass", "x": round((TB + CARCASS_FRONT) / 2, 4), "y": 0.0, "z": z, "kind": "contact", "title": t(f"{'Right' if side == 'r' else 'Left'} side panel on the floor", f"{'Rechte' if side == 'r' else 'Linke'} Seitenwand auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": round(CARCASS_FRONT - TB, 3), "footprintWidthM": T, "frictionCoefficient": 0.5})
    for end, v in (("bottom", 0.05), ("top", 1.40)):
        for side, z in (("r", -(HW - T / 2)), ("l", HW - T / 2)):
            out.append({"id": f"grip-{end}-{side}", "bone": "carcass", "x": 0.0, "y": v, "z": z, "kind": "support", "title": t(f"Hand of the {'lower' if end == 'bottom' else 'upper'} carrier under the {'right' if side == 'r' else 'left'} side panel", f"Hand des {'unteren' if end == 'bottom' else 'oberen'} Trägers unter der {'rechten' if side == 'r' else 'linken'} Seitenwand"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.09, "footprintWidthM": 0.016, "frictionCoefficient": 0.6})
    out.append({"id": "wall-fixing", "bone": "carcass", "x": 0.001, "y": round(TOP[1] + BRACKET_H - 0.010, 4), "z": round(sum(BRACKET_W) / 2, 4), "kind": "support", "title": t("Remaining right wall bracket screwed to the wall (horizontal restraint)", "Verbliebener rechter Wandwinkel an die Wand geschraubt (waagrechte Halterung)"), "support": "roller", "normalDeg": 0})
    for i in range(3):
        out.append({"id": f"shoes-{i + 1}", "bone": flap_bone(i), "x": 0.15, "y": 0.06, "kind": "load", "title": t(f"Four pairs of shoes in the tray of flap {i + 1}", f"Vier Paar Schuhe im Fach der Klappe {i + 1}")})
    return out


def shoe_loads(flaps=(0, 1, 2)):
    """👞 Four pairs of 0.8 kg per tray: 31.4 N at the pocket of each loaded flap."""
    f = round(-SHOE_PAIRS * SHOE_PAIR_KG * G, 2)
    return [{"id": f"shoes-{i + 1}", "kind": "imposed", "title": t(f"Shoes in flap {i + 1} (4 pairs, 3.2 kg)", f"Schuhe in Klappe {i + 1} (4 Paar, 3,2 kg)"), "anchor": f"shoes-{i + 1}", "fxN": 0.0, "fyN": f, "source": "authored-estimate", "reference": "semio-used-household"} for i in flaps]


def binding(state):
    """🌡️ Thermal binding of every region for a pose family."""
    reg = {
        "stored": {"carcass": (1.0, 0.1), "back-panel": (1.0, 0.0), "plinth": (1.0, 0.0), "flaps": (1.0, 0.15), "blanket": (0.45, 0.05, 0.5)},
        "carried": {"carcass": (1.0, 0.1), "back-panel": (1.0, 0.0), "plinth": (1.0, 0.0), "flaps": (1.0, 0.2), "blanket": (0.5, 0.05, 0.5)},
        "in-use": {"carcass": (0.95, 0.0), "back-panel": (0.1, 0.0, 0.85), "plinth": (1.0, 0.0), "flaps": (1.0, 0.05), "blanket": (0.5, 0.05, 0.5)},
    }[state]
    regions = {}
    for k, v in reg.items():
        b = {"exposedFraction": v[0], "sunlitFraction": v[1]}
        if len(v) == 3:
            b["contact"] = {"fraction": v[2], "tempC": 16 if k == "blanket" else 18}
        regions[k] = b
    return {"regions": regions, "emitters": {}}


CONTACTS = {
    "stored": ["blanket-floor-r", "blanket-floor-l"],
    "grips": ["grip-bottom-r", "grip-bottom-l", "grip-top-r", "grip-top-l"],
    "in-use": ["foot-r", "foot-l", "wall-fixing"],
}
CARRIED = {"lift": {"dy": round(0.70 - BLANKET_T, 4)}, "carcass": {"angleDeg": 85}}
IN_USE = {"lift": {"dy": -BLANKET_T}, "blanket": {"dx": 0.25}}


def poses():
    """🧍 stored (default), carried, in-use, flap-open, all-flaps-open and the two key poses of the carry clip."""
    x = TEXTS["poses"]
    def pose(pid, bones, contacts, state, loads=(), key=False):
        p = {"id": pid, "title": x[pid]["title"], "description": x[pid]["description"], "bones": bones, "ground": "authored", "contacts": contacts, "thermal": binding(state), "loads": list(loads)}
        if key:
            p["key"] = True
        return p
    open_one = dict(IN_USE, **{"flap-3": {"angleDeg": -OPEN_DEG}})
    open_all = dict(IN_USE, **{flap_bone(i): {"angleDeg": -OPEN_DEG} for i in range(3)})
    return [
        pose("stored", {}, CONTACTS["stored"], "stored"),
        pose("carried", CARRIED, CONTACTS["grips"], "carried"),
        pose("in-use", IN_USE, CONTACTS["in-use"], "in-use", shoe_loads()),
        pose("flap-open", open_one, CONTACTS["in-use"], "in-use", shoe_loads()),
        pose("all-flaps-open", open_all, CONTACTS["in-use"], "in-use", shoe_loads()),
        pose("carry-tilted-key", {"carcass": {"angleDeg": 45}}, CONTACTS["stored"] + ["grip-top-r", "grip-top-l"], "carried", key=True),
        pose("carry-lifting-key", {"lift": {"dy": 0.30}, "carcass": {"angleDeg": 70}}, CONTACTS["grips"], "carried", key=True),
    ]


def clips():
    """🎞️ carry (stored to carried: tilted back about the bottom edge, then lifted by two people) and tip-out-flap (the top flap opens to 40 degrees and falls shut)."""
    x = TEXTS["clips"]
    carry = {"id": "carry", "title": x["carry"]["title"], "description": x["carry"]["description"], "durationS": 3.6, "fps": 30, "loop": False, "rootMotion": "none",
             "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.4, "pose": "carry-tilted-key", "ease": "smooth"}, {"t": 2.4, "pose": "carry-lifting-key", "ease": "smooth"}, {"t": 3.6, "pose": "carried", "ease": "smooth"}],
             "contacts": [{"anchor": "blanket-floor-r", "fromS": 0, "toS": 1.6}, {"anchor": "blanket-floor-l", "fromS": 0, "toS": 1.6}, {"anchor": "grip-top-r", "fromS": 0.3, "toS": 3.6}, {"anchor": "grip-top-l", "fromS": 0.3, "toS": 3.6}, {"anchor": "grip-bottom-r", "fromS": 1.4, "toS": 3.6}, {"anchor": "grip-bottom-l", "fromS": 1.4, "toS": 3.6}],
             "phases": [{"id": "tilt", "fromS": 0, "toS": 1.4, "title": t("Tilting back on the bottom edge", "Nach hinten auf die Unterkante kippen")}, {"id": "lift", "fromS": 1.4, "toS": 3.6, "title": t("Lifting by two carriers", "Anheben durch zwei Träger")}],
             "events": [{"t": 0.3, "kind": "contact-on", "title": t("Upper carrier takes the top end", "Oberer Träger fasst das obere Ende"), "target": "grip-top-r"}, {"t": 1.4, "kind": "contact-on", "title": t("Lower carrier takes the bottom end", "Unterer Träger fasst das untere Ende"), "target": "grip-bottom-r"}, {"t": 1.6, "kind": "contact-off", "title": t("Cabinet leaves the blanket", "Schrank verlässt die Decke"), "target": "blanket-floor-r"}, {"t": 3.6, "kind": "motion", "title": t("Carried nearly flat, flaps up", "Fast flach getragen, Klappen oben")}],
             "tracks": []}
    tip = {"id": "tip-out-flap", "title": x["tip-out-flap"]["title"], "description": x["tip-out-flap"]["description"], "durationS": 3.0, "fps": 30, "loop": True, "rootMotion": "none",
           "keys": [{"t": 0, "pose": "in-use", "ease": "smooth"}, {"t": 0.8, "pose": "flap-open", "ease": "linear"}, {"t": 2.0, "pose": "flap-open", "ease": "in"}, {"t": 2.6, "pose": "in-use", "ease": "linear"}, {"t": 3.0, "pose": "in-use", "ease": "linear"}],
           "contacts": [{"anchor": a, "fromS": 0, "toS": 3.0} for a in CONTACTS["in-use"]],
           "phases": [{"id": "open", "fromS": 0, "toS": 0.8, "title": t("Pulled open at the handle", "Am Griff aufgezogen")}, {"id": "take-shoes", "fromS": 0.8, "toS": 2.0, "title": t("Shoes taken out at 40 degrees", "Schuhe bei 40 Grad entnommen")}, {"id": "close", "fromS": 2.0, "toS": 2.6, "title": t("Let go, the flap falls shut", "Losgelassen, die Klappe fällt zu")}, {"id": "rest", "fromS": 2.6, "toS": 3.0, "title": t("Closed", "Geschlossen")}],
           "events": [{"t": 0.8, "kind": "motion", "title": t("Stop of the tip-out fitting at 40 degrees", "Anschlag des Kippbeschlags bei 40 Grad"), "target": "flap-3"}, {"t": 2.6, "kind": "motion", "title": t("Flap knocks shut against the carcass", "Klappe schlägt gegen den Korpus zu"), "target": "flap-3"}],
           "tracks": []}
    return [carry, tip]


def region(rid, kind_title, material, area_m2, char, geom, svf, layer, extra=None):
    """🔥 A passive thermal region with its single board layer."""
    mat = next(m for m in MATERIALS if m["id"] == material)
    thick, title = layer
    res = round(thick / mat["conductivityWmK"], 5)
    cap = round(mat["densityKgM3"] * mat["specificHeatJKgK"] * thick, 1)
    out = {"id": rid, "title": kind_title, "kind": "surface", "model": "passive", "material": material, "areaM2": area_m2, "characteristicLengthM": char, "geometry": geom, "skyViewFactor": svf,
           "layers": [{"id": f"{rid}-layer", "title": title, "kind": "polymer" if material == "polyester-nonwoven" else "timber", "material": material, "thicknessM": thick, "resistanceM2KW": res, "heatCapacityJM2K": cap, "source": "literature" if material != "polyester-nonwoven" else "authored-estimate", "reference": "iso-10456-2007"}],
           "heatCapacityJM2K": cap, "source": "authored-estimate", "reference": "iso-10456-2007"}
    if extra:
        out.update(extra)
    return out


AREAS = {"carcass": 0.92, "back-panel": 1.22, "plinth": 0.119, "flaps": 1.28, "blanket": 1.13}


def thermal():
    """🌡️ Five passive regions; the reuse depot hall in autumn as the reference case."""
    regions = [
        region("carcass", t("Side panels, top, bottom and dividers (outer faces)", "Seitenwände, Deckel, Boden und Zwischenböden (Außenflächen)"), "chipboard-foil", AREAS["carcass"], 1.45, "vertical-plate", 0.4, (T, t("Chipboard 16 mm", "Spanplatte 16 mm")), {"ratedPowerW": 0, "ratedPowerBasis": "passive"}),
        region("back-panel", t("HDF back panel", "HDF-Rückwand"), "hdf-back", AREAS["back-panel"], 1.39, "vertical-plate", 0.3, (TB, t("HDF 3 mm", "HDF 3 mm"))),
        region("plinth", t("Kick board of the plinth", "Sockelblende"), "chipboard-foil", AREAS["plinth"], PLINTH_H, "vertical-plate", 0.2, (T, t("Chipboard 16 mm", "Spanplatte 16 mm"))),
        region("flaps", t("Three tip-out flap fronts", "Drei Kippklappenfronten"), "chipboard-foil", AREAS["flaps"], FLAP_H, "vertical-plate", 0.45, (T, t("Chipboard 16 mm", "Spanplatte 16 mm"))),
        region("blanket", t("Folded moving blanket", "Gefaltete Umzugsdecke"), "polyester-nonwoven", AREAS["blanket"], BLANKET_L, "horizontal-plate-up", 0.9, (BLANKET_T, t("Polyester nonwoven, five layers 15 mm", "Polyestervlies, fünf Lagen 15 mm"))),
    ]
    env = {"airTempC": 16, "meanRadiantTempC": 15, "relativeHumidity": 0.55, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 150, "diffuseFraction": 0.6, "pressurePa": 101325}
    return {"minimumAirPassageM": None, "environment": env, "operating": [], "regions": regions, "emitters": [], "assumptions": TEXTS["thermal-assumptions"], "passengers": []}


def tipping(mass):
    """📉 Stability about the front edge of the side panels (x = 0.154) with every flap opened 40 degrees and four pairs of shoes in each tray (EN 14749 / ISO 7171 case: extension elements open)."""
    edge = CARCASS_FRONT
    m_c, cu, cv = mass["carcass"]
    restoring = m_c * G * (edge - cu)
    over = 0.0
    for i in range(3):
        m, fu, fv = mass[flap_bone(i)]
        x_open, _ = rotate_open(i, fu, fv, OPEN_DEG)
        over += m * G * (x_open - edge)
        xs, _ = rotate_open(i, 0.15, 0.06, OPEN_DEG)
        over += SHOE_PAIRS * SHOE_PAIR_KG * G * (xs - edge)
    closed_over = 0.0
    for i in range(3):
        m, fu, fv = mass[flap_bone(i)]
        x_closed, _ = flap_world(i, fu, fv)
        closed_over += m * G * (x_closed - edge)
    wall = (over - restoring) / (TOP[1] + BRACKET_H - 0.010)
    return {"restoringNm": restoring, "overturningNm": over, "closedFlapsNm": closed_over, "wallForceN": wall}


def representation(scale, data):
    """🗺️ One authored representation."""
    pens = {50: {"body": 0.35, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.5}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, 500: {"body": 0.2, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}[scale]
    rng = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": rng[0], "maxDenominator": rng[1]}, "pens": pens, "hatches": [], "geometry": {ELEV: data["shapes"][ELEV], FRONT: data["shapes"][FRONT], PLAN: data["shapes"][PLAN]}, "decisions": decisions_rep(scale)}


def mass_markers(scale, mass, radius):
    """🎯 The concentrated mass of every bone at its centre (one marker for the whole cabinet at 1:500)."""
    if scale == 500:
        total = sum(mass[b][0] for b in mass)
        cu = sum(mass["carcass"][0] * mass["carcass"][1] for _ in [0])
        cv = mass["carcass"][0] * mass["carcass"][2]
        for i in range(3):
            m, fu, fv = mass[flap_bone(i)]
            x, y = flap_world(i, fu, fv)
            cu += m * x
            cv += m * y
        return [{"id": "mass-cabinet", "kind": "ellipsoid", "bone": "carcass", "material": "chipboard-foil", "role": "detail", "tone": 0.3, "radii": [radius] * 3, "transform": {"translation": [r(cu / total), r(cv / total), 0.0]}, "member": "carcass", "part": "mass"}]
    out = []
    for b, (m, cu, cv) in mass.items():
        out.append({"id": f"mass-{b}", "kind": "ellipsoid", "bone": b, "material": "chipboard-foil", "role": "detail", "tone": 0.3, "radii": [radius] * 3, "transform": {"translation": [r(cu), r(cv), 0.0]}, "member": b, "part": "mass"})
    return out


def symbol():
    """🔣 True-extent symbol: the cabinet on its blanket in the three views."""
    y0 = BLANKET_T
    sh = lambda sid, pts, filled=True, tone=0.85, role="body": {"id": sid, "bone": "floor", "role": role, "z": 1 if sid.startswith("blanket") else 2, "filled": filled, "tone": tone, "kind": "poly", "points": [[r(a), r(b)] for a, b in pts], "closed": True}
    elev = [sh("blanket", [(BLANKET_X0, 0), (BLANKET_X0 + BLANKET_L, 0), (BLANKET_X0 + BLANKET_L, y0), (BLANKET_X0, y0)], tone=0.35), sh("cabinet", [(0, y0), (D, y0), (D, y0 + H), (0, y0 + H)])]
    front = [sh("blanket", [(-BLANKET_HW, 0), (BLANKET_HW, 0), (BLANKET_HW, y0), (-BLANKET_HW, y0)], tone=0.35), sh("cabinet", [(-HW, y0), (HW, y0), (HW, y0 + H), (-HW, y0 + H)])]
    for i, b in enumerate(FLAP_BOTTOMS[1:]):
        front.append({"id": f"flap-joint-{i + 1}", "bone": "floor", "role": "detail", "z": 3, "filled": False, "tone": 0.5, "kind": "poly", "points": [[-FLAP_HW, r(y0 + b - FLAP_GAP / 2)], [FLAP_HW, r(y0 + b - FLAP_GAP / 2)]], "closed": False})
    plan = [sh("blanket", [(BLANKET_X0, -BLANKET_HW), (BLANKET_X0 + BLANKET_L, -BLANKET_HW), (BLANKET_X0 + BLANKET_L, BLANKET_HW), (BLANKET_X0, BLANKET_HW)], tone=0.35), sh("cabinet", [(0, -HW), (D, -HW), (D, HW), (0, HW)]),
            {"id": "flap-face", "bone": "floor", "role": "detail", "z": 3, "filled": False, "tone": 0.5, "kind": "poly", "points": [[CARCASS_FRONT, -HW], [CARCASS_FRONT, HW]], "closed": False}]
    x = TEXTS["symbol"]
    return {"id": "used-shoe-cabinet-symbol", "title": x["title"], "description": x["description"], "semantics": "true-extent", "units": "model-m", "denominators": [200, 100, 50], "pens": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, "hatches": [], "geometry": {ELEV: elev, FRONT: front, PLAN: plan}, "dimensionalNote": x["note"]}


def build():
    """📄 Assembles and writes the asset document."""
    data = {50: scale_50(), 100: scale_100(), 200: scale_200(), 500: scale_500()}
    mass = masses(data[50]["body"])
    radius = {50: 0.02, 100: 0.02, 200: 0.02, 500: 0.04}
    vols = {}
    for s, d in data.items():
        structure = d["structure"] + mass_markers(s, mass, radius[s])
        vols[str(s)] = {"status": "authored", "denominator": s, "body": d["body"], "thermal": d["thermal"], "structure": structure, "decisions": decisions_vol(s)}
    total = sum(m for m, _, _ in mass.values())
    th = thermal()
    th.pop("minimumAirPassageM")
    tip = tipping(mass)
    struct = {"gravityMS2": G, "dynamics": "quasi-static", "supports": [], "assumptions": TEXTS["structure-assumptions"](tip, mass), "clinical": False}
    asset = {
        "id": "used-shoe-cabinet", "category": "furniture", "title": TEXTS["title"], "summary": TEXTS["summary"],
        "tags": ["used", "household", "second-hand", "hallway", "shoe-cabinet", "storage", "tip-out", "chipboard", "flat-pack", "moving-blanket"],
        "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": NOTES},
        "dimensions": {"heightM": r(BLANKET_T + TOP[1] + BRACKET_H, 4), "widthM": BLANKET_L, "depthM": 2 * BLANKET_HW, "massKg": round(total, 2)},
        "materials": MATERIALS, "rig": rig(mass), "anchors": anchors(),
        "representations": {str(s): representation(s, data[s]) for s in (500, 200, 100, 50)},
        "symbols": [symbol()], "poses": poses(), "clips": clips(), "thermal": th, "structure": struct,
        "volumes": {k: vols[k] for k in ("50", "100", "200", "500")},
        "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n")
    if "--report" in sys.argv:
        print(json.dumps({"mass": {b: [round(v, 4) for v in m] for b, m in mass.items()}, "totalKg": round(total, 3), "tipping": {k: round(v, 2) for k, v in tip.items()}}, indent=1))


if __name__ == "__main__":
    build()
