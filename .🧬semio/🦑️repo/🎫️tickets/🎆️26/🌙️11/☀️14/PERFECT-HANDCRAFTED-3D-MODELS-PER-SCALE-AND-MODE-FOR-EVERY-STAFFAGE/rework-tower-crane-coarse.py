"""🔩 Tower crane, second round pass 1: parts of the coarse scales that were still 1:50 parts below the feature.

1:200 and 1:500: the two 16 mm cheek plates of the trolley sheave block become one closed block 1.0 x 0.3 x 0.112 m
(the outer faces of the cheeks); at 1:500 the two 60 mm sheaves and their 58 mm axles inside it are merged into it.
1:500: heel and tip cross tubes of the jib at 75 mm, the pin joints of jib and counter-jib at 75 mm.
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "machinery", "tower-crane.json")
BLOCK = {"id": "trolley-sheave-block", "kind": "box", "bone": "trolley", "material": "machinery", "role": "body", "tone": 0.35, "size": [1, 0.3, 0.112], "transform": {"translation": [0, -0.37, 0]}}
MERGED = {"200": {"trolley-cheek-r", "trolley-cheek-l"}, "500": {"trolley-cheek-r", "trolley-cheek-l", "trolley-sheave-1", "trolley-sheave-2", "trolley-sheave-1-axle", "trolley-sheave-2-axle"}}


def main():
    src = json.load(open(SOURCE, encoding="utf-8"))
    for sc, gone in MERGED.items():
        v = src["volumes"][sc]
        if any(s["id"] == BLOCK["id"] for s in v["body"]):
            continue
        at = next(i for i, s in enumerate(v["body"]) if s["id"] in gone)
        v["body"] = [s for s in v["body"] if s["id"] not in gone]
        v["body"].insert(at, dict(BLOCK))
        for lst, ref in (("thermal", {"id": "th-trolley-sheave-block", "of": BLOCK["id"], "region": "trolley"}), ("structure", {"id": "member-trolley-sheave-block", "of": BLOCK["id"], "member": "trolley", "part": "member"})):
            old = [s for s in v[lst] if s.get("of") in gone]
            print(sc, lst, "replaces", [s["id"] for s in old])
            if old:
                at = v[lst].index(old[0])
                v[lst] = [s for s in v[lst] if s.get("of") not in gone]
                keep = dict(ref)
                keep.update({k: old[0][k] for k in ("region", "member", "part") if k in old[0]})
                v[lst].insert(at, keep)
    v = src["volumes"]["500"]
    for s in v["body"]:
        if s["id"] in ("jib-heel-cross", "jib-tip-cross"):
            s["profile"] = [[0, 0.0375], [s["profile"][1][0], 0.0375]]
    for s in v["structure"]:
        if s["id"] in ("joint-jib-heel", "joint-jib-heel-opposite"):
            s["profile"] = [[0, 0.0375], [0.075, 0.0375]]
            if s["id"] == "joint-jib-heel-opposite":
                s["origin"] = [0.8, -0.74, 0.465]
        if s["id"] in ("joint-counter-heel", "joint-counter-heel-right"):
            s["profile"] = [[0, 0.0375], [0.12, 0.0375]]
    json.dump(src, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)


if __name__ == "__main__":
    main()
