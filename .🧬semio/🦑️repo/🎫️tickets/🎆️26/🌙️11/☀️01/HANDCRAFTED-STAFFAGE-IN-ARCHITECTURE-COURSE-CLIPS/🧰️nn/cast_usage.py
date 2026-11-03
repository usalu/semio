"""🔗 Map each staffage cast key that the energy clip modules reference to its catalog asset, clip and view."""
import json
import re
import sys
from pathlib import Path

ROOT = Path(sys.argv[1])
MANIFEST = Path(sys.argv[2])
casts = json.loads(MANIFEST.read_text(encoding="utf-8"))["casts"]
modules = [p for p in ROOT.rglob("🐍️.py") if "🎬️clip" in p.parts]
usage = {}
for path in modules:
    text = path.read_text(encoding="utf-8")
    for key in casts:
        if re.search(r"[\"']" + re.escape(key) + r"[\"']", text):
            usage.setdefault(key, set()).add(str(path.relative_to(ROOT)).replace("\\", "/"))
for key in sorted(usage):
    cast = casts[key]
    print(key, "|", cast.get("asset"), cast.get("clip"), cast.get("view"), "|", sorted(usage[key])[0])
print(len(usage), "casts used by", len(modules), "clip modules")
