"""🐈 domestic-cat: the sitting pose on the inclined scapulae, and the decisions texts of the girdle, neck and 1:200 skeleton.

A scapula that rests 15.7 degrees out of the sagittal plane carries the forelimb inward when it swings back: with the authored 28 degrees the forepaws met 13 mm beside the median plane and ran into each other. The scapulae swing 16 degrees now and humerus, forearm, metacarpus and paw are solved in the sagittal plane so that wrist, paw joint and paw tip stay where they were in side elevation; the paws stand 21 mm beside the plane, side by side.

One-shot: `python3 rework-domestic-cat-sitting.py --apply` (catalog `poses`, volumes source `decisions`); publish the source with `volumes-insert.py`.
"""
import copy
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


G = load("rework-domestic-cat-girdle")
K = G.K
SCAPULA_DEG = -16
GIRDLE = {
    "en": " Girdles (rig of 2026-11-22): each scapula is a bone of its own that rests 15.7 degrees out of the sagittal plane, from its dorsal border 22 mm beside the median plane to the shoulder joint 40 mm beside it, and the humerus hangs from its tip; each hip bone (os coxae) is a fixed bone from the sacroiliac joint 19 mm beside the plane to the hip joint 30 mm beside it, and the femur hangs from its tip. Ilium and ischium are bound to the hip bones, the scapula solids lie in the plane of their bone.",
    "de": " Gürtel (Rig vom 22.11.2026): jedes Schulterblatt ist ein eigener Knochen, der 15,7 Grad aus der Sagittalebene geneigt ruht, vom Rückenrand 22 mm neben der Medianebene zum Schultergelenk 40 mm daneben, und der Oberarm hängt an seiner Spitze; jedes Hüftbein (Os coxae) ist ein fester Knochen vom Kreuzdarmbeingelenk 19 mm neben der Ebene zum Hüftgelenk 30 mm daneben, und der Oberschenkel hängt an seiner Spitze. Darmbein und Sitzbein sind an die Hüftbeine gebunden, die Körper der Schulterblätter liegen in der Ebene ihres Knochens.",
}
COARSE_GIRDLE = {
    "en": " Girdles (rig of 2026-11-22): the scapulae rest 15.7 degrees out of the sagittal plane on bones of their own and carry the forelimbs; the two hip bones are fixed bones from the sacroiliac joints to the hip joints, and at this scale the one pelvis block is their member solid as well (`members`), because a 42 mm hip bone is not a part of its own here.",
    "de": " Gürtel (Rig vom 22.11.2026): die Schulterblätter ruhen 15,7 Grad aus der Sagittalebene geneigt auf eigenen Knochen und tragen die Vorderläufe; die beiden Hüftbeine sind feste Knochen von den Kreuzdarmbeingelenken zu den Hüftgelenken, und in diesem Maßstab ist der eine Beckenblock auch ihr Stabkörper (`members`), weil ein 42 mm langes Hüftbein hier kein eigenes Teil ist.",
}
TRUE_200 = {
    "en": " Femur, tibia, humerus, radius with ulna, the cervical and lumbar spine and the five caudal rods keep their true sections (shafts 8 to 10 mm, ends 14 to 19 mm, caudal rods 12 to 4 mm) and are declared slender, instead of the 30 mm sausages of the earlier list; foot bones, scapulae, thoracic spine and joint markers stay at the 30 mm feature.",
    "de": " Oberschenkel, Schienbein, Oberarm, Speiche mit Elle, Hals- und Lendenwirbelsäule und die fünf Schwanzstäbe behalten ihre wahren Querschnitte (Schäfte 8 bis 10 mm, Enden 14 bis 19 mm, Schwanzstäbe 12 bis 4 mm) und sind als schlank erklärt, statt der 30-mm-Würste der früheren Liste; Fußknochen, Schulterblätter, Brustwirbelsäule und Gelenkmarken bleiben beim Merkmal von 30 mm.",
}
NECK = {
    "en": " The neck is slimmed to life: 74 by 88 mm at the chest, 62 by 74 mm in the middle (girth 0.21 m) and 60 by 68 mm behind the skull, narrower than the 88 mm head; before it was an 84 by 90 mm ball.",
    "de": " Der Hals ist auf das wirkliche Maß verschlankt: 74 auf 88 mm an der Brust, 62 auf 74 mm in der Mitte (Umfang 0,21 m) und 60 auf 68 mm hinter dem Schädel, schmaler als der 88 mm breite Kopf; vorher war er eine Kugel von 84 auf 90 mm.",
}


