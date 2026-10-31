"""Assembles and writes wheel-loader."""
import math
from loader import *

EXPORTS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"]
LOAD_N = 6000 * 9.80665
materials = [mat_paint("paint-steel", "Painted structural steel (yellow)", "Lackierter Baustahl (gelb)", 0.45), mat_paint("wear-steel", "Bucket wear steel", "Schaufel-Verschleißstahl", 0.7), MAT_GLASS, MAT_RUBBER, MAT_IRON, MAT_EXHAUST,
             material("chrome", "Chromed piston rod", "Verchromte Kolbenstange", 0.12, 0.35, 7850, 50, 460, "literature", "ashrae-2021")]
R = region
regions = [
    R("engine-bay", "Engine hood", "Motorhaube", "engine", "operating", "paint-steel", 8, 1.6, "horizontal-plate-up", 0.7, [steel_layer("hood", 0.002)], 7200, op=("engine-load", 3000), ref="heywood-2018"),
    R("counterweight", "Counterweight casting", "Gegengewicht", "surface", "passive", "cast-iron", 6, 1.4, "vertical-plate", 0.5, [layer("iron", "Cast iron 0.05 m (thermally active part)", "Gusseisen 0,05 m (thermisch wirksamer Teil)", "metal", "cast-iron", 0.05, 0.001, 150000, None, "authored-estimate")], 150000),
    R("exhaust-stack", "Exhaust stack", "Auspuffrohr", "exhaust", "operating", "exhaust-steel", 0.5, 0.13, "cylinder", 0.8, [steel_layer("pipe", 0.0015, "exhaust-steel")], 5400, op=("engine-load", 3500), ref="heywood-2018"),
    R("cab", "Cab paintwork", "Lackierung der Kabine", "surface", "passive", "paint-steel", 14, 1.9, "vertical-plate", 0.6, [steel_layer("sheet", 0.0015)], 5400, ref="ashrae-2021"),
    R("cab-glazing", "Cab glazing", "Kabinenverglasung", "glazing", "passive", "glass", 6, 1.2, "vertical-plate", 0.5, [layer("pane", "Safety glass 6 mm", "Sicherheitsglas 6 mm", "glass", "glass", 0.006, 0.006, 12600, None, "literature", "incropera-2011")], 12600, ref="ashrae-2021"),
    R("front-frame", "Front frame and towers", "Vorderwagen und Stützen", "surface", "passive", "paint-steel", 10, 1.5, "vertical-plate", 0.4, [steel_layer("plate", 0.012)], 43000, ref="ashrae-2021"),
    R("lift-arm", "Lift arm", "Hubarm", "surface", "passive", "paint-steel", 9, 2.0, "cylinder", 0.7, [steel_layer("plate", 0.02)], 72000, ref="ashrae-2021"),
    R("bucket", "Bucket", "Schaufel", "surface", "passive", "wear-steel", 14, 1.6, "vertical-plate", 0.6, [steel_layer("plate", 0.025, "wear-steel")], 90000, ref="ashrae-2021"),
    R("hydraulics", "Hydraulic cylinders", "Hydraulikzylinder", "engine", "operating", "chrome", 2.2, 0.2, "cylinder", 0.6, [steel_layer("tube", 0.008, "paint-steel")], 29000, op=("hydraulic", 2000)),
    R("tyres", "Tyres 23.5R25", "Reifen 23.5R25", "tyre", "operating", "rubber", 14, 1.75, "cylinder", 0.5, [layer("rubber", "Tyre rubber 25 mm", "Reifengummi 25 mm", "polymer", "rubber", 0.025, 0.17, 46750, None, "literature", "incropera-2011")], 46750, op=("driving", 6000)),
]
RID = [r["id"] for r in regions]
DEF = {"engine-bay": {"exposedFraction": 0.4, "sunlitFraction": 0.6}, "counterweight": {"exposedFraction": 1, "sunlitFraction": 0.2}, "exhaust-stack": {"exposedFraction": 1, "sunlitFraction": 0.8},
       "cab": {"exposedFraction": 1, "sunlitFraction": 0.6}, "cab-glazing": {"exposedFraction": 1, "sunlitFraction": 0.5}, "front-frame": {"exposedFraction": 1, "sunlitFraction": 0.3},
       "lift-arm": {"exposedFraction": 1, "sunlitFraction": 0.6}, "bucket": {"exposedFraction": 1, "sunlitFraction": 0.6}, "hydraulics": {"exposedFraction": 1, "sunlitFraction": 0.4},
       "tyres": {"exposedFraction": 1, "sunlitFraction": 0.2, "contact": {"fraction": 0.05, "tempC": 32}}}
