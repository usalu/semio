import sys
from build import *

REL = "🏗️machinery/forklift"
KEYS = [
    opkey("traction", "Traction drive", "Fahrantrieb", [(0, "Off", "Aus"), (0.05, "Ready, standing", "Bereit, stehend"), (0.5, "Driving", "Fahren"), (1, "Full power", "Volle Leistung")]),
    opkey("lifting", "Lifting and tilting", "Heben und Neigen", [(0, "Off", "Aus"), (0.1, "Tilting", "Neigen"), (0.8, "Lifting a load", "Last heben"), (1, "Full load", "Volllast")]),
    opkey("rolling", "Rolling speed", "Rollgeschwindigkeit", [(0, "Standstill", "Stillstand"), (0.5, "About 8 km/h", "Etwa 8 km/h"), (1, "About 16 km/h", "Etwa 16 km/h")]),
    opkey("braking", "Braking", "Bremsen", [(0, "Not braking", "Nicht gebremst"), (0.3, "Service braking", "Betriebsbremsung"), (1, "Full stop from 16 km/h", "Vollbremsung aus 16 km/h")]),
]
steel = lambda mm, hc, m="steel": layer("plate", "Steel %d mm" % mm, "Stahl %d mm" % mm, "metal", m, mm / 1000, mm / 1000 / 50, hc, source="literature", ref="incropera-2011")
REGIONS = [
    region("chassis-paint", "Painted frame and hood", "Lackierter Rahmen und Haube", "surface", "passive", "paint", 8, 1.2, "vertical-plate", 0.5, 12000, [steel(3, 12000, "paint")], ref="ashrae-2021"),
    region("guard", "Overhead guard", "Fahrerschutzdach", "surface", "passive", "paint", 4, 0.05, "cylinder", 0.8, 3000, []),
    region("battery-pack", "Battery case", "Batteriekasten", "surface", "operating", "battery-case", 4.5, 1, "vertical-plate", 0.4, 25000, [steel(5, 25000, "battery-case")], ("traction", 540)),
    region("counterweight", "Counterweight", "Gegengewicht", "surface", "passive", "cast-iron", 3.5, 1, "vertical-plate", 0.5, 100000, [layer("iron", "Cast iron 0.15 m (active part)", "Gusseisen 0,15 m (wirksamer Teil)", "metal", "cast-iron", 0.15, 0.15 / 50, 100000, source="authored-estimate")], ref="ashrae-2021"),
    region("drive-motor", "Traction motor housing", "Fahrmotorgehäuse", "engine", "operating", "drive-electronics", 0.9, 0.4, "cylinder", 0.2, 15000, [], ("traction", 430)),
    region("controller", "Power electronics housing", "Leistungselektronikgehäuse", "engine", "operating", "drive-electronics", 0.5, 0.3, "vertical-plate", 0.3, 8000, [], ("traction", 150)),
    region("hydraulics", "Tilt cylinders", "Neigezylinder", "surface", "operating", "hydraulic", 0.4, 0.1, "cylinder", 0.7, 5000, [], ("lifting", 100)),
    region("mast", "Mast channels", "Mastprofile", "surface", "passive", "steel", 5, 0.15, "vertical-plate", 0.8, 20000, [steel(8, 20000)], ref="ashrae-2021"),
    region("fork-gear", "Carriage and forks", "Gabelträger und Gabeln", "surface", "passive", "steel", 1.6, 0.5, "vertical-plate", 0.7, 150000, [steel(40, 150000)], ref="ashrae-2021"),
    region("tyres", "Tyre treads and sidewalls", "Reifenlaufflächen und -flanken", "tyre", "operating", "rubber", 1.6, 0.5, "cylinder", 0.4, 40000, [layer("tread", "Rubber tread", "Gummilauffläche", "polymer", "rubber", 0.02, 0.02 / 0.2, 40000, source="authored-estimate", ref="wong-2008")], ("rolling", 380), ref="wong-2008"),
    region("brakes", "Brake drums seen through the rim holes", "Bremstrommeln, durch die Felgenöffnungen gesehen", "surface", "operating", "cast-iron", 0.4, 0.3, "vertical-plate", 0.2, 18000, [], ("braking", 27000)),
    region("pallet", "Pallet timber", "Palettenholz", "surface", "passive", "wood", 2, 1.2, "vertical-plate", 0.6, 21000, [layer("timber", "Pine boards 25 mm", "Kieferbretter 25 mm", "timber", "wood", 0.025, 0.025 / 0.13, 21000, source="literature", ref="incropera-2011")], ref="vdi-2198"),
    region("cargo", "Wrapped goods", "Folienverpackte Ware", "surface", "passive", "goods", 5, 1, "vertical-plate", 0.6, 8000, [], source="illustrative"),
]
EMITTERS = [emitter("motor-vent", "vent", "Motor and electronics cooling air", "Kühlluft von Motor und Elektronik", "motor-vent", 180, 0.25, "traction", 0.12, 20, 0.0)]
DEFAULTS = {"chassis-paint": (1, 0.5), "guard": (1, 0.7), "battery-pack": (0.6, 0), "counterweight": (1, 0.4), "drive-motor": (0.4, 0), "controller": (0.5, 0), "hydraulics": (1, 0.3), "mast": (1, 0.5), "fork-gear": (1, 0.4), "tyres": (1, 0.1), "brakes": (0.5, 0), "pallet": (1, 0.3), "cargo": (1, 0.4)}
CONTACTS = {"tyres": (0.15, 22)}
P = lambda t, l, r, b: {"operating": {"traction": t, "lifting": l, "rolling": r, "braking": b}, "emitters": {"motor-vent": 1 if t > 0 else 0}}
POSES = {"lowered-forks": P(0.05, 0, 0, 0.1), "lifted-load": P(0.05, 0.8, 0, 0.1), "tilted-mast": P(0.4, 0.1, 0.5, 0), "approach": P(0.3, 0, 0.4, 0.3), "travel-accelerated": P(0.7, 0.1, 0.8, 0),
         "travel-cruised": P(0.5, 0.1, 0.8, 0), "travel-stopped": P(0.05, 0, 0.2, 0.9)}


