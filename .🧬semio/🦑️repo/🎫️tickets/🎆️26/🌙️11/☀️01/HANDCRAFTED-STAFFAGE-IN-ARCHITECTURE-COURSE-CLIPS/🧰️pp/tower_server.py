"""🖥️ pp: write the handcrafted catalog asset tower-server (single office tower server, 90 W typical, 200 W rated) and its manifest entry."""
import copy
import json
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
CAT = ROOT / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
OUT = CAT / "🔌️appliances/tower-server/🔣️.json"
monitor = json.loads((CAT / "🔌️appliances/desktop-monitor/🔣️.json").read_text(encoding="utf-8"))
mon = monitor["asset"]


def t(en, de):
    return {"en": en, "de": de}


def material(mid, en, de, e, a, rho, k, c, source="authored-estimate", ref="iso-10456-2007"):
    return {"id": mid, "title": t(en, de), "emissivity": e, "solarAbsorptivity": a, "densityKgM3": rho, "conductivityWmK": k,
            "specificHeatJKgK": c, "source": source, "reference": ref}


def region(rid, en, de, mat, area, length, level_w, layer, thickness, k, cap, kind="surface", geometry="vertical-plate", sky=0.5):
    layer_kind = "metal" if k > 10 else "polymer"
    body = {"id": rid, "title": t(en, de), "kind": kind, "model": "operating" if level_w else "passive", "material": mat, "areaM2": area,
            "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": sky,
            "layers": [{"id": layer[0], "title": t(layer[1], layer[2]), "kind": layer_kind, "material": mat, "thicknessM": thickness,
                        "resistanceM2KW": round(thickness / k, 6), "heatCapacityJM2K": cap, "source": "authored-estimate", "reference": "iso-10456-2007"}],
            "heatCapacityJM2K": cap}
    if level_w:
        body["operating"] = {"key": "power", "heatW": level_w}
    body["source"] = "authored-estimate"
    body["reference"] = "wilkins-hosni-2011"
    return body


STEEL = dict(k=50.0, cap=3800)
regions = [
    region("side-panel", "Side panels, powder-coated sheet steel", "Seitenwände, pulverbeschichtetes Stahlblech", "chassis-steel", 0.38, 0.45, 22,
           ("sheet", "Powder-coated steel sheet 0.8 mm", "Pulverbeschichtetes Stahlblech 0,8 mm"), 0.0008, 50.0, 3800),
    region("cpu-grille", "Perforated side grille over the processor heat sink", "Gelochtes Seitengitter über dem Prozessorkühler", "chassis-steel", 0.02, 0.12, 4,
           ("grille", "Perforated steel 0.8 mm", "Lochblech 0,8 mm"), 0.0008, 50.0, 1900),
    region("front-bezel", "Front bezel with the air intake", "Frontblende mit Lufteinlass", "bezel-abs", 0.09, 0.4, 5,
           ("bezel", "ABS bezel 2.5 mm over a mesh", "ABS-Blende 2,5 mm über einem Gitter"), 0.0025, 0.17, 3300),
    region("psu-vent", "Power supply with its exhaust grille", "Netzteil mit Abluftgitter", "chassis-steel", 0.04, 0.16, 8,
           ("psu-shell", "Power supply sheet 0.8 mm", "Netzteilblech 0,8 mm"), 0.0008, 50.0, 3800),
    region("rear-panel", "Rear panel with the fan exhaust and the ports", "Rückwand mit Lüfterausblas und Anschlüssen", "chassis-steel", 0.07, 0.4, 10,
           ("rear-sheet", "Steel sheet 0.8 mm", "Stahlblech 0,8 mm"), 0.0008, 50.0, 3800),
    region("feet", "Four rubber feet", "Vier Gummifüße", "rubber-foot", 0.002, 0.04, 0,
           ("foot", "Rubber 12 mm", "Gummi 12 mm"), 0.012, 0.15, 21600, geometry="horizontal-plate-down", sky=0.1),
]

materials = [
    material("chassis-steel", "Powder-coated steel", "Pulverbeschichteter Stahl", 0.9, 0.7, 7850, 50, 470),
    material("bezel-abs", "Black ABS front bezel", "Schwarze ABS-Frontblende", 0.9, 0.9, 1050, 0.17, 1400),
    copy.deepcopy(mon["materials"][-1]),
]

