"""Emitters with vapourKgPerKg 0: the member is the absolute specific humidity of the source air (code: absent = the ambient air), so 0 dried the air of the 26 vents and fans whose authors meant "no vapour added"; the member is removed (idempotent), absent already means the ambient air."""
import glob, json, sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE")
from catalog_edit import S, edit
total = 0
for path in sorted(glob.glob(S + "*/*/🔣️.json")):
    if '"vapourKgPerKg": 0' not in open(path, encoding="utf-8").read(): continue
    n = edit(path, lambda t: '"vapourKgPerKg": 0' in t and '"anchor"' in t and '"directionDeg"' in t, lambda o: {k: v for k, v in o.items() if k != "vapourKgPerKg"} if o.get("vapourKgPerKg") == 0 and "directionDeg" in o else None)
    total += n
print(total)