def fix_clips(clips):
    for c in clips:
        c["tracks"] = [t for t in c["tracks"] if not t["target"].startswith("emitter:")]
        for t in c["tracks"]:
            t["target"] = {"operating:lift": "operating:lifting"}.get(t["target"], t["target"])
        if c["id"] == "lift-pallet":
            c["events"].insert(0, {"t": 0.2, "kind": "ignition", "title": tx("Key on, drives ready", "Schlüssel an, Antriebe bereit"), "target": "motor-vent"})
            c["tracks"].append({"target": "emitter:motor-vent", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 0.2, "v": 1}]})
            c["tracks"].append({"target": "operating:braking", "ease": "smooth", "keys": [{"t": 0, "v": 0.3}, {"t": 3, "v": 0}, {"t": 10, "v": 0}]})
            c["tracks"].append({"target": "operating:rolling", "ease": "smooth", "keys": [{"t": 0, "v": 0.4}, {"t": 3, "v": 0}, {"t": 10, "v": 0}]})
        if c["id"] == "drive-with-load":
            c["events"].append({"t": 7.2, "kind": "shutdown", "title": tx("Key off", "Schlüssel aus"), "target": "motor-vent"})
            c["tracks"].append({"target": "emitter:motor-vent", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 7.2, "v": 0}]})
            c["tracks"].append({"target": "operating:lifting", "ease": "smooth", "keys": [{"t": 0, "v": 0.1}, {"t": 7.5, "v": 0.1}]})
            for t in c["tracks"]:
                if t["target"] == "operating:traction":
                    t["keys"] = [{"t": 0, "v": 0.2}, {"t": 1.5, "v": 0.85}, {"t": 2.5, "v": 0.55}, {"t": 5.5, "v": 0.5}, {"t": 6.5, "v": 0.1}, {"t": 7.5, "v": 0}]
        if c["id"] == "lift-pallet":
            c["tracks"].append({"target": "operating:traction_unused", "ease": "step", "keys": []}) if False else None


