"""🏗️ PW-03 base step: builds a species module (rig from the 3D skeleton, masses, thermal model, poses, clips, loads) with the NW-V1 assembly and writes the base document and the 3D skeleton sidecar for the volume step.
Usage: python pw03_build.py <species module> <out dir>
@see ./pw03_tree.py
@see ./pw03_asset.ts"""
import importlib, json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from pw03_tree import finish_doc
from common import assemble

mod = importlib.import_module(sys.argv[1])
S = mod.build()
doc, areas = assemble(S)
side = finish_doc(S, S.tree, doc)
side["barkAreas"] = S.tree.areas()
os.makedirs(sys.argv[2], exist_ok=True)
json.dump(doc, open(os.path.join(sys.argv[2], "base.json"), "w"), ensure_ascii=False)
json.dump(side, open(os.path.join(sys.argv[2], "skeleton.json"), "w"), ensure_ascii=False)
a = doc["asset"]
print("[DEBUG]", S.id, "mass", a["dimensions"]["massKg"], "bones", len(a["rig"]["bones"]), "bark", {k: round(v, 2) for k, v in side["barkAreas"].items()})
for b in a["rig"]["bones"]:
    print("[DEBUG]  ", b["id"], b["parent"], "x", b["x"], "y", b["y"], "z", b.get("z", 0), "ang", b["angleDeg"], "L", b["lengthM"], "kg", b.get("mass", {}).get("kg"))
