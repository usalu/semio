"""Assembles and writes diesel-generator."""
from genset import *

EXPORTS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"]
FUEL_N = 420 * 9.80665
materials = [mat_paint("container-steel", "Painted container steel (RAL 7035 light grey)", "Lackierter Containerstahl (RAL 7035 lichtgrau)", 0.45), MAT_EXHAUST, MAT_ALU, MAT_GALV]
R = region
regions = [
    R("enclosure-walls", "Acoustic enclosure walls and doors", "Schallschutzwände und Türen", "wall", "operating", "container-steel", 29, 2.4, "vertical-plate", 0.5, [layer("wool", "Mineral wool 50 mm (inside)", "Mineralwolle 50 mm (innen)", "insulation", "container-steel", 0.05, 1.25, 4000, None, "literature", "ashrae-2021"), steel_layer("skin", 0.002, "container-steel")], 7200, op=("engine-load", 800), ref="ashrae-2021"),
    R("roof", "Container roof", "Containerdach", "surface", "passive", "container-steel", 14, 2.4, "horizontal-plate-up", 1.0, [steel_layer("sheet", 0.002, "container-steel")], 7200, ref="ashrae-2021"),
    R("inlet-louvres", "Combustion and cooling air inlet louvres", "Eintrittslamellen für Verbrennungs- und Kühlluft", "surface", "passive", "aluminium", 4.2, 1.5, "vertical-plate", 0.5, [layer("alu", "Aluminium louvre 2 mm", "Aluminiumlamelle 2 mm", "metal", "aluminium", 0.002, 1e-5, 4860, None, "literature", "incropera-2011")], 4860, ref="incropera-2011"),
    R("skid-tank", "Base frame and fuel base tank", "Grundrahmen und Kraftstoff-Grundtank", "surface", "passive", "container-steel", 12, 1.5, "vertical-plate", 0.3, [steel_layer("plate", 0.005, "container-steel")], 200000, ref="ashrae-2021"),
    R("silencer", "Roof silencer casing (lagged)", "Dachschalldämpfer (gedämmt)", "exhaust", "operating", "exhaust-steel", 3.0, 0.55, "cylinder", 0.9, [steel_layer("shell", 0.003, "exhaust-steel")], 3000, op=("engine-load", 3000), ref="heywood-2018"),
    R("exhaust-stack", "Exhaust stack pipe", "Abgasrohr", "exhaust", "operating", "exhaust-steel", 0.3, 0.2, "cylinder", 1.0, [steel_layer("pipe", 0.002, "exhaust-steel")], 3600, op=("engine-load", 1200), ref="heywood-2018"),
    R("rain-cap", "Stack rain cap flap", "Regenklappe des Abgasrohrs", "surface", "passive", "galvanised-steel", 0.1, 0.3, "horizontal-plate-up", 1.0, [steel_layer("flap", 0.002, "galvanised-steel")], 7200, ref="ashrae-2021"),
    R("discharge-hood", "Radiator discharge hood", "Kühlerluft-Austrittshaube", "surface", "operating", "container-steel", 4, 1.1, "vertical-plate", 0.7, [steel_layer("hood", 0.002, "container-steel")], 7200, op=("cooling-fan", 2000), ref="ashrae-2021"),
    R("outlet-flap", "Discharge weather flap", "Wetterklappe des Luftaustritts", "surface", "passive", "galvanised-steel", 0.7, 0.63, "vertical-plate", 0.6, [steel_layer("flap", 0.002, "galvanised-steel")], 7200, ref="ashrae-2021"),
]
RID = [r["id"] for r in regions]
DEF = {"enclosure-walls": {"exposedFraction": 1, "sunlitFraction": 0.5}, "roof": {"exposedFraction": 1, "sunlitFraction": 1.0}, "inlet-louvres": {"exposedFraction": 1, "sunlitFraction": 0.4},
       "skid-tank": {"exposedFraction": 0.8, "sunlitFraction": 0.1, "contact": {"fraction": 0.05, "tempC": 30}}, "silencer": {"exposedFraction": 1, "sunlitFraction": 0.9}, "exhaust-stack": {"exposedFraction": 1, "sunlitFraction": 0.8},
       "rain-cap": {"exposedFraction": 1, "sunlitFraction": 1.0}, "discharge-hood": {"exposedFraction": 1, "sunlitFraction": 0.8}, "outlet-flap": {"exposedFraction": 1, "sunlitFraction": 0.5}}