notes_en = ("Side elevation of a single office tower server (pedestal class, for example one processor socket and two disks), 0.45 m high, 0.20 m wide and 0.45 m deep, 11 kg: "
            "a powder-coated steel case on four rubber feet, a perforated front bezel for the intake air, a perforated side panel over the processor heat sink, "
            "the power supply with its grille at the rear top and one rear fan. The sizes are rounded from this product class (authored estimate, no single product). "
            "The 200 W at level 1 are the rated load, the typical office load of 90 W is level 0.45 (an authored estimate in the 60 to 150 W class of measured small servers, "
            "Wilkins and Hosni 2011), standby 6 W follows the idle state of a powered-off but connected machine. All electrical power ends as heat in the room: about 51 W at the rated load "
            "leave through the case surfaces, the other 149 W leave with the exhaust air of the rear fan (0.016 m3/s, 7.8 K).")
notes_de = ("Seitenansicht eines einzelnen Büro-Tower-Servers (Pedestal-Klasse, zum Beispiel ein Prozessorsockel und zwei Festplatten), 0,45 m hoch, 0,20 m breit und 0,45 m tief, 11 kg: "
            "ein pulverbeschichtetes Stahlgehäuse auf vier Gummifüßen, eine gelochte Frontblende für die Zuluft, eine gelochte Seitenwand über dem Prozessorkühler, "
            "das Netzteil mit seinem Gitter hinten oben und ein Lüfter an der Rückwand. Die Maße sind aus dieser Produktklasse gerundet (Schätzwert, kein einzelnes Produkt). "
            "Die 200 W auf Stufe 1 sind die Nennlast, die typische Bürolast von 90 W entspricht Stufe 0,45 (Schätzwert in der Klasse von 60 bis 150 W gemessener Kleinserver, "
            "Wilkins und Hosni 2011), der Bereitschaftsbetrieb mit 6 W folgt dem Zustand einer ausgeschalteten, aber vernetzten Maschine. Alle elektrische Leistung endet als Wärme im Raum: "
            "bei Nennlast verlassen etwa 51 W das Gehäuse über die Oberflächen, die übrigen 149 W gehen mit der Abluft des Rücklüfters (0,016 m3/s, 7,8 K) ab.")

provenance = copy.deepcopy(mon["provenance"])
provenance["references"] = [r for r in provenance["references"] if r["id"] != "energy-star-displays-v8"]
provenance["references"].append({"id": "energy-star-servers-v4", "citation": "ENERGY STAR Program Requirements for Computer Servers, Version 4.0 (2019) (idle state power of one-socket servers)."})
provenance["references"].append({"id": "ashrae-tc99-2021", "citation": "ASHRAE TC 9.9 (2021). Thermal Guidelines for Data Processing Environments, 5th ed. (air flow and temperature rise of IT equipment)."})
provenance["notes"] = t(notes_en, notes_de)
provenance["author"] = "Semio staffage content (pp)"

bones = [
    {"id": "case", "parent": None, "title": t("Case with feet and components", "Gehäuse mit Füßen und Komponenten"), "x": 0, "y": 0, "angleDeg": 0,
     "lengthM": 0.45, "joint": {"type": "fixed"},
     "mass": {"kg": 11.0, "comX": -0.02, "comY": 0.2, "source": "authored-estimate", "distribution": "point"}},
]
anchors = [
    {"id": "feet-contact", "bone": "case", "x": 0, "y": 0, "kind": "contact", "title": t("Feet on the floor", "Füße auf dem Boden"), "support": "contact",
     "footprintLengthM": 0.4, "footprintWidthM": 0.17, "frictionCoefficient": 0.6},
    {"id": "rear-outlet", "bone": "case", "x": -0.225, "y": 0.3, "kind": "emitter", "title": t("Rear fan exhaust", "Abluft des Rücklüfters")},
    {"id": "case-top", "bone": "case", "x": -0.1, "y": 0.45, "kind": "emitter", "title": t("Warm air above the case", "Warme Luft über dem Gehäuse")},
]

level_title = [
    (0, t("Off", "Aus")), (0.03, t("Standby (6 W)", "Bereitschaft (6 W)")), (0.45, t("Typical office load (90 W)", "Typische Büroauslastung (90 W)")),
    (1, t("Rated load (200 W)", "Nennlast (200 W)")),
]
operating = [{"id": "power", "title": t("Electrical power", "Elektrische Leistung"), "levels": [{"v": v, "title": ti} for v, ti in level_title]}]

