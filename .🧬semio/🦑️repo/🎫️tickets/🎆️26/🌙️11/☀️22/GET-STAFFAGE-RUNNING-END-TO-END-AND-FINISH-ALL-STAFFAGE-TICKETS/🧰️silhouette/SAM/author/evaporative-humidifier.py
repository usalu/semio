"""💧️ Repair ops of `evaporative-humidifier`: the casing is 0.400 m deep; at 1:50 the four fasteners of the service cover stand 4 mm proud at the front and the water and drain fittings 3.5 mm at the back (body 0.4075 m), which the declared 0.40 m missed by 7.5000000000000071 mm against 7.5 mm. Declared 0.408 m (1:100 and 1:200 leave the fasteners out: 0.400 m, 8 mm off, inside their 15 and 30 mm)."""
from oplib import write

write("evaporative-humidifier", [{"op": "dims", "set": {"depthM": 0.408}}])
