"""🧊️ Repair ops of `used-fridge-freezer`: at the back the mains lead runs from the taped cable coil into the compressor recess and closes the 4.5 mm slot between coil and cabinet above the plug; the side elevation of 1:50 drew the coil but not the lead, so the slot stayed open to the recess in the drawing only (1:50 elevation 16.9 mm). The lead is drawn as the stroke of its solid `cable-lead`, like the coil."""
from oplib import write

lead = {"id": "cable-lead", "bone": "cabinet", "role": "detail", "z": 2.95, "filled": False, "tone": 0.2, "kind": "path", "d": "M 0.172 0.274 L 0.092 0.205"}
write("used-fridge-freezer", [{"op": "shape-add", "scale": "50", "view": "elevation", "after": "cable-coil", "shape": lead}])
