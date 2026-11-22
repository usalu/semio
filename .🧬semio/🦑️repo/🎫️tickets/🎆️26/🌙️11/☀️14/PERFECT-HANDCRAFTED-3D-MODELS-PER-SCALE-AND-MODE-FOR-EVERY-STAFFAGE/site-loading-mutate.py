"""🧬 Writes deliberately broken copies of frozen asset documents, so that the equality of validator findings is proven on documents that do have findings, with measured numbers in them.
Usage: python3 site-loading-mutate.py <frozen dir> <slug>...  — writes `<slug>.json` copies into `<frozen dir>/mutated-a` and `<frozen dir>/mutated-b`.
Mutation a: the declared height grows by 10 % and the width shrinks by 10 % (extent findings carry the measured extents).
Mutation b: every non-root bone of the rig is moved by 7 % of its offset (drawings, volumes, thermal and structural lists disagree with what was authored)."""
import json, os, sys
root, slugs = sys.argv[1], sys.argv[2:]
for kind in ("a", "b"):
    os.makedirs(os.path.join(root, f"mutated-{kind}"), exist_ok=True)
for slug in slugs:
    for kind in ("a", "b"):
        document = json.load(open(os.path.join(root, f"{slug}.json")))
        asset = document["asset"]
        if kind == "a":
            asset["dimensions"]["heightM"] = round(asset["dimensions"]["heightM"] * 1.1, 4)
            asset["dimensions"]["widthM"] = round(asset["dimensions"]["widthM"] * 0.9, 4)
        else:
            for bone in asset["rig"]["bones"]:
                if bone.get("parent") is not None:
                    bone["x"] = round(bone.get("x", 0) * 1.07, 5)
                    bone["y"] = round(bone.get("y", 0) * 1.07, 5)
        json.dump(document, open(os.path.join(root, f"mutated-{kind}", f"{slug}.json"), "w"))
