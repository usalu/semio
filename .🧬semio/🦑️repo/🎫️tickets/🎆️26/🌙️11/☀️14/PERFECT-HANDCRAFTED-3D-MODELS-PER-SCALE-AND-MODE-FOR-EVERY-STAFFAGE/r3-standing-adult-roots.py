"""🌱 Root fixes of `humans/standing-adult` (round 3): rig joints, foot on a real sole, standing pose, anchors, dimensions, provenance and the redrawn 2D shapes.

Usage: python3 r3-standing-adult-roots.py            writes the draft asset `🗑️generated/r3-standing-adult/asset.json`
       python3 r3-standing-adult-roots.py --apply    re-reads the catalog asset and replaces only the changed members in it
Every value is stated here by hand with its reason; the redrawn shapes come from `🗑️generated/r3-standing-adult/shapes.json` (written by r3-standing-adult-2d.ts) when that file exists. The rig and clip changes are relative to the round-2 asset and are applied once (while the hip joint still stands at 0.883 m); everything else can be applied again.
"""
import copy
import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import r3_standing_adult_fmt as fmt

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(T, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"))
ASSET = os.path.join(CATALOG, "👤️humans/standing-adult/🔣️.json")
MANIFEST = os.path.join(CATALOG, "🔣️.json")
OUT = os.path.join(T, "🗑️generated/r3-standing-adult")
SHAPES = os.path.join(OUT, "shapes.json")

SOLE = 0.02
BAREFOOT_HIP = 0.883
STATURE = 1.69
ANKLE = 0.039 * STATURE + SOLE
BALL_AHEAD = 0.1496
FOOT_LEN = round(math.hypot(BALL_AHEAD, ANKLE), 5)
FOOT_DROP = math.degrees(math.atan2(ANKLE, BALL_AHEAD))
C7 = (-0.020, 0.855 * STATURE + SOLE)
S1 = (-0.035, 0.883 + SOLE + 0.0951)
AO = (0.0132, round(0.912 * STATURE + SOLE, 3))
SHOULDER = (0.0, 0.883 + SOLE + 0.0951 + 0.39941)
HEAD_TOP = STATURE + SOLE

LEGS = {"thigh": -1, "shank": -5, "foot": 6}
ARM_SPREAD = 6
HANGING = {"arm-l": (3, 0), "forearm-l": (8, 7), "hand-l": (4, 3), "arm-r": (5, 1), "forearm-r": (10, 9), "hand-r": (6, 4)}
IDLE = {"arm-r": -4, "arm-l": -3, "forearm-r": -1}
CROSSED = {
    "arm-l": {"angleDeg": 30, "abductionDeg": 4, "twistDeg": 80},
    "forearm-l": {"angleDeg": 92},
    "hand-l": {"angleDeg": 32},
    "arm-r": {"angleDeg": 37, "abductionDeg": -4, "twistDeg": -80},
    "forearm-r": {"angleDeg": 99},
    "hand-r": {"angleDeg": 32},
}


def fix_rig(asset):
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    pelvis = bones["pelvis"]
    pelvis["y"] = round(pelvis["y"] + SOLE, 5)
    torso = bones["torso"]
    torso["title"] = {"en": "Torso with sweatshirt", "de": "Rumpf mit Sweatshirt"}
    torso["y"] = round(-S1[0], 5)
    lean = math.degrees(math.atan2(C7[0] - S1[0], C7[1] - S1[1]))
    torso["angleDeg"] = round(-lean, 3)
    torso["lengthM"] = round(math.hypot(C7[0] - S1[0], C7[1] - S1[1]), 5)
    a = math.radians(90 - lean)
    u, v = (math.cos(a), math.sin(a)), (-math.sin(a), math.cos(a))
    into = lambda p: (round((p[0] - S1[0]) * u[0] + (p[1] - S1[1]) * u[1], 5), round((p[0] - S1[0]) * v[0] + (p[1] - S1[1]) * v[1], 5))
    com = into((0.0, S1[1] + torso["mass"]["comX"]))
    torso["mass"]["comX"], torso["mass"]["comY"] = round(com[0], 4), round(com[1], 4)
    for side in ("l", "r"):
        arm = bones["arm-" + side]
        arm["x"], arm["y"] = into(SHOULDER)
        arm["angleDeg"] = round(180 + lean, 3)
    neck = bones["neck"]
    tilt = math.degrees(math.atan2(AO[0] - C7[0], AO[1] - C7[1]))
    neck["x"], neck["y"] = torso["lengthM"], 0
    neck["angleDeg"] = round(lean - tilt, 3)
    neck["lengthM"] = round(math.hypot(AO[0] - C7[0], AO[1] - C7[1]), 5)
    neck["mass"]["comX"] = round(neck["lengthM"] / 2, 4)
    head = bones["head"]
    old_com_y = 0.883 + 0.0951 + 0.4422 + 0.0951 * math.cos(math.radians(8)) + head["mass"]["comX"] + SOLE
    head["x"] = neck["lengthM"]
    head["angleDeg"] = round(tilt, 3)
    head["lengthM"] = round(HEAD_TOP - AO[1], 5)
    head["mass"]["comX"] = round(old_com_y - AO[1], 4)
    for side in ("l", "r"):
        foot, toe = bones["foot-" + side], bones["toe-" + side]
        foot["mass"]["comX"] = round(foot["mass"]["comX"] * FOOT_LEN / foot["lengthM"], 4)
        foot["angleDeg"] = round(90 - FOOT_DROP, 3)
        foot["lengthM"] = FOOT_LEN
        toe["x"] = FOOT_LEN
        toe["angleDeg"] = round(FOOT_DROP, 3)


def foot_local(dx, dy):
    a = math.radians(-FOOT_DROP)
    u, v = (math.cos(a), math.sin(a)), (-math.sin(a), math.cos(a))
    return round(dx * u[0] + dy * u[1], 4), round(dx * v[0] + dy * v[1], 4)


def fix_anchors(asset):
    anchors = {a["id"]: a for a in asset["anchors"]}
    head_len = next(b for b in asset["rig"]["bones"] if b["id"] == "head")["lengthM"]
    for side in ("l", "r"):
        heel, ball = anchors["heel-" + side], anchors["ball-" + side]
        heel["x"], heel["y"] = foot_local(-0.032, -ANKLE)
        heel["footprintLengthM"], heel["footprintWidthM"] = 0.07, 0.062
        ball["x"], ball["y"] = FOOT_LEN, 0
        ball["footprintLengthM"], ball["footprintWidthM"] = 0.09, 0.092
    anchors["mouth"]["x"], anchors["mouth"]["y"] = round(head_len - 0.180, 4), -0.101
    anchors["nostrils"]["x"], anchors["nostrils"]["y"] = round(head_len - 0.160, 4), -0.107
    anchors["crown"]["x"], anchors["crown"]["y"] = head_len, 0
    anchors["ear"]["x"], anchors["ear"]["y"] = round(head_len - 0.128, 4), 0.01


def fix_poses(asset, fresh):
    for pose in asset["poses"]:
        b = pose["bones"]
        for side in ("l", "r"):
            for part, deg in LEGS.items():
                b[part + "-" + side]["angleDeg"] = deg
        for side, sign in (("l", 1), ("r", -1)):
            arm = b["arm-" + side]
            if "abductionDeg" not in arm and abs(arm["angleDeg"]) <= 10 and b["forearm-" + side]["angleDeg"] <= 20:
                arm["abductionDeg"] = sign * ARM_SPREAD
                for part in ("arm-", "forearm-", "hand-"):
                    old, new = HANGING[part + side]
                    if b[part + side]["angleDeg"] == old:
                        b[part + side]["angleDeg"] = new
    pose = next(p for p in asset["poses"] if p["id"] == "arms-crossed")
    pose["bones"].update(CROSSED)
    pose["description"] = {
        "en": "Feet at hip width, both upper arms brought forward and turned inwards, the forearms folded across the belly one above the other, each hand resting on the opposite arm, shoulders slightly raised.",
        "de": "Füße hüftbreit, beide Oberarme nach vorn geführt und einwärts gedreht, die Unterarme übereinander vor dem Bauch verschränkt, jede Hand ruht auf dem anderen Arm, Schultern leicht hochgezogen.",
    }
    shiver = next(c for c in asset["clips"] if c["id"] == "shiver")
    for index, key in enumerate(shiver["keys"]):
        if "bones" in key:
            sway = -1 if index % 2 else 1
            key["bones"]["arm-l"] = {**CROSSED["arm-l"], "angleDeg": CROSSED["arm-l"]["angleDeg"] - 2 * sway}
            key["bones"]["arm-r"] = {**CROSSED["arm-r"], "angleDeg": CROSSED["arm-r"]["angleDeg"] + 2 * sway}
            key["bones"]["forearm-l"] = {"angleDeg": CROSSED["forearm-l"]["angleDeg"] - 3 * sway}
            key["bones"]["forearm-r"] = {"angleDeg": CROSSED["forearm-r"]["angleDeg"] + 3 * sway}
    for clip in asset["clips"] if fresh else []:
        for key in clip["keys"]:
            for bone, off in key.get("bones", {}).items():
                if bone.startswith("thigh-") and "angleDeg" in off:
                    off["angleDeg"] = round(off["angleDeg"] - 7, 1)
                if clip["id"] == "idle-breathing" and bone in IDLE:
                    off["angleDeg"] = round(off["angleDeg"] + IDLE[bone], 1)
        if clip["id"] == "fanning":
            for key in clip["keys"]:
                key["bones"]["arm-r"]["abductionDeg"] = 0


def fix_provenance(asset):
    refs = asset["provenance"]["references"]
    have = {r["id"] for r in refs}
    new = [
        {"id": "gordon-2014", "citation": "Gordon, C. C. et al. (2014). 2012 Anthropometric Survey of U.S. Army Personnel: Methods and Summary Statistics (ANSUR II). Natick/TR-15/007; breadths, depths, circumferences and landmark heights of adults."},
        {"id": "din-33402-2-2005", "citation": "DIN 33402-2:2005 Ergonomie, Körpermaße des Menschen, Teil 2: Werte; percentiles of body dimensions of adults in Germany."},
    ]
    refs.extend(r for r in new if r["id"] not in have)
    notes = asset["provenance"]["notes"]
    add_en = " Round 3 (2026-11-19): the skeleton stands on a 20 mm sneaker sole (stature 1.69 m barefoot, 1.71 m shod), the trunk bone runs along the spine from the lumbosacral joint 35 mm behind the hip joints to the seventh cervical vertebra (0.855 of stature) and the head joint is the atlanto-occipital joint; breadths, depths and girths follow ANSUR II and DIN 33402-2 for a slim adult of 59 kg; the drawings of every scale are the orthographic views of that scale's handcrafted 3D body."
    add_de = " Runde 3 (19.11.2026): Das Skelett steht auf einer 20 mm starken Sneakersohle (Körperhöhe barfuß 1,69 m, mit Schuhen 1,71 m), der Rumpfknochen folgt der Wirbelsäule vom Lumbosakralgelenk 35 mm hinter den Hüftgelenken bis zum siebten Halswirbel (0,855 der Körperhöhe) und das Kopfgelenk ist das Atlantookzipitalgelenk; Breiten, Tiefen und Umfänge folgen ANSUR II und DIN 33402-2 für eine schlanke erwachsene Person von 59 kg; die Zeichnungen jedes Maßstabs sind die orthogonalen Ansichten des handgefertigten 3D-Körpers dieses Maßstabs."
    if "Round 3" not in notes["en"]:
        notes["en"] += add_en
        notes["de"] += add_de
    asset["structure"]["assumptions"][1] = {
        "en": "Contacts are heel and ball of each foot; both feet stand flat at hip-joint width in every named pose.",
        "de": "Kontakte sind Ferse und Ballen jedes Fußes; beide Füße stehen in jeder benannten Pose flach in Hüftgelenkbreite.",
    }
    for pose in asset["poses"]:
        for lang, old, new_ in (("en", "Feet together", "Feet at hip width"), ("de", "Füße beieinander", "Füße hüftbreit")):
            pose["description"][lang] = pose["description"][lang].replace(old, new_)


def fix_shapes(asset):
    if not os.path.exists(SHAPES):
        return False
    shapes = json.load(open(SHAPES, encoding="utf-8"))
    for scale, rep in shapes.items():
        target = asset["representations"][scale]
        target["geometry"] = rep["geometry"]
        if "decisions" in rep:
            target["decisions"] = rep["decisions"]
        if "hatches" in rep:
            target["hatches"] = rep["hatches"]
    return True


def fix_dimensions(asset):
    """📏 The envelope of the default pose: 1.69 m stature on a 20 mm sole under 10 mm of hair; toe tip to the hood on the back; hand to hand with the arms hanging 6 degrees off the trunk (bideltoid 0.435 m plus sleeves is 0.46 m)."""
    asset["dimensions"]["heightM"] = 1.72
    asset["dimensions"]["widthM"] = 0.32
    asset["dimensions"]["depthM"] = 0.49


DECISIONS = {
    "50": {
        "silhouette": {
            "en": "The three views are the true orthographic views of the 1:50 body in the relaxed pose, 34.4 mm tall: head with jaw, nose and chin under the hair cap, ear and eye, neck, hooded sweatshirt with collar rib, hem rib, the hood on the upper back and its rolled edge over the shoulders, sleeves, forearms and rib cuffs, hands with thumb and fingers, jeans seat, thighs with kneecaps, shanks, socks, sneakers with sole, toe cap and toe sole. Inner lines: pocket, drawstrings, collar seam, hood fold, side seams, knee folds, hems, laces, finger separations, brow, mouth and hair strands. Front view (the figure's left is on the right of the sheet): the hair shows as a fringe over the forehead and a rim beside the temples; the hands are seen edge-on. Plan: shoes under shanks under thighs, hands, forearms and sleeves beside the sweatshirt, hood, neck, head and hair on top. Every part is bound to its own bone and carried rigidly through every pose.",
            "de": "Die drei Ansichten sind die wahren orthogonalen Ansichten des 1:50-Körpers in der entspannten Pose, 34,4 mm hoch: Kopf mit Kiefer, Nase und Kinn unter der Haarkappe, Ohr und Auge, Hals, Kapuzensweatshirt mit Kragenbund, Saumbund, der Kapuze auf dem oberen Rücken und ihrem Rollrand über den Schultern, Ärmel, Unterarme und Bündchen, Hände mit Daumen und Fingern, Jeansgesäß, Oberschenkel mit Kniescheiben, Unterschenkel, Socken, Sneaker mit Sohle, Zehenkappe und Zehensohle. Innenlinien: Tasche, Kordeln, Kragennaht, Kapuzenfalte, Seitennähte, Kniefalten, Säume, Schnürung, Fingertrennungen, Braue, Mund und Haarsträhnen. Vorderansicht (die linke Körperseite liegt rechts im Blatt): Das Haar zeigt sich als Pony über der Stirn und als Rand neben den Schläfen; die Hände sind von der Kante gesehen. Grundriss: Schuhe unter Unterschenkeln unter Oberschenkeln, Hände, Unterarme und Ärmel neben dem Sweatshirt, darüber Kapuze, Hals, Kopf und Haar. Jedes Teil ist an seinen eigenen Knochen gebunden und wird starr durch jede Pose getragen.",
        },
        "merged": {"en": "Eyelets merge into the lace strokes; hair strands are three strokes instead of single hairs; lips, chin and brows merge into the head outline.", "de": "Ösen gehen in den Schnürstrichen auf; Haarsträhnen sind drei Striche statt einzelner Haare; Lippen, Kinn und Brauen gehen im Kopfumriss auf."},
        "omitted": {"en": "Pores, individual hairs, stitching holes, fabric fibres; finger separations in the front view, where the hand is seen edge-on.", "de": "Poren, einzelne Haare, Stichlöcher, Gewebefasern; Fingertrennungen in der Vorderansicht, in der die Hand von der Kante gesehen ist."},
    },
    "100": {
        "silhouette": {
            "en": "The three views are the true orthographic views of the 1:100 body, 17.2 mm tall: head with nose under the hair cap, ear, neck, sweatshirt with the hood on the upper back, sleeves, forearms, mitten hands with thumb, jeans seat, thighs, shanks, one shoe form and a toe box per foot. Inner lines: cuff, hem, pocket, collar, side seam and sole. Front view (the figure's left is on the right of the sheet): hair fringe over the forehead, ears beside the head, hem, collar and pocket lines, jeans hems and sole lines. Plan: shoes, legs, hands, arms, sweatshirt, hood, neck, head and hair in the order of their height; no inner lines.",
            "de": "Die drei Ansichten sind die wahren orthogonalen Ansichten des 1:100-Körpers, 17,2 mm hoch: Kopf mit Nase unter der Haarkappe, Ohr, Hals, Sweatshirt mit der Kapuze auf dem oberen Rücken, Ärmel, Unterarme, Fäustlingshände mit Daumen, Jeansgesäß, Oberschenkel, Unterschenkel, eine Schuhform und eine Zehenkappe je Fuß. Innenlinien: Bündchen, Saum, Tasche, Kragen, Seitennaht und Sohle. Vorderansicht (die linke Körperseite liegt rechts im Blatt): Pony über der Stirn, Ohren neben dem Kopf, Saum-, Kragen- und Taschenlinien, Jeanssäume und Sohlenlinien. Grundriss: Schuhe, Beine, Hände, Arme, Sweatshirt, Kapuze, Hals, Kopf und Haar in der Reihenfolge ihrer Höhe; ohne Innenlinien.",
        },
    },
    "200": {
        "silhouette": {
            "en": "The three views are the true orthographic views of the 1:200 body, 8.6 mm tall, one mass per part: head, hair cap, neck, sweatshirt, hood, jeans seat, sleeves, forearms, mitten hands, thighs, shanks, shoes and toe boxes; the pocket is the only inner line. Front view (the figure's left is on the right of the sheet): the same masses, the legs keep a gap of more than 30 mm real size. Plan: the masses in the order of their height.",
            "de": "Die drei Ansichten sind die wahren orthogonalen Ansichten des 1:200-Körpers, 8,6 mm hoch, eine Masse je Teil: Kopf, Haarkappe, Hals, Sweatshirt, Kapuze, Jeansgesäß, Ärmel, Unterarme, Fäustlingshände, Oberschenkel, Unterschenkel, Schuhe und Zehenkappen; die Tasche ist die einzige Innenlinie. Vorderansicht (die linke Körperseite liegt rechts im Blatt): dieselben Massen, die Beine halten eine Lücke von mehr als 30 mm wahrer Größe. Grundriss: die Massen in der Reihenfolge ihrer Höhe.",
        },
    },
    "500": {
        "silhouette": {
            "en": "An upright figure of 1.72 m (3.4 mm), the true orthographic views of the 1:500 body: head with hair as one mass, neck, sweatshirt with the hood merged into the upper back, jeans seat, and per side sleeve, forearm, hand, thigh, shank, shoe and toe box. Front view (the figure's left is on the right of the sheet): the same masses with both arms hanging beside the trunk; the hands are 43 mm thick so that they print. Plan: the masses in the order of their height.",
            "de": "Eine aufrechte Figur von 1,72 m (3,4 mm), die wahren orthogonalen Ansichten des 1:500-Körpers: Kopf mit Haar als eine Masse, Hals, Sweatshirt mit im oberen Rücken aufgegangener Kapuze, Jeansgesäß und je Seite Ärmel, Unterarm, Hand, Oberschenkel, Unterschenkel, Schuh und Zehenkappe. Vorderansicht (die linke Körperseite liegt rechts im Blatt): dieselben Massen mit beiden Armen neben dem Rumpf hängend; die Hände sind 43 mm dick, damit sie drucken. Grundriss: die Massen in der Reihenfolge ihrer Höhe.",
        },
        "merged": {"en": "Face and hair; hood into the sweatshirt.", "de": "Gesicht und Haar; Kapuze im Sweatshirt."},
        "omitted": {"en": "Sleeve cuffs, hood fold, laces, sole line, thumb and fingers.", "de": "Ärmelbündchen, Kapuzenfalte, Schnürung, Sohlenlinie, Daumen und Finger."},
    },
}


def fix_decisions(asset):
    for scale, texts in DECISIONS.items():
        asset["representations"][scale]["decisions"].update(texts)


def fixed(asset):
    out = copy.deepcopy(asset)
    fresh = out["rig"]["bones"][0]["y"] == BAREFOOT_HIP
    if fresh:
        fix_rig(out)
    fix_anchors(out)
    fix_poses(out, fresh)
    fix_provenance(out)
    fix_shapes(out)
    fix_decisions(out)
    fix_dimensions(out)
    return out


CHANGED = ("provenance", "dimensions", "rig", "anchors", "representations", "poses", "clips", "structure")


def main():
    text = open(ASSET, encoding="utf-8").read()
    document = json.loads(text)
    asset = fixed(document["asset"])
    if "--apply" not in sys.argv:
        os.makedirs(OUT, exist_ok=True)
        json.dump({**document, "asset": asset}, open(os.path.join(OUT, "asset.json"), "w", encoding="utf-8"), ensure_ascii=False)
        print("draft written", os.path.join(OUT, "asset.json"))
        return
    result = text
    for key in CHANGED:
        if asset[key] == document["asset"][key]:
            continue
        start, end = fmt.member_range(result, "    ", key)
        old = result[start:end]
        plain = "    " + json.dumps(key) + ": " + json.dumps(document["asset"][key], indent=2, ensure_ascii=False).replace("\n", "\n    ")
        new = "    " + json.dumps(key) + ": " + json.dumps(asset[key], indent=2, ensure_ascii=False).replace("\n", "\n    ") if old == plain else fmt.member_text(key, asset[key], 2)
        result = result[:start] + new + result[end:]
    check = json.loads(result)
    if check["asset"] != {**asset, "volumes": document["asset"]["volumes"]} or list(check["asset"]) != list(document["asset"]):
        raise SystemExit("root edit did not round-trip")
    if open(ASSET, encoding="utf-8").read() != text:
        raise SystemExit("asset changed while editing; retry")
    open(ASSET, "w", encoding="utf-8").write(result)
    print("roots applied:", ", ".join(k for k in CHANGED if asset[k] != document["asset"][k]))
    mtext = open(MANIFEST, encoding="utf-8").read()
    at = mtext.index('"id": "standing-adult"')
    end = mtext.index('"powerW"', at)
    block = mtext[at:end]
    d0, d1 = document["asset"]["dimensions"], asset["dimensions"]
    fresh = block.replace('"heightM": %s,' % json.dumps(d0["heightM"]), '"heightM": %s,' % json.dumps(d1["heightM"])).replace('"widthM": %s,' % json.dumps(d0["widthM"]), '"widthM": %s,' % json.dumps(d1["widthM"]))
    if fresh != block:
        if open(MANIFEST, encoding="utf-8").read() != mtext:
            raise SystemExit("manifest changed while editing; retry")
        open(MANIFEST, "w", encoding="utf-8").write(mtext[:at] + fresh + mtext[end:])
        print("manifest entry updated")


main()