emitters = [
    {"id": "rear-exhaust", "kind": "fan", "title": t("Exhaust air of the rear fan", "Abluft des Rücklüfters"), "anchor": "rear-outlet", "directionDeg": 180,
     "diameterM": 0.12, "operatingKey": "power", "volumeFlowM3S": 0.016, "temperatureRiseK": 7.8, "vapourKgPerKg": 0,
     "source": "authored-estimate", "reference": "ashrae-tc99-2021"},
    {"id": "case-plume", "kind": "plume", "title": t("Free convection above the warm case", "Freie Konvektion über dem warmen Gehäuse"), "anchor": "case-top",
     "directionDeg": 90, "diameterM": 0.3, "region": "psu-vent", "source": "authored-estimate", "reference": "incropera-2018"},
]
assumptions = [
    t("Every watt ends as heat in the room: at level 1 the case surfaces release 22 W (side panels), 4 W (processor grille), 5 W (front bezel), 8 W (power supply), 10 W (rear panel), "
      "together 49 W; the rear fan carries the other 151 W as 0.016 m3/s of air 7.8 K warmer than the room (0.016 m3/s x 1.2 kg/m3 x 1005 J/(kg K) x 7.8 K). The typical load of 90 W is level 0.45.",
      "Jedes Watt endet als Wärme im Raum: bei Stufe 1 geben die Gehäuseflächen 22 W (Seitenwände), 4 W (Prozessorgitter), 5 W (Frontblende), 8 W (Netzteil) und 10 W (Rückwand) ab, "
      "zusammen 49 W; der Rücklüfter führt die übrigen 151 W als 0,016 m3/s um 7,8 K wärmere Luft ab (0,016 m3/s x 1,2 kg/m3 x 1005 J/(kg K) x 7,8 K). Die typische Last von 90 W ist Stufe 0,45."),
    t("The air crosses the case from the front intake to the rear fan, perpendicular to the side elevation; the exhaust emitter sits at the rear wall and points backwards.",
      "Die Luft durchströmt das Gehäuse vom Einlass vorn zum Lüfter hinten, senkrecht zur Seitenansicht; der Abluft-Emitter sitzt an der Rückwand und zeigt nach hinten."),
]
thermal = {"environment": mon["thermal"]["environment"], "passengers": [], "operating": operating, "regions": regions, "emitters": emitters, "assumptions": assumptions}


def pose_thermal(level, plume, fan):
    fractions = {"side-panel": 1, "cpu-grille": 1, "front-bezel": 1, "psu-vent": 1, "rear-panel": 1}
    regs = {rid: {"exposedFraction": 1, "sunlitFraction": 0} for rid in fractions}
    regs["feet"] = {"exposedFraction": 0.9, "sunlitFraction": 0, "contact": {"fraction": 0.05, "tempC": 21}}
    return {"regions": regs, "emitters": {"rear-exhaust": fan, "case-plume": plume}, "operating": {"power": level}}


def pose(pid, ti, desc, level, plume, fan):
    return {"id": pid, "title": ti, "description": desc, "bones": {}, "ground": "auto", "contacts": ["feet-contact"], "thermal": pose_thermal(level, plume, fan), "loads": []}


poses = [
    pose("typical", t("Typical office load", "Typische Büroauslastung"),
         t("90 W: the fan blows air 3.5 K warmer out of the rear, the power supply grille and the processor grille are the warm spots, the case sides are a few kelvin above the room.",
           "90 W: der Lüfter bläst um 3,5 K wärmere Luft nach hinten aus, Netzteilgitter und Prozessorgitter sind die warmen Stellen, die Gehäuseseiten liegen einige Kelvin über der Raumtemperatur."), 0.45, 1, 1),
    pose("rated", t("Rated load", "Nennlast"),
         t("200 W: every surface runs warmer, the exhaust air is 7.8 K above the room.", "200 W: alle Flächen werden wärmer, die Abluft liegt 7,8 K über der Raumtemperatur."), 1, 1, 1),
    pose("standby", t("Standby", "Bereitschaft"), t("6 W: the case is within a kelvin of the room, the fan stands still.", "6 W: das Gehäuse liegt innerhalb eines Kelvin der Raumtemperatur, der Lüfter steht."), 0.03, 0, 0),
    pose("off", t("Switched off", "Ausgeschaltet"), t("Unplugged, room temperature.", "Vom Netz getrennt, Raumtemperatur."), 0, 0, 0),
]


