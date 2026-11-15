"""Standing-adult: bind ears and nose to the face skin (1:50), ears, nose and soles at 1:100."""
import sys; sys.path.insert(0, ".")
from author_33_lib import *
slug = "standing-adult"
d = load(slug); v50 = d["volumes"]["50"]; v100 = d["volumes"]["100"]
b50, b100 = v50["body"], v100["body"]
def ref(v, sid, region): 
    if not any(t["id"] == f"th-{sid}" for t in v["thermal"]): v["thermal"].append({"id": f"th-{sid}", "of": sid, "region": region})
for sid in ("ear-l", "ear-r", "nose"): ref(v50, sid, "face-skin")
for sid in ("ear-l", "ear-r", "nose"):
    if not any(s["id"] == sid for s in b100): b100.append(copy.deepcopy(by(b50, sid)))
    ref(v100, sid, "face-skin")
for sid in ("sole-l", "sole-r", "toe-sole-l", "toe-sole-r"):
    if any(s["id"] == sid for s in b100): continue
    s = copy.deepcopy(by(b50, sid))
    for q in s["sections"]: q["ry"] = max(q["ry"], 0.0078)
    b100.append(s)
save(slug, d)