ops = [opkey("engine-load", "Engine load", "Motorlast", [(0, "Engine off", "Motor aus"), (0.1, "Idle, no load", "Leerlauf, ohne Last"), (0.5, "Half load", "Halblast"), (0.75, "Three quarter load", "Dreiviertellast"), (1, "Prime power", "Dauerleistung (Prime)")]),
       opkey("cooling-fan", "Radiator fan", "Kühlerlüfter", [(0, "Stopped", "Stillstand"), (0.5, "Low speed", "Niedrige Drehzahl"), (1, "Full speed", "Volle Drehzahl")])]
emitters = [emitter("exhaust-stack-jet", "exhaust", "Exhaust jet of the stack", "Abgasstrahl des Abgasrohrs", "stack-mouth", 90, 0.2, "engine-load", 1.66, 450, 0.04, "heywood-2018"),
            emitter("radiator-air", "fan", "Radiator fan air leaving through the discharge hood", "Kühlerlüfterluft aus der Austrittshaube", "radiator-outlet", 0, 1.2, "cooling-fan", 9.0, 30, 0.0)]
anchors = [anchor("contact-l", "container", -2.9, 0.0, "contact", "Corner casting support, left", "Eckbeschlag-Auflager links", "contact", 0.3, 2.0, 0.6, 90),
           anchor("contact-r", "container", 2.9, 0.0, "contact", "Corner casting support, right", "Eckbeschlag-Auflager rechts", "contact", 0.3, 2.0, 0.6, 90),
           anchor("stack-mouth", "stack", 0.33, 0.0, "emitter", "Exhaust stack mouth", "Abgasmündung"), anchor("radiator-outlet", "container", HX, 2.9, "emitter", "Radiator air outlet", "Kühlerluftaustritt"),
           anchor("fuel-cog", "container", 0.0, 0.1, "load", "Fuel base tank centre", "Mitte des Kraftstoff-Grundtanks")]
CONTACTS = ["contact-l", "contact-r"]


def th(op, em):
    return tbind(RID, DEF, {}, {"exhaust-stack-jet": em, "radiator-air": em}, op)


FUEL = [{"id": "fuel", "kind": "equipment", "title": tx("Diesel in the base tank (500 l, 420 kg)", "Diesel im Grundtank (500 l, 420 kg)"), "anchor": "fuel-cog", "fxN": 0, "fyN": round(-FUEL_N, 1), "source": "authored-estimate"}]


def fl(cap, out):
    return {"cap-flap": {"angleDeg": cap}, "outlet-flap": {"angleDeg": out}}