def reps(a, old):
    R = {}
    O = {s["id"]: s for s in old}
    take = lambda ids: convert_v1_shapes([O[i] for i in ids])
    body = lambda regs, d=None: path("chassis-body", "chassis", "body", 2, d or O["chassis-body"]["d"], 0.6, True, regions=regs, region=None, material="paint")
    st = lambda id, bone, d, region, pen, z=3: path(id, bone, "body", z, d, 0.3, False, region=region, material="paint", pen=pen)
    guard500 = path("guard-frame", "chassis", "body", 3, "M -1.0,1.05 L -1.0,2.125 L 0.15,2.125 M -0.05,0.96 L -0.05,2.1", 0.3, False, region="guard", material="paint", pen=0.15)
    battery = lambda: rect("battery-case", "battery", "body", 2.2, -0.55, -0.02, 1.0, 0.47, tone=0.4, region="battery-pack", material="battery-case")
    cw = lambda: path("counterweight-body", "counterweight", "body", 2.5, O["counterweight-body"]["d"], 0.4, True, region="counterweight", material="cast-iron")
    mast_o = lambda regs: rect("mast-outer-body", "mast-outer", "body", 3, -0.25, -0.07, 2.05, 0.14, tone=0.5, regions=regs, material="steel")
    mast_i = lambda: rect("mast-inner-body", "mast-inner", "body", 3.1, 0, -0.05, 1.6, 0.1, tone=0.5, region="mast", material="steel")
    carriage = lambda: rect("carriage-plate", "carriage", "body", 3.2, 0, -0.05, 0.6, 0.1, tone=0.5, region="fork-gear", material="steel")
    forks = lambda: [rect("fork-shank", "forks", "body", 3.3, 0, -0.09, 1.05, 0.09, tone=0.5, region="fork-gear", material="steel"), rect("fork-tine", "forks", "body", 3.3, 0, -1.07, 0.045, 0.98, tone=0.5, region="fork-gear", material="steel")]
    pallet500 = lambda: [rect("pallet-block", "pallet", "body", 3, 0, 0, 1.2, 0.144, tone=0.6, region="pallet", material="wood"), rect("pallet-goods", "pallet", "body", 3.1, 0.02, 0.144, 1.16, 0.9, 0.02, tone=0.75, region="cargo", material="goods")]
    wheel500 = lambda sfx, bone, r: ell("tyre-" + sfx, bone, "body", 4, 0, 0, r, tone=0.1, regions=["tyres", "brakes"], material="rubber")
    e500 = [body(["chassis-paint", "drive-motor", "controller"]), guard500, ell("beacon", "chassis", "body", 3, -0.5, 2.2, 0.06, tone=0.8, material="interior"), battery(), cw(), mast_o(["mast", "hydraulics"]), mast_i(), carriage()] + forks() + pallet500() + [wheel500("f", "wheel-f", 0.29), wheel500("r", "wheel-r", 0.23)]
    e500 = [s for s in e500]
    R["500"] = rep(500, *RANGES[500], PRINT_PENS[500], [], e500, decisions(
        tx("Counterbalance forklift as one body mass with battery and counterweight blocks, a line-drawn overhead guard, the mast, carriage and fork as thin solid bars, a pallet with goods block and two black wheel discs.", "Gegengewichtsstapler als eine Fahrzeugmasse mit Batterie- und Gegengewichtsblock, einem als Linienzeichnung gegebenen Fahrerschutzdach, Mast, Gabelträger und Gabel als dünne volle Balken, einer Palette mit Warenblock und zwei schwarzen Radscheiben."),
        tx("Body, guard, battery, counterweight, both mast stages, carriage, fork (backrest and tine), pallet block with goods, both wheels; the mast still tilts and lifts.", "Aufbau, Schutzdach, Batterie, Gegengewicht, beide Maststufen, Gabelträger, Gabel (Rückenlehne und Zinken), Palettenblock mit Ware, beide Räder; der Mast neigt und hebt sich weiter."),
        tx("Motor housing and electronics merge into the body fill; the tilt cylinders merge into the outer mast; the pallet feet and deck are one block; rim, holes and drums merge into the wheel disc.", "Motorgehäuse und Elektronik gehen in der Aufbaufläche auf; die Neigezylinder im äußeren Mast; Palettenfüße und Deck sind ein Block; Felge, Löcher und Trommeln gehen in der Radscheibe auf."),
        tx("Seat, steering wheel, controls, beacon, tilt cylinder bones' shapes, slats of the guard, fork backrest bars.", "Sitz, Lenkrad, Bedienelemente, Rundumleuchte, Formen der Neigezylinderknochen, Schutzdachlamellen, Gabelrückenstreben."),
        tx("The guard posts (0.06 m) are lines of 0.15 mm (0.075 m); the mast bars and fork are real size (0.1 to 0.14 m = 0.2 to 0.3 mm, fork tine 0.045 m = 0.09 mm is drawn as outline only).", "Die Schutzdachstützen (0,06 m) sind Linien von 0,15 mm (0,075 m); Mastbalken und Gabel in Realgröße (0,1 bis 0,14 m = 0,2 bis 0,3 mm, die Gabelzinke mit 0,045 m = 0,09 mm nur als Kontur)."),
        tx("None.", "Keine."), tx("Body mid-tone, battery and counterweight darker, goods light, tyres black.", "Aufbau mittlerer Ton, Batterie und Gegengewicht dunkler, Ware hell, Reifen schwarz."),
        tx("Merged regions follow the host shapes; battery, counterweight, guard, mast, fork gear, pallet, goods and tyres are separate.", "Zusammengefasste Regionen folgen den Trägerformen; Batterie, Gegengewicht, Schutzdach, Mast, Gabelzeug, Palette, Ware und Reifen sind getrennt.")))
    guard200 = [ell("beacon", "chassis", "detail", 3, -0.5, 2.2, 0.06, tone=0.8, material="interior"), rect("guard-roof", "chassis", "body", 3, -1.2, 2.1, 1.35, 0.06, tone=0.4, region="guard", material="paint"), cap("guard-post-rear", "chassis", "detail", 3, -1.0, 1.05, -1.0, 2.1, 0.03, tone=0.4, region="guard", material="paint"), cap("guard-post-front", "chassis", "detail", 3, -0.1, 0.96, -0.05, 2.1, 0.03, tone=0.4, region="guard", material="paint")]
    pallet200 = [path("pallet-feet", "pallet", "detail", 3, O["pallet-feet"]["d"], 0.6, True, region="pallet", material="wood"), rect("pallet-deck", "pallet", "body", 3, 0, 0.1, 1.2, 0.06, tone=0.6, region="pallet", material="wood"), rect("pallet-goods", "pallet", "body", 3.1, 0.02, 0.16, 1.16, 0.884, 0.02, tone=0.75, region="cargo", material="goods")]
    wheel200 = lambda sfx, bone, r, rr: [ell("tyre-" + sfx, bone, "body", 4, 0, 0, r, tone=0.1, region="tyres", material="rubber"), ell("rim-" + sfx, bone, "detail", 4.2, 0, 0, rr, tone=0.5, region="brakes", material="paint"), ell("hub-" + sfx, bone, "detail", 4.4, 0, 0, rr * 0.35, tone=0.7, material="paint")]
    e200 = [body(None) if False else path("chassis-body", "chassis", "body", 2, O["chassis-body"]["d"], 0.6, True, region="chassis-paint", material="paint"),
            rect("drive-cover", "chassis", "detail", 2.1, -0.3, 0.58, 0.5, 0.1, tone=0.3, region="drive-motor", material="drive-electronics"), path("cowl-dash", "chassis", "detail", 2.1, O["cowl-dash"]["d"], 0.3, True, region="controller", material="drive-electronics")] + guard200 + [
            battery(), cw(), cap("tilt-barrel", "tilt-cyl", "detail", 3.4, 0, 0, 0.45, 0, 0.04, tone=0.4, region="hydraulics", material="hydraulic"), cap("tilt-rod-shape", "tilt-rod", "detail", 3.45, 0, 0, 0.45, 0, 0.025, tone=0.7, region="hydraulics", material="hydraulic"),
            mast_o(["mast"]), mast_i(), carriage()] + forks() + pallet200 + wheel200("f", "wheel-f", 0.29, 0.17) + wheel200("r", "wheel-r", 0.23, 0.13)
    e200[-6]["region"] = "tyres"
    R["200"] = rep(200, *RANGES[200], PRINT_PENS[200], [], e200, decisions(
        tx("Body with hood, drive cover and dash, overhead guard with roof and two posts, battery and counterweight, the two-stage mast with carriage, fork and tilt cylinder, a pallet with feet, deck and goods, two wheels with rim and hub.", "Aufbau mit Haube, Antriebsabdeckung und Armaturenbrett, Fahrerschutzdach mit Dach und zwei Stützen, Batterie und Gegengewicht, zweistufiger Mast mit Gabelträger, Gabel und Neigezylinder, eine Palette mit Füßen, Deck und Ware, zwei Räder mit Felge und Nabe."),
        tx("Everything that carries a thermal region: motor cover, dash electronics, tilt cylinder, mast stages, fork gear, pallet and goods.", "Alles, was eine thermische Region trägt: Motorabdeckung, Armaturenelektronik, Neigezylinder, Maststufen, Gabelzeug, Palette und Ware."),
        tx("Rim and drum are one lighter disc (brakes region); fork backrest and tine are two bars.", "Felge und Trommel sind eine hellere Scheibe (Region Bremsen); Gabelrückenlehne und Zinke sind zwei Balken."),
        tx("Seat, steering wheel, beacon, guard slats, battery lines, goods courses and straps.", "Sitz, Lenkrad, Rundumleuchte, Schutzdachlamellen, Batterielinien, Warenlagen und Gurte."),
        tx("Guard posts 0.06 m = 0.3 mm real; the pallet deck is drawn 0.06 m (real 0.044 m).", "Schutzdachstützen 0,06 m = 0,3 mm real; das Palettendeck ist 0,06 m (real 0,044 m) gezeichnet."),
        tx("None.", "Keine."), tx("Body mid-tone, tyres black, goods light.", "Aufbau mittlerer Ton, Reifen schwarz, Ware hell."), tx("Every region has its own shape.", "Jede Region hat eine eigene Form.")))
    e100 = [s for s in e200 if s["id"] not in ("hub-f", "hub-r")] + take(["steering-wheel", "seat-cushion", "seat-back", "controls", "holes-f", "holes-r", "hub-f", "hub-r", "goods-strap", "pallet-feet"])
    e100 = [s for s in e100 if not (s["id"] == "pallet-feet" and "lod" not in s and s.get("bone") == "pallet" and e100.count(s) > 1)]
    seen, e100b = set(), []
    for s in e100:
        if s["id"] in seen:
            continue
        seen.add(s["id"])
        e100b.append(s)
    e100 = e100b
    R["100"] = rep(100, *RANGES[100], PRINT_PENS[100], [], e100, decisions(
        tx("The 1:200 truck plus seat, steering wheel, controls, beacon, guard slats, hole circles in the rims and goods straps.", "Der Stapler von 1:200 mit Sitz, Lenkrad, Bedienelementen, Rundumleuchte, Schutzdachlamellen, Lochkreisen in den Felgen und Warengurten."),
        tx("Seat, wheel, controls, beacon, rim holes (brakes region), straps.", "Sitz, Lenkrad, Bedienelemente, Rundumleuchte, Felgenlöcher (Region Bremsen), Gurte."),
        tx("Goods courses and fork backrest bars are not drawn.", "Warenlagen und Gabelrückenstreben werden nicht gezeichnet."), tx("Battery lines, chassis skirt, vents.", "Batterielinien, Rahmenschürze, Lüftungsschlitze."),
        tx("None thickened.", "Nichts verstärkt."), tx("None.", "Keine."), tx("As 1:200.", "Wie 1:200."), tx("As 1:200.", "Wie 1:200.")))
    e50 = convert_v1_shapes(old)
    R["50"] = rep(50, *RANGES[50], PRINT_PENS[50], [], e50, decisions(
        tx("Full side elevation of the first version.", "Vollständige Seitenansicht der ersten Fassung."), tx("Everything: courses, straps, slats, backrest bars, vents.", "Alles: Lagen, Gurte, Lamellen, Rückenstreben, Schlitze."), tx("Nothing merged.", "Nichts zusammengefasst."),
        tx("Hoses and cables (not modelled).", "Schläuche und Kabel (nicht modelliert)."), tx("Fork tine 0.045 m = 0.9 mm.", "Gabelzinke 0,045 m = 0,9 mm."), tx("None.", "Keine."), tx("Three line weights 0.35 / 0.25 / 0.13 mm.", "Drei Linienstärken 0,35 / 0,25 / 0,13 mm."), tx("All regions at real extent.", "Alle Regionen in realer Ausdehnung.")))
    return R