def clip(cid, ti, desc, dur, loop, keys, phases, events, tracks):
    return {"id": cid, "title": ti, "description": desc, "durationS": dur, "fps": 1, "loop": loop, "rootMotion": "none",
            "keys": [{"t": k, "pose": p, "ease": "step"} for k, p in keys], "contacts": [{"anchor": "feet-contact", "fromS": 0, "toS": dur}],
            "phases": [{"id": i, "fromS": a, "toS": b, "title": tt} for i, a, b, tt in phases], "events": events,
            "tracks": [{"target": tg, "ease": "step", "keys": [{"t": tt, "v": v} for tt, v in ks]} for tg, ks in tracks]}


clips = [
    clip("power-up", t("Power up and warm up", "Einschalten und aufwärmen"),
         t("Cold start: the server is switched on after five seconds, loads to the typical 90 W and the case warms up over five minutes.",
           "Kaltstart: der Server wird nach fünf Sekunden eingeschaltet, läuft mit typischen 90 W und das Gehäuse wärmt sich über fünf Minuten auf."),
         300, False, [(0, "off"), (5, "typical"), (300, "typical")],
         [("cold", 0, 5, t("Off", "Aus")), ("warm-up", 5, 300, t("On, warming up", "An, wärmt auf"))],
         [{"t": 5, "kind": "operating", "title": t("Switched on", "Eingeschaltet"), "target": "power"}],
         [("operating:power", [(0, 0), (5, 0.45)]), ("emitter:rear-exhaust", [(0, 0), (5, 1)]), ("emitter:case-plume", [(0, 0), (5, 1)])]),
    clip("load-surge", t("Load surge to rated", "Lastspitze bis Nennlast"),
         t("The server runs at the typical 90 W, takes a batch job after two seconds and rises to 200 W for four seconds before it settles back; the fan keeps its speed, a looping clip for the exhaust air.",
           "Der Server läuft mit typischen 90 W, übernimmt nach zwei Sekunden einen Stapelauftrag und steigt für vier Sekunden auf 200 W, bevor er sich wieder beruhigt; der Lüfter behält seine Drehzahl, ein Schleifenclip für die Abluft."),
         8, True, [(0, "typical"), (2, "rated"), (6, "typical"), (8, "typical")],
         [("typical", 0, 2, t("Typical load", "Typische Last")), ("surge", 2, 6, t("Rated load", "Nennlast")), ("settle", 6, 8, t("Back to typical", "Zurück zu typisch"))],
         [{"t": 2, "kind": "operating", "title": t("Batch job starts", "Stapelauftrag startet"), "target": "power"}],
         [("operating:power", [(0, 0.45), (2, 1), (6, 0.45)])]),
]

structure = {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": ["feet-contact"],
             "assumptions": [t("A rigid 11 kg body on four rubber feet; its centre of mass lies 0.02 m behind the case centre, inside the 0.4 m by 0.17 m footprint.",
                               "Ein starrer Körper von 11 kg auf vier Gummifüßen; sein Schwerpunkt liegt 0,02 m hinter der Gehäusemitte, innerhalb der Standfläche von 0,4 m mal 0,17 m.")],
             "clinical": False}

PENS = {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}


def rect(sid, x, y, w, h, z, tone, region_, mat, role="body", r=None, filled=True):
    s = {"id": sid, "bone": "case", "role": role, "z": z, "filled": filled, "tone": tone, "kind": "rect", "x": x, "y": y, "w": w, "h": h}
    if r:
        s["r"] = r
    s["region"] = region_
    s["material"] = mat
    return s


def path(sid, d, z, tone, region_=None, mat=None, regions=None, role="body", filled=True):
    s = {"id": sid, "bone": "case", "role": role, "z": z, "filled": filled, "tone": tone, "kind": "path", "d": d}
    if regions:
        s["regions"] = regions
    else:
        s["region"] = region_
    if mat:
        s["material"] = mat
    return s


def lines(sid, segments, z, region_):
    d = " ".join(f"M {a:.4f} {b:.4f} L {c:.4f} {e:.4f}" for a, b, c, e in segments)
    return {"id": sid, "bone": "case", "role": "detail", "z": z, "filled": False, "kind": "path", "d": d, "region": region_}


CASE_PATH = "M -0.225 0.012 L 0.225 0.012 L 0.225 0.443 C 0.225 0.448 0.222 0.45 0.217 0.45 L -0.217 0.45 C -0.222 0.45 -0.225 0.448 -0.225 0.443 Z"
S = "side-panel"
ALL_CASE = ["side-panel", "rear-panel", "front-bezel", "psu-vent", "cpu-grille"]

