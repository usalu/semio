import json
import os
import re
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(r"C:\git\semio")
ARCH = ROOT / "🎓️teaching" / "🏛️architecture"
MANIFEST = ARCH / "⚡️energy" / "🏗️authoring" / "🎨️visuals" / "🧍️staffage" / "🔣️.json"
OUT = Path(sys.argv[1])
casts = json.loads(MANIFEST.read_text(encoding="utf-8"))["casts"]
pattern = re.compile("|".join(sorted((re.escape(c) for c in casts), key=len, reverse=True)))
refs = defaultdict(list)
for base, dirs, files in os.walk(ARCH):
    dirs[:] = [d for d in dirs if d not in {"node_modules", "__pycache__", ".git", "🧍️staffage"} or base.endswith("🎨️visuals") and d == "🧍️staffage" and False]
    dirs[:] = [d for d in dirs if "🗑️" not in d]
    for f in files:
        if not f.endswith((".py", ".json", ".ts", ".tsx", ".md", ".sh")):
            continue
        p = Path(base) / f
        if p == MANIFEST:
            continue
        try:
            text = p.read_text(encoding="utf-8")
        except Exception:
            continue
        if "staffage" not in text and not pattern.search(text):
            continue
        for n, line in enumerate(text.splitlines(), 1):
            for m in pattern.finditer(line):
                refs[m.group(0)].append((p.relative_to(ARCH).as_posix(), n, line.strip()[:200]))
res = {}
for c in casts:
    res[c] = refs.get(c, [])
(OUT / "xc_refs.json").write_text(json.dumps(res, ensure_ascii=False, indent=1), encoding="utf-8")
lines = []
for c, rs in sorted(res.items()):
    lines.append(f"## {c} ({len(rs)})")
    for f, n, l in rs:
        lines.append(f"   {f.replace('🎬️clip/', '')}:{n}: {l}")
(OUT / "xc_refs.txt").write_text("\n".join(lines), encoding="utf-8")
print("unreferenced:", [c for c, r in res.items() if not r])
print("referenced casts:", sum(1 for r in res.values() if r))
