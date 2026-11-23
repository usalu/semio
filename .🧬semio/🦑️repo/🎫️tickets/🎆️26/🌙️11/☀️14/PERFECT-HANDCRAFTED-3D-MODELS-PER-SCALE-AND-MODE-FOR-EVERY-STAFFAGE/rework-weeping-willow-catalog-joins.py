"""🌿 vegetation/weeping-willow, the published round-3 model (not the rebuild of `rework-weeping-willow-model.py`): its measured `volumes-connected` findings are closed.

Measured 2026-11-23 on the catalog file: 1:200 body, three fringe masses hang free (`fringe-05`, `fringe-06` at least 180 mm, `fringe-04` 73.1 mm from the next solid); 1:500 body, five (`fringe-03` at least 450 mm, `fringe-06` 397.6 mm …) and the three sprays of `tress-c23` 45.7 mm from `inner-16`): the coarser meshes of these scales no longer reach their neighbours. 1:100 structure: the switch members of `curt-e3` and `curt-w3` start on the limb axis, 31.2 / 30.8 mm inside the surface of the limb core, and the two collar joints that tie them to it at 1:50 are missing.
1:200 and 1:500: each of the six fringe masses hangs on a 12 mm withy from the switch of its curtain bone (from the switch station nearest along the bone to the middle of the mass), slender, with a thermal twin in `limb-bark`.
1:500: the sprays of `tress-c23` hang on a withy from the switch of `curt-e4` in the same way.
1:100: the two collar joints `joint-curt-e3` and `joint-curt-w3` of 1:50 are in the list again, 10 mm larger in radius so that they reach the surface of the coarser limb core (5.8 mm short otherwise). The withy of `tress-c23` ends on the vertex of the spray nearest its middle (the middle itself lies outside the thin spray).
One-shot on `volumes/vegetation/weeping-willow.json`; publish with `volumes-insert.py`.
"""
import copy
import json
import os

SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "volumes", "vegetation", "weeping-willow.json")
SENTENCE = {
    "en": "Each of the six fringe masses hangs on a 12 mm withy from the switch of its curtain, because the coarser masses of this scale no longer reach their neighbours.",
    "de": "Jede der sechs Saummassen hängt an einer 12-mm-Rute an der Gerte ihres Vorhangs, weil die gröberen Massen dieses Maßstabs ihre Nachbarn nicht mehr erreichen.",
}


def main():
    """▶️ Adds the withies to the two coarse scales and the two collar joints to 1:100."""
    with open(SOURCE, encoding="utf8") as f:
        document = json.load(f)
    volumes = document["volumes"]
    for scale in ("200", "500"):
        v = volumes[scale]
        body = {solid["id"]: solid for solid in v["body"]}
        for number in range(1, 7):
            fringe = body[f"fringe-{number:02d}"]
            ident = f"fringe-{number:02d}-withy"
            if ident in body:
                continue
            count = len(fringe["vertices"])
            middle = [round(sum(vertex[axis] for vertex in fringe["vertices"]) / count, 4) for axis in range(3)]
            switch = body["switch-" + fringe["bone"].removeprefix("curt-")]
            station = min((section["at"] for section in switch["sections"]), key=lambda at: abs(at[0] - middle[0]))
            withy = {"id": ident, "kind": "capsule", "bone": fringe["bone"], "material": switch["material"], "role": "body", "tone": switch["tone"], "p0": station, "p1": middle, "r0": 0.006, "r1": 0.006, "slender": True}
            v["body"].insert(next(i for i, solid in enumerate(v["body"]) if solid["id"] == fringe["id"]) + 1, withy)
            v["thermal"].append({"id": "th-" + ident, "of": ident, "region": "limb-bark"})
        if SENTENCE not in v["decisions"]:
            v["decisions"].append(SENTENCE)
    coarse = volumes["500"]
    body = {solid["id"]: solid for solid in coarse["body"]}
    if "tress-c23-withy" not in body:
        spray, switch = body["tress-c23-spray-1"], body["switch-e4"]
        count = len(spray["vertices"])
        middle = [round(sum(vertex[axis] for vertex in spray["vertices"]) / count, 4) for axis in range(3)]
        station = min((section["at"] for section in switch["sections"]), key=lambda at: abs(at[0] - middle[0]))
        tip = min(spray["vertices"], key=lambda vertex: sum((vertex[axis] - middle[axis]) ** 2 for axis in range(3)))
        coarse["body"].insert(next(i for i, solid in enumerate(coarse["body"]) if solid["id"] == "tress-c23-spray-1"), {"id": "tress-c23-withy", "kind": "capsule", "bone": spray["bone"], "material": switch["material"], "role": "body", "tone": switch["tone"], "p0": station, "p1": tip, "r0": 0.006, "r1": 0.006, "slender": True})
        coarse["thermal"].append({"id": "th-tress-c23-withy", "of": "tress-c23-withy", "region": "limb-bark"})
    fine = {solid["id"]: solid for solid in volumes["50"]["structure"]}
    medium = volumes["100"]["structure"]
    for ident, after in (("joint-curt-e3", "member-curt-e3"), ("joint-curt-w3", "member-curt-w3")):
        if all(solid["id"] != ident for solid in medium):
            joint = copy.deepcopy(fine[ident])
            joint["radii"] = [round(radius + 0.01, 4) for radius in joint["radii"]]
            medium.insert(next(i for i, solid in enumerate(medium) if solid["id"] == after) + 1, joint)
    with open(SOURCE, "w", encoding="utf8") as f:
        json.dump(document, f, ensure_ascii=False)
        f.write("\n")
    print("applied")


if __name__ == "__main__":
    main()
