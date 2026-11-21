"""🐿 Round 3 red squirrel, 1:100 body: authored anew for a smallest feature of 15 mm. Masses are merged (fingers into the paw, throat patch into the chest bib, no eye, no nose pad), every mass has three to five own sections; proportions and joints are those of the animal, not of the 1:50 list."""
from r3_red_squirrel_geo import st, dome, loft

RED, WHITE, TAIL, SKIN = "fur-red", "fur-white", "tail-fur", "bare-skin"
SIDES = (("near", "n", -1), ("far", "f", 1))
TONE = {"near": 0.35, "far": 0.5}


def trunk():
    """Rump and chest as two plain masses with domes about their common joint, the neck as a short collar."""
    r = [st(-11, 6, 6.4, -0.4), st(-4, 18.6, 20.4, -0.3), st(8, 24.4, 28), st(36, 26.3, 31.4), st(72, 25.2, 28.4)] + dome(72, 26, 25.2, 28.4)
    c = dome(0, -26, 25.2, 28.4) + [st(0, 25.2, 28.4), st(30, 24.4, 26.4, 0.5), st(62, 19.2, 19.6, 4.4)] + dome(62, 18, 19.2, 19.6, 4.4, vc1=6)
    n = dome(0, -17, 18.4, 18.8) + [st(0, 18.4, 18.8), st(24, 17.6, 17.6, 1.5)] + dome(24, 16, 17.6, 17.6, 1.5, vc1=3)
    return [loft("rump", "spine-r", RED, 0.38, r), loft("chest", "spine-f", RED, 0.4, c), loft("neck", "neck", RED, 0.38, n)]


def head():
    """The head as one blunt wedge from the round back of the skull to the muzzle."""
    s = [st(-14, 6, 6, 3), st(-8, 14.6, 14, 3), st(4, 19.8, 18.4, 4.2), st(20, 20.6, 18.8, 4.9), st(34, 17, 15.6, 3), st(46, 12.4, 11.2, 0.6), st(52, 8.4, 7.9, 0), st(54.6, 2.4, 2.4, 0)]
    return loft("head", "head", RED, 0.4, s)


def ear(name, sign):
    """The pinna as a plain leaf of four sections."""
    X = (0.866, 0.0, -sign * 0.5)
    Y = (0.5, 0.0, sign * 0.866)
    A = (-0.16, 0.975, sign * 0.155)
    return loft(f"{name}-ear", "head", RED, 0.3, [st(0, 6.6, 2.7), st(10, 8.5, 2.5), st(23, 5.4, 1.9), st(32, 1, 0.8)], frame=((8.0, 15.0, sign * 12.4), A, X, Y))


def tail():
    """The brush in its two links: thin at the root, the full plume bowed towards the back, swinging over the crown and curling back at the tip; dome about the mid-tail joint."""
    a = [st(-7, 2.6, 2.6), st(8, 13, 10, -1), st(36, 23.5, 17, -4.5), st(64, 27, 19.5, -5), st(85, 27.5, 20, -3)] + dome(85, 20, 27.5, 20, -3)
    b = dome(0, -20, 27.5, 20, -3) + [st(0, 27.5, 20, -3), st(36, 27.5, 20.2, -4), st(66, 24, 17.8, 0.6), st(88, 16.5, 13, 7.5), st(102, 9, 7.8, 13.2), st(109.5, 1.8, 1.8, 17)]
    return [loft("tail-1", "tail-1", TAIL, 0.3, a), loft("tail-2", "tail-2", TAIL, 0.3, b)]


def bibs():
    """The white underside as two plain shells, 2.2 mm proud of the trunk below; the chest shell runs on under the throat."""
    c = dome(0, -14, 18.1, 15.6, -15) + [st(0, 18.1, 15.6, -15), st(30, 17.6, 14.6, -13.6), st(62, 13.8, 10.8, -6.6), st(78, 11, 9, -3.4), st(86, 5, 5, -1.5)]
    r = [st(3, 6, 6, -21), st(14, 16.6, 15.8, -16), st(40, 18.9, 17.2, -16.2), st(72, 18.1, 15.6, -15)] + dome(72, 14, 18.1, 15.6, -15)
    return [loft("belly-patch", "spine-f", WHITE, 0.92, c), loft("belly-patch-rump", "spine-r", WHITE, 0.92, r)]


def limbs(name, side, sign):
    """Seven masses per side: upper arm, forearm, the paw with its fingers in one piece, thigh, shank, foot and the bare sole."""
    arm = dome(0, -9, 8, 11.5, -0.6) + [st(0, 8, 11.5, -0.6), st(20, 6.8, 8.8, -1.1), st(40, 5.2, 6.4)] + dome(40, 6, 5.2, 6.4)
    fore = dome(0, -6, 5.2, 6.4) + [st(0, 5.2, 6.4), st(10, 5.4, 6.7, -0.5), st(40, 3.8, 3.8)] + dome(40, 3.6, 3.8, 3.8)
    paw = [st(-4, 1.6, 1.6), st(0, 4, 3.8), st(11, 5.2, 3.3, -0.4), st(19, 4.4, 2.5, -1.1), st(27.2, 1.2, 0.9, -2.6)]
    thigh = dome(-2, -16, 10.6, 18, -1.6, sign * 6) + [st(-2, 10.6, 18, -1.6, sign * 6), st(12, 11.6, 19.5, -2, sign * 6.8), st(34, 9.4, 14, -0.8, sign * 3), st(50, 7.6, 9.2)] + dome(50, 8.4, 7.6, 9.2)
    shank = dome(0, -8.4, 7.6, 9.2) + [st(0, 7.6, 9.2), st(12, 7.1, 9.5, -1.4), st(38, 4.7, 5.5, -0.4), st(58, 4, 4.3)] + dome(58, 4.2, 4, 4.3)
    foot = [st(-8.2, 1.6, 1.6, -6.2), st(-2, 4.8, 4.7, -2.6), st(16, 5.7, 3.6, -2.2), st(38, 6.6, 2.8, -1.3), st(48, 4.6, 2.1, -1.2), st(51.6, 1.4, 1, -2)]
    sole = [st(-6.6, 3.4, 1.3, -7.33), st(0, 4.8, 1.3, -6.8), st(20, 5.7, 1.3, -5.2), st(38, 6.4, 1.3, -3.76), st(48, 4.4, 1.3, -2.96), st(51.4, 1.4, 0.8, -2.4)]
    t = TONE[name]
    return [
        loft(f"{name}-arm", f"arm-{side}", RED, t, arm), loft(f"{name}-forearm", f"forearm-{side}", RED, t, fore), loft(f"{name}-hand", f"hand-{side}", SKIN, t + 0.2, paw, e=2.4),
        loft(f"{name}-thigh", f"thigh-{side}", RED, t, thigh), loft(f"{name}-shank", f"shank-{side}", RED, t, shank),
        loft(f"{name}-foot", f"foot-{side}", RED, t + 0.1, foot, e=2.4), loft(f"{name}-sole", f"foot-{side}", SKIN, t + 0.3, sole, e=2.4),
    ]


def body():
    out = trunk() + [head()] + tail() + bibs()
    for name, side, sign in SIDES:
        out += limbs(name, side, sign)
    return out + [ear("near", -1), ear("far", 1)]
