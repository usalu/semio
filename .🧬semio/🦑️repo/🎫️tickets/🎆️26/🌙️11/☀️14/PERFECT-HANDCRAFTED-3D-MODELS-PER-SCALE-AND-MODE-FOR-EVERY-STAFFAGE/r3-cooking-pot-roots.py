"""🍲 Round 3 cooking pot: the root fixes outside `volumes` (rig, poses, clips, anchors, provenance, the 2D drawings of 1:50, 1:100 and 1:200 as true views of each scale's body).

Usage: imported by r3-cooking-pot-build.py (`apply(asset)`).
"""
import importlib.util
import math
import os

T_DIR = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_cooking_pot_kit", os.path.join(T_DIR, "r3-cooking-pot-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)


def r5(x):
    return round(x, 5) + 0.0


def local(bone, pts):
    """🧭 World (x, y) of the side elevation in the frame of a bone."""
    if bone == "handles":
        return [[r5(x + K.REACH), r5(y - K.GRIP_Y)] for x, y in pts]
    return [[r5(y - K.ORIGIN[bone]), r5(-x)] for x, y in pts]


def shape(sid, bone, role, z, filled, tone, geometry, region=None, regions=None, material=None, pen=None):
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geometry)
    if region is not None:
        s["region"] = region
    if regions is not None:
        s["regions"] = regions
    if material is not None:
        s["material"] = material
    if pen is not None:
        s["penMm"] = pen
    return s


def poly(pts, closed=True):
    return {"kind": "poly", "points": [[r5(x), r5(y)] for x, y in pts], "closed": closed}


def ellipse(cx, cy, rx, ry):
    return {"kind": "ellipse", "cx": r5(cx), "cy": r5(cy), "rx": r5(rx), "ry": r5(ry)}


def rings(bone, *outlines):
    """✒️ Several closed outlines of the side elevation as one path, a ring each."""
    return {"kind": "path", "d": " ".join("M " + " L ".join("%s %s" % (x, y) for x, y in local(bone, o)) + " Z" for o in outlines)}


def mirror(pts):
    return [(-x, y) for x, y in reversed(pts)]


def both(half):
    """🪞 A half outline (from the axis side outwards and back) completed by its mirror image."""
    return half + mirror(half)


def flip_z(pts):
    return [(x, -z) for x, z in reversed(pts)]


Y1 = K.FLOOR
WALL_CUT = [(K.EDGE_R, Y1), (K.RO, K.KNEE_Y), (K.RO, K.LIP_Y), (K.RIM, K.H), (K.RIM_IN, K.H), (K.RI, K.LIP_Y), (K.RI, K.KNEE_Y), (K.FOOT_R, Y1)]
WALL_FAR = [(-K.RI, K.FILL), (K.RI, K.FILL), (K.RI, K.LIP_Y), (K.RIM_IN, K.H), (-K.RIM_IN, K.H), (-K.RI, K.LIP_Y)]
BASE_CUT = [(-K.CAP_R, 0.0), (K.CAP_R, 0.0), (K.EDGE_R, Y1), (-K.EDGE_R, Y1)]
WATER_50 = [(-K.FOOT_R, Y1), (K.FOOT_R, Y1), (K.RI, K.KNEE_Y), (K.RI, K.FILL), (-K.RI, K.FILL), (-K.RI, K.KNEE_Y)]
SWELL_LINE = [(-K.RI, K.FILL), (-0.06, K.FILL + 0.0015), (-0.03, K.FILL - 0.001), (0.0, K.FILL + 0.002), (0.03, K.FILL - 0.001), (0.06, K.FILL + 0.0015), (K.RI, K.FILL)]

ARM_SIDE = [(0.104, 0.0815), (0.112, 0.083), (0.122, 0.083), (0.12412, 0.08388), (0.125, 0.086), (0.12412, 0.08812), (0.122, 0.089), (0.112, 0.089), (0.104, 0.0875)]
PAD_X = K.RO + 0.0014
BOW_SIDE_50 = [(K.RO, 0.066), (PAD_X, 0.066), (PAD_X, 0.0772)] + ARM_SIDE + [(PAD_X, 0.0833), (PAD_X, 0.084), (K.RO, 0.084)]
BOW_SIDE_100 = [(K.RO, 0.0769)] + ARM_SIDE + [(K.RO, 0.0831)]

