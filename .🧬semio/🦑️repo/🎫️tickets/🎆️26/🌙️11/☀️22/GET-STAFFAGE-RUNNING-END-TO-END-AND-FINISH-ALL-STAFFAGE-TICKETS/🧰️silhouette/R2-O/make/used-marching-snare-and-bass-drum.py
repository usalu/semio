"""🥁️ Writes the repair ops of `used-marching-snare-and-bass-drum` of batch R2-O (the ops of batch SO for 1:500 stay in force and run before): at 1:200 the two J bars of the bass drum carrier get, in the side elevation, a stroke of 0.11 mm (22 mm, the tube) along the centre line of each slender tube beside its filled band of that width, which stays. The guard widens a slender solid to the stroke it follows; a bent filled band it cannot follow, so it took the width from its own estimate of the tube (27.6 and 28.9 mm) along its jagged centre line and closed the slit of 9 to 30 mm between bar and drum shell that the drawing shows open (60.5 mm against 60 mm). Batch SO tried the stroke at the body pen of 0.18 mm (36 mm), which closed the slit further."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, shape, solid, write
a = asset("used-marching-snare-and-bass-drum")
ops = []
for bar in ("bass-bar-r", "bass-bar-l"):
    old = shape(a, "200", "elevation", bar)
    line = [[round(s["at"][0], 6), round(s["at"][1], 6)] for s in solid(a, "200", bar)["sections"]]
    ops.append({"op": "shape-add", "scale": "200", "view": "elevation", "after": bar, "shape": {**{k: v for k, v in old.items() if k not in ("points", "closed", "filled", "penMm", "d")}, "id": f"{bar}-axis", "filled": False, "kind": "poly", "points": line, "closed": False, "penMm": 0.11}})
write("used-marching-snare-and-bass-drum", ops)
print(f"{len(ops)} ops")
