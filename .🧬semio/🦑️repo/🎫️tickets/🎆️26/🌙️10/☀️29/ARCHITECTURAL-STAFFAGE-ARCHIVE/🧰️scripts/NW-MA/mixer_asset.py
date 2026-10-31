"""Assembles and writes concrete-mixer-truck."""
import math
from mixer import *

EXPORTS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"]
CONC_N = 21600 * 9.80665

materials = [mat_paint("paint-steel", "Painted structural steel", "Lackierter Baustahl", 0.55), mat_paint("drum-paint", "White drum paint on steel", "Weißer Trommellack auf Stahl", 0.35),
             MAT_GALV, MAT_GLASS, MAT_RUBBER, MAT_IRON, MAT_EXHAUST]

R = lambda *a, **k: region(*a, **k)
regions = [
    R("cab", "Cab paintwork", "Lackierung des Fahrerhauses", "surface", "passive", "paint-steel", 24, 2.2, "vertical-plate", 0.6, [steel_layer("sheet", 0.0015)], 5400, ref="ashrae-2021"),
    R("cab-glazing", "Cab glazing", "Verglasung des Fahrerhauses", "glazing", "passive", "glass", 4.5, 1.2, "vertical-plate", 0.5, [layer("pane", "Safety glass 6 mm", "Sicherheitsglas 6 mm", "glass", "glass", 0.006, 0.006, 12600, None, "literature", "incropera-2011")], 12600, ref="ashrae-2021"),
    R("chassis", "Frame, pedestals and tanks", "Rahmen, Lagerböcke und Tanks", "surface", "passive", "paint-steel", 12, 1.5, "vertical-plate", 0.4, [steel_layer("rail", 0.008)], 28900, ref="ashrae-2021"),
    R("engine-bay", "Engine compartment shell under the cab", "Motorraumhülle unter dem Fahrerhaus", "engine", "operating", "paint-steel", 3.2, 1.5, "vertical-plate", 0.3, [steel_layer("shell", 0.002)], 7200, op=("engine-load", 4000), ref="heywood-2018"),
    R("exhaust-stack", "Exhaust pipe and silencer outlet", "Auspuffrohr und Schalldämpferausgang", "exhaust", "operating", "exhaust-steel", 0.8, 0.15, "cylinder", 0.8, [steel_layer("pipe", 0.0015, "exhaust-steel")], 5400, op=("engine-load", 5000), ref="heywood-2018"),
    R("drum", "Mixer drum shell", "Mischtrommelhülle", "surface", "passive", "drum-paint", 36, 2.4, "cylinder", 0.7, [steel_layer("shell", 0.005, "drum-paint")], 18000, ref="ashrae-2021"),
    R("drum-drive", "Hydraulic drum drive housing", "Hydraulikantriebsgehäuse der Trommel", "engine", "operating", "paint-steel", 2.5, 0.8, "vertical-plate", 0.3, [steel_layer("housing", 0.008)], 28900, op=("drum-drive", 1500)),
    R("hopper", "Charging hopper", "Einfülltrichter", "surface", "passive", "galvanised-steel", 3, 0.8, "vertical-plate", 0.7, [steel_layer("sheet", 0.003, "galvanised-steel")], 10800, ref="ashrae-2021"),
    R("chute", "Discharge chute and extension", "Entladerutsche und Verlängerung", "surface", "passive", "galvanised-steel", 4, 0.5, "horizontal-plate-up", 0.8, [steel_layer("sheet", 0.003, "galvanised-steel")], 10800, ref="ashrae-2021"),
    R("tyres", "Tyre sidewalls and treads", "Reifenflanken und Laufflächen", "tyre", "operating", "rubber", 10, 1.04, "cylinder", 0.5, [layer("rubber", "Tyre rubber 12 mm", "Reifengummi 12 mm", "polymer", "rubber", 0.012, 0.08, 22440, None, "literature", "incropera-2011")], 22440, op=("driving", 5000)),
    R("brakes", "Brake drums and hubs", "Bremstrommeln und Naben", "engine", "operating", "cast-iron", 2.4, 0.6, "cylinder", 0.4, [layer("iron", "Cast iron 15 mm", "Gusseisen 15 mm", "metal", "cast-iron", 0.015, 0.00033, 49680, None, "literature", "ashrae-2021")], 49680, op=("braking", 15000)),
]
RID = [r["id"] for r in regions]
DEF = {"cab": {"exposedFraction": 1, "sunlitFraction": 0.6}, "cab-glazing": {"exposedFraction": 1, "sunlitFraction": 0.5}, "chassis": {"exposedFraction": 1, "sunlitFraction": 0.15},
       "engine-bay": {"exposedFraction": 0.3, "sunlitFraction": 0}, "exhaust-stack": {"exposedFraction": 1, "sunlitFraction": 0.8}, "drum": {"exposedFraction": 1, "sunlitFraction": 0.65},
       "drum-drive": {"exposedFraction": 0.3, "sunlitFraction": 0.1}, "hopper": {"exposedFraction": 1, "sunlitFraction": 0.7}, "chute": {"exposedFraction": 1, "sunlitFraction": 0.6},
       "tyres": {"exposedFraction": 1, "sunlitFraction": 0.2, "contact": {"fraction": 0.06, "tempC": 32}}, "brakes": {"exposedFraction": 0.5, "sunlitFraction": 0}}
