"""🛴 Round 3, 1:200 of vehicles/electric-kick-scooter, authored for 30 mm features: seven masses that still read as a folding e-scooter at a glance (the deck plate with its rear fork arms, the gooseneck with the steering head, the fork yoke, two wheel discs, the stem and the handlebar); mudguards, mat, motor, rim, brake rotor and controller live in the thermal list, the box girder, axles and masses in the structure list."""
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
PENS = {"body": 0.18, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.25}
ALU, RUB, TAPE, PLA, STEEL, MOT = "anodised", "rubber", "griptape", "plastic", "steel-disc", "motor-alloy"

DECK_PLAN = [[-0.075, 0.25], [-0.055, 0.266], [0.055, 0.266], [0.075, 0.25], [0.075, -0.3], [0.06, -0.452], [0.03, -0.452], [0.03, -0.316], [-0.03, -0.316], [-0.03, -0.452], [-0.06, -0.452], [-0.075, -0.3]]
NECK = [((0.235, 0.124), 0.05, 0.027), ((0.3, 0.185), 0.032, 0.028), ((0.398, 0.31), 0.027, 0.03)]
YOKE = [[-0.027, 0.0], [0.027, 0.0], [0.027, 0.1275], [0.047, 0.1275], [0.047, 0.292], [0.03, 0.292], [0.03, 0.156], [-0.03, 0.156], [-0.03, 0.292], [-0.047, 0.292], [-0.047, 0.1275], [-0.027, 0.1275]]
YOKE_V = (-0.016, 0.02)
WHEEL = [[-0.025, 0.0], [-0.025, 0.096], [-0.014, 0.108], [0.014, 0.108], [0.025, 0.096], [0.025, 0.0]]
STEM = [[0.0, 0.027], [0.05, 0.022], [0.737, 0.02]]
GUARD_R = (0.109, 0.121)


def many(S, i, regions):
    """🧩 Binds the drawn shape i to several thermal regions (a merged form)."""
    for shapes in S.shapes.values():
        for s in shapes:
            if s["id"] == i:
                s.pop("region", None)
                s["regions"] = regions


