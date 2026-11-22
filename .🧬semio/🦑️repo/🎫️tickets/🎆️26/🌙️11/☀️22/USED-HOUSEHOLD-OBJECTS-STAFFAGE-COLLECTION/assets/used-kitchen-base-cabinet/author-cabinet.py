"""🗄️ Authoritative generator of the staffage asset `used-kitchen-base-cabinet`: `python3 author-cabinet.py` writes `🔣️.json` next to it.

Every number is stated in `author_parts.py` (world rest metres), every scale in `author_scales.py`; this file adds materials, rig,
anchors, poses, clips, the thermal and structural models, the decisions and assembles the document.

@see ../../brief.md — the author brief
@see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts — the contract
"""
import json
import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).parent))
from author_geom import T, R6
from author_parts import FR, BL, LEG, CB, CT, WT, WTT, CX, FT, FX, BD, HALF, MARGIN, FRONT_TOP, DOOR_B_TOP, DRAWER_BOTTOM, TAP, LEGS_X, LEGS_Z, LEG_STEP
from author_scales import build
from author_texts import TEXTS, REPRESENTATION_DECISIONS, VOLUME_DECISIONS, REFERENCES, ASSUMPTIONS_THERMAL, ASSUMPTIONS_STRUCTURE

HERE = pathlib.Path(__file__).parent
AREA = json.loads((HERE / "author-areas.json").read_text()) if (HERE / "author-areas.json").exists() else {}


def material(mid, en, de, eps, alpha, rho, lam, c, e, f, src, ref):
    """🧱 One material with its thermal and structural data."""
    return {"id": mid, "title": T(en, de), "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": rho, "conductivityWmK": lam, "specificHeatJKgK": c, "elasticModulusPa": e, "strengthPa": f, "source": src, "reference": ref}


MATERIALS = [
    material("melamine-chipboard-aged", "Melamine-faced chipboard P2, 18 mm, white decor yellowed by 25 years of kitchen use", "Melaminbeschichtete Spanplatte P2, 18 mm, weißes Dekor nach 25 Jahren Küchengebrauch vergilbt", 0.90, 0.32, 650, 0.13, 1700, 1.6e9, 11.5e6, "literature", "en-312"),
    material("swollen-chipboard", "Water-swollen chipboard at the sink (thickness swelling, loosened chips)", "Wassergequollene Spanplatte an der Spüle (Dickenquellung, gelockerte Späne)", 0.92, 0.55, 560, 0.12, 1700, 0.7e9, 4.0e6, "authored-estimate", "en-317"),
    material("foil-mdf-front", "MDF front 19 mm wrapped in ivory PVC foil, dulled", "MDF-Front 19 mm mit elfenbeinfarbener PVC-Folie ummantelt, stumpf geworden", 0.91, 0.36, 750, 0.14, 1700, 2.2e9, 20e6, "literature", "en-622-5"),
    material("hdf-back", "HDF back panel 3 mm, nailed on", "HDF-Rückwand 3 mm, aufgenagelt", 0.90, 0.45, 850, 0.17, 1700, 3.5e9, 35e6, "literature", "iso-10456-2007"),
    material("hpl-worktop", "Postformed HPL worktop 38 mm on P3 chipboard, beige granite decor, worn and scratched", "Postforming-Arbeitsplatte 38 mm, HPL auf P3-Spanplatte, beiges Granitdekor, abgenutzt und zerkratzt", 0.90, 0.58, 680, 0.14, 1700, 2.0e9, 13e6, "literature", "en-438"),
    material("stainless-steel-sink", "Stainless steel 1.4301, 0.8 mm, brushed, dulled by limescale", "Edelstahl 1.4301, 0,8 mm, gebürstet, durch Kalk stumpf", 0.35, 0.45, 7900, 15, 500, 200e9, 230e6, "literature", "en-10088-2"),
    material("chrome-brass", "Chrome-plated brass (single-lever mixer), limescale at the base", "Verchromtes Messing (Einhebelmischer), Kalk am Fuß", 0.12, 0.40, 8500, 110, 380, 100e9, 250e6, "literature", "incropera-2018"),
    material("pp-plastic", "Polypropylene adjustable legs and plinth clips, black", "Polypropylen-Stellfüße und Sockelclips, schwarz", 0.92, 0.85, 905, 0.22, 1900, 1.3e9, 30e6, "literature", "iso-10456-2007"),
    material("moving-blanket", "Quilted cotton and polyester moving blanket, folded in five layers", "Gesteppte Möbeldecke aus Baumwolle und Polyester, fünflagig gefaltet", 0.90, 0.75, 380, 0.045, 1300, 1.0e6, 0.5e6, "authored-estimate", "removal-blanket"),
    material("aluminium-handle", "Anodised aluminium bow handle, 128 mm centres", "Eloxierter Aluminium-Bügelgriff, Lochabstand 128 mm", 0.30, 0.40, 2700, 160, 900, 70e9, 160e6, "literature", "incropera-2018"),
    material("stainless-bar-handle", "Brushed stainless bar handle, 160 mm centres (replacement)", "Gebürsteter Edelstahl-Stangengriff, Lochabstand 160 mm (Ersatz)", 0.30, 0.45, 7900, 15, 500, 200e9, 230e6, "literature", "en-10088-2"),
    material("zinc-steel-runner", "Zinc-plated steel roller runners", "Verzinkte Stahl-Rollenauszüge", 0.25, 0.50, 7850, 50, 450, 210e9, 235e6, "literature", "iso-10456-2007"),
    material("braided-hose", "Stainless-braided flexible supply hose, cut", "Edelstahlummantelter Panzerschlauch, abgeschnitten", 0.40, 0.45, 1400, 0.3, 1500, 0.5e9, 20e6, "authored-estimate", "incropera-2018"),
]


