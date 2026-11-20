"""🌰 Round 4 red squirrel, 1:200 and 1:500: body, thermal and structure of the two digital scales, authored anew for a smallest feature of 30 mm and 75 mm (millimetres in the bone frames). The drawing is unavailable at both scales (the animal is 1.05 mm and 0.42 mm high on paper); the model keeps the sitting squirrel recognisable by its masses: haunch, upright trunk, wedge head with ears, the brush behind the back, hands at the mouth, long hind feet.

Structure: the skeleton as slender bars with a skull and a pelvis, the column dorsal in the trunk, rib hoops under it.
"""
from r3_red_squirrel_geo import st, dome, loft, ellipsoid
from r3_red_squirrel_lists import ref, member, rib, BONE, BONE_TONE, MASS_TONE

RED, WHITE, TAIL, SKIN = "fur-red", "fur-white", "tail-fur", "bare-skin"
SIDES = (("near", "n", -1), ("far", "f", 1))
TONE = {"near": 0.35, "far": 0.5}
EAR_X = lambda sign: (0.866, 0.0, -sign * 0.5)
EAR_Y = lambda sign: (0.5, 0.0, sign * 0.866)
EAR_A = lambda sign: (-0.16, 0.975, sign * 0.155)


def body200():
    """22 masses: rump, chest, neck collar, head, two ears, the brush in two links, two white shells, and per side upper arm, forearm, paw, thigh, shank and the whole foot."""
    out = [
        loft("rump", "spine-r", RED, 0.38, [st(-10, 7, 7, -0.4), st(2, 23, 26), st(36, 26.3, 31.4), st(72, 25.2, 28.4)] + dome(72, 26, 25.2, 28.4)),
        loft("chest", "spine-f", RED, 0.4, dome(0, -26, 25.2, 28.4) + [st(0, 25.2, 28.4), st(34, 24, 26, 0.6), st(62, 19.2, 19.6, 4.4)] + dome(62, 18, 19.2, 19.6, 4.4, vc1=6)),
        loft("neck", "neck", RED, 0.38, dome(0, -17, 18.4, 18.8) + [st(0, 18.4, 18.8), st(24, 17.6, 17.6, 1.5)] + dome(24, 16, 17.6, 17.6, 1.5, vc1=3)),
        loft("head", "head", RED, 0.4, [st(-13, 7, 7, 3), st(-4, 17, 16, 3.4), st(14, 20.6, 18.8, 4.8), st(34, 17, 15.4, 3), st(50, 9.6, 8.8, 0.2), st(54.6, 2.6, 2.6, 0)]),
        loft("tail-1", "tail-1", TAIL, 0.3, [st(-7, 2.6, 2.6), st(10, 14, 11), st(44, 24.5, 16.2, 4), st(85, 27, 17.5, 3)] + dome(85, 17, 27, 17.5, 3)),
        loft("tail-2", "tail-2", TAIL, 0.3, dome(0, -17, 27, 17.5, 3) + [st(0, 27, 17.5, 3), st(50, 25.4, 16.4, 2.6), st(92, 16, 11.4, 7.6), st(109, 1.8, 1.8, 16)]),
        loft("belly-patch", "spine-f", WHITE, 0.92, dome(0, -14, 18.1, 15.6, -15) + [st(0, 18.1, 15.6, -15), st(34, 17.4, 14.2, -13.2), st(66, 12.8, 10, -6), st(84, 5, 5, -2)]),
        loft("belly-patch-rump", "spine-r", WHITE, 0.92, [st(3, 6, 6, -21), st(18, 17.4, 16.4, -16), st(72, 18.1, 15.6, -15)] + dome(72, 14, 18.1, 15.6, -15)),
    ]
    for name, side, sign in SIDES:
        t = TONE[name]
        out += [
            loft(f"{name}-arm", f"arm-{side}", RED, t, dome(0, -9, 8, 11.5, -0.6) + [st(0, 8, 11.5, -0.6), st(40, 5.2, 6.4)] + dome(40, 6, 5.2, 6.4)),
            loft(f"{name}-forearm", f"forearm-{side}", RED, t, dome(0, -6, 5.2, 6.4) + [st(0, 5.2, 6.4), st(40, 3.9, 3.9)] + dome(40, 3.6, 3.9, 3.9)),
            loft(f"{name}-hand", f"hand-{side}", SKIN, t + 0.2, [st(-4, 1.6, 1.6), st(2, 4.4, 3.7), st(14, 5, 3, -0.6), st(27.2, 1.2, 0.9, -2.6)], e=2.4),
            loft(f"{name}-thigh", f"thigh-{side}", RED, t, dome(-2, -16, 10.6, 18, -1.6, sign * 6) + [st(-2, 10.6, 18, -1.6, sign * 6), st(14, 11.6, 19.4, -2, sign * 6.6), st(50, 7.6, 9.2)] + dome(50, 8.4, 7.6, 9.2)),
            loft(f"{name}-shank", f"shank-{side}", RED, t, dome(0, -8.4, 7.6, 9.2) + [st(0, 7.6, 9.2), st(14, 7, 9.2, -1.3), st(58, 4, 4.3)] + dome(58, 4.2, 4, 4.3)),
            loft(f"{name}-sole", f"foot-{side}", RED, t + 0.1, [st(-8.2, 1.6, 1.8, -6), st(-1, 4.9, 5, -3), st(22, 6, 3.9, -3), st(42, 6.4, 3.2, -2.4), st(51.6, 1.4, 1, -2.2)], e=2.4),
        ]
    for name, side, sign in SIDES:
        out.append(loft(f"{name}-ear", "head", RED, 0.3, [st(0, 6.6, 2.8), st(12, 8.4, 2.6), st(32, 1, 0.8)], frame=((8.0, 15.0, sign * 12.4), EAR_A(sign), EAR_X(sign), EAR_Y(sign))))
    return out


