"""Is the template released after freezing? Usage: xl4_free.py [storeys]"""
import gc
import sys
import weakref

import xl4_common as C

visuals = C.load()
visuals._FROZEN_SHAPES = 0
storeys = int(sys.argv[1]) if len(sys.argv) > 1 else 2
fit = C.program_fit()
refs = []
original = visuals._Frozen.__init__


def spy(self, drawing):
    refs.append(weakref.ref(drawing))
    original(self, drawing)


visuals._Frozen.__init__ = spy
draw = visuals.NakaginDrawing()
before = C.rss()
drawing = draw.tower_axon(**fit, types=False, towers=(0,), where=lambda s, n=storeys: s.tower == 0 and s.storey < n, core=False, podium=False, capital=False)
print(f"[free] template alive before gc: {refs[0]() is not None} rss={C.rss() / 1e6:.0f}MB", flush=True)
collected = gc.collect()
print(f"[free] gc collected {collected}; template alive after gc: {refs[0]() is not None} rss={C.rss() / 1e6:.0f}MB", flush=True)
if refs[0]() is not None:
    holders = gc.get_referrers(refs[0]())
    print("[free] referrers:", [type(h).__name__ + ":" + (",".join(list(h)[:6]) if isinstance(h, dict) else "") for h in holders][:8], flush=True)
