"""🛵 Round 3, 1:500 of vehicles/motor-scooter, authored for 75 mm features: eleven masses (two wheel discs, floor, leg shield, rear body, seat, one handlebar mass that takes in headset, grips and mirrors, the fork, the engine unit and the silencer); everything else is merged or lives in the thermal and structure lists."""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


pt = load("r3-motor-scooter-parts")
kit = pt.kit
AF, AR, H, PIVOT, HINGE = pt.AF, pt.AR, pt.H, pt.PIVOT, pt.HINGE
RG, TONE = pt.RG, pt.TONE
PAINT, RUBBER, ALLOY, VINYL, IRON, EXHAUST, ENGINE, GLASS = pt.PAINT, pt.RUBBER, pt.ALLOY, pt.VINYL, pt.IRON, pt.EXHAUST, pt.ENGINE, pt.GLASS
sp, ax = kit.steer_pt, kit.axis_x
PENS = {"body": 0.15, "detail": None, "texture": None, "glazing": 0.13, "ground": 0.25}

FLOOR = [(-0.165, 0.178, 0.302, 0.19, 4), (0.3, 0.178, 0.302, 0.205, 4), (0.385, 0.24, 0.318, 0.16, 3)]
SHIELD = [(0.288, 0.32, 0.395, 0.2, 2.6), (0.45, 0.365, 0.48, 0.232, 2.4), (0.62, 0.335, 0.52, 0.225, 2.2), (0.8, 0.275, 0.44, 0.17, 2.4), (0.965, 0.245, 0.36, 0.09, 2.6)]
TAIL = [(-0.035, 0.297, 0.375, 0.12, 2.4), (-0.1, 0.297, 0.64, 0.15, 2.7), (-0.3, 0.36, 0.725, 0.21, 3), (-0.62, 0.4, 0.722, 0.23, 3), (-0.9, 0.5, 0.7, 0.15, 2.6), (-0.948, 0.56, 0.675, 0.09, 2.4)]
SEAT = [(-0.135, 0.715, 0.795, 0.09, 2.6), (-0.45, 0.712, 0.8, 0.17, 3), (-0.7, 0.722, 0.83, 0.145, 3), (-0.815, 0.735, 0.812, 0.085, 2.4)]
BAR = [(-0.355, 0.28, 1.094, 0.04, 0.058), (-0.2, 0.288, 1.045, 0.06, 0.07), (0.0, 0.305, 1.03, 0.1, 0.078), (0.2, 0.288, 1.045, 0.06, 0.07), (0.355, 0.28, 1.094, 0.04, 0.058)]
ENGINE_ROWS = [(-0.2, 0.2, 0.34, 0.06, 3, 0.0), (-0.4, 0.165, 0.345, 0.12, 3, 0.03), (-0.445, 0.165, 0.34, 0.05, 3, 0.11), (-0.78, 0.17, 0.31, 0.05, 3, 0.11)]


