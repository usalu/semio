"""⌚️ Generator of `👤️humans/person-checking-watch`: a 1.75 m woman waiting impatiently, glancing at the wristwatch on her raised left wrist and tapping her right foot.

Derived from `standing-phone-user` (rig, outlines, the four handcrafted volumes, ponytail and the headphones worn round the neck) by a similarity
of 1.75/1.69 about the ground origin; the smartphone is removed, a mechanical wristwatch (steel case, leather strap) is added on the left wrist,
and all poses, clips, texts, tones and the scenario are authored here. Run: `python3 gen_person_checking_watch.py`."""
import copy
import json

from hw17lib import *

SLUG = "person-checking-watch"
STATURE, BASE_STATURE = 1.75, 1.69
S_ = STATURE / BASE_STATURE
BODY_KG, CLOTHES_KG, AGE = 64.0, 1.2, 46
DROP_BONES = {"phone"}
DROP_REGIONS = {"phone"}
DROP_MATERIALS = {"phone-glass-metal"}
DROP_ANCHORS = {"phone-cog", "grip-r"}


def strip_phone(a):
    """📵️ Removes the smartphone everywhere: bone, anchor, material, region, operating key, shapes, solids and tracks."""
    a["rig"]["bones"] = [b for b in a["rig"]["bones"] if b["id"] not in DROP_BONES]
    a["anchors"] = [x for x in a["anchors"] if x["id"] not in DROP_ANCHORS and x["bone"] not in DROP_BONES]
    a["materials"] = [m for m in a["materials"] if m["id"] not in DROP_MATERIALS]
    t = a["thermal"]
    t["regions"] = [r for r in t["regions"] if r["id"] not in DROP_REGIONS]
    t["operating"] = []
    for rep in a["representations"].values():
        for view, lst in rep["geometry"].items():
            keep = []
            for sh in lst:
                if sh["bone"] in DROP_BONES or sh.get("region") in DROP_REGIONS or sh.get("material") in DROP_MATERIALS:
                    continue
                if "regions" in sh:
                    sh["regions"] = [r for r in sh["regions"] if r not in DROP_REGIONS]
                    if len(sh["regions"]) == 1:
                        sh["region"] = sh.pop("regions")[0]
                keep.append(sh)
            rep["geometry"][view] = keep
    for vol in a["volumes"].values():
        gone = {s["id"] for s in vol["body"] if s.get("bone") in DROP_BONES or s.get("material") in DROP_MATERIALS}
        vol["body"] = [s for s in vol["body"] if s["id"] not in gone]
        vol["thermal"] = [s for s in vol["thermal"] if s.get("region") not in DROP_REGIONS and s.get("of") not in gone and s.get("bone") not in DROP_BONES]
        vol["structure"] = [s for s in vol["structure"] if s.get("bone") not in DROP_BONES and s.get("member") not in DROP_BONES and s.get("of") not in gone]


def remass(a):
    """⚖️ Female de Leva (1996) fractions kept from the base, rescaled to 64 kg with 1.2 kg of clothing; the 80 g watch rides on the left forearm."""
    have = mass_sum(a)
    f = (BODY_KG + CLOTHES_KG) / have
    for b in a["rig"]["bones"]:
        if "mass" in b:
            b["mass"]["kg"] = round(b["mass"]["kg"] * f, 4)
    fl = next(b for b in a["rig"]["bones"] if b["id"] == "forearm-l")
    fl["mass"]["kg"] = round(fl["mass"]["kg"] + 0.08, 4)


TONES_REGION = {"hoodie": 0.74, "jeans": 0.16, "hair": 0.1, "sneakers": 0.92}
TONES_MATERIAL = {"fleece-sweatshirt": 0.74, "denim": 0.16, "hair": 0.1, "canvas-sneaker": 0.92}


def retone(a):
    """🎨️ Her palette (tone is lightness): a light sage sweatshirt, black jeans, black hair, white canvas sneakers (body shapes only; soles and details keep theirs)."""
    for rep in a["representations"].values():
        for view in rep["geometry"].values():
            for sh in view:
                regs = [sh["region"]] if "region" in sh else sh.get("regions", [])
                if sh["role"] == "body" and len(regs) == 1 and regs[0] in TONES_REGION:
                    sh["tone"] = TONES_REGION[regs[0]]
    for vol in a["volumes"].values():
        for so in vol["body"]:
            m = so.get("material")
            if m in TONES_MATERIAL and so.get("role") == "body":
                so["tone"] = TONES_MATERIAL[m]