poses = [
    pose("off", "Standby, engine off", "Bereitschaft, Motor aus", "Flaps closed, engine and radiator fan stopped, everything at ambient.", "Klappen geschlossen, Motor und Kühlerlüfter stehen, alles auf Umgebungstemperatur.", fl(0, 0), CONTACTS, th({"engine-load": 0, "cooling-fan": 0}, 0), FUEL),
    pose("idle", "Idling without load", "Leerlauf ohne Last", "Engine at idle for warm-up: rain cap lifted a little, discharge flap pushed open by the low airflow.", "Motor im Leerlauf zum Warmlaufen: Regenklappe leicht gehoben, Austrittsklappe vom geringen Luftstrom geöffnet.", fl(-35, 25), CONTACTS, th({"engine-load": 0.1, "cooling-fan": 0.5}, 1), FUEL),
    pose("half-load", "Half load", "Halblast", "Half of the prime power, radiator fan at low speed.", "Halbe Dauerleistung, Kühlerlüfter mit niedriger Drehzahl.", fl(-70, 55), CONTACTS, th({"engine-load": 0.5, "cooling-fan": 0.5}, 1), FUEL),
    pose("rated-load", "Prime power", "Dauerleistung", "Full prime power, rain cap fully open and the discharge flap blown wide open by 9 m3/s of radiator air.", "Volle Dauerleistung, Regenklappe ganz offen und Austrittsklappe von 9 m3/s Kühlerluft weit aufgeblasen.", fl(-90, 80), CONTACTS, th({"engine-load": 1, "cooling-fan": 1}, 1), FUEL),
    pose("loading-up", "Load stepping up", "Laststufe steigt", "Key pose of the start sequence between idle and half load.", "Schlüsselpose der Anlauffolge zwischen Leerlauf und Halblast.", fl(-55, 40), CONTACTS, th({"engine-load": 0.3, "cooling-fan": 0.5}, 1), FUEL, key=True),
]
clips = [
    clip("load-cycle", "Load cycling between half and prime", "Lastwechsel zwischen Halblast und Dauerleistung", "A 12 s loop: the load rises from half to prime power and back as the building demand cycles; flaps and fan follow, exhaust and radiator air keep flowing.",
         "Eine Schleife von 12 s: Die Last steigt von Halblast auf Dauerleistung und zurück, wie der Gebäudebedarf wechselt; Klappen und Lüfter folgen, Abgas und Kühlerluft strömen weiter.", 12, 30, True,
         [key(0, "half-load", "smooth"), key(4, "rated-load", "smooth"), key(8, "rated-load", "smooth"), key(12, "half-load", "smooth")],
         [{"anchor": a, "fromS": 0, "toS": 12} for a in CONTACTS], [phase("up", 0, 4, "Load rising", "Last steigt"), phase("hold", 4, 8, "Prime power", "Dauerleistung"), phase("down", 8, 12, "Load falling", "Last fällt")],
         [event(0, "operating", "Running at half load", "Betrieb mit Halblast", "engine-load")],
         [track("operating:engine-load", [(0, 0.5), (4, 1), (8, 1), (12, 0.5)]), track("operating:cooling-fan", [(0, 0.5), (4, 1), (8, 1), (12, 0.5)]), track("emitter:exhaust-stack-jet", [(0, 1)], "step"), track("emitter:radiator-air", [(0, 1)], "step")]),
    clip("start-run-stop", "Start, load step and shutdown", "Anlassen, Laststufe und Abstellen", "A 26 s sequence: key on at 1 s, warm-up idle, a step to prime power at 8 s, hold, unload and shut down at 24 s while the flaps fall shut.",
         "Eine Folge von 26 s: Zündung bei 1 s, Warmlauf im Leerlauf, Sprung auf Dauerleistung bei 8 s, Halten, Entlasten und Abstellen bei 24 s, während die Klappen zufallen.", 26, 30, False,
         [key(0, "off", "smooth"), key(1.5, "off", "smooth"), key(3, "idle", "smooth"), key(7, "idle", "smooth"), key(9, "loading-up", "smooth"), key(11, "rated-load", "smooth"), key(20, "rated-load", "smooth"), key(23, "idle", "smooth"), key(25, "off", "smooth"), key(26, "off", "smooth")],
         [{"anchor": a, "fromS": 0, "toS": 26} for a in CONTACTS], [phase("crank", 0, 3, "Cranking and run-up", "Anlassen und Hochlauf"), phase("warm", 3, 8, "Warm-up at idle", "Warmlauf im Leerlauf"), phase("load", 8, 22, "Loaded", "Unter Last"), phase("stop", 22, 26, "Unloading and stopping", "Entlasten und Abstellen")],
         [event(1.0, "ignition", "Engine starts", "Motor startet", "exhaust-stack-jet"), event(8.0, "operating", "Load step to prime power", "Laststufe auf Dauerleistung", "engine-load"), event(24.0, "shutdown", "Engine stops", "Motor stoppt", "exhaust-stack-jet")],
         [track("operating:engine-load", [(0, 0), (1, 0), (1.5, 0.1), (7, 0.1), (11, 1), (20, 1), (23, 0.1), (24, 0)]), track("operating:cooling-fan", [(0, 0), (2, 0), (3, 0.5), (8, 0.5), (11, 1), (20, 1), (23, 0.5), (26, 0)]),
          track("emitter:exhaust-stack-jet", [(0, 0), (1, 1), (24, 0)], "step"), track("emitter:radiator-air", [(0, 0), (1.5, 1), (24.5, 0)], "step")]),
]


