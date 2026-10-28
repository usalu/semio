"""🔎 Reproduce the exact physics slip caption against reused and isolated SVG caches."""

import importlib
import json
import os
import sys
import traceback
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
from manim import Text, tempconfig

BASE = Path(__file__).parent
OUT = BASE / "🗑️generated/o"
MODULE = "🎓️teaching.🏛️architecture.⚡️energy"
fonts = importlib.import_module(MODULE + ".🏗️authoring.🔤️fonts.🐍️")
visuals = importlib.import_module(MODULE + ".🏗️authoring.🎨️visuals.🐍️")
physics = importlib.import_module(MODULE + ".🧲️physics.🎬️clip.🧲️fundamentals.🐍️")
fonts.apply_body_font()
events = []
original_svg = Text._text2svg

def trace_svg(self, color):
    """📄 Record native text output before Manim parses its document."""
    path = original_svg(self, color)
    file = Path(path)
    event = {"text": self.text, "font": self.font, "size": self.__dict__.get("_font_size"), "svg": str(file), "bytes": file.stat().st_size}
    events.append(event)
    if event["bytes"] == 0:
        print("[DEBUG] EMPTY " + json.dumps(event, ensure_ascii=False), flush=True)
    return path

Text._text2svg = trace_svg
slip = visuals.subtitle_text(physics.Beat1_EnergieImAlltag.NARRATION, "slip")
results = []
for directory in ("qa", "qd"):
    root = OUT / directory
    root.mkdir(parents=True, exist_ok=True)
    try:
        with tempconfig({"media_dir": str(root), "text_dir": str(root / "t"), "tex_dir": str(root / "x"), "pixel_width": 1280, "pixel_height": 720}):
            caption = visuals.caption_bar(slip)
            result = {"directory": directory, "success": True, "width": caption.width, "height": caption.height}
    except Exception as error:
        result = {"directory": directory, "success": False, "error": str(error), "trace": traceback.format_exc()}
    results.append(result)
    print("[DEBUG] CAPTION " + json.dumps(result, ensure_ascii=False), flush=True)
report = "# Native Slip Caption Investigation\n\n"
report += "Exact caption:\n\n" + slip + "\n\n"
report += "The verified empty QA SVG was removed before this run. No other QA cache files were removed.\n\n"
report += "## Reproduction Results\n\n" + "\n".join("- " + json.dumps(item, ensure_ascii=False) for item in results)
report += "\n\n## Native Text Outputs\n\n" + "\n".join("- " + json.dumps(item, ensure_ascii=False) for item in events)
(BASE / "pango-debug.md").write_text(report + "\n", encoding="utf-8")
print("[DEBUG] Native SVG events: " + str(len(events)), flush=True)
