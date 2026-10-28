import importlib, os, sys, time
from pathlib import Path
T = Path(__file__).resolve().parent
ROOT = next(p for p in T.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(T / "🗑️generated/fonts/probe")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
import manim, numpy as np
fonts.apply_scene_style if False else None
def t(s):
    return fonts.body_text(s, font_size=16.2, color="#ffffff")
t("warm")
for s in ("0", "00", "1111", "0000", "0.5", "0,5", " m", "0 m", "Ø 0.30 m", "−1.2", "3.25 m"):
    st = time.time()
    x = t(s)
    print("[DEBUG]", repr(s), "n=", len(x.submobjects), "w=%.4f h=%.4f" % (x.width, x.height), "%.0f ms" % ((time.time()-st)*1000), [round(float(m.get_center()[0]-x.submobjects[0].get_center()[0]), 4) for m in x.submobjects])
for ch in "0123456789":
    x = t(ch + ch)
    print("[DEBUG] adv", ch, round(float(x.submobjects[1].get_center()[0]-x.submobjects[0].get_center()[0]), 4), round(x.submobjects[0].width,4), round(x.submobjects[0].get_center()[1], 4))