ops = [opkey("engine-load", "Engine load", "Motorlast", [(0, "Engine off", "Motor aus"), (0.15, "Idle", "Leerlauf"), (0.5, "Part load", "Teillast"), (1, "Rated power", "Nennleistung")]),
       opkey("drum-drive", "Drum drive", "Trommelantrieb", [(0, "Stopped", "Stillstand"), (0.2, "Agitating", "Rühren"), (0.7, "Discharging", "Entleeren"), (1, "Charging, full speed", "Beladen, volle Drehzahl")]),
       opkey("driving", "Road speed", "Fahrgeschwindigkeit", [(0, "Standing", "Stillstand"), (0.5, "Urban, 30 km/h", "Stadt, 30 km/h"), (1, "Road, 60 km/h", "Landstraße, 60 km/h")]),
       opkey("braking", "Braking", "Bremsen", [(0, "Released", "Gelöst"), (0.3, "Light braking", "Leichtes Bremsen"), (1, "Full braking", "Vollbremsung")])]
emitters = [
    emitter("exhaust-stack-jet", "exhaust", "Exhaust jet of the vertical stack", "Abgasstrahl des senkrechten Auspuffrohrs", "stack-mouth", 90, 0.15, "engine-load", 1.0, 340, 0.04, "heywood-2018"),
    emitter("radiator-air", "vent", "Radiator cooling air leaving under the cab", "Kühlerabluft unter dem Fahrerhaus", "radiator-outlet", 190, 0.9, "engine-load", 7.5, 30, 0.0),
]
anchors = [anchor(f"contact-{i}", f"wheel-{i}" if False else "chassis", x, 0.0, "contact", f"Tyre contact patch, axle {i}", f"Reifenaufstandsfläche, Achse {i}", "contact", 0.3, 0.6, 0.7, 90) for i, x in enumerate(AX, 1)]
anchors += [anchor("stack-mouth", "chassis", 2.32, 3.35, "emitter", "Exhaust stack mouth", "Auspuffmündung"), anchor("radiator-outlet", "chassis", 2.3, 0.75, "emitter", "Radiator air outlet", "Kühlerluftaustritt"),
            anchor("drum-cog", "drum", 2.3, 0.0, "load", "Concrete centre of gravity in the drum", "Betonschwerpunkt in der Trommel")]
CONTACTS = [a["id"] for a in anchors if a["kind"] == "contact"]


def th(op, em, over=None):
    return tbind(RID, DEF, over or {}, {"exhaust-stack-jet": em, "radiator-air": em}, op)


def load(f):
    return [{"id": "concrete", "kind": "carried", "title": tx("Fresh concrete in the drum (9 m3 at 2.4 t/m3 at full load)", "Frischbeton in der Trommel (9 m3 zu 2,4 t/m3 bei voller Ladung)"), "anchor": "drum-cog", "fxN": 0, "fyN": round(-CONC_N * f, 1), "source": "authored-estimate"}] if f > 0 else []