ops = [opkey("engine-load", "Engine load", "Motorlast", [(0, "Engine off", "Motor aus"), (0.2, "Idle", "Leerlauf"), (0.7, "Loading work", "Ladearbeit"), (1, "Rated power", "Nennleistung")]),
       opkey("hydraulic", "Hydraulic work", "Hydraulikarbeit", [(0, "Off", "Aus"), (0.1, "Standby", "Bereitschaft"), (0.9, "Working", "Arbeitend"), (1, "Full flow", "Volle Fördermenge")]),
       opkey("driving", "Travel", "Fahrt", [(0, "Standing", "Stillstand"), (0.5, "Loaded carry, 10 km/h", "Beladene Fahrt, 10 km/h"), (1, "Road speed, 30 km/h", "Straßenfahrt, 30 km/h")])]
emitters = [emitter("exhaust-stack-jet", "exhaust", "Exhaust jet of the vertical stack", "Abgasstrahl des senkrechten Auspuffrohrs", "stack-mouth", 90, 0.13, "engine-load", 0.85, 340, 0.04, "heywood-2018"),
            emitter("radiator-air", "vent", "Radiator air leaving at the rear grille", "Kühlerluft am hinteren Gitter", "radiator-outlet", 180, 0.8, "engine-load", 4.0, 25, 0.0)]
anchors = [anchor("contact-r", "rear-frame", XR, 0.0, "contact", "Rear tyre contact patch", "Hinterreifen-Aufstandsfläche", "contact", 0.8, 1.2, 0.7, 90),
           anchor("contact-f", "front-frame", XF, 0.0, "contact", "Front tyre contact patch", "Vorderreifen-Aufstandsfläche", "contact", 0.8, 1.2, 0.7, 90),
           anchor("stack-mouth", "rear-frame", -1.23, 3.15, "emitter", "Exhaust stack mouth", "Auspuffmündung"), anchor("radiator-outlet", "rear-frame", -3.5, 1.6, "emitter", "Radiator air outlet", "Kühlerluftaustritt"),
           anchor("bucket-cog", "bucket", 0.6, -0.1, "load", "Bucket content centre of gravity", "Schwerpunkt der Schaufelfüllung")]
CONTACTS = ["contact-r", "contact-f"]


def th(op, em, over=None):
    return tbind(RID, DEF, over or {}, {"exhaust-stack-jet": em, "radiator-air": em}, op)


def load(f):
    return [{"id": "bucket-load", "kind": "carried", "title": tx("Bucket content (3.5 m3 at 1.7 t/m3, rated 6 t)", "Schaufelfüllung (3,5 m3 zu 1,7 t/m3, Nennlast 6 t)"), "anchor": "bucket-cog", "fxN": 0, "fyN": round(-LOAD_N * f, 1), "source": "authored-estimate"}] if f > 0 else []


