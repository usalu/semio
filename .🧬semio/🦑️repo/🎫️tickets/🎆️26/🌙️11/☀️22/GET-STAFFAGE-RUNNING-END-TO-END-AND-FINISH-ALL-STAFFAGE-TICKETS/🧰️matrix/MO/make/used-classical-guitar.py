"""🎸️ Writes the repair ops of `used-classical-guitar`: the gig bag lies flat on the floor in the rest pose, is carried upright (82 degrees) and tipped (45 degrees), and its lid opens 105 degrees. Its front and plan shapes were flat sheets, so the front kept a bag 0.15 m high where the side elevation shows it standing 1.06 m (520 mm apart), and the six contact anchors of the bag lay up to 520 mm off it. Every panel of the bag and of the lid becomes one solid in the front and the plan of every band: its plan outline (the guitar-shaped outline of the bag) swept along the bone normal over the thickness the side elevation draws."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, has, scales, slab, write
a = asset("used-classical-guitar")
BAG = lambda x: -0.41889 - x
LID = lambda x: x + 1.46889
ops = []
for k in scales(a):
    for sid, along in (("back-panel", BAG), ("gusset", BAG), ("bag-body", BAG), ("strap-r", BAG), ("strap-l", BAG), ("handle", BAG), ("lid-panel", LID), ("pocket", LID)):
        if has(a, k, "plan", sid) and has(a, k, "front", sid) and has(a, k, "elevation", sid):
            ops += slab(a, k, sid, along)
write("used-classical-guitar", ops)