CH = {"chute": {"angleDeg": 165}, "chute-ext": {"angleDeg": -180}}
poses = [
    pose("transit-agitating", "Transit, drum agitating", "Fahrt, Trommel rührt", "Driving to site with the full drum turning slowly, chute folded up on the drum, engine at part load.", "Fahrt zur Baustelle mit voller, langsam drehender Trommel, Rutsche hochgeklappt auf der Trommel, Motor in Teillast.", {}, CONTACTS, th({"engine-load": 0.5, "drum-drive": 0.2, "driving": 0.5, "braking": 0}, 1), load(1)),
    pose("parked-off", "Parked, everything off", "Geparkt, alles aus", "Empty and washed out, engine off, drum still, chute stowed.", "Leer und ausgewaschen, Motor aus, Trommel steht, Rutsche verstaut.", {}, CONTACTS, th({"engine-load": 0, "drum-drive": 0, "driving": 0, "braking": 0}, 0)),
    pose("charging", "Charging at the plant", "Beladen im Werk", "Standing under the batching plant hopper, drum at full charging speed, engine at part load.", "Stehend unter dem Mischwerk-Trichter, Trommel mit voller Beladedrehzahl, Motor in Teillast.", {}, CONTACTS, th({"engine-load": 0.5, "drum-drive": 1, "driving": 0, "braking": 0}, 1, {"hopper": {"exposedFraction": 0.7, "sunlitFraction": 0.2}}), load(0.6)),
    pose("discharging", "Discharging over the chute", "Entleeren über die Rutsche", "Chute swung out and lowered, extension unfolded, drum turning at discharge speed, concrete half discharged.", "Rutsche ausgeschwenkt und abgesenkt, Verlängerung ausgeklappt, Trommel mit Entleerdrehzahl, Beton zur Hälfte entleert.", CH, CONTACTS, th({"engine-load": 0.5, "drum-drive": 0.7, "driving": 0, "braking": 0}, 1, {"chute": {"exposedFraction": 1, "sunlitFraction": 0.6, "wettedness": 0.8}}), load(0.5)),
    pose("chute-swing", "Chute swinging out", "Rutsche schwenkt aus", "Key pose of the discharge sequence: the chute is half way down, the extension half unfolded.", "Schlüsselpose der Entleerfolge: Rutsche auf halbem Weg nach unten, Verlängerung halb ausgeklappt.", {"chute": {"angleDeg": 80}, "chute-ext": {"angleDeg": -90}}, CONTACTS, th({"engine-load": 0.5, "drum-drive": 0.2, "driving": 0, "braking": 0}, 1), load(1), key=True),
]


def saw(t0, t1, T, dt, fps, spiral_on=lambda t: True):
    out, t = [], 0.0
    n = int(round((t1 - t0) / dt))
    for k in range(n + 1):
        t = round(t0 + k * dt, 6)
        ph = ((t - t0) % T) / T
        if k > 0 and abs(((t - t0) % T)) < 1e-9 and t > t0:
            out.append((round(t - 1.0 / fps, 6), PITCH * (1 - 1.0 / (T * fps)), "step"))
        out.append((t, PITCH * ph, "linear"))
    return out


def smooth(t, a, b):
    if t <= a:
        return 0.0
    if t >= b:
        return 1.0
    u = (t - a) / (b - a)
    return u * u * (3 - 2 * u)


loop_keys = []
sk = saw(0, 10, 10, 0.5, 30)
for t, dx, ease in sk[:-1]:
    loop_keys.append(key(t, "transit-agitating", ease, {"drum-spiral": {"dx": round(dx, 5)}}))
loop_keys.append(key(10.0, "transit-agitating", "linear", {"drum-spiral": {"dx": 0}}))

dkeys = []
pts = [(0.0, 0.0, "linear")]
spiral = saw(2.0, 14.0, 5.0, 0.5, 30)
times = sorted(set([round(0.5 * k, 6) for k in range(0, 33)]))
spt = {}
for t, dx, ease in spiral:
    spt[t] = (dx, ease)
alltimes = sorted(set(times) | set(spt))
for t in alltimes:
    dx, ease = spt.get(t, (0.0 if t < 2 else None, "linear"))
    if dx is None:
        if t > 14.0:
            dx = 0.0
        else:
            continue
    pn = "parked-off" if t < 1.0 else ("chute-swing" if 3.0 <= t < 5.0 else ("discharging" if t >= 5.0 and t < 14.5 else ("transit-agitating" if t < 5.0 else "parked-off")))
    ca = 165 * smooth(t, 2.5, 5.5) * (1 - smooth(t, 14.0, 16.0))
    ea = -180 * smooth(t, 3.5, 5.5) * (1 - smooth(t, 13.5, 15.0))
    dkeys.append(key(t, pn, ease, {"chute": {"angleDeg": round(ca, 3)}, "chute-ext": {"angleDeg": round(ea, 3)}, "drum-spiral": {"dx": round(dx, 5)}}))
dkeys.sort(key=lambda k: k["t"])

