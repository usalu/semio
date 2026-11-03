import sys, os, time, ast, json, importlib, tempfile
from pathlib import Path
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, "C:/git/semio")
from unittest.mock import patch
from manim import tempconfig
CLIP = Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip")
CASES = json.loads((CLIP / "🧪️cases/🔣️.json").read_text(encoding="utf-8"))
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
visuals._fonts.apply_body_font()
out = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]); out.mkdir(parents=True, exist_ok=True)
with tempfile.TemporaryDirectory(dir=out) as folder, tempconfig({"media_dir": folder}):
    total = 0; t0 = time.time()
    for chapter in CASES["chapters"]:
        tree = ast.parse((CLIP / chapter["folder"] / "🐍️.py").read_text(encoding="utf-8"))
        scene = next(n for n in tree.body if isinstance(n, ast.ClassDef) and n.name == chapter["scene"])
        narration = next(ast.literal_eval(n.value) for n in scene.body if isinstance(n, ast.Assign) and any(isinstance(t, ast.Name) and t.id == "NARRATION" for t in n.targets))
        t = time.time(); n = 0
        for audience in CASES["audiences"]:
            for li, language in enumerate(CASES["languages"], 1):
                with patch.dict(os.environ, SEMIO_TEACHING_LANGUAGE=language, SEMIO_TEACHING_AUDIENCE=audience["id"]):
                    for clause in narration[:3]:
                        visuals._art.caption_bar(clause[li]); n += 1
        print(chapter["folder"], "clauses", len(narration), "sampled", n, "sec/caption", round((time.time() - t) / n, 3), "projected", round(len(narration) * 4 * (time.time() - t) / n), flush=True)
