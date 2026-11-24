"""🎹️ Writes the repair ops of `used-upright-piano`: at 1:50 the front draws the dark backs of the three pedal slots of the knee board (the volume is closed behind them by bottom beam and soundboard; the open slots stood 81 mm from the body)."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, shape, write
board = shape(asset("used-upright-piano"), 50, "front", "knee-board")
write("used-upright-piano", [{"op": "shape-add", "scale": "50", "view": "front", "after": "knee-board", "shape": {"id": f"pedal-slot-{name}", "bone": "case", "role": "detail", "z": 4.1, "filled": True, "tone": 0.1, "kind": "rect", "x": x, "y": 0.173, "w": 0.04, "h": 0.055, "region": board["region"], "material": board["material"]}} for name, x in (("soft", -0.14), ("mute", -0.02), ("sustain", 0.1))])