DIG_B = -25
DIG_A = solve_arm_for_lip0(DIG_B)
PARK_B = -12
PARK_A = solve_arm_for_lip0(PARK_B)
DUMP_A, DUMP_B = 62, -50
poses = [
    pose("carry", "Carry position, loaded", "Transportstellung, beladen", "Travelling with a full bucket rolled back and held low, engine at loading level.", "Fahrt mit voller, zurückgekippter und tief gehaltener Schaufel, Motor auf Ladestufe.", bones_for(0, 0), CONTACTS, th({"engine-load": 0.7, "hydraulic": 0.1, "driving": 0.5}, 1), load(1)),
    pose("dig", "Digging into the pile", "Graben im Haufwerk", "Lift arm lowered until the cutting edge sits on the ground, bucket tilted forward, hydraulics working.", "Hubarm abgesenkt, bis die Schneide auf dem Boden steht, Schaufel nach vorn gekippt, Hydraulik arbeitet.", bones_for(DIG_A, DIG_B), CONTACTS, th({"engine-load": 1, "hydraulic": 0.9, "driving": 0}, 1), load(0.4)),
    pose("dump-high", "Dumping at full height", "Entleeren in voller Höhe", "Arm raised to the high-dump stop and the bucket dumped over a truck body.", "Arm bis zum Hochkippanschlag gehoben, Schaufel über einer Lkw-Mulde entleert.", bones_for(DUMP_A, DUMP_B), CONTACTS, th({"engine-load": 0.7, "hydraulic": 0.9, "driving": 0}, 1), load(0.25)),
    pose("parked-off", "Parked, bucket on the ground", "Geparkt, Schaufel am Boden", "Engine off, bucket flat on the ground, all systems cold.", "Motor aus, Schaufel flach am Boden, alle Systeme kalt.", bones_for(PARK_A, PARK_B), CONTACTS, th({"engine-load": 0, "hydraulic": 0, "driving": 0}, 0), []),
    pose("raising", "Raising the arm", "Hubarm hebt", "Key pose between carry and dump: arm half raised, bucket rolled back.", "Schlüsselpose zwischen Transport und Entleeren: Arm halb gehoben, Schaufel zurückgekippt.", bones_for(30, 10), CONTACTS, th({"engine-load": 0.7, "hydraulic": 0.9, "driving": 0}, 1), load(1), key=True),
]
K = lambda t, base, a, b, ease="smooth": key(t, base, ease, bones_for(a, b))
clips = [
    clip("load-cycle", "V-cycle loading loop", "Ladezyklus im V-Verfahren", "A 12 s loop: dig, roll back, carry, raise, dump over the truck body, lower and return. The bucket load builds while digging and drops at the dump.", "Eine Schleife von 12 s: Graben, Zurückkippen, Fahren, Heben, Entleeren über die Lkw-Mulde, Senken und Zurückfahren. Die Schaufellast wächst beim Graben und fällt beim Entleeren.", 12, 30, True,
         [K(0, "dig", DIG_A, DIG_B), K(1.5, "dig", DIG_A + 1.5, 0), K(2.8, "carry", 0, 25), K(4.0, "carry", 0, 0), K(6.0, "raising", 30, 10), K(7.5, "dump-high", DUMP_A, 10), K(8.6, "dump-high", DUMP_A, DUMP_B), K(10.0, "carry", 6, -10), K(12.0, "dig", DIG_A, DIG_B)],
         [{"anchor": a, "fromS": 0, "toS": 12} for a in CONTACTS],
         [phase("dig", 0, 3, "Digging and rolling back", "Graben und Zurückkippen"), phase("carry", 3, 5, "Carry", "Fahren"), phase("dump", 5, 9, "Raise and dump", "Heben und Entleeren"), phase("return", 9, 12, "Lower and return", "Senken und Zurückfahren")],
         [event(0, "operating", "Engine at loading level", "Motor auf Ladestufe", "engine-load"), event(2.8, "load-on", "Bucket full", "Schaufel voll", "bucket-load"), event(8.6, "load-off", "Bucket dumped", "Schaufel entleert", "bucket-load")],
         [track("operating:engine-load", [(0, 1), (3, 0.7), (6, 0.7), (10, 0.7), (12, 1)]), track("operating:hydraulic", [(0, 0.9), (4, 0.2), (5, 0.9), (9, 0.9), (10.5, 0.3), (12, 0.9)]), track("emitter:exhaust-stack-jet", [(0, 1)], "step"), track("emitter:radiator-air", [(0, 1)], "step"),
          track("load:bucket-load", [(0, 0.1), (2.6, 1), (8.0, 1), (8.9, 0.05), (12, 0.1)])]),
    clip("start-and-lift", "Start, lift and shut down", "Anlassen, Heben und Abstellen", "A 10 s sequence: key on, the arm lifts from the ground, the bucket is dumped high and everything is lowered and shut down.", "Eine Folge von 10 s: Zündung, der Arm hebt vom Boden ab, die Schaufel wird hoch gekippt, alles wird abgesenkt und abgestellt.", 10, 30, False,
         [K(0, "parked-off", PARK_A, PARK_B), K(1, "parked-off", PARK_A, PARK_B), K(3.5, "raising", 30, 10), K(5.0, "dump-high", DUMP_A, 10), K(6.0, "dump-high", DUMP_A, DUMP_B), K(8.0, "carry", 6, -10), K(10, "parked-off", PARK_A, PARK_B)],
         [{"anchor": a, "fromS": 0, "toS": 10} for a in CONTACTS],
         [phase("start", 0, 1, "Start-up", "Anlassen"), phase("lift", 1, 6, "Lifting and dumping", "Heben und Entleeren"), phase("stow", 6, 10, "Lowering and shutdown", "Senken und Abstellen")],
         [event(0.4, "ignition", "Ignition", "Zündung", "exhaust-stack-jet"), event(9.4, "shutdown", "Engine off", "Motor aus", "exhaust-stack-jet")],
         [track("operating:engine-load", [(0, 0), (0.4, 0.2), (1.5, 0.7), (8, 0.7), (9.2, 0.2), (9.4, 0)]), track("operating:hydraulic", [(0, 0), (1, 0.1), (1.5, 0.9), (8, 0.9), (9, 0.1), (9.4, 0)]),
          track("emitter:exhaust-stack-jet", [(0, 0), (0.4, 1), (9.4, 0)], "step"), track("emitter:radiator-air", [(0, 0), (0.4, 1), (9.4, 0)], "step")]),
]


