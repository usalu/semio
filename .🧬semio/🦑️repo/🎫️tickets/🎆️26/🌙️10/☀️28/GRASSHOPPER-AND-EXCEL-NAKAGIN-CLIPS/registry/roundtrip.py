import json, sys
from pathlib import Path
for name in sys.argv[1:]:
    p = Path(name); t = p.read_text(encoding="utf-8")
    for nl in ("\n",):
        out = json.dumps(json.loads(t), indent=2, ensure_ascii=False)
        print(name[-60:], out == t, out + "\n" == t, "\r\n" in t)
