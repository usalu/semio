import importlib, os, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
v = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🎨️visuals.🐍️")
print([c.__name__ for c in v.ClipScene.__mro__])
print(len(v.source_files()), [p.name for p in v.source_files()][:12])