def nearest(candidates, cost):
    return min(candidates, key=cost)


def sit(asset, report):
    bones = asset["rig"]["bones"]
    by_id = {bone["id"]: bone for bone in bones}
    pose = next(pose for pose in asset["poses"] if pose["id"] == "sitting-tail-wrapped")
    offsets = pose["bones"]
    if offsets["scapula-ff"]["angleDeg"] == SCAPULA_DEG:
        return False
    was = K.frames(bones, copy.deepcopy(offsets))
    wrist, knuckle, toe = was["metacarpus-ff"][0][:2], was["paw-ff"][0][:2], G.end_of(was, by_id["paw-ff"])[:2]
    offsets["scapula-ff"] = {"angleDeg": SCAPULA_DEG}
    trial = lambda extra: K.frames(bones, {**offsets, **{bone: {"angleDeg": angle} for bone, angle in extra.items()}})
    h, f = offsets["humerus-ff"]["angleDeg"], offsets["forearm-ff"]["angleDeg"]
    for span, step in ((20, 1.0), (1, 0.05), (0.05, 0.0025)):
        count = round(span / step)
        h, f = nearest([(h + i * step, f + j * step) for i in range(-count, count + 1) for j in range(-count, count + 1)], lambda pair: math.dist(trial({"humerus-ff": pair[0], "forearm-ff": pair[1]})["metacarpus-ff"][0][:2], wrist))
    fixed = {"humerus-ff": round(h, 2), "forearm-ff": round(f, 2)}
    m = nearest([offsets["metacarpus-ff"]["angleDeg"] + i * 0.01 for i in range(-2000, 2001)], lambda angle: math.dist(trial({**fixed, "metacarpus-ff": angle})["paw-ff"][0][:2], knuckle))
    fixed["metacarpus-ff"] = round(m, 2)
    p = nearest([offsets["paw-ff"]["angleDeg"] + i * 0.01 for i in range(-2000, 2001)], lambda angle: math.dist(G.end_of(trial({**fixed, "paw-ff": angle}), by_id["paw-ff"])[:2], toe))
    fixed["paw-ff"] = round(p, 2)
    for bone, angle in {"scapula-ff": SCAPULA_DEG, **fixed}.items():
        offsets[bone] = {"angleDeg": angle}
        offsets[bone[:-2] + "fn"] = {"angleDeg": angle}
    now = K.frames(bones, offsets)
    if report:
        print("[DEBUG] sitting forelimb", {bone: offsets[bone] for bone in ("scapula-ff", *fixed)})
        for bone in ("metacarpus-ff", "paw-ff", "paw-fn"):
            print("[DEBUG]", bone, "origin before", [round(c, 4) for c in was[bone][0]], "now", [round(c, 4) for c in now[bone][0]])
        print("[DEBUG] paw tip before", [round(c, 4) for c in G.end_of(was, by_id["paw-ff"])], "now", [round(c, 4) for c in G.end_of(now, by_id["paw-ff"])])
    return True


def texts(volumes):
    for scale, volume in volumes.items():
        body = next(text for text in volume["decisions"] if text["en"].startswith("Body"))
        structure = next(text for text in volume["decisions"] if text["en"].startswith(("Structure", "Skeleton", "Forces")))
        if "slimmed to life" in body["en"]:
            raise SystemExit("decisions already extended")
        for language in ("en", "de"):
            body[language] += NECK[language]
            structure[language] += (GIRDLE if scale in ("50", "100") else COARSE_GIRDLE)[language] + (TRUE_200[language] if scale == "200" else "")


def main():
    original = text = open(G.CATALOG, encoding="utf8").read()
    document = json.loads(text)
    before = copy.deepcopy(document["asset"]["poses"])
    if not sit(document["asset"], True):
        raise SystemExit("already applied")
    source = json.load(open(G.SOURCE, encoding="utf8"))
    for volume in source["volumes"].values():
        print("[DEBUG] decisions start with", [entry["en"][:24] for entry in volume["decisions"]])
    texts(source["volumes"])
    if "--apply" not in sys.argv:
        return
    text = K.splice(text, "poses", before, document["asset"]["poses"])
    if json.loads(text) != document or open(G.CATALOG, encoding="utf8").read() != original:
        raise SystemExit("not written: splice did not round-trip or the catalog changed")
    open(G.CATALOG, "w", encoding="utf8").write(text)
    json.dump(source, open(G.SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(G.SOURCE, "a", encoding="utf8").write("\n")
    print("applied")


if __name__ == "__main__":
    main()