def build_reps():
    r = {}
    r["500"] = rep(500, 350, 1000, pens(0.15, None, None, 0.13, 0.25), [], body500(), [
        tx("Rear body with hood and counterweight, front body with tower, cab, a single glass band, the lift arm as one bar with its cylinders, the bucket as one wedge, two tyre discs and the exhaust stack.", "Hinterwagen mit Haube und Gegengewicht, Vorderwagen mit Stütze, Kabine, ein einziges Glasband, der Hubarm als eine Leiste mit seinen Zylindern, die Schaufel als ein Keil, zwei Reifenscheiben und das Auspuffrohr."),
        tx("The cranked arm outline, the bucket with cutting edge and the stack at 0.18 m (0.36 mm); the overall 8.6 m length in the carry pose.", "Die abgekröpfte Armkontur, die Schaufel mit Schneide und das Rohr mit 0,18 m (0,36 mm); die Gesamtlänge in Transportstellung."),
        tx("Hood and counterweight form one rear body, frame and tower one front body, cylinders merge into the arm, rim and hub into the tyre disc.", "Haube und Gegengewicht bilden einen Hinterwagen, Rahmen und Stütze einen Vorderwagen, Zylinder gehen im Arm auf, Felge und Nabe in der Reifenscheibe."),
        tx("Cylinders (0.2 m but not separable from the arm at 0.1 mm gap), rims, tread, steps, mirrors, rails, door.", "Zylinder (0,2 m, vom Arm bei 0,1 mm Abstand nicht trennbar), Felgen, Profil, Tritte, Spiegel, Geländer, Tür."),
        tx("Only the stack is thin (0.36 mm); it is drawn at real width.", "Nur das Rohr ist dünn (0,36 mm); es ist in Realbreite gezeichnet."),
        tx("None.", "Keine."),
        tx("Bodies mid-dark, bucket and tyres darkest, glass lightest; pens 0.15 mm, glass 0.13 mm.", "Wagenteile mitteldunkel, Schaufel und Reifen am dunkelsten, Glas am hellsten; Stifte 0,15 mm, Glas 0,13 mm."),
        tx("Rear body carries engine-bay and counterweight, front body the front-frame, the arm shape carries lift-arm and hydraulics (merged).", "Der Hinterwagen trägt Motorraum und Gegengewicht, der Vorderwagen den Rahmen, die Armform trägt Hubarm und Hydraulik (zusammengefasst)."),
    ])
    r["200"] = rep(200, 150, 350, pens(0.18, 0.13, None, 0.13, 0.25), [], body200(), [
        tx("Hood and counterweight as separate masses, front frame, cab with two glass panes, arm, four visible cylinder parts, bucket, tyre with rim disc.", "Haube und Gegengewicht als getrennte Massen, Vorderwagen, Kabine mit zwei Glasscheiben, Arm, vier sichtbare Zylinderteile, Schaufel, Reifen mit Felgenscheibe."),
        tx("Cylinder barrels and rods that extend with the arm, the pillar between the panes, the rim disc.", "Zylinderrohre und -stangen, die mit dem Arm ausfahren, der Pfosten zwischen den Scheiben, die Felgenscheibe."),
        tx("Mirrors, handrails and steps merge into the surrounding forms.", "Spiegel, Handläufe und Tritte gehen in den umgebenden Formen auf."),
        tx("Tread, bolts, hinges, handles, louvres.", "Profil, Bolzen, Scharniere, Griffe, Lüftungsschlitze."),
        tx("Rods are 0.1 m (0.5 mm) and 0.09 m (0.45 mm); stack 0.18 m (0.9 mm); all at real width.", "Stangen sind 0,1 m (0,5 mm) und 0,09 m (0,45 mm); Rohr 0,18 m (0,9 mm); alle in Realbreite."),
        tx("None.", "Keine."),
        tx("Tyres darkest, rods lightest, bucket dark, frame mid-tone.", "Reifen am dunkelsten, Stangen am hellsten, Schaufel dunkel, Rahmen mittlerer Ton."),
        tx("Every region has its own form; the four cylinder parts all carry hydraulics.", "Jede Region hat eine eigene Form; alle vier Zylinderteile tragen die Hydraulik."),
    ])
    r["100"] = rep(100, 75, 150, pens(0.25, 0.18, None, 0.13, 0.35), [], body100(False), [
        tx("As 1:200 with rear grille, handrail, access step, bucket cutting edge and wheel hubs with six bolt circles.", "Wie 1:200 mit hinterem Gitter, Handlauf, Einstieg, Schaufelschneide und Radnaben mit sechs Bolzenkreisen."),
        tx("Hub and six bolt outlines turn with the wheel.", "Nabe und sechs Bolzenumrisse drehen mit dem Rad."),
        tx("Tyre lugs merge into the tyre outline.", "Reifenstollen gehen in der Reifenkontur auf."),
        tx("Lugs, mirrors, handles, lamps.", "Stollen, Spiegel, Griffe, Leuchten."),
        tx("Bolts are 0.09 m (0.9 mm) outlines.", "Bolzen sind Umrisse von 0,09 m (0,9 mm)."),
        tx("None.", "Keine."),
        tx("As 1:200 with detail pen 0.18 mm.", "Wie 1:200 mit Detailstift 0,18 mm."),
        tx("As 1:200; hub and bolts carry the tyres region.", "Wie 1:200; Nabe und Bolzen tragen die Reifenregion."),
    ])
    r["50"] = rep(50, 20, 75, pens(0.35, 0.25, 0.13, 0.18, 0.5), [], body100(True), [
        tx("All 1:100 parts plus 20 tread lugs per tyre, lamp, mirror, door handle and hood louvres.", "Alle 1:100-Teile plus 20 Profilstollen je Reifen, Leuchte, Spiegel, Türgriff und Lüftungsschlitze."),
        tx("Tread lugs of 0.09 m (1.8 mm) give a visible rotation of the tyres.", "Profilstollen von 0,09 m (1,8 mm) machen die Reifendrehung sichtbar."),
        tx("Nothing merged.", "Nichts zusammengefasst."),
        tx("Teeth of the cutting edge, hinges, rails.", "Zähne der Schneide, Scharniere, Geländer."),
        tx("Lamp lens 0.05 m wide is drawn as an outline.", "Das 0,05 m breite Leuchtenglas ist als Umriss gezeichnet."),
        tx("None.", "Keine."),
        tx("Heavier pens 0.35 / 0.25 mm.", "Stärkere Stifte 0,35 / 0,25 mm."),
        tx("As 1:100.", "Wie 1:100."),
    ])
    return r

