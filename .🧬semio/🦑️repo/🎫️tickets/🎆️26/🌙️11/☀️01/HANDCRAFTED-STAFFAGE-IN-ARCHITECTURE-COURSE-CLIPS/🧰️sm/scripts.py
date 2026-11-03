import json, re, subprocess, sys
from pathlib import Path
R = Path("C:/git/semio")
A = R / "🎓️teaching/🏛️architecture"
found = {}
for m in A.rglob("🔣️.json"):
    if "node_modules" in str(m):
        continue
    try:
        d = json.loads(m.read_text(encoding="utf-8"))
    except Exception:
        continue
    if not isinstance(d, dict) or "videos" not in d:
        continue
    def walk(o):
        if isinstance(o, dict):
            s = o.get("script")
            if isinstance(s, str):
                found.setdefault(s, set()).add(m.name)
            for v in o.values():
                walk(v)
        elif isinstance(o, list):
            for v in o:
                walk(v)
    walk(d["videos"])
    print("manifest", m.relative_to(R), len(d["videos"]), file=sys.stderr)
out = []
for s in sorted(found):
    p = R / "🎓️teaching" / s
    out.append((s, p.exists()))
for s, e in out:
    print(("OK " if e else "MISSING ") + s)
