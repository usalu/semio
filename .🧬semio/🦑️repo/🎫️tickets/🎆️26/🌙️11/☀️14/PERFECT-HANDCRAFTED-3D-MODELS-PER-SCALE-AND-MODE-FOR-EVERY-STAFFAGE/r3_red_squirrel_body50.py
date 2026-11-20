"""🐿 Round 3 red squirrel, 1:50 body: every solid stated by hand, station by station, in millimetres in the frame of its bone (fur surface of Sciurus vulgaris in the summer coat). Segments end in domes about their joints so that every pose stays closed."""
from r3_red_squirrel_geo import st, dome, loft, ellipsoid

RED, WHITE, TAIL, SKIN = "fur-red", "fur-white", "tail-fur", "bare-skin"
SIDES = (("near", "n", -1), ("far", "f", 1))
TONE = {"near": 0.35, "far": 0.5}


def rump():
    """Pelvis, loin and belly: the pear of the sitting animal, widest over the haunch; rear rounded behind the tail root, front a dome about the thoraco-lumbar joint."""
    s = dome(-1, -11, 21, 23, -0.4)
    s += [st(-1, 21, 23, -0.4), st(6, 23.6, 27), st(16, 25.4, 30), st(30, 26.2, 31.4), st(44, 26.2, 31.2), st(58, 25.8, 30), st(72, 25.2, 28.4)]
    s += dome(72, 26.5, 25.2, 28.4)
    return loft("rump", "spine-r", RED, 0.38, s, e=2.0)


def chest():
    """Ribcage and shoulders: barrel behind, narrowing to the thoracic inlet; rear dome about the thoraco-lumbar joint, front dome about the neck root."""
    s = dome(0, -26, 24.6, 27.8)
    s += [st(0, 25.2, 28.4), st(12, 25.4, 28.2), st(26, 24.8, 27, 0.3), st(40, 23.2, 24.8, 1.2), st(52, 21.2, 22.2, 2.6), st(62, 19.2, 19.6, 4.4)]
    s += dome(62, 18, 19.2, 19.6, 4.4, vc1=6)
    return loft("chest", "spine-f", RED, 0.4, s, e=2.0)


def neck():
    """Short thick neck between the two domes about the neck root and the atlas."""
    s = dome(0, -17, 18.4, 18.8)
    s += [st(0, 18.4, 18.8), st(8, 17.8, 18.2), st(16, 17.4, 17.8), st(24, 17.6, 17.6, 1.5)]
    s += dome(24, 16.5, 17.6, 17.6, 1.5, vc1=3)
    return loft("neck", "neck", RED, 0.38, s, e=2.0)


def head():
    """Skull under fur: round braincase, high convex forehead, cheeks widest at the zygomatic arches, a short deep blunt muzzle (the eye sits 27 mm behind the nose tip)."""
    s = dome(-3, -12, 17.2, 16.6, 3)
    s += [st(-3, 17.6, 17, 3), st(6, 20, 18.6, 4.4), st(16, 20.8, 19, 5), st(25, 19.8, 18.2, 4.6), st(33, 17.4, 16, 3.2), st(40, 15, 13.6, 1.8), st(46, 12.4, 11.2, 0.6), st(50, 10, 9.4, 0)]
    s += dome(50, 4.6, 10, 9.4, 0)
    return loft("head", "head", RED, 0.4, s, e=2.0)


def ear(name, sign):
    """A pinna with its short summer tuft: a pointed leaf 32 mm tall and 17 mm wide, 5.4 mm thick at the base with its fur, its hollow facing outwards and ahead, rooted 7 mm inside the skull fur at the back of the crown."""
    c, s_ = 0.866, 0.5
    X = (c, 0.0, -sign * s_)
    Y = (s_, 0.0, sign * c)
    A = (-0.16, 0.975, sign * 0.155)
    stations = [st(0, 6.4, 2.7), st(5, 8.1, 2.6), st(11, 8.5, 2.4), st(18, 7.2, 2.1), st(24, 5.2, 1.8), st(29, 2.9, 1.4), st(32, 0.9, 0.7)]
    return loft(f"{name}-ear", "head", RED, 0.3, stations, e=2.0, frame=((8.0, 15.0, sign * 12.4), A, X, Y))


def eyes():
    """The large dark eyes on the side of the head, bulging a millimetre out of the fur."""
    return [ellipsoid(f"{name}-eye", "head", SKIN, 0.04, (27.5, 9.8, sign * 16.2), (5, 4.8, 3.4)) for name, _, sign in SIDES]


