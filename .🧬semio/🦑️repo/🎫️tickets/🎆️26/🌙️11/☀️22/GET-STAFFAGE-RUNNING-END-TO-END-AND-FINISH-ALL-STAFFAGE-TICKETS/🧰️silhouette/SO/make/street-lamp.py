"""💡️ Writes the repair ops of `street-lamp`: at 1:50 the six cooling fins of the lamp head are a `texture` solid of the volume (4 mm plates, thinner than the smallest feature), so their strokes in the three views are drawn in the `texture` role too (the pen is unchanged, 0.13 mm) and no longer count as body 15 to 18 mm outside the housing."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, write
a = asset("street-lamp")
write("street-lamp", [{"op": "shape-set", "scale": "50", "view": v, "id": s["id"], "set": {"role": "texture"}} for v in ("elevation", "front", "plan") for s in a["representations"]["50"]["geometry"][v] if s["id"].startswith("head-fin-")])
