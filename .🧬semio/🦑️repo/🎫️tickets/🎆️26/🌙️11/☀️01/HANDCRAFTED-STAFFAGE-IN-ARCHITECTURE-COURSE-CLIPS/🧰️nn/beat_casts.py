"""🧭 For each claim line, print the enclosing class or function and the staffage casts it draws, resolved to catalog assets."""
import json
import re
import sys
from pathlib import Path

ENERGY = Path(sys.argv[1])
MANIFEST = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))["casts"]
TARGETS = [item.split(":", 1) for item in sys.argv[3:]]
PREFIXES = ("cooling-internal-gains-",)
for rel, line_text in TARGETS:
    path = ENERGY / rel.split("energy/", 1)[-1] if "energy/" in rel else ENERGY / rel
    lines = path.read_text(encoding="utf-8").splitlines()
    line = int(line_text)
    start = 0
    for index in range(line - 1, -1, -1):
        if re.match(r"(class|def) ", lines[index]):
            start = index
            break
    stop = min(len(lines), line + 60)
    for index in range(start + 1, len(lines)):
        if re.match(r"(class|def) ", lines[index]) and index > line:
            stop = index
            break
    block = "\n".join(lines[start:stop])
    keys = set(re.findall(r"[\"']([a-z0-9-]+)[\"']", block))
    casts = []
    for key in sorted(keys):
        if key in MANIFEST:
            casts.append(f"{key}->{MANIFEST[key].get('asset')}")
        elif key.startswith(PREFIXES) and key in MANIFEST:
            casts.append(key)
    dynamic = sorted(k for k in keys if k.startswith("cooling-internal-gains") or k in ("standing", "seated-worker", "sleeper", "walker", "jogger"))
    print(f"{rel}:{line} | {lines[start].strip()[:60]} | {'; '.join(casts) or '-'} | dyn {dynamic}")
