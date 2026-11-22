"""🖐️ Jogger, second round pass 2: finger bones on both hands and the shirt hem on the pelvis.

Usage: python3 rework-jogger-extremities.py <candidate out> [--publish]
Reads the catalog document as it is. The hand bone ends at the knuckles (wrist to the third metacarpal head) and a `fingers`
bone runs from there to the tips of the loosely curled fingers, flexing about the knuckle line; the finger solids are re-expressed
in it without moving. The part of the shirt below L5/S1 becomes `shirt-hem` on the pelvis with a domed top inside the lumbar
piece, so a forward fold neither lifts the hem off the seat nor opens the back. All numbers by hand.
"""
import copy
import importlib.util
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))


def module(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(HERE, name + ".py"))
    m = importlib.util.module_from_spec(spec)
    sys.argv, keep = [sys.argv[0]], sys.argv
    spec.loader.exec_module(m)
    sys.argv = keep
    return m


lib = module("rework-02-lib")
ASSET = "humans/jogger"
KNUCKLE = [0.1, -0.004, -0.005]
FINGERTIP = [0.138, -0.004, -0.05]
HAND_LENGTH = 0.1
FINGER_LIMITS = {"type": "revolute", "minDeg": -50, "maxDeg": 40}
LUMBOSACRAL = [0.1034, 0.032]
DOME_HEIGHTS = [0.012, 0.028, 0.044, 0.058, 0.07, 0.078, 0.083]
DOME_INSET = 0.003
DOME_WIDTH_SPAN = 0.1
DOME_FRONT_RISE = 0.9
OVERLAP = 0.005
SIDES = {"l": ("Left", "Linke", 1), "r": ("Right", "Rechte", -1)}
DECISIONS = {
    "body": {
        "en": " The shirt is cut at T12/L1 and at L5/S1: chest piece, lumbar piece and a hem on the pelvis whose domed top lies inside the lumbar piece, so the trunk folds on its vertebral joints without a crack and the hem stays on the seat.",
        "de": " Das Shirt ist bei Th12/L1 und bei L5/S1 geteilt: Brustteil, Lendenteil und ein Saum am Becken, dessen gewölbtes oberes Ende im Lendenteil liegt; so beugt sich der Rumpf in seinen Wirbelgelenken ohne Spalt, und der Saum bleibt am Gesäss.",
    },
    "coarse": {
        "en": " The trunk hangs on the lumbar and the thoracic spine, the arms on the clavicles and the thighs on the hip bones; at this scale the shirt is one solid on the ribcage.",
        "de": " Der Rumpf hängt an Lenden- und Brustwirbelsäule, die Arme an den Schlüsselbeinen und die Oberschenkel an den Hüftbeinen; in diesem Massstab ist das Shirt ein Körper am Brustkorb.",
    },
    "thermal": {
        "en": " Shirt fabric and the air under it follow the three shirt pieces of the body (chest, lumbar, hem on the pelvis) with the same layers.",
        "de": " Shirtstoff und die Luft darunter folgen den drei Shirtteilen des Körpers (Brust, Lende, Saum am Becken) mit denselben Schichten.",
    },
    "structure": {
        "en": " The column stands on the lumbar and thoracic bones of the rig behind the ribcage middle, the clavicles are the clavicle bones from the sternoclavicular joint to the acromion, each hip bone carries the load from the sacroiliac joint to the hip socket, and the phalanges hang on the finger bones at the knuckles.",
        "de": " Die Wirbelsäule steht auf dem Lenden- und dem Brustknochen des Rigs hinter der Brustkorbmitte, die Schlüsselbeine sind die Schlüsselbeinknochen vom Sternoklavikulargelenk zum Akromion, jedes Hüftbein trägt die Last vom Iliosakralgelenk zur Hüftpfanne, und die Fingerglieder hängen an den Fingerknochen in den Grundgelenken.",
    },
    "drawing": {
        "en": " The shirt outline is cut at the waist joint so that it bends with the lumbar and the thoracic spine; the hand is drawn as one curled form on the hand bone.",
        "de": " Der Shirtumriss ist am Taillengelenk geteilt, damit er sich mit Lenden- und Brustwirbelsäule beugt; die Hand ist als eine gekrümmte Form am Handknochen gezeichnet.",
    },
}


def r5(x):
    return round(x, 5)


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def dot(a, b):
    return sum(a[i] * b[i] for i in range(3))