def materials(a):
    """🧱️ Outfit and watch."""
    by = {m["id"]: m for m in a["materials"]}
    by["fleece-sweatshirt"]["title"] = tx("Cotton-polyester fleece, sage hooded sweatshirt", "Baumwoll-Polyester-Fleece, salbeifarbenes Kapuzensweatshirt")
    by["fleece-sweatshirt"]["solarAbsorptivity"] = 0.45
    by["denim"]["title"] = tx("Black stretch denim", "Schwarzer Stretch-Denim")
    by["denim"]["solarAbsorptivity"] = 0.88
    by["canvas-sneaker"]["title"] = tx("White canvas sneaker with rubber sole", "Weißer Stoffsneaker mit Gummisohle")
    by["canvas-sneaker"]["solarAbsorptivity"] = 0.3
    by["hair"]["solarAbsorptivity"] = 0.9
    a["materials"] += [
        {"id": "brushed-steel", "title": tx("Brushed stainless steel watch case", "Gebürstetes Edelstahl-Uhrgehäuse"), "emissivity": 0.3, "solarAbsorptivity": 0.45, "densityKgM3": 7900, "conductivityWmK": 16, "specificHeatJKgK": 500, "source": "literature", "reference": "ashrae-2021"},
        {"id": "leather-strap", "title": tx("Brown leather watch strap", "Braunes Lederarmband"), "emissivity": 0.95, "solarAbsorptivity": 0.8, "densityKgM3": 900, "specificHeatJKgK": 1500, "source": "authored-estimate", "reference": "ashrae-2021"}]


