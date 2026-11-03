"""Working set with several live full-building drawings at the program-clip fit. Usage: xl4_multi.py COUNT  (env XL4_FROZEN, XL4_GUARD_GB)"""
import gc
import sys

import xl4_common as C

visuals = C.load()
C.phases(visuals)
C.watch()
count = int(sys.argv[1]) if len(sys.argv) > 1 else 3
fit = C.program_fit()
draw = visuals.NakaginDrawing()
live = []
for index in range(count):
    live.append(draw.tower_axon(**fit, types=False, tint=(lambda k, i=index: "#c0392b" if hash(k) % 3 == i % 3 else None)))
    gc.collect()
    C.say(f"live drawings={len(live)} frozen={type(next(iter(visuals._TOWERS.values()))).__name__}")
live.pop(0)
gc.collect()
live.append(draw.tower_axon(**fit, types=False))
C.say(f"replaced one: live={len(live)}")
keys = sorted(live[0].capsules)[:10]
copies = draw.tower_capsules(keys, None, **fit)
C.say(f"10 capsule copies; live={len(live)}")
print(f"[xl4] RESULT multi count={count} peak_mb={C.peak() / 1024**2:.0f}", flush=True)
