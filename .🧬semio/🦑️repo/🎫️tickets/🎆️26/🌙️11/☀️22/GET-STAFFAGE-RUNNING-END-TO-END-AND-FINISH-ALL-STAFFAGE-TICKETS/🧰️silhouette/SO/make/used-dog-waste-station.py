"""🐕️ Writes the repair ops of `used-dog-waste-station`: at 1:50 the mounting plate between post and bin runs up to the top of the bin (0.41 m instead of 0.37 m) in the volume, the side elevation and the front; the 5 mm slot it left between post and bin over the top 40 mm was closed in the body as the guard rasterises it and open in the drawing, 42 mm deep."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import write
write("used-dog-waste-station", [
    {"op": "solid-set", "scale": "50", "id": "bin-mount", "set": {"size": [0.41, 0.005, 0.12], "transform": {"translation": [0.255, 0.0025, 0.0]}}},
    {"op": "shape-set", "scale": "50", "view": "elevation", "id": "bin-mount", "set": {"w": 0.41}},
    {"op": "shape-set", "scale": "50", "view": "front", "id": "bin-mount", "set": {"h": 0.41}},
])