def add_watch(a, rest):
    """⌚️ Strap ring and case on the back of the left wrist at the end of the cuff, at 1:50 and 1:100; merged into the forearm below."""
    s = S_
    uc = 0.238 * s
    R = 0.0306 * s + 0.0025
    half = 0.0085
    zf = rest["forearm-l"]["o"][2]
    case = [[round(uc - 0.019, 5), -0.017], [round(uc + 0.019, 5), -0.017], [round(uc + 0.019, 5), 0.017], [round(uc - 0.019, 5), 0.017]]
    band = [[round(uc - half, 5), 0], [round(uc - half, 5), round(R, 5)], [round(uc + half, 5), round(R, 5)], [round(uc + half, 5), 0]]
    for key in ("50", "100"):
        g = a["representations"][key]["geometry"]
        rev = {"kind": "revolved", "axisBone": "forearm-l", "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}
        ext = {"kind": "extruded", "sweep": "w", "z0": round(zf + R - 0.001, 5), "z1": round(zf + R + 0.008, 5)}
        for view, z in (("elevation", 0.56), ("front", 4.56), ("plan", 2.2)):
            g[view].append({"id": "watch-strap", "bone": "forearm-l", "role": "body", "z": z, "filled": True, "tone": 0.62, "region": "hoodie", "material": "leather-strap", "kind": "poly", "points": band, "closed": True, "solid": copy.deepcopy(rev)})
            if key == "50":
                g[view].append({"id": "watch-case", "bone": "forearm-l", "role": "body", "z": z + 0.01, "filled": True, "tone": 0.2, "region": "hoodie", "material": "brushed-steel", "kind": "poly", "points": case, "closed": True, "solid": copy.deepcopy(ext)})
        vol = a["volumes"][key]
        vol["body"].append({"id": "watch-strap", "kind": "loft", "bone": "forearm-l", "material": "leather-strap", "role": "body", "tone": 0.62,
                            "sections": [{"at": [round(uc - half, 5), 0, 0], "rx": round(R, 5), "ry": round(R, 5), "exponent": 2, "rotation": [0, -0.70711, 0, 0.70711]},
                                         {"at": [round(uc + half, 5), 0, 0], "rx": round(R, 5), "ry": round(R, 5), "exponent": 2, "rotation": [0, -0.70711, 0, 0.70711]}], "capStart": True, "capEnd": True})
        if key == "50":
            vol["body"].append({"id": "watch-case", "kind": "box", "bone": "forearm-l", "material": "brushed-steel", "role": "body", "tone": 0.2, "size": [0.038, 0.034, 0.009], "cornerRadius": 0.004, "transform": {"translation": [round(uc, 5), 0, round(R + 0.0035, 5)]}})
        vol["thermal"].append({"id": "th-watch-strap", "of": "watch-strap", "region": "hoodie"})
        if key == "50":
            vol["thermal"].append({"id": "th-watch-case", "of": "watch-case", "region": "hoodie"})


def stance():
    """🦵️ Feet flat at hip width (the base stance)."""
    return {"thigh-l": {"angleDeg": -1}, "shank-l": {"angleDeg": -5}, "foot-l": {"angleDeg": 6}, "toe-l": {"angleDeg": 0},
            "thigh-r": {"angleDeg": -1}, "shank-r": {"angleDeg": -5}, "foot-r": {"angleDeg": 6}, "toe-r": {"angleDeg": 0}}


ARM_R_DOWN = {"arm-r": {"angleDeg": 2, "abductionDeg": -6}, "forearm-r": {"angleDeg": 12}, "hand-r": {"angleDeg": 4}}
ARM_L_DOWN = {"arm-l": {"angleDeg": 1, "abductionDeg": 6}, "forearm-l": {"angleDeg": 10}, "hand-l": {"angleDeg": 3}}
WATCH = {"arm-l": {"angleDeg": 8, "abductionDeg": 6, "twistDeg": 30}, "forearm-l": {"angleDeg": 100, "twistDeg": -80}, "hand-l": {"angleDeg": -45, "abductionDeg": -4, "twistDeg": 0}}
LOOK_DOWN = {"torso": {"angleDeg": -2}, "neck": {"angleDeg": -14}, "head": {"angleDeg": -18}}
TAP = {}

REGIONS = ("face-skin", "hair", "hand-skin", "hoodie", "jeans", "sneakers", "headphones")


def binding(met, breathing, sun=0.0):
    """🌡️ Thermal binding: a covered platform in the evening, no sun."""
    r = {k: {"exposedFraction": 1, "sunlitFraction": sun} for k in REGIONS}
    r["hoodie"]["exposedFraction"] = 0.92
    r["sneakers"]["contact"] = {"fraction": 0.3, "tempC": 10}
    return {"activityMet": met, "breathing": breathing, "regions": r, "emitters": {"body-plume": 0.6}, "operating": {}}


def pose(pid, title, desc, bones, contacts, met, breathing, key=False):
    """🧍️ A pose."""
    p = {"id": pid, "title": title, "description": desc}
    if key:
        p["key"] = True
    p.update({"bones": bones, "ground": "auto", "contacts": contacts, "thermal": binding(met, breathing), "loads": []})
    return p


def poses():
    """🧍️ Waiting, glancing at the watch, tapping."""
    four = ["heel-l", "ball-l", "heel-r", "ball-r"]
    watch = {**stance(), **LOOK_DOWN, **WATCH, **ARM_R_DOWN}
    tap = {**watch, **TAP}
    wait = {**stance(), "torso": {"angleDeg": 0}, "neck": {"angleDeg": 1}, "head": {"angleDeg": 3}, **ARM_L_DOWN, **ARM_R_DOWN}
    hips = {**stance(), "torso": {"angleDeg": -1}, "neck": {"angleDeg": 2}, "head": {"angleDeg": 5}, **ARM_R_DOWN,
            "arm-l": {"angleDeg": -12, "abductionDeg": 28, "twistDeg": 40}, "forearm-l": {"angleDeg": 95, "twistDeg": -30}, "hand-l": {"angleDeg": 10}}
    return [
        pose("checking-watch", tx("Checking the watch", "Auf die Uhr schauen"),
             tx("Standing with the feet at hip width, the left forearm raised across the front of the body with the wrist turned so the watch face looks up, head bowed to read it, the right arm hanging.",
                "Mit den Füßen in Hüftbreite stehend, den linken Unterarm vor dem Körper gehoben und das Handgelenk so gedreht, dass das Zifferblatt nach oben schaut, den Kopf zum Ablesen gesenkt, der rechte Arm hängt."),
             watch, four, 1.2, "quiet-standing"),
        pose("tapping-foot", tx("Tapping the foot", "Mit dem Fuß tippen"),
             tx("Still reading the watch, the right forefoot lifted off the floor on the heel, ready to tap down again: impatience.",
                "Noch auf die Uhr schauend, den rechten Vorfuß auf der Ferse vom Boden gehoben, bereit wieder aufzutippen: Ungeduld."),
             tap, ["heel-l", "ball-l", "heel-r"], 1.3, "quiet-standing"),
        pose("waiting", tx("Waiting, looking out", "Warten, Ausschau halten"),
             tx("Both arms hanging, head raised a little to look down the platform for the train.",
                "Beide Arme hängen, der Kopf etwas gehoben, um den Bahnsteig entlang nach dem Zug zu schauen."),
             wait, four, 1.2, "quiet-standing"),
        pose("hand-on-hip", tx("Hand on the hip", "Hand in der Hüfte"),
             tx("The left hand set on the hip with the elbow out, head raised, the right arm hanging: waiting has gone on too long.",
                "Die linke Hand in die Hüfte gestützt, der Ellbogen nach außen, der Kopf gehoben, der rechte Arm hängt: Das Warten dauert schon zu lange."),
             hips, four, 1.2, "sighing"),
    ]


def clips():
    """🎞️ Impatient tapping (loop), a glance at the watch, quiet breathing."""
    period, n = 0.6, 4
    keys, spans, events = [], [], []
    for i in range(n):
        t0 = round(i * period, 3)
        keys += [{"t": t0, "pose": "checking-watch", "ease": "smooth"}, {"t": round(t0 + 0.12, 3), "pose": "checking-watch", "ease": "smooth"},
                 {"t": round(t0 + 0.36, 3), "pose": "tapping-foot", "ease": "smooth"}]
        spans.append({"anchor": "ball-r", "fromS": t0, "toS": round(t0 + 0.12, 3)})
        events.append({"t": t0, "kind": "contact-on", "title": tx("Forefoot taps down", "Vorfuß tippt auf"), "target": "ball-r"})
    keys.append({"t": round(n * period, 3), "pose": "checking-watch", "ease": "smooth"})
    dur = round(n * period, 3)
    tap = {"id": "impatient-tapping", "title": tx("Impatient foot tapping", "Ungeduldiges Fußtippen"),
           "description": tx("A loop of four taps at 100 per minute: the right forefoot rises on the heel and taps down again while the left wrist stays raised and the eyes stay on the watch.",
                             "Eine Schleife aus vier Tippern bei 100 pro Minute: Der rechte Vorfuß hebt sich auf der Ferse und tippt wieder auf, während das linke Handgelenk gehoben bleibt und der Blick auf der Uhr."),
           "durationS": dur, "fps": 30, "loop": True, "rootMotion": "none", "keys": keys,
           "contacts": [{"anchor": "heel-l", "fromS": 0, "toS": dur}, {"anchor": "ball-l", "fromS": 0, "toS": dur}, {"anchor": "heel-r", "fromS": 0, "toS": dur}] + spans,
           "phases": [{"id": f"tap-{i + 1}", "fromS": round(i * period, 3), "toS": round((i + 1) * period, 3), "title": tx(f"Tap {i + 1}", f"Tipper {i + 1}")} for i in range(n)],
           "events": events,
           "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": dur, "v": 0}]},
                      {"target": "activityMet", "ease": "linear", "keys": [{"t": 0, "v": 1.3}, {"t": dur, "v": 1.3}]}]}
    glance = {"id": "glance-at-watch", "title": tx("Glancing at the watch", "Blick auf die Uhr"),
              "description": tx("From waiting with the arms down, the left wrist comes up and turns, the head bows to read the time, holds, sighs and the arm sinks again.",
                                "Aus dem Warten mit hängenden Armen hebt und dreht sich das linke Handgelenk, der Kopf senkt sich zum Ablesen, verharrt, seufzt und der Arm sinkt wieder."),
              "durationS": 4, "fps": 30, "loop": False, "rootMotion": "none",
              "keys": [{"t": 0, "pose": "waiting", "ease": "smooth"}, {"t": 1.0, "pose": "checking-watch", "ease": "smooth"}, {"t": 2.4, "pose": "checking-watch", "ease": "smooth", "bones": {"head": {"angleDeg": -16}}},
                       {"t": 4, "pose": "waiting", "ease": "smooth"}],
              "contacts": [{"anchor": k, "fromS": 0, "toS": 4} for k in ("heel-l", "ball-l", "heel-r", "ball-r")],
              "phases": [{"id": "raise", "fromS": 0, "toS": 1, "title": tx("Raising the wrist", "Handgelenk heben")}, {"id": "read", "fromS": 1, "toS": 2.4, "title": tx("Reading the time", "Die Zeit ablesen")},
                         {"id": "lower", "fromS": 2.4, "toS": 4, "title": tx("Lowering the arm", "Arm senken")}],
              "events": [{"t": 2.4, "kind": "motion", "title": tx("Sigh", "Seufzer")}],
              "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 2.4, "v": 1}, {"t": 3.4, "v": 0}, {"t": 4, "v": 0}]}]}
    idle = {"id": "idle-breathing", "title": tx("Idle breathing", "Ruhiges Atmen"),
            "description": tx("A five-second loop while waiting: quiet nose breathing lifts the chest, the head settles in a slow small nod.",
                              "Eine Schleife von fünf Sekunden beim Warten: Ruhiges Nasenatmen hebt die Brust, der Kopf nickt langsam und klein."),
            "durationS": 5, "fps": 30, "loop": True, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "waiting", "ease": "smooth"}, {"t": 1.25, "pose": "waiting", "ease": "smooth", "bones": {"torso": {"angleDeg": -0.4}, "head": {"angleDeg": 4.5}}},
                     {"t": 2.5, "pose": "waiting", "ease": "smooth", "bones": {"torso": {"angleDeg": 0.3}, "head": {"angleDeg": 2.4}}},
                     {"t": 3.75, "pose": "waiting", "ease": "smooth", "bones": {"torso": {"angleDeg": -0.3}, "head": {"angleDeg": 4}}}, {"t": 5, "pose": "waiting", "ease": "smooth"}],
            "contacts": [{"anchor": k, "fromS": 0, "toS": 5} for k in ("heel-l", "ball-l", "heel-r", "ball-r")],
            "phases": [{"id": "idle", "fromS": 0, "toS": 5, "title": tx("Quiet standing", "Ruhiges Stehen")}], "events": [],
            "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 5, "v": 0}]}]}
    return [tap, glance, idle]


