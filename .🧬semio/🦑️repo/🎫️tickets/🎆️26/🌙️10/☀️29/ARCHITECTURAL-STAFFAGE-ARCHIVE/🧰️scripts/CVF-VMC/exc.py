import sys
from build import *

REL = "🏗️machinery/excavator"
KEYS = [
    opkey("engine-load", "Engine load", "Motorlast", [(0, "Engine off", "Motor aus"), (0.2, "Idle", "Leerlauf"), (0.8, "Digging", "Graben"), (1, "Full load", "Volllast")]),
    opkey("hydraulic", "Hydraulic work", "Hydraulikarbeit", [(0, "Off", "Aus"), (0.1, "Standby", "Bereitschaft"), (0.9, "Working", "Arbeitend"), (1, "Full flow", "Volle Fördermenge")]),
]
plate = lambda mm, hc, mat="paint-steel": layer("plate", "Steel plate %d mm" % mm, "Stahlblech %d mm" % mm, "metal", mat, mm / 1000, mm / 1000 / 50, hc, source="literature", ref="incropera-2011")
REGIONS = [
    region("house", "Upper structure paintwork", "Lackierung des Oberwagens", "surface", "passive", "paint-steel", 30, 2, "vertical-plate", 0.6, 15000, [plate(4, 15000)], ref="ashrae-2021"),
    region("counterweight", "Counterweight", "Gegengewicht", "surface", "passive", "cast-iron", 4.5, 1.2, "vertical-plate", 0.6, 150000, [layer("iron", "Cast iron 0.2 m (thermally active part)", "Gusseisen 0,2 m (thermisch wirksamer Teil)", "metal", "cast-iron", 0.2, 0.2 / 50, 150000, source="authored-estimate")], ref="ashrae-2021"),
    region("cab-glazing", "Cab glazing", "Kabinenverglasung", "glazing", "passive", "glass", 6, 1, "vertical-plate", 0.6, 15000, [layer("glass", "Toughened glass 8 mm", "Einscheibensicherheitsglas 8 mm", "glass", "glass", 0.008, 0.008, 15000, source="literature", ref="incropera-2011")], ref="ashrae-2021"),
    region("engine-compartment", "Engine compartment surfaces", "Motorraumoberflächen", "engine", "operating", "engine-metal", 2.5, 0.8, "cylinder", 0.1, 12000, [], ("engine-load", 1400), ref="heywood-2018"),
    region("exhaust-stack", "Exhaust stack", "Abgasrohr", "exhaust", "operating", "exhaust-steel", 0.22, 0.12, "cylinder", 0.9, 11000, [layer("pipe", "Steel pipe 3 mm", "Stahlrohr 3 mm", "metal", "exhaust-steel", 0.003, 0.003 / 45, 11000, source="authored-estimate")], ("engine-load", 2000), ref="heywood-2018"),
    region("hydraulics", "Hydraulic cylinders", "Hydraulikzylinder", "surface", "operating", "paint-steel", 3.5, 0.3, "cylinder", 0.7, 20000, [], ("hydraulic", 1200)),
    region("attachment", "Boom, stick and bucket steel", "Stahl von Ausleger, Stiel und Löffel", "surface", "passive", "paint-steel", 28, 2, "vertical-plate", 0.7, 38000, [plate(10, 38000)], ref="ashrae-2021"),
    region("tracks", "Track shoes and frames", "Kettenglieder und Laufwerksrahmen", "surface", "passive", "track-steel", 14, 1.2, "vertical-plate", 0.4, 60000, [plate(20, 60000, "track-steel")], ref="ashrae-2021"),
]
EMITTERS = [emitter("exhaust-stack", "exhaust", "Exhaust stack jet", "Abgasstrahl des Auspuffrohrs", "stack-mouth", 90, 0.12, "engine-load", 0.45, 360, 0.04, ref="heywood-2018")]
DEFAULTS = {"house": (1, 0.6), "counterweight": (1, 0.5), "cab-glazing": (1, 0.5), "engine-compartment": (0.3, 0), "exhaust-stack": (1, 0.8), "hydraulics": (1, 0.5), "attachment": (1, 0.7), "tracks": (1, 0.2)}
CONTACTS = {"tracks": (0.29, 30)}
P = lambda e, h: {"operating": {"engine-load": e, "hydraulic": h}, "emitters": {"exhaust-stack": 1 if e > 0 else 0}}
POSES = {"stowed": P(0, 0), "digging": P(0.8, 0.9), "raised-boom": P(0.7, 0.8)}