def symbols():
    def box(x0, y0, x1, y1):
        return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
    ang = 35
    sh = [G.poly("rear-frame-plan", "rear-frame", "body", 1, box(-3.5, -0.9, 0, 0.9), tone=0.5)]
    for x in (XR,):
        for y in (-1.1, 1.1):
            sh.append(G.poly(f"rear-tyre-{'l' if y < 0 else 'r'}", "rear-frame", "body", 2, box(x - WR, y - 0.375, x + WR, y + 0.375), tone=0.15))
    sh.append(G.poly("front-frame-plan", "rear-frame", "body", 1.5, rot_pts(box(0, -0.8, 2.4, 0.8), 0, 0, ang), tone=0.45))
    for y in (-1.1, 1.1):
        sh.append(G.poly(f"front-tyre-{'l' if y < 0 else 'r'}", "rear-frame", "body", 2, rot_pts(box(XF - WR, y - 0.375, XF + WR, y + 0.375), 0, 0, ang), tone=0.15))
    sh.append(G.poly("bucket-plan", "rear-frame", "body", 1.6, rot_pts(box(3.6, -1.5, 5.4, 1.5), 0, 0, ang), tone=0.3))
    sh.append(G.poly("lift-arm-plan", "rear-frame", "detail", 1.7, rot_pts(box(1.1, -0.85, 3.9, -0.6), 0, 0, ang), tone=0.4, filled=False))
    sh.append(G.poly("lift-arm-plan-2", "rear-frame", "detail", 1.7, rot_pts(box(1.1, 0.6, 3.9, 0.85), 0, 0, ang), tone=0.4, filled=False))
    sh.append(G.ell("articulation-joint", "rear-frame", "detail", 3, 0, 0, 0.25, tone=0.1))
    return [{"id": "articulated-steering-plan", "title": tx("Plan view at full articulated steering lock (35 degrees)", "Grundriss bei vollem Knickeinschlag (35 Grad)"), "description": tx("Plan symbol of the loader with the front frame, tyres and bucket turned 35 degrees about the vertical articulation joint.", "Planzeichen des Laders mit um 35 Grad um das senkrechte Knickgelenk eingeschlagenem Vorderwagen, Reifen und Schaufel."),
             "semantics": "true-extent", "units": "model-m", "denominators": [500, 200], "pens": pens(0.25, 0.18, None, None, 0.25), "hatches": [], "geometry": {"elevation": [], "plan": sh},
             "dimensionalNote": tx("True extent in model metres: frame width 2.4 m, tyres 1.75 m diameter and 0.75 m wide, bucket 3.0 m wide, front frame rotated 35 degrees; real size at both listed scales.", "Wahre Ausdehnung in Modellmetern: Rahmenbreite 2,4 m, Reifen 1,75 m Durchmesser und 0,75 m breit, Schaufel 3,0 m breit, Vorderwagen um 35 Grad gedreht; in beiden gelisteten Maßstäben in Realgröße.")}]