PAD_Z0, PAD_Z1 = 0.0142, 0.0264
BOW_FRONT_50 = both([(0.0, 0.083), (PAD_Z0, 0.083), (PAD_Z0, 0.066), (PAD_Z1, 0.066), (PAD_Z1, 0.084), (0.025, 0.084), (0.025, 0.089), (0.0, 0.089)][1:-1])


def bow_plan(steps):
    """🧲 Plan outline of the right loop handle from under the pouring rim (the rim disc lies over it): the outer edge out and round, the inner edge back."""
    outer = [(0.079, 0.0238), (0.104, 0.0245), (0.112, 0.025)]
    inner = [(0.0803, 0.0178), (0.104, 0.0185), (0.112, 0.019)]
    for k in range(1, steps + 1):
        a = math.pi / 2 * k / steps
        outer.append((0.112 + 0.013 * math.sin(a), 0.012 + 0.013 * math.cos(a)))
        inner.append((0.112 + 0.007 * math.sin(a), 0.012 + 0.007 * math.cos(a)))
    ring = outer + flip_z(outer)
    hole = inner + flip_z(inner)
    return ring + list(reversed(hole))


LUG_PLAN_200 = [(0.079, 0.0231), (0.113, 0.025), (0.125, 0.013), (0.125, -0.013), (0.113, -0.025), (0.079, -0.0231), (0.0815, -0.0095), (0.082, 0.0), (0.0815, 0.0095)]


def left(pts):
    return [(-x, z) for x, z in reversed(pts)]


