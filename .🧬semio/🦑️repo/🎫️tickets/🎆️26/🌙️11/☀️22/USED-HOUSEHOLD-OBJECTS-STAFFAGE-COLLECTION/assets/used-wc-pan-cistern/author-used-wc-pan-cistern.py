"""🚽️ Authoritative chisel of the asset `used-wc-pan-cistern`: the used close-coupled floor-standing WC with its ceramic cistern and seat, outlet capped, on a half pallet in a reuse depot; writes `🔣️.json` next to this file.

Run: `python3 author-used-wc-pan-cistern.py` (writes `🔣️.json`, prints the masses and centres it derived).

@see ./author_geometry.py — every hand-chosen number
@see ./author_solids.py — the solids per scale, thermal and structure lists, masses
@see ./author_drawings.py — the 2D representations
@see ../../brief.md — the brief of the wave
"""
import json
import math
from pathlib import Path

from author_drawings import erect, rep50, rep100, rep200, rep500, sh, wpoly, wrect
from author_geometry import (BUTTON, CIS, CLID, FOOT, G, HINGE, LID_OPEN_DEG, ORIGIN, PAL_X, PAL_Z, PH, REAR, SEAT, SPIGOT, T, XB, loc, r6)
from author_solids import B50, B100, B200, MASS, ST50, ST100, ST200, TH50, TH100, TH200, TOTAL_KG, solid_volume

HERE = Path(__file__).resolve().parent
WC_KG = TOTAL_KG - MASS["pallet"]["kg"]
PALLET_KG = MASS["pallet"]["kg"]


def bone_mass(b, ref):
    """⚖️ The point mass of a bone from its 1:50 solids."""
    m = MASS[b]
    return {"kg": round(m["kg"], 5), "comX": round(m["comX"], 5), "comY": round(m["comY"], 5), "distribution": "point", "source": "authored-estimate", "reference": ref}


def section(area, i, c, material, en, de):
    """📏️ A member cross-section for stress."""
    return {"areaM2": r6(area), "secondMomentM4": float(f"{i:.4e}"), "extremeFibreM": r6(c), "material": material, "title": T(en, de)}


def bone(id, parent, en, de, x, y, length, joint, mass_ref=None, sec=None):
    """🦴️ One bone resting at angle 0 at (x, y) in its parent's frame."""
    b = {"id": id, "parent": parent, "title": T(en, de), "x": r6(x), "y": r6(y), "angleDeg": 0, "lengthM": r6(length), "joint": joint}
    if mass_ref:
        b["mass"] = bone_mass(id, mass_ref)
    if sec:
        b["section"] = sec
    return b


def rel(child, parent):
    """↘️ The rest offset of a bone origin from its parent's origin."""
    return ORIGIN[child][0] - ORIGIN[parent][0], ORIGIN[child][1] - ORIGIN[parent][1]


FIX = {"type": "fixed"}
BONES = [
    bone("ground", None, "Depot floor reference", "Bodenbezug im Lager", 0, 0, 0, FIX),
    bone("pallet", "ground", "Half pallet with the cardboard sheet (stays behind)", "Halbpalette mit Pappe (bleibt zurück)", *rel("pallet", "ground"), 2 * PAL_X, FIX, "pallet-type",
         section(0.0176, 1.47e-5, 0.0855, "spruce-pallet", "Deck and bottom boards 8 x 100 x 22 mm", "Deck- und Bodenbretter 8 x 100 x 22 mm")),
    bone("travel", "ground", "Carrying travel along the floor", "Tragweg über den Boden", 0, 0, 0, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 1.5}),
    bone("lift", "travel", "Lift by the two carriers", "Hub durch die beiden Träger", 0, 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": -0.16, "maxM": 0.3}),
    bone("pan", "lift", "WC pan (pivot at the back of the foot on the floor)", "WC-Becken (Drehpunkt hinten am Fuß auf dem Boden)", *ORIGIN["pan"], 0.44, {"type": "revolute", "minDeg": -10, "maxDeg": 10}, "en-997-2018",
         section(0.0038, 1.75e-5, 0.1175, "vitreous-china-pan", "Foot walls 2 x 8 mm, 235 mm high", "Fußwände 2 x 8 mm, 235 mm hoch")),
    bone("cistern", "pan", "Close-coupled cistern, bolted to the deck", "Aufgesetzter Spülkasten, auf der Auflage verschraubt", *rel("cistern", "pan"), CIS["x1"] - CIS["x0"], FIX, "en-14055-2018",
         section(0.0129, 2.74e-4, 0.1825, "vitreous-china-cistern", "Tank walls 9 mm, 160 x 370 mm", "Tankwände 9 mm, 160 x 370 mm")),
    bone("cistern-lid", "cistern", "Loose cistern lid", "Loser Spülkastendeckel", *rel("cistern-lid", "cistern"), CLID["x1"] - CLID["x0"], FIX, "en-14055-2018",
         section(0.0068, 1.84e-7, 0.009, "vitreous-china-cistern", "Lid plate 380 x 18 mm", "Deckelplatte 380 x 18 mm")),
    bone("button", "cistern-lid", "Dual-flush push button (replaced)", "Zwei-Mengen-Drücker (ersetzt)", *rel("button", "cistern-lid"), BUTTON["r"], {"type": "prismatic", "axisDeg": 90, "minM": -0.008, "maxM": 0}, "en-14055-2018"),
    bone("seat", "pan", "Seat ring on its hinge", "Sitzring an seinem Scharnier", *rel("seat", "pan"), SEAT["cx"] + SEAT["rx"] - HINGE[0], {"type": "revolute", "minDeg": 0, "maxDeg": 92}, "din-19516",
         section(0.0014, 2.3e-8, 0.005, "duroplast-seat", "Ring band 2 x 70 x 10 mm", "Ringband 2 x 70 x 10 mm")),
    bone("lid", "pan", "Seat lid on the same hinge", "Sitzdeckel am selben Scharnier", *rel("lid", "pan"), SEAT["cx"] + SEAT["rx"] - HINGE[0], {"type": "revolute", "minDeg": 0, "maxDeg": LID_OPEN_DEG}, "din-19516",
         section(0.0018, 3.7e-9, 0.0025, "duroplast-seat", "Lid shell 356 x 5 mm", "Deckelschale 356 x 5 mm")),
]


def anchor(id, bone_id, x, y, z, kind, en, de, **kw):
    """🪝️ One anchor from a world rest point."""
    u, v = loc(bone_id, x, y)
    a = {"id": id, "bone": bone_id, "x": u, "y": v, "z": r6(z), "kind": kind, "title": T(en, de)}
    a.update(kw)
    return a