clips = [
    clip("agitate-at-rest", "Drum agitating at idle", "Trommel rührt im Leerlauf", "A 10 s loop of the full drum turning at 6 rpm while the engine idles: the spiral flight marks drift one pitch per revolution and the idle exhaust and radiator air keep flowing.",
         "Eine Schleife von 10 s mit voller Trommel bei 6 U/min bei Motorleerlauf: Die Wendelmarken wandern je Umdrehung eine Steigung, Leerlaufabgas und Kühlerluft strömen weiter.", 10, 30, True, loop_keys,
         [{"anchor": a, "fromS": 0, "toS": 10} for a in CONTACTS], [phase("agitate", 0, 10, "Agitating at 6 rpm", "Rühren mit 6 U/min")],
         [event(0, "operating", "Engine idling, drum agitating", "Motor im Leerlauf, Trommel rührt", "engine-load")],
         [track("operating:engine-load", [(0, 0.15), (10, 0.15)], "linear"), track("operating:drum-drive", [(0, 0.2), (10, 0.2)], "linear"), track("emitter:exhaust-stack-jet", [(0, 1)], "step"), track("emitter:radiator-air", [(0, 1)], "step")]),
    clip("discharge-sequence", "Start, discharge and shut down", "Anlassen, Entleeren und Abstellen", "A 16 s sequence: key on, drum starts, the chute swings out and the extension unfolds, 9 m3 flow out while the drum turns at discharge speed, chute stows, engine off.",
         "Eine Folge von 16 s: Zündung, Trommel läuft an, Rutsche schwenkt aus und Verlängerung klappt aus, 9 m3 fließen bei Entleerdrehzahl heraus, Rutsche wird verstaut, Motor aus.", 16, 30, False, dkeys,
         [{"anchor": a, "fromS": 0, "toS": 16} for a in CONTACTS],
         [phase("start", 0, 2, "Start-up", "Anlassen"), phase("swing", 2, 6, "Chute swings out", "Rutsche schwenkt aus"), phase("pour", 6, 13, "Pouring", "Entleeren"), phase("stow", 13, 16, "Stowing and shutdown", "Verstauen und Abstellen")],
         [event(0.5, "ignition", "Ignition", "Zündung", "exhaust-stack-jet"), event(2.0, "operating", "Drum drive engaged", "Trommelantrieb zugeschaltet", "drum-drive"), event(6.5, "load-on", "Concrete starts to leave the drum", "Beton verlässt die Trommel", "concrete"), event(15.5, "shutdown", "Engine off", "Motor aus", "exhaust-stack-jet")],
         [track("operating:engine-load", [(0, 0), (0.5, 0.15), (2, 0.5), (14, 0.5), (15.5, 0.15), (15.6, 0)]), track("operating:drum-drive", [(0, 0), (2, 0.2), (6, 0.7), (13, 0.7), (14, 0)]),
          track("emitter:exhaust-stack-jet", [(0, 0), (0.5, 1), (15.5, 0)], "step"), track("emitter:radiator-air", [(0, 0), (0.5, 1), (15.5, 0)], "step"), track("load:concrete", [(0, 1), (6.5, 1), (13, 0.05), (16, 0.05)]),
          track("region:chute:wettedness", [(0, 0), (6.5, 0), (7.5, 0.8), (13, 0.8), (14.5, 0.1)])]),
]