NOTES = tx("Electric counterbalance forklift, 2.5 t: no exhaust; waste heat comes from the traction motor, power electronics, battery case and hydraulic lifting system. Steering is a vertical-axis motion and not an elevation joint. The pallet chain bones are massless guide bones; pallet and goods are drawn at every scale.",
           "Elektro-Gegengewichtsstapler, 2,5 t: kein Abgas; Abwärme stammt von Fahrmotor, Leistungselektronik, Batteriekasten und der hydraulischen Hubanlage. Die Lenkung ist eine Bewegung um die Hochachse und kein Gelenk der Ansicht. Die Palettenkette sind masselose Führungsknochen; Palette und Ware sind in jedem Maßstab gezeichnet.")
ASSUMPTIONS = [tx("Heat release at level 1: battery case 0.54 kW (+12 K), traction motor 0.43 kW (+40 K), electronics 0.15 kW (+30 K), hydraulic cylinders 0.1 kW, tyres 0.38 kW (about 16 km/h), brake drums 27 kW for a full stop from 16 km/h (40 kJ in 1.5 s) on 16 kg of iron (18 kJ/(m2 K) on 0.4 m2).",
                  "Wärmefreisetzung bei Stufe 1: Batteriekasten 0,54 kW (+12 K), Fahrmotor 0,43 kW (+40 K), Elektronik 0,15 kW (+30 K), Hydraulikzylinder 0,1 kW, Reifen 0,38 kW (etwa 16 km/h), Bremstrommeln 27 kW bei einer Vollbremsung aus 16 km/h (40 kJ in 1,5 s) auf 16 kg Eisen (18 kJ/(m2 K) auf 0,4 m2)."),
               tx("Cooling air at level 1: 0.12 m3/s through a 0.25 m opening in the counterweight, 20 K above ambient, dry; flow and temperature rise scale with the traction level. Indoor reference: no solar load in the default environment; pose sun fractions apply once the viewer enables sun.",
                  "Kühlluft bei Stufe 1: 0,12 m3/s durch eine 0,25-m-Öffnung im Gegengewicht, 20 K über der Umgebung, trocken; Volumenstrom und Temperaturerhöhung skalieren mit der Fahrstufe. Innenraum-Referenz: keine Sonnenlast in der Standardumgebung; die Sonnenanteile der Haltungen gelten, sobald die Betrachtung Sonne einschaltet.")]
SPEC = dict(sky=18, ground=20, operating=KEYS, regions=REGIONS, emitters=EMITTERS, assumptions=ASSUMPTIONS, defaults=DEFAULTS, contacts=CONTACTS, poses=POSES, fix_clips=fix_clips, reps=reps, notes=NOTES, category="machinery")
if __name__ == "__main__":
    a = assemble("forklift", REL, SPEC)
    if len(sys.argv) > 1:
        previews(a, "fork", poses=sys.argv[2:] or None, dens=[int(x) for x in sys.argv[1].split(",")], zoom=(3,))