FEET = []
ANCHORS = []
for name, x, en_p, de_p in (("front", 0.15, "Front", "Vorderer"), ("rear", -0.17, "Rear", "Hinterer")):
    for side, z, en_s, de_s in (("l", 0.07, "left", "linker"), ("r", -0.07, "right", "rechter")):
        ANCHORS.append(anchor(f"foot-{name}-{side}", "pan", x, PH, z, "contact", f"{en_p} {en_s} edge of the foot", f"{de_p} {de_s} Rand des Fußes", support="contact", normalDeg=90, footprintLengthM=0.06, footprintWidthM=0.03, frictionCoefficient=0.5))
        FEET.append(f"foot-{name}-{side}")
PADS = []
for name, x in (("rear", -0.35), ("front", 0.35)):
    for side, z in (("l", 0.25), ("r", -0.25)):
        ANCHORS.append(anchor(f"pallet-{name}-{side}", "pallet", x, 0.0, z, "contact", f"Pallet {name} {'left' if side == 'l' else 'right'} block on the floor", f"Palette, {'hinterer' if name == 'rear' else 'vorderer'} {'linker' if side == 'l' else 'rechter'} Klotz auf dem Boden", support="contact", normalDeg=90, footprintLengthM=0.1, footprintWidthM=0.1, frictionCoefficient=0.5))
        PADS.append(f"pallet-{name}-{side}")
ANCHORS += [
    anchor("grip-front", "pan", XB + 0.24, PH + 0.365, 0.0, "support", "Hands of the front carrier under the rim", "Hände des vorderen Trägers unter dem Rand", support="pin"),
    anchor("grip-rear", "pan", -0.28, PH + REAR["y0"], 0.0, "support", "Hands of the rear carrier under the rear body", "Hände des hinteren Trägers unter dem Hinterkörper", support="pin"),
    anchor("occupant", "seat", SEAT["hx"], PH + SEAT["y1"], 0.0, "load", "Seated person on the ring", "Sitzende Person auf dem Ring"),
    anchor("cistern-water", "cistern", -0.255, PH + 0.55, 0.0, "load", "Water in the cistern", "Wasser im Spülkasten"),
    anchor("press", "button", BUTTON["x"], PH + BUTTON["y1"], 0.0, "load", "Finger on the push button", "Finger auf dem Drücker"),
]

FLOOR_UNDER_PALLET = {"id": "floor-under-pallet", "kind": "imposed", "title": T("Floor reaction under the pallet, which lies on its own", "Bodenreaktion unter der Palette, die für sich liegt"), "bone": "pallet", "x": round(MASS["pallet"]["comX"], 5), "y": round(MASS["pallet"]["comY"], 5), "fxN": 0, "fyN": round(PALLET_KG * G, 3), "source": "literature", "reference": "en-1991-1-1"}
WATER = {"id": "water-load", "kind": "equipment", "title": T("6 l of water in the cistern", "6 l Wasser im Spülkasten"), "anchor": "cistern-water", "fxN": 0, "fyN": round(-6.0 * G, 3), "source": "literature", "reference": "en-14055-2018"}
OCCUPANT = {"id": "occupant-load", "kind": "live", "title": T("Seated person, 85 kg, 0.65 kN on the ring", "Sitzende Person, 85 kg, 0,65 kN auf dem Ring"), "anchor": "occupant", "fxN": 0, "fyN": -650.0, "source": "authored-estimate", "reference": "din-33402-2-2020"}
PRESS = {"id": "press-load", "kind": "live", "title": T("Finger pressing the button, 25 N", "Finger drückt den Drücker, 25 N"), "anchor": "press", "fxN": 0, "fyN": -25.0, "source": "authored-estimate", "reference": "en-14055-2018"}

LIFT, TILT, TRAVEL = 0.21, 4.0, 0.5
IN_USE = {"travel": {"dx": 1.1}, "lift": {"dy": -PH}}


def binding(lid_sun=0.0, water=False):
    """🌬️ The thermal binding of every region of a pose."""
    r = {k: {"exposedFraction": 1, "sunlitFraction": 0} for k in ("pan", "cistern", "seat-set", "outlet-cap", "pallet", "cardboard")}
    r["seat-set"]["sunlitFraction"] = lid_sun
    r["cardboard"] = {"exposedFraction": 0.6, "sunlitFraction": 0, "contact": {"fraction": 0.4, "tempC": 15}}
    if water:
        r["cistern"] = {"exposedFraction": 1, "sunlitFraction": 0, "contact": {"fraction": 0.5, "tempC": 12}}
    return {"regions": r, "emitters": {}}


def pose(id, en, de, den, dde, bones, contacts, loads, thermal, key=False):
    """🧍️ One pose."""
    p = {"id": id}
    if key:
        p["key"] = True
    p.update({"title": T(en, de), "description": T(den, dde), "bones": bones, "ground": "authored", "contacts": contacts, "thermal": thermal, "loads": loads})
    return p


