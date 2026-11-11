"""🎞️ Exercise actual per-chapter focus and restore packets in publication order."""
import importlib
import sys
from pathlib import Path
from PIL import Image

root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(root))
work = Path(__file__).parent / "🗑️generated" / "actual-restore"
work.mkdir(parents=True, exist_ok=True)
snapshot = work / "snapshot.png"
Image.new("RGB", (854, 480), "#222222").save(snapshot)
presentation = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🐍️")
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
settings = {"language": "en", "quality": "-ql", "resolution": None, "fps": 15, "audience": "speaker"}
session = runtime.ToolSession()
for scene in ("Ventilation_01_WarumLueften",):
    scene_work = work / scene
    scene_work.mkdir(exist_ok=True)
    for phase in ("restore",):
        _, count = presentation._transition(scene, snapshot, scene_work, settings, session, phase=phase)
        print(scene, phase, count, flush=True)