def unit(a):
    n = math.sqrt(dot(a, a))
    return [x / n for x in a]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def finger_frame():
    """📐️ The left finger frame in the hand frame: u from the knuckles to the fingertips, w the knuckle line (hand v), so a positive angle curls."""
    u = unit(sub(FINGERTIP, KNUCKLE))
    w = [0, 1, 0]
    return [u, cross(w, u), w]


def placement(basis, at):
    """🧮️ Rest placement of a child frame given in its parent's frame: R = Rw(angle) · Rv(abduction) · Ru(twist)."""
    u, w = basis[0], basis[2]
    b = math.asin(max(-1, min(1, u[2])))
    a = math.atan2(u[1], u[0])
    v0 = [-math.sin(a), math.cos(a), 0]
    w0 = [-math.cos(a) * math.sin(b), -math.sin(a) * math.sin(b), math.cos(b)]
    t = math.atan2(dot(w, v0), dot(w, w0))
    return {"x": r5(at[0]), "y": r5(at[1]), "z": r5(at[2]), "angleDeg": round(math.degrees(a), 4), "abductionDeg": round(math.degrees(b), 4), "twistDeg": round(math.degrees(t), 4)}


def into_fingers(p, sign):
    """🔗️ A point of the hand frame in the finger frame of that side (the right hand is the mirror of the left)."""
    q = sub([p[0], p[1], p[2] * sign], KNUCKLE)
    x, y, z = (dot(axis, q) for axis in finger_frame())
    return [r5(x), r5(y), r5(z * sign)]


def is_finger(solid):
    return solid.get("kind") == "capsule" and (solid["id"].startswith("finger-") or solid["id"].startswith("sk-phalanx-"))


def fingers(asset):
    """🖐️ Adds `fingers-l/r`, shortens the hand bones to the knuckles and re-binds the finger solids."""
    bones = asset["rig"]["bones"]
    assert not any(b["id"].startswith("fingers-") for b in bones), "finger bones already placed"
    place = placement(finger_frame(), KNUCKLE)
    length = r5(math.dist(FINGERTIP, KNUCKLE))
    rebuilt = []
    for b in bones:
        rebuilt.append(b)
        if b["id"].startswith("hand-"):
            side = b["id"][-1]
            en, de, sign = SIDES[side]
            b["lengthM"] = HAND_LENGTH
            b["mass"]["comX"] = r5(0.79 * HAND_LENGTH)
            own = place if sign > 0 else {**place, "z": -place["z"], "abductionDeg": -place["abductionDeg"], "twistDeg": -place["twistDeg"]}
            rebuilt.append({"id": f"fingers-{side}", "parent": b["id"], "title": {"en": f"{en} fingers", "de": f"{de} Finger"}, **own, "lengthM": length, "joint": dict(FINGER_LIMITS)})
    asset["rig"]["bones"] = rebuilt
    for scale, v in asset["volumes"].items():
        for name in ("body", "thermal", "structure"):
            out = []
            for s in v[name]:
                out.append(s)
                hand = s.get("bone", "")
                if not hand.startswith("hand-"):
                    continue
                side = hand[-1]
                bone = f"fingers-{side}"
                if is_finger(s):
                    s["bone"] = bone
                    s["p0"], s["p1"] = into_fingers(s["p0"], SIDES[side][2]), into_fingers(s["p1"], SIDES[side][2])
                    if "member" in s:
                        s["member"] = bone
                elif name == "structure" and s["id"] == f"joint-hand-{side}":
                    marker = copy.deepcopy(s)
                    marker.update(id=f"joint-{bone}", bone=bone, member=bone)
                    out.append(marker)
            v[name] = out
        if not any(s.get("member", "").startswith("fingers-") and s.get("part") == "member" for s in v["structure"]):
            for s in v["structure"]:
                if s["id"] in ("sk-fingers-l", "sk-fingers-r", "sk-hand-l", "sk-hand-r") and s.get("part") == "member":
                    side = s["id"][-1]
                    if not any(o["id"] == f"sk-fingers-{side}" for o in v["structure"]) or s["id"].startswith("sk-fingers-"):
                        s["members"] = [f"fingers-{side}"]
    return place, length


def lerp(a, b, t):
    return a + (b - a) * t


def section_at(sections, u):
    """📏️ The section of a loft at the station u (linear between its neighbours)."""
    for a, b in zip(sections, sections[1:]):
        if a["at"][0] <= u <= b["at"][0]:
            t = (u - a["at"][0]) / (b["at"][0] - a["at"][0])
            s = copy.deepcopy(a)
            s["at"] = [u, lerp(a["at"][1], b["at"][1], t), lerp(a["at"][2], b["at"][2], t)]
            s["rx"], s["ry"] = lerp(a["rx"], b["rx"], t), lerp(a["ry"], b["ry"], t)
            return s
    raise AssertionError("station outside the loft")


