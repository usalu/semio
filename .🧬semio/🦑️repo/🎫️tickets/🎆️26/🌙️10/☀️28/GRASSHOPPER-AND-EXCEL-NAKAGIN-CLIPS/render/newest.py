"""🕵️ List the newest fingerprinted sources of a segment to find who invalidates a render."""
import importlib
import sys
import time
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8")
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
topic, scene = sys.argv[1:3]
state = runtime.segment_state({"script": f"🏛️architecture/{topic}/🐍️.py", "scene": scene},
                              {"language": "en", "audience": "speaker", "quality": "h", "resolution": None, "fps": 30})
rows = sorted(((Path(p).stat().st_mtime, p) for p in state["sources"]), reverse=True)[:8]
print(len(state["sources"]), "sources")
for mtime, path in rows:
    print(time.strftime("%H:%M:%S", time.localtime(mtime)), path)
