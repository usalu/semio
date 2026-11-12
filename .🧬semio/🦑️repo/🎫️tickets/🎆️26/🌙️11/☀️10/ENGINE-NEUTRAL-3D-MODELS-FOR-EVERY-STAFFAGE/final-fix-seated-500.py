"""seated-office-worker 1:500 front: the arm solids were drawn 0.18 m thick in v (twice the 0.1 m minimum feature and about twice the elevation outline of the same arm), so a raised hand reached 0.06 m higher in the front than in the elevation and in the 3D volume; v half-width 0.05 m (the minimum feature), lateral width 0.1 m about the same centre."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE")
from catalog_edit import S, edit
path = S + "👤️humans/seated-office-worker/🔣️.json"
ARMS = {"upper-arm-l", "upper-arm-r", "forearm-l", "forearm-r", "hand-l", "hand-r"}
def fix(o):
    if o.get("id") in ARMS and o.get("kind") == "poly" and isinstance(o.get("solid"), dict) and o["solid"].get("kind") == "extruded" and o["solid"].get("sweep") == "w" and any(abs(abs(p[1]) - 0.09) < 1e-9 for p in o["points"]):
        o["points"] = [[p[0], round(p[1] * 0.05 / 0.09, 4)] for p in o["points"]]
        c = (o["solid"]["z0"] + o["solid"]["z1"]) / 2
        o["solid"]["z0"], o["solid"]["z1"] = round(c - 0.05, 4), round(c + 0.05, 4)
        return o
    if o.get("id") in ("fist-l", "fist-r") and o.get("kind") == "poly" and isinstance(o.get("solid"), dict) and o["solid"].get("kind") == "revolved" and any(abs(p[1] - 0.09) < 1e-9 for p in o["points"]):
        o["points"] = [[p[0], round(p[1] * 0.05 / 0.09, 4)] for p in o["points"]]
        return o
    return None
print(edit(path, lambda t: '"points"' in t and '"solid"' in t and ('"id": "upper-arm' in t or '"id": "forearm' in t or '"id": "hand-' in t or '"id": "fist-' in t), fix))
