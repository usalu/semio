import importlib
import sys
from pathlib import Path

root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(root))
visuals = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
for folder in ("🪟️natural", "⚙️mechanical", "🤝️handoff"):
    mod = importlib.import_module(f"🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.{folder}.🐍️")
    for name in dir(mod):
        scene = getattr(mod, name)
        narration = getattr(scene, "NARRATION", None)
        if not isinstance(narration, list) or not name.startswith("Beat"):
            continue
        for key, en, de in narration:
            for lang, text in (("de", de), ("en", en)):
                try:
                    bar = visuals.caption_bar(text)
                    lines = len(bar[1].submobjects) if len(bar) > 1 and hasattr(bar[1], "submobjects") and bar[1].submobjects else 1
                except ValueError as err:
                    print("TOO LONG", ascii(folder), name, key, lang, len(text))
