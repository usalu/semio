"""🔍 pa: first differing line of a failed roundtrip - usage: diffrt.py <asset-id>."""
import json
import sys

from jio import dumps, locate

raw = locate(sys.argv[1]).read_text(encoding="utf8")
out = dumps(json.loads(raw))
for i, (x, y) in enumerate(zip(raw.split("\n"), out.split("\n"))):
    if x != y:
        print(i + 1, repr(x), repr(y))
        break
