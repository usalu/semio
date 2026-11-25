"""🚲️ Ops writer of `city-bicycle`: the contact `stand-foot`, one anchor in the centre plane for the two feet of the two-leg stand, which stand 90 mm to the left and to the right of it (78 mm off every drawn part of the stand in front and plan), becomes one contact per foot (D45), with one touch-down event per foot: `python3 make/city-bicycle.py` writes `ops/city-bicycle.json`."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("city-bicycle")
write("city-bicycle", stand_feet(a, "stand-foot", "stand-foot", 0.232, 0.09, 0.03, 0.022))
