"""Check every casting caption fits two lines in both languages."""
import importlib
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parents[1]
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts/casting-captions")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
casting = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🫗️casting.🐍️")
manim = importlib.import_module("manim")
bad = 0
for key, en, de in casting.Beat4_Casting.NARRATION:
    for language, text in (("en", en), ("de", de)):
        try:
            art.caption_bar(text)
        except ValueError:
            bad += 1
            print("[DEBUG] too long", language, key, len(text))
print("[DEBUG] captions too long:", bad)
