"""🦶 Forefoot of the 1:100 skeleton of `humans/person-on-crutches`: tarsus, three metatarsal rays and three toes instead of two plates.

At 15 mm per feature the five rays of the 61 mm fan cannot stand apart, so the first ray, the central rays (2 and 3) and the lateral rays
(4 and 5) are one rod each with a feature of air between them. Runs once on `volumes/humans/person-on-crutches.json`.

See ./rework-person-on-crutches-skeleton.ts for the five rays of 1:50.
"""
import json
import os

SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "volumes", "humans", "person-on-crutches.json")
RAYS = [(-0.028, 0.16, 0.058), (0.002, 0.162, 0.047), (0.03, 0.146, 0.033)]
BONE = {"material": "bone", "role": "body", "tone": 0.62}


def side_of(solids, side):
    sign = 1 if side == "l" else -1
    foot, toe = "foot-" + side, "toe-" + side
    out = [{"id": "tarsus-" + side, "kind": "ellipsoid", "bone": foot, **BONE, "radii": [0.022, 0.015, 0.027], "transform": {"translation": [0.05, -0.004, 0]}, "member": foot, "part": "member"}]
    for index, (w, head, tip) in enumerate(RAYS):
        out.append({"id": "metatarsal-%d-%s" % (index + 1, side), "kind": "capsule", "bone": foot, **BONE, "p0": [0.062, -0.002, round(sign * w * 0.6, 5)], "p1": [head, 0.034, sign * w], "r0": 0.0075, "r1": 0.0085, "member": foot, "part": "member"})
        start = [round((head - 0.17693) * 0.8436 + 0.034 * 0.537 - 0.002, 5), round((0.17693 - head) * 0.537 + 0.034 * 0.8436, 5), sign * w]
        out.append({"id": "toe-phalanges-%d-%s" % (index + 1, side), "kind": "capsule", "bone": toe, **BONE, "p0": start, "p1": [tip, 0.03, round(sign * w * 1.02, 5)], "r0": 0.0078, "r1": 0.0075, "member": toe, "part": "member"})
    return out


def main():
    document = json.load(open(SOURCE, encoding="utf-8"))
    rows = document["volumes"]["100"]["structure"]
    for side in ("l", "r"):
        at = next(i for i, s in enumerate(rows) if s["id"] == "metatarsals-" + side)
        rows[:] = [s for s in rows if s["id"] not in ("metatarsals-" + side, "phalanges-" + side)]
        rows[at:at] = side_of(rows, side)
    for entry in document["volumes"]["100"]["decisions"]:
        if "Structure" in entry["en"] and "forefoot" not in entry["en"]:
            entry["en"] += " The forefoot is a tarsus with three rays: the first, the central and the lateral metatarsals with their toes, one feature apart."
            entry["de"] += " Der Vorfuß ist eine Fußwurzel mit drei Strahlen: erster, mittlere und seitliche Mittelfußknochen mit ihren Zehen, eine Strichstärke auseinander."
            break
    open(SOURCE, "w", encoding="utf-8").write(json.dumps(document, ensure_ascii=False, indent=1) + "\n")


main()
