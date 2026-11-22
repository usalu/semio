"""🥁️ Layout of the used five-piece drum kit: the rig (bones at rest = stored), the poses as bone offsets, and every part of every scale in world coordinates of the stored rest pose (or of the played pose where a part is placed against a played neighbour).

World: x ahead (from the drummer towards the audience), y up, z left of the drummer; the throne stands at the origin.
"""
import math
from author_kit_math import frames, to_local, to_world, dir_local, sincos, add, sub, mul, unit, norm, cross, comb

TROLLEY = (-0.05, 0.165, 1.25)
RUG = ((-0.66, 1.20), (-0.92, 1.58), 0.006)
DECK_TOP = 0.195
CASES = {
    "case-kick": {"parent": "trolley", "at": (0.265, 0.03), "R": 0.325, "H": 0.55},
    "case-floor": {"parent": "trolley", "at": (-0.335, 0.03), "R": 0.255, "H": 0.44},
    "case-tom12": {"parent": "case-kick", "at": (0.55, 0.0), "R": 0.21, "H": 0.27},
    "case-tom10": {"parent": "case-tom12", "at": (0.27, 0.0), "R": 0.185, "H": 0.25},
    "case-snare": {"parent": "case-floor", "at": (0.44, 0.0), "R": 0.22, "H": 0.18},
}
WALL, FLOOR, SKIRT_T, SKIRT_H = 0.004, 0.004, 0.004, 0.06

KICK = {"R": 0.278, "D": 0.457, "ply": 0.007, "hoopO": 0.297, "hoopI": 0.283, "hoopOut": 0.012, "hoopIn": 0.018, "lugs": 8, "lugR": 0.010}
TOM10 = {"R": 0.126, "D": 0.203, "ply": 0.006, "hoopO": 0.135, "hoopI": 0.127, "lugs": 6, "lugR": 0.007}
TOM12 = {"R": 0.1515, "D": 0.229, "ply": 0.006, "hoopO": 0.161, "hoopI": 0.153, "lugs": 6, "lugR": 0.007}
FLOORTOM = {"R": 0.202, "D": 0.406, "ply": 0.007, "hoopO": 0.212, "hoopI": 0.204, "lugs": 8, "lugR": 0.008}
SNARE = {"R": 0.177, "D": 0.140, "ply": 0.001, "hoopO": 0.186, "hoopI": 0.178, "lugs": 10, "lugR": 0.009}
HOOP_UP, HOOP_DOWN, HEAD_T = 0.014, 0.006, 0.0005

PLAYED = {
    "kick": {"o": (0.62, 0.297, 0.0)},
    "kick-holder": {"o": (0.82, 0.537, 0.0)},
    "tom10": {"o": (0.70, 0.66, 0.15)},
    "tom12": {"o": (0.72, 0.68, -0.17)},
    "floor-tom": {"o": (0.18, 0.206, -0.50)},
    "snare": {"o": (0.30, 0.507, 0.12)},
}
STANDS = {
    "hihat": {"at": (0.38, 0.46), "tubeR": 0.0127, "tubeTop": 0.58, "hub": 0.18, "legR": 0.0105, "reach": 0.26, "legs": (20, 140, 260), "upperR": 0.0095, "upperLen": 0.45, "upperRest": 0.14, "upperPlay": 0.16},
    "snare-stand": {"at": (0.30, 0.12), "tubeR": 0.011, "tubeTop": 0.32, "hub": 0.15, "legR": 0.0095, "reach": 0.21, "legs": (0, 120, 240), "upperR": 0.008, "upperLen": 0.34, "upperRest": 0.03, "upperPlay": 0.14},
    "crash": {"at": (0.95, 0.62), "tubeR": 0.0127, "tubeTop": 0.55, "hub": 0.19, "legR": 0.0105, "reach": 0.27, "legs": (0, 120, 240), "upperR": 0.0095, "upperLen": 0.45, "upperRest": 0.15, "upperPlay": 0.22, "boom": 0.40, "tilt": 15, "cymR": 0.203},
    "ride": {"at": (0.95, -0.66), "tubeR": 0.0127, "tubeTop": 0.55, "hub": 0.19, "legR": 0.0105, "reach": 0.27, "legs": (0, 120, 240), "upperR": 0.0095, "upperLen": 0.45, "upperRest": 0.15, "upperPlay": 0.14, "boom": 0.40, "tilt": 10, "cymR": 0.254},
}
THRONE = {"tubeR": 0.0175, "tubeTop": 0.36, "hub": 0.20, "legR": 0.011, "reach": 0.27, "legs": (90, 210, 330), "postR": 0.014, "seatRest": 0.40, "seatPlay": 0.06, "cushR": 0.175}
FOOT_R, FOOT_H = 0.016, 0.03
BEATER_REST, BEATER_STRIKE = 125.0, -45.6
HIHAT_OPEN = 0.012