def build_reps():
    r = {}
    nb = lambda a, b: tx(a, b)
    r["500"] = rep(500, 350, 1000, pens(0.15, None, None, 0.13, 0.25), [], shapes(500), [
        nb("One closed container block, a roof silencer with its stack and rain cap flap, the discharge hood with its weather flap.", "Ein geschlossener Containerblock, ein Dachschalldämpfer mit Abgasrohr und Regenklappe, die Austrittshaube mit ihrer Wetterklappe."),
        nb("The 6.06 x 2.59 m container, silencer, 0.2 m stack (0.4 mm), hood and both flaps (they rotate).", "Der Container von 6,06 x 2,59 m, Schalldämpfer, Abgasrohr von 0,2 m (0,4 mm), Haube und beide Klappen (sie drehen)."),
        nb("Walls, roof, louvres and the base tank are one form.", "Wände, Dach, Lamellen und Grundtank sind eine Form."),
        nb("Door lines, louvre patterns, corner castings, clamps, lifting eyes.", "Türlinien, Lamellenmuster, Eckbeschläge, Schellen, Hebeösen."),
        nb("The stack (0.4 mm) is at real width; the flaps are drawn 0.1 to 0.12 m thick (real about 0.05 m), the least thickening that stays above the 0.2 mm feature limit.", "Das Rohr (0,4 mm) ist in Realbreite; die Klappen sind 0,1 bis 0,12 m dick gezeichnet (real etwa 0,05 m), die geringste Verdickung über der Grenze von 0,2 mm."),
        nb("None.", "Keine."),
        nb("Container mid-tone, silencer and stack dark, flaps light; pens 0.15 mm.", "Container mittlerer Ton, Schalldämpfer und Rohr dunkel, Klappen hell; Stifte 0,15 mm."),
        nb("The container form carries walls, roof, louvres and base tank (merged); every other region has its own form.", "Die Containerform trägt Wände, Dach, Lamellen und Grundtank (zusammengefasst); jede andere Region hat eine eigene Form."),
    ])
    r["200"] = rep(200, 150, 350, pens(0.18, 0.13, None, 0.13, 0.25), [], shapes(200), [
        nb("Base frame, wall, roof band, two inlet louvre fields, a door outline, silencer, stack with rain cap, hood with flap.", "Grundrahmen, Wand, Dachband, zwei Eintrittslamellenfelder, ein Türumriss, Schalldämpfer, Abgasrohr mit Regenklappe, Haube mit Klappe."),
        nb("The louvre fields and the door outline; flaps at 0.07 and 0.06 m (0.35 and 0.3 mm).", "Die Lamellenfelder und der Türumriss; Klappen mit 0,07 und 0,06 m (0,35 und 0,3 mm)."),
        nb("Corner castings and clamps are not drawn separately.", "Eckbeschläge und Schellen sind nicht einzeln gezeichnet."),
        nb("Clamps, castings, handles, lifting eyes, louvre blades.", "Schellen, Beschläge, Griffe, Hebeösen, Lamellenblätter."),
        nb("Flaps are 0.3 to 0.35 mm at about real thickness.", "Klappen sind 0,3 bis 0,35 mm in etwa Realdicke."),
        nb("None.", "Keine."),
        nb("Wall mid-tone, louvres dark, roof band and base darker.", "Wand mittlerer Ton, Lamellen dunkel, Dachband und Sockel dunkler."),
        nb("Every region has its own form.", "Jede Region hat eine eigene Form."),
    ])
    hat = [hatch("louvre", "Louvre blades", "Lamellenblätter", "lines", 0, 0.5, 0.13)]
    r["100"] = rep(100, 75, 150, pens(0.25, 0.18, None, 0.13, 0.35), hat, shapes(100), [
        nb("As 1:200 with a second door, silencer clamps, corner castings and louvre blade hatching.", "Wie 1:200 mit zweiter Tür, Schalldämpferschellen, Eckbeschlägen und Lamellenschraffur."),
        nb("Both doors, clamps, castings.", "Beide Türen, Schellen, Beschläge."),
        nb("Louvre blades are a hatch at 0.5 mm spacing (real 0.05 m pitch).", "Lamellenblätter sind eine Schraffur mit 0,5 mm Abstand (real 0,05 m Teilung)."),
        nb("Handles, lifting eyes.", "Griffe, Hebeösen."),
        nb("Flaps 0.06 and 0.07 m (0.6 and 0.7 mm).", "Klappen 0,06 und 0,07 m (0,6 und 0,7 mm)."),
        nb("Louvre hatch only.", "Nur die Lamellenschraffur."),
        nb("As 1:200, detail pen 0.18 mm.", "Wie 1:200, Detailstift 0,18 mm."),
        nb("As 1:200; castings carry the skid-tank region.", "Wie 1:200; Beschläge tragen die Grundtank-Region."),
    ])
    hat50 = [hatch("louvre", "Louvre blades", "Lamellenblätter", "lines", 0, 1.0, 0.13)]
    r["50"] = rep(50, 20, 75, pens(0.35, 0.25, 0.13, 0.18, 0.5), hat50, shapes(50), [
        nb("All 1:100 parts plus door handle, fuel filler, lifting eye and silencer flange.", "Alle 1:100-Teile plus Türgriff, Tankstutzen, Hebeöse und Schalldämpferflansch."),
        nb("Louvre blades at the real 0.05 m pitch (1 mm).", "Lamellenblätter in der realen Teilung von 0,05 m (1 mm)."),
        nb("Nothing merged.", "Nichts zusammengefasst."),
        nb("Corrugation of the walls, hinges.", "Wellung der Wände, Scharniere."),
        nb("Handle 0.06 m (1.2 mm).", "Griff 0,06 m (1,2 mm)."),
        nb("Louvre hatch at 1 mm spacing.", "Lamellenschraffur mit 1 mm Abstand."),
        nb("Heavier pens 0.35 / 0.25 mm.", "Stärkere Stifte 0,35 / 0,25 mm."),
        nb("As 1:100.", "Wie 1:100."),
    ])
    return r