def bone(bid, parent, en, de, x, y, angle, length, joint, z=None, section=None):
    """🦴 One rig bone; the mass is filled from the 1:50 solids."""
    b = {"id": bid, "parent": parent, "title": T(en, de), "x": R6(x), "y": R6(y), "angleDeg": angle, "lengthM": length, "joint": joint}
    if z is not None:
        b["z"] = R6(z)
    if section is not None:
        b["section"] = section
    return b


def rig(masses):
    """🌳 Floor reference, blanket, the lift of carrying, the carcass with worktop, sink and tap, two door bones on their hinge axes and the drawer on its runners."""
    bones = [
        bone("floor", None, "Floor reference of the depot or the kitchen", "Bodenbezug des Lagers oder der Küche", 0, 0, 0, 0, {"type": "fixed"}),
        bone("blanket", "floor", "Moving blanket (stays behind on the floor)", "Möbeldecke (bleibt auf dem Boden liegen)", 0, 0, 0, 0.58, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.9}),
        bone("lift", "floor", "Lift of the units off the blanket or the floor (carrying height)", "Anheben der Schränke von der Decke oder vom Boden (Tragehöhe)", 0, BL, 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": -BL, "maxM": 0.7}),
        bone("carcass", "lift", "Two screwed base unit carcasses on their legs", "Zwei verschraubte Unterschrankkorpusse auf ihren Füßen", 0, 0, 0, CX, {"type": "fixed"}),
        bone("worktop", "carcass", "Laminate worktop 1200 x 600 x 38 mm, screwed to the rails", "Laminat-Arbeitsplatte 1200 x 600 x 38 mm, an die Traversen geschraubt", 0, CT - BL, 0, 0.600, {"type": "fixed"},
             section={"areaM2": 0.0456, "secondMomentM4": 5.4872e-06, "extremeFibreM": 0.019, "material": "hpl-worktop", "title": T("Worktop 1200 x 38 mm", "Arbeitsplatte 1200 x 38 mm")}),
        bone("sink", "worktop", "Inset stainless sink 860 x 500 mm with bowl and drainer", "Einbauspüle aus Edelstahl 860 x 500 mm mit Becken und Abtropffläche", 0.05, WT, 0, 0.50, {"type": "fixed"}),
        bone("tap", "sink", "Single-lever mixer, supply hoses cut off", "Einhebelmischer, Zulaufschläuche abgeschnitten", 0.05, 0.008, 0, 0.205, {"type": "fixed"}, z=TAP[2]),
        bone("door-a", "carcass", "Door of the sink unit on its left hinge axis (concealed cup hinges)", "Tür des Spülenschranks an ihrer linken Scharnierachse (Topfscharniere)", CX, CB - BL, 90, R6(FRONT_TOP - CB), {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": 0, "twistMaxDeg": 110}, z=HALF - MARGIN,
             section={"areaM2": 0.011372, "secondMomentM4": 3.421e-07, "extremeFibreM": 0.0095, "material": "foil-mdf-front", "title": T("Door 598.5 x 19 mm", "Tür 598,5 x 19 mm")}),
        bone("door-b", "carcass", "Door under the drawer on its right hinge axis", "Tür unter der Schublade an ihrer rechten Scharnierachse", CX, CB - BL, 90, R6(DOOR_B_TOP - CB), {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -110, "twistMaxDeg": 0}, z=-(HALF - MARGIN),
             section={"areaM2": 0.011372, "secondMomentM4": 3.421e-07, "extremeFibreM": 0.0095, "material": "foil-mdf-front", "title": T("Door 598.5 x 19 mm", "Tür 598,5 x 19 mm")}),
        bone("drawer", "carcass", "Drawer of unit B on roller runners (3/4 extension)", "Schublade von Schrank B auf Rollenauszügen (Teilauszug 3/4)", 0.060, DRAWER_BOTTOM - BL, 0, 0.519, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.45}, z=-0.3),
    ]
    total = 0.0
    for b in bones:
        items = [m for m in masses if m[0] == b["id"]]
        if not items:
            continue
        kg = sum(m[1] for m in items)
        cx = sum(m[1] * m[2][0] for m in items) / kg
        cy = sum(m[1] * m[2][1] for m in items) / kg
        cz = sum(m[1] * m[2][2] for m in items) / kg
        u, v, _ = FR[b["id"]].local(cx, cy, cz)
        b["mass"] = {"kg": round(kg, 3), "comX": R6(u), "comY": R6(v), "distribution": "point", "source": "authored-estimate", "reference": "semio-used-household"}
        total += round(kg, 3)
    return {"kind": "mechanical", "root": "floor", "bones": bones}, round(total, 3)


