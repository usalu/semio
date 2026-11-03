import json
import sys
from pathlib import Path

ROOT = Path(r"C:\git\semio")
cache = ROOT / ".🧬semio/🎓️teaching/⚡️cache/staffage"
manifest = json.loads((ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json").read_text(encoding="utf-8"))["casts"]
best = {}
for index in cache.glob("*/*/🔣️.json"):
    cast = index.parent.name
    if cast in manifest:
        m = index.stat().st_mtime
        if cast not in best or m > best[cast][0]:
            best[cast] = (m, index)
out = {}
for cast in manifest:
    if cast in best:
        d = json.loads(best[cast][1].read_text(encoding="utf-8"))
        out[cast] = {"h": d["heightM"], "b": d["boundsM"], "mm": d["mmPerMetre"], "n": len(d["frames"]), "fps": d["fps"], "dir": str(best[cast][1].parent)}
    else:
        out[cast] = None
Path(sys.argv[1]).write_text(json.dumps(out, ensure_ascii=False, indent=1), encoding="utf-8")
print(sum(1 for v in out.values() if v), "cached of", len(out), [c for c, v in out.items() if not v])