def asset():
    return {
        "id": "diesel-generator", "category": "machinery", "title": tx("Silenced container diesel generator, 400 kVA", "Schallgedämmtes Container-Dieselaggregat, 400 kVA"),
        "summary": tx("A 400 kVA diesel generating set in a silenced 20 ft container in side elevation: louvred inlets, roof silencer with a stack and rain cap flap, and the radiator discharge hood with a weather flap. Exhaust jet and radiator fan air are separate emitters; the flaps open with load.",
                      "Ein 400-kVA-Dieselaggregat im schallgedämmten 20-ft-Container in der Seitenansicht: Eintrittslamellen, Dachschalldämpfer mit Abgasrohr und Regenklappe sowie die Kühlerluft-Austrittshaube mit Wetterklappe. Abgasstrahl und Kühlerlüfterluft sind getrennte Emitter; die Klappen öffnen mit der Last."),
        "tags": ["machine", "generator", "genset", "diesel", "container", "silenced", "power"],
        "provenance": {"author": "Semio staffage content worker NW-MA", "method": "handcrafted", "created": "2026-10-29",
                       "references": references(["iso-8528", "heywood-2018", "incropera-2011", "ashrae-2021", "class-data-note"]),
                       "notes": tx("Class data of 400 kVA prime-power container gensets (about 320 kW electrical, 360 kW engine, 7.2 t dry in a 20 ft container with roof silencer). Prime rating per ISO 8528-1; all machine numbers are authored estimates of that class. Fan blade rotation is not visible in the side elevation; the flaps are the moving parts.",
                                   "Klassendaten von 400-kVA-Containeraggregaten mit Dauerleistung (etwa 320 kW elektrisch, 360 kW Motor, 7,2 t trocken im 20-ft-Container mit Dachschalldämpfer). Dauerleistung nach ISO 8528-1; alle Maschinenzahlen sind Schätzungen dieser Klasse. Die Lüfterdrehung ist in der Seitenansicht nicht sichtbar; die Klappen sind die bewegten Teile.")},
        "dimensions": {"heightM": 3.57, "widthM": 6.06, "depthM": 2.44, "massKg": 7200},
        "materials": materials, "rig": rig.doc(), "anchors": anchors, "representations": build_reps(), "symbols": [],
        "poses": poses, "clips": clips,
        "thermal": {"environment": DEFAULT_ENV, "passengers": [], "operating": ops, "regions": regions, "emitters": emitters,
                    "assumptions": [tx("Engine 360 kW at prime: exhaust 0.0022 kg/s per kW = 0.78 kg/s at 470 C (rho 0.47 kg/m3) gives 1.66 m3/s through a 0.2 m stack (53 m/s) and 0.04 kg/kg vapour (Heywood 2018); flow and rise scale linearly with the engine level.", "Motor 360 kW bei Dauerleistung: Abgas 0,0022 kg/s je kW = 0,78 kg/s bei 470 C (rho 0,47 kg/m3) ergibt 1,66 m3/s durch ein 0,2-m-Rohr (53 m/s) und 0,04 kg/kg Dampf (Heywood 2018); Volumenstrom und Erhöhung skalieren linear mit der Motorstufe."),
                                    tx("Radiator and charge-air cooler reject about 325 kW: 9 m3/s of air at +30 K leave through the discharge hood (rho cp V dT = 1.2 x 1005 x 9 x 30). The hood shell is heated by 2 kW of that, the walls by 0.8 kW through the 50 mm wool, the lagged silencer by 3 kW and the stack pipe by 1.2 kW. Authored estimates.", "Kühler und Ladeluftkühler führen etwa 325 kW ab: 9 m3/s Luft mit +30 K verlassen die Austrittshaube (rho cp V dT = 1,2 x 1005 x 9 x 30). Davon heizen 2 kW die Haube, 0,8 kW die Wände durch die 50 mm Wolle, 3 kW den gedämmten Schalldämpfer und 1,2 kW das Rohr. Schätzungen."),
                                    tx("The base tank holds 420 kg of diesel (heat capacity 0.2 MJ/(m2 K) as a buffer); the flaps are driven by pressure and temperature, modelled by the pose bindings.", "Der Grundtank enthält 420 kg Diesel (Wärmekapazität 0,2 MJ/(m2 K) als Puffer); die Klappen werden von Druck und Temperatur bewegt, modelliert durch die Posenbindungen.")]},
        "structure": structure("The container rests on two corner-casting supports; the fuel in the base tank is a point load; gravity only.", "Der Container ruht auf zwei Eckbeschlag-Auflagern; der Kraftstoff im Grundtank ist eine Einzellast; nur Schwerkraft."),
        "presentation": {"defaultPose": "off", "defaultClip": "load-cycle", "defaultScale": 100, "viewpoints": ["elevation"]},
        "exports": EXPORTS, "completion": "authored",
    }


if __name__ == "__main__":
    a = asset()
    for p in selfcheck(a):
        print("PROBLEM", p)
    for d in (500, 200, 100, 50):
        print(d, [round(v, 2) for v in bbox(a, rig, d, {})])
    print(write(a, "diesel-generator"))
