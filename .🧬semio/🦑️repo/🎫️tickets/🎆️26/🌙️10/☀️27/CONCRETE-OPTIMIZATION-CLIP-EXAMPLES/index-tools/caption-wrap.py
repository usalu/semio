"""Print the wrapped caption lines of the index scene in both languages and flag one-word last lines."""

import ast
import importlib
import os
import sys
import tempfile
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_CACHE_DIR"] = tempfile.mkdtemp()
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
fonts.apply_body_font()
source = (ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/⚖️index/🐍️.py").read_text(encoding="utf-8")
tree = ast.parse(source)
cls = next(n for n in tree.body if isinstance(n, ast.ClassDef) and n.name == "Beat4_NormalizedIndex")
rows = next(ast.literal_eval(n.value) for n in cls.body if isinstance(n, ast.Assign) and any(isinstance(t, ast.Name) and t.id == "NARRATION" for t in n.targets))
for key, en, de in rows:
    for tag, text in (("en", en), ("de", de)):
        lines = fonts.wrap_body_lines(text, font_size=fonts.CAPTION_FONT_SIZE, max_width=art.CAPTION_MAX_WIDTH)
        flag = "ORPHAN" if len(lines) > 1 and len(lines[-1].split()) == 1 else ""
        if flag or len(lines) > 2:
            print(f"{key:14} {tag} {flag} {lines}")
