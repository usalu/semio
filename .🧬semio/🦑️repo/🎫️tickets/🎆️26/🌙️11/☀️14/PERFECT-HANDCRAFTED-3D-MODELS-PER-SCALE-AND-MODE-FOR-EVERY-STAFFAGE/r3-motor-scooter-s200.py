"""🛵 Round 3, 1:200 of vehicles/motor-scooter, authored for 30 mm features: sixteen masses that still read as a step-through scooter at a glance (floor, one lens-shaped leg shield, rear body with the cowls merged in, seat, a handlebar spindle with the grips, two mirrors on their stalks, column, fork leg, mudguard crescent, two wheel discs, the engine unit with its belt case, the silencer); lamps, rims and brake parts live in the thermal list, the backbone, rails, axles and masses in the structure list."""
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
PENS = {"body": 0.18, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.25}

FLOOR = [(-0.165, 0.182, 0.3, 0.19, 5), (0.28, 0.178, 0.302, 0.208, 5), (0.385, 0.25, 0.318, 0.16, 3)]
SHIELD = [(0.288, 0.335, 0.392, 0.2, 2.6), (0.42, 0.375, 0.455, 0.232, 2.4), (0.56, 0.355, 0.535, 0.235, 2.2), (0.72, 0.3, 0.48, 0.21, 2.2), (0.88, 0.255, 0.4, 0.12, 2.4), (0.965, 0.25, 0.355, 0.065, 2.6)]
TAIL = [(-0.03, 0.297, 0.34, 0.115, 2.4), (-0.075, 0.297, 0.56, 0.13, 2.6), (-0.12, 0.3, 0.69, 0.15, 2.8), (-0.2, 0.35, 0.722, 0.19, 3), (-0.42, 0.38, 0.725, 0.228, 3), (-0.62, 0.4, 0.722, 0.23, 3), (-0.82, 0.47, 0.708, 0.19, 2.8), (-0.948, 0.565, 0.672, 0.09, 2.4)]
SEAT = [(-0.135, 0.722, 0.778, 0.085, 2.6), (-0.3, 0.712, 0.8, 0.158, 3), (-0.48, 0.715, 0.798, 0.17, 3), (-0.68, 0.726, 0.83, 0.146, 3), (-0.815, 0.74, 0.802, 0.08, 2.4)]
SPINDLE = [(-0.34, 0.288, 1.008, 0.018, 0.018), (-0.2, 0.288, 1.008, 0.018, 0.018), (-0.165, 0.29, 1.008, 0.03, 0.027), (-0.09, 0.3, 1.011, 0.08, 0.052), (0.0, 0.305, 1.012, 0.1, 0.062), (0.09, 0.3, 1.011, 0.08, 0.052), (0.165, 0.29, 1.008, 0.03, 0.027), (0.2, 0.288, 1.008, 0.018, 0.018), (0.34, 0.288, 1.008, 0.018, 0.018)]
ENGINE_ROWS = [(-0.19, 0.25, 0.35, 0.05, 3, 0.0), (-0.3, 0.18, 0.35, 0.1, 3, 0.02), (-0.425, 0.172, 0.345, 0.125, 3, 0.03), (-0.445, 0.17, 0.345, 0.05, 3, 0.11), (-0.62, 0.158, 0.325, 0.05, 3, 0.11), (-0.787, 0.2, 0.265, 0.03, 3, 0.11)]
GUARD = [[-0.095, 0.237], [-0.095, 0.25], [-0.075, 0.267], [0.075, 0.267], [0.095, 0.25], [0.095, 0.237], [-0.095, 0.237]]


