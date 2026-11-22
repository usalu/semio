"""🚿 Pass 1 chisel of appliances/shower-head: hose leg on its own bone, face relief, select button, ferrules, elbow support.

Usage: python3 rework-shower-head-pass1.py candidate|publish
Reads the pass-0 volumes in rework-shower-head-pass1-base.json, writes the volumes source and either a
candidate document (🗑️generated/r3-shower-head/🔣️.json) or the root fields of the freshly re-read catalog file.
"""
import copy
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
S = os.path.join(T.split("/.🧬semio/")[0], "🎓️teaching", "🏛️architecture", "🧍️staffage")
CATALOG = os.path.join(S, "🗂️catalog", "🔌️appliances", "shower-head", "🔣️.json")
BASE = os.path.join(T, "rework-shower-head-pass1-base.json")
SOURCE = os.path.join(T, "volumes", "appliances", "shower-head.json")
CANDIDATE = os.path.join(T, "🗑️generated", "r3-shower-head", "🔣️.json")

PIVOT = (0.105, 0.62)
REST_DEG = 72.0
NUT_U = -0.036
ELBOW_ORIGIN = (0.0, 0.04, -0.1)
BIGHT_START = (0.08, -0.24, 0.082)
HOSE_MASS_KG = 0.15
ELBOW_MASS_KG = 0.40
PLUG = 0.75
ACROSS_BONE = [0, -0.70711, 0, 0.70711]
HOSE_R = 0.0065
HOSE_TOP_500 = [[0.09388, 0.54576, 0.1], [0.08449, 0.50017, 0.09967], [0.08342, 0.45389, 0.09867], [0.085, 0.4, 0.097]]
HOSE_RISE_200 = [([0.09697, 0.55527, 0.1], 0.55), ([0.09388, 0.54576, 0.1], 1), ([0.0915, 0.5, 0.0997], 1), ([0.091, 0.45, 0.0987], 1), ([0.091, 0.4, 0.097], 1),
                 ([0.091, 0.309, 0.0935], 1), ([0.091, 0.2, 0.0909], 1), ([0.0905, 0.084, 0.08875], 1), ([0.089, -0.034, 0.0868], 1), ([0.0855, -0.145, 0.0846], 1)]
TEXT_200 = (
    {"en": "The hose is drawn at the feature of 30 mm, as the rail is, and keeps its bight.",
     "de": "Der Schlauch ist wie die Stange im kleinsten Merkmal von 30 mm gezeichnet und behält seinen Bogen."},
    {"en": "The hose is drawn at the feature of 30 mm, as the rail is, and keeps its bight; its rising leg runs 91 mm from the wall, clear of the 30 mm pole, and starts with a rounded end inside the handle rod.",
     "de": "Der Schlauch ist wie die Stange im kleinsten Merkmal von 30 mm gezeichnet und behält seinen Bogen; sein steigender Strang läuft 91 mm vor der Wand, frei vom 30-mm-Stab, und beginnt mit gerundetem Ende im Griffstab."})
STRUCTURE_FIX = {
    "50": {"member-outlet": [[0.0, 0.006], [0.0, 0.0105], [0.042, 0.0105], [0.042, 0.006], [0.0, 0.006]]},
    "100": {"member-shower": [[-0.04, 0.0], [-0.04, 0.008], [0.164, 0.008], [0.164, 0.0]]},
    "200": {"member-shower": [[-0.04, 0.0], [-0.04, 0.004], [0.158, 0.004], [0.158, 0.0]]},
    "500": {"member-shower": [[-0.04, 0.0], [-0.04, 0.004], [0.165, 0.004], [0.165, 0.0]]},
}
TILTS = {"running": -8.0, "eco": -4.0}


def nut(tilt):
    a = math.radians(REST_DEG + tilt)
    return (PIVOT[0] + NUT_U * math.cos(a), PIVOT[1] + NUT_U * math.sin(a))


def world(at):
    return (ELBOW_ORIGIN[0] + at[0], ELBOW_ORIGIN[1] + at[1], ELBOW_ORIGIN[2] + at[2])


