"""🔎 Check final calculation/chart imports and the long shared chapter title."""

import ast
import importlib
import os
import sys
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
paths = [
    "🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧮️calculation/🐍️.py",
    "🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py",
]
for relative in paths:
    path = ROOT / relative
    ast.parse(path.read_text(encoding="utf-8"), filename=str(path))
    importlib.import_module(".".join(Path(relative).with_suffix("").parts))

from manim import Scene, config, tempconfig

fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
theme = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️")
output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) / "qa/final-title"
output.mkdir(parents=True, exist_ok=True)
with tempconfig({"pixel_width": 1280, "pixel_height": 720, "media_dir": str(output), "text_dir": str(output / "t")}):
    scene = Scene()
    fonts.apply_scene_style(scene)
    title = fonts.scene_title("Die Hauptgleichung des Heizwärmebedarfs (DIN V 18599)")
    assert title.get_left()[0] >= -config.frame_width / 2 + theme.UI_SPACING
    assert title.get_right()[0] <= config.frame_width / 2 - theme.UI_SPACING
    assert title.get_top()[1] <= config.frame_height / 2 - fonts.TITLE_EDGE_BUFF + 1e-8
    assert title.get_bottom()[1] > 0
    frame, content = title
    assert content.width < frame.width and content.height < frame.height
    scene.camera.reset()
    scene.camera.capture_mobjects([title])
    scene.camera.get_image().save(output / "calculation-title.png")
print("[DEBUG] Final calculation and ventilation modules parse/import; shared long title stays inside viewport and chrome")
