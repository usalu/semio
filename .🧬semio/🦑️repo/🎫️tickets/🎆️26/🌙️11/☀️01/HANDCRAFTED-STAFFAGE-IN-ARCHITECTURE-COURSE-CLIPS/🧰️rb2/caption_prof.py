import sys, os, importlib, tempfile, cProfile, pstats
from pathlib import Path
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
sys.path.insert(0, "C:/git/semio")
from unittest.mock import patch
from manim import tempconfig
visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
visuals._fonts.apply_body_font()
out = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]); out.mkdir(parents=True, exist_ok=True)
with tempfile.TemporaryDirectory(dir=out) as folder, tempconfig({"media_dir": folder}), patch.dict(os.environ, SEMIO_TEACHING_LANGUAGE="en", SEMIO_TEACHING_AUDIENCE="speaker"):
    visuals._art.caption_bar("Warm up the cache of this caption.")
    pr = cProfile.Profile(); pr.enable()
    for i in range(5): visuals._art.caption_bar(f"Every wall carries a distinct claim number {i} that is long enough to wrap onto a second caption line.")
    pr.disable()
    pstats.Stats(pr).sort_stats("cumulative").print_stats(18)