def fix_clips(clips):
    for c in clips:
        c["tracks"] = [t for t in c["tracks"] if not t["target"].startswith("emitter:")]
        if c["id"] == "dig-cycle":
            c["tracks"].append({"target": "emitter:exhaust-stack", "ease": "step", "keys": [{"t": 0, "v": 1}]})
        if c["id"] == "dump-and-stow":
            c["tracks"].append({"target": "emitter:exhaust-stack", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 6.9, "v": 0}]})
    c0 = [c for c in clips if c["id"] == "dump-and-stow"][0]
    clips.append({"id": "start-and-idle", "title": tx("Start and idle", "Starten und Leerlauf"), "description": tx("Cold start of the stowed machine: the diesel fires, idles and the hydraulics go to standby; surfaces begin to warm.", "Kaltstart der eingefahrenen Maschine: der Diesel springt an, läuft im Leerlauf, die Hydraulik geht auf Bereitschaft; die Oberflächen beginnen sich zu erwärmen."),
                  "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none", "keys": [{"t": 0, "pose": "stowed", "ease": "linear"}, {"t": 8, "pose": "stowed", "ease": "linear"}],
                  "contacts": [{"anchor": a, "fromS": 0, "toS": 8} for a in ("track-r2", "track-r1", "track-f1", "track-f2")],
                  "phases": [{"id": "crank", "fromS": 0, "toS": 1.5, "title": tx("Cranking and ignition", "Anlassen und Zündung")}, {"id": "idle", "fromS": 1.5, "toS": 8, "title": tx("Idle warm-up", "Warmlauf im Leerlauf")}],
                  "events": [{"t": 0.8, "kind": "ignition", "title": tx("Diesel starts", "Diesel springt an"), "target": "stack-mouth"}],
                  "tracks": [{"target": "operating:engine-load", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 0.8, "v": 0.5}, {"t": 1.5, "v": 0.2}, {"t": 8, "v": 0.2}]},
                             {"target": "operating:hydraulic", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 2, "v": 0.1}, {"t": 8, "v": 0.1}]},
                             {"target": "emitter:exhaust-stack", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 0.8, "v": 1}]}]})