POSES = [
    pose("stored", "Stored on the pallet", "Auf der Palette gelagert",
         "Emptied, cleaned and drained in the reuse depot: the WC stands on a sheet of cardboard on a half pallet, seat and lid down, the cistern lid taped to the tank, the outlet capped, the water inlet sealed.",
         "Im Wiederverwendungslager geleert, gereinigt und entwässert: das WC steht auf einer Pappe auf einer Halbpalette, Sitz und Deckel unten, der Spülkastendeckel mit dem Tank verklebt, der Abgang verschlossen, der Wasseranschluss abgedichtet.",
         {}, PADS, [FLOOR_UNDER_PALLET], binding()),
    pose("carried", "Carried by two people", "Von zwei Personen getragen",
         f"Lifted off the pallet by two carriers, the front one with both hands under the rim, the rear one under the rear body behind the outlet, hands at about 0.72 and 0.60 m, tilted {TILT:.0f} degrees front up so the cistern weight rests on the rear carrier; the pallet and the cardboard stay behind.",
         f"Von zwei Trägern von der Palette gehoben, der vordere mit beiden Händen unter dem Rand, der hintere unter dem Hinterkörper hinter dem Abgang, Hände auf etwa 0,72 und 0,60 m, {TILT:.0f} Grad vorn angehoben, damit das Gewicht des Spülkastens auf dem hinteren Träger liegt; Palette und Pappe bleiben zurück.",
         {"travel": {"dx": TRAVEL}, "lift": {"dy": LIFT}, "pan": {"angleDeg": TILT}}, ["grip-front", "grip-rear"], [FLOOR_UNDER_PALLET], binding()),
    pose("in-use", "In a bathroom again, in use", "Wieder in einem Bad, in Gebrauch",
         "Set on a bathroom floor and screwed down, the cistern filled with 6 l of cold water, the lid raised against the cistern and a person seated on the ring.",
         "Auf einen Badezimmerboden gestellt und verschraubt, der Spülkasten mit 6 l kaltem Wasser gefüllt, der Deckel gegen den Spülkasten hochgeklappt und eine Person auf dem Ring sitzend.",
         {**IN_USE, "lid": {"angleDeg": LID_OPEN_DEG}}, FEET, [FLOOR_UNDER_PALLET, WATER, OCCUPANT], binding(0.15, True)),
    pose("lid-open", "Lid raised", "Deckel hochgeklappt",
         "In the bathroom with the lid raised against the cistern and nobody seated.",
         "Im Bad mit gegen den Spülkasten hochgeklapptem Deckel, niemand sitzt.",
         {**IN_USE, "lid": {"angleDeg": LID_OPEN_DEG}}, FEET, [FLOOR_UNDER_PALLET, WATER], binding(0.15, True)),
    pose("lid-closed", "Lid down", "Deckel zu",
         "In the bathroom with seat and lid down and the cistern full.",
         "Im Bad mit Sitz und Deckel unten und vollem Spülkasten.",
         {**IN_USE}, FEET, [FLOOR_UNDER_PALLET, WATER], binding(0.15, True)),
    pose("flush-pressed", "Button pressed", "Drücker gedrückt",
         "The big half of the dual-flush button is pressed 8 mm down and the cistern empties into the bowl.",
         "Die große Hälfte des Zwei-Mengen-Drückers ist 8 mm gedrückt, und der Spülkasten entleert sich ins Becken.",
         {**IN_USE, "lid": {"angleDeg": LID_OPEN_DEG}, "button": {"dy": -0.008}}, FEET, [FLOOR_UNDER_PALLET, WATER, PRESS], binding(0.15, True), key=True),
    pose("lift-off-key", "Lifting off the pallet", "Abheben von der Palette",
         "Both carriers have taken hold and lift the WC clear of the cardboard.",
         "Beide Träger haben gefasst und heben das WC von der Pappe ab.",
         {"lift": {"dy": 0.03}, "pan": {"angleDeg": 1}}, ["grip-front", "grip-rear"], [FLOOR_UNDER_PALLET], binding(), key=True),
]

CLIPS = [
    {"id": "carry", "title": T("Carry it off the pallet", "Von der Palette wegtragen"), "description": T("Two carriers take the rim and the rear body, lift the WC off the cardboard, tilt it front up and carry it away; pallet and cardboard stay in the depot.", "Zwei Träger fassen Rand und Hinterkörper, heben das WC von der Pappe, neigen es vorn hoch und tragen es fort; Palette und Pappe bleiben im Lager."), "durationS": 2.4, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 0.5, "pose": "lift-off-key", "ease": "smooth"}, {"t": 1.8, "pose": "carried", "ease": "smooth"}, {"t": 2.4, "pose": "carried", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 0.2} for a in PADS] + [{"anchor": a, "fromS": 0, "toS": 2.4} for a in ("grip-front", "grip-rear")],
     "phases": [{"id": "grip", "fromS": 0, "toS": 0.5, "title": T("Take hold and lift", "Fassen und anheben")}, {"id": "swing", "fromS": 0.5, "toS": 1.8, "title": T("Tilt and walk", "Neigen und gehen")}, {"id": "carry", "fromS": 1.8, "toS": 2.4, "title": T("Carry", "Tragen")}],
     "events": [{"t": 0.2, "kind": "contact-off", "title": T("Foot leaves the cardboard", "Fuß verlässt die Pappe")}, {"t": 1.8, "kind": "motion", "title": T("Carried level with the hands", "Auf Handhöhe getragen")}], "tracks": []},
    {"id": "flush", "title": T("Raise the lid, flush, lower it", "Deckel auf, spülen, Deckel zu"), "description": T("In use: the lid is raised against the cistern, the big half of the button is pressed and released, the rinse runs and the lid is lowered again; the refill of the cistern takes about a minute in reality and is not shown.", "In Gebrauch: der Deckel wird gegen den Spülkasten hochgeklappt, die große Drückerhälfte gedrückt und losgelassen, die Spülung läuft und der Deckel wird wieder geschlossen; das Nachfüllen des Spülkastens dauert in Wirklichkeit etwa eine Minute und wird nicht gezeigt."), "durationS": 8.0, "fps": 30, "loop": True, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "lid-closed", "ease": "smooth"}, {"t": 1.2, "pose": "lid-open", "ease": "smooth"}, {"t": 2.0, "pose": "flush-pressed", "ease": "smooth"}, {"t": 2.6, "pose": "lid-open", "ease": "smooth"}, {"t": 6.5, "pose": "lid-open", "ease": "smooth"}, {"t": 8.0, "pose": "lid-closed", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 8.0} for a in FEET],
     "phases": [{"id": "open", "fromS": 0, "toS": 1.2, "title": T("Raise the lid", "Deckel hochklappen")}, {"id": "press", "fromS": 1.2, "toS": 2.6, "title": T("Press and release", "Drücken und loslassen")}, {"id": "rinse", "fromS": 2.6, "toS": 6.5, "title": T("Rinse", "Spülen")}, {"id": "close", "fromS": 6.5, "toS": 8.0, "title": T("Lower the lid", "Deckel schließen")}],
     "events": [{"t": 1.2, "kind": "motion", "title": T("Lid against the cistern", "Deckel am Spülkasten")}, {"t": 2.0, "kind": "motion", "title": T("Flush valve opens", "Spülventil öffnet")}, {"t": 8.0, "kind": "motion", "title": T("Lid down", "Deckel zu")}], "tracks": []},
]


def mat(id, en, de, eps, alpha, rho, k, c, E, f, source, reference):
    """🧱️ One material."""
    m = {"id": id, "title": T(en, de), "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": c}
    if E:
        m["elasticModulusPa"] = E
    if f:
        m["strengthPa"] = f
    m.update({"source": source, "reference": reference})
    return m