def bone(i, parent, x, y, angle, length, joint, title, z=0.0, abduction=0.0):
    """🦴️ One bone of the rig as a dict in the shape of the schema."""
    b = {"id": i, "parent": parent, "title": title, "x": round(x, 6), "y": round(y, 6), "angleDeg": angle, "lengthM": length, "joint": joint}
    if z:
        b["z"] = round(z, 6)
    if abduction:
        b["abductionDeg"] = abduction
    return b


def T(en, de):
    """🌐️ A bilingual text."""
    return {"en": en, "de": de}


FIXED = {"type": "fixed"}


def rig():
    """🌳️ Every bone in parent-first order with its rest placement (rest = stored) and the drum chains (lift in the case plane, shift across, then the drum)."""
    out = [bone("rug", None, RUG[0][0], 0.0, 0, RUG[0][1] - RUG[0][0], FIXED, T("Drum rug under kit and trolley", "Schlagzeugteppich unter Schlagzeug und Wagen"))]
    out.append(bone("throne", "rug", -RUG[0][0], 0.0, 0, 0.27, FIXED, T("Throne base: tripod, outer tube and spindle housing", "Hockerbasis: Dreibein, Außenrohr und Spindelgehäuse")))
    out.append(bone("throne-seat", "throne", 0.0, THRONE["tubeTop"], 90, 0.24, {"type": "prismatic", "axisDeg": 90, "minM": 0.0, "maxM": 0.12}, T("Throne seat on its height-adjustable post", "Hockersitz auf seiner höhenverstellbaren Stange")))
    out.append(bone("kick-pedal", "throne", 0.29, 0.0, 0, 0.31, FIXED, T("Bass drum pedal frame: base plate, posts and axle", "Fußmaschinenrahmen: Grundplatte, Säulen und Achse")))
    out.append(bone("kick-footboard", "kick-pedal", 0.01, 0.016, 15.4, 0.26, {"type": "revolute", "minDeg": -14, "maxDeg": 2}, T("Bass drum pedal footboard on its heel hinge", "Trittplatte der Fußmaschine an ihrem Fersengelenk")))
    out.append(bone("kick-beater", "kick-pedal", 0.265, 0.20, BEATER_REST, 0.20, {"type": "revolute", "minDeg": -50, "maxDeg": 5}, T("Felt beater on the pedal axle", "Filzschlägel an der Pedalachse")))
    for key, s in STANDS.items():
        x, z = s["at"]
        out.append(bone(f"{key}-stand" if key != "snare-stand" else key, "throne", x, 0.0, 90, s["tubeTop"], FIXED, stand_title(key), z=z))
    st = lambda k: f"{k}-stand" if k != "snare-stand" else k
    for key, s in STANDS.items():
        upper = "snare-basket" if key == "snare-stand" else f"{key}-upper"
        out.append(bone(upper, st(key), s["tubeTop"], 0.0, 0, s["upperRest"] if key != "snare-stand" else 0.06, {"type": "prismatic", "axisDeg": 0, "minM": -0.10, "maxM": 0.30}, upper_title(key)))
    out.append(bone("hihat-rod", "hihat-upper", 0.0, 0.0, 0, 0.30, {"type": "prismatic", "axisDeg": 0, "minM": -0.03, "maxM": 0.03}, T("Hi-hat pull rod with clutch and top cymbal", "Hi-Hat-Zugstange mit Clutch und oberem Becken")))
    out.append(bone("hihat-footboard", "hihat-stand", 0.02, STANDS["hihat"]["at"][0] - 0.15, -75.0, 0.24, {"type": "revolute", "minDeg": -12, "maxDeg": 1}, T("Hi-hat pedal footboard", "Trittplatte des Hi-Hat-Pedals")))
    for key in ("crash", "ride"):
        s = STANDS[key]
        out.append(bone(f"{key}-boom", f"{key}-upper", s["upperRest"], 0.0, 0, s["boom"], {"type": "revolute", "minDeg": 0, "maxDeg": 120}, T(f"{key.capitalize()} boom arm on its tilter", f"{'Crash' if key == 'crash' else 'Ride'}-Galgenarm an seinem Neigegelenk")))
        out.append(bone(f"{key}-cymbal", f"{key}-boom", s["boom"], 0.0, -90, 0.03, {"type": "revolute", "minDeg": -90, "maxDeg": 30}, T(f"{'16-inch crash' if key == 'crash' else '20-inch ride'} cymbal on its felts", f"{'16-Zoll-Crash' if key == 'crash' else '20-Zoll-Ride'}-Becken auf seinen Filzen")))
    out.append(bone("trolley-track", "throne", TROLLEY[0], TROLLEY[1], 0, 0.02, FIXED, T("Floor track of the trolley (where it is parked)", "Fahrweg des Wagens am Boden (Abstellort)"), z=TROLLEY[2]))
    out.append(bone("trolley", "trolley-track", 0.0, 0.0, 0, 0.60, {"type": "prismatic", "axisDeg": 0, "minM": 0.0, "maxM": 3.0}, T("Platform trolley 1200 x 700 mm with push handle", "Plattformwagen 1200 x 700 mm mit Schiebebügel")))
    for key, xl in (("front", 0.48), ("rear", -0.48)):
        out.append(bone(f"trolley-wheels-{key}", "trolley", xl, 0.0625 - TROLLEY[1], 0, 0.0625, {"type": "revolute", "minDeg": -3600, "maxDeg": 3600}, T(f"{'Front' if key == 'front' else 'Rear'} castor wheel pair", f"{'Vorderes' if key == 'front' else 'Hinteres'} Lenkrollenpaar")))
    for cid, c in CASES.items():
        ang = 90 if c["parent"] == "trolley" else 0
        out.append(bone(cid, c["parent"], c["at"][0], c["at"][1], ang, c["H"], FIXED, case_title(cid)))
    return out


