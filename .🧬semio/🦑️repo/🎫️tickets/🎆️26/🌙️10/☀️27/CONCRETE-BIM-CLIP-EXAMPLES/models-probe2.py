import importlib, json, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
M = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
m = M.load_model()
print(json.dumps(m["translation"], ensure_ascii=False, indent=1))
print(json.dumps(m["rules"], ensure_ascii=False)[:1500])
print(json.dumps(m["models"], ensure_ascii=False)[:1500])