MATERIALS = [
    mat("vitreous-china-pan", "Vitreous china, white glaze, aged (limescale, fine crazing)", "Sanitärporzellan, weiße Glasur, gealtert (Kalk, feine Haarrisse)", 0.92, 0.3, 2350, 1.3, 850, 7.0e10, 4.0e7, "literature", "en-997-2018"),
    mat("vitreous-china-cistern", "Vitreous china, white glaze", "Sanitärporzellan, weiße Glasur", 0.92, 0.26, 2350, 1.3, 850, 7.0e10, 4.0e7, "literature", "en-997-2018"),
    mat("duroplast-seat", "Duroplast (urea-formaldehyde) seat set, yellowed", "Duroplast-Sitzgarnitur (Harnstoffharz), vergilbt", 0.9, 0.4, 1500, 0.3, 1300, 9.0e9, 6.0e7, "authored-estimate", "vdi-waermeatlas-2013"),
    mat("rubber-buffer", "Rubber buffers", "Gummipuffer", 0.92, 0.9, 1200, 0.16, 1400, 5.0e6, 1.0e7, "literature", "iso-10456-2007"),
    mat("chromed-zinc", "Chromed zinc die-cast hinge", "Verchromter Zinkdruckguss des Scharniers", 0.1, 0.4, 6600, 110, 390, 9.6e10, 2.8e8, "literature", "incropera-2018"),
    mat("pp-fittings", "Polypropylene button and valves", "Drücker und Ventile aus Polypropylen", 0.9, 0.3, 905, 0.22, 1800, 1.4e9, 3.2e7, "literature", "iso-10456-2007"),
    mat("pe-cap", "Red polyethylene outlet cap", "Rote Abgangskappe aus Polyethylen", 0.9, 0.65, 950, 0.4, 1900, 1.0e9, 2.0e7, "literature", "iso-10456-2007"),
    mat("spruce-pallet", "Spruce pallet timber, weathered", "Fichtenholz der Palette, verwittert", 0.9, 0.6, 470, 0.13, 1600, 1.1e10, 2.4e7, "literature", "en-338-2016"),
    mat("corrugated-board", "Corrugated cardboard, double wall, 5 mm", "Wellpappe, zweiwellig, 5 mm", 0.9, 0.7, 120, 0.065, 1340, None, None, "authored-estimate", "iso-10456-2007"),
]

AREA = json.loads((HERE / "areas.json").read_text()) if (HERE / "areas.json").exists() else {"pan": 0.95, "cistern": 0.42, "seat-set": 0.36, "outlet-cap": 0.025, "pallet": 1.6, "cardboard": 0.18}


def layer(id, en, de, kind, material, t, k, rho, c):
    """🧶️ One single layer with its resistance and areal heat capacity rho c min(t, sqrt(alpha 3600 s))."""
    depth = min(t, math.sqrt(k / (rho * c) * 3600))
    return {"id": id, "title": T(en, de), "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": round(t / k, 6), "heatCapacityJM2K": round(rho * c * depth), "source": "literature", "reference": "iso-10456-2007"}


