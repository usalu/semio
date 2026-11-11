"""🖼️ Exercise one real browser presentation frame in owned ticket output."""
import importlib
import sys
from pathlib import Path
from PIL import Image
root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(root))
work = Path(__file__).parent / "🗑️generated" / "ventilation-one-full"
work.mkdir(parents=True, exist_ok=True)
snapshot = work / "snapshot.png"
Image.new("RGB", (854, 480), "#222222").save(snapshot)
render = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🥞️hierarchy.🖌️render.🐍️")
requests = [{"video_id": "Ventilation_01_WarumLueften", "phase": phase, "t": t, "output": work / f"{phase}-{t}.png", "snapshot": snapshot} for phase in ("focus", "restore") for t in (i / 95 for i in range(96))]
settings = {"language": "en", "quality": "-ql", "resolution": None, "fps": 15, "audience": "speaker"}
render.frame_batch(requests, settings)
print("rendered", len(requests), "frames")
