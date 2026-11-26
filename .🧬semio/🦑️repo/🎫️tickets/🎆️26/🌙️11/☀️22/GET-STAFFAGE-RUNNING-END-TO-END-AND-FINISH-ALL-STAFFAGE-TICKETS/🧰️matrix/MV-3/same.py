"""🪞️ Whether two asset documents draw the same: `python3 same.py <a.json> <b.json>` compares `asset.representations` and `asset.anchors` and prints `representations equal|differ`, `anchors equal|differ` (anchors differ when only one of the two ran `anchors.ts`)."""
import json, sys

a, b = (json.load(open(path))["asset"] for path in sys.argv[1:3])
print(", ".join(f"{name} {'equal' if a[name] == b[name] else 'differ'}" for name in ("representations", "anchors")))