def build():
    S = kit.Scale(200, PENS)
    y0, y1 = pt.DECK_Y
    merged = [RG["frame"], RG["mat"], RG["guard"]]
    S.put(S.solid("deck", "extrude", "deck", ALU, 0.6, profile=DECK_PLAN, plane="zx", **{"from": y0, "to": y1}), RG["frame"])
    S.flat("elevation", "deck", "deck", "body", 0.3, None, ALU, [[-0.452, y0], [0.266, y0], [0.266, y1], [-0.452, y1]], [0.266, y1, -0.075], tol=0, regions=merged)
    S.flat("front", "deck", "deck", "body", 0.3, None, ALU, [[-0.075, y0], [0.075, y0], [0.075, y1], [-0.075, y1]], [0.266, y1, -0.075], tol=0, regions=merged)
    S.flat("plan", "deck", "deck", "body", 0.3, None, ALU, [[x, z] for z, x in DECK_PLAN], [0.266, y1, -0.075], tol=0, regions=merged)
    girder = [[0.097, -0.073], [0.097, 0.073], [0.148, 0.073], [0.148, -0.073]]
    S.struct(S.solid("deck-girder", "extrude", "deck", ALU, 0.5, profile=girder, holes=[kit.inset(girder, 0.003)], plane="yz", **{"from": -0.3, "to": 0.25}), "deck")
    for side, sign in (("l", 1), ("r", -1)):
        z0, z1 = sorted((sign * 0.03, sign * 0.06))
        S.struct(S.box("arm-" + side, "deck", [0.16, 0.04, 0.03], [-0.372, 0.118, (z0 + z1) / 2], ALU, 0.5), "deck")
    mat = S.box("mat", "deck", [pt.MAT_X[1] - pt.MAT_X[0], 0.0015, 2 * pt.MAT_HALF], [(pt.MAT_X[0] + pt.MAT_X[1]) / 2, y1 + 0.00075, 0.0], TAPE, 0.15)
    mat["layer"] = "tape"
    S.therm(mat, RG["mat"], own=True)
    S.therm(S.box("bms", "deck", [0.09, 0.03, 0.1], [0.17, 0.122, 0.0], PLA, 0.2, role="detail"), RG["bms"], own=True)
    S.struct(S.box("mass-battery", "deck", [0.36, 0.036, 0.11], [-0.04, 0.122, 0.0], STEEL, 0.2, role="detail"), "deck", "mass")
    S.therm(S.prism("fender-r", "deck", pt.band(AR, GUARD_R[0], GUARD_R[1], pt.GUARD_REAR[0], pt.GUARD_REAR[1], 12), -0.03, 0.03, PLA, 0.3), RG["guard"], own=True)
    S.therm(S.prism("fender-f", "fork", pt.band(AF, GUARD_R[0], GUARD_R[1], pt.GUARD_FRONT[0], pt.GUARD_FRONT[1], 8), -0.03, 0.03, PLA, 0.3), RG["guard"], own=True)
    S.put(S.tube("neck", "deck", [([p[0][0], p[0][1], 0.0], p[1], p[2], 4.0) for p in NECK], ALU, 0.6), RG["frame"])
    S.struct_of("neck", "deck")
    S.flat("elevation", "neck", "deck", "body", 0.3, RG["frame"], ALU, pt.neck_side(NECK), [0.43, 0.34, -0.05], tol=0.002)
    S.flat("plan", "neck", "deck", "body", 0.3, RG["frame"], ALU, pt.neck_plan(NECK), [0.43, 0.34, -0.05], tol=0.002)
    S.flat("front", "neck", "deck", "body", 0.3, RG["frame"], ALU, pt.neck_front(NECK), [0.3, 0.34, -0.05], tol=0.002)
    S.put(S.solid("fork", "extrude", "fork", ALU, 0.6, profile=YOKE, plane="zx", **{"from": YOKE_V[0], "to": YOKE_V[1]}), RG["steer"])
    S.struct_of("fork", "fork")
    s = {"kind": "poly", "id": "fork", "bone": "fork", "role": "body", "z": 0, "filled": True, "tone": 0.4, "region": RG["steer"], "material": ALU, "points": [[u, w] for w, u in YOKE], "closed": True, "solid": {"kind": "extruded", "sweep": "v", "z0": YOKE_V[0], "z1": YOKE_V[1]}}
    S.shape(("elevation", "front", "plan"), s, [0.47, 0.38, -0.047])
    for i, c, regions, mid in (("wheel-f", AF, [RG["tyre"], RG["motor"]], MOT), ("wheel-r", AR, [RG["tyre"], RG["rim"], RG["rotor"]], ALU)):
        S.put(S.rev(i, i, WHEEL, c + (0.0,), [0, 0, 1], RUB, 0.07), RG["tyre"])
        S.disc(i, i, c, 0.216, -0.025, 0.025, "body", 0.1, None, RUB)
        many(S, i, regions)
        hub = S.cyl("hub-" + i[-1], i, c + (-0.0225,), c + (0.0225,), 0.077, mid, 0.5)
        S.therm(hub, RG["motor"] if i == "wheel-f" else RG["rim"], own=True)
        S.struct(hub, i)
        S.struct(S.cyl("axle-" + i[-1], i, c + (-0.055,), c + (0.055,), 0.015, STEEL, 0.3, role="detail"), i, "joint")
    S.therm(S.rev("disc-r", "wheel-r", [[0, pt.ROTOR[0]], [0.003, pt.ROTOR[0]], [0.003, pt.ROTOR[1]], [0, pt.ROTOR[1]], [0, pt.ROTOR[0]]], AR + (0.031,), [0, 0, 1], STEEL, 0.7, role="texture"), RG["rotor"], own=True)
    S.struct(S.cyl("mass-motor", "wheel-f", AF + (-0.02,), AF + (0.02,), 0.05, STEEL, 0.2, role="detail"), "wheel-f", "mass")
    up = [-kit.DOWN[0], -kit.DOWN[1], 0.0]
    S.put(S.rev("stem", "stem", STEM + [[0.737, 0.0]], tp(0.0, AV), up, ALU, 0.65), RG["steer"])
    S.struct_of("stem", "stem")
    outline = [tp(u, AV + r)[:2] for u, r in STEM] + [tp(u, AV - r)[:2] for u, r in STEM[::-1]]
    S.swept("stem", "stem", outline, -0.022, 0.022, "body", 0.4, RG["steer"], ALU, tol=0)
    B = tp(pt.BAR_U, AV)
    S.put(S.cyl("bars", "stem", B[:2] + [-pt.GRIP[1]], B[:2] + [pt.GRIP[1]], 0.017, RUB, 0.2), RG["steer"])
    S.struct_of("bars", "stem")
    S.disc("bars", "stem", B, 0.034, -pt.GRIP[1], pt.GRIP[1], "body", 0.2, RG["steer"], RUB)
    S.struct(S.cyl("hinge-pin", "fork", [P[0], P[1], -0.027], [P[0], P[1], 0.027], 0.015, STEEL, 0.3, role="detail"), "stem", "joint")
    S.struct(S.ball("mass-stem", "stem", [0.09, 0.016, 0.016], tp(0.36, AV), STEEL, 0.2, role="detail"), "stem", "mass")
    return S


