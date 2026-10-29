import importlib, os, sys, copy
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_LANGUAGE"] = "de"
v = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🎨️visuals.🐍️")
xl = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧱️model.🐍️")
s = v.ClipScene()
w = s.xl_window(xl.workbook(language="de"), "Type A", at=(0, 0, 0), width=8.5, height=5.0, cols="A:B", rows=(1, 10))
try:
    copy.deepcopy(w); print("[DEBUG] deepcopy ok")
except Exception as e:
    print("[DEBUG] deepcopy fails", e)
print("[DEBUG] tabs", list(w.tabs), [t for t in dir(w) if not t.startswith("_")][:0])
print("[DEBUG] text B2", w.text_of("B2"), "A2", w.text_of("A2"))