def nose():
    """The bare nose pad at the tip of the muzzle."""
    return ellipsoid("nose", "head", SKIN, 0.12, (52.6, 0.8, 0), (2.6, 2.9, 3.7))


def tail1():
    """Proximal brush: thin root inside the rump fur, widening to the full flattened brush (54 mm wide, 35 mm deep); dome about the mid-tail joint."""
    s = [st(-7, 2.5, 2.5), st(-3, 7.5, 7), st(4, 11, 9.5), st(14, 16, 12), st(28, 21.2, 14.6, 2), st(46, 24.8, 16.4, 4.4), st(66, 26.6, 17.2, 4.6), st(85, 27, 17.5, 3)]
    s += dome(85, 17.5, 27, 17.5, 3)
    return loft("tail-1", "tail-1", TAIL, 0.3, s, e=2.0)


def tail2():
    """Distal brush: full width over the first half, tapering to the hair tip 19 mm beyond the last vertebra, the tip curling away from the back."""
    s = dome(0, -17.5, 26.6, 17.1, 3)
    s += [st(0, 27, 17.5, 3), st(24, 27, 17.4, 2.2), st(48, 25.6, 16.6, 2.6), st(70, 22.4, 15, 4.4), st(88, 17.6, 12.4, 6.6), st(99, 11.6, 9, 10), st(105.5, 6.4, 5.4, 13.4), st(109, 1.6, 1.6, 16)]
    return loft("tail-2", "tail-2", TAIL, 0.3, s, e=2.0)


def bibs():
    """The white ventral fur from the chin side of the throat to the groin: three shells, each a band 72 % as wide as the trunk that stands 2.2 mm proud of the red coat below and cuts it at a clear edge (their upper parts lie inside the trunk)."""
    c = dome(0, -14, 18.1, 15.6, -15)
    c += [st(0, 18.1, 15.6, -15), st(12, 18.3, 15.5, -14.9), st(26, 17.9, 14.9, -14), st(40, 16.7, 13.6, -12.2), st(52, 15.3, 12.2, -9.6), st(62, 13.8, 10.8, -6.6), st(70, 11.5, 8, -6.5), st(74, 6, 5, -5.5)]
    r = [st(2, 5, 5, -21.5), st(8, 12, 12, -17.6), st(16, 17.5, 16.5, -15.7), st(30, 18.9, 17.3, -16.3), st(44, 18.9, 17.2, -16.2), st(58, 18.6, 16.5, -15.7), st(72, 18.1, 15.6, -15)]
    r += dome(72, 14, 18.1, 15.6, -15)
    n = dome(0, -8, 12.6, 10, -10.8)
    n += [st(0, 12.6, 10, -10.8), st(8, 12.2, 9.8, -10.4), st(16, 11.6, 9.6, -10.2), st(24, 10.4, 9, -9.1), st(30, 6, 5.5, -9.5)]
    return [loft("belly-patch", "spine-f", WHITE, 0.92, c, e=2.0), loft("belly-patch-rump", "spine-r", WHITE, 0.92, r, e=2.0), loft("throat-patch", "neck", WHITE, 0.92, n, e=2.0)]


def arm(name, side, sign):
    """Upper arm: the shoulder and triceps mass about the humerus, tapering to the elbow; domes about shoulder and elbow."""
    s = dome(0, -9, 8, 11.5, -0.6)
    s += [st(0, 8, 11.5, -0.6), st(10, 7.8, 10.6, -1.4), st(22, 6.6, 8.4, -1), st(32, 5.6, 7, -0.4), st(40, 5.2, 6.4)]
    s += dome(40, 6, 5.2, 6.4)
    return loft(f"{name}-arm", f"arm-{side}", RED, TONE[name], s, e=2.0)


def forearm(name, side, sign):
    """Forearm: the flexor belly below the elbow, slim to the wrist."""
    s = dome(0, -6, 5, 6.2)
    s += [st(0, 5.2, 6.4), st(8, 5.4, 6.8, -0.5), st(20, 4.6, 5.4, -0.3), st(32, 3.9, 4.1), st(40, 3.8, 3.8)]
    s += dome(40, 3.6, 3.8, 3.8)
    return loft(f"{name}-forearm", f"forearm-{side}", RED, TONE[name], s, e=2.0)


