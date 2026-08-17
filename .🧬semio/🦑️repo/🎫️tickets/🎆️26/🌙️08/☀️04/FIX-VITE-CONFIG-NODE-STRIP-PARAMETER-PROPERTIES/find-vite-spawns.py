from pathlib import Path
import re
root = Path("/Users/ueli/Documents/semio")
pat = re.compile(r'["\']vite["\']')
for p in root.rglob("*.ts"):
    s = str(p)
    if "node_modules" in s or "/target/" in s or "/.git/" in s:
        continue
    try:
        t = p.read_text()
    except Exception:
        continue
    for i, line in enumerate(t.splitlines(), 1):
        if not pat.search(line):
            continue
        if "vitest" in line.lower():
            continue
        if any(x in line for x in ("run", "spawn", "bunx", "Bunx", "args", "[")):
            print(f"{p.relative_to(root)}:{i}:{line.strip()[:180]}")
