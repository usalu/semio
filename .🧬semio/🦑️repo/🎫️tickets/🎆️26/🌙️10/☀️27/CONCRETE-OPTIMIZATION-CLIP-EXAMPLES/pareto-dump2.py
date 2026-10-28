"""Which front members does the weighted sum reach."""
import importlib, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
prev = None
for k in range(0, 1001):
    w = k / 1000
    i = m.weighted_pick(w)
    if i != prev:
        print(w, i); prev = i