def hand(name, side, sign):
    """Palm from the wrist to the knuckles: a flat paw 10 mm wide and 7.6 mm thick with its pads."""
    s = dome(0, -4, 3.6, 3.6)
    s += [st(0, 4, 3.8), st(5, 4.8, 3.6, -0.2), st(11, 5.2, 3.2, -0.4), st(14.5, 4.6, 2.6, -0.6), st(16.5, 2.4, 1.4, -0.7)]
    return loft(f"{name}-hand", f"hand-{side}", RED, TONE[name] + 0.1, s, e=2.4)


def fingers(name, side, sign):
    """The four long fingers as one tapering group, slightly curled towards the palm, bare skin."""
    s = [st(9, 3.4, 2.0, -0.4), st(11.5, 4.9, 2.9, -0.5), st(16, 4.7, 2.7, -0.8), st(21, 3.9, 2.2, -1.4), st(25, 2.8, 1.7, -2.1), st(27.2, 1.2, 0.9, -2.6)]
    return loft(f"{name}-fingers", f"hand-{side}", SKIN, TONE[name] + 0.25, s, e=2.5)


def thigh(name, side, sign):
    """Thigh: the broad flat muscle plate of gluteals, quadriceps and hamstrings (39 mm deep, 23 mm thick), narrowing to the knee; domes about hip and knee."""
    s = dome(-2, -16, 10.6, 18, -1.6, sign * 6)
    s += [st(-2, 10.6, 18, -1.6, sign * 6), st(7, 11.6, 19.6, -2, sign * 7), st(18, 11.5, 19, -1.8, sign * 6), st(30, 10, 15.4, -1, sign * 4), st(40, 8.6, 11.6, -0.4, sign * 1.6), st(50, 7.6, 9.2)]
    s += dome(50, 8.4, 7.6, 9.2)
    return loft(f"{name}-thigh", f"thigh-{side}", RED, TONE[name], s, e=2.0)


def shank(name, side, sign):
    """Lower leg: the calf behind the knee, the slim shin to the ankle."""
    s = dome(0, -8.4, 7.3, 8.9)
    s += [st(0, 7.6, 9.2), st(8, 7.4, 9.8, -1.2), st(20, 6.4, 8.4, -1.4), st(34, 5, 6, -0.6), st(46, 4.2, 4.6), st(58, 4, 4.3)]
    s += dome(58, 4.2, 4, 4.3)
    return loft(f"{name}-shank", f"shank-{side}", RED, TONE[name], s, e=2.0)


FOOT = [(-8.2, 1.4, 1.4, -6.0), (-6.6, 3.6, 3.2, -4.4), (-3, 4.6, 4.5, -2.8), (0, 5, 4.9, -2.2), (10, 5.4, 3.9, -2.4), (22, 6, 3.3, -2.0), (34, 6.6, 2.9, -1.5), (40, 6.6, 2.7, -1.2), (46, 5.4, 2.3, -1.1), (50, 3.6, 1.9, -1.2), (51.6, 1.4, 1, -2.0)]


def foot(name, side, sign):
    """The long plantigrade hind foot, thinly furred above: heel behind the ankle, the underside one straight line from heel to toes, widest at the ball, tapering toes; 59 mm from heel to toe tips, 13 mm wide."""
    return loft(f"{name}-foot", f"foot-{side}", RED, TONE[name] + 0.1, [st(*f) for f in FOOT], e=2.4)


def sole(name, side, sign):
    """The bare sole with its pads: a strip 2.6 mm thick under the foot from the heel to the toes, 1 mm proud of it."""
    s = [st(u, rw if rw < 2 else rw - 0.2, 1.3 if rw > 2 else 0.8, -6.8 + 0.08 * u + (0 if rw > 2 else 0.4)) for u, rw, rv, vc in FOOT[1:]]
    return loft(f"{name}-sole", f"foot-{side}", SKIN, TONE[name] + 0.3, s, e=2.4)


def body():
    out = [rump(), chest(), neck(), head(), tail1(), tail2()]
    out += bibs()
    for name, side, sign in SIDES:
        out += [arm(name, side, sign), forearm(name, side, sign), hand(name, side, sign), fingers(name, side, sign), thigh(name, side, sign), shank(name, side, sign), foot(name, side, sign), sole(name, side, sign)]
    out += [ear("near", -1), ear("far", 1)] + eyes() + [nose()]
    return out
