"""📏 Caption contract for the sampling scene only: every clause fits two lines in both languages and both audiences; reports short last lines."""
import ast
import importlib
import os
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CLIP = ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip"
tree = ast.parse((CLIP / "🎲️sampling/🐍️.py").read_text(encoding="utf-8"))
rows = next(ast.literal_eval(node.value) for cls in tree.body if isinstance(cls, ast.ClassDef) for node in cls.body
            if isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id == "NARRATION" for t in node.targets))
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🐍️")
visuals._fonts.apply_body_font()
bad = orphans = 0
for audience, captions in (("speaker", True), ("spectator", False)):
    for index, language in ((1, "en"), (2, "de")):
        os.environ["SEMIO_TEACHING_LANGUAGE"], os.environ["SEMIO_TEACHING_AUDIENCE"] = language, audience
        reference = visuals._art.caption_bar("Ag")
        for row in rows:
            caption = visuals._art.caption_bar(row[index])
            lines = 2 if caption.height > reference.height * 1.4 else 1
            ok = len(caption) == (2 if captions else 1) and caption.width <= visuals._engine.config.frame_width and caption.get_bottom()[1] >= -visuals._engine.config.frame_height / 2
            if not ok:
                bad += 1
                print("FAIL", audience, language, row[0], len(caption))
            if captions and lines == 2:
                ys = sorted(member.get_center()[1] for member in caption[-1].family_members_with_points())
                middle = (ys[0] + ys[-1]) / 2
                last = sum(1 for y in ys if y < middle)
                if last <= 8:
                    orphans += 1
                    print("ORPHAN", language, row[0], last, "glyphs on the last line")
print(len(rows), "clauses checked,", bad, "failures,", orphans, "short last lines")
