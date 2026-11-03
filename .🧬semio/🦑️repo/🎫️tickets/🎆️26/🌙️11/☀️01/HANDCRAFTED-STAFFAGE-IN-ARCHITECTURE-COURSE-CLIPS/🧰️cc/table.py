import json, sys
from pathlib import Path
rows = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
man = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))["casts"]
for r in rows:
    if "error" in r:
        print("ERR", r); continue
    c = r["cast"]
    m = man.get(c, {})
    asset = m.get("asset", "-")
    view = m.get("view", "-")
    print(f'{asset:24} {c:42} {view:9} {r["file"][:70]:70} L{r["line"]:<4} {r["call"]:16} {json.dumps(r["kw"], ensure_ascii=False)}')