WATCH_AREA = 0.0062


def thermal(a):
    """🌡️ Evening on a covered station platform in autumn."""
    t = a["thermal"]
    t["environment"] = {"airTempC": 10, "meanRadiantTempC": 9, "skyTempC": 4, "groundTempC": 10, "relativeHumidity": 0.75, "windMS": 1.5, "windDirectionDeg": 90, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}
    t["baseMet"] = 1.2
    skin = round(dubois(BODY_KG, STATURE), 3)
    bmr = (10 * BODY_KG + 6.25 * STATURE * 100 - 5 * AGE - 161) * 4184 / 86400
    t["body"].update({"massKg": BODY_KG, "skinAreaM2": skin, "basalMetabolismW": round(bmr, 1), "source": "authored-estimate", "reference": "mifflin-1990"})
    t["respiration"]["patterns"] = [
        {"id": "quiet-standing", "title": tx("Quiet standing", "Ruhiges Stehen"), "rateBpm": 14, "tidalVolumeL": 0.4, "inhaleFraction": 0.38, "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"},
        {"id": "sighing", "title": tx("Impatient sigh", "Ungeduldiges Seufzen"), "rateBpm": 9, "tidalVolumeL": 0.9, "inhaleFraction": 0.34, "pauseAfterInhaleFraction": 0.06, "exhaleFraction": 0.5, "pauseAfterExhaleFraction": 0.1, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "icrp-66-1994"}]
    reg = {r["id"]: r for r in t["regions"]}
    reg["hair"]["title"] = tx("Black hair tied in a ponytail", "Schwarzes Haar zum Pferdeschwanz gebunden")
    reg["hoodie"]["title"] = tx("T-shirt and sage fleece sweatshirt on trunk and arms", "T-Shirt und salbeifarbenes Fleece-Sweatshirt an Rumpf und Armen")
    reg["jeans"]["title"] = tx("Black slim jeans over the legs", "Schwarze enge Jeans über den Beinen")
    for e in t["emitters"]:
        e.pop("operatingKey", None)
    t["assumptions"] = [
        tx(f"Skin area {skin} m2 after DuBois and DuBois (1916) for {BODY_KG:.0f} kg and {STATURE} m; regions keep the skin fractions of the base figure. Basal metabolism {bmr:.1f} W after Mifflin et al. (1990) for a woman of {AGE}.",
           f"Hautfläche {skin} m2 nach DuBois und DuBois (1916) für {BODY_KG:.0f} kg und {STATURE} m; die Regionen behalten die Hautanteile der Ausgangsfigur. Grundumsatz {bmr:.1f} W nach Mifflin et al. (1990) für eine Frau von {AGE} Jahren."),
        tx("Clothing is about 0.7 clo (T-shirt, fleece sweatshirt, jeans, socks, canvas sneakers) after ISO 9920; the headphones round the neck are a passive surface warmed by the body; the wristwatch sits on the sweatshirt cuff and belongs to its region. The watch is self-winding: it has no battery and draws no power.",
           "Die Kleidung hat etwa 0,7 clo (T-Shirt, Fleece-Sweatshirt, Jeans, Socken, Stoffsneaker) nach ISO 9920; die Kopfhörer um den Hals sind eine passive Fläche, die vom Körper erwärmt wird; die Armbanduhr sitzt auf dem Sweatshirtbündchen und gehört zu dessen Region. Die Uhr hat Automatikaufzug: keine Batterie und keine Leistungsaufnahme."),
        tx("Reference case: a covered station platform on an autumn evening, 10 degrees C, 75 % humidity, 1.5 m/s draught along the platform, no sun; standing is 1.2 met (ISO 7730 Annex A), tapping the foot 1.3 met (authored).",
           "Referenzfall: ein überdachter Bahnsteig an einem Herbstabend, 10 Grad C, 75 % Feuchte, 1,5 m/s Zugluft entlang des Bahnsteigs, keine Sonne; Stehen sind 1,2 met (ISO 7730 Anhang A), Fußtippen 1,3 met (geschätzt)."),
        tx("Breathing patterns are authored after quiet adult values (ICRP 66); the impatient sigh is a slow deep breath through nose and mouth.",
           "Die Atemmuster sind nach ruhigen Erwachsenenwerten (ICRP 66) geschätzt; das ungeduldige Seufzen ist ein langsamer tiefer Atemzug durch Nase und Mund.")]


