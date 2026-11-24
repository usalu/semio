"""🥁️ Writes the repair ops of `used-marching-snare-and-bass-drum`: at 1:500 the four J bars of the carriers get, in the side elevation, a stroke at the body pen (0.13 mm = 65 mm) along the centre line of each slender tube beside the filled band of that width, which stays. The guard widens a slender solid to the stroke it follows, a bent filled band it cannot follow, so the 22 mm tubes stood against 65 mm bands (IoU 0.908). The same strokes at 1:200 (0.18 mm) were tried and made its side elevation worse (64.2 mm against 60.5 mm), so 1:200 is left as it was."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, shape, solid, write
a = asset("used-marching-snare-and-bass-drum")
ops = []
for scale, pen in (("500", 0.13),):
    for bar in ("bass-bar-r", "bass-bar-l", "snare-bar-r", "snare-bar-l"):
        old = shape(a, scale, "elevation", bar)
        line = [[round(s["at"][0], 6), round(s["at"][1], 6)] for s in solid(a, scale, bar)["sections"]]
        ops.append({"op": "shape-add", "scale": scale, "view": "elevation", "after": bar, "shape": {**{k: v for k, v in old.items() if k not in ("points", "closed", "filled", "penMm", "d")}, "id": f"{bar}-axis", "filled": False, "kind": "poly", "points": line, "closed": False, "penMm": pen}})
write("used-marching-snare-and-bass-drum", ops)
