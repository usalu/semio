"""🚌️ Writes the body repair ops of `city-bus` (ops/city-bus.json), the first step of make/city-bus.sh.

The upper tail wall (`tail-upper`) ended at x 0.35 m, exactly where the two upper side walls begin: the solids touched in a plane without sharing material, and the side elevation of the body showed a seam of no width from the belt line to the roof (the 335 mm of 1:50 and 1:100). The tail wall runs 5 mm into the side walls, as a welded corner does."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, solid, write

SLUG = "city-bus"
a = asset(SLUG)
ops = []
for scale in ("50", "100"):
    s = solid(a, scale, "tail-upper")
    assert all(p[1] <= 0.355 + 1e-9 for p in s["profile"])
    ops.append({"op": "solid-set", "scale": scale, "id": "tail-upper", "set": {"profile": [[p[0], 0.355 if p[1] >= 0.35 - 1e-9 else p[1]] for p in s["profile"]]}})
write(SLUG, ops)
print(f"{SLUG}: {len(ops)} ops")
