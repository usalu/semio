"""🚽️ Writes the repair ops of `used-wc-pan-cistern`: in use the lid stands open at 90.5 degrees. Its front and plan shapes were flat, so the front kept the open lid a line (239 mm at 1:200) and the whole front 119 mm lower than the side elevation. The lid and the seat ring, which turns on the same hinge, become solids in the front and the plan of every band: the rounded outline the plan draws, swept along the bone normal over the thickness the side elevation draws; the two skirts of 1:50, which the plan does not draw under the lid and the ring, take the same outline in the front."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, has, scales, slab, write
a = asset("used-wc-pan-cistern")
HINGE = lambda x: x + 0.145
ops = []
for k in scales(a):
    for main, skirt in ((("lid", "lid-100", "lid-200"), "lid-skirt"), (("seat-ring", "seat-ring-100", "seat-ring-200"), "seat-skirt")):
        sid = next(s for s in main if has(a, k, "plan", s))
        if has(a, k, "front", skirt):
            ops += slab(a, k, skirt, HINGE, source=sid)
        ops += slab(a, k, sid, HINGE)
write("used-wc-pan-cistern", ops)
