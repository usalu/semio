import importlib, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model = m.load_model()
sp = next(s for s in model["spaces"] if s["tag"] == "SP-TERRACE")
print("[DEBUG]", sp["storey"], sp.get("reference_area"), [ (k, type(v).__name__) for k,v in sp.items()])
g = m.building_geometry(model)
print("[DEBUG]", next(s for s in g["spaces"] if s["tag"]=="SP-TERRACE"))