def leg(tilt):
    n = nut(tilt)
    b = world(BIGHT_START)
    return n, math.degrees(math.atan2(b[1] - n[1], b[0] - n[0])), math.hypot(b[0] - n[0], b[1] - n[1])


N0, PHI0, LEG0 = leg(0.0)
HOSE_REST_DEG = round(PHI0 - REST_DEG, 3)


def pose_angle(tilt):
    return round(leg(tilt)[1] - PHI0 - tilt, 3)


def to_leg(at):
    w = world(at)
    c, s = math.cos(math.radians(-PHI0)), math.sin(math.radians(-PHI0))
    dx, dy = w[0] - N0[0], w[1] - N0[1]
    return [round(c * dx - s * dy, 5), round(s * dx + c * dy, 5), round(w[2], 5)]


def split(solid, rise_id):
    k = next(i for i, x in enumerate(solid["sections"]) if all(abs(x["at"][j] - BIGHT_START[j]) < 1e-6 for j in range(3)))
    rise = copy.deepcopy(solid)
    rise["id"] = rise_id
    rise["bone"] = "hose"
    rise["sections"] = [dict(x, at=to_leg(x["at"])) for x in solid["sections"][: k + 1]] + [plug(dict(solid["sections"][k + 1], at=to_leg(solid["sections"][k + 1]["at"])))]
    rise["sections"] = [dict(x, rotation=ACROSS_BONE) for x in rise["sections"]]
    fixed = copy.deepcopy(solid)
    fixed["sections"] = [plug(solid["sections"][k - 1])] + solid["sections"][k:]
    return fixed, rise


def plug(section):
    return dict(section, rx=round(section["rx"] * PLUG, 5), ry=round(section["ry"] * PLUG, 5))


def length(sections):
    return sum(math.dist(a["at"], b["at"]) for a, b in zip(sections, sections[1:]))


def revolve(id, bone, material, role, tone, profile, axis, origin, **more):
    return dict({"id": id, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone,
                 "profile": profile, "axis": axis, "origin": origin}, **more)


def insert_after(items, after_id, new):
    i = next(n for n, x in enumerate(items) if x["id"] == after_id)
    items[i + 1:i + 1] = new


BODY_50 = {
    "nut": [revolve("ferrule-shower", "shower", "chrome", "detail", 0.75,
                    [[-0.052, 0.0], [-0.052, 0.0078], [-0.05, 0.0086], [-0.034, 0.0086], [-0.034, 0.0]], [1, 0, 0], [0.0, 0.0, 0.0])],
    "nut-elbow": [revolve("ferrule-elbow", "elbow", "chrome", "detail", 0.75,
                          [[0.038, 0.0], [0.038, 0.0086], [0.054, 0.0086], [0.056, 0.0078], [0.056, 0.0]], [0, -1, 0], [0.04, 0.0, 0.0])],
    "handle": [revolve("select-button", "shower", "abs-chrome", "detail", 0.92,
                       [[0.008, 0.0], [0.008, 0.008], [0.0152, 0.008], [0.016, 0.0066], [0.016, 0.0]], [0, -1, 0], [0.1, 0.0, 0.0])],
    "face": [revolve("face-zone-ring", "head", "abs-chrome", "detail", 0.85,
                     [[0.011, 0.027], [0.0128, 0.0275], [0.0128, 0.0305], [0.011, 0.031]], [1, 0, 0], [0.0, 0.0, 0.0], texture=True),
             revolve("face-centre", "head", "abs-chrome", "detail", 0.85,
                     [[0.011, 0.0], [0.011, 0.0125], [0.0128, 0.0115], [0.0133, 0.0], ], [1, 0, 0], [0.0, 0.0, 0.0], texture=True)],
}
THERMAL_50 = [("th-nut", {"id": "th-ferrule-shower", "of": "ferrule-shower", "region": "handle", "layer": "shell"}),
              ("th-handle", {"id": "th-select-button", "of": "select-button", "region": "handle", "layer": "shell"}),
              ("th-nut-elbow", {"id": "th-ferrule-elbow", "of": "ferrule-elbow", "region": "valve", "layer": "body"}),
              ("th-face", {"id": "th-face-zone-ring", "of": "face-zone-ring", "region": "head", "layer": "shell"}),
              ("th-face", {"id": "th-face-centre", "of": "face-centre", "region": "head", "layer": "shell"})]
