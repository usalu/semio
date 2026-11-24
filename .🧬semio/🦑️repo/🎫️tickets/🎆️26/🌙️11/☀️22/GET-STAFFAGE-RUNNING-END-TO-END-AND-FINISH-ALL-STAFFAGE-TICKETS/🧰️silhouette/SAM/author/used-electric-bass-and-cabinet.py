"""🎸️ Repair ops of `used-electric-bass-and-cabinet`: the eight string solids of 1:50 (0.6 to 1.3 mm radius, role `texture`, drawn in no view) lacked the `texture` flag, so they counted as body outline and closed a pocket over the bass body that the drawing does not have (1:50 elevation 16.0 mm). They are flagged as the surface texture they are."""
from oplib import load, write

asset = load("used-electric-bass-and-cabinet")
write("used-electric-bass-and-cabinet", [{"op": "solid-set", "scale": "50", "id": s["id"], "set": {"texture": True}} for s in asset["volumes"]["50"]["body"] if s["id"].startswith("string-")])