def texts(a):
    """🗣️ Title, summary, tags and provenance."""
    a["id"] = SLUG
    a["title"] = tx("Person Checking a Watch", "Person schaut auf die Uhr")
    a["summary"] = tx(
        "A woman of 1.75 m waits on a station platform in a sage hooded sweatshirt, black slim jeans and white canvas sneakers, headphones round her neck and her hair in a ponytail; she raises her left wrist to read a mechanical wristwatch and taps her right foot. Poses: checking the watch, tapping the foot, waiting and a hand on the hip; clips: impatient foot tapping, a glance at the watch and quiet breathing.",
        "Eine Frau von 1,75 m wartet auf einem Bahnsteig in salbeifarbenem Kapuzensweatshirt, schwarzen engen Jeans und weißen Stoffsneakern, Kopfhörer um den Hals und das Haar zum Pferdeschwanz gebunden; sie hebt das linke Handgelenk, um eine mechanische Armbanduhr abzulesen, und tippt mit dem rechten Fuß. Posen: auf die Uhr schauen, mit dem Fuß tippen, warten und Hand in der Hüfte; Clips: ungeduldiges Fußtippen, ein Blick auf die Uhr und ruhiges Atmen.")
    a["tags"] = ["human", "adult", "woman", "standing", "waiting", "impatient", "wristwatch", "foot-tapping", "platform", "ponytail", "headphones"]
    pv = a["provenance"]
    pv.update({"author": "Semio staffage human worker HW-17", "method": "handcrafted", "created": "2026-11-22"})
    pv["notes"] = tx(
        "Rig, outlines and the four handcrafted volumes are derived from the archive's standing-phone-user figure: every length is scaled by 1.75/1.69 about the ground origin (stature 1.75 m barefoot), the segment masses keep the female fractions of de Leva (1996) for 64 kg with 1.2 kg of clothing; the smartphone is removed and a 38 mm steel wristwatch on a leather strap (80 g) is added on the left wrist; all poses and clips, materials, tones and the platform scenario are authored for this person. Breadths and depths scale with the stature and are those of a tall, slim woman (about the 95th percentile of stature of DIN 33402-2).",
        "Skelett, Umrisse und die vier handgefertigten Volumen sind aus der Archivfigur standing-phone-user abgeleitet: Jede Länge ist um 1,75/1,69 um den Bodenursprung skaliert (Körperhöhe barfuß 1,75 m), die Segmentmassen behalten die Frauenanteile von de Leva (1996) für 64 kg mit 1,2 kg Kleidung; das Smartphone ist entfernt und eine Stahl-Armbanduhr von 38 mm am Lederarmband (80 g) ist am linken Handgelenk ergänzt; alle Posen und Clips, Materialien, Tonwerte und das Bahnsteigszenario sind für diese Person geschaffen. Breiten und Tiefen skalieren mit der Körperhöhe und sind die einer großen, schlanken Frau (etwa das 95. Perzentil der Körperhöhe nach DIN 33402-2).")