def body500():
    """19 masses: rump, chest with the neck in its front dome, head, two ears, the brush in two links, two white shells, and per side upper arm, forearm with the paw, thigh, shank and foot."""
    out = [
        loft("rump", "spine-r", RED, 0.38, [st(-10, 8, 8, -0.4), st(6, 24, 27.5), st(72, 25.2, 28.4)] + dome(72, 26, 25.2, 28.4)),
        loft("chest", "spine-f", RED, 0.4, dome(0, -26, 25.2, 28.4) + [st(0, 25.2, 28.4), st(62, 19.4, 19.8, 4.4)] + dome(62, 30, 19.4, 19.8, 4.4, vc1=5)),
        loft("head", "head", RED, 0.4, dome(-2, -16, 17.4, 16.8, 3) + [st(-2, 18, 17.2, 3.2), st(16, 20.4, 18.6, 4.8), st(50, 9.8, 9, 0.2), st(54.6, 2.8, 2.8, 0)]),
        loft("tail-1", "tail-1", TAIL, 0.3, [st(-7, 2.8, 2.8), st(14, 15, 11.5, 1), st(46, 25, 16.4, 4.2), st(85, 27, 17.5, 3)] + dome(85, 17, 27, 17.5, 3)),
        loft("tail-2", "tail-2", TAIL, 0.3, dome(0, -17, 27, 17.5, 3) + [st(0, 27, 17.5, 3), st(40, 26, 16.8, 2.4), st(84, 19, 13, 6), st(109, 2, 2, 16)]),
        loft("belly-patch", "spine-f", WHITE, 0.92, dome(0, -14, 18.1, 15.6, -15) + [st(0, 18.1, 15.6, -15), st(62, 13.6, 10.6, -6.4), st(84, 5, 5, -2)]),
        loft("belly-patch-rump", "spine-r", WHITE, 0.92, [st(3, 6, 6, -21), st(20, 17.6, 16.5, -16), st(72, 18.1, 15.6, -15)] + dome(72, 14, 18.1, 15.6, -15)),
    ]
    for name, side, sign in SIDES:
        t = TONE[name]
        out += [
            loft(f"{name}-arm", f"arm-{side}", RED, t, dome(0, -9, 8, 11.2, -0.6) + [st(0, 8, 11.2, -0.6), st(40, 5.4, 6.6)] + dome(40, 6, 5.4, 6.6)),
            loft(f"{name}-hand", f"forearm-{side}", SKIN, t + 0.2, dome(0, -6, 5.4, 6.6) + [st(0, 5.4, 6.6), st(40, 4, 4), st(50, 2.4, 2)]),
            loft(f"{name}-thigh", f"thigh-{side}", RED, t, dome(-2, -16, 10.6, 18, -1.6, sign * 6) + [st(-2, 10.6, 18, -1.6, sign * 6), st(10, 11.4, 19, -2, sign * 6.4), st(50, 7.8, 9.4)] + dome(50, 8.4, 7.8, 9.4)),
            loft(f"{name}-shank", f"shank-{side}", RED, t, dome(0, -8.4, 7.8, 9.4) + [st(0, 7.8, 9.4), st(58, 4.2, 4.5)] + dome(58, 4.2, 4.2, 4.5)),
            loft(f"{name}-sole", f"foot-{side}", RED, t + 0.1, [st(-8, 1.8, 2, -6), st(0, 5, 5, -3), st(40, 6.4, 3.4, -2.6), st(51.6, 1.6, 1.2, -2.2)], e=2.4),
        ]
    for name, side, sign in SIDES:
        out.append(loft(f"{name}-ear", "head", RED, 0.3, [st(0, 6.8, 2.9), st(14, 8, 2.5), st(32, 1.1, 0.9)], frame=((8.0, 15.0, sign * 12.4), EAR_A(sign), EAR_X(sign), EAR_Y(sign))))
    return out


