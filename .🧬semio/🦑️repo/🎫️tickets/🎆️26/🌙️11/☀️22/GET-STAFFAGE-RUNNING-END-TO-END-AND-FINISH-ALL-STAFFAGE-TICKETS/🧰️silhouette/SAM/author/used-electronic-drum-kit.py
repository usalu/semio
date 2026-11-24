"""🥁️ Repair ops of `used-electronic-drum-kit`: the rubber rims of the five pads are rings of 21 × 16 mm section declared `slender`; the guard compares a slender solid as a stroke along its centre line, which stood up to 15 mm off the drawn pads (1:50 elevation 20.6 mm, front 16.9 mm). At 1:50 a 16 mm rim is above the smallest feature of 7.5 mm, so the rims are compared as the bodies they are (no `slender` at 1:50)."""
from oplib import load, write

asset = load("used-electronic-drum-kit")
write("used-electronic-drum-kit", [{"op": "set", "path": ["volumes", "50", "body", {"id": s["id"]}, "slender"]} for s in asset["volumes"]["50"]["body"] if s["id"].endswith("-rim")])
