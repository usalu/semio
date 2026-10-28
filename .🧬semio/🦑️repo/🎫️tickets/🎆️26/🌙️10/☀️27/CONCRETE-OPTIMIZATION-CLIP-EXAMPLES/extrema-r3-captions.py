"""💬 Print the wrapped caption lines of every extrema clause and flag orphan last lines (two words or fewer)."""
import ast
import importlib
import os
import sys
from pathlib import Path

sys.path.insert(0, "C:/git/semio")
sys.stdout.reconfigure(encoding="utf-8")
TICKET = Path(__file__).resolve().parent
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / "🗑️generated/fonts/captions")
os.environ["SEMIO_TEACHING_LANGUAGE"] = "en"
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")
fonts, art = visuals._fonts, visuals._art
fonts.apply_body_font()
tree = ast.parse(Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/📉️extrema/🐍️.py").read_text(encoding="utf-8"))
scene = next(node for node in tree.body if isinstance(node, ast.ClassDef))
rows = next(ast.literal_eval(node.value) for node in scene.body if isinstance(node, ast.Assign) and node.targets[0].id == "NARRATION")
for key, en, de in rows:
    for language, text in (("en", en), ("de", de)):
        lines = fonts.wrap_body_lines(text, font_size=fonts.CAPTION_FONT_SIZE, max_width=art.CAPTION_MAX_WIDTH)
        flag = "ORPHAN" if len(lines) > 1 and len(lines[-1].split()) <= 2 else ""
        print(f"[DEBUG] {key:12} {language} lines={len(lines)} {flag:6} | {lines[-1]}")