SUPPORT_ELBOW = {
    "50": [[0.0, 0.0], [0.0, 0.016], [0.01, 0.0064], [0.01, 0.0]],
    "100": [[0.0, 0.0], [0.0, 0.018], [0.016, 0.0072], [0.016, 0.0]],
    "200": [[0.0, 0.0], [0.0, 0.015], [0.03, 0.006], [0.03, 0.0]],
    "500": [[0.0, 0.0], [0.0, 0.014], [0.024, 0.0056], [0.024, 0.0]],
}
MASS_HOSE = {"50": [0.014, 0.005, 0.005], "100": [0.016, 0.0075, 0.0075], "200": [0.02, 0.015, 0.015]}
MASS_HOLDER = {"50": [0.007, 0.006, 0.006], "100": [0.009, 0.0075, 0.0075]}
DECISIONS = {
    "body": {
        "en": " Pass 1 of the rework: the hose is two lofts that meet in one seam at the top of the bight, each ending in a plug of three quarters of the hose diameter inside the other, the falling leg with the bight on the elbow and the rising leg on its own bone hinged in the conical nut, so that the hose stays centred in the nut when the hand shower is tilted.",
        "de": " Überarbeitung, Durchgang 1: der Schlauch besteht aus zwei Lofts, die sich am oberen Ende des Bogens in einer Fuge treffen und je mit einem Zapfen von drei Vierteln des Schlauchdurchmessers im anderen stecken, der fallende Strang mit dem Bogen am Wandanschluss und der steigende Strang an einem eigenen, in der Konusmutter gelenkig gelagerten Knochen, damit der Schlauch beim Neigen der Handbrause mittig in der Mutter bleibt.",
    },
    "body50": {
        "en": " Added at 7.5 mm: the crimp ferrules of 17 mm at both hose ends, the 16 mm spray selector button on the front of the handle under the disc, and the relief of the spray face (centre cap of 25 mm and the ring of 6 mm between the inner and the outer nozzle zone) as texture.",
        "de": " Für 7,5 mm ergänzt: die Presshülsen von 17 mm an beiden Schlauchenden, die 16 mm grosse Strahlwahltaste vorn am Griff unter der Scheibe und das Relief der Düsenfläche (Mittelkappe von 25 mm und der 6 mm breite Ring zwischen innerer und äusserer Düsenzone) als Textur.",
    },
    "thermal": {
        "en": " Pass 1 of the rework: hose wall and water column follow the split of the hose into the leg on the elbow and the leg on the hose bone, so the warm water stays inside the hose in every pose.",
        "de": " Überarbeitung, Durchgang 1: Schlauchwand und Wassersäule folgen der Teilung des Schlauchs in den Strang am Wandanschluss und den Strang am Schlauchknochen, sodass das warme Wasser in jeder Pose im Schlauch bleibt.",
    },
    "structure": {
        "en": " Pass 1 of the rework: the wall elbow is carried by the wall itself (third support), the rising hose leg hangs as its own member with its mass from the hose nut of the hand shower, and the falling leg hangs from the elbow.",
        "de": " Überarbeitung, Durchgang 1: der Wandanschluss wird von der Wand selbst getragen (drittes Auflager), der steigende Schlauchstrang hängt als eigenes Glied mit seiner Masse an der Schlauchmutter der Handbrause, und der fallende Strang hängt am Wandanschluss.",
    },
}


