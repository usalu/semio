import importlib
import os
import re
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
module = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🌳️trees.🐍️")
scene = module.Beat5_DataTrees
rows = scene.NARRATION
source = Path(module.__file__).read_text(encoding="utf-8")
keys = [r[0] for r in rows]
used = set(re.findall(r'self\.step\("(\w+)"', source)) | {k + "_one" for k in re.findall(r'operate\("(\w+)"', source)} | set(re.findall(r'operate\("(\w+)"', source))
print("[DEBUG] clauses", len(rows), "unique", len(set(keys)) == len(keys), "bilingual", all(len(r) == 3 and r[1] and r[2] for r in rows))
print("[DEBUG] unused", [k for k in keys if k not in used], "missing", [k for k in used if k not in keys])
print("[DEBUG] title", scene.TITLE, "present_trees", hasattr(scene, "present_trees"))
