import importlib
import os
import sys
from pathlib import Path

root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(root))
prefix = "🎓️teaching.🏛️architecture.⚡️energy."
shared = importlib.import_module(prefix + "🏗️authoring.🎨️visuals.🐍️")
bad = 0
for folder in ("❓️need", "📦️volume", "🌡️heat-flow"):
    module = importlib.import_module(prefix + "🌬️ventilation.🎬️clip." + folder + ".🐍️")
    for name in dir(module):
        scene = getattr(module, name)
        narration = getattr(scene, "NARRATION", None)
        if not isinstance(narration, list) or not name.startswith("Beat"):
            continue
        for key, en, de in narration:
            for lang, text in (("en", en), ("de", de)):
                try:
                    shared.caption_bar(text)
                except Exception as error:
                    bad += 1
                    sys.stdout.buffer.write(f"{name} {key} {lang}: {str(error)[:60]}\n".encode("utf-8"))
sys.stdout.buffer.write(f"bad={bad}\n".encode("utf-8"))
