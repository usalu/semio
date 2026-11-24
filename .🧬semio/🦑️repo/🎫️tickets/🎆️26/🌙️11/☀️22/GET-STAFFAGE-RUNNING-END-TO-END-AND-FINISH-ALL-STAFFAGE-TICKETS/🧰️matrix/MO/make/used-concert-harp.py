"""🪕️ Writes the repair ops of `used-concert-harp`: playing tilts the harp 18 degrees back onto the shoulder. Its front and plan shapes were flat sheets, so at 1:500 the plan kept the column 250 mm and the front the neck 151 mm and the base 102 mm from where the side elevation shows them, the shoulder contact lay 123 mm off the soundbox in the plan and the rear feet 71 mm off the base in the front. The parts become solids in the front and the plan of every band: the column (foot, shaft and crown) a body of revolution about its bone, the base, its moulding, the pedal blocks, the neck and its two action plates their side elevation outline swept over their width. The soundbox stays three flat outlines: its back is a tapering half cone that no single solid hint describes, and a solid soundboard beside a flat back and foot uncovered slivers of them below the print minimum in the played pose (0.198 mm at 1:200)."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, has, prism, scales, turned, write
a = asset("used-concert-harp")
ops = []
for k in scales(a):
    for view in ("front", "plan"):
        for sid in ("column", "column-foot", "column-shaft", "crown"):
            if has(a, k, view, sid):
                ops.append(turned(a, k, view, sid, "column"))
        for sid in ("base", "base-moulding", "pedals-left", "pedals-right", "neck", "plate-left", "plate-right"):
            if has(a, k, view, sid) and has(a, k, "elevation", sid):
                ops.append(prism(a, k, view, sid))
write("used-concert-harp", ops)
