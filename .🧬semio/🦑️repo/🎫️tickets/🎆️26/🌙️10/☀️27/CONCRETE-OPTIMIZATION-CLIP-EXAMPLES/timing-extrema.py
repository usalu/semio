"""⏱️ Print the reading hold of every extrema clause in both languages and the total."""
import ast
import importlib
import os
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8")
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
tree = ast.parse((ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py").read_text(encoding="utf-8"))
scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == "Beat1_LocalGlobalExtrema")
rows = next(ast.literal_eval(node.value) for node in scene.body if isinstance(node, ast.Assign) and node.targets[0].id == "NARRATION")
for index, language in ((1, "en"), (2, "de")):
    art.set_vo_language(language)
    clauses = [(key, en if language == "en" else de, en) for key, en, de in rows]
    clauses = [(row[0], row[index], row[1]) for row in rows]
    localized = [(key, text, text) for key, text, _ in clauses]
    seconds = [(key, art.narration_seconds(localized, key)) for key, _, _ in clauses]
    for key, value in seconds:
        print("[DEBUG]", language, key, round(value, 1))
    print("[DEBUG]", language, "total", round(sum(value for _, value in seconds), 1), "s")