DECISIONS = [
    {"en": "Body at 1:200, authored for 30 mm features: seven masses. The deck plate with its two rear fork arms reaching the axle (one plan outline with the wheel slot), the gooseneck with the steering head as one three-section body, the fork yoke with the steering column as one U-shaped plate, two wheel discs with a rounded tread, the stem as one tapered body and the handlebar with its grips as one bar. Mudguards, mat, motor, rim, rotor and controller are not in the body. The model is minimal: nothing more can be merged without losing the scooter.", "de": "Körper bei 1:200, für 30-mm-Einzelheiten entworfen: sieben Massen. Die Trittbrettplatte mit ihren zwei hinteren Gabelarmen bis zur Achse (ein Grundrissumriss mit dem Radschlitz), der Schwanenhals mit dem Steuerkopf als ein Körper aus drei Querschnitten, das Gabeljoch mit der Lenksäule als eine U-förmige Platte, zwei Radscheiben mit gerundeter Lauffläche, die Lenkstange als ein konischer Körper und der Lenker mit seinen Griffen als eine Stange. Schutzbleche, Matte, Motor, Felge, Bremsscheibe und Steuerung sind nicht im Körper. Das Modell ist minimal: mehr lässt sich nicht zusammenfassen, ohne den Roller zu verlieren."},
    {"en": "Thermal: deck, neck, fork yoke, stem, bars and both wheel discs by reference (the wheel discs are the tyres); real own solids for what the body merges away: the 1.5 mm grip mat layer, the hidden controller box, both mudguard arcs, the motor shell inside the front wheel, the rim inside the rear wheel and the brake rotor ring.", "de": "Thermik: Trittbrett, Hals, Gabeljoch, Lenkstange, Lenker und beide Radscheiben per Verweis (die Radscheiben sind die Reifen); wirkliche eigene Körper für das, was der Körper zusammenfasst: die 1,5-mm-Griffmattenschicht, der verdeckte Steuerungskasten, beide Schutzblechbögen, das Motorgehäuse im Vorderrad, die Felge im Hinterrad und der Bremsscheibenring."},
    {"en": "Structure: the hollow deck box girder with its two rear arms, the neck, the fork yoke, the stem and the bar; each wheel's rim cylinder is its member; joints at the hinge pin and the two axles; masses at the battery pack, the motor stator and the stem.", "de": "Tragwerk: der hohle Trittbrett-Kastenträger mit seinen zwei Heckarmen, der Hals, das Gabeljoch, die Lenkstange und der Lenker; der Felgenzylinder jedes Rads ist sein Stab; Gelenke am Gelenkbolzen und an den zwei Achsen; Massen an Akkupack, Motorstator und Lenkstange."},
]

