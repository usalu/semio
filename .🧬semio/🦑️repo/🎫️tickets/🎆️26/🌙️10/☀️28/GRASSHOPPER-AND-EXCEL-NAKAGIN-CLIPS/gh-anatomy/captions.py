import importlib, os, sys
from pathlib import Path
T = Path(__file__).resolve().parents[1]
ROOT = next(p for p in T.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(T / "🗑️generated/fonts/captions")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
mod = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🦴️anatomy.🐍️")
for key, en, de in mod.Beat1_Anatomy.NARRATION:
    for lang, text in (("en", en), ("de", de)):
        try:
            art.caption_bar(text)
        except ValueError as e:
            print("LONG", key, lang, len(text))
print("checked")
