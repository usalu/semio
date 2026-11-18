"""🧬 Generates a trace spec (see humansb-D-lib.sh build) for one asset and scale: per view, per bone with volume solids, the main drawn body shape of the bone is replaced by the traced silhouette of all solids of the bone. python3 humansb-D-gen.py <slug> <scale> [skipBones,..] [tol]"""
import json, sys
slug, scale = sys.argv[1], sys.argv[2]
skip = set((sys.argv[3] if len(sys.argv) > 3 else "").split(",")) - {""}
tol = sys.argv[4] if len(sys.argv) > 4 else "0.002"
d = json.load(open("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/" + slug + "/🔣️.json"))["asset"]
bones = {}
for s in d["volumes"][scale]["body"]:
    if s.get("texture"): continue
    bones.setdefault(s["bone"], []).append(s["id"])
for view in ("elevation", "front", "plan"):
    shapes = d["representations"][scale]["geometry"][view]
    for bone, solids in bones.items():
        if bone in skip: continue
        cand = [s for s in shapes if s.get("bone") == bone and s.get("role") == "body" and s.get("filled", True)]
        if not cand: continue
        pick = next((s for s in cand if s["id"] == bone), None) or min(cand, key=lambda s: s.get("z", 0))
        print(f"T {view} {bone} {','.join(solids)} {pick['id']} ={pick['id']} {pick['id']} {tol}")
