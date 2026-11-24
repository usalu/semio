"""🥁️ Writes the first repair ops of `used-drum-kit` (ops/used-drum-kit.json) from the document of its generator before this repair: the skirts of the five case lids.

A skirt is a band of 4 mm PE, 56 mm (kick case) or less high, around the lid. It was a `slender` ring: the guard draws a slender ring as a stroke of the height of the band along its centre circle, which seen from the side is a line with square ends, so the band stood half its height past the case on both sides (33.8 mm) and closed the gap between two cases from above (103 mm at 1:100). A band is no slender member, and with 4 mm on 56 mm no thin sheet either: at 1:50 and 1:100 the skirt is modelled at the smallest feature (7.5 and 15 mm), grown inwards into the wall of the case, so its outline is the true one and every view compares it as it is."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, write

SLUG = "used-drum-kit"
a = asset(SLUG)
ops = []
for scale, feature in (("50", 0.0075), ("100", 0.015)):
    for s in a["volumes"][scale]["body"]:
        if not (s["id"].endswith("-skirt") and s.get("slender")):
            continue
        outer = max(p[1] for p in s["profile"])
        inner = min(p[1] for p in s["profile"])
        ops.append({"op": "solid-set", "scale": scale, "id": s["id"], "set": {"profile": [[p[0], round(outer - feature, 6) if p[1] == inner else p[1]] for p in s["profile"]]}})
        ops.append({"op": "set", "path": ["volumes", scale, "body", {"id": s["id"]}, "slender"]})
write(SLUG, ops)
print(f"{SLUG}: {len(ops)} ops")
