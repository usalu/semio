"""🎞️ Render one quick 480p15 speaker preview of any manifest segment straight through Manim from one import snapshot.

Usage: python preview_any.py <script-path> <Scene> <Output> <lang> [patch-module]
`script-path` is the manifest segment script (for example 🏛️architecture/🏗️bim/🎬️clip/📊️information/🐍️.py).
An optional preview-only patch module from this folder (with `install()`) is applied before the scene is imported.
Output: .🧬semio/🎓️teaching/⚡️cache/videos/<lang>/speaker/preview/<Output>_speaker_<lang>_480p15.mp4 plus a .json record.
"""
import hashlib
import importlib
import json
import os
import shutil
import sys
import time
from pathlib import Path

TICKET = Path(__file__).resolve().parents[1]
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
path, name, output, language = sys.argv[1:5]
patch = sys.argv[5] if len(sys.argv) > 5 else None
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / f"🗑️generated/fonts/preview-{name}-{language}")
media = TICKET / "🗑️generated/preview" / f"{name}-{language}"
target = ROOT / ".🧬semio/🎓️teaching/⚡️cache/videos" / language / "speaker/preview" / f"{output}_speaker_{language}_480p15.mp4"
sys.path.insert(0, str(Path(__file__).resolve().parent))
if patch:
    importlib.import_module(patch).install()
manim = importlib.import_module("manim")
module = "🎓️teaching." + path.removesuffix(".py").replace("/", ".")
scene_class = getattr(importlib.import_module(module), name)
started = time.time()
with manim.tempconfig({"pixel_width": 854, "pixel_height": 480, "frame_rate": 15, "write_to_movie": True, "disable_caching": True,
                       "media_dir": str(media), "verbosity": "WARNING", "progress_bar": "none"}):
    scene = scene_class()
    scene.render()
    rendered = Path(scene.renderer.file_writer.movie_file_path)
target.parent.mkdir(parents=True, exist_ok=True)
shutil.copyfile(rendered, target)
record = {"scene": name, "module": module, "language": language, "audience": "speaker", "size": [854, 480], "fps": 15,
          "seconds": round(scene.renderer.time, 2), "render_seconds": round(time.time() - started), "rendered_at": time.strftime("%Y-%m-%dT%H:%M:%S"),
          "sha256": hashlib.sha256(target.read_bytes()).hexdigest(), "patch": patch,
          "note": "preview from one import snapshot; not a certified master"}
target.with_suffix(".json").write_text(json.dumps(record, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print("[DEBUG] preview", target, record["seconds"], "s", flush=True)