def geometry_50():
    st, wa = "stainless", "water"
    elevation = [
        shape("wall-far", "pot", "detail", 3.1, True, 0.55, poly(local("pot", WALL_FAR)), region="wall", material=st),
        shape("water", "water", "body", 3.2, True, 0.45, poly(local("water", WATER_50)), region="water", material=wa),
        shape("surface", "surface", "detail", 3.3, False, 0.9, poly(local("surface", SWELL_LINE), False), region="water", material=wa),
        shape("wall-left", "pot", "body", 3.5, True, 0.4, poly(local("pot", mirror(WALL_CUT))), region="wall", material=st),
        shape("wall-right", "pot", "body", 3.5, True, 0.4, poly(local("pot", WALL_CUT)), region="wall", material=st),
        shape("base", "pot", "body", 3.5, True, 0.4, poly(local("pot", BASE_CUT)), region="base", material=st),
        shape("handle-left", "handles", "body", 3.6, True, 0.2, poly(local("handles", mirror(BOW_SIDE_50))), region="handle", material=st),
        shape("handle-right", "handles", "body", 3.6, True, 0.2, poly(local("handles", BOW_SIDE_50)), region="handle", material=st),
    ]
    for name, sign in (("rivet-left", -1), ("rivet-right", 1)):
        c = local("handles", [(sign * (K.RO + 0.0014), K.RIVET_Y)])[0]
        elevation.append(shape(name, "handles", "detail", 3.7, True, 0.5, ellipse(c[0], c[1], 0.00125, 0.004), region="handle", material=st, pen=0.18))
    wall_end = both([(K.EDGE_R, Y1), (K.RO, K.KNEE_Y), (K.RO, K.LIP_Y), (K.RIM, K.H)])
    rivet_z = (K.RO + K.PAD_T + 0.0002) * math.sin(K.PHI)
    front = [
        shape("wall-end", "pot", "body", 3.5, True, 0.4, poly(wall_end), region="wall", material=st),
        shape("base-end", "pot", "body", 3.5, True, 0.4, poly(BASE_CUT), region="base", material=st),
        shape("rim-line", "pot", "detail", 3.6, False, 0.3, poly([(-K.RO, K.LIP_Y), (K.RO, K.LIP_Y)], False), region="wall"),
        shape("handle-end", "handles", "body", 3.6, True, 0.2, poly(BOW_FRONT_50), region="handle", material=st),
        shape("handle-rivet", "handles", "detail", 3.7, True, 0.5, ellipse(rivet_z, K.RIVET_Y, 0.0039, 0.004), region="handle", material=st),
        shape("handle-rivet-near", "handles", "detail", 3.7, True, 0.5, ellipse(-rivet_z, K.RIVET_Y, 0.0039, 0.004), region="handle", material=st),
    ]
    plan = [
        shape("pot-top", "pot", "body", 3.5, True, 0.4, ellipse(0, 0, K.RIM, K.RIM), region="wall", material=st),
        shape("water-top", "water", "body", 3.6, True, 0.45, ellipse(0, 0, K.RI, K.RI), region="water", material=wa),
        shape("rim-inner", "pot", "detail", 3.65, False, 0.3, ellipse(0, 0, K.RI, K.RI), region="wall", material=st),
    ]
    for tag, x, z, r in K.BUBBLES:
        plan.append(shape("bubble-" + tag, "surface", "detail", 3.7, True, 0.9, ellipse(x, z, r, r), region="water", material=wa))
    plan += [
        shape("handle-left", "handles", "body", 3.4, True, 0.2, poly(left(bow_plan(3))), region="handle", material=st),
        shape("handle-right", "handles", "body", 3.4, True, 0.2, poly(bow_plan(3)), region="handle", material=st),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def geometry_100():
    st, wa = "stainless", "water"
    cut = [(K.RO, K.FLOOR), (K.RO, K.LIP_Y), (K.RIM, K.H), (K.RIM_IN, K.H), (K.RI, K.LIP_Y), (K.RI, K.FLOOR)]
    plate = [(-0.0768, 0.0), (0.0768, 0.0), (K.RO, K.FLOOR), (-K.RO, K.FLOOR)]
    arm = [(0.0177, 0.077), (0.0237, 0.077), (0.025, 0.083), (0.0177, 0.083)]
    elevation = [
        shape("wall-far", "pot", "body", 3.1, True, 0.55, poly(local("pot", WALL_FAR)), region="wall", material=st),
        shape("water", "water", "body", 3.2, True, 0.45, poly(local("water", [(-K.RI, K.FLOOR), (K.RI, K.FLOOR), (K.RI, K.FILL), (-K.RI, K.FILL)])), region="water", material=wa),
        shape("pot", "pot", "body", 3.5, True, 0.4, rings("pot", mirror(cut), cut), region="wall", material=st),
        shape("base", "pot", "body", 3.5, True, 0.4, poly(local("pot", plate)), region="base", material=st),
        shape("handle-left", "handles", "body", 3.6, True, 0.2, poly(local("handles", mirror(BOW_SIDE_100))), region="handle", material=st),
        shape("handle-right", "handles", "body", 3.6, True, 0.2, poly(local("handles", BOW_SIDE_100)), region="handle", material=st),
    ]
    front = [
        shape("pot-end", "pot", "body", 3.5, True, 0.4, poly(both([(K.RO, K.FLOOR), (K.RO, K.LIP_Y), (K.RIM, K.H)])), region="wall", material=st),
        shape("base-end", "pot", "body", 3.5, True, 0.4, poly(plate), region="base", material=st),
        shape("handle-end", "handles", "body", 3.6, True, 0.2, {"kind": "rect", "x": -0.025, "y": 0.083, "w": 0.05, "h": 0.006}, region="handle", material=st),
        shape("handle-arm-far", "handles", "body", 3.6, True, 0.2, poly(arm), region="handle", material=st),
        shape("handle-arm-near", "handles", "body", 3.6, True, 0.2, poly(mirror(arm)), region="handle", material=st),
    ]
    plan = [
        shape("pot-top", "pot", "body", 3.5, True, 0.4, ellipse(0, 0, K.RIM, K.RIM), region="wall", material=st),
        shape("water-top", "water", "body", 3.6, True, 0.45, ellipse(0, 0, K.RI, K.RI), region="water", material=wa),
        shape("handle-left", "handles", "body", 3.4, True, 0.2, poly(left(bow_plan(2))), region="handle", material=st),
        shape("handle-right", "handles", "body", 3.4, True, 0.2, poly(bow_plan(2)), region="handle", material=st),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def geometry_200():
    st, wa = "stainless", "water"
    cut = [(K.RO, K.FLOOR), (K.RO, K.LIP_Y), (K.RIM, K.LIP_Y), (K.RIM, K.H), (K.RI, K.H), (K.RI, K.FLOOR)]
    plate = [(-K.RO, 0.0), (K.RO, 0.0), (K.RO, K.FLOOR), (-K.RO, K.FLOOR)]
    bar = [(K.RO, 0.081), (K.REACH, 0.081), (K.REACH, 0.089), (K.RO, 0.089)]
    elevation = [
        shape("wall-far", "pot", "body", 3.1, True, 0.55, poly(local("pot", [(-K.RI, K.FILL), (K.RI, K.FILL), (K.RI, K.H), (-K.RI, K.H)])), region="wall", material=st),
        shape("water", "water", "body", 3.2, True, 0.45, poly(local("water", [(-K.RI, K.FLOOR), (K.RI, K.FLOOR), (K.RI, K.FILL), (-K.RI, K.FILL)])), region="water", material=wa),
        shape("pot", "pot", "body", 3.5, True, 0.4, rings("pot", mirror(cut), cut), region="wall", material=st),
        shape("base", "pot", "body", 3.5, True, 0.4, poly(local("pot", plate)), region="base", material=st, pen=0.18),
        shape("handles", "handles", "body", 3.6, True, 0.2, rings("handles", mirror(bar), bar), region="handle", material=st, pen=0.18),
    ]
    front = [
        shape("pot-end", "pot", "body", 3.5, True, 0.4, poly(both([(K.RO, K.FLOOR), (K.RO, K.LIP_Y), (K.RIM, K.LIP_Y), (K.RIM, K.H)])), region="wall", material=st),
        shape("base-end", "pot", "body", 3.5, True, 0.4, poly(plate), region="base", material=st, pen=0.18),
        shape("handle-end", "handles", "body", 3.6, True, 0.2, {"kind": "rect", "x": -0.025, "y": 0.081, "w": 0.05, "h": 0.008}, region="handle", material=st, pen=0.18),
    ]
    plan = [
        shape("pot-top", "pot", "body", 3.5, True, 0.4, ellipse(0, 0, K.RIM, K.RIM), region="wall", material=st),
        shape("water-top", "water", "body", 3.6, True, 0.45, ellipse(0, 0, K.RI, K.RI), region="water", material=wa),
        shape("handle-left", "handles", "body", 3.4, True, 0.2, poly(left(LUG_PLAN_200)), region="handle", material=st),
        shape("handle-right", "handles", "body", 3.4, True, 0.2, poly(LUG_PLAN_200), region="handle", material=st),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


DECISIONS = {
    "50": {
        "silhouette": K.text(
            "A 0.25 m wide stew pot cut open through its axis: the 0.8 mm wall with its base corner and the 45 degree pouring rim, the 5 mm sandwich base standing on 150 mm, water to 55 mm and, beyond the cut, the far arm of each loop handle rising from its riveted pad to the grip. Front: wall, base and the line where the rim flares, with the near handle end-on as an arch, the grip rod across two pads with their rivets; plan: the pouring rim, the water with nine boiling bubbles and the two open loops of 6 mm rod.",
            "Ein 0,25 m breiter Kochtopf, durch die Achse aufgeschnitten: die 0,8 mm starke Wand mit Bodenrundung und 45-Grad-Schüttrand, der 5-mm-Sandwichboden mit 150 mm Aufstandsfläche, Wasser bis 55 mm und hinter dem Schnitt der ferne Arm jedes Bügelgriffs, der von seiner genieteten Lasche zum Griffstab ansteigt. Vorderansicht: Wand, Boden und die Linie, an der der Rand ausschwingt, mit dem nahen Griff von vorn als Bogen, der Griffstab über zwei Laschen mit ihren Nieten; Draufsicht: der Schüttrand, das Wasser mit neun Kochblasen und die zwei offenen Bügel aus 6 mm Rundstahl.",
        ),
        "retained": K.text(
            "Wall, base, pouring rim, both handles with pads and rivets, the water body and its surface line. The bubbles are bound to the surface bone and mark the boiling pose in plan.",
            "Wand, Boden, Schüttrand, beide Griffe mit Laschen und Nieten, der Wasserkörper und seine Oberflächenlinie. Die Blasen sind an den Oberflächenknochen gebunden und kennzeichnen die kochende Pose in der Draufsicht.",
        ),
        "merged": K.text(
            "The capsule disc and the pot floor are one base shape; the pads lie under the pouring rim in plan and are part of the handle outline in the elevations.",
            "Kapselscheibe und Topfboden sind eine Bodenform; die Laschen liegen in der Draufsicht unter dem Schüttrand und gehören in den Ansichten zum Griffumriss.",
        ),
        "omitted": K.text("The lid, the set heads of the rivets inside the pot and the seam of the capsule.", "Der Deckel, die Schließköpfe der Nieten im Topfinneren und die Fuge der Kapsel."),
        "thinElements": K.text(
            "The wall is drawn at its true 0.8 mm (0.016 mm on paper): the cut reads through its outline pen, not through its fill. The handle rod is 6 mm (0.12 mm); the rivet heads in the section take the 0.18 mm pen so that they print.",
            "Die Wand ist in ihren wahren 0,8 mm gezeichnet (0,016 mm auf Papier): der Schnitt liest sich über den Umrissstift, nicht über die Füllung. Der Griffstab misst 6 mm (0,12 mm); die Nietköpfe im Schnitt erhalten den 0,18-mm-Stift, damit sie drucken.",
        ),
    },
    "100": {
        "silhouette": K.text(
            "A 2.5 mm wide cut pot: the two thin cut walls with the flared rim on the 5 mm base plate, the water block and the far arm of each loop handle. Front: the pot outline with chamfered base edge and flared rim and the near handle end-on as a low arch; plan: the rim disc with the water disc and the two open loops.",
            "Ein 2,5 mm breiter aufgeschnittener Topf: die zwei dünnen Schnittwände mit dem ausgestellten Rand auf der 5-mm-Bodenplatte, der Wasserblock und der ferne Arm jedes Bügelgriffs. Vorderansicht: der Topfumriss mit gefaster Bodenkante und ausgestelltem Rand und der nahe Griff von vorn als flacher Bogen; Draufsicht: die Randscheibe mit der Wasserscheibe und den zwei offenen Bügeln.",
        ),
        "retained": K.text("Walls, base plate, the pouring rim, the water block and the handle loops with their 20 x 32 mm openings.", "Wände, Bodenplatte, der Schüttrand, der Wasserblock und die Griffbügel mit ihren 20 x 32 mm großen Öffnungen."),
        "merged": K.text("Floor, capsule and base corner are one chamfered base plate, the two cut walls one shape; pads and rivets are merged into the handle rod.", "Topfboden, Kapsel und Bodenrundung sind eine gefaste Bodenplatte, die zwei Schnittwände eine Form; Laschen und Nieten gehen im Griffstab auf."),
        "overlays": K.text("Wall, base, handles and water are four regions, each with its own shapes.", "Wand, Boden, Griffe und Wasser sind vier Regionen, jede mit eigenen Formen."),
        "omitted": K.text("Surface line, bubbles, rivets and the rim line.", "Oberflächenlinie, Blasen, Nieten und die Randlinie."),
        "thinElements": K.text(
            "The wall is the true 0.8 mm and reads through the 0.18 mm pen; the handle rod is 6 mm (0.06 mm) and reads through its outline.",
            "Die Wand hat die wahren 0,8 mm und liest sich über den 0,18-mm-Stift; der Griffstab misst 6 mm (0,06 mm) und liest sich über seinen Umriss.",
        ),
    },
    "200": {
        "silhouette": K.text(
            "A 1.25 mm wide cut pot with a 5 mm rim band and a flat lug at each side just under the rim. Front: the pot with its rim band and the near lug end-on in front; plan: the rim disc with the water disc inside and the two lugs with clipped corners on the x axis.",
            "Ein 1,25 mm breiter aufgeschnittener Topf mit einem 5-mm-Randband und je einer flachen Lasche knapp unter dem Rand. Vorderansicht: der Topf mit Randband und der nahen Lasche von vorn davor; Draufsicht: die Randscheibe mit der Wasserscheibe darin und den zwei Laschen mit gekappten Ecken auf der x-Achse.",
        ),
        "retained": K.text("Walls, base plate, the rim band, the water block and the two handle lugs.", "Wände, Bodenplatte, das Randband, der Wasserblock und die zwei Grifflaschen."),
        "overlays": K.text("Wall, base, handles and water are four regions, each with its own shapes.", "Wand, Boden, Griffe und Wasser sind vier Regionen, jede mit eigenen Formen."),
        "merged": K.text(
            "Each loop handle is filled to one lug; in the section both lugs are one shape of two bars, one at each side of the pot.",
            "Jeder Bügelgriff ist zu einer Lasche gefüllt; im Schnitt sind beide Laschen eine Form aus zwei Balken, je einer an jeder Topfseite.",
        ),
        "omitted": K.text("Surface line, bubbles, the handle openings, pads, rivets and the base corner.", "Oberflächenlinie, Blasen, die Grifföffnungen, Laschen, Nieten und die Bodenrundung."),
        "thinElements": K.text(
            "The wall stays 0.8 mm and reads through the 0.13 mm pen; the pouring rim is generalised to a 5 mm band, the handle rod of 6 mm to an 8 mm lug; the 5 mm base plate and the lugs in the elevations take the 0.18 mm pen so that they print.",
            "Die Wand bleibt 0,8 mm und liest sich über den 0,13-mm-Stift; der Schüttrand ist zu einem 5-mm-Band verallgemeinert, der 6-mm-Griffstab zu einer 8-mm-Lasche; die 5-mm-Bodenplatte und die Laschen in den Ansichten erhalten den 0,18-mm-Stift, damit sie drucken.",
        ),
    },
}

NOTES = K.text(
    "A stainless stew pot of 0.16 m inner width and 0.10 m height: a deep-drawn 0.8 mm wall (0.1616 m outside) with an 8 mm base corner and a 45 degree pouring rim of 0.172 m, a 5 mm sandwich base (0.8 mm floor on a 4.2 mm encapsulated disc, 0.15 m on the hob) and two loop handles of 6 mm round rod riveted through forged pads, reaching to 0.25 m overall, filled with 1 L of water to 50 mm above the floor. It is cut open through its axis so that the water, the hot base and the wall read in the thermal view. Pot and handles are 0.64 kg, the water 1.0 kg; the origin is the centre of the base on the hob. Heating 1 kg of water by 80 K takes 335 kJ and evaporating it 2257 kJ (Cengel). The 1 kg of water needs 419 kJ from 0 to 100 degC and about 331 kJ for the 79 K from the room: the hob delivers 1.2 kW into the pot (gas-hob), so the open pot boils after about 5 minutes; at 100 degC the phase change of the water holds the temperature and turns the surplus heat into steam.",
    "Ein Edelstahl-Kochtopf von 0,16 m Innenweite und 0,10 m Höhe: eine tiefgezogene 0,8 mm starke Wand (außen 0,1616 m) mit 8 mm Bodenrundung und einem 45-Grad-Schüttrand von 0,172 m, ein 5 mm starker Sandwichboden (0,8 mm Topfboden auf einer 4,2 mm starken Kapselscheibe, 0,15 m auf dem Kochfeld) und zwei Bügelgriffe aus 6 mm Rundstahl, über geschmiedete Laschen angenietet, bis 0,25 m Gesamtbreite, mit 1 l Wasser bis 50 mm über dem Boden gefüllt. Er ist durch die Achse aufgeschnitten, damit Wasser, heißer Boden und Wand in der Thermalansicht lesbar sind. Topf und Griffe wiegen 0,64 kg, das Wasser 1,0 kg; der Ursprung ist die Mitte des Bodens auf dem Kochfeld. Die Erwärmung von 1 kg Wasser um 80 K braucht 335 kJ, das Verdampfen 2257 kJ (Cengel). Das 1 kg Wasser braucht von 0 auf 100 °C 419 kJ und für die 79 K ab Raumtemperatur rund 331 kJ: das Kochfeld liefert 1,2 kW in den Topf (gas-hob), sodass der offene Topf nach etwa 5 Minuten kocht; bei 100 °C hält der Phasenwechsel des Wassers die Temperatur und macht die überschüssige Wärme zu Dampf.",
)


def turn(offsets):
    """🧭 A pose offset of the surface bone, stated as a rise, in the frame of the upright water bone (rise = along u)."""
    if "surface" in offsets and "dy" in offsets["surface"]:
        offsets["surface"] = {"dx": offsets["surface"]["dy"]}


def apply(asset):
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    bones["pot"].update({"x": 0, "y": 0, "angleDeg": 90, "lengthM": K.H})
    bones["pot"]["title"] = K.text("Vessel axis from the base to the rim", "Gefäßachse vom Boden zum Rand")
    bones["pot"]["mass"].update({"comX": 0.024, "comY": 0})
    bones["handles"].update({"x": K.GRIP_Y, "y": K.REACH, "angleDeg": -90, "lengthM": 2 * K.REACH})
    bones["handles"]["title"] = K.text("Loop handles, from the left grip to the right grip", "Bügelgriffe, vom linken zum rechten Griffstab")
    bones["handles"]["mass"].update({"comX": K.REACH, "comY": -0.003})
    bones["water"].update({"x": K.FLOOR, "y": 0, "angleDeg": 0, "lengthM": 0})
    bones["water"]["title"] = K.text("Water fill, carried by the floor", "Wasserfüllung, vom Boden getragen")
    bones["water"]["mass"].update({"comX": 0.025, "comY": 0})
    bones["surface"].update({"x": K.FILL - K.FLOOR, "y": 0, "angleDeg": 0, "lengthM": 0})
    bones["surface"]["joint"] = {"type": "prismatic", "axisDeg": 0, "minM": -0.004, "maxM": 0.004}
    for anchor in asset["anchors"]:
        if anchor["id"] == "steam-top":
            anchor.update({"x": 0.015, "y": 0})
    for pose in asset["poses"]:
        turn(pose["bones"])
    for clip in asset["clips"]:
        for key in clip["keys"]:
            if "bones" in key:
                turn(key["bones"])
    asset["provenance"]["notes"] = NOTES
    reps = asset["representations"]
    for scale, geometry in (("50", geometry_50()), ("100", geometry_100()), ("200", geometry_200())):
        reps[scale]["geometry"] = geometry
        reps[scale]["decisions"].update(DECISIONS[scale])
    reps["500"]["reason"] = K.text(
        "At 1:500 the asset is only 0.5 mm wide on paper and its smallest part, the 0.8 mm pot wall, is far below every print profile.",
        "Bei 1:500 ist das Gerät nur 0.5 mm breit auf Papier und sein kleinstes Teil, die 0,8 mm starke Topfwand, liegt weit unter jedem Druckprofil.",
    )
    reps["500"]["limitingFeature"] = K.text("The 0.8 mm pot wall", "Die 0,8 mm starke Topfwand")
    reps["500"]["limitingSizeM"] = 0.0008
    return asset