def asset():
    return {
        "id": "concrete-mixer-truck", "category": "machinery",
        "title": tx("Concrete mixer truck, 9 m3", "Fahrmischer, 9 m3"),
        "summary": tx("An 8x4 truck mixer of the 9 m3 class in side elevation: heavy chassis with cab-over-engine cab and vertical exhaust stack, a tilted mixer drum whose spiral flight marks drift with its rotation, charging hopper and a folding discharge chute with extension.",
                      "Ein Fahrmischer 8x4 der 9-m3-Klasse in der Seitenansicht: schweres Fahrgestell mit Frontlenker-Fahrerhaus und senkrechtem Auspuffrohr, eine geneigte Mischtrommel, deren Wendelmarken mit der Drehung wandern, Einfülltrichter und eine klappbare Entladerutsche mit Verlängerung."),
        "tags": ["machine", "truck-mixer", "concrete", "diesel", "construction", "wheeled", "rotating-drum"],
        "provenance": {"author": "Semio staffage content worker NW-MA", "method": "handcrafted", "created": "2026-10-29",
                       "references": references(["directive-96-53-ec", "heywood-2018", "incropera-2011", "ashrae-2021", "class-data-note"]),
                       "notes": tx("Class data of a 9 m3 truck mixer on an 8x4 heavy chassis (about 13.5 t unladen, 32 t gross, 330 kW diesel); all machine numbers are authored estimates from that class, dimensions respect the 2.55 m width limit of Directive 96/53/EC. The drum rotation is shown by a flight pattern that drifts along the drum axis (the rotation axis lies in the drawing plane); the drum, hopper and chute move rigidly with the rig.",
                                   "Klassendaten eines 9-m3-Fahrmischers auf schwerem 8x4-Fahrgestell (etwa 13,5 t leer, 32 t Gesamtgewicht, 330-kW-Diesel); alle Maschinenzahlen sind Schätzungen dieser Klasse, die Maße halten die Breitengrenze 2,55 m der Richtlinie 96/53/EG ein. Die Trommeldrehung wird durch ein längs der Trommelachse wanderndes Wendelmuster gezeigt (die Drehachse liegt in der Zeichenebene); Trommel, Trichter und Rutsche bewegen sich starr mit dem Rig.")},
        "dimensions": {"heightM": 3.95, "widthM": 9.0, "depthM": 2.55, "massKg": 13500},
        "materials": materials, "rig": rig.doc(), "anchors": anchors, "representations": build_reps(), "symbols": [],
        "poses": poses, "clips": clips,
        "thermal": {"environment": DEFAULT_ENV, "passengers": [], "operating": ops, "regions": regions, "emitters": emitters,
                    "assumptions": [tx("Engine class 330 kW (12.8 l diesel): exhaust about 0.5 kg/s at 360 C (rho 0.52 kg/m3) gives 1.0 m3/s through a 0.15 m tailpipe (57 m/s) and 0.04 kg/kg water vapour (Heywood 2018); flow and rise scale linearly with the engine level, so idle (0.15) emits 0.15 m3/s at +51 K.", "Motorklasse 330 kW (12,8-l-Diesel): Abgas etwa 0,5 kg/s bei 360 C (rho 0,52 kg/m3) ergibt 1,0 m3/s durch ein 0,15-m-Endrohr (57 m/s) und 0,04 kg/kg Wasserdampf (Heywood 2018); Volumenstrom und Erhöhung skalieren linear mit der Motorstufe, im Leerlauf (0,15) also 0,15 m3/s bei +51 K."),
                                    tx("Radiator: about 280 kW of coolant heat at rated power, +30 K of air gives 7.5 m3/s; the air leaves under the cab. Heat into the shells at level 1: engine compartment 4 kW, exhaust pipe 5 kW, drum drive housing 1.5 kW (20 percent of 7.5 kW hydraulic power), tyres 5 kW (the sidewall share of about 42 kW rolling resistance at 60 km/h), brake drums 15 kW (retarder and service brake). All authored estimates.", "Kühler: etwa 280 kW Kühlwasserwärme bei Nennleistung, +30 K Luft ergibt 7,5 m3/s; die Luft tritt unter dem Fahrerhaus aus. Wärme in die Hüllen bei Stufe 1: Motorraum 4 kW, Auspuffrohr 5 kW, Antriebsgehäuse 1,5 kW (20 Prozent von 7,5 kW Hydraulikleistung), Reifen 5 kW (Flankenanteil von etwa 42 kW Rollwiderstand bei 60 km/h), Bremstrommeln 15 kW (Dauerbremse und Betriebsbremse). Alles Schätzungen."),
                                    tx("The fresh concrete is a mass load only; its hydration heat and the drum wall contact are not modelled. Heat capacities are active shell masses (steel 5 mm 18 kJ/(m2 K)).", "Der Frischbeton ist nur eine Massenlast; Hydratationswärme und Wandkontakt der Trommel sind nicht modelliert. Wärmekapazitäten sind aktive Schalenmassen (Stahl 5 mm 18 kJ/(m2 K)).")]},
        "structure": structure("Four axle contact patches share the load as unilateral supports; the 21.6 t of concrete is a point load at the drum centre of gravity (9 m3 at 2.4 t/m3). The drum rotation is not an elevation joint; the flight marks are a drawing pattern.", "Vier Achsaufstandsflächen teilen die Last als einseitige Auflager; die 21,6 t Beton sind eine Einzellast im Trommelschwerpunkt (9 m3 zu 2,4 t/m3). Die Trommeldrehung ist kein Gelenk der Ansicht; die Wendelmarken sind ein Zeichenmuster."),
        "presentation": {"defaultPose": "transit-agitating", "defaultClip": "agitate-at-rest", "defaultScale": 200, "viewpoints": ["elevation"]},
        "exports": EXPORTS, "completion": "authored",
    }


if __name__ == "__main__":
    a = asset()
    for p in selfcheck(a):
        print("PROBLEM", p)
    print(write(a, "concrete-mixer-truck"))
