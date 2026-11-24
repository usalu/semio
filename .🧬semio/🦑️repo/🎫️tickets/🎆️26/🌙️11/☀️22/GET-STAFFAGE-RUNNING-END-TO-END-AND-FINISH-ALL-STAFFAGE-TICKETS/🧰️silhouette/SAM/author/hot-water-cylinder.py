"""🛢️ Repair ops of `hot-water-cylinder`: the jacket is 0.600 m across, the thermometer bezel and the inspection cover stand 8 and 10 mm proud of it at the front (body of 1:50 and 1:100: 0.6096 m deep, drawn so in the plan), so the declared depth of 0.60 m missed the body of 1:50 by 9.6 mm. The depth is declared at 0.61 m (1:200 and 1:500 leave the cover out: 0.599 m, 11 mm off, inside their 30 and 75 mm)."""
from oplib import write

write("hot-water-cylinder", [{"op": "dims", "set": {"depthM": 0.61}}])