def solve_tap(doc):
    """🦶️ The tapping foot: forefoot raised 40 mm on the planted heel, solved through the library kinematics."""
    path = f"{GEN}/pcw-solve-base.json"
    json.dump(doc, open(path, "w"))
    flat = frames(path, ["checking-watch"])["checking-watch"]["anchors"]
    hx, hy = flat["heel-r"][0], flat["heel-r"][1]

    def make(x):
        return {"thigh-r": {"angleDeg": -1 + x[0]}, "shank-r": {"angleDeg": -5 - x[1]}, "foot-r": {"angleDeg": 6 + x[2]}, "toe-r": {"angleDeg": 0}}

    def res(st):
        an = st["anchors"]
        return [an["heel-r"][0] - hx, an["heel-r"][1] - hy, an["ball-r"][1] - (flat["ball-r"][1] + 0.04)]

    x, r = solve_pose(doc, "checking-watch", make, res, [2.0, 2.0, -12.0])
    print("[DEBUG] tap solve", [round(v, 3) for v in x], [round(v * 1000, 2) for v in r])
    return make([round(v, 2) for v in x])


def main():
    doc = load("standing-phone-user")
    a = doc["asset"]
    strip_phone(a)
    scale_asset(a, S_, 1.0)
    remass(a)
    retone(a)
    materials(a)
    tmp = f"{GEN}/pcw-rest.json"
    json.dump(doc, open(tmp, "w"))
    rest = frames(tmp)["rest"]
    add_watch(a, rest)
    texts(a)
    thermal(a)
    a["poses"] = poses()
    a["clips"] = clips()
    TAP.update(solve_tap(doc))
    a["poses"] = poses()
    a["presentation"] = {"defaultPose": "checking-watch", "defaultClip": "impatient-tapping", "defaultScale": 100}
    a["structure"]["supports"] = []
    a["structure"]["assumptions"] = [
        tx("Segments are point masses at the scaled de Leva centres (female table); the 80 g wristwatch is part of the left forearm.", "Segmente sind Punktmassen in den skalierten Schwerpunkten nach de Leva (Frauentabelle); die 80 g schwere Armbanduhr gehört zum linken Unterarm."),
        tx("Contacts are heel and ball of each foot; while tapping, the right foot keeps only its heel on the floor.", "Kontakte sind Ferse und Ballen jedes Fußes; beim Tippen bleibt der rechte Fuß nur mit der Ferse am Boden.")]
    a["dimensions"]["massKg"] = round(mass_sum(a), 2)
    write(SLUG, doc)
    w, h, d = volume_dims(SLUG, "checking-watch", 50)
    doc = load(SLUG)
    doc["asset"]["dimensions"].update({"widthM": round(w, 3), "heightM": round(h, 3), "depthM": round(d, 3)})
    write(SLUG, doc)
    print("[DEBUG] written", SLUG, doc["asset"]["dimensions"])


if __name__ == "__main__":
    main()
