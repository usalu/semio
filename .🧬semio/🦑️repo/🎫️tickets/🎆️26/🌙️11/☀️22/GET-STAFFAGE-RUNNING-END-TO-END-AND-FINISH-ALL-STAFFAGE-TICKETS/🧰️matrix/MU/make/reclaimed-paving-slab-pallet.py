"""🧱️ Ops writer of `reclaimed-paving-slab-pallet`: the rear slabs twist by 180 degrees about their edge on the way to the bed; front and plan draw them as bodies, the side elevation drew the edge as a flat rectangle that stayed an edge view halfway. The edge takes the body the front draws, the same rectangle at rest: `python3 make/reclaimed-paving-slab-pallet.py` writes its ops."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *
a = asset("reclaimed-paving-slab-pallet")
write("reclaimed-paving-slab-pallet", [{"op": "solid-from", "scale": str(scale), "view": "elevation", "id": f"slab-r{i}-edge", "from": "front", "source": f"slab-r{i}-face"} for scale in scales(a) for i in (1, 2, 3)])
