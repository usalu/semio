"""🔭️ Survey of an asset of batch MC before its repair: the bones a pose or a clip turns (angle, twist, spread; the root too), and what `solidify` would do and leave flat on those bones and the bones below them: `python3 survey.py <asset.json>`."""
import json, sys
from mc import *
a = json.load(open(sys.argv[1]))["asset"]
turned = {}
def note(b, off, where):
    for k in ("angleDeg", "twistDeg", "abductionDeg"):
        if abs(off.get(k, 0)) > 1e-9 and abs(off[k]) > abs(turned.get((b, k), (0,))[0]):
            turned[(b, k)] = (off[k], where)
for p in a["poses"]:
    for b, off in p.get("bones", {}).items():
        note(b, off, p["id"])
    note("<root>", p.get("root", {}), p["id"])
for c in a.get("clips", []):
    for t in c.get("tracks", []):
        for k in t.get("keys", []):
            if isinstance(k.get("value"), (int, float)):
                note(t.get("bone", "<root>"), {t.get("channel", t.get("property", "")): k["value"]}, "clip " + c["id"])
            for b, off in (k.get("bones") or {}).items():
                note(b, off, "clip " + c["id"])
print(a["id"], "turned:", {f"{b}.{k}": v for (b, k), v in turned.items()})
if a.get("clips"):
    print("  clip tracks sample:", json.dumps(a["clips"][0].get("tracks", [])[:1])[:300])
roots = {b for (b, k) in turned}
bones = {b["id"] for b in a["rig"]["bones"]} if "<root>" in roots else subtree(a, roots)
ops = solidify(a, bones)
print("  solidify would write", len(ops), "ops on", sorted(bones))
