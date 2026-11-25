"""🔌️ Ops writer of `diesel-generator`: the rain cap of the exhaust stack is a dome that swings up by 90 degrees at rated load. In the front of 1:50 it was a flat outline and in the front of 1:100 a revolved solid without `mirror`, which draws the meridian only: seen along its axis the open cap showed its edge and the front ended 150 mm below the side elevation. Both become the side elevation profile revolved about the cap axis as a whole body (`mirror`), a disc when the cap stands open. The three hinge knuckles of the radiator flap lie on the hinge axis and look the same at every flap angle: in front and plan they are bound to the container, which carries the hinge: `python3 make/diesel-generator.py` writes `ops/diesel-generator.json`."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("diesel-generator")
DOME = {"kind": "revolved", "axisBone": "cap-flap", "axis": "v", "axisX": -0.15, "axisZ": -0.9, "mirror": True}
ops = [op(scale, "front", "rain-cap", solid_of(shape(a, scale, "front", "rain-cap"), shape(a, scale, "elevation", "rain-cap"), DOME)) for scale in ("50", "100")]
ops += rebind(a, {"outlet-flap-knuckle"}, "container")
write("diesel-generator", ops)