el50 = [
    rect("foot-front", 0.16, 0, 0.04, 0.012, 3, 0.15, "feet", "rubber-foot"),
    rect("foot-rear", -0.2, 0, 0.04, 0.012, 3, 0.15, "feet", "rubber-foot"),
    path("case-side", CASE_PATH, 3.1, 0.55, S, "chassis-steel"),
    rect("front-bezel", 0.2, 0.024, 0.025, 0.414, 3.3, 0.18, "front-bezel", "bezel-abs"),
    rect("rear-panel", -0.225, 0.012, 0.012, 0.438, 3.2, 0.5, "rear-panel", "chassis-steel"),
    rect("psu-vent", -0.225, 0.33, 0.026, 0.108, 3.4, 0.38, "psu-vent", "chassis-steel"),
    rect("cpu-grille", -0.07, 0.202, 0.17, 0.13, 3.4, 0.38, "cpu-grille", "chassis-steel"),
    lines("grille-slits", [(-0.04 + 0.03 * i, 0.212, -0.04 + 0.03 * i, 0.322) for i in range(5)], 3.6, "cpu-grille"),
    lines("psu-slits", [(-0.225, 0.342 + 0.009 * i, -0.199, 0.342 + 0.009 * i) for i in range(11)], 3.6, "psu-vent"),
    lines("intake-slits", [(0.2, 0.062 + 0.03 * i, 0.225, 0.062 + 0.03 * i) for i in range(12)], 3.6, "front-bezel"),
    {"id": "fan-ring", "bone": "case", "role": "detail", "z": 3.6, "filled": False, "kind": "ellipse", "cx": -0.222, "cy": 0.2, "rx": 0.003, "ry": 0.05, "region": "rear-panel"},
]
fr50 = [
    rect("foot-left", 0.06, 0, 0.03, 0.012, 3, 0.15, "feet", "rubber-foot"),
    rect("foot-right", -0.09, 0, 0.03, 0.012, 3, 0.15, "feet", "rubber-foot"),
    rect("front-bezel", -0.1, 0.012, 0.2, 0.438, 3.1, 0.18, "front-bezel", "bezel-abs", r=0.005),
    rect("wall-left", 0.092, 0.012, 0.008, 0.438, 3.2, 0.55, S, "chassis-steel"),
    rect("wall-right", -0.1, 0.012, 0.008, 0.438, 3.2, 0.55, S, "chassis-steel"),
    rect("intake-mesh", -0.085, 0.05, 0.17, 0.34, 3.3, 0.3, "front-bezel", "bezel-abs", role="detail"),
    lines("intake-ribs", [(-0.085, 0.062 + 0.03 * i, 0.085, 0.062 + 0.03 * i) for i in range(12)], 3.5, "front-bezel"),
    rect("drive-bay", -0.07, 0.395, 0.14, 0.03, 3.4, 0.4, "front-bezel", "bezel-abs", role="detail"),
    rect("power-button", 0.05, 0.03, 0.012, 0.012, 3.4, 0.7, "front-bezel", "bezel-abs", role="detail"),
]
pl50 = [
    rect("top-sheet", -0.225, -0.1, 0.45, 0.2, 3.1, 0.55, S, "chassis-steel", r=0.004),
    rect("front-bezel", 0.2, -0.1, 0.025, 0.2, 3.3, 0.18, "front-bezel", "bezel-abs"),
    rect("rear-panel", -0.225, -0.1, 0.012, 0.2, 3.2, 0.5, "rear-panel", "chassis-steel"),
    rect("psu-vent", -0.2, -0.085, 0.09, 0.17, 3.4, 0.38, "psu-vent", "chassis-steel"),
    lines("psu-top-slits", [(-0.195 + 0.015 * i, -0.075, -0.195 + 0.015 * i, 0.075) for i in range(6)], 3.6, "psu-vent"),
    rect("cpu-edge", -0.07, 0.092, 0.17, 0.008, 3.5, 0.38, "cpu-grille", "chassis-steel", role="detail"),
]
el100 = [
    rect("foot-front", 0.15, 0, 0.05, 0.012, 3, 0.15, "feet", "rubber-foot"),
    rect("foot-rear", -0.2, 0, 0.05, 0.012, 3, 0.15, "feet", "rubber-foot"),
    path("case-side", CASE_PATH, 3.1, 0.55, S, "chassis-steel"),
    rect("front-bezel", 0.195, 0.024, 0.03, 0.414, 3.3, 0.18, "front-bezel", "bezel-abs"),
    rect("rear-panel", -0.225, 0.012, 0.014, 0.438, 3.2, 0.5, "rear-panel", "chassis-steel"),
    rect("psu-vent", -0.225, 0.33, 0.03, 0.108, 3.4, 0.38, "psu-vent", "chassis-steel"),
    rect("cpu-grille", -0.07, 0.202, 0.17, 0.13, 3.4, 0.38, "cpu-grille", "chassis-steel"),
    lines("grille-slits", [(-0.04 + 0.04 * i, 0.212, -0.04 + 0.04 * i, 0.322) for i in range(4)], 3.6, "cpu-grille"),
]
fr100 = [
    rect("foot-left", 0.05, 0, 0.04, 0.012, 3, 0.15, "feet", "rubber-foot"),
    rect("foot-right", -0.09, 0, 0.04, 0.012, 3, 0.15, "feet", "rubber-foot"),
    rect("front-bezel", -0.1, 0.012, 0.2, 0.438, 3.1, 0.18, "front-bezel", "bezel-abs", r=0.006),
    rect("wall-left", 0.088, 0.012, 0.012, 0.438, 3.2, 0.55, S, "chassis-steel"),
    rect("wall-right", -0.1, 0.012, 0.012, 0.438, 3.2, 0.55, S, "chassis-steel"),
    rect("intake-mesh", -0.08, 0.05, 0.16, 0.34, 3.3, 0.3, "front-bezel", "bezel-abs", role="detail"),
]
pl100 = [
    rect("top-sheet", -0.225, -0.1, 0.45, 0.2, 3.1, 0.55, S, "chassis-steel", r=0.005),
    rect("front-bezel", 0.195, -0.1, 0.03, 0.2, 3.3, 0.18, "front-bezel", "bezel-abs"),
    rect("rear-panel", -0.225, -0.1, 0.014, 0.2, 3.2, 0.5, "rear-panel", "chassis-steel"),
    rect("psu-vent", -0.2, -0.085, 0.09, 0.17, 3.4, 0.38, "psu-vent", "chassis-steel"),
    rect("cpu-edge", -0.07, 0.088, 0.17, 0.012, 3.5, 0.38, "cpu-grille", "chassis-steel", role="detail"),
]
el200 = [
    path("plinth", "M -0.2 0 L 0.2 0 L 0.2 0.024 L -0.2 0.024 Z", 3, 0.3, "feet", "rubber-foot"),
    path("case", CASE_PATH, 3.1, 0.5, regions=ALL_CASE, mat="chassis-steel"),
]
fr200 = [
    path("plinth", "M -0.08 0 L 0.08 0 L 0.08 0.024 L -0.08 0.024 Z", 3, 0.3, "feet", "rubber-foot"),
    rect("case", -0.1, 0.012, 0.2, 0.438, 3.1, 0.4, None, "chassis-steel"),
]
fr200[1].pop("region")
fr200[1]["regions"] = ["front-bezel", "side-panel"]
pl200 = [
    rect("top", -0.225, -0.1, 0.45, 0.2, 3.1, 0.5, None, "chassis-steel"),
]
pl200[0].pop("region")
pl200[0]["regions"] = ALL_CASE

