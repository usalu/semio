import importlib, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model = m.load_model()
found = set()
def walk(n):
    if isinstance(n, dict):
        if isinstance(n.get("unit"), str): found.add(n["unit"])
        for v in n.values(): walk(v)
    elif isinstance(n, list):
        for v in n: walk(v)
walk(model)
print("[DEBUG] units", sorted(found).__repr__().encode("ascii", "backslashreplace").decode())
st = [ (s["tag"], s["name"]["en"], s["elevation"]) for s in model["storeys"]]
print("[DEBUG] storeys", st)
sp = next(s for s in model["spaces"] if s["tag"]=="SP-TERRACE")
print("[DEBUG] terrace", sp["storey"], sp.get("name"))
