"""🌱 Root fixes of `humans/standing-phone-user` (round 3): the corrected rig on a real sole with a phone that pivots in the right hand, the three poses and two clips posed in three dimensions, anchors, dimensions, provenance, the redrawn 2D shapes and their decision texts.

Usage: python3 r3-standing-phone-user-roots.py            writes the draft asset `🗑️generated/r3-standing-phone-user/asset.json`
       python3 r3-standing-phone-user-roots.py --apply    re-reads the catalog asset and replaces only the changed members in it
Every value is stated here by hand (the arm offsets come from `r3-standing-phone-user-grip.py`); the redrawn shapes come from `🗑️generated/r3-standing-phone-user/shapes.json` (written by r3-standing-phone-user-2d.ts) when that file exists. Every fix states absolute values, so the script can be applied again.
"""
import copy
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import r3_standing_phone_user_fmt as fmt

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = os.path.normpath(os.path.join(T, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"))
ASSET = os.path.join(CATALOG, "👤️humans/standing-phone-user/🔣️.json")
MANIFEST = os.path.join(CATALOG, "🔣️.json")
OUT = os.path.join(T, "🗑️generated/r3-standing-phone-user")
SHAPES = os.path.join(OUT, "shapes.json")

import r3_standing_phone_user_rig as rig

MASS_CENTRES = {"torso": (0.2053, -0.0284), "neck": (0.0508, 0), "head": (0.0463, 0), "foot-l": (0.0742, 0), "foot-r": (0.0742, 0), "phone": (0.0735, 0)}
LEGS = {"thigh": -1, "shank": -5, "foot": 6, "toe": 0}
TEXTING = {
    "torso": {"angleDeg": -3},
    "neck": {"angleDeg": -18},
    "head": {"angleDeg": -22},
    "arm-r": {"angleDeg": -1.4, "abductionDeg": -7.0, "twistDeg": -27.2},
    "forearm-r": {"angleDeg": 103.9, "twistDeg": 75.2},
    "hand-r": {"angleDeg": -8.9, "abductionDeg": 21.6, "twistDeg": -0.1},
    "arm-l": {"angleDeg": 0.9, "abductionDeg": 7.2, "twistDeg": 27.7},
    "forearm-l": {"angleDeg": 97.0, "twistDeg": -78.3},
    "hand-l": {"angleDeg": -8.5, "abductionDeg": -25.8, "twistDeg": 3.0},
    "phone": {"angleDeg": 0},
}
CALL = {
    "torso": {"angleDeg": 0},
    "neck": {"angleDeg": -2},
    "head": {"angleDeg": 3},
    "arm-r": {"angleDeg": 55.4, "abductionDeg": 0.5, "twistDeg": -19.8},
    "forearm-r": {"angleDeg": 150, "twistDeg": -13.5},
    "hand-r": {"angleDeg": 38.8, "abductionDeg": -5.4, "twistDeg": -8.7},
    "arm-l": {"angleDeg": 27.8, "abductionDeg": 12.3, "twistDeg": 90},
    "forearm-l": {"angleDeg": 88.7, "twistDeg": -27.3},
    "hand-l": {"angleDeg": 9.3, "abductionDeg": -0.5, "twistDeg": -1.1},
    "phone": {"angleDeg": -62.3},
}
PHOTO = {
    "torso": {"angleDeg": 1},
    "neck": {"angleDeg": 0},
    "head": {"angleDeg": 2},
    "arm-r": {"angleDeg": 66.5, "abductionDeg": -12.1, "twistDeg": -38.0},
    "forearm-r": {"angleDeg": 84.1, "twistDeg": 90},
    "hand-r": {"angleDeg": -0.5, "abductionDeg": 22.3, "twistDeg": 1.1},
    "arm-l": {"angleDeg": 68.1, "abductionDeg": 10.7, "twistDeg": 38.4},
    "forearm-l": {"angleDeg": 76.2, "twistDeg": -90},
    "hand-l": {"angleDeg": -0.5, "abductionDeg": -27.9, "twistDeg": -0.9},
    "phone": {"angleDeg": 0},
}
POSES = {"texting": TEXTING, "phone-call": CALL, "photo-pose": PHOTO}
DESCRIPTIONS = {
    "texting": {
        "en": "Feet at hip width, weight on both legs, the trunk leaning 3 degrees forward, neck and head bowed 40 degrees further; the upper arms hang beside the trunk, the elbows are bent about 100 degrees, the forearms turned palm-up and brought together so that both hands cradle the phone on the midline in front of the lower chest, thumbs on the screen, 35 cm below the eyes.",
        "de": "Füße hüftbreit, Gewicht auf beiden Beinen, der Rumpf 3 Grad vorgeneigt, Hals und Kopf weitere 40 Grad gesenkt; die Oberarme hängen neben dem Rumpf, die Ellbogen sind etwa 100 Grad gebeugt, die Unterarme mit den Handflächen nach oben gedreht und zusammengeführt, sodass beide Hände das Telefon in der Körpermitte vor der unteren Brust halten, die Daumen auf dem Bildschirm, 35 cm unter den Augen.",
    },
    "phone-call": {
        "en": "Feet at hip width, head upright; the right elbow is raised forward and fully bent, the right hand holds the phone along the cheek to the right ear; the left upper arm is turned inwards and the left forearm lies across the waist.",
        "de": "Füße hüftbreit, Kopf aufrecht; der rechte Ellbogen ist nach vorn angehoben und ganz gebeugt, die rechte Hand hält das Telefon entlang der Wange am rechten Ohr; der linke Oberarm ist einwärts gedreht und der linke Unterarm liegt quer vor dem Bauch.",
    },
    "photo-pose": {
        "en": "Feet at hip width, head upright; both upper arms raised forward, the elbows bent about 80 degrees, both hands hold the phone upright on the midline at eye height, 40 cm in front of the face.",
        "de": "Füße hüftbreit, Kopf aufrecht; beide Oberarme nach vorn angehoben, die Ellbogen etwa 80 Grad gebeugt, beide Hände halten das Telefon aufrecht in der Körpermitte auf Augenhöhe, 40 cm vor dem Gesicht.",
    },
}
SCROLL = [
    None,
    {"head": {"angleDeg": -25}, "neck": {"angleDeg": -19}, "torso": {"angleDeg": -4}, "hand-l": {"angleDeg": -7}},
    {"head": {"angleDeg": -19}, "neck": {"angleDeg": -16.5}, "torso": {"angleDeg": -2}, "hand-l": {"angleDeg": -10}},
    {"head": {"angleDeg": -24}, "neck": {"angleDeg": -18.5}, "torso": {"angleDeg": -3.5}, "hand-l": {"angleDeg": -7.5}},
    None,
]
ANSWER = [
    None,
    {"neck": {"angleDeg": -7}, "head": {"angleDeg": -8}, "torso": {"angleDeg": -1}},
    {
        "torso": {"angleDeg": -1},
        "neck": {"angleDeg": -4},
        "head": {"angleDeg": 0},
        "arm-r": {"angleDeg": 38, "abductionDeg": -5, "twistDeg": -24},
        "forearm-r": {"angleDeg": 132, "twistDeg": 30},
        "hand-r": {"angleDeg": 18, "abductionDeg": 6, "twistDeg": -4},
        "phone": {"angleDeg": -38},
        "arm-l": {"angleDeg": 16, "abductionDeg": 10, "twistDeg": 64},
        "forearm-l": {"angleDeg": 96, "twistDeg": -50},
        "hand-l": {"angleDeg": 3, "abductionDeg": -9, "twistDeg": 0},
    },
    None,
    None,
]


def fix_rig(asset):
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    for bid, _parent, x, y, z, angle, length in rig.BONES:
        bone = bones[bid]
        bone["x"], bone["y"], bone["angleDeg"], bone["lengthM"] = x, y, angle, length
        if z:
            bone["z"] = z
        if bid in MASS_CENTRES:
            bone["mass"]["comX"], bone["mass"]["comY"] = MASS_CENTRES[bid]
    bones["phone"]["joint"] = {"type": "revolute", "minDeg": -75, "maxDeg": 30}
    bones["torso"]["title"] = {"en": "Torso with sweatshirt", "de": "Rumpf mit Sweatshirt"}


def fix_anchors(asset):
    anchors = {a["id"]: a for a in asset["anchors"]}
    for side in ("l", "r"):
        heel, ball = anchors["heel-" + side], anchors["ball-" + side]
        heel["x"], heel["y"] = 0.015, -0.0904
        heel["footprintLengthM"], heel["footprintWidthM"] = 0.07, 0.062
        ball["x"], ball["y"] = 0.17251, 0
        ball["footprintLengthM"], ball["footprintWidthM"] = 0.09, 0.092
    anchors["mouth"]["x"], anchors["mouth"]["y"] = -0.031, -0.101
    anchors["nostrils"]["x"], anchors["nostrils"]["y"] = -0.011, -0.107
    anchors["crown"]["x"], anchors["crown"]["y"] = 0.149, 0
    anchors["ear"]["x"], anchors["ear"]["y"], anchors["ear"]["z"] = 0.021, 0.01, -0.079
    anchors["phone-cog"]["x"], anchors["phone-cog"]["y"] = 0.0735, 0
    grip = anchors["grip-r"]
    grip["x"], grip["y"], grip["z"] = GRIP
    grip["bone"] = "hand-r"


GRIP = (0.1237, 0.0346, 0.0366)


def fix_poses(asset):
    for pose in asset["poses"]:
        bones = {}
        for side in ("l", "r"):
            for part, deg in LEGS.items():
                bones[part + "-" + side] = {"angleDeg": deg}
        bones.update(json.loads(json.dumps(POSES[pose["id"]])))
        pose["bones"] = bones
        pose["description"] = DESCRIPTIONS[pose["id"]]
    for clip, overrides in (("scroll-and-sway", SCROLL), ("answer-call", ANSWER)):
        keys = next(c for c in asset["clips"] if c["id"] == clip)["keys"]
        if len(keys) != len(overrides):
            raise SystemExit("clip %s has %d keys" % (clip, len(keys)))
        for key, bones in zip(keys, overrides):
            key.pop("bones", None)
            if bones:
                key["bones"] = json.loads(json.dumps(bones))
    answer = next(c for c in asset["clips"] if c["id"] == "answer-call")
    answer["description"] = {
        "en": "Non-looping, four seconds: the head lifts, the left hand lets go, the right arm raises the phone from the chest to the right ear, turning it in the hand, while the left forearm folds across the waist; the legs stand still.",
        "de": "Ohne Wiederholung, vier Sekunden: Der Kopf hebt sich, die linke Hand lässt los, der rechte Arm führt das Telefon von der Brust zum rechten Ohr und dreht es dabei in der Hand, während sich der linke Unterarm quer vor den Bauch legt; die Beine stehen still.",
    }


def fix_provenance(asset):
    refs = asset["provenance"]["references"]
    have = {r["id"] for r in refs}
    new = [
        {"id": "gordon-2014", "citation": "Gordon, C. C. et al. (2014). 2012 Anthropometric Survey of U.S. Army Personnel: Methods and Summary Statistics (ANSUR II). Natick/TR-15/007; breadths, depths, circumferences and landmark heights of adults."},
        {"id": "din-33402-2-2005", "citation": "DIN 33402-2:2005 Ergonomie, Körpermaße des Menschen, Teil 2: Werte; percentiles of body dimensions of adults in Germany."},
        {"id": "lee-2015", "citation": "Lee, S., Kang, H. and Shin, G. (2015). Head flexion angle while using a smartphone. Ergonomics 58(2), 220-226; head flexion of 33 to 45 degrees while texting, larger standing than sitting."},
    ]
    refs.extend(r for r in new if r["id"] not in have)
    notes = asset["provenance"]["notes"]
    add_en = " Round 3 (2026-11-20): the skeleton stands on a 20 mm sneaker sole (stature 1.69 m barefoot, 1.71 m shod), the trunk bone runs along the spine from the lumbosacral joint 35 mm behind the hip joints to the seventh cervical vertebra (0.855 of stature) and the head joint is the atlanto-occipital joint; the arms are posed in three dimensions (spread, inward rotation of the upper arm, supination of the forearm) so that both hands meet on the phone; the phone (147 x 71.5 x 7.8 mm) pivots in the right hand; the head is bowed 43 degrees against the upright trunk (Lee et al. 2015); breadths, depths and girths follow ANSUR II and DIN 33402-2 for a slim adult of 59 kg; the drawings of every scale are the orthographic views of that scale's handcrafted 3D body."
    add_de = " Runde 3 (20.11.2026): Das Skelett steht auf einer 20 mm starken Sneakersohle (Körperhöhe barfuß 1,69 m, mit Schuhen 1,71 m), der Rumpfknochen folgt der Wirbelsäule vom Lumbosakralgelenk 35 mm hinter den Hüftgelenken bis zum siebten Halswirbel (0,855 der Körperhöhe) und das Kopfgelenk ist das Atlantookzipitalgelenk; die Arme sind räumlich gestellt (Abspreizung, Einwärtsdrehung des Oberarms, Auswärtsdrehung des Unterarms), sodass sich beide Hände am Telefon treffen; das Telefon (147 x 71,5 x 7,8 mm) dreht sich in der rechten Hand; der Kopf ist gegenüber dem aufrechten Rumpf 43 Grad gesenkt (Lee et al. 2015); Breiten, Tiefen und Umfänge folgen ANSUR II und DIN 33402-2 für eine schlanke erwachsene Person von 59 kg; die Zeichnungen jedes Maßstabs sind die orthogonalen Ansichten des handgefertigten 3D-Körpers dieses Maßstabs."
    if "Round 3" not in notes["en"]:
        notes["en"] += add_en
        notes["de"] += add_de
    asset["structure"]["assumptions"][1] = {
        "en": "Contacts are heel and ball of each foot; both feet stand flat at hip-joint width in every named pose.",
        "de": "Kontakte sind Ferse und Ballen jedes Fußes; beide Füße stehen in jeder benannten Pose flach in Hüftgelenkbreite.",
    }
    asset["summary"]["en"] = asset["summary"]["en"].replace("a ponytail, weight on one leg.", "a ponytail, feet at hip width.")
    asset["summary"]["de"] = asset["summary"]["de"].replace("Pferdeschwanz, Gewicht auf einem Bein.", "Pferdeschwanz, Füße hüftbreit.")


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


DIMENSIONS = {"heightM": 1.68, "widthM": 0.5, "depthM": 0.48}


def fix_dimensions(asset):
    """📏 The envelope of the texting pose, read from the 1:50 body (see the report): height to the crown of the bowed head, width from the hood on the back to the upper edge of the phone, depth from elbow to elbow."""
    asset["dimensions"].update(DIMENSIONS)


DECISIONS = {}
if os.path.exists(os.path.join(T, "r3_standing_phone_user_texts.py")):
    import r3_standing_phone_user_texts

    DECISIONS = r3_standing_phone_user_texts.DECISIONS


def fix_decisions(asset):
    for scale, texts in DECISIONS.items():
        asset["representations"][scale]["decisions"].update(texts)


def fixed(asset):
    out = copy.deepcopy(asset)
    fix_rig(out)
    fix_anchors(out)
    fix_poses(out)
    fix_provenance(out)
    fix_shapes(out)
    fix_decisions(out)
    fix_dimensions(out)
    return out


CHANGED = ("summary", "provenance", "dimensions", "rig", "anchors", "representations", "poses", "clips", "structure")


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
    at = mtext.index('"id": "standing-phone-user"')
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