def decisions(sil, ret, mer, omi, thin, hat, sty, ov):
    return {k: t(*v) for k, v in zip(("silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"), (sil, ret, mer, omi, thin, hat, sty, ov))}


rep50 = {"status": "authored", "denominator": 50, "validRange": {"minDenominator": 20, "maxDenominator": 75}, "pens": PENS, "hatches": [], "geometry": {"elevation": el50, "front": fr50, "plan": pl50},
         "decisions": decisions(
             ("A 0.45 m side view, 9 mm tall on paper: a square-ish steel case on two visible rubber feet, a dark front bezel on the right and a rear panel with the power supply on the left.",
              "Eine Seitenansicht von 0,45 m, auf Papier 9 mm hoch: ein fast quadratisches Stahlgehäuse auf zwei sichtbaren Gummifüßen, rechts eine dunkle Frontblende, links eine Rückwand mit dem Netzteil."),
             ("Overall height 0.45 m, depth 0.45 m, the 12 mm feet, the 25 mm front bezel, the processor grille and the power supply grille at their true places.",
              "Gesamthöhe 0,45 m, Tiefe 0,45 m, die 12 mm hohen Füße, die 25 mm dicke Frontblende, das Prozessorgitter und das Netzteilgitter an ihren wahren Stellen."),
             ("The two feet at each end are one side-view foot each; the case side is one sheet.", "Die je zwei Füße an einem Ende sind in der Seitenansicht ein Fuß; die Gehäuseseite ist ein Blech."),
             ("Ports, drive bays, the power button and the cable, which lie outside the silhouette or behind the side sheet.", "Anschlüsse, Laufwerksschächte, Einschalter und Kabel, die außerhalb der Silhouette oder hinter dem Seitenblech liegen."),
             ("The intake slits are 25 mm long detail strokes of 0.13 mm; the grille slits are 0.11 m long.", "Die Einlassschlitze sind 25 mm lange Detailstriche von 0,13 mm; die Gitterschlitze sind 0,11 m lang."),
             ("None.", "Keine."),
             ("Case 0.55, bezel 0.18, grilles 0.38; one body pen of 0.18 mm and one detail pen of 0.13 mm.", "Gehäuse 0,55, Blende 0,18, Gitter 0,38; ein Körperstift von 0,18 mm und ein Detailstift von 0,13 mm."),
             ("Every thermal region has its own shape: side panels, bezel, rear panel, power supply grille, processor grille and the feet.", "Jede thermische Region hat eine eigene Form: Seitenwände, Blende, Rückwand, Netzteilgitter, Prozessorgitter und die Füße."))}