def reps(a, old):
    R = {}
    O = {s["id"]: s for s in old}
    take = lambda ids: convert_v1_shapes([O[i] for i in ids])
    bucket500 = path("bucket-body", "bucket", "body", 3, "M 1.35,0 L 1.3,0.05 C 1.05,0.62 0.35,0.78 -0.08,0.34 L -0.12,0.06 L 0.02,-0.12 C 0.4,-0.1 0.95,-0.1 1.35,0 L 1.5,-0.03 L 1.28,0.07 Z", 0.35, True, region="attachment", material="wear-steel")
    house = lambda regs=None, region="house": path("house", "upper", "body", 2, O["house"]["d"].replace("M -1.75,0 L 1.6,0", "M -1.75,-0.16 L 1.6,-0.16"), 0.55, True, region=None if regs else region, regions=regs, material="paint-steel")
    belt = lambda: path("track-belt", "undercarriage", "body", 1, O["track-belt"]["d"], 0.25, True, region="tracks", material="track-steel")
    cw = lambda: path("counterweight-body", "counterweight", "body", 2.5, O["counterweight-body"]["d"].replace("M 0.65,0 L -0.5,0 C -0.6,0 -0.65,0.1", "M 0.65,-0.16 L -0.5,-0.16 C -0.6,-0.16 -0.65,0.1"), 0.4, True, region="counterweight", material="cast-iron")
    boom = lambda hyd=False: path("boom-body", "boom", "body", 2.8, O["boom-body"]["d"], 0.55, True, region=None if hyd else "attachment", regions=["attachment", "hydraulics"] if hyd else None, material="paint-steel")
    stick = lambda: path("stick-body", "stick", "body", 2.9, O["stick-body"]["d"], 0.55, True, region="attachment", material="paint-steel")
    cabglass200 = lambda: path("cab-glass-pane", "upper", "glazing", 6, "M 0.35,0.62 L 0.95,0.62 L 1.0,0.58 L 1.46,0.58 C 1.45,1.1 1.4,1.6 1.34,1.82 C 1.3,1.87 1.24,1.87 1.2,1.87 L 0.45,1.87 C 0.38,1.87 0.35,1.83 0.35,1.75 Z", 0.85, True, region="cab-glazing", material="glass")
    cabglass = lambda: path("cab-glass", "upper", "glazing", 6, "M 0.3,0.6 L 0.95,0.6 L 1.02,0.55 L 1.56,0.55 C 1.55,1.1 1.5,1.6 1.43,1.84 C 1.38,1.9 1.3,1.9 1.2,1.9 L 0.45,1.9 C 0.35,1.9 0.3,1.85 0.3,1.75 Z", 0.85, True, region="cab-glazing", material="glass")
    HC = "M -2.85,-0.16 L 1.6,-0.16 L 1.6,0.3 L 1.58,0.95 C 1.56,1.4 1.5,1.75 1.42,1.88 C 1.36,1.96 1.28,1.97 1.2,1.97 L 0.4,1.97 C 0.22,1.97 0.12,1.88 0.12,1.7 L 0.12,1.25 L -2.5,1.22 C -2.75,1.2 -2.95,1.1 -2.98,0.85 L -3.0,0.25 C -3.0,0.1 -2.95,-0.16 -2.85,-0.16 Z"
    e500 = [belt(), path("house-counterweight", "upper", "body", 2, HC, 0.5, True, regions=["house", "engine-compartment", "exhaust-stack", "counterweight"], material="paint-steel"), cabglass(), boom(True), stick(), bucket500]
    R["500"] = rep(500, *RANGES[500], PRINT_PENS[500], [], e500, decisions(
        tx("Crawler excavator as one dark track belt, one house with cab, the counterweight and three massive arm parts; the stowed pose folds the arm over the cab.", "Kettenbagger als ein dunkles Laufwerk, ein Oberwagen mit Kabine, das Gegengewicht und drei massive Armteile; in der eingefahrenen Haltung liegt der Arm über der Kabine."),
        tx("Track belt, house, cab glass, counterweight, boom, stick, bucket with teeth and the boom cylinder (barrel and rod) so the arm drive is still shown.", "Laufwerk, Oberwagen, Kabinenglas, Gegengewicht, Ausleger, Stiel, Löffel mit Zähnen und der Auslegerzylinder (Rohr und Stange), damit der Armantrieb sichtbar bleibt."),
        tx("Engine compartment and exhaust stack merge into the house fill (area-weighted overlay); track frame, rollers, sprocket and idler merge into the belt.", "Motorraum und Abgasrohr gehen in der Oberwagenfläche auf (flächengewichtete Überlagerung); Laufwerksrahmen, Rollen, Antriebsrad und Leitrad gehen im Laufwerk auf."),
        tx("Stick and bucket cylinders (bones remain), pins, welds, grousers, louvres, seat, hand rails.", "Stiel- und Löffelzylinder (Knochen bleiben erhalten), Bolzen, Schweißnähte, Stollen, Lamellen, Sitz, Handläufe."),
        tx("The boom cylinder barrel is 0.2 m (0.4 mm) and the rod 0.14 m (0.28 mm, real 0.1 m, thickened 0.04 m); all other parts at real size.", "Der Auslegerzylinder hat 0,2 m (0,4 mm) Rohr und 0,14 m (0,28 mm, real 0,1 m, um 0,04 m verstärkt) Stange; alle anderen Teile in Realgröße."),
        tx("None.", "Keine."), tx("Belt dark, house and arm mid-tone, counterweight dark, glass light, chrome rod lightest.", "Laufwerk dunkel, Oberwagen und Arm mittlerer Ton, Gegengewicht dunkel, Glas hell, verchromte Stange am hellsten."),
        tx("Thermal overlays: the house shape shows the merged house, engine and exhaust heat; the arm shows attachment steel; the cylinder shows hydraulics.", "Thermische Überlagerungen: die Oberwagenform zeigt die zusammengefasste Wärme von Oberwagen, Motor und Abgas; der Arm den Stahl des Arbeitsgeräts; der Zylinder die Hydraulik.")))
    # 200
    e200 = [belt(), path("track-frame", "undercarriage", "detail", 1.5, O["track-frame"]["d"], 0.5, True, region="tracks", material="paint-steel"),
            ell("sprocket", "undercarriage", "detail", 1.6, -1.65, 0.43, 0.33, tone=0.5, region="tracks", material="track-steel"), ell("idler", "undercarriage", "detail", 1.6, 1.65, 0.43, 0.33, tone=0.5, region="tracks", material="track-steel"),
            house(), path("hood-grille", "upper", "detail", 2.2, O["hood-grille"]["d"], 0.2, True, region="engine-compartment", material="engine-metal"),
            cap("stack", "upper", "detail", 2.2, -1.2, 1.25, -1.2, 1.8, 0.055, tone=0.4, region="exhaust-stack", material="exhaust-steel"),
            cabglass200(), cw(), boom(), stick(), bucket500]
    for n, b, r0, L in (("boom-cyl", "boom-cyl", 0.1, 1.45), ("stick-cyl", "stick-cyl", 0.09, 1.6)):
        e200.append(cap(n + "-barrel", b, "detail", 3.5, 0, 0, L, 0, r0, tone=0.4, region="hydraulics", material="paint-steel"))
    for n, b, r0, L in (("boom-rod", "boom-rod", 0.05, 1.65), ("stick-rod", "stick-rod", 0.045, 2.35)):
        e200.append(cap(n, b, "detail", 3.55, 0, 0, L, 0, r0, tone=0.7, region="hydraulics", material="chrome"))
    R["200"] = rep(200, *RANGES[200], pens(0.13, 0.1, None, 0.1, 0.25), [], e200, decisions(
        tx("Belt with sprocket and idler circles, house with cab and engine hood, counterweight, boom, stick and bucket as massive composed forms, three cylinders.", "Laufwerk mit Antriebs- und Leitradkreisen, Oberwagen mit Kabine und Motorhaube, Gegengewicht, Ausleger, Stiel und Löffel als massive zusammengesetzte Formen, drei Zylinder."),
        tx("Track frame, sprocket, idler, house, hood grille, exhaust stack (0.11 m = 0.55 mm), cab glass, counterweight, arm, all hydraulic barrels and rods.", "Laufwerksrahmen, Antriebsrad, Leitrad, Oberwagen, Motorhaubengitter, Abgasrohr (0,11 m = 0,55 mm), Kabinenglas, Gegengewicht, Arm, alle Hydraulikrohre und -stangen."),
        tx("Cab front and side glass form one pane; the stack cap merges into the stack.", "Kabinenfront- und Seitenglas bilden eine Scheibe; die Rohrkappe geht im Rohr auf."),
        tx("Rollers (0.22 m but not resolved against the belt), pins, bosses, welds, grousers, louvres, seat.", "Laufrollen (0,22 m, gegen das Laufwerk nicht aufgelöst), Bolzen, Naben, Schweißnähte, Stollen, Lamellen, Sitz."),
        tx("Rods are 0.09 to 0.1 m (0.45 to 0.5 mm) at real size.", "Stangen sind 0,09 bis 0,1 m (0,45 bis 0,5 mm) in Realgröße."),
        tx("None.", "Keine."), tx("Belt dark, house and arm mid-tone, hood dark, rods lightest; pens 0.18 / 0.13 mm.", "Laufwerk dunkel, Oberwagen und Arm mittlerer Ton, Haube dunkel, Stangen am hellsten; Stifte 0,18 / 0,13 mm."),
        tx("Every region has its own shape; the hood grille and stack show the engine and exhaust heat.", "Jede Region hat eine eigene Form; Haubengitter und Rohr zeigen Motor- und Abgaswärme.")))
    # 100
    e100 = [s for s in e200 if s["id"] not in ("sprocket", "idler", "cab-glass-pane")] + [cabglass(), cap("bucket-cyl-barrel", "bucket-cyl", "detail", 3.5, 0, 0, 0.85, 0, 0.07, tone=0.4, region="hydraulics", material="paint-steel"), cap("bucket-rod", "bucket-rod", "detail", 3.55, 0, 0, 0.95, 0, 0.04, tone=0.7, region="hydraulics", material="chrome")] + take(["sprocket", "idler", "sprocket-hub", "idler-hub", "roller-1", "roller-2", "roller-3", "roller-4", "roller-5", "slew-ring", "boom-pin", "boom-root-boss", "boom-tip-boss", "stick-root-boss", "stick-tip-boss", "bucket-pin", "cab-sill", "bucket-teeth", "skirt-line", "hood-rail", "cab-seat", "boom-foot"])
    e100 = [s for s in e100 if s["id"] not in ("slew-ring", "bucket-rod")]
    for s in e100:
        if s["id"] == "boom-root-boss":
            s["rx"] = s["ry"] = 0.2
        if s["id"] == "bucket-rod":
            s["r0"] = s["r1"] = 0.03
        if s["id"] in ("boom-pin", "boom-root-boss", "boom-tip-boss", "stick-root-boss", "stick-tip-boss", "bucket-pin"):
            s["role"] = "detail"
    R["100"] = rep(100, *RANGES[100], pens(0.18, 0.13, None, 0.1, 0.35), [], e100, decisions(
        tx("The 1:200 drawing plus rollers, pin bosses, slew ring, boom foot and seat so the machine reads as a hinged arm on a swivelling house.", "Die 1:200-Zeichnung mit Laufrollen, Bolzennaben, Drehkranz, Auslegerfuß und Sitz, sodass die Maschine als Gelenkarm auf drehbarem Oberwagen lesbar ist."),
        tx("Five track rollers, sprocket and idler hubs, boss circles at the three arm pins, slew ring, cab sill, seat, hood rail, bucket teeth.", "Fünf Laufrollen, Antriebs- und Leitradnaben, Nabenkreise an den drei Armgelenken, Drehkranz, Kabinenschweller, Sitz, Haubenreling, Löffelzähne."),
        tx("Bucket plate and ear merge into the bucket body.", "Löffelplatte und Löffelohr gehen im Löffelkörper auf."),
        tx("Grousers, welds, louvres, rod eyes (below 0.04 m).", "Stollen, Schweißnähte, Lamellen, Stangenaugen (unter 0,04 m)."),
        tx("None thickened.", "Nichts verstärkt."), tx("None.", "Keine."), tx("As 1:200; pens 0.25 / 0.18 mm.", "Wie 1:200; Stifte 0,25 / 0,18 mm."), tx("As 1:200.", "Wie 1:200.")))
    e50 = convert_v1_shapes(old)
    for s in e50:
        if s["id"] == "cab-glass-front":
            s["d"] = "M 1.02,0.58 L 1.46,0.58 C 1.45,1.1 1.4,1.6 1.34,1.82 C 1.3,1.87 1.24,1.87 1.2,1.87 L 1.02,1.87 Z"
        if s["id"] == "cab-sill":
            s["d"] = "M 0.12,0.55 L 1.45,0.55"
        if s["id"] == "boom-cyl-rod":
            s["r0"] = s["r1"] = 0.035
        if s["id"] == "bucket-cyl-rod":
            s["r0"] = s["r1"] = 0.03
    R["50"] = rep(50, *RANGES[50], pens(0.25, 0.18, 0.1, 0.13, 0.5), [], e50, decisions(
        tx("Full side elevation of the first version.", "Vollständige Seitenansicht der ersten Fassung."),
        tx("Everything: grousers, louvres, welds, bosses, rod eyes, seams.", "Alles: Stollen, Lamellen, Schweißnähte, Naben, Stangenaugen, Nähte."), tx("Nothing merged.", "Nichts zusammengefasst."),
        tx("Hoses, handrails, lamps (not modelled).", "Schläuche, Handläufe, Leuchten (nicht modelliert)."), tx("Grouser ticks 0.07 m long at texture pen.", "Stollenstriche 0,07 m lang mit Texturstift."), tx("None.", "Keine."),
        tx("Three line weights 0.35 / 0.25 / 0.13 mm.", "Drei Linienstärken 0,35 / 0,25 / 0,13 mm."), tx("All regions at real extent.", "Alle Regionen in realer Ausdehnung.")))
    return R


