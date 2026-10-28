"""📐 Find captions whose wrap separates a number from its unit (the shared wrapper splits on U+00A0)."""

import ast
import importlib
import os
import re
import sys
from pathlib import Path

ROOT = Path("C:/git/semio")
sys.path.insert(0, str(ROOT))
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/🗑️generated/fonts/wrap")
fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
fonts.apply_body_font()
source = (ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🏆️pareto/🐍️.py").read_text(encoding="utf-8")
scene = next(node for node in ast.parse(source).body if isinstance(node, ast.ClassDef))
rows = next(ast.literal_eval(node.value) for node in scene.body if isinstance(node, ast.Assign) and node.targets[0].id == "NARRATION")
bad = 0
for key, en, de in rows:
    for language, text in (("en", en), ("de", de)):
        lines = fonts.wrap_body_lines(text, font_size=fonts.CAPTION_FONT_SIZE, max_width=art.CAPTION_MAX_WIDTH)
        for first, second in zip(lines, lines[1:]):
            if re.search(r"\d$", first) and re.match(r"(CHF|kWh|%|m\b|cm|a\b|Jahre|years)", second):
                bad += 1
                print("[DEBUG] split", key, language, repr(first[-14:]), repr(second[:10]))
print("[DEBUG] captions checked, splits:", bad)
