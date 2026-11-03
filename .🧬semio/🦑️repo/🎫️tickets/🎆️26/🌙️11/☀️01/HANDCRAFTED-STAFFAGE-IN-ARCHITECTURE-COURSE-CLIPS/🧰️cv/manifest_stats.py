import json, sys
from pathlib import Path
p = Path(sys.argv[1])
m = json.loads(p.read_text(encoding="utf-8"))["casts"]
print(len(m))
for k, v in m.items():
    if "transport" in v or "timeline" in v or "thermalSpanK" in v and "timeline" in v:
        print(k, {a: b for a, b in v.items() if a in ("transport", "transportAgeS", "timeline", "environment", "clip", "pose", "fps", "view")})
