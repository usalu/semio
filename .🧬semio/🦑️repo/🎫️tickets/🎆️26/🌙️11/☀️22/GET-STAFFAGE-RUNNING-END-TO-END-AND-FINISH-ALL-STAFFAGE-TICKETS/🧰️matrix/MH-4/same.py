"""🟰️ Whether two asset documents are the same but for `asset.tags` (the tags of the whole catalog were rewritten as titled objects on 2026-11-25 19:09 by their owner, after the trial bases were copied): `python3 same.py <a.json> <b.json>`, exit code 0 when they are."""
import json, sys
a, b = (json.load(open(p)) for p in sys.argv[1:3])
a["asset"].pop("tags", None); b["asset"].pop("tags", None)
sys.exit(0 if a == b else 1)
