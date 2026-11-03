import re, sys
from pathlib import Path
S = Path("🎓️teaching/🏛️architecture/🧍️staffage").resolve()
start = S / "🔨️modules/📤️export/🎭️cast/🟦️.ts"
seen = set(); stack = [start]
rx = re.compile(r'(?:from|import)\s*\(?\s*"(\.[^"]*)"')
while stack:
    f = stack.pop()
    if f in seen: continue
    seen.add(f)
    for m in rx.findall(f.read_text(encoding="utf-8")):
        p = (f.parent / m).resolve()
        if p.is_file(): stack.append(p)
        else: print("UNRESOLVED", f.name, m)
for f in sorted(seen):
    rel = f.relative_to(S).as_posix()
    if not (rel.startswith("🔨️modules/") or rel.startswith("🧬️schema/")): print("OUTSIDE", rel)
print(len(seen))