NOTES = tx("Diesel crawler excavator of the 20 t class. Slewing is a vertical-axis motion and not an elevation joint; the cylinder pairs close the pin geometry at every authored key. Thermal: engine compartment and stack heated by waste heat at engine-load level 1, hydraulic cylinders by hydraulic losses. The 110 kW class engine data are authored estimates tied to ISO 10265 (ground pressure) and Heywood (diesel exhaust).",
           "Diesel-Kettenbagger der 20-t-Klasse. Das Schwenken ist eine Bewegung um die Hochachse und kein Gelenk der Ansicht; die Zylinderpaare schließen die Bolzengeometrie an jedem Schlüsselbild. Thermik: Motorraum und Auspuffrohr werden bei Motorlaststufe 1 durch Abwärme beheizt, die Hydraulikzylinder durch hydraulische Verluste. Die Motordaten der 110-kW-Klasse sind Schätzwerte mit Bezug auf ISO 10265 (Bodenpressung) und Heywood (Dieselabgas).")
ASSUMPTIONS = [
    tx("Heat release at level 1: engine compartment 1.4 kW (about +45 K), exhaust stack 2.0 kW (about +330 K), hydraulic cylinders 1.2 kW (+35 K). Heat capacities are active shell masses: plate 4 mm 15 kJ/(m2 K), boom plate 10 mm 38 kJ/(m2 K), counterweight 0.2 m of iron 150 kJ/(m2 K).",
       "Wärmefreisetzung bei Stufe 1: Motorraum 1,4 kW (etwa +45 K), Auspuffrohr 2,0 kW (etwa +330 K), Hydraulikzylinder 1,2 kW (+35 K). Wärmekapazitäten sind aktive Schalenmassen: Blech 4 mm 15 kJ/(m2 K), Auslegerblech 10 mm 38 kJ/(m2 K), Gegengewicht 0,2 m Eisen 150 kJ/(m2 K)."),
    tx("Stack at level 1 (about 110 kW rated): exhaust 0.25 kg/s at about 380 C (rho 0.52 kg/m3), hence 0.45 m3/s through a 0.12 m stack (40 m/s); lean diesel exhaust carries about 0.04 kg of water vapour per kg. Flow and temperature rise scale linearly with the load level.",
       "Rohr bei Stufe 1 (etwa 110 kW Nennleistung): Abgas 0,25 kg/s bei etwa 380 C (rho 0,52 kg/m3), also 0,45 m3/s durch ein 0,12-m-Rohr (40 m/s); mageres Dieselabgas führt etwa 0,04 kg Wasserdampf je kg. Volumenstrom und Temperaturerhöhung skalieren linear mit der Laststufe."),
    tx("The engine hood is closed in every pose (30 % of the engine surface exposed to air). Tracks: 29 % of the area rests on ground at 30 C.", "Die Motorhaube ist in jeder Haltung geschlossen (30 % der Motoroberfläche der Luft ausgesetzt). Laufwerk: 29 % der Fläche liegen auf 30 C warmem Boden.")]
SPEC = dict(sky=8, ground=30, operating=KEYS, regions=REGIONS, emitters=EMITTERS, assumptions=ASSUMPTIONS, defaults=DEFAULTS, contacts=CONTACTS, poses=POSES, fix_clips=fix_clips, reps=reps, notes=NOTES, category="machinery")
if __name__ == "__main__":
    a = assemble("excavator", REL, SPEC)
    if len(sys.argv) > 1:
        previews(a, "exc", poses=sys.argv[2:] or None, dens=[int(x) for x in sys.argv[1].split(",")], zoom=(3,))