def stand_title(key):
    """🏷️ Title of a stand base bone."""
    return {
        "hihat": T("Hi-hat stand: tripod, lower tube and pedal frame", "Hi-Hat-Maschine: Dreibein, Unterrohr und Pedalrahmen"),
        "snare-stand": T("Snare stand: tripod and lower tube", "Snareständer: Dreibein und Unterrohr"),
        "crash": T("Crash boom stand: tripod and lower tube", "Crash-Galgenständer: Dreibein und Unterrohr"),
        "ride": T("Ride boom stand: tripod and lower tube", "Ride-Galgenständer: Dreibein und Unterrohr"),
    }[key]


def upper_title(key):
    """🏷️ Title of a telescoping upper tube bone."""
    return {
        "hihat": T("Hi-hat upper tube with the bottom cymbal", "Hi-Hat-Oberrohr mit dem unteren Becken"),
        "snare-stand": T("Snare stand upper tube with the basket", "Oberrohr des Snareständers mit dem Korb"),
        "crash": T("Crash stand upper tube", "Oberrohr des Crash-Ständers"),
        "ride": T("Ride stand upper tube", "Oberrohr des Ride-Ständers"),
    }[key]


def case_title(cid):
    """🏷️ Title of a case bone."""
    return {
        "case-kick": T("Bass drum hard case 22 x 18 inch", "Bassdrum-Hartschalenkoffer 22 x 18 Zoll"),
        "case-floor": T("Floor tom hard case 16 x 16 inch", "Standtom-Hartschalenkoffer 16 x 16 Zoll"),
        "case-tom12": T("Rack tom hard case 12 x 9 inch", "Hängetom-Hartschalenkoffer 12 x 9 Zoll"),
        "case-tom10": T("Rack tom hard case 10 x 8 inch", "Hängetom-Hartschalenkoffer 10 x 8 Zoll"),
        "case-snare": T("Snare hard case 14 x 5.5 inch", "Snare-Hartschalenkoffer 14 x 5,5 Zoll"),
    }[cid]


DRUMS = {
    "kick": {"case": "case-kick", "rot": True, "title": T("Bass drum 22 x 18 inch", "Bassdrum 22 x 18 Zoll"), "restAngle": 180, "playAngle": 90, "min": 0, "max": 90},
    "kick-holder": {"case": "case-kick", "rot": True, "title": T("Double tom holder with ball clamps and arms", "Doppel-Tomhalter mit Kugelklemmen und Armen"), "restAngle": 90, "playAngle": -90, "min": -90, "max": 0},
    "tom12": {"case": "case-tom12", "rot": True, "title": T("Rack tom 12 x 9 inch", "Hängetom 12 x 9 Zoll"), "restAngle": -90, "playAngle": 10, "min": -5, "max": 20},
    "tom10": {"case": "case-tom10", "rot": True, "title": T("Rack tom 10 x 8 inch", "Hängetom 10 x 8 Zoll"), "restAngle": -90, "playAngle": 10, "min": -5, "max": 20},
    "floor-tom": {"case": "case-floor", "rot": False, "title": T("Floor tom 16 x 16 inch", "Standtom 16 x 16 Zoll")},
    "snare": {"case": "case-snare", "rot": False, "title": T("Snare drum 14 x 5.5 inch, chrome-plated steel shell", "Snaredrum 14 x 5,5 Zoll, verchromter Stahlkessel")},
}


