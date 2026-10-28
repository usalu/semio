import importlib, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
M = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
m = M.mesh("B3", 8)
print(sorted(m)); print(m["vertex_count"], m["triangle_count"], m["vertices"][:3], m["faces"][:3], m["kept"], m["lost"])
