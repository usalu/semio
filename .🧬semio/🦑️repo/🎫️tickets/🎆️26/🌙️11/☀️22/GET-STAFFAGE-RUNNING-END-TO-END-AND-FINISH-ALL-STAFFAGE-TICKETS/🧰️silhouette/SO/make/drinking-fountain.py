"""⛲️ Writes the repair ops of `drinking-fountain`: at 1:100, 1:200 and 1:500 the water jet is a `texture` solid of the volume (thinner than the smallest feature), so its stroke in the elevation and the front is drawn in the `texture` role too and no longer counts as body (at 1:200 the stroke closed the arch of the bubbler over the basin into a filled area)."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import write
write("drinking-fountain", [{"op": "shape-set", "scale": s, "view": v, "id": "jet", "set": {"role": "texture"}} for s in ["100", "200", "500"] for v in ["elevation", "front"]])
