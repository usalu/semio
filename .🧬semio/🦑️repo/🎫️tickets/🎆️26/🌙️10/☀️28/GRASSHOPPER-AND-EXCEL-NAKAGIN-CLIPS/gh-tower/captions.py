"""🧪 Check that every clause of Beat3_Tower wraps to at most two caption lines in both languages."""
import importlib
import os
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
scene = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🗼️tower.🐍️").Beat3_Tower
bad = 0
for key, en, de in scene.NARRATION:
    for language, text in (("en", en), ("de", de)):
        try:
            art.caption_bar(text)
        except ValueError as error:
            bad += 1
            print(f"[DEBUG] {key} {language}: {str(error)[:60]}")
print(f"[DEBUG] {bad} clauses too long")