def region(id, en, de, material, geometry, L, sky, lay, extra=None):
    """🔥️ One passive thermal region."""
    r = {"id": id, "title": T(en, de), "kind": "surface", "model": "passive", "material": material, "areaM2": AREA[id], "characteristicLengthM": L, "geometry": geometry, "skyViewFactor": sky, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"], "source": "authored-estimate", "reference": "iso-10456-2007"}
    if extra:
        r.update(extra)
    return r


REGIONS = [
    region("pan", "Glazed pan with bowl, foot, rear body and spigot", "Glasiertes Becken mit Schüssel, Fuß, Hinterkörper und Stutzen", "vitreous-china-pan", "vertical-plate", 0.4, 0.3, layer("pan-wall", "Vitreous china 10 mm", "Sanitärporzellan 10 mm", "masonry", "vitreous-china-pan", 0.010, 1.3, 2350, 850), {"ratedPowerW": 0, "ratedPowerBasis": "passive"}),
    region("cistern", "Cistern tank and lid", "Spülkasten mit Deckel", "vitreous-china-cistern", "vertical-plate", 0.37, 0.3, layer("cistern-wall", "Vitreous china 9 mm", "Sanitärporzellan 9 mm", "masonry", "vitreous-china-cistern", 0.009, 1.3, 2350, 850)),
    region("seat-set", "Seat ring, lid and hinge", "Sitzring, Deckel und Scharnier", "duroplast-seat", "horizontal-plate-up", 0.36, 0.5, layer("seat-shell", "Duroplast 6 mm", "Duroplast 6 mm", "polymer", "duroplast-seat", 0.006, 0.3, 1500, 1300)),
    region("outlet-cap", "Red outlet cap", "Rote Abgangskappe", "pe-cap", "cylinder", 0.116, 0.2, layer("cap-wall", "Polyethylene 4 mm", "Polyethylen 4 mm", "polymer", "pe-cap", 0.004, 0.4, 950, 1900)),
    region("pallet", "Half pallet", "Halbpalette", "spruce-pallet", "horizontal-plate-up", 0.6, 0.5, layer("pallet-board", "Spruce 22 mm", "Fichte 22 mm", "timber", "spruce-pallet", 0.022, 0.13, 470, 1600)),
    region("cardboard", "Cardboard sheet under the foot", "Pappe unter dem Fuß", "corrugated-board", "horizontal-plate-up", 0.4, 0.5, layer("cardboard-sheet", "Corrugated board 5 mm", "Wellpappe 5 mm", "insulation", "corrugated-board", 0.005, 0.065, 120, 1340)),
]

SYM_EL = [(-PAL_X, 0), (PAL_X, 0), (PAL_X, 0.144), (XB + 0.245, 0.144), (XB + 0.245, PH + 0.40), (XB + 0.243, PH + 0.451), (-0.17, PH + 0.451), (-0.17, PH + 0.79), (-0.34, PH + 0.79), (-0.34, PH + 0.772), (-0.335, PH + 0.772), (-0.335, PH + 0.40), (-0.325, PH + 0.40), (-0.325, PH + 0.388), (-0.31, PH + 0.388), (-0.31, PH + 0.238), (-0.3245, PH + 0.238), (-0.3245, PH + 0.122), (FOOT["cx"] - FOOT["rx"], PH), (-PAL_X, PH - 0.005), (-PAL_X, 0.144)]
SYMBOL = {"id": "used-wc-pan-cistern-symbol", "title": T("WC on its pallet, true extent", "WC auf seiner Palette, wahre Ausdehnung"), "description": T("The stored close-coupled WC in all three views: rim 0.40 m and cistern 0.79 m above the cardboard, depth 0.67 m, width 0.38 m, on a 0.8 x 0.6 m half pallet.", "Das gelagerte Kombi-WC in allen drei Ansichten: Rand 0,40 m und Spülkasten 0,79 m über der Pappe, Ausladung 0,67 m, Breite 0,38 m, auf einer Halbpalette 0,8 x 0,6 m."),
          "semantics": "true-extent", "units": "model-m", "denominators": [200, 100, 50], "pens": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, "hatches": [],
          "geometry": {
              "elevation": [sh("pallet", "ground", "body", 1, True, 0.6, wrect(-PAL_X, 0, PAL_X, 0.144)), sh("wc", "ground", "body", 2, True, 0.9, wpoly([(FOOT["cx"] - FOOT["rx"], PH), (FOOT["cx"] + FOOT["rx"], PH), (XB + 0.13, PH + 0.235), (XB + 0.245, PH + 0.365), (XB + 0.245, PH + 0.40), (XB + 0.243, PH + 0.451), (-0.17, PH + 0.451), (-0.17, PH + 0.79), (-0.34, PH + 0.79), (-0.34, PH + 0.772), (-0.335, PH + 0.772), (-0.335, PH + 0.40), (-0.31, PH + 0.388), (-0.31, PH + 0.238), (-0.3245, PH + 0.238), (-0.3245, PH + 0.122), (FOOT["cx"] - FOOT["rx"], PH + 0.122)])), sh("rim-line", "ground", "detail", 3, False, 0.3, wpoly([(XB + 0.245, PH + 0.40), (-0.135, PH + 0.40)], closed=False))],
              "front": [sh("pallet", "ground", "body", 1, True, 0.6, wrect(-PAL_Z, 0, PAL_Z, 0.144)), sh("wc", "ground", "body", 2, True, 0.9, wpoly([(-FOOT["rz"], PH), (FOOT["rz"], PH), (0.093, PH + 0.235), (0.18, PH + 0.365), (0.18, PH + 0.40), (0.185, PH + 0.40), (0.19, PH + 0.772), (0.19, PH + 0.79), (-0.19, PH + 0.79), (-0.19, PH + 0.772), (-0.185, PH + 0.40), (-0.18, PH + 0.40), (-0.18, PH + 0.365), (-0.093, PH + 0.235)])), sh("seat-line", "ground", "detail", 3, False, 0.3, wpoly([(-0.178, PH + 0.451), (0.178, PH + 0.451)], closed=False))],
              "plan": [sh("pallet", "ground", "body", 1, True, 0.6, wrect(-PAL_X, -PAL_Z, PAL_X, PAL_Z)), sh("wc", "ground", "body", 2, True, 0.9, wpoly([(-0.34, -0.19), (-0.17, -0.19), (-0.17, -0.155), (-0.135, -0.155), (0.09, -0.178), (0.25, -0.13), (0.333, 0.0), (0.25, 0.13), (0.09, 0.178), (-0.135, 0.155), (-0.17, 0.155), (-0.17, 0.19), (-0.34, 0.19)]))],
          },
          "dimensionalNote": T("Model metres at real size in the frame of the asset (x ahead to the front of the bowl, lateral positive to the left, y up): the side view is seen from the right, the front view from ahead, the top view from above; all three show the stored pose on the pallet.", "Modellmeter in wahrer Größe im Koordinatensystem des Objekts (x nach vorn zur Beckenfront, seitlich positiv nach links, y nach oben): die Seitenansicht blickt von rechts, die Vorderansicht von vorn, die Draufsicht von oben; alle drei zeigen die gelagerte Pose auf der Palette.")}

REFERENCES = [
    {"id": "en-33-2019", "citation": "EN 33:2019 Pedestal WC pans and WC suites - Connecting dimensions (horizontal outlet with its axis 180 mm above the floor and a spigot of about 100-110 mm outside diameter; rim height of a floor-standing pan about 400 mm; seat fixing holes in the deck behind the bowl)."},
    {"id": "en-997-2018", "citation": "EN 997:2018 WC pans and WC suites with integral trap (vitreous china or fireclay; static load test of the pan with 400 kg; flush volumes of 6 l and 3 l of class 1; water seal of at least 50 mm)."},
    {"id": "en-14055-2018", "citation": "EN 14055:2018 WC and urinal flushing cisterns (dual-flush cisterns of 6 l / 3 l, close-coupled cisterns on the deck of the pan, flush valve, fill valve and actuating button)."},
    {"id": "din-19516", "citation": "DIN 19516 Sitze und Deckel für Klosettbecken - Anforderungen und Prüfung (WC seats and lids: dimensions, buffers, hinge fixing and load tests of seat and lid)."},
    {"id": "din-18022-2007", "citation": "DIN 18022:2007-11 Küche, Bad und WC im Wohnungsbau - Planungsgrundlagen (placing area of a WC 0.40 x 0.60 m to 0.75 m, movement area in front of it; floor-standing WC 0.36-0.40 m wide, 0.60-0.75 m deep with a close-coupled cistern)."},
    {"id": "neufert-2019", "citation": "Neufert, E., Kister, J. (2019). Bauentwurfslehre, 42nd ed., Springer Vieweg (sanitary rooms: floor-standing WC with close-coupled cistern about 0.36 x 0.67 m, rim 0.40-0.42 m, cistern top about 0.78-0.80 m)."},
    {"id": "wc-suite-type", "citation": "Type data of German close-coupled washdown WC suites of 1985-2005 as builders' merchants offered them: white vitreous china, horizontal outlet, pan 18-22 kg, close-coupled 6/3 l cistern with lid 10-13 kg, duroplast seat and lid 1.8-2.6 kg, whole suite 30-38 kg; 0.36-0.38 m wide, 0.65-0.70 m deep, 0.78-0.80 m high; no single product, no brand."},
    {"id": "pallet-type", "citation": "Type data of the half pallet (Düsseldorfer Palette) of 800 x 600 x 144 mm as pallet traders offer it: spruce deck and bottom boards 22 mm, blocks 78 mm, about 9-12 kg, used as a storage base in building-component reuse depots; no single product."},
    {"id": "en-338-2016", "citation": "EN 338:2016 Structural timber - Strength classes (C24 softwood: 24 N/mm2 bending, 11 kN/mm2 modulus, 420 kg/m3 characteristic density; pallet spruce about 470 kg/m3 at depot moisture)."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: General actions - Densities, self-weight, imposed loads for buildings (self-weight as the product of volume and density)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (softwood 500 kg/m3 0.13 W/mK 1600 J/kgK, polyethylene, polypropylene, rubber, ceramics; cardboard as a fibrous insulation of low density)."},
    {"id": "vdi-waermeatlas-2013", "citation": "VDI-Wärmeatlas, 11th ed. (2013), Springer Vieweg (thermophysical properties of porcelain about 2300-2400 kg/m3, 1.2-1.6 W/mK, 800-900 J/kgK, and of thermosets such as urea-formaldehyde about 1500 kg/m3, 0.3 W/mK)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (total emissivity of glazed porcelain about 0.92, polished chromium about 0.1; solar absorptance of white glazed surfaces about 0.2-0.3, rising with deposits)."},
    {"id": "din-33402-2-2020", "citation": "DIN 33402-2:2020-12 Ergonomie - Körpermaße des Menschen - Teil 2: Werte (body mass and grip heights of adults: hands of a carrier holding a load in front of the body at about 0.70-0.75 m; a seated adult of 85 kg puts about three quarters of the body weight on the seat)."},
]

VOLUMES = {
    "50": {"status": "authored", "denominator": 50, "body": B50, "thermal": TH50, "structure": ST50, "decisions": [
        T("Body for the 7.5 mm feature: the bowl and rim as one revolved vitreous-china shell (outer jacket, 21 mm rim, inner bowl down to the 64 mm throat, about 15 mm wall), stretched to 0.36 m width; the hollow foot with 8 mm walls tapering from 440 x 210 mm at the floor, its 8 mm top plate; the hollow rear body with its bottom plate, the neck that fills behind the bowl, the 12 mm cistern deck; the 102 mm outlet spigot as a tube at 0.18 m with its red polyethylene cap; the trapway as three capsules; chromed hinge brackets and pin.", "Körper für das 7,5-mm-Merkmal: Becken und Rand als eine gedrehte Porzellanschale (Außenmantel, 21-mm-Rand, inneres Becken bis zum 64-mm-Hals, etwa 15 mm Wand), auf 0,36 m Breite gestreckt; der hohle Fuß mit 8-mm-Wänden, von 440 x 210 mm am Boden verjüngt, mit 8-mm-Deckplatte; der hohle Hinterkörper mit Bodenplatte, der Hals hinter dem Becken, die 12-mm-Spülkastenauflage; der 102-mm-Abgangsstutzen als Rohr auf 0,18 m mit roter Polyethylenkappe; der Siphon als drei Kapseln; verchromte Scharnierhalter mit Stift."),
        T("Cistern of 9 mm walls with its bottom, flush valve and fill valve inside, the 18 mm loose lid with the hole of the replaced 56 mm button; the seat ring as a 10 mm band with its 5 mm skirt on four 8 mm rubber buffers, the lid as a 5 mm shell (sheet) with a 4 mm skirt, both with their lugs on the pin; the half pallet board by board (five top boards, three cross boards, nine blocks, three bottom boards) with the 5 mm cardboard (sheet) under the foot.", "Spülkasten mit 9-mm-Wänden, Boden, Spül- und Füllventil innen, der lose 18-mm-Deckel mit dem Loch des ersetzten 56-mm-Drückers; der Sitzring als 10-mm-Band mit 5-mm-Schürze auf vier 8-mm-Gummipuffern, der Deckel als 5-mm-Schale (Platte) mit 4-mm-Schürze, beide mit Laschen am Stift; die Halbpalette Brett für Brett (fünf Deckbretter, drei Querbretter, neun Klötze, drei Bodenbretter) mit der 5-mm-Pappe (Platte) unter dem Fuß."),
        T("Thermal: every body solid in its region; the seat ring of the thermal list reaches down to the rim, so the shut lid closes the bowl (the 8 mm buffer gap is real but lets hardly any air in) and the hollow foot, rear body and cistern are closed cavities, so only the outer glazed surface is area. Structure: every part a member of its bone, buffers, pin, brackets and the two cistern bolts as joints, seven mass markers; no permanent support: the pose contacts (pallet blocks, foot, hands) carry it.", "Thermik: jeder Körper in seiner Region; der Sitzring der Thermikliste reicht bis auf den Rand, so schließt der zugeklappte Deckel das Becken (der 8-mm-Pufferspalt ist echt, lässt aber kaum Luft durch), und hohler Fuß, Hinterkörper und Spülkasten sind geschlossene Hohlräume, so zählt nur die glasierte Außenfläche. Tragwerk: jedes Teil ein Stab seines Knochens, Puffer, Stift, Halter und die zwei Spülkastenschrauben als Gelenke, sieben Massenmarken; kein dauerndes Auflager: die Kontakte der Posen (Palettenklötze, Fuß, Hände) tragen."),
    ]},
    "100": {"status": "authored", "denominator": 100, "body": B100, "thermal": TH100, "structure": ST100, "decisions": [
        T("Body for the 15 mm feature: the bowl shell with eleven profile points, the hollow foot (a vessel, judged by its extents) with a 15 mm top plate, the rear body as one mass under the deck, the neck, spigot and cap as one turned part, a two-piece trapway, the brackets; the cistern as one mass with its lid and the button; the seat ring down on the rim (buffers merged), seat lugs, lid and lid lugs; the pallet with top boards, three stringers (cross board and blocks as one comb profile) and bottom boards, the cardboard sheet.", "Körper für das 15-mm-Merkmal: die Beckenschale mit elf Profilpunkten, der hohle Fuß (ein Gefäß, nach seinen Abmessungen beurteilt) mit 15-mm-Deckplatte, der Hinterkörper als eine Masse unter der Auflage, der Hals, Stutzen und Kappe als ein gedrehtes Teil, ein zweiteiliger Siphon, die Halter; der Spülkasten als eine Masse mit Deckel und Drücker; der Sitzring auf dem Rand (Puffer zusammengefasst), Sitzlaschen, Deckel und Deckellaschen; die Palette mit Deckbrettern, drei Querträgern (Querbrett und Klötze als ein Kammprofil) und Bodenbrettern, die Pappe."),
        T("Thermal and structure reference the body: the seat ring now closes the bowl itself; masses 24 mm, cistern bolts 15 mm.", "Thermik und Tragwerk verweisen auf den Körper: der Sitzring schließt das Becken jetzt selbst; Massen 24 mm, Spülkastenschrauben 15 mm."),
    ]},
    "200": {"status": "authored", "denominator": 200, "body": B200, "thermal": TH200, "structure": ST200, "decisions": [
        T("Body for the 30 mm feature: the bowl as a filled turned body with its 0.10 m deep dish, the tapered foot, rear body, neck and a 30 mm deck, the outlet and cap as one turned part; the cistern, a 30 mm lid and the button as a 30 mm disc; seat ring and lid as 30 mm bands that reach back to the hinge axis; the pallet as a 44 mm deck slab on three runners of blocks and a 30 mm bottom board, and the cardboard sheet.", "Körper für das 30-mm-Merkmal: das Becken als gefüllter Drehkörper mit 0,10 m tiefer Mulde, der verjüngte Fuß, Hinterkörper, Hals und eine 30-mm-Auflage, Abgang und Kappe als ein gedrehtes Teil; der Spülkasten, ein 30-mm-Deckel und der Drücker als 30-mm-Scheibe; Sitzring und Deckel als 30-mm-Bänder bis zur Scharnierachse; die Palette als 44-mm-Deckplatte auf drei Kufen aus Klötzen und einem 30-mm-Bodenbrett, und die Pappe."),
        T("Thermal: every body solid in its region; structure: every part a member, the cistern bolts as 30 mm joints, masses 36 mm.", "Thermik: jeder Körper in seiner Region; Tragwerk: jedes Teil ein Stab, die Spülkastenschrauben als 30-mm-Gelenke, Massen 36 mm."),
    ]},
    "500": {"status": "unavailable", "denominator": 500},
}

PAN_KG, CIS_KG, SEAT_KG = MASS["pan"]["kg"], MASS["cistern"]["kg"] + MASS["cistern-lid"]["kg"] + MASS["button"]["kg"], MASS["seat"]["kg"] + MASS["lid"]["kg"]

ASSET = {
    "id": "used-wc-pan-cistern", "category": "objects",
    "title": T("Used WC Pan with Cistern", "Gebrauchtes Stand-WC mit Spülkasten"),
    "summary": T(f"A white close-coupled floor-standing washdown WC of the 1990s with a 6/3 l ceramic cistern and a duroplast seat, 0.67 x 0.37 x 0.80 m and {WC_KG:.1f} kg, horizontal outlet at 0.18 m capped in red, limescale in the bowl, a chipped foot, old silicone and a yellowed, scratched lid; stored on cardboard on a half pallet with the cistern lid taped, carried by two people, back in a bathroom with the lid raised, a person seated and a flush.", f"Ein weißes Kombi-Stand-WC (Tiefspüler) der 1990er-Jahre mit 6/3-l-Keramikspülkasten und Duroplastsitz, 0,67 x 0,37 x 0,80 m und {WC_KG:.1f} kg, waagerechter Abgang auf 0,18 m mit roter Kappe, Kalk im Becken, abgeplatzter Fuß, alte Silikonfuge und vergilbter, verkratzter Deckel; auf Pappe auf einer Halbpalette mit verklebtem Spülkastendeckel gelagert, von zwei Personen getragen, wieder in einem Bad mit hochgeklapptem Deckel, sitzender Person und Spülung."),
    "tags": ["used", "household", "second-hand", "bathroom", "wc", "toilet", "sanitary", "ceramic", "vitreous-china", "cistern", "seat", "pallet", "reuse-depot", "1990s", "interior"],
    "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES,
                   "notes": T(f"German close-coupled washdown WC suite of the 1990s type, taken out at a flat clearance, emptied, cleaned and drained. Pan of white vitreous china: rim 0.40 m high, 0.49 x 0.36 m, bowl down to a 64 mm throat, trapway to the horizontal outlet spigot of 102 mm outside diameter at 0.18 m (EN 33), foot 0.44 x 0.21 m at the floor, rear body and 12 mm deck carrying the 0.16 x 0.37 x 0.37 m cistern of 9 mm walls with its flush and fill valves and an 18 mm loose lid; a duroplast seat ring on four 8 mm buffers and a lid on a chromed hinge. Traces of use: limescale ring in the bowl, a chipped glaze at the front of the foot, a bead of old silicone along the foot, a yellowed and scratched lid, the white replacement button, a depot sticker; nothing that stops it working. Stored on a 5 mm cardboard sheet on an 800 x 600 mm half pallet, the cistern lid taped, the outlet closed by a red cap; pallet and cardboard stay behind when it is carried. Masses from the 1:50 volumes x density: pan {PAN_KG:.2f} kg, cistern with lid and button {CIS_KG:.2f} kg, seat and lid {SEAT_KG:.2f} kg, pallet with cardboard {PALLET_KG:.2f} kg.",
                              f"Deutsches Kombi-WC (Tiefspüler) vom Typ der 1990er-Jahre, bei einer Haushaltsauflösung ausgebaut, geleert, gereinigt und entwässert. Becken aus weißem Sanitärporzellan: Rand 0,40 m hoch, 0,49 x 0,36 m, Becken bis zu einem 64-mm-Hals, Siphon zum waagerechten Abgangsstutzen mit 102 mm Außendurchmesser auf 0,18 m (EN 33), Fuß 0,44 x 0,21 m am Boden, Hinterkörper und 12-mm-Auflage für den Spülkasten 0,16 x 0,37 x 0,37 m mit 9-mm-Wänden, Spül- und Füllventil und losem 18-mm-Deckel; ein Duroplast-Sitzring auf vier 8-mm-Puffern und ein Deckel an einem verchromten Scharnier. Gebrauchsspuren: Kalkrand im Becken, abgeplatzte Glasur vorn am Fuß, eine Raupe alten Silikons am Fuß, ein vergilbter und verkratzter Deckel, der weiße Ersatzdrücker, ein Lageraufkleber; nichts, was den Gebrauch hindert. Gelagert auf einer 5-mm-Pappe auf einer Halbpalette 800 x 600 mm, der Spülkastendeckel verklebt, der Abgang mit einer roten Kappe verschlossen; Palette und Pappe bleiben beim Tragen zurück. Massen aus den 1:50-Volumen x Dichte: Becken {PAN_KG:.2f} kg, Spülkasten mit Deckel und Drücker {CIS_KG:.2f} kg, Sitz und Deckel {SEAT_KG:.2f} kg, Palette mit Pappe {PALLET_KG:.2f} kg.")},
    "dimensions": {"heightM": round(PH + BUTTON["y1"], 4), "widthM": round(2 * PAL_X, 4), "depthM": round(2 * PAL_Z, 4), "massKg": round(TOTAL_KG, 5)},
    "materials": MATERIALS,
    "rig": {"kind": "mechanical", "root": "ground", "bones": BONES},
    "anchors": ANCHORS,
    "representations": {"50": rep50(), "100": rep100(), "200": rep200(), "500": rep500()},
    "symbols": [SYMBOL],
    "poses": POSES,
    "clips": CLIPS,
    "thermal": {"environment": {"airTempC": 21, "meanRadiantTempC": 20, "relativeHumidity": 0.55, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 200, "diffuseFraction": 0.3, "pressurePa": 101325}, "operating": [], "regions": REGIONS, "emitters": [], "passengers": [],
                "assumptions": [
                    T("Vitreous china after VDI-Wärmeatlas (2350 kg/m3, 1.3 W/mK, 850 J/kgK); areal heat capacity rho c min(t, sqrt(alpha 3600 s)): the 8-15 mm walls take part fully.", "Sanitärporzellan nach VDI-Wärmeatlas (2350 kg/m3, 1,3 W/mK, 850 J/kgK); flächenbezogene Wärmekapazität rho c min(t, sqrt(alpha 3600 s)): die 8- bis 15-mm-Wände nehmen voll teil."),
                    T("Aged surfaces: limescale and fine crazing raise the solar absorptance of the pan glaze from about 0.22 new to 0.30, the cistern stays at 0.26; the yellowed duroplast absorbs 0.40 against about 0.25 new (authored estimates within the range of white glazes and plastics after Incropera).", "Gealterte Oberflächen: Kalk und feine Haarrisse heben die Solarabsorption der Beckenglasur von etwa 0,22 neu auf 0,30, der Spülkasten bleibt bei 0,26; das vergilbte Duroplast absorbiert 0,40 statt etwa 0,25 neu (Schätzwerte im Bereich weißer Glasuren und Kunststoffe nach Incropera)."),
                    T("The areas are the outer surfaces of the default pose: with the lid shut the bowl is a closed cavity; raised, the inner bowl (about 0.2 m2) also exchanges heat, which this passive model does not add. In use, half the cistern surface stands against 12 degC tap water (contact fraction 0.5), so the tank is the cold, condensation-prone part of the suite.", "Die Flächen sind die Außenflächen der Standardpose: mit geschlossenem Deckel ist das Becken ein geschlossener Hohlraum; hochgeklappt tauscht auch das innere Becken (etwa 0,2 m2) Wärme aus, was dieses passive Modell nicht hinzurechnet. In Gebrauch liegt die halbe Spülkastenfläche an 12 degC kaltem Leitungswasser (Kontaktanteil 0,5), so ist der Tank der kalte, schwitzwassergefährdete Teil der Garnitur."),
                    T("Reference case: a bathroom at 21 degC, 55 % relative humidity, still air, a little window sun on the lid in use; the cardboard lies with 0.4 of its surface on the 15 degC pallet.", "Referenzfall: ein Bad bei 21 degC, 55 % relativer Feuchte, ruhender Luft und etwas Fenstersonne auf dem Deckel in Gebrauch; die Pappe liegt mit 0,4 ihrer Fläche auf der 15 degC kühlen Palette."),
                    T("No electrical element: a WC and its cistern are passive, 0 W stated explicitly on the pan region; the person and the water are loads, not parts of the asset.", "Kein elektrisches Element: WC und Spülkasten sind passiv, 0 W ausdrücklich an der Region Becken angegeben; Person und Wasser sind Lasten, nicht Teile des Assets."),
                ]},
    "structure": {"gravityMS2": G, "dynamics": "quasi-static", "supports": [],
                  "assumptions": [
                      T("One rigid body on the contacts of each pose: stored, the four outer blocks of the pallet stand on the depot floor and carry WC, cardboard and pallet; carried, the hands of the two carriers under the rim and under the rear body are pins; in use, the foot stands on the bathroom floor (four contacts at the edge of its footprint).", "Ein starrer Körper auf den Kontakten jeder Pose: gelagert stehen die vier äußeren Klötze der Palette auf dem Lagerboden und tragen WC, Pappe und Palette; getragen sind die Hände der beiden Träger unter dem Rand und unter dem Hinterkörper Gelenke; in Gebrauch steht der Fuß auf dem Badezimmerboden (vier Kontakte am Rand seiner Aufstandsfläche)."),
                      T(f"The pallet with its cardboard ({PALLET_KG:.2f} kg) belongs to the asset but lies on its own: its floor reaction is entered as an upward load at its centre in every pose, so the hands carry the WC alone ({WC_KG:.2f} kg = {WC_KG * G:.0f} N) and the pallet is never lifted with it.", f"Die Palette mit Pappe ({PALLET_KG:.2f} kg) gehört zum Asset, liegt aber für sich: ihre Bodenreaktion ist in jeder Pose als aufwärts gerichtete Last in ihrer Mitte angesetzt, so tragen die Hände nur das WC ({WC_KG:.2f} kg = {WC_KG * G:.0f} N), und die Palette wird nie mit angehoben."),
                      T("Members: vitreous china shells (E about 70 GPa, flexural strength about 40 N/mm2), the cistern bolted to the deck by two M10 bolts through a rubber seal, the duroplast seat set on buffers and a chromed hinge; spruce pallet boards C24 (EN 338). Masses are section x density of the 1:50 solids; the trapway counts its 9 mm wall, not the capsule that stands for its outer envelope.", "Stäbe: Porzellanschalen (E etwa 70 GPa, Biegefestigkeit etwa 40 N/mm2), der Spülkasten mit zwei M10-Schrauben durch eine Gummidichtung auf der Auflage verschraubt, die Duroplast-Sitzgarnitur auf Puffern und einem verchromten Scharnier; Palettenbretter aus Fichte C24 (EN 338). Massen sind Querschnitt x Dichte der 1:50-Körper; der Siphon zählt seine 9-mm-Wand, nicht die Kapsel, die für seine Außenhülle steht."),
                      T("In use: 6 l of water in the cistern (EN 14055), a seated person of 85 kg putting 0.65 kN on the ring (DIN 33402-2), 25 N on the button; the pan is tested to 400 kg (EN 997) and the seat to the loads of DIN 19516, so the used suite carries them with a wide margin.", "In Gebrauch: 6 l Wasser im Spülkasten (EN 14055), eine sitzende Person von 85 kg mit 0,65 kN auf dem Ring (DIN 33402-2), 25 N auf dem Drücker; das Becken ist auf 400 kg geprüft (EN 997) und der Sitz auf die Lasten der DIN 19516, so trägt die gebrauchte Garnitur sie mit großer Reserve."),
                  ], "clinical": False},
    "volumes": VOLUMES,
    "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

ORDER = ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure", "volumes", "presentation", "exports", "completion"]
DOC = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": {k: ASSET[k] for k in ORDER}}

if __name__ == "__main__":
    (HERE / "🔣️.json").write_text(json.dumps(DOC, ensure_ascii=False, indent=1) + "\n")
    for b, m in MASS.items():
        print(f"[DEBUG] {b}: {m['kg']:.4f} kg com ({m['comX']:.4f}, {m['comY']:.4f}, {m['comZ']:.4f})")
    print(f"[DEBUG] total {TOTAL_KG:.4f} kg, WC {WC_KG:.4f} kg, pallet {PALLET_KG:.4f} kg")
