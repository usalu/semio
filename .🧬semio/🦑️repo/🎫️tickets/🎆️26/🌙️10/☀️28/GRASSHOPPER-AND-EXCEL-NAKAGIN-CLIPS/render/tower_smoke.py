"""🗼 Time one axon tower drawing of the shared Nakagin visuals, to judge whether previews can render now."""
import importlib
import os
import sys
import time
from pathlib import Path

TICKET = Path(__file__).resolve().parents[1]
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ.setdefault("SEMIO_TEACHING_LANGUAGE", "en")
os.environ.setdefault("SEMIO_TEACHING_AUDIENCE", "speaker")
sys.path.insert(0, str(Path(__file__).resolve().parent))
importlib.import_module("painter").install()
module = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🎨️visuals.🐍️")
scene = module.ClipScene()
started = time.time()
drawing = scene.tower_axon(scale=0.05, segments=1)
print("[DEBUG] tower", len(drawing.family_members_with_points()), "parts", round(time.time() - started, 1), "s", flush=True)
roles = {}
for part in drawing.family_members_with_points():
    roles[getattr(part, "semantic_role", None)] = roles.get(getattr(part, "semantic_role", None), 0) + 1
print("[DEBUG] roles", sorted(roles.items(), key=lambda row: -row[1])[:12], flush=True)
manim = importlib.import_module("manim")
with manim.tempconfig({"pixel_width": 854, "pixel_height": 480, "frame_rate": 15}):
    camera = manim.Camera()
    started = time.time()
    for _ in range(3):
        camera.reset()
        camera.capture_mobjects([drawing])
    print("[DEBUG] frame", round((time.time() - started) / 3, 2), "s", flush=True)
    (TICKET / "🗑️generated").mkdir(exist_ok=True)
    camera.get_image().save(TICKET / "🗑️generated/tower-smoke.png")
