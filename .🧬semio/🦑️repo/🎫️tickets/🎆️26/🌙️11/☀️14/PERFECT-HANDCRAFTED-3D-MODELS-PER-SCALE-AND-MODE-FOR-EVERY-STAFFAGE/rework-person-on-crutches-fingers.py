"""🖐️ Finger bones of `humans/person-on-crutches`: three phalanges per finger and two of the thumb on both hands alike.

Every bone runs along the phalanx of the closed grip as it is modelled at 1:50 (joint to joint), its joint axis is the normal of the curl
(the bone before it crossed with the bone itself), so a positive angle closes the finger further and a negative one opens the grip.
The phalanx solids of body and structure at 1:50 are re-bound to their bones; the coarser scales keep the merged fingers on the hand.

Usage: python3 rework-person-on-crutches-fingers.py [--publish]

See ./rig-lateral-rest.md
"""
import importlib.util
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("girdle", os.path.join(HERE, "rework-person-on-crutches-girdle.py"))
girdle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(girdle)
Vec, Mat, solve, dump, splice = girdle.Vec, girdle.Mat, girdle.solve, girdle.dump, girdle.splice

FINGERS = [("index", "1", "index finger", "Zeigefinger"), ("middle", "2", "middle finger", "Mittelfinger"), ("ring", "3", "ring finger", "Ringfinger"), ("little", "4", "little finger", "Kleinfinger")]
PHALANGES = [("proximal", "", "proximal phalanx", "Grundglied"), ("middle", "-mid", "middle phalanx", "Mittelglied"), ("distal", "-tip", "distal phalanx", "Endglied")]
SIDES = {"l": ("Left", "Linker"), "r": ("Right", "Rechter")}
IDENTITY = (Vec([0, 0, 0]), Mat([[1, 0, 0], [0, 1, 0], [0, 0, 1]]))


def cross(a, b):
    return Vec([a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]])


def unit(a):
    return a * (1 / math.sqrt(sum(c * c for c in a)))


def chain(side, body, rows):
    """🔗 The bones of one hand as (bone, body solid id) from the capsules of the closed grip, in the frame of the hand."""
    out = []
    for name, number, en, de in FINGERS:
        ids = ["hand-%s-metacarpal-%s" % (side, number)] + ["hand-%s-finger-%s%s" % (side, number, suffix) for _, suffix, _, _ in PHALANGES]
        out += links(side, body, ids, [(name + "-" + part, "%s %s, %s" % (SIDES[side][0], en, pen), "%s %s, %s" % (SIDES[side][1], de, pde)) for part, _, pen, pde in PHALANGES], (-85, 15))
    ids = ["hand-%s-thumb" % side, "hand-%s-thumb-tip" % side]
    out += links(side, body, [None] + ids, [("thumb-proximal", "%s thumb, proximal phalanx" % SIDES[side][0], "%s Daumen, Grundglied" % SIDES[side][1]), ("thumb-distal", "%s thumb, distal phalanx" % SIDES[side][0], "%s Daumen, Endglied" % SIDES[side][1])], (-40, 20))
    return out


def links(side, body, ids, names, limits):
    out = []
    parent, parent_id = IDENTITY, "hand-" + side
    axes = [Vec(body[i]["p1"]) - Vec(body[i]["p0"]) if i else None for i in ids]
    for index, (name, en, de) in enumerate(names, start=1):
        solid = body[ids[index]]
        along = axes[index]
        before, after = (axes[index - 1], along) if axes[index - 1] is not None else (along, axes[index + 1])
        u = unit(along)
        w = cross(before, after)
        w = unit(w - u * sum(a * b for a, b in zip(w, u)))
        v = cross(w, u)
        basis = Mat([[u[i], v[i], w[i]] for i in range(3)])
        length = girdle.r5(math.sqrt(sum(c * c for c in along)))
        bone = {"id": "%s-%s" % (name, side), "parent": parent_id, "title": {"en": en, "de": de}, **solve(parent, Vec(solid["p0"]), basis), "lengthM": length, "joint": {"type": "revolute", "minDeg": limits[0], "maxDeg": limits[1]}}
        out.append((bone, ids[index], length))
        parent, parent_id = (Vec(solid["p0"]), basis), bone["id"]
    return out


def main():
    text = open(girdle.CATALOG, encoding="utf-8").read()
    doc = json.loads(text)
    asset = doc["asset"]
    source = json.load(open(girdle.SOURCE, encoding="utf-8"))
    asset["volumes"] = source["volumes"]
    bones = asset["rig"]["bones"]
    if any(b["id"] == "index-proximal-l" for b in bones):
        raise SystemExit("the rig already has finger bones: this chisel ran")
    fine = asset["volumes"]["50"]
    body = {s["id"]: s for s in fine["body"]}
    structure = {s["id"]: s for s in fine["structure"]}
    for side in SIDES:
        made = chain(side, body, fine)
        at = next(i for i, b in enumerate(bones) if b["id"] == "hand-" + side) + 1
        bones[at:at] = [bone for bone, _, _ in made]
        for bone, solid_id, length in made:
            for solid in (body[solid_id], structure[solid_id.replace("hand-%s-" % side, "bones-%s-" % side)]):
                solid.update({"bone": bone["id"], "p0": [0, 0, 0], "p1": [length, 0, 0]})
        print("[DEBUG]", side, [(b["id"], b["x"], b["y"], b["z"], b["angleDeg"], b["abductionDeg"], b["twistDeg"], b["lengthM"]) for b, _, _ in made[:3]])
    fine["decisions"].append({
        "en": "Hands: three phalanges per finger and two of the thumb are bones of the rig on both hands; each runs along its phalanx of the closed grip and turns about the normal of the curl, so the grip can open; the phalanx solids of body and skeleton are bound to them.",
        "de": "Hände: Drei Fingerglieder je Finger und zwei des Daumens sind an beiden Händen Knochen des Rigs; jeder liegt in seinem Glied des geschlossenen Griffs und dreht um die Normale der Krümmung, sodass der Griff sich öffnen kann; die Gliedkörper von Körper und Skelett sind an sie gebunden.",
    })
    os.makedirs(os.path.dirname(girdle.CANDIDATE), exist_ok=True)
    open(girdle.CANDIDATE, "w", encoding="utf-8").write(dump(doc, 0) + "\n")
    print("candidate", girdle.CANDIDATE)
    if "--publish" in sys.argv:
        if open(girdle.CATALOG, encoding="utf-8").read() != text:
            raise SystemExit("catalog asset changed meanwhile; retry")
        result = splice(splice(text, "rig", asset["rig"]), "volumes", asset["volumes"])
        if json.loads(result) != doc:
            raise SystemExit("splice did not round-trip")
        open(girdle.SOURCE, "w", encoding="utf-8").write(dump({"volumes": asset["volumes"]}, 0) + "\n")
        open(girdle.CATALOG, "w", encoding="utf-8").write(result)
        print("published")


if __name__ == "__main__":
    main()
