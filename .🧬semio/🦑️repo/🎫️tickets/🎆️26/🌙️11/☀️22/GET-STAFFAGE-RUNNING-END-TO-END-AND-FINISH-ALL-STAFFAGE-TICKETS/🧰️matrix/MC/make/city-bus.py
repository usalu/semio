"""🚌️ Ops writer of `city-bus`: the front tyres (and the twin rear tyres) of the front view were flat sheets on the spinning wheel bones and turned over with the wheels, so the tyre contacts lay up to 411 mm off them while driving; they are bound to the axle bones, which carry the wheels and do not spin: `python3 make/city-bus.py` writes `ops/city-bus.json`."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("city-bus")
write("city-bus", rebind(a, {"tyre-fl", "tyre-fr"}, "axle-f") + rebind(a, {"tyre-rl", "tyre-rr"}, "axle-r"))
