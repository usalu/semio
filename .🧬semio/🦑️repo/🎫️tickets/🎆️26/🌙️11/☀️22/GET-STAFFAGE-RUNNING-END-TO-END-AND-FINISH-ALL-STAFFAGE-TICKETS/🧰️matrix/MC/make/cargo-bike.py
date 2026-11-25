"""🚲️ Ops writer of `cargo-bike`: the contact `stand-pad`, one anchor in the centre plane for the two feet of the two-leg stand, which stand 205 mm to the left and to the right of it (77 to 162 mm off every drawn part of the stand in front and plan), becomes one contact per foot (D45): `python3 make/cargo-bike.py` writes `ops/cargo-bike.json`."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("cargo-bike")
write("cargo-bike", stand_feet(a, "stand-pad", "stand-pad", 0.235, 0.205, 0.05, 0.052))
