"""🛴 Round 3, 1:100 of vehicles/electric-kick-scooter, authored for 15 mm features: the deck with nose and tail as one plate body, two plain stays, a four-section gooseneck, the head tube with its headset as one cylinder, a fork of crown and two plates, plain arc mudguards, plain tyres, the hub motor and the rear wheel as dished discs, the stem with hinge foot and cockpit as one turned body and the handlebar with two grips; thermal entries, load path and the three drawings."""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


pt = load("r3-electric-kick-scooter-parts")
kit = pt.kit
AF, AR, P = pt.AF, pt.AR, pt.P
RG = pt.REGION
sp, tp = kit.steer_pt, kit.stem_pt
AV = pt.AXIS_V
PENS = {"body": 0.18, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35}
ALU, RUB, TAPE, PLA, STEEL, MOT = "anodised", "rubber", "griptape", "plastic", "steel-disc", "motor-alloy"

DECK_PLAN = [[-0.075, 0.246], [-0.06, 0.264], [0.06, 0.264], [0.075, 0.246], [0.075, -0.294], [0.0575, -0.316], [-0.0575, -0.316], [-0.075, -0.294]]
STAY = [[-0.3, 0.096], [-0.3, 0.15], [-0.4, 0.13]] + pt.arc(AR, 0.022, 100, 260, 4) + [[-0.4, 0.088]]
STAY_Z = (0.0425, 0.0575)
NECK = [((0.228, 0.124), 0.056, 0.026), ((0.27, 0.146), 0.042, 0.027), ((0.33, 0.228), 0.027, 0.025), ((0.396, 0.303), 0.023, 0.026)]
HEAD_Y = (0.254, 0.353)
TYRE = [[-0.02, 0.068], [-0.025, 0.08], [-0.025, 0.096], [-0.014, 0.108], [0.014, 0.108], [0.025, 0.096], [0.025, 0.08], [0.02, 0.068], [-0.02, 0.068]]
MOTOR = [[-0.033, 0.0], [-0.033, 0.05], [-0.0225, 0.07], [-0.0225, 0.077], [0.0225, 0.077], [0.0225, 0.07], [0.033, 0.05], [0.033, 0.0]]
WHEEL = [[-0.04, 0.0], [-0.04, 0.02], [-0.0075, 0.03], [-0.0075, 0.062], [-0.0225, 0.066], [-0.0225, 0.077], [0.0225, 0.077], [0.0225, 0.066], [0.0075, 0.062], [0.0075, 0.03], [0.04, 0.02], [0.04, 0.0]]
GUARD_R = (0.109, 0.121)
BLADE_Z = (0.034, 0.049)
STEM = [[0.0, 0.027], [0.03, 0.027], [0.058, 0.022], [0.67, 0.0185], [0.69, 0.026], [0.737, 0.026]]


def many(S, i, regions):
    """🧩 Binds the drawn shape i to several thermal regions (a merged form)."""
    for shapes in S.shapes.values():
        for s in shapes:
            if s["id"] == i:
                s.pop("region", None)
                s["regions"] = regions


