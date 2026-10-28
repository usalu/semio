"""⏱️ Time a Manim animation range of one optimization scene at a given quality, optionally several copies in parallel.

Usage: python render-time-segment.py <topic-folder> <SceneClass> <q: l|h> <start,end> <copies> <label>
"""
import json
import os
import subprocess
import sys
import time
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
T = Path(__file__).resolve().parent
topic, scene, quality, window, copies, label = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4], int(sys.argv[5]), sys.argv[6]
script = ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip" / topic / "🐍️.py"
python = ROOT / ".🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe"
base = T / "🗑️generated/render-dry/time" / label
processes = []
started = time.monotonic()
for index in range(copies):
    media = base / str(index)
    media.mkdir(parents=True, exist_ok=True)
    config = media / "render.cfg"
    config.write_text(f"[CLI]\nmedia_dir = {media.as_posix()}\nvideo_dir = {{media_dir}}/v\npartial_movie_dir = {{media_dir}}/p/{{scene_name}}\n"
                      f"images_dir = {{media_dir}}/i\ntext_dir = {{media_dir}}/t\ntex_dir = {{media_dir}}/x\nlog_dir = {{media_dir}}/logs\nprogress_bar = none\nverbosity = WARNING\n", encoding="utf-8")
    command = [str(python), "-m", "manim", "--config_file", str(config), "-q" + quality, "--media_dir", str(media), "-n", window]
    if quality == "h":
        command.extend(["--fps", "30"])
    command.extend(["--disable_caching", str(script), scene])
    env = {**os.environ, "PYTHONUTF8": "1", "SEMIO_TEACHING_OUTPUT_ROOT": str(media), "VO_TRACE": "1", "SEMIO_TEACHING_LANGUAGE": "en", "SEMIO_TEACHING_AUDIENCE": "speaker"}
    processes.append((media, subprocess.Popen(command, cwd=ROOT, env=env, stdout=(media / "out.log").open("w"), stderr=subprocess.STDOUT)))
codes = [process.wait() for _, process in processes]
elapsed = time.monotonic() - started
print(json.dumps({"label": label, "quality": quality, "window": window, "copies": copies, "codes": codes, "wall": round(elapsed, 1)}))