def stored_origin(did, rest_frames):
    """📦️ World origin of a drum bone in the stored rest pose (on the inner floor of its case)."""
    c = CASES[DRUMS[did]["case"]]
    co = rest_frames[DRUMS[did]["case"]][0]
    floor_top = co[1] + FLOOR
    if did == "kick":
        return [co[0], floor_top + KICK["D"] + KICK["hoopOut"], co[2]]
    if did == "kick-holder":
        return [co[0] + 0.13, floor_top + KICK["D"] + KICK["hoopOut"] + HEAD_T + 0.025, co[2]]
    return [co[0], floor_top + HOOP_DOWN, co[2]]


def add_drum_chains(bones):
    """🔗️ Appends lift (in the case plane), shift (across) and the drum bone of every drum; the lift direction is the stored-to-played travel."""
    rest = frames(bones)
    plays = {}
    for did, d in DRUMS.items():
        so = stored_origin(did, rest)
        po = list(PLAYED[did]["o"])
        delta = sub(po, so)
        cf = rest[d["case"]]
        local = dir_local(cf, [delta[0], delta[1], 0.0])
        axis = math.degrees(math.atan2(local[1], local[0]))
        travel = math.hypot(local[0], local[1])
        lo = to_local(cf, so)
        lift = f"{did}-lift"
        shift = f"{did}-shift" if d["rot"] else did
        bones.append(bone(lift, d["case"], lo[0], lo[1], 0, 0.02, {"type": "prismatic", "axisDeg": axis, "minM": -0.1, "maxM": 3.0}, T(f"Lift of the {d['title']['en'].lower()} out of its case", f"Herausheben: {d['title']['de']} aus dem Koffer"), z=lo[2], abduction=90))
        bones.append(bone(shift, lift, 0.0, 0.0, 0, 0.02 if d["rot"] else (0.40 if did == "floor-tom" else 0.14), {"type": "prismatic", "axisDeg": 0, "minM": -3.0, "maxM": 3.0}, d["title"] if not d["rot"] else T(f"Lateral travel of the {d['title']['en'].lower()}", f"Seitlicher Weg: {d['title']['de']}"), abduction=-90))
        plays[lift] = {"dx": travel * math.cos(math.radians(axis)), "dy": travel * math.sin(math.radians(axis))}
        plays[shift] = {"dx": delta[2]}
        if d["rot"]:
            length = {"kick": KICK["D"], "kick-holder": 0.24, "tom12": 0.24, "tom10": 0.24}[did]
            bones.append(bone(did, shift, 0.0, 0.0, d["restAngle"], length, {"type": "revolute", "minDeg": d["min"], "maxDeg": d["max"]}, d["title"]))
            plays[did] = {"angleDeg": d["playAngle"]}
    for side, zs in (("l", 1), ("r", -1)):
        bones.append(bone(f"kick-spur-{side}", "kick", 0.33, -0.217, 180, SPUR_L, {"type": "revolute", "minDeg": 0, "maxDeg": 160}, T(f"Bass drum spur, {'left' if zs > 0 else 'right'}", f"Bassdrum-Sporn, {'links' if zs > 0 else 'rechts'}"), z=0.218 * zs))
        plays[f"kick-spur-{side}"] = {"angleDeg": 155}
    bones.append(bone("floor-tom-legs", "floor-tom", 0.0, 0.0, 0, 0.40, {"type": "prismatic", "axisDeg": 0, "minM": -0.25, "maxM": 0.0}, T("Floor tom legs in their brackets", "Standtombeine in ihren Halterungen")))
    plays["floor-tom-legs"] = {"dx": -0.20}
    return plays


SPUR_ANGLE = -25.0
SPUR_TIP_R = 0.011
SPUR_L = round((0.080 - SPUR_TIP_R * math.cos(math.radians(25))) / math.sin(math.radians(25)), 4)


def played_offsets(plays):
    """🥁️ The bone offsets of the pose `played`: drums out of the cases, stands extended, booms swung, seat raised."""
    off = dict(plays)
    for key, s in STANDS.items():
        upper = "snare-basket" if key == "snare-stand" else f"{key}-upper"
        off[upper] = {"dx": s["upperPlay"]}
    off["throne-seat"] = {"dy": THRONE["seatPlay"]}
    off["crash-boom"] = {"angleDeg": 60}
    off["crash-cymbal"] = {"angleDeg": -60 + STANDS["crash"]["tilt"]}
    off["ride-boom"] = {"angleDeg": 60}
    off["ride-cymbal"] = {"angleDeg": -60 + STANDS["ride"]["tilt"]}
    off["hihat-rod"] = {"dx": -HIHAT_OPEN}
    off["hihat-footboard"] = {"angleDeg": -6}
    return off


def ordered(bones):
    """📚️ Parent-first order of the bones."""
    by = {b["id"]: b for b in bones}
    out, seen = [], set()

    def visit(b):
        if b["id"] in seen:
            return
        if b["parent"]:
            visit(by[b["parent"]])
        seen.add(b["id"])
        out.append(b)

    for b in bones:
        visit(b)
    return out
