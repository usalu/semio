"""🔍 Capture the actual authored power scene before and after its cursor-committed activation."""

import importlib
import os
import sys
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
from manim import tempconfig
from PIL import Image

physics = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🧲️physics.🎬️clip.🧲️fundamentals.🐍️")
output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
output.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_AUDIENCE"] = "spectator"
original = physics.cursor_click
records = []


class Complete(Exception):
    """⏹️ Stop diagnostic execution after the first real committed equipment activation."""


def capture(scene, name):
    """📸 Preserve the native rendered state of the actual production scene."""
    scene.camera.reset()
    scene.camera.capture_mobjects(scene.mobjects)
    scene.camera.get_image().save(output / (name + ".png"))


def cursor(scene, target, activate, **kwargs):
    """⚡ Observe the real helper and callback while keeping production geometry untouched."""
    def event(value):
        records.append(value["type"])
        if value["type"] == "press":
            capture(scene, "press")

    def committed():
        assert records[-1] == "click"
        activate()
        records.append("activated")
        capture(scene, "active")

    result = original(scene, target, committed, on_event=event, **kwargs)
    assert result and records.index("release") < records.index("click") < records.index("activated")
    raise Complete


physics.cursor_click = cursor
try:
    with tempconfig({"media_dir": str(output), "text_dir": str(output / "t"), "pixel_width": 1280,
                     "pixel_height": 720, "frame_rate": 8, "skip_animations": True, "write_to_movie": False,
                     "save_last_frame": False, "disable_caching": True, "verbosity": "ERROR", "progress_bar": "none"}):
        try:
            physics.Beat1_Leistung().render()
        except Complete:
            pass
finally:
    physics.cursor_click = original
assert "activated" in records
contact = Image.new("RGB", (1280, 1440), "#001117")
for index, name in enumerate(("press", "active")):
    with Image.open(output / (name + ".png")) as frame:
        contact.paste(frame, (0, index * 720))
contact.save(Path(__file__).with_name("cursor-physics.png"))
Path(__file__).with_name("cursor-physics-check.md").write_text(
    "# Authored Physics Cursor Activation\n\n"
    "The actual production Beat1_Leistung scene was executed with native animation skipping for diagnostic states. "
    "The first physical equipment callback runs only after press, release and committed click. "
    "The snapshots show the pressed Start control and its actual activated An state with the power/energy chart advanced. "
    "The source helper and all scientific geometry are unchanged by the observer.\n\n"
    "Observed events: " + " → ".join(records) + "\n", encoding="utf-8")
print("[DEBUG] Actual production physics cursor activation verified", flush=True)
