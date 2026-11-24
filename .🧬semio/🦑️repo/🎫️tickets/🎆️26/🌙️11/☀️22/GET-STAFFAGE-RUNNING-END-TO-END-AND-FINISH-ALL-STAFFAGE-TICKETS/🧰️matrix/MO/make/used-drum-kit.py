"""🥁️ Writes the repair ops of `used-drum-kit`: setting up turns the bass drum 90 degrees out of its case onto its hoops and swings the two cymbal booms. The band 1:500 drew the bass drum, the boom tubes and the crash cymbal as flat front and plan shapes, so the played kit kept a line where the side elevation shows the drum (297 mm and 241 mm apart), the booms 173 mm short, the crash cymbal 53 mm low, and the two hoop contacts 226 to 297 mm off the drum. They become the bodies of revolution they are in the front and the plan of 1:500: the bass drum and the boom tubes about their bones, the two cymbals about the normal of their bones. At 1:200 the two boom tubes and their joints were flat as well (173 mm in the reported tier): the tubes become bodies of revolution, the joints their side elevation disc swept over their width."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, op, prism, shape, solid_of, turned, write
a = asset("used-drum-kit")
ops = []
for view in ("front", "plan"):
    for sid, bone in (("kick-body", "kick"), ("crash-boom-tube", "crash-boom"), ("ride-boom-tube", "ride-boom")):
        ops.append(turned(a, "500", view, sid, bone))
    for sid in ("crash-cymbal", "ride-cymbal"):
        ops.append(op("500", view, sid, solid_of(shape(a, "500", view, sid), shape(a, "500", "elevation", sid), {"kind": "revolved", "axisBone": sid, "axis": "v", "axisX": 0, "mirror": True})))
    for side in ("crash", "ride"):
        ops.append(turned(a, "200", view, f"{side}-boom-tube", f"{side}-boom"))
        ops.append(prism(a, "200", view, f"{side}-boom-joint"))
write("used-drum-kit", ops)