def deck(S):
    y0, y1 = pt.DECK_Y
    S.put(S.solid("deck", "extrude", "deck", ALU, 0.6, profile=DECK_PLAN, plane="zx", **{"from": y0, "to": y1}), RG["frame"])
    S.flat("elevation", "deck", "deck", "body", 0.3, None, ALU, [[-0.316, y0], [0.264, y0], [0.264, y1], [-0.316, y1]], [0.264, y1, -0.075], tol=0, regions=[RG["frame"], RG["mat"]])
    S.flat("front", "deck", "deck", "body", 0.3, None, ALU, [[-0.075, y0], [0.075, y0], [0.075, y1], [-0.075, y1]], [0.264, y1, -0.075], tol=0, regions=[RG["frame"], RG["mat"]])
    S.flat("plan", "deck", "deck", "body", 0.3, None, ALU, [[x, z] for z, x in DECK_PLAN], [0.264, y1, -0.075], tol=0, regions=[RG["frame"], RG["mat"]])
    girder = [[0.097, -0.073], [0.097, 0.073], [0.148, 0.073], [0.148, -0.073]]
    S.struct(S.solid("deck-girder", "extrude", "deck", ALU, 0.5, profile=girder, holes=[kit.inset(girder, 0.003)], plane="yz", **{"from": -0.29, "to": 0.24}), "deck")
    mat = S.box("mat", "deck", [pt.MAT_X[1] - pt.MAT_X[0], 0.0015, 2 * pt.MAT_HALF], [(pt.MAT_X[0] + pt.MAT_X[1]) / 2, y1 + 0.00075, 0.0], TAPE, 0.15)
    mat["layer"] = "tape"
    S.therm(mat, RG["mat"], own=True)
    S.therm(S.box("bms", "deck", [0.09, 0.02, 0.1], [0.17, 0.123, 0.0], PLA, 0.2, role="detail"), RG["bms"], own=True)
    S.struct(S.box("mass-battery", "deck", [0.36, 0.036, 0.11], [-0.04, 0.122, 0.0], STEEL, 0.2, role="detail"), "deck", "mass")
    for side, sign in (("l", 1), ("r", -1)):
        z0, z1 = sorted((sign * STAY_Z[0], sign * STAY_Z[1]))
        S.put(S.prism("stay-" + side, "deck", STAY, z0, z1, ALU, 0.6), RG["frame"])
        S.struct_of("stay-" + side, "deck")
        S.swept("stay-" + side, "deck", STAY, z0, z1, "body", 0.3, RG["frame"], ALU, tol=0)
    S.put(S.tube("neck", "deck", [([p[0][0], p[0][1], 0.0], p[1], p[2], 4.0) for p in NECK], ALU, 0.6), RG["frame"])
    S.struct_of("neck", "deck")
    S.flat("elevation", "neck", "deck", "body", 0.3, RG["frame"], ALU, pt.neck_side(NECK), [0.42, 0.33, -0.056], tol=0.001)
    S.flat("plan", "neck", "deck", "body", 0.3, RG["frame"], ALU, pt.neck_plan(NECK), [0.42, 0.33, -0.056], tol=0.001)
    S.flat("front", "neck", "deck", "body", 0.3, RG["frame"], ALU, pt.neck_front(NECK), [0.3, 0.33, -0.056], tol=0.001)
    lo, hi = pt.axis_pt(HEAD_Y[0]), pt.axis_pt(HEAD_Y[1])
    S.put(S.cyl("head-tube", "deck", lo, hi, 0.027, ALU, 0.6), RG["frame"])
    S.struct_of("head-tube", "deck")
    S.swept("head-tube", "deck", pt.quad(lo, hi, 0.027), -0.027, 0.027, "body", 0.3, RG["frame"], ALU, tol=0)
    for i, y in (("bearing-lower", HEAD_Y[0]), ("bearing-upper", HEAD_Y[1] - 0.015)):
        a, b = pt.axis_pt(y), pt.axis_pt(y + 0.015)
        S.struct(S.cyl(i, "deck", a, b, 0.0255, STEEL, 0.3, role="detail"), "fork", "joint")


def rear(S):
    a0, a1 = pt.GUARD_REAR
    outline = pt.band(AR, GUARD_R[0], GUARD_R[1], a0, a1, 12)
    S.put(S.prism("fender-r", "deck", outline, -0.03, 0.03, PLA, 0.3), RG["guard"])
    S.swept("fender-r", "deck", outline, -0.03, 0.03, "body", 0.5, RG["guard"], PLA, tol=0.0008)
    S.put(S.rev("tyre-r", "wheel-r", TYRE, AR + (0.0,), [0, 0, 1], RUB, 0.07), RG["tyre"])
    pt.ring2d(S, "tyre-r", "wheel-r", AR, 0.108, 0.07, -0.025, 0.025, "body", 0.1, RG["tyre"], RUB)
    S.put(S.rev("wheel-disc-r", "wheel-r", WHEEL, AR + (0.0,), [0, 0, 1], ALU, 0.6), RG["rim"])
    S.struct_of("wheel-disc-r", "wheel-r")
    S.disc("wheel-disc-r", "wheel-r", AR, 0.154, -0.0225, 0.0225, "body", 0.75, None, ALU)
    many(S, "wheel-disc-r", [RG["rim"], RG["rotor"]])
    S.disc("hub-r", "wheel-r", AR, 0.04, -0.04, 0.04, "detail", 0.45, RG["rim"], ALU)
    z0, z1 = pt.ROTOR_Z
    S.therm(S.rev("disc-r", "wheel-r", [[0, pt.ROTOR[0]], [z1 - z0, pt.ROTOR[0]], [z1 - z0, pt.ROTOR[1]], [0, pt.ROTOR[1]], [0, pt.ROTOR[0]]], AR + (z0,), [0, 0, 1], STEEL, 0.7, role="texture"), RG["rotor"], own=True)
    S.struct(S.cyl("axle-r", "wheel-r", AR + (-0.0575,), AR + (0.0575,), 0.0075, STEEL, 0.3, role="detail"), "wheel-r", "joint")