def build():
    S = kit.Sheet(200, PENS)
    P, ST, D = RG["panels"], RG["steer"], TONE
    S.put(S.lofted("floor", "frame", FLOOR, PAINT, D["paint"]), P)
    S.put(S.lofted("shield", "frame", SHIELD, PAINT, D["paint"], along="y"), P)
    S.put(S.lofted("body", "frame", TAIL, PAINT, D["paint"]), P)
    S.therm(S.box("tail-lamp", "frame", [0.03, 0.07, 0.13], [-0.94, 0.622, 0.0], GLASS, 0.3, role="glazing"), RG["lamp"], own=True)
    for i in ("floor", "shield", "body"):
        S.proj(i, [i], "body", D["paint"], P, PAINT)
    S.put(S.tube("headset", "steer", [([x, y, z], rx, ry, 2.4) for z, x, y, rx, ry in SPINDLE], PAINT, D["paint"] + 0.03, hint=(1.0, 0.0, 0.0)), ST)
    S.proj("headset", ["headset"], "body", D["paint"] + 0.03, None, PAINT, span=(-0.2, 0.2), cards=("front", "plan"), regions=[ST, RG["lamp"]])
    S.therm(S.ball("headlamp", "steer", [0.03, 0.05, 0.056], [0.379, 1.012, 0.0], GLASS, D["glass"], role="glazing", rotation=kit.rot_w(65)), RG["lamp"], own=True)
    for side, sign in (("l", 1), ("r", -1)):
        stalk = [[c[0], c[1], sign * c[2]] for c in pt.STALK]
        S.put(S.tube("stalk-" + side, "steer", [(p, 0.004, 0.004) for p in stalk], ALLOY, D["trim"], role="detail", slender=True), ST)
        S.put(S.ball("mirror-" + side, "steer", [0.015, 0.042, 0.056], [pt.MIRROR[0], pt.MIRROR[1], sign * pt.MIRROR[2]], ALLOY, D["trim"], role="detail", rotation=kit.rot_w(65)), ST)
        S.proj("mirror-" + side, ["stalk-" + side, "mirror-" + side], "detail", D["trim"], ST, ALLOY, span=tuple(sorted((sign * 0.135, sign * 0.37))), cards=("front", "plan"))
    S.put(S.cyl("column", "steer", sp(0.02), sp(0.6), 0.02, IRON, D["steel"]), ST)
    S.struct_of("column", "steer")
    leg = [((0.5306, 0.4698, 0.0), 0.02), ((0.59, 0.39, -0.07), 0.019), ((0.665, 0.315, pt.FORK_Z), 0.019), ((0.69, 0.24, pt.FORK_Z), 0.019)]
    S.put(S.tube("fork-leg", "steer", [(list(p), r, r) for p, r in leg], IRON, D["steel"]), ST)
    S.struct_of("fork-leg", "steer")
    S.put(pt.channel(S, "mudguard", "steer", AF, GUARD, 24.0, 160.0, PAINT, D["paint"]), P)
    S.proj("column", ["column"], "body", D["steel"], ST, IRON, views=("elevation", "front"), span=(-0.02, 0.02))
    S.proj("fork-leg", ["fork-leg"], "body", D["steel"], ST, IRON, views=("elevation", "front"), span=(pt.FORK_Z - 0.019, pt.FORK_Z + 0.019))
    S.swept("mudguard", "steer", pt.band(AF, 0.237, 0.267, 24.0, 160.0, 10), -0.095, 0.095, "body", D["paint"], P, PAINT, tol=0.002)
    for name, c, h in (("f", AF, 0.055), ("r", AR, 0.06)):
        bone = "wheel-" + name
        S.put(S.rev("wheel-" + name, bone, [[-h, 0.0], [-h, 0.2], [-h + 0.02, 0.23], [h - 0.02, 0.23], [h, 0.2], [h, 0.0]], c + (0.0,), [0, 0, 1], RUBBER, D["tyre"] + 0.05), RG["tyre"])
        pt.ring2d(S, "wheel-" + name, bone, c, 0.23, None, -h, h, "body", D["tyre"] + 0.05, None, RUBBER, regions=[RG["tyre"], RG["rim"], RG["brake"]])
        rim = S.cyl("rim-" + name, bone, c + (-0.049,), c + (0.049,), 0.166, ALLOY, D["rim"])
        S.therm(rim, RG["rim"], own=True)
        S.struct(rim, bone)
        S.struct(S.cyl("axle-" + name, bone, c + (-0.07,), c + (0.07,), 0.015, IRON, 0.3, role="detail"), bone, "joint")
    S.therm(S.cyl("disc-f", "wheel-f", AF + (-0.063,), AF + (-0.033,), 0.1, IRON, D["iron"]), RG["brake"], own=True)
    S.therm(S.cyl("drum-r", "wheel-r", AR + (0.022,), AR + (0.054,), 0.083, IRON, D["iron"]), RG["brake"], own=True)
    S.put(S.lofted("engine", "swing", ENGINE_ROWS, ENGINE, D["engine"]), RG["engine"])
    S.struct_of("engine", "swing")
    (m0, m1, mz, mr) = pt.MUFFLER
    S.put(S.cap("muffler", "swing", [m0[0] - 0.05, m0[1], mz], [m1[0] + 0.05, m1[1], mz], mr, EXHAUST, D["exhaust"]), RG["exhaust"])
    S.thermal.append(dict(S.cyl("tailpipe-outlet", "swing", [m1[0] + 0.027, m1[1], mz], [m1[0] - 0.003, m1[1], mz], 0.0175, EXHAUST, 0.2), region=RG["exhaust"], tags=["tailpipe"]))
    S.proj("engine", ["engine"], "body", D["engine"], RG["engine"], ENGINE, views=("elevation",))
    S.proj("muffler", ["muffler"], "body", D["exhaust"], RG["exhaust"], EXHAUST, views=("elevation",))
    S.put(S.lofted("seat", "seat", SEAT, VINYL, D["seat"]), RG["seat"])
    S.proj("seat", ["seat"], "body", D["seat"], RG["seat"], VINYL, views=("elevation", "plan"))
    S.proj("seat-front", ["seat"], "body", D["seat"], RG["seat"], VINYL, views=("front",), span=(-0.168, 0.168))

    back = kit.bent([sp(0.09), sp(0.52), [0.4, 0.3, 0.0], [0.3, 0.235, 0.0], [-0.17, 0.235, 0.0]], [0.05, 0.05, 0.04], 3)
    S.struct(S.tube("backbone", "frame", [(p, 0.05, 0.035, 4) for p in back], PAINT, 0.5), "frame")
    for side, sign in (("l", 1), ("r", -1)):
        rail = [([-0.15, 0.24, sign * 0.07], 0.03, 0.02, 4), ([-0.3, 0.6, sign * 0.165], 0.03, 0.02, 4), ([-0.86, 0.64, sign * 0.13], 0.03, 0.02, 4)]
        S.struct(S.tube("rail-" + side, "frame", rail, PAINT, 0.5, hint=(0.0, 1.0, 0.0)), "frame")
    S.struct(S.box("cross-rear", "frame", [0.03, 0.05, 0.27], [-0.86, 0.64, 0.0], PAINT, 0.5), "frame")
    S.struct(S.cap("shock-r", "frame", [-0.66, 0.292, 0.118], [-0.615, 0.622, 0.142], 0.017, IRON, D["steel"], role="detail"), "frame")
    S.struct(S.cyl("handlebar", "steer", [pt.BAR[0], pt.BAR[1], -0.3], [pt.BAR[0], pt.BAR[1], 0.3], 0.015, IRON, D["steel"]), "steer")
    S.struct(S.box("seat-pan", "seat", [0.64, 0.03, 0.26], [-0.47, 0.728, 0.0], PAINT, D["steel"]), "seat")
    S.struct(S.box("stand-frame", "stand", [0.204, 0.03, 0.28], kit.world("stand", [0.102, 0.0, 0.0]), IRON, D["steel"]), "stand")
    S.struct(S.cyl("bearing", "steer", sp(0.07), sp(0.1), 0.031, IRON, 0.3, role="detail"), "steer", "joint")
    S.struct(S.cyl("swing-pivot", "swing", [PIVOT[0], PIVOT[1], -0.066], [PIVOT[0], PIVOT[1], 0.066], 0.015, IRON, 0.3, role="detail"), "swing", "joint")
    S.struct(S.cyl("seat-hinge", "seat", [HINGE[0], HINGE[1], -0.06], [HINGE[0], HINGE[1], 0.06], 0.015, IRON, 0.3, role="detail"), "seat", "joint")
    SP = kit.STAND_PIVOT
    S.struct(S.cyl("stand-pin", "stand", [SP[0], SP[1], -0.08], [SP[0], SP[1], 0.08], 0.015, IRON, 0.3, role="detail"), "stand", "joint")
    c = kit.loc("steer", (AF[0], 0.015, 0.0))
    S.struct(S.solid("contact-pad-f", "box", "steer", RUBBER, 0.1, size=[0.09, 0.03, 0.11], transform={"translation": kit.vec(c), "rotation": kit.rot_w(90 - kit.RAKE)}), "steer", "support", anchor="contact-f")
    S.struct(S.ball("mass-frame", "frame", [0.09, 0.06, 0.08], [-0.25, 0.5, 0.0], IRON, 0.2, role="detail"), "frame", "mass")
    S.struct(S.ball("mass-steer", "steer", [0.05, 0.016, 0.016], sp(0.42, 0.0), IRON, 0.2, role="detail"), "steer", "mass")
    S.struct(S.ball("mass-engine", "swing", [0.08, 0.05, 0.05], [-0.47, 0.258, 0.02], IRON, 0.2, role="detail"), "swing", "mass")
    return S