DRAWN = {
    "silhouette": {"en": "All three views are the true orthographic views of the 1:200 body: two solid wheel discs, the deck bar with its rear arms beside the rear wheel, the gooseneck rising to the steering head, the fork yoke, the stem leaning back by 8 degrees and the handlebar; the folded stem lies along the deck onto the rear wheel.", "de": "Alle drei Ansichten sind die wahren orthogonalen Ansichten des 1:200-Körpers: zwei volle Radscheiben, der Trittbrettbalken mit seinen Heckarmen neben dem Hinterrad, der zum Steuerkopf ansteigende Schwanenhals, das Gabeljoch, die um 8 Grad zurückgeneigte Lenkstange und der Lenker; die geklappte Lenkstange liegt über dem Trittbrett auf dem Hinterrad."},
    "retained": {"en": "Wheels, deck, neck, fork, stem and handlebar.", "de": "Räder, Trittbrett, Hals, Gabel, Lenkstange und Lenker."},
    "merged": {"en": "Grip mat, battery enclosure, controller, stays and mudguards are part of the deck bar; tyre and hub motor, or tyre, rim and brake rotor, are one disc per wheel; head tube, hinge base, steerer, crown and blades are one yoke; grips and bar are one shape.", "de": "Griffmatte, Akkugehäuse, Steuerung, Streben und Schutzbleche gehören zum Trittbrettbalken; Reifen und Nabenmotor beziehungsweise Reifen, Felge und Bremsscheibe sind eine Scheibe je Rad; Steuerrohr, Gelenksockel, Gabelschaft, Krone und Scheiden sind ein Joch; Griffe und Stange sind eine Form."},
    "omitted": {"en": "Mudguard arcs, levers, bell, lights, side stand, rim and rotor detail.", "de": "Schutzblechbögen, Hebel, Klingel, Leuchten, Seitenständer, Felgen- und Scheibendetail."},
    "thinElements": {"en": "The 40 mm stem is 0.2 mm, the 34 mm handlebar 0.17 mm and the 55 mm deck 0.28 mm on paper, drawn as filled shapes with a 0.18 mm outline.", "de": "Die 40 mm dicke Lenkstange ist 0,2 mm, der 34 mm dicke Lenker 0,17 mm und das 55 mm dicke Trittbrett 0,28 mm auf dem Papier, als gefüllte Formen mit 0,18 mm Kontur gezeichnet."},
    "hatching": {"en": "None.", "de": "Keine."},
    "styles": {"en": "Dark wheel discs and handlebar, mid-tone deck, neck, fork and stem.", "de": "Dunkle Radscheiben und Lenker, mitteltoniges Trittbrett, Hals, Gabel und Lenkstange."},
    "overlays": {"en": "Wheel discs paint tyres with hub motor (front) or rim and brake rotor (rear) as area-weighted mean; the deck paints battery enclosure, grip mat and mudguards merged; the controller is hidden. Steering: the fork bone runs from the steering head down the steering axis (8 degrees rake) to the front axle and carries the front wheel and the stem, so the steered braking pose is a twist about that axis; the stem bone starts at the hinge pin behind the steering head and folds by 95 degrees.", "de": "Radscheiben zeigen Reifen mit Nabenmotor (vorn) beziehungsweise Felge und Bremsscheibe (hinten) als flächengewichtetes Mittel; das Trittbrett zeigt Akkugehäuse, Griffmatte und Schutzbleche vereint; die Steuerung ist verdeckt. Lenkung: Der Gabelknochen läuft vom Steuerkopf die Lenkachse (8 Grad Neigung) hinab zur Vorderachse und trägt Vorderrad und Lenkstange, die gelenkte Bremspose ist eine Drehung um diese Achse; der Lenkstangenknochen beginnt am Gelenkbolzen hinter dem Steuerkopf und klappt um 95 Grad."},
}