def fork(S):
    S.put(S.cyl("hinge-base", "fork", sp(0.0), sp(0.0273), 0.027, ALU, 0.5), RG["steer"])
    S.swept("hinge-base", "fork", pt.quad(sp(0.0), sp(0.0273), 0.027), -0.027, 0.027, "body", 0.3, RG["steer"], ALU, tol=0)
    S.struct(S.cyl("steerer", "fork", sp(0.0), sp(0.134), 0.0145, STEEL, 0.4), "fork")
    S.struct(S.cyl("hinge-pin", "fork", [P[0], P[1], -0.027], [P[0], P[1], 0.027], 0.0085, STEEL, 0.3, role="detail"), "stem", "joint")
    S.put(S.box("crown", "fork", [0.028, 0.044, 0.102], sp(0.1415, 0.0015), ALU, 0.55), RG["steer"])
    S.struct_of("crown", "fork")
    S.swept("crown", "fork", pt.quad(sp(0.1275, 0.0015), sp(0.1555, 0.0015), 0.022), -0.051, 0.051, "body", 0.35, RG["steer"], ALU, tol=0)
    blade = pt.blade_outline()
    blade = [blade[0]] + blade[1:-1:3] + [blade[-1]]
    for side, sign in (("l", 1), ("r", -1)):
        z0, z1 = sorted((sign * BLADE_Z[0], sign * BLADE_Z[1]))
        S.put(S.prism("blade-" + side, "fork", blade, z0, z1, ALU, 0.6), RG["steer"])
        S.struct_of("blade-" + side, "fork")
        S.swept("blade-" + side, "fork", blade, z0, z1, "body", 0.4, RG["steer"], ALU, tol=0)
    a0, a1 = pt.GUARD_FRONT
    outline = pt.band(AF, GUARD_R[0], GUARD_R[1], a0, a1, 8)
    S.put(S.prism("fender-f", "fork", outline, -0.03, 0.03, PLA, 0.3), RG["guard"])
    S.swept("fender-f", "fork", outline, -0.03, 0.03, "body", 0.5, RG["guard"], PLA, tol=0.0008)
    S.put(S.rev("tyre-f", "wheel-f", TYRE, AF + (0.0,), [0, 0, 1], RUB, 0.07), RG["tyre"])
    pt.ring2d(S, "tyre-f", "wheel-f", AF, 0.108, 0.07, -0.025, 0.025, "body", 0.1, RG["tyre"], RUB)
    S.put(S.rev("motor-f", "wheel-f", MOTOR, AF + (0.0,), [0, 0, 1], MOT, 0.5), RG["motor"])
    S.struct_of("motor-f", "wheel-f")
    S.disc("motor-f", "wheel-f", AF, 0.154, -0.033, 0.033, "body", 0.75, RG["motor"], MOT)
    S.struct(S.cyl("axle-f", "wheel-f", AF + (-0.052,), AF + (0.052,), 0.0075, STEEL, 0.3, role="detail"), "wheel-f", "joint")
    S.struct(S.cyl("mass-motor", "wheel-f", AF + (-0.02,), AF + (0.02,), 0.05, STEEL, 0.2, role="detail"), "wheel-f", "mass")


