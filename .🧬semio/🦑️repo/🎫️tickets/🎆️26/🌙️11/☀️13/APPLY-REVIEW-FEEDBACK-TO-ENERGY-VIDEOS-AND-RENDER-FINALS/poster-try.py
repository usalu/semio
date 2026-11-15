import importlib
import json
import os
import sys
import time
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
root = Path(__file__).resolve().parents[7]
sys.path.insert(0, str(root))
out = Path(__file__).resolve().parent / "🗑️generated" / "poster-try"
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(out)
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
profile = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🐍️")
scene = sys.argv[1]
script = sys.argv[2]
extra = sys.argv[3:]
media = out / "m" / scene
config = runtime.render_config(media)
chosen = profile.profile("preview")
command = [*runtime.manim_command(), "--config_file", str(config), "--media_dir", str(media), "--resolution", f"{chosen.width},{chosen.height}", "--fps", f"{chosen.fps:g}",
           "--disable_caching", "-s", *extra, str(root / "🎓️teaching" / script), scene]
environment = {"SEMIO_TEACHING_OUTPUT_ROOT": str(out), "SEMIO_TEACHING_SPEECH_ROOT": str(runtime.source_speech_root(root / "🎓️teaching" / script, language="de").parent),
               "SEMIO_TEACHING_LANGUAGE": "de", "SEMIO_TEACHING_AUDIENCE": "speaker",
               **profile.environment("preview", job_id=f"preview-{scene}-{time.time_ns():x}", fps=chosen.fps, resolution=f"{chosen.width},{chosen.height}"),
               "SEMIO_TEACHING_WORK": str(media / "work.json")}
started = time.time()
runtime.run_command(command, cwd=root / "🎓️teaching", env=environment, log=media / f"try-{time.time_ns()}.log")
print("seconds", time.time() - started)
print(sorted(str(p) for p in media.rglob("*.png")))