def build():
    S = kit.Sheet(500, PENS)
    P, ST, D = RG["panels"], RG["steer"], TONE
    S.put(S.lofted("floor", "frame", FLOOR, PAINT, D["paint"]), P)
    S.put(S.lofted("shield", "frame", SHIELD, PAINT, D["paint"], along="y"), P)
    S.put(S.lofted("body", "frame", TAIL, PAINT, D["paint"]), P)
    for i in ("floor", "shield", "body"):
        S.proj(i, [i], "body", D["paint"], P, PAINT)
    S.put(S.tube("bar", "steer", [([x, y, z], rx, ry, 2.4) for z, x, y, rx, ry in BAR], PAINT, D["paint"] - 0.12, hint=(1.0, 0.0, 0.0)), ST)
    S.proj("bar", ["bar"], "body", D["paint"] - 0.12, None, PAINT, span=(-0.37, 0.37), cards=("front", "plan"), regions=[ST, RG["lamp"]])
    S.therm(S.ball("headlamp", "steer", [0.04, 0.05, 0.056], [0.37, 1.02, 0.0], GLASS, D["glass"], role="glazing", rotation=kit.rot_w(65)), RG["lamp"], own=True)
    fork = [(sp(0.06), 0.0375), (sp(0.56), 0.0375), ([0.63, 0.34, -0.07], 0.0375), ([0.68, 0.25, -0.085], 0.0375)]
    S.put(S.tube("fork", "steer", [(list(p), r, r) for p, r in fork], IRON, D["steel"]), ST)
    S.struct_of("fork", "steer")
    S.proj("fork", ["fork"], "body", D["steel"], ST, IRON, views=("elevation", "front"), span=(-0.1225, 0.0375))
    for name, c, h in (("f", AF, 0.055), ("r", AR, 0.06)):
        bone = "wheel-" + name
        S.put(S.rev("wheel-" + name, bone, [[-h, 0.0], [-h, 0.21], [-h + 0.02, 0.23], [h - 0.02, 0.23], [h, 0.21], [h, 0.0]], c + (0.0,), [0, 0, 1], RUBBER, D["tyre"] + 0.08), RG["tyre"])
        pt.ring2d(S, "wheel-" + name, bone, c, 0.23, None, -h, h, "body", D["tyre"] + 0.08, None, RUBBER, regions=[RG["tyre"], RG["rim"], RG["brake"]])
        rim = S.cyl("rim-" + name, bone, c + (-0.049,), c + (0.049,), 0.166, ALLOY, D["rim"])
        S.therm(rim, RG["rim"], own=True)
        S.struct(rim, bone)
        S.struct(S.cyl("axle-" + name, bone, c + (-0.075,), c + (0.075,), 0.0375, IRON, 0.3, role="detail"), bone, "joint")
        S.therm(S.cyl("brake-" + name, bone, c + (-0.0376,), c + (0.0376,), 0.09, IRON, D["iron"]), RG["brake"], own=True)
    S.put(pt.channel(S, "mudguard", "steer", AF, [[-0.1, 0.237], [0.1, 0.237], [0.1, 0.312], [-0.1, 0.312], [-0.1, 0.237]], 40.0, 160.0, PAINT, D["paint"]), P)
    S.swept("mudguard", "steer", pt.band(AF, 0.237, 0.312, 40.0, 160.0, 8), -0.1, 0.1, "body", D["paint"], P, PAINT, tol=0.004)
    S.put(S.lofted("engine", "swing", ENGINE_ROWS, ENGINE, D["engine"]), RG["engine"])
    S.struct_of("engine", "swing")
    (m0, m1, mz, mr) = pt.MUFFLER
    S.put(S.cap("muffler", "swing", [m0[0] - 0.05, m0[1], mz], [m1[0] + 0.05, m1[1], mz], mr, EXHAUST, D["exhaust"]), RG["exhaust"])
    S.thermal.append(dict(S.cyl("tailpipe-outlet", "swing", [m1[0] + 0.072, m1[1], mz], [m1[0] - 0.003, m1[1], mz], 0.0375, EXHAUST, 0.2), region=RG["exhaust"], tags=["tailpipe"]))
    S.proj("engine", ["engine"], "body", D["engine"], RG["engine"], ENGINE, views=("elevation",))
    S.proj("muffler", ["muffler"], "body", D["exhaust"], RG["exhaust"], EXHAUST, views=("elevation",))
    S.put(S.lofted("seat", "seat", SEAT, VINYL, D["seat"]), RG["seat"])
    S.proj("seat", ["seat"], "body", D["seat"], RG["seat"], VINYL, views=("elevation", "plan"))
    S.proj("seat-front", ["seat"], "body", D["seat"], RG["seat"], VINYL, views=("front",), span=(-0.168, 0.168))

    back = kit.bent([sp(0.09), sp(0.52), [0.4, 0.3, 0.0], [0.3, 0.24, 0.0], [-0.17, 0.24, 0.0], [-0.32, 0.56, 0.0], [-0.86, 0.63, 0.0]], [0.06, 0.05, 0.05, 0.08, 0.1], 3)
    S.struct(S.tube("backbone", "frame", [(p, 0.05, 0.0375, 4) for p in back], PAINT, 0.5), "frame")
    S.struct(S.box("seat-pan", "seat", [0.64, 0.075, 0.26], [-0.47, 0.75, 0.0], PAINT, D["steel"]), "seat")
    S.struct(S.box("stand-frame", "stand", [0.204, 0.075, 0.28], kit.world("stand", [0.102, 0.0, 0.0]), IRON, D["steel"]), "stand")
    S.struct(S.cyl("bearing", "steer", sp(0.05), sp(0.125), 0.045, IRON, 0.3, role="detail"), "steer", "joint")
    S.struct(S.cyl("swing-pivot", "swing", [PIVOT[0], PIVOT[1], -0.07], [PIVOT[0], PIVOT[1], 0.07], 0.0375, IRON, 0.3, role="detail"), "swing", "joint")
    c = kit.loc("steer", (AF[0], 0.0375, 0.0))
    S.struct(S.solid("contact-pad-f", "box", "steer", RUBBER, 0.1, size=[0.09, 0.075, 0.11], transform={"translation": kit.vec(c), "rotation": kit.rot_w(90 - kit.RAKE)}), "steer", "support", anchor="contact-f")
    S.struct(S.ball("mass-frame", "frame", [0.09, 0.06, 0.08], [-0.25, 0.5, 0.0], IRON, 0.2, role="detail"), "frame", "mass")
    S.struct(S.ball("mass-steer", "steer", [0.06, 0.038, 0.038], sp(0.42, 0.0), IRON, 0.2, role="detail"), "steer", "mass")
    S.struct(S.ball("mass-engine", "swing", [0.08, 0.05, 0.05], [-0.47, 0.258, 0.02], IRON, 0.2, role="detail"), "swing", "mass")
    return S


