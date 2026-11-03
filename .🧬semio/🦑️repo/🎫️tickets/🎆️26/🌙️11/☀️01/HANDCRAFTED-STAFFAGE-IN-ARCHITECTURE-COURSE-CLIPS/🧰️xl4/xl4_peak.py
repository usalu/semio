"""Peak working set of one tower_axon at the program-clip fit. Usage: xl4_peak.py full|N [copy]  (env XL4_GUARD_GB, default 6)

full builds the whole building (populates the ordering cache under the fixed digest); N builds tower 0 up to N storeys."""
import gc
import sys

import xl4_common as C

visuals = C.load()
C.phases(visuals)
C.watch()
mode = sys.argv[1] if len(sys.argv) > 1 else "full"
fit = C.program_fit()
draw = visuals.NakaginDrawing()
kwargs = {} if mode == "full" else dict(towers=(0,), where=lambda s, n=int(mode): s.tower == 0 and s.storey < n, core=False, podium=False, capital=False)
C.say(f"start mode={mode} scale={fit['scale']:.4f}")
drawing = draw.tower_axon(**fit, types=False, **kwargs)
shapes = sum(1 for m in drawing.get_family() if len(m.points))
C.say(f"built shapes={shapes} peak={C.peak() / 1024**2:.0f}MB")
gc.collect()
C.say(f"after gc shapes={shapes}")
print(f"[xl4] RESULT mode={mode} shapes={shapes} peak_mb={C.peak() / 1024**2:.0f}", flush=True)
