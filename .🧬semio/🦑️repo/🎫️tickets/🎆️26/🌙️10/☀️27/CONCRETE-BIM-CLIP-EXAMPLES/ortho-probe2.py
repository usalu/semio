import importlib, os, sys, time, json
from pathlib import Path
T = Path(__file__).resolve().parent
ROOT = next(p for p in T.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(T / "🗑️generated/fonts/probe")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
fonts.body_text("warm", font_size=16.2, color="#ffffff")
s = time.time()
for k in range(40):
    fonts.body_text(f"{k * 0.37:.2f} m", font_size=16.2, color="#ffffff")
print("[DEBUG] 40 texts warm", time.time() - s)
for key, kind in (("W-S","plan"),("W-S","section"),("W-S","elevation"),("W-S","3d"),("W-S","axis"),("W-S","footprint"),("W-S","body"),("B3","plan"),("SL-1","plan"),("RW-S","plan")):
    try:
        r = model.represent(key, kind)
        print("[DEBUG]", key, kind, json.dumps(r, ensure_ascii=False)[:700])
    except Exception as e:
        print("[DEBUG] ERR", key, kind, e)
m = model.load_model()
print("[DEBUG] translation", json.dumps(m["translation"], ensure_ascii=False)[:1500])
print("[DEBUG] notes", m["orientation_note"])
