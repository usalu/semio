import importlib
import os
import sys
from pathlib import Path

os.environ.setdefault("SEMIO_TEACHING_LANGUAGE", "de")
root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(root))
P = "🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip"
vis = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
bad = 0
for lang in ("de", "en"):
    vis.set_vo_language(lang)
    for clip in ("🏷️certificate", "🧪️practice"):
        mod = importlib.import_module(f"{P}.{clip}.🐍️")
        for name, obj in vars(mod).items():
            if name.startswith("Beat") and hasattr(obj, "NARRATION"):
                for key, en, de in obj.NARRATION:
                    text = vis.subtitle_text(obj.NARRATION, key)
                    try:
                        vis.caption_bar(text)
                    except Exception as exc:
                        bad += 1
                        print(lang, clip, name, key, str(exc)[:60])
print("bad", bad)