rep100 = {"status": "authored", "denominator": 100, "validRange": {"minDenominator": 75, "maxDenominator": 150}, "pens": PENS, "hatches": [], "geometry": {"elevation": el100, "front": fr100, "plan": pl100},
          "decisions": decisions(
              ("A 0.45 m side view, 4.5 mm tall on paper: a case with a dark front bezel, a rear panel, a power supply block and a grille patch.", "Eine Seitenansicht von 0,45 m, auf Papier 4,5 mm hoch: ein Gehäuse mit dunkler Frontblende, Rückwand, Netzteilblock und Gitterfeld."),
              ("Depth, height, feet, the bezel, the rear panel, the power supply and the processor grille.", "Tiefe, Höhe, Füße, Blende, Rückwand, Netzteil und Prozessorgitter."),
              ("Both feet at each end are one block each.", "Je zwei Füße an einem Ende sind ein Block."),
              ("Intake slits, power supply slits and the fan ring.", "Einlassschlitze, Netzteilschlitze und der Lüfterring."),
              ("Bezel and rear panel are widened to 2.5 mm and 1.4 mm so that they reach the feature width.", "Blende und Rückwand sind auf 25 mm und 14 mm verbreitert, damit sie die Mindestbreite erreichen."),
              ("None.", "Keine."), ("Case 0.55, bezel 0.18, grilles 0.38; four slit strokes of 0.13 mm in the grille patch.", "Gehäuse 0,55, Blende 0,18, Gitter 0,38; vier Schlitzstriche von 0,13 mm im Gitterfeld."),
              ("As at 1:50: one shape per thermal region.", "Wie bei 1:50: eine Form je thermischer Region."))}