DECISIONS = [
    {"en": "Body at 1:200, authored for 30 mm features: sixteen masses. The leg shield with nose, wings and glovebox is one lens-shaped plate; the rear body takes the side cowls in; headset, switch housings and grips are one handlebar spindle; the mirrors stay as discs on their true 8 mm stalks (slender); column and fork leg (run down to the axle, link merged) are two bars; the mudguard is a 30 mm crescent; each wheel is one disc with a rounded tread; crankcase, cylinder, belt case and air box are one engine body that narrows to the left of the rear wheel; the silencer is one capsule. Lamps, plate holder, grab rail, shock, tail pipe and the stand are not in the body.", "de": "Körper bei 1:200, für 30-mm-Einzelheiten entworfen: sechzehn Massen. Das Beinschild mit Nase, Flügeln und Handschuhfach ist eine linsenförmige Platte; die Heckkarosserie nimmt die Seitenbacken auf; Lenkkopf, Schaltergehäuse und Griffe sind eine Lenkerspindel; die Spiegel bleiben Scheiben auf ihren echten 8-mm-Armen (schlank); Lenksäule und Gabelrohr (bis zur Achse geführt, Schwinge eingeschmolzen) sind zwei Stäbe; das Schutzblech ist eine 30-mm-Sichel; jedes Rad ist eine Scheibe mit gerundeter Lauffläche; Kurbelgehäuse, Zylinder, Riemengehäuse und Luftfilterkasten sind ein Motorkörper, der sich links neben das Hinterrad verjüngt; der Schalldämpfer ist eine Kapsel. Leuchten, Kennzeichenträger, Haltebügel, Federbein, Endrohr und Ständer sind nicht im Körper."},
    {"en": "Thermal: the merged bodies by reference, and real own solids for what the body merges away: headlamp lens and tail lamp, the rim cylinder inside each wheel disc, the front brake disc and the rear drum as 30 mm cylinders; the exhaust outlet patch sits on the rear end of the silencer.", "de": "Thermik: die zusammengefassten Körper per Verweis und wirkliche eigene Körper für das, was der Körper zusammenfasst: Scheinwerferglas und Rücklicht, der Felgenzylinder in jeder Radscheibe, die vordere Bremsscheibe und die hintere Trommel als 30-mm-Zylinder; der Auslassfleck des Abgases sitzt am hinteren Ende des Schalldämpfers."},
    {"en": "Structure: one backbone from the head tube down the leg shield and along the floor, two shell rails to the tail with a rear cross member, the rear shock; handlebar tube, column and fork leg; the engine body is the swinging arm; a rim cylinder per wheel; seat pan; the stand as one U-frame plate. Joints: head bearing, swing pivot, seat hinge, stand pin, two axles. Contact pad under the front tyre; three masses.", "de": "Tragwerk: ein Rückgrat vom Steuerrohr durch das Beinschild und den Boden entlang, zwei Schalenholme zum Heck mit einem Heckquerträger, das hintere Federbein; Lenkerrohr, Lenksäule und Gabelrohr; der Motorkörper ist die Triebsatzschwinge; ein Felgenzylinder je Rad; Sitzwanne; der Ständer als eine U-Rahmen-Platte. Gelenke: Steuerlager, Triebsatzlager, Sitzscharnier, Ständerbolzen, zwei Achsen. Kontaktpolster unter dem Vorderreifen; drei Massen."},
]