def anchors():
    """📍 Blanket on the floor, the eight legs, and the four grips of the two movers under the outer side panels."""
    out = [
        {"id": "blanket-back", "bone": "blanket", "x": 0.12, "y": 0, "kind": "contact", "title": T("Back half of the blanket on the floor", "Hintere Hälfte der Decke auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.24, "footprintWidthM": 1.2, "frictionCoefficient": 0.6},
        {"id": "blanket-front", "bone": "blanket", "x": 0.46, "y": 0, "kind": "contact", "title": T("Front half of the blanket on the floor", "Vordere Hälfte der Decke auf dem Boden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.24, "footprintWidthM": 1.2, "frictionCoefficient": 0.6},
    ]
    k = 0
    for x in LEGS_X:
        for z0 in LEGS_Z:
            for j in range(2):
                k += 1
                z = z0 + j * LEG_STEP
                out.append({"id": f"leg-{k}", "bone": "carcass", "x": x, "y": 0, "z": R6(z), "kind": "contact", "title": T(f"Leg {k} on the floor ({'front' if x > 0.2 else 'back'} row)", f"Fuß {k} auf dem Boden ({'vordere' if x > 0.2 else 'hintere'} Reihe)"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.05, "footprintWidthM": 0.05, "frictionCoefficient": 0.5})
    for name, x, z, en, de in (("grip-right-back", 0.06, -0.591, "right mover, rear hand", "rechter Träger, hintere Hand"), ("grip-right-front", 0.50, -0.591, "right mover, front hand", "rechter Träger, vordere Hand"),
                               ("grip-left-back", 0.06, 0.591, "left mover, rear hand", "linker Träger, hintere Hand"), ("grip-left-front", 0.50, 0.591, "left mover, front hand", "linker Träger, vordere Hand")):
        out.append({"id": name, "bone": "carcass", "x": x, "y": CB - BL, "z": z, "kind": "support", "title": T(f"Grip under the outer side panel: {en}", f"Griff unter der äußeren Seitenwand: {de}"), "support": "pin", "normalDeg": 90, "footprintLengthM": 0.09, "footprintWidthM": 0.018, "frictionCoefficient": 0.6})
    return out


LEG_IDS = [f"leg-{k}" for k in range(1, 9)]
GRIPS = ["grip-right-back", "grip-right-front", "grip-left-back", "grip-left-front"]
BLANKET = ["blanket-back", "blanket-front"]
REGIONS = ["carcass", "fronts", "worktop", "sink", "tap", "plinth", "blanket"]


def binding(sunlit_top, blanket_contact, wet=0.0):
    """🌬 The region binding of a pose: every region exposed, the worktop and sink partly sunlit, the blanket underside on the floor."""
    r = {k: {"exposedFraction": 1.0, "sunlitFraction": 0.0} for k in REGIONS}
    r["worktop"]["sunlitFraction"] = sunlit_top
    r["sink"]["sunlitFraction"] = sunlit_top
    r["blanket"] = {"exposedFraction": 0.5, "sunlitFraction": 0.1 if blanket_contact else 0.0, "contact": {"fraction": 0.5, "tempC": 14}}
    r["plinth"]["exposedFraction"] = 0.9
    if wet:
        r["sink"]["wettedness"] = wet
    return {"regions": r, "emitters": {}}


def loads(open_drawer):
    """🧲 Service loads in a kitchen: equipment on the worktop, a person leaning on the front edge, water in the bowl, cutlery in the drawer."""
    out = [
        {"id": "worktop-equipment", "kind": "imposed", "title": T("Kitchen equipment on the worktop, 0.5 kN/m2 over 1.2 x 0.6 m", "Küchengeräte auf der Arbeitsplatte, 0,5 kN/m2 über 1,2 x 0,6 m"), "distributed": {"bone": "worktop", "fromM": 0, "toM": 0.6, "wxNPerM": 0, "wyNPerM": -600.0}, "fxN": 0, "fyN": 0, "source": "authored-estimate", "reference": "en-1991-1-1"},
        {"id": "leaning-person", "kind": "occupant", "title": T("Person leaning on the front edge with the forearms, 300 N", "Person stützt sich mit den Unterarmen auf die Vorderkante, 300 N"), "bone": "worktop", "x": 0.58, "y": WT, "z": -0.30, "fxN": 0, "fyN": -300.0, "source": "authored-estimate", "reference": "en-14749"},
        {"id": "bowl-water", "kind": "carried", "title": T("10 litres of washing-up water in the bowl", "10 Liter Spülwasser im Becken"), "bone": "sink", "x": 0.28, "y": -0.12, "z": 0.34, "fxN": 0, "fyN": -98.1, "source": "authored-estimate", "reference": "en-1991-1-1"},
        {"id": "cutlery", "kind": "carried", "title": T("Cutlery and utensils in the drawer, 4 kg", "Besteck und Küchenhelfer in der Schublade, 4 kg"), "bone": "drawer", "x": 0.26, "y": 0.04, "fxN": 0, "fyN": -39.2, "source": "authored-estimate", "reference": "en-14749"},
    ]
    return out


def poses():
    """🧍 Stored on the blanket (default), carried by two movers, in use in a kitchen, and in use with both doors and the drawer open."""
    return [
        {"id": "stored", "title": T("Stored on a moving blanket", "Auf einer Möbeldecke gelagert"), "description": T("As the flat clearance left it in the stairwell and the reuse depot took it in: both units with the worktop on their legs on a moving blanket folded to the footprint, doors and drawer shut, tap hoses cut off under the sink.", "Wie die Haushaltsauflösung es im Treppenhaus abgestellt und das Wiederverwendungslager übernommen hat: beide Schränke mit der Arbeitsplatte auf ihren Füßen auf einer auf die Grundfläche gefalteten Möbeldecke, Türen und Schublade zu, Armaturschläuche unter der Spüle abgeschnitten."),
         "bones": {}, "ground": "auto", "contacts": BLANKET, "thermal": binding(0.2, True), "loads": []},
        {"id": "carried", "title": T("Carried by two movers", "Von zwei Trägern getragen"), "description": T("Two movers at the two ends lift the run level by the bottom edges of the outer side panels to knuckle height (grips 0.71 m above the floor); the closing springs of the cup hinges and the self-closing dip of the runners keep doors and drawer shut; the blanket stays on the floor.", "Zwei Träger an den beiden Enden heben die Zeile waagerecht an den Unterkanten der äußeren Seitenwände auf Knöchelhöhe (Griffe 0,71 m über dem Boden); die Schließfedern der Topfscharniere und der Selbsteinzug der Auszüge halten Türen und Schublade zu; die Decke bleibt auf dem Boden liegen."),
         "bones": {"lift": {"dy": 0.55}}, "ground": "authored", "contacts": GRIPS, "thermal": binding(0.0, False), "loads": []},
        {"id": "in-use", "title": T("Back in use in a kitchen", "Wieder in Gebrauch in einer Küche"), "description": T("Set down on its eight legs in a kitchen, the blanket pulled forward as a floor mat; equipment on the worktop, a person leaning on the front edge, washing-up water in the bowl, cutlery in the drawer.", "Auf seinen acht Füßen in einer Küche abgestellt, die Decke als Bodenmatte nach vorn gezogen; Geräte auf der Arbeitsplatte, eine Person stützt sich auf die Vorderkante, Spülwasser im Becken, Besteck in der Schublade."),
         "bones": {"lift": {"dy": -BL}, "blanket": {"dx": 0.75}}, "ground": "auto", "contacts": LEG_IDS, "thermal": binding(0.2, True, 0.4), "loads": loads(False)},
        {"id": "doors-open", "title": T("In use with doors and drawer open", "In Gebrauch mit offenen Türen und Schublade"), "description": T("In the kitchen with the drawer pulled out 420 mm on its runners and both doors swung open 95 degrees on their concealed hinges.", "In der Küche mit 420 mm auf den Auszügen herausgezogener Schublade und beiden Türen 95 Grad an ihren Topfscharnieren aufgeschwenkt."),
         "bones": {"lift": {"dy": -BL}, "blanket": {"dx": 0.75}, "drawer": {"dx": 0.42}, "door-a": {"twistDeg": 95}, "door-b": {"twistDeg": -95}}, "ground": "auto", "contacts": LEG_IDS, "thermal": binding(0.2, True, 0.4), "loads": loads(True)},
    ]


def clips():
    """🎞 Carrying off the blanket, and a kitchen use cycle of the drawer and both doors."""
    use = "in-use"
    base = {"lift": {"dy": -BL}, "blanket": {"dx": 0.75}}
    k = lambda t, extra: {"t": t, "pose": use, "ease": "smooth", "bones": {**base, **extra}}
    return [
        {"id": "carry", "title": T("Carry off the blanket", "Von der Decke wegtragen"), "description": T("The two movers take hold under the outer side panels and lift the run level by 0.55 m; the blanket stays where it lay.", "Die zwei Träger greifen unter die äußeren Seitenwände und heben die Zeile waagerecht um 0,55 m an; die Decke bleibt liegen."),
         "durationS": 4.0, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.0, "pose": "stored", "ease": "smooth"}, {"t": 3.0, "pose": "carried", "ease": "smooth"}, {"t": 4.0, "pose": "carried", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 1.3} for a in BLANKET] + [{"anchor": a, "fromS": 1.0, "toS": 4.0} for a in GRIPS],
         "phases": [{"id": "grip", "fromS": 0, "toS": 1.0, "title": T("Taking hold", "Zugreifen")}, {"id": "lift", "fromS": 1.0, "toS": 3.0, "title": T("Lifting", "Anheben")}, {"id": "carry", "fromS": 3.0, "toS": 4.0, "title": T("Carrying", "Tragen")}],
         "events": [{"t": 1.0, "kind": "contact-on", "title": T("Hands under the side panels", "Hände unter den Seitenwänden"), "target": "grip-right-front"}, {"t": 1.3, "kind": "contact-off", "title": T("Legs leave the blanket", "Füße verlassen die Decke"), "target": "blanket-front"}, {"t": 1.0, "kind": "motion", "title": T("Lift starts", "Anheben beginnt")}],
         "tracks": []},
        {"id": "use-doors-and-drawer", "title": T("Drawer and doors in use", "Schublade und Türen im Gebrauch"), "description": T("Cutlery is taken from the drawer (out 420 mm and back), the sink door opens to the waste bin and closes, then the door under the drawer opens for a pot and closes.", "Besteck wird aus der Schublade genommen (420 mm heraus und zurück), die Spülentür öffnet zum Mülleimer und schließt, dann öffnet die Tür unter der Schublade für einen Topf und schließt."),
         "durationS": 10.0, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [k(0, {}), k(1.2, {"drawer": {"dx": 0.42}}), k(2.4, {"drawer": {"dx": 0.42}}), k(3.4, {}), k(4.6, {"door-a": {"twistDeg": 95}}), k(5.8, {"door-a": {"twistDeg": 95}}), k(6.8, {}), k(7.8, {"door-b": {"twistDeg": -95}}), k(8.8, {"door-b": {"twistDeg": -95}}), k(10.0, {})],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 10.0} for a in LEG_IDS],
         "phases": [{"id": "drawer", "fromS": 0, "toS": 3.4, "title": T("Drawer out and in", "Schublade heraus und hinein")}, {"id": "sink-door", "fromS": 3.4, "toS": 6.8, "title": T("Sink door", "Spülentür")}, {"id": "pot-door", "fromS": 6.8, "toS": 10.0, "title": T("Door under the drawer", "Tür unter der Schublade")}],
         "events": [{"t": 0.0, "kind": "motion", "title": T("Drawer pulled", "Schublade gezogen"), "target": "drawer"}, {"t": 3.4, "kind": "motion", "title": T("Sink door opened", "Spülentür geöffnet"), "target": "door-a"}, {"t": 6.8, "kind": "motion", "title": T("Door under the drawer opened", "Tür unter der Schublade geöffnet"), "target": "door-b"}],
         "tracks": []},
    ]


def region(rid, en, de, kind, mat, area, length, geom, sky, layer_mat, thickness, lam, rho, c, extra=None):
    """🔥 A passive region with one layer of its material."""
    hc = round(rho * c * thickness, 1)
    r = {"id": rid, "title": T(en, de), "kind": kind, "model": "passive", "material": mat, "areaM2": AREA.get(rid, area), "characteristicLengthM": length, "geometry": geom, "skyViewFactor": sky,
         "layers": [{"id": f"{rid}-layer", "title": T(f"{en} ({thickness * 1000:g} mm)", f"{de} ({thickness * 1000:g} mm)"), "kind": "timber" if "chipboard" in layer_mat or "mdf" in layer_mat or "hpl" in layer_mat else ("metal" if layer_mat in ("stainless-steel-sink", "chrome-brass") else ("polymer" if layer_mat == "pp-plastic" else "insulation")), "material": layer_mat,
                     "resistanceM2KW": round(thickness / lam, 5), "heatCapacityJM2K": hc, "source": "literature", "reference": "iso-10456-2007"}],
         "heatCapacityJM2K": hc, "source": "authored-estimate", "reference": "iso-10456-2007"}
    if extra:
        r.update(extra)
    return r


def thermal():
    """🌡 Seven passive regions in the depot hall (15 degC, a skylight sun patch on the worktop); furniture draws no power."""
    return {
        "environment": {"airTempC": 15, "meanRadiantTempC": 14, "relativeHumidity": 0.6, "windMS": 0.15, "windDirectionDeg": 0, "solarWM2": 250, "diffuseFraction": 0.3, "pressurePa": 101325},
        "passengers": [], "operating": [], "emitters": [],
        "regions": [
            region("carcass", "Carcasses: side panels, bottoms, rails, shelf, HDF backs and drawer box", "Korpusse: Seitenwände, Böden, Traversen, Fachboden, HDF-Rückwände und Schubkasten", "surface", "melamine-chipboard-aged", 3.0, 0.72, "vertical-plate", 0.4, "melamine-chipboard-aged", 0.018, 0.13, 650, 1700, {"ratedPowerW": 0, "ratedPowerBasis": "passive"}),
            region("fronts", "Doors, drawer front and handles", "Türen, Schubladenblende und Griffe", "surface", "foil-mdf-front", 1.0, 0.72, "vertical-plate", 0.4, "foil-mdf-front", 0.019, 0.14, 750, 1700),
            region("worktop", "Laminate worktop with the swollen front lip", "Laminat-Arbeitsplatte mit gequollener Vorderkante", "surface", "hpl-worktop", 0.8, 0.6, "horizontal-plate-up", 0.6, "hpl-worktop", 0.038, 0.14, 680, 1700),
            region("sink", "Stainless sink: rim, drainer and bowl", "Edelstahlspüle: Rand, Abtropffläche und Becken", "surface", "stainless-steel-sink", 0.8, 0.4, "horizontal-plate-up", 0.6, "stainless-steel-sink", 0.0008, 15, 7900, 500),
            region("tap", "Single-lever mixer with the cut hoses", "Einhebelmischer mit den abgeschnittenen Schläuchen", "surface", "chrome-brass", 0.05, 0.045, "cylinder", 0.5, "chrome-brass", 0.003, 110, 8500, 380),
            region("plinth", "Plinth panel, eight legs and clips", "Sockelblende, acht Füße und Clips", "surface", "melamine-chipboard-aged", 0.5, 0.146, "vertical-plate", 0.1, "melamine-chipboard-aged", 0.016, 0.13, 650, 1700),
            region("blanket", "Moving blanket", "Möbeldecke", "surface", "moving-blanket", 1.45, 0.58, "horizontal-plate-up", 0.5, "moving-blanket", 0.012, 0.045, 380, 1300),
        ],
        "assumptions": ASSUMPTIONS_THERMAL,
    }


PENS = {
    "50": {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.5},
    "100": {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35},
    "200": {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25},
    "500": {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18},
}
RANGES = {"50": (20, 75), "100": (75, 150), "200": (150, 350), "500": (350, 1000)}


def main():
    """🧾 Builds the four scales, the rig with the 1:50 masses and writes the asset document."""
    scales = {d: build(d) for d in ("50", "100", "200", "500")}
    rig_doc, mass = rig(scales["50"].masses)
    reps, vols = {}, {}
    for d, sc in scales.items():
        geometry = {v: sorted(shapes, key=lambda s: s["z"]) for v, shapes in sc.views.items()}
        reps[d] = {"status": "authored", "denominator": int(d), "validRange": {"minDenominator": RANGES[d][0], "maxDenominator": RANGES[d][1]}, "pens": PENS[d], "hatches": [], "geometry": geometry, "decisions": REPRESENTATION_DECISIONS[d]}
        vols[d] = {"status": "authored", "denominator": int(d), "body": sc.body, "thermal": sc.thermal, "structure": sc.structure, "decisions": VOLUME_DECISIONS[d]}
    asset = {
        "id": "used-kitchen-base-cabinet", "category": "furniture",
        "title": T("Used Kitchen Base Cabinet with Worktop", "Gebrauchter Küchenunterschrank mit Arbeitsplatte"),
        "summary": TEXTS["summary"],
        "tags": ["used", "household", "second-hand", "kitchen", "base-cabinet", "sink", "worktop", "chipboard", "laminate", "stainless-steel", "flat-pack", "water-damage", "storage"],
        "provenance": {"author": "Semio staffage content (used household objects)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": TEXTS["notes"]},
        "dimensions": {"heightM": 1.071, "widthM": 0.609, "depthM": 1.2, "massKg": mass},
        "materials": MATERIALS, "rig": rig_doc, "volumes": vols, "anchors": anchors(), "representations": reps, "symbols": [],
        "poses": poses(), "clips": clips(), "thermal": thermal(),
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "assumptions": ASSUMPTIONS_STRUCTURE, "clinical": False},
        "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n")
    print(f"[DEBUG] wrote 🔣️.json: mass {mass} kg, solids " + ", ".join(f"1:{d} {len(s.body)}/{len(s.thermal)}/{len(s.structure)}" for d, s in scales.items()))


if __name__ == "__main__":
    main()
