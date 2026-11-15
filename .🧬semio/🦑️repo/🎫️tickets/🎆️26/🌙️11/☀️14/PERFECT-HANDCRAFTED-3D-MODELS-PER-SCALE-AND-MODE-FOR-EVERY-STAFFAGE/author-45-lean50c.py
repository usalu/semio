"""Leaning-person 1:50 step c: prune detail shells, rebuild loft shells, structure, joints."""
from author_33_lib import *
slug = "leaning-person"
DET = {"eye-l","eye-r","brow-l","brow-r","ear-l","ear-r","nose","mouth","thumb-l","thumb-r","fingers-l-0","fingers-l-1","fingers-l-2","fingers-r-0","fingers-r-1","fingers-r-2","hood-string-l","hood-string-r","hoodie-pocket","shoulder-l","shoulder-r","cuff-l","cuff-r","hoodie-band","beanie-rim","phone","phone-screen"}
print(prune_and_rebuild_shells(slug, 50, DET))
d = load(slug); th = d["volumes"]["50"]["thermal"]
th[:] = [t for t in th if t["id"] not in ("th-wall-brick", "th-wall-plaster")]
th.append({"id": "th-wall-brick", "of": "wall", "region": "wall-face", "layer": "brick", "sectionFaces": ["+v"]})
th.append({"id": "th-wall-plaster", "kind": "box", "bone": "wall", "material": "masonry", "role": "ground", "tone": 0.8, "size": [0.015, 1.9, 1.2], "transform": {"translation": [-0.3725, 0.95, 0]}, "sectionFaces": ["+v"], "texture": True, "region": "wall-face", "layer": "plaster"})
save(slug, d)
person_structure(slug, 50)
d = load(slug); d["volumes"]["50"]["structure"] = [t for t in d["volumes"]["50"]["structure"] if t.get("bone") not in ("wall", "wall-x", "wall-frame")]
PADS = {"wall": ([0.045, 1.33, 0], [0.07, 0.16, 0.26]), "wall-x": ([0.11, 0.157, 0], [0.08, 0.12, 0.22]), "wall-frame": ([-0.073, 0.007, 0], [0.07, 0.1, 0.22])}
for bone, (c_, sz) in PADS.items():
    d["volumes"]["50"]["structure"].append({"id": f"member-{bone}", "kind": "box", "bone": bone, "material": "masonry", "role": "body", "tone": 0.5, "size": sz, "transform": {"translation": c_}, "member": bone, "part": "member"})
save(slug, d)
print(bun("author-33-joints.ts", f"humans/{slug}", "--scale", "50", "--write")[-400:])