def build_volumes():
    volumes = json.load(open(BASE))["volumes"]
    lengths = {}
    for scale, volume in volumes.items():
        body, thermal, structure = volume["body"], volume["thermal"], volume["structure"]
        i = next(n for n, x in enumerate(body) if x["id"] == "hose")
        if scale == "200":
            k = next(n for n, x in enumerate(body[i]["sections"]) if all(abs(x["at"][j] - BIGHT_START[j]) < 1e-6 for j in range(3)))
            for solid, radius in ((body[i], 0.015), (next(x for x in thermal if x["id"] == "th-hose"), HOSE_R)):
                solid["sections"][:k] = [{"at": at, "rx": radius * taper, "ry": radius * taper} for at, taper in HOSE_RISE_200]
            for language in ("en", "de"):
                assert TEXT_200[0][language] in volume["decisions"][0][language]
                volume["decisions"][0][language] = volume["decisions"][0][language].replace(TEXT_200[0][language], TEXT_200[1][language])
        if scale == "500":
            body[i]["sections"][0:3] = [{"at": at, "rx": HOSE_R, "ry": HOSE_R} for at in HOSE_TOP_500]
        fixed, rise = split(body[i], "hose-rise")
        lengths[scale] = round(length(body[i]["sections"]), 3)
        body[i:i + 1] = [fixed, rise]
        for n, x in list(enumerate(thermal)):
            if x["id"] in ("th-water-hose", "th-hose") and "sections" in x:
                thermal[n:n + 1] = list(split(x, x["id"] + "-rise"))
        for n, x in list(enumerate(thermal)):
            if x.get("of") == "hose":
                thermal.insert(n + 1, dict(x, id="th-hose-rise", of="hose-rise"))
        if scale == "50":
            for after, new in BODY_50.items():
                insert_after(body, after, new)
            for after, new in THERMAL_50:
                insert_after(thermal, after, [new])
            insert_after(structure, "member-shower", [{"id": "member-lever", "of": "lever", "member": "lever", "part": "member"}])
        for solid in structure:
            if solid["id"] in STRUCTURE_FIX[scale]:
                solid["profile"] = STRUCTURE_FIX[scale][solid["id"]]
        if scale == "50":
            insert_after(structure, "member-shower", [{"id": "member-nut", "of": "nut", "member": "shower", "part": "member"}])
        if scale == "100":
            insert_after(structure, "member-elbow", [revolve("member-outlet", "elbow", "chrome", "body", 0.7, [[0.0, 0.0], [0.0, 0.009], [0.042, 0.009], [0.042, 0.0]],
                                                             [0, -1, 0], [0.04, 0.0, 0.0], member="elbow", part="member")])
        members = [n for n, x in enumerate(structure) if x["part"] == "member"]
        hose_member = [n for n, x in enumerate(structure) if x.get("of") == "hose"]
        at = hose_member[0] if hose_member else members[-1]
        new_members = [{"id": "member-hose-rise", "of": "hose-rise", "member": "hose", "part": "member"}]
        if not hose_member:
            new_members.insert(0, {"id": "member-hose", "of": "hose", "member": "elbow", "part": "member"})
        structure[at + 1:at + 1] = new_members
        insert_after(structure, "support-wall-top", [revolve("support-wall-elbow", "elbow", "chrome", "ground", 0.3, SUPPORT_ELBOW[scale],
                                                             [1, 0, 0], [0.0, 0.0, 0.0], member="elbow", part="support", anchor="wall-elbow")])
        if scale in MASS_HOLDER:
            structure.append({"id": "mass-holder", "kind": "ellipsoid", "bone": "holder", "material": "chrome", "role": "detail", "tone": 0.7,
                              "radii": MASS_HOLDER[scale], "transform": {"translation": [0.015, 0.0, 0.0]}, "member": "holder", "part": "mass"})
        if scale in MASS_HOSE:
            structure.append({"id": "mass-hose", "kind": "ellipsoid", "bone": "hose", "material": "epdm-hose", "role": "detail", "tone": 0.55,
                              "radii": MASS_HOSE[scale], "transform": {"translation": [0.39, 0.0, 0.0]}, "member": "hose", "part": "mass"})
        for n, key in enumerate(("body", "thermal", "structure")):
            for language in ("en", "de"):
                volume["decisions"][n][language] += DECISIONS[key][language]
                if scale == "50" and key == "body":
                    volume["decisions"][n][language] += DECISIONS["body50"][language]
    return volumes, lengths