rep200 = {"status": "authored", "denominator": 200, "validRange": {"minDenominator": 150, "maxDenominator": 350}, "pens": {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
          "hatches": [], "geometry": {"elevation": el200, "front": fr200, "plan": pl200},
          "decisions": decisions(
              ("A block 2.25 mm wide and 2.25 mm high on paper: a case on a low plinth.", "Ein Block von 2,25 mm Breite und 2,25 mm Höhe auf Papier: ein Gehäuse auf einem niedrigen Sockel."),
              ("Height 0.45 m, depth 0.45 m.", "Höhe 0,45 m, Tiefe 0,45 m."),
              ("Case sheet, bezel, rear panel, power supply and processor grille are one case shape listing all five regions; the four feet are one plinth.", "Gehäuseblech, Blende, Rückwand, Netzteil und Prozessorgitter sind eine Gehäuseform mit allen fünf Regionen; die vier Füße sind ein Sockel."),
              ("Slits, fan ring, grilles as separate shapes.", "Schlitze, Lüfterring und Gitter als eigene Formen."),
              ("The feet are drawn as a 24 mm plinth so that they reach the feature width.", "Die Füße sind als 24 mm hoher Sockel gezeichnet, damit sie die Mindestbreite erreichen."),
              ("None.", "Keine."), ("Case 0.5, plinth 0.3; one body pen of 0.18 mm.", "Gehäuse 0,5, Sockel 0,3; ein Körperstift von 0,18 mm."),
              ("The case shape lists the five case regions as merged regions; the feet are their own region.", "Die Gehäuseform führt die fünf Gehäuseregionen zusammengeführt; die Füße sind eine eigene Region."))}
rep500 = {"status": "unavailable", "denominator": 500,
          "reason": t("At 1:500 the server is a 0.9 mm square with no printable feature; its thermal regions cannot be told apart.", "Bei 1:500 ist der Server ein Quadrat von 0,9 mm ohne druckbares Merkmal; seine thermischen Regionen sind nicht zu unterscheiden."),
          "limitingFeature": t("Front bezel depth of 25 mm", "Tiefe der Frontblende von 25 mm"), "limitingSizeM": 0.025,
          "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "scale", "denominator": 100}]}

DIST_EN = "A single office tower server: 90 W at typical load and 200 W rated, mostly carried away by the rear fan; the 5 kW rack is the data-centre counterpart."
DIST_DE = "Ein einzelner Büro-Tower-Server: 90 W bei typischer Last und 200 W Nennlast, überwiegend vom Rücklüfter abgeführt; das 5-kW-Rack ist das Rechenzentrums-Gegenstück."
asset = {"id": "tower-server", "category": "appliances",
         "title": t("Office Tower Server", "Büro-Tower-Server"),
         "summary": t("A single office tower server, 0.45 m high and 0.45 m deep: about 90 W at typical load and 200 W rated, a quarter of it through the case surfaces and the rest with the exhaust air of its rear fan, 6 W in standby.",
                      "Ein einzelner Büro-Tower-Server, 0,45 m hoch und 0,45 m tief: etwa 90 W bei typischer Last und 200 W Nennlast, ein Viertel davon über die Gehäuseflächen, der Rest mit der Abluft des Rücklüfters, 6 W im Bereitschaftsbetrieb."),
         "tags": ["server", "computer", "office-equipment", "internal-gain", "electrical", "heat-source"],
         "provenance": provenance, "dimensions": {"heightM": 0.45, "widthM": 0.45, "depthM": 0.2, "massKg": 11.0},
         "materials": materials, "rig": {"kind": "mechanical", "root": "case", "bones": bones}, "anchors": anchors, "symbols": [],
         "representations": {"500": rep500, "200": rep200, "100": rep100, "50": rep50},
         "poses": poses, "clips": clips, "thermal": thermal, "structure": structure,
         "presentation": {"defaultPose": "typical", "defaultClip": "power-up", "defaultScale": 50},
         "exports": mon["exports"], "completion": "authored"}
asset["representations"] = {k: asset["representations"][k] for k in ("50", "100", "200", "500")}
doc = {"$schema": monitor["$schema"], "version": monitor["version"], "asset": asset}
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "🧰️pa"))
import jio

OUT.parent.mkdir(parents=True, exist_ok=True)
OUT.write_bytes(jio.dumps(doc).encode("utf-8"))
manifest_path = CAT / "🔣️.json"
raw = manifest_path.read_bytes().decode("utf-8")
if '"id": "tower-server"' not in raw:
    entry = f"""    {{
      "id": "tower-server",
      "category": "appliances",
      "title": {json.dumps(asset["title"], ensure_ascii=False)},
      "heightM": 0.45,
      "widthM": 0.45,
      "poses": {json.dumps([p["id"] for p in poses])},
      "clips": {json.dumps([c["id"] for c in clips])},
      "representations": {{ "50": "authored", "100": "authored", "200": "authored", "500": "unavailable" }},
      "symbols": [],
      "analytics": ["thermal", "transport", "structural"],
      "exports": {json.dumps(mon["exports"])},
      "distinction": {{
        "en": {json.dumps(DIST_EN, ensure_ascii=False)},
        "de": {json.dumps(DIST_DE, ensure_ascii=False)}
      }},
      "completion": "authored"
    }},
"""
    marker = '    {\n      "id": "pendant-lamp",\n      "category": "appliances",'
    assert marker in raw
    raw = raw.replace(marker, entry + marker, 1)
    manifest_path.write_bytes(raw.encode("utf-8"))
    print("manifest entry added")
print("ok", OUT)
