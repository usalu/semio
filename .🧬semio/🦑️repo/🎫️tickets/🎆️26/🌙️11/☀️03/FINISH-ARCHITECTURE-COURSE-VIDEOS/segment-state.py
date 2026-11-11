"""🧬 Record source segment fingerprints independently of presentation scanning."""
import hashlib
import importlib
import json
import sys
from pathlib import Path
root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(root))
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
settings = {"language": "en", "quality": "-ql", "resolution": None, "fps": 15, "audience": "speaker"}
selected = {v["id"]: v for v in runtime.load_manifest() if v["id"] in ("physical_fundamentals", "optimization_extrema")}
for name, video in selected.items():
    segment = video["chapters"][0]["segments"][0]
    state = runtime.segment_state(segment, settings)
    print(name, hashlib.sha256(json.dumps(state, sort_keys=True).encode()).hexdigest())