def thermal(body, denominator):
    """One surface per region on the body solids of the scale; nose, breath outlet and the plume saddle on the loin as own solids at the size of the scale."""
    ids = {s["id"] for s in body}
    k = 1.0 if denominator == 200 else 1.15
    out = [ref("th-rump", "rump", "fur-back"), ref("th-chest", "chest", "fur-back"), ref("th-belly", "belly-patch", "fur-belly"), ref("th-belly-rump", "belly-patch-rump", "fur-belly"), ref("th-head", "head", "head-fur")]
    if "neck" in ids:
        out.append(ref("th-neck", "neck", "head-fur"))
    out.append(ellipsoid("th-nose", "head", SKIN, 0.12, (50.9, 0, 0), (4.4 * k, 7.4 * k, 7.8 * k), region="nose"))
    out += [ref("th-near-ear", "near-ear", "ears"), ref("th-far-ear", "far-ear", "ears"), ref("th-tail-1", "tail-1", "tail-fur"), ref("th-tail-2", "tail-2", "tail-fur")]
    for name, side, sign in SIDES:
        out.append(ref(f"th-{name}-arm", f"{name}-arm", "arms-fur"))
        if f"{name}-forearm" in ids:
            out.append(ref(f"th-{name}-forearm", f"{name}-forearm", "arms-fur"))
        out.append(ref(f"th-{name}-hand", f"{name}-hand", "hands-skin"))
        out += [ref(f"th-{name}-thigh", f"{name}-thigh", "legs-fur"), ref(f"th-{name}-shank", f"{name}-shank", "legs-fur"), ref(f"th-{name}-sole", f"{name}-sole", "feet-skin")]
    out.append(ellipsoid("th-breath-nose", "head", SKIN, 0.2, (52.2, 0.6, 0), (2.9 * k, 2.9 * k, 3.5 * k), region="nose", tags=["emitter", "breath-nose"]))
    plume = [st(21, 4, 3, 28.6), st(31, 11.6 * k, 7.8, 25), st(49, 11.6 * k, 7.8, 24.8), st(60, 4, 3, 28)]
    out.append(loft("th-plume-back", "spine-r", RED, 0.8, plume, sub=1, region="fur-back", tags=["emitter", "plume-back"]))
    return out


