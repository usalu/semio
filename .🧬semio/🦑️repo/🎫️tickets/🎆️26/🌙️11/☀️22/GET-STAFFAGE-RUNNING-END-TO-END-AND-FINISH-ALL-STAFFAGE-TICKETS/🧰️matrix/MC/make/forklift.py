"""🏗️ Ops writer of `forklift`: the front and plan tyres of 1:50, 1:100 and 1:200 were flat sheets on the spinning wheel bones and turned over with the wheels (front 116 mm above the ground while driving, the tyre contacts up to 212 mm off them); they are bound to the axle bones, which carry the wheels and do not spin. The tyres of 1:500 are cylinders already and stay on their wheels: `python3 make/forklift.py` writes `ops/forklift.json`."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("forklift")
write("forklift", rebind(a, {"tyre-f-l", "tyre-f-r"}, "axle-f") + rebind(a, {"tyre-r-l", "tyre-r-r"}, "axle-r"))
