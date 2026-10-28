"""Per-clause speech budget of the algorithms scene (max of TTS estimate and subtitle reading time), both languages."""
import ast
import importlib
import os
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CLIP = ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip"
tree = ast.parse((CLIP / "🧬️algorithms/🐍️.py").read_text(encoding="utf-8"))
rows = next(ast.literal_eval(node.value) for cls in tree.body if isinstance(cls, ast.ClassDef) for node in cls.body
            if isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id == "NARRATION" for t in node.targets))
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
totals = {}
for index, language in ((1, "en"), (2, "de")):
    os.environ["SEMIO_TEACHING_LANGUAGE"] = language
    art.set_vo_language(language)
    localized = [(row[0], row[1], row[index]) for row in rows]
    total = 0.0
    for row in rows:
        budget = max(art.narration_seconds(localized, row[0]), art.subtitle_read_seconds(localized, row[0]))
        total += budget
        print(language, f"{row[0]:20s} {budget:5.1f}")
    totals[language] = total
print(totals)
