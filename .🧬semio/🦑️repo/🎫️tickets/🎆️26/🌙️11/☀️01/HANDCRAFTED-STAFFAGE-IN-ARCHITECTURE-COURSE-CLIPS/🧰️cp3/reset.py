import json, sys
from pathlib import Path
p = Path(sys.argv[1]); s = json.loads(p.read_text(encoding="utf-8"))
for k in sys.argv[2:]:
    s.pop(k, None)
p.write_text(json.dumps(s, indent=2, ensure_ascii=False), encoding="utf-8")