def stem(S):
    up = [-kit.DOWN[0], -kit.DOWN[1], 0.0]
    S.put(S.rev("stem", "stem", STEM + [[0.737, 0.0]], tp(0.0, AV), up, ALU, 0.65), RG["steer"])
    S.struct_of("stem", "stem")
    for i, line, half in (("stem", STEM[2:4], 0.02), ("stem-foot", STEM[:3], 0.027), ("stem-head", STEM[3:], 0.026)):
        outline = [tp(u, AV + r)[:2] for u, r in line] + [tp(u, AV - r)[:2] for u, r in line[::-1]]
        S.swept(i, "stem", outline, -half, half, "body", 0.4, RG["steer"], ALU, tol=0)
    B = tp(pt.BAR_U, AV)
    S.put(S.cyl("bar", "stem", B[:2] + [-pt.GRIP[0]], B[:2] + [pt.GRIP[0]], 0.011, ALU, 0.65), RG["steer"])
    S.struct_of("bar", "stem")
    S.disc("bar", "stem", B, 0.022, -pt.GRIP[0], pt.GRIP[0], "body", 0.4, RG["steer"], ALU)
    for side, sign in (("l", 1), ("r", -1)):
        z0, z1 = sorted((sign * pt.GRIP[0], sign * pt.GRIP[1]))
        S.put(S.cyl("grip-" + side, "stem", B[:2] + [z0], B[:2] + [z1], 0.017, RUB, 0.1), RG["steer"])
        S.disc("grip-" + side, "stem", B, 0.034, z0, z1, "body", 0.1, RG["steer"], RUB)
    S.struct(S.ball("mass-stem", "stem", [0.09, 0.014, 0.014], tp(0.36, AV), STEEL, 0.2, role="detail"), "stem", "mass")


def build():
    S = kit.Scale(100, PENS)
    deck(S)
    rear(S)
    fork(S)
    stem(S)
    return S


DECISIONS = [
    {"en": "Body at 1:100, authored for 15 mm features: the deck with nose and tail is one closed plate body cut from the plan outline, the stays are two plain 15 mm plates, the gooseneck has four sections, head tube and headset are one cylinder, the fork is the hinge base, a crown block and two 15 mm plates, the mudguards are plain 12 mm arc bands, the tyres have a plain rounded section, the hub motor is one dished body, the rear wheel one dished disc with its hub, the stem with hinge foot and cockpit head is one turned body, the handlebar a bar with two plain grips. Mat, side stand, caliper, rotor, tail light, catch, levers, bell, display and headlight are merged away.", "de": "Körper bei 1:100, für 15-mm-Einzelheiten entworfen: Das Trittbrett mit Bug und Heck ist ein geschlossener Plattenkörper aus dem Grundrissumriss, die Streben sind zwei glatte 15-mm-Platten, der Schwanenhals hat vier Querschnitte, Steuerrohr und Steuersatz sind ein Zylinder, die Gabel besteht aus Gelenksockel, Kronenblock und zwei 15-mm-Platten, die Schutzbleche sind glatte 12-mm-Bogenbänder, die Reifen haben einen einfachen gerundeten Querschnitt, der Nabenmotor ist ein gewölbter Körper, das Hinterrad eine gewölbte Scheibe mit Nabe, die Lenkstange mit Gelenkfuß und Cockpitkopf ist ein Drehkörper, der Lenker eine Stange mit zwei glatten Griffen. Matte, Seitenständer, Bremssattel, Bremsscheibe, Rücklicht, Haken, Hebel, Klingel, Anzeige und Scheinwerfer sind zusammengefasst."},
    {"en": "Thermal: body surfaces by reference; the grip mat is its own 1.5 mm tape layer on the deck, the controller a hidden box in the deck nose, the brake rotor its own 3 mm ring beside the rear wheel disc.", "de": "Thermik: Körperoberflächen per Verweis; die Griffmatte ist eine eigene 1,5-mm-Belagschicht auf dem Trittbrett, die Steuerung ein verdeckter Kasten im Trittbrettbug, die Bremsscheibe ein eigener 3-mm-Ring neben der Hinterradscheibe."},
    {"en": "Structure: the hollow deck box girder (3 mm wall) with stays, neck and head tube; the steerer through the head tube, crown and blades; the stem and the bar; the motor shell and the rear wheel disc; joints at both head bearings, the hinge pin and the two axles; masses at the battery pack, the motor stator and the stem.", "de": "Tragwerk: der hohle Trittbrett-Kastenträger (3 mm Wand) mit Streben, Hals und Steuerrohr; der Gabelschaft durch das Steuerrohr, Krone und Scheiden; Lenkstange und Lenker; Motorgehäuse und Hinterradscheibe; Gelenke an beiden Steuerlagern, am Gelenkbolzen und an den zwei Achsen; Massen an Akkupack, Motorstator und Lenkstange."},
]