def asset():
    a = {
        "id": "wheel-loader", "category": "machinery", "title": tx("Articulated wheel loader, 20 t", "Knickgelenkter Radlader, 20 t"),
        "summary": tx("A 20 t articulated wheel loader with a 3.5 m3 bucket in side elevation: rear frame with hood and counterweight, front frame with cab and arm towers, a lift arm and bucket driven by closed-pin lift and tilt cylinders, tyres with lugs and a vertical exhaust stack. Steering articulation is shown in a plan symbol.",
                      "Ein knickgelenkter Radlader von 20 t mit 3,5-m3-Schaufel in der Seitenansicht: Hinterwagen mit Haube und Gegengewicht, Vorderwagen mit Kabine und Armstützen, Hubarm und Schaufel mit Hub- und Kippzylindern mit geschlossenen Bolzen, Stollenreifen und senkrechtes Auspuffrohr. Die Knicklenkung zeigt ein Grundrisszeichen."),
        "tags": ["machine", "wheel-loader", "articulated", "diesel", "hydraulic", "construction", "wheeled"],
        "provenance": {"author": "Semio staffage content worker NW-MA", "method": "handcrafted", "created": "2026-10-29",
                       "references": references(["iso-6746", "heywood-2018", "incropera-2011", "ashrae-2021", "class-data-note"]),
                       "notes": tx("Class data of a 20 t wheel loader with 3.5 m3 bucket and about 190 kW diesel (23.5R25 tyres, 3.4 m wheelbase). The articulation joint turns about a vertical axis and is not an elevation joint; the plan symbol shows it. The tilt cylinder is drawn acting directly on the bucket (the real Z-bar linkage has a bell crank). All machine numbers are authored estimates.",
                                   "Klassendaten eines 20-t-Radladers mit 3,5-m3-Schaufel und etwa 190-kW-Diesel (Reifen 23.5R25, Radstand 3,4 m). Das Knickgelenk dreht um eine senkrechte Achse und ist kein Gelenk der Ansicht; das Grundrisszeichen zeigt es. Der Kippzylinder ist direkt an der Schaufel angreifend gezeichnet (die echte Z-Kinematik hat einen Kipphebel). Alle Maschinenzahlen sind Schätzungen.")},
        "dimensions": {"heightM": 0, "widthM": 0, "depthM": 2.95, "massKg": 20500},
        "materials": materials, "rig": rig.doc(), "anchors": anchors, "representations": build_reps(), "symbols": symbols(),
        "poses": poses, "clips": clips,
        "thermal": {"environment": DEFAULT_ENV, "passengers": [], "operating": ops, "regions": regions, "emitters": emitters,
                    "assumptions": [tx("Engine class 190 kW: exhaust 0.0023 kg/s per kW = 0.43 kg/s at 380 C (rho 0.52 kg/m3) gives 0.83 m3/s through a 0.13 m stack (63 m/s) and 0.04 kg/kg vapour (Heywood 2018); flow and rise scale linearly with the engine level.", "Motorklasse 190 kW: Abgas 0,0023 kg/s je kW = 0,43 kg/s bei 380 C (rho 0,52 kg/m3) ergibt 0,83 m3/s durch ein 0,13-m-Rohr (63 m/s) und 0,04 kg/kg Dampf (Heywood 2018); Volumenstrom und Erhöhung skalieren linear mit der Motorstufe."),
                                    tx("Radiator: about 110 kW coolant heat, +25 K air gives 4.0 m3/s leaving at the rear grille. Heat into shells at level 1: hood 3 kW, stack 3.5 kW, hydraulic cylinders 2 kW, tyres 6 kW (sidewall share of rolling resistance at 30 km/h). Authored estimates.", "Kühler: etwa 110 kW Kühlwasserwärme, +25 K Luft ergibt 4,0 m3/s am hinteren Gitter. Wärme in die Hüllen bei Stufe 1: Haube 3 kW, Rohr 3,5 kW, Hydraulikzylinder 2 kW, Reifen 6 kW (Flankenanteil des Rollwiderstands bei 30 km/h). Schätzungen."),
                                    tx("Heat capacities are active shell masses. Bucket content is a mass load only.", "Wärmekapazitäten sind aktive Schalenmassen. Die Schaufelfüllung ist nur eine Massenlast.")]},
        "structure": structure("Two tyre patch groups share the load as unilateral supports; the bucket content (rated 6 t) is a point load at the bucket content centre; the lift and tilt motion is quasi-static. The tipping line about the front axle appears when the arm is raised.", "Zwei Reifenaufstandsgruppen teilen die Last als einseitige Auflager; die Schaufelfüllung (Nennlast 6 t) ist eine Einzellast im Schwerpunkt der Füllung; Heben und Kippen sind quasi-statisch. Die Kipplinie um die Vorderachse erscheint bei gehobenem Arm."),
        "presentation": {"defaultPose": "carry", "defaultClip": "load-cycle", "defaultScale": 200, "viewpoints": ["elevation"]},
        "exports": EXPORTS, "completion": "authored",
    }
    x0, x1, y0, y1 = bbox(a, rig, 50, poses[0]["bones"])
    a["dimensions"]["widthM"] = round(x1 - x0, 2)
    a["dimensions"]["heightM"] = round(y1 - 0, 2)
    return a


if __name__ == "__main__":
    a = asset()
    for p in selfcheck(a):
        print("PROBLEM", p)
    for d in (500, 200, 100, 50):
        print(d, [round(v, 2) for v in bbox(a, rig, d, poses[0]["bones"])])
    print(a["dimensions"], write(a, "wheel-loader"))
