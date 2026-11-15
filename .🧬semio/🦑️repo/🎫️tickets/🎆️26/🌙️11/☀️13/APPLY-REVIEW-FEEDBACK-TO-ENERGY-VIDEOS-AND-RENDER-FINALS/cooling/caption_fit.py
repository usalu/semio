import importlib
import inspect
import os
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
PKG = "🎓️teaching.🏛️architecture.⚡️energy"
VIS = importlib.import_module(PKG + ".🏗️authoring.🎨️visuals.🐍️")
MODULES = ["⚖️heating-comparison", "👥️internal-gains", "💧️transmission-humidity", "☀️solar-radiation", "📐️system-sizing", "🌤️summer-strategy"]
bad = 0
for name in MODULES:
    module = importlib.import_module(f"{PKG}.❄️cooling.🎬️clip.{name}.🐍️")
    for cls_name, cls in inspect.getmembers(module, inspect.isclass):
        narration = cls.__dict__.get("NARRATION")
        if not narration:
            continue
        for key, en, de in narration:
            for lang, text in (("en", en), ("de", de)):
                VIS.set_vo_language(lang)
                os.environ["SEMIO_TEACHING_LANGUAGE"] = lang
                try:
                    VIS.caption_bar(text)
                except Exception as error:
                    bad += 1
                    print(ascii(f"{cls_name}.{key}.{lang}"), ascii(str(error)[-120:]))
print("bad", bad)