def apply_root(asset):
    rig = asset["rig"]
    rig["bones"] = [b for b in rig["bones"] if b["id"] != "hose"]
    for bone in rig["bones"]:
        if bone["id"] == "elbow":
            bone["mass"]["kg"] = ELBOW_MASS_KG
            bone["title"] = {"en": "Wall elbow with shut-off valve and falling hose leg", "de": "Wandanschlussbogen mit Absperrventil und fallendem Schlauchstrang"}
    i = next(n for n, b in enumerate(rig["bones"]) if b["id"] == "shower")
    rig["bones"].insert(i + 1, {
        "id": "hose", "parent": "shower", "x": NUT_U, "y": 0, "angleDeg": HOSE_REST_DEG,
        "title": {"en": "Rising hose leg, hinged in the conical nut", "de": "Steigender Schlauchstrang, in der Konusmutter gelenkig"},
        "lengthM": round(LEG0, 3), "joint": {"type": "revolute", "minDeg": -12, "maxDeg": 42},
        "mass": {"kg": HOSE_MASS_KG, "comX": 0.39, "comY": 0, "distribution": "point", "source": "authored-estimate"}})
    anchors = [a for a in asset["anchors"] if a["id"] != "wall-elbow"]
    i = next(n for n, a in enumerate(anchors) if a["id"] == "wall-top")
    anchors.insert(i + 1, {"id": "wall-elbow", "bone": "elbow", "x": 0, "y": 0, "kind": "support",
                           "title": {"en": "Wall elbow screwed into the wall fitting", "de": "Wandanschluss, in die Wandscheibe geschraubt"},
                           "support": "pin", "normalDeg": 0, "footprintLengthM": 0.06, "footprintWidthM": 0.06})
    asset["anchors"] = anchors
    for pose in asset["poses"]:
        if pose["id"] in TILTS:
            pose["bones"]["hose"] = {"angleDeg": pose_angle(TILTS[pose["id"]])}
        if "wall-elbow" not in pose["contacts"]:
            pose["contacts"].append("wall-elbow")
    for clip in asset["clips"]:
        if not any(c["anchor"] == "wall-elbow" for c in clip["contacts"]):
            clip["contacts"].append({"anchor": "wall-elbow", "fromS": 0, "toS": clip["durationS"]})
    structure = asset["structure"]
    structure["supports"] = ["wall-low", "wall-top", "wall-elbow"]
    structure["assumptions"] = [{
        "en": "The rail with holder and hand shower hangs on the lower bracket (pin) and is held at the upper bracket (roller): the pair, 0.686 m apart, resists the overturning moment of the hand shower, whose disc stands 0.2 m from the wall. The wall elbow is screwed into its own wall fitting (pin) and carries the falling hose leg and the bight; the rising hose leg of 0.15 kg hangs from the hose nut of the hand shower.",
        "de": "Die Stange mit Halter und Handbrause hängt am unteren Halter (Festlager) und wird am oberen Halter gehalten (Rollenlager): das Paar, 0,686 m auseinander, nimmt das Kippmoment der Handbrause auf, deren Scheibe 0,2 m vor der Wand steht. Der Wandanschluss ist in seine eigene Wandscheibe geschraubt (Festlager) und trägt den fallenden Schlauchstrang und den Bogen; der steigende Schlauchstrang von 0,15 kg hängt an der Schlauchmutter der Handbrause."}]


def main():
    mode = sys.argv[1]
    volumes, lengths = build_volumes()
    document = json.load(open(CATALOG))
    apply_root(document["asset"])
    print("[DEBUG] hose rest", HOSE_REST_DEG, "leg", round(LEG0, 4), "poses", {k: pose_angle(v) for k, v in TILTS.items()}, "hose lengths", lengths)
    if mode == "candidate":
        document["asset"]["volumes"] = volumes
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w"), ensure_ascii=False, indent=2)
        print("candidate", CANDIDATE)
    elif mode == "publish":
        with open(SOURCE, "w") as f:
            f.write(json.dumps({"volumes": volumes}, ensure_ascii=False, indent=1))
        with open(CATALOG, "w") as f:
            f.write(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
        print("source and catalog root written")


main()
