import re
import sys
from pathlib import Path

root = Path("🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip")
for f in sorted(root.glob("*/🐍️.py")):
    lines = f.read_text(encoding="utf-8").splitlines()
    defs = {}
    cur = None
    for i, l in enumerate(lines):
        m = re.match(r"(?:def|class) (\w+)", l)
        if m:
            cur = m.group(1)
            defs[cur] = []
        elif cur:
            defs[cur].append(l)
    base = {n for n, b in defs.items() if any(re.search(r"\bstaffage(_motion)?\(|fitted_staffage\(|_cast\(", x) for x in b)}
    grow = set(base)
    for _ in range(4):
        for n, b in defs.items():
            if any(re.search(r"\b%s\(" % re.escape(g), x) for g in grow for x in b):
                grow.add(n)
    scenes = [n for n in defs if n in grow and re.search(r"class " + n + r"\(Scene", "\n".join(lines))]
    print(f.parent.name, scenes)