def skeleton(denominator):
    """Skull and pelvis as masses; column (dorsal, 15 mm above the trunk axis), neck, tail and every long bone as slender bars; rib hoops hung under the column (three at 1:200, two at 1:500); two masses."""
    d = 0.5 if denominator == 200 else 1.0
    bar = lambda sid, bone, stations: member(sid, bone, stations, sub=1, slender=True)
    out = [
        bar("m-spine-r", "spine-r", [st(-4 - d, 1.9, 1.9, 12), st(32, 1.9, 1.9, 15.4), st(76 + d, 1.9, 1.9, 14)]),
        member("pelvis", "spine-r", [st(-6, 8.6 + d, 4, -1.6), st(10, 12.6 + d, 7.4, 1.5), st(24, 12.8 + d, 7.2, 5), st(42, 9 + d, 3.4, 12.4)], sub=1),
        bar("m-spine-f", "spine-f", [st(-4 - d, 1.7, 1.7, 14), st(22, 1.7, 1.7, 14.4), st(44, 1.7, 1.7, 12), st(66 + d, 1.7, 1.7, 7.4)]),
        bar("m-neck", "neck", [st(-3 - d, 0.7, 0.7, 1.6), st(12, 0.7, 0.7, 2.4), st(27 + d, 0.7, 0.7, 0.5)]),
        member("m-head", "head", [st(-1, 4.2, 4.2, 3), st(5, 10.8 + d, 8.8, 4.8), st(20, 13 + d, 9.6, 5.2), st(30, 12 + d, 7.4, 3.4), st(51.5, 2.2, 2.4, 0.6)], sub=1),
        bar("m-tail-1", "tail-1", [st(-4 - d, 2.2, 2.2), st(42, 2.2, 2.2, -1.2), st(88 + d, 2, 2)]),
        bar("m-tail-2", "tail-2", [st(-3 - d, 1.9, 1.9), st(46, 1.6, 1.6, 2), st(90 + d, 0.9, 0.9, 6.6)]),
    ]
    hoops = [(12, 14.4, -1.2), (28, 16.4, -2.4), (46, 12.6, 0.2)] if denominator == 200 else [(14, 14.8, -1.4), (38, 14.6, -1)]
    out += [rib(i + 1, u, radius, vc) for i, (u, radius, vc) in enumerate(hoops)]
    rods = {
        "arm": [st(-2 - d, 1, 1), st(42 + d, 1, 1)],
        "forearm": [st(-5 - d, 1, 1, -1.6), st(4, 1, 1), st(41.5 + d, 1, 1)],
        "hand": [st(-1.5 - d, 0.65, 0.65), st(13, 0.65, 0.65, -0.5), st(26 + d, 0.65, 0.65, -2.2)],
        "thigh": [st(-3 - d, 1.35, 1.35), st(53 + d, 1.35, 1.35)],
        "shank": [st(-3 - d, 1.5, 1.5), st(60 + d, 1.5, 1.5)],
        "foot": [st(-7 - d, 1.35, 1.35, -3), st(0, 1.35, 1.35, -1.6), st(50 + d, 1.35, 1.35, -1.2)],
    }
    for name, side, sign in SIDES:
        for key, stations in rods.items():
            out.append(bar(f"m-{key}-{side}", f"{key}-{side}", stations))
    r = 7.9 if denominator == 200 else 8.2
    out += [ellipsoid("mass-spine-f", "spine-f", BONE, MASS_TONE, (31, 0, 0), (r, r, r), member="spine-f", part="mass"), ellipsoid("mass-head", "head", BONE, MASS_TONE, (22, 2.6, 0), (r - 0.3, r - 0.3, r - 0.3), member="head", part="mass")]
    return out


