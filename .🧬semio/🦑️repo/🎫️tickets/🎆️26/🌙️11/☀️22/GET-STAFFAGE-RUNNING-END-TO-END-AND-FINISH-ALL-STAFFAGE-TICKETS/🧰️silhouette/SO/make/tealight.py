"""🕯️ Writes the repair ops of `tealight`: at 1:50 a pen of 0.13 mm is 6.5 mm on a 38 mm object, so the wick is drawn as the filled 2 mm strand of the volume instead of a stroke, and the wax line lies 1.75 mm lower so that its stroke ends at the cup rim instead of standing above it (IoU of the elevation and the front 0.914 before)."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import write
write("tealight", [
    {"op": "shape-set", "scale": "50", "view": "elevation", "id": "wick", "set": {"filled": True, "closed": True, "points": [[0, -0.001], [0.007, -0.001], [0.007, 0.001], [0, 0.001]]}},
    {"op": "shape-set", "scale": "50", "view": "front", "id": "wick", "set": {"filled": True, "closed": True, "points": [[-0.001, 0.0145], [0.001, 0.0145], [0.001, 0.0215], [-0.001, 0.0215]]}},
    {"op": "shape-set", "scale": "50", "view": "elevation", "id": "wax-surface", "set": {"points": [[0.01225, 0.014], [0.01225, -0.014]]}},
    {"op": "shape-set", "scale": "50", "view": "front", "id": "wax-surface", "set": {"points": [[-0.014, 0.01275], [0.014, 0.01275]]}},
])