def on_pelvis(section):
    s = copy.deepcopy(section)
    s["at"] = [r5(s["at"][0] + LUMBOSACRAL[0]), r5(s["at"][1] + LUMBOSACRAL[1]), s["at"][2]]
    s["rx"], s["ry"] = r5(s["rx"]), r5(s["ry"])
    return s


def hem_of(waist):
    """👕️ Cuts a lumbar shirt loft at L5/S1: returns the hem on the pelvis with a dome about the joint; the lumbar piece keeps a 5 mm lap."""
    sections = waist["sections"]
    joint = section_at(sections, 0)
    hem = copy.deepcopy(waist)
    hem["id"] = waist["id"].replace("shirt-waist", "shirt-hem")
    hem["bone"] = "pelvis"
    rows = [on_pelvis(s) for s in sections if s["at"][0] < 0] + [on_pelvis(joint)]
    back = joint["at"][1] + joint["ry"] - DOME_INSET
    front = joint["at"][1] - joint["ry"] + DOME_INSET
    for h in DOME_HEIGHTS:
        b = math.sqrt(max(back * back - h * h, 0))
        f = front + DOME_FRONT_RISE * h
        s = copy.deepcopy(joint)
        s["at"] = [h, (b + f) / 2, joint["at"][2]]
        s["rx"] = (joint["rx"] - DOME_INSET) * math.sqrt(1 - (h / DOME_WIDTH_SPAN) ** 2)
        s["ry"] = (b - f) / 2
        rows.append(on_pelvis(s))
    hem["sections"] = rows
    hem["capStart"], hem["capEnd"] = True, True
    waist["sections"] = [section_at(sections, -OVERLAP)] + [s for s in sections if s["at"][0] > -OVERLAP]
    waist["sections"][0]["at"] = [r5(x) for x in waist["sections"][0]["at"]]
    waist["sections"][0]["rx"], waist["sections"][0]["ry"] = r5(waist["sections"][0]["rx"]), r5(waist["sections"][0]["ry"])
    return hem


def hem(asset):
    """👕️ Moves the shirt below L5/S1 onto the pelvis in every list that has a lumbar shirt piece."""
    done = []
    for scale, v in asset["volumes"].items():
        for name in ("body", "thermal"):
            out = []
            for s in v[name]:
                out.append(s)
                if s.get("bone") == "torso" and "shirt-waist" in s["id"] and s.get("kind") == "loft" and s["sections"][0]["at"][0] < 0:
                    out.append(hem_of(s))
                    done.append(f"{scale}:{name}:{out[-1]['id']}")
                elif s.get("of") == "shirt-waist":
                    out.append({**copy.deepcopy(s), "id": s["id"].replace("shirt-waist", "shirt-hem"), "of": "shirt-hem"})
            v[name] = out
    return done


def decisions(asset):
    """📝️ States the trunk bones, the girdles, the finger bones and the shirt cut in the decisions of every scale."""
    for scale, v in asset["volumes"].items():
        fine = scale in ("50", "100")
        for i, key in enumerate(("body" if fine else "coarse", "thermal" if fine else None, "structure")):
            if key:
                for lang in ("en", "de"):
                    v["decisions"][i][lang] += DECISIONS[key][lang]
        if fine:
            d = asset["representations"][scale]["decisions"]["silhouette"]
            for lang in ("en", "de"):
                d[lang] += DECISIONS["drawing"][lang]


def main():
    out = sys.argv[1]
    base = lib.load_catalog(ASSET)
    document = copy.deepcopy(base)
    asset = document["asset"]
    place, length = fingers(asset)
    done = hem(asset)
    decisions(asset)
    os.makedirs(os.path.dirname(out), exist_ok=True)
    with open(out, "w", encoding="utf-8") as f:
        json.dump(document, f, ensure_ascii=False, indent=2)
        f.write("\n")
    if "--publish" in sys.argv:
        assert lib.load_catalog(ASSET) == base, "the catalog document changed since it was read"
        lib.save_source(ASSET, {"volumes": asset["volumes"]})
        lib.save_catalog(ASSET, document)
    print(json.dumps({"fingers": place, "lengthM": length, "hem": done}))


if __name__ == "__main__":
    main()