def text(en, de):
    return {"en": en, "de": de}


DECISIONS = {
    "200": [
        text(
            "Body at 1:200 (smallest feature 30 mm), authored anew for the digital model; the drawing is unavailable because the sitting animal is 1.05 mm high on paper. 22 masses with three to six own sections: rump, chest, a neck collar, the head as one wedge, two leaf ears, the brush in two links bowed into an S, the white underside as two shells, and per side upper arm, forearm, paw, thigh, shank and the whole foot. Eyes, nose pad, fingers, throat patch and the separate sole are merged.",
            "Körper bei 1:200 (kleinstes Merkmal 30 mm), für das digitale Modell neu verfasst; die Zeichnung entfällt, weil das sitzende Tier auf dem Papier 1,05 mm hoch ist. 22 Massen mit drei bis sechs eigenen Schnitten: Becken, Brust, ein Halskragen, der Kopf als ein Keil, zwei Blattohren, der Schwanz in zwei zu einem S gebogenen Gliedern, die weiße Unterseite als zwei Schalen und je Seite Oberarm, Unterarm, Pfote, Oberschenkel, Unterschenkel und der ganze Fuß. Augen, Nasenspiegel, Finger, Kehlfleck und die eigene Sohle sind verschmolzen.",
        ),
        text(
            "Thermal at 1:200: one surface per region; the foot carries the region of the bare sole, the paw that of the hand; nose, breath outlet and the plume saddle on the loin are own solids. Structure at 1:200: skull and pelvis as masses, the column dorsal in the trunk, neck, tail and every long bone as slender bars, three rib hoops under the column, two masses.",
            "Thermik bei 1:200: eine Oberfläche je Region; der Fuß trägt die Region der nackten Sohle, die Pfote die der Hand; Nase, Atemauslass und der Fahnensattel auf der Lende sind eigene Körper. Tragwerk bei 1:200: Schädel und Becken als Massen, die Wirbelsäule dorsal im Rumpf, Hals, Schwanz und jeder Röhrenknochen als schlanke Stäbe, drei Rippenbögen unter der Wirbelsäule, zwei Massen.",
        ),
    ],
    "500": [
        text(
            "Body at 1:500 (smallest feature 75 mm), authored anew for the digital model; the animal is 0.42 mm high on paper. 19 masses with two to four own sections: rump, chest with the neck in its front dome, head, two ears, the brush in two links, two white shells, and per side upper arm, forearm with the paw as one piece, thigh, shank and foot.",
            "Körper bei 1:500 (kleinstes Merkmal 75 mm), für das digitale Modell neu verfasst; das Tier ist auf dem Papier 0,42 mm hoch. 19 Massen mit zwei bis vier eigenen Schnitten: Becken, Brust mit dem Hals in ihrer vorderen Kuppel, Kopf, zwei Ohren, der Schwanz in zwei Gliedern, zwei weiße Schalen und je Seite Oberarm, Unterarm mit der Pfote als ein Stück, Oberschenkel, Unterschenkel und Fuß.",
        ),
        text(
            "Thermal at 1:500: one surface per region; the forearm piece carries the region of the hand, the foot that of the sole. Structure at 1:500: skull, pelvis, the dorsal column, neck, tail and long bones as slender bars, two rib hoops, two masses.",
            "Thermik bei 1:500: eine Oberfläche je Region; das Unterarmstück trägt die Region der Hand, der Fuß die der Sohle. Tragwerk bei 1:500: Schädel, Becken, die dorsale Wirbelsäule, Hals, Schwanz und Röhrenknochen als schlanke Stäbe, zwei Rippenbögen, zwei Massen.",
        ),
    ],
}
