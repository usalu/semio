"""🛵️ Ops writer of `motor-scooter`: the contact `stand-foot`, one anchor in the centre plane for the two feet of the centre stand, which stand 143 mm to the left and to the right of it (117 to 123 mm off every drawn part of the stand in front and plan), becomes one contact per foot (D45), with one touch-down event per foot. The rear tyre of the front of 1:50, the only wheel part that was a flat sheet on its spinning wheel bone (the rear contact 64 mm off it in the ride loop), is bound to the swing arm that carries the wheel: `python3 make/motor-scooter.py` writes `ops/motor-scooter.json`."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("motor-scooter")
write("motor-scooter", stand_feet(a, "stand-foot", "stand-foot", 0.215, 0.143, 0.06, 0.052) + rebind(a, {"tyre-r"}, "swing", only=("front",)))
