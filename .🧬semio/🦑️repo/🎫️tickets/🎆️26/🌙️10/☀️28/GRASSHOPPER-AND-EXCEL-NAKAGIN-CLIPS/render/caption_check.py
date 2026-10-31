"""📏 Static caption check: wraps every NARRATION clause of the nine scenes exactly like `caption_bar` and lists overflows.
Usage: python caption_check.py [en|de ...]
"""
import importlib
import inspect
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parents[1]
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
languages = sys.argv[1:] or ["en", "de"]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = languages[0]
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts/caption-check")
SCENES = [("🦗️grasshopper", "💊️capsule"), ("🦗️grasshopper", "🗼️tower"), ("🦗️grasshopper", "🌳️trees"), ("🦗️grasshopper", "🫗️casting"),
          ("🦗️grasshopper", "🦴️anatomy"), ("📊️excel", "🦴️anatomy"), ("📊️excel", "📍️references"), ("📊️excel", "📋️program"), ("📊️excel", "⚡️energy")]
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
visuals = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
bad = 0
for series, topic in SCENES:
    module = importlib.import_module(f"🎓️teaching.🏛️architecture.{series}.🎬️clip.{topic}.🐍️")
    for name, cls in inspect.getmembers(module, inspect.isclass):
        rows = cls.__dict__.get("NARRATION")
        if not rows or cls.__module__ != module.__name__:
            continue
        for row in rows:
            for index, lang in ((1, "en"), (2, "de")):
                if lang not in languages:
                    continue
                if fonts.body_text(row[index], font_size=fonts.CAPTION_FONT_SIZE, color="#FFFFFF").width <= 1.6 * visuals.CAPTION_MAX_WIDTH:
                    continue
                lines = fonts.wrap_body_lines(row[index], font_size=fonts.CAPTION_FONT_SIZE, max_width=visuals.CAPTION_MAX_WIDTH)
                if len(lines) > visuals.CAPTION_MAX_LINES:
                    bad += 1
                    print(f"OVER {series}/{topic} {name} {row[0]} {lang} lines={len(lines)} len={len(row[index])}: {row[index]}")
print("[DEBUG] overflows:", bad)
