"""Does the heap reuse memory after a freeze and between thaws? Usage: xl4_reuse.py [storeys]  (frozen threshold 0)"""
import gc
import sys

import numpy as np

import xl4_common as C

visuals = C.load()
visuals._FROZEN_SHAPES = 0
storeys = int(sys.argv[1]) if len(sys.argv) > 1 else 4
fit = C.program_fit()
draw = visuals.NakaginDrawing()
options = dict(towers=(0,), where=lambda s, n=storeys: s.tower == 0 and s.storey < n, core=False, podium=False, capital=False)
C.say("start")
first = draw.tower_axon(**fit, types=False, **options)
gc.collect()
C.say("after first thaw (template frozen and freed)")
del first
gc.collect()
C.say("after dropping first thaw")
second = draw.tower_axon(**fit, types=False, **options)
gc.collect()
C.say("after second thaw")
third = draw.tower_axon(**fit, types=False, **options)
gc.collect()
C.say("after third thaw (two live)")
print("[reuse] opaque", len(visuals._OPAQUE_VISIBILITY), "orders", len(visuals._OCCLUSION_ORDERS), "blocks", sys.getallocatedblocks(), flush=True)
del second, third
gc.collect()
C.say("after dropping both")
print("[reuse] blocks", sys.getallocatedblocks(), flush=True)
