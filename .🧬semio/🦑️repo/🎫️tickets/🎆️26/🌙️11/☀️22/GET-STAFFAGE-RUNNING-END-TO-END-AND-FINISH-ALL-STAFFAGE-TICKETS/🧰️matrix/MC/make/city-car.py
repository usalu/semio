"""🚗️ Ops writer of `city-car`: the front and plan tyres were flat sheets on the spinning wheel bones and turned over with the wheels (front 166 to 230 mm above the ground while driving); they are bound to the axle bones, which carry the wheels and do not spin: `python3 make/city-car.py` writes `ops/city-car.json`."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("city-car")
write("city-car", rebind(a, {"tyre-fl", "tyre-fr"}, "axle-f") + rebind(a, {"tyre-rl", "tyre-rr"}, "axle-r"))
