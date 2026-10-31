"""🎞️ Render several previews sequentially in ONE process (one import snapshot, one geometry memo) and replace each target only after completion.

Usage: python preview_many.py <patch-module> <lang> <list-file>   (list lines: <script-path> <Scene> <Output>)
Writes .🧬semio/🎓️teaching/⚡️cache/videos/<lang>/speaker/preview/<Output>_speaker_<lang>_480p15.mp4 plus .json (no fallback note).
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
sys.path.insert(0, str(Path(__file__).resolve().parent))
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
patch, language, listing = sys.argv[1:4]
os.environ["SEMIO_TEACHING_AUDIENCE"] = "speaker"
os.environ["SEMIO_TEACHING_LANGUAGE"] = language
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(TICKET / f"🗑️generated/fonts/preview-many-{language}")
patched = importlib.import_module(patch)
manim = importlib.import_module("manim")
patched.install()
for line in Path(listing).read_text(encoding="utf-8").split("\n"):
    if not line.strip():
        continue
    path, name, output = line.split()
    module = "🎓️teaching." + path.removesuffix(".py").replace("/", ".")
    media = TICKET / "🗑️generated/preview" / f"{name}-{language}-many"
    target = ROOT / ".🧬semio/🎓️teaching/⚡️cache/videos" / language / "speaker/preview" / f"{output}_speaker_{language}_480p15.mp4"
    started = time.time()
    scene_class = getattr(importlib.import_module(module), name)
    with manim.tempconfig({"pixel_width": 854, "pixel_height": 480, "frame_rate": 15, "write_to_movie": True, "disable_caching": True,
                           "media_dir": str(media), "verbosity": "WARNING", "progress_bar": "none"}):
        scene = scene_class()
        scene.render()
        rendered = Path(scene.renderer.file_writer.movie_file_path)
    staged = target.with_suffix(".new.mp4")
    shutil.copyfile(rendered, staged)
    os.replace(staged, target)
    stats = patched.STATS
    record = {"scene": name, "module": module, "language": language, "audience": "speaker", "size": [854, 480], "fps": 15,
              "seconds": round(scene.renderer.time, 2), "render_seconds": round(time.time() - started), "rendered_at": time.strftime("%Y-%m-%dT%H:%M:%S"),
              "sha256": hashlib.sha256(target.read_bytes()).hexdigest(), "patch": patch,
              "geometry_cache": {"source_sha256": stats["source"], "hits": stats["hits"], "misses": stats["misses"], "miss_seconds": round(stats["miss_seconds"])},
              "note": "preview from current code in one import snapshot with a persisted exact building geometry; not a certified master"}
    target.with_suffix(".json").write_text(json.dumps(record, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print("[DEBUG] preview", target.name, record["seconds"], "s", flush=True)
print("[DEBUG] all done", flush=True)