DECISIONS = [
    {"en": "Body at 1:500, authored for 75 mm features: eleven masses. Two wheel discs; the front mudguard as a 75 mm crescent 0.2 m wide over the front tyre; the floor slab; the leg shield as one lens-shaped plate; the rear body with cowls and lamp; the seat; one handlebar mass that is the envelope of headset, grips and mirrors (0.74 m wide, top at 1.12 m); the fork as one bar from the headset to beside the front axle; the engine unit as one body left of the rear wheel; the silencer capsule on the right. Stand, lamps and every detail are left out. The model is minimal: nothing more can be merged without losing the scooter.", "de": "Körper bei 1:500, für 75-mm-Einzelheiten entworfen: elf Massen. Zwei Radscheiben; das vordere Schutzblech als 75-mm-Sichel von 0,2 m Breite über dem Vorderreifen; die Bodenplatte; das Beinschild als eine linsenförmige Platte; die Heckkarosserie mit Backen und Leuchte; die Sitzbank; eine Lenkermasse als Hülle von Lenkkopf, Griffen und Spiegeln (0,74 m breit, Oberkante 1,12 m); die Gabel als ein Stab vom Lenkkopf bis neben die Vorderachse; der Triebsatz als ein Körper links des Hinterrads; die Schalldämpferkapsel rechts. Ständer, Leuchten und alle Einzelheiten entfallen. Das Modell ist minimal: mehr lässt sich nicht zusammenfassen, ohne den Roller zu verlieren."},
    {"en": "Thermal: the eleven masses by reference, and own solids for the headlamp lens, the rim cylinder and the brake cylinder inside each wheel disc; the exhaust outlet patch sits on the rear end of the silencer.", "de": "Thermik: die elf Massen per Verweis und eigene Körper für das Scheinwerferglas sowie den Felgen- und den Bremszylinder in jeder Radscheibe; der Auslassfleck des Abgases sitzt am hinteren Ende des Schalldämpfers."},
    {"en": "Structure: one backbone from the head tube through the leg shield, along the floor and up to the tail; the fork bar; the engine body as the swinging arm; a rim cylinder per wheel; the seat pan and the stand as plates. Joints: head bearing, swing pivot, two axles. Contact pad under the front tyre; three masses.", "de": "Tragwerk: ein Rückgrat vom Steuerrohr durch das Beinschild, den Boden entlang und hinauf zum Heck; der Gabelstab; der Motorkörper als Triebsatzschwinge; ein Felgenzylinder je Rad; Sitzwanne und Ständer als Platten. Gelenke: Steuerlager, Triebsatzlager, zwei Achsen. Kontaktpolster unter dem Vorderreifen; drei Massen."},
]