DRAWN = {
    "silhouette": {"en": "All three views are the true orthographic views of the 1:100 body: tyre rings, motor and rear wheel discs with the rear hub, deck plate, stays, gooseneck, head tube, hinge base, fork crown and blades, both mudguard arcs, the stem with hinge foot and cockpit head, the handlebar and the grips.", "de": "Alle drei Ansichten sind die wahren orthogonalen Ansichten des 1:100-Körpers: Reifenringe, Motor- und Hinterradscheibe mit der Hinternabe, Trittbrettplatte, Streben, Schwanenhals, Steuerrohr, Gelenksockel, Gabelkrone und -scheiden, beide Schutzblechbögen, die Lenkstange mit Gelenkfuß und Cockpitkopf, der Lenker und die Griffe."},
    "retained": {"en": "Wheels, hub motor, deck, stays, neck, head tube, fork, mudguards, stem, handlebar and grips.", "de": "Räder, Nabenmotor, Trittbrett, Streben, Hals, Steuerrohr, Gabel, Schutzbleche, Lenkstange, Lenker und Griffe."},
    "merged": {"en": "Deck, nose, tail, grip mat and controller are one plate; head tube and headset are one cylinder; rim, spokes and brake rotor are one rear wheel disc; hinge foot, stem and cockpit are one body.", "de": "Trittbrett, Bug, Heck, Griffmatte und Steuerung sind eine Platte; Steuerrohr und Steuersatz sind ein Zylinder; Felge, Speichen und Bremsscheibe sind eine Hinterradscheibe; Gelenkfuß, Lenkstange und Cockpit sind ein Körper."},
    "omitted": {"en": "Side stand, caliper, tail light, latch catch, fold lever, brake lever, throttle, bell, headlight, axles, tread.", "de": "Seitenständer, Bremssattel, Rücklicht, Rasthaken, Klapphebel, Bremshebel, Gasgriff, Klingel, Scheinwerfer, Achsen, Profil."},
    "thinElements": {"en": "The 12 mm mudguard bands are 0.12 mm and the 15 mm stays and fork plates 0.15 mm on paper, drawn as filled shapes with a 0.18 mm outline; the 22 mm handlebar is 0.22 mm.", "de": "Die 12 mm starken Schutzblechbänder sind 0,12 mm und die 15 mm starken Streben und Gabelplatten 0,15 mm auf dem Papier, als gefüllte Formen mit 0,18 mm Kontur gezeichnet; der 22-mm-Lenker ist 0,22 mm."},
    "hatching": {"en": "None.", "de": "Keine."},
    "styles": {"en": "Dark tyres and grips, light wheel discs, mid-tone frame, fork and stem, lighter mudguards.", "de": "Dunkle Reifen und Griffe, helle Radscheiben, mitteltoniger Rahmen, Gabel und Lenkstange, hellere Schutzbleche."},
    "overlays": {"en": "The deck plate paints battery enclosure and grip mat merged, the rear wheel disc paints rim and brake rotor merged; the controller is hidden. Steering: the fork bone runs from the steering head down the steering axis (8 degrees rake) to the front axle and carries the front wheel and the stem, so the steered braking pose is a twist about that axis; the stem bone starts at the hinge pin behind the steering head and folds by 95 degrees.", "de": "Die Trittbrettplatte zeigt Akkugehäuse und Griffmatte vereint, die Hinterradscheibe Felge und Bremsscheibe vereint; die Steuerung ist verdeckt. Lenkung: Der Gabelknochen läuft vom Steuerkopf die Lenkachse (8 Grad Neigung) hinab zur Vorderachse und trägt Vorderrad und Lenkstange, die gelenkte Bremspose ist eine Drehung um diese Achse; der Lenkstangenknochen beginnt am Gelenkbolzen hinter dem Steuerkopf und klappt um 95 Grad."},
}
