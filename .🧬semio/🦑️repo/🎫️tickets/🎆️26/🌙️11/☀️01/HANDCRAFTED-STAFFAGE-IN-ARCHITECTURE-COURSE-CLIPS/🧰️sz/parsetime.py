"""⏱️ Parse time of the cast frames with the Python staffage parser: every fourth frame of every cast of an export folder, plus the largest frame of each cast; prints seconds and frame count.

@see ./compare.py — pixel comparison
"""

import importlib
import json
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path.cwd()))
staffage = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
root = Path(sys.argv[1])
jobs = []
for folder in sorted(p for p in root.iterdir() if p.is_dir()):
    index = json.loads((folder / "🔣️.json").read_text(encoding="utf-8"))
    names = sorted(p.name for p in folder.glob("*.svg"))
    chosen = {name for i, name in enumerate(names) if i % 4 == 0} | {max(names, key=lambda n: (folder / n).stat().st_size)}
    jobs += [(folder / name, index) for name in sorted(chosen)]
texts = [(path.read_text(encoding="utf-8"), index) for path, index in jobs]
started = time.perf_counter()
points = sum(len(staffage._parse_frame(text, index).points) for text, index in texts)
print(json.dumps({"frames": len(texts), "seconds": round(time.perf_counter() - started, 2), "points": points}))
