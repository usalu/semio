import importlib
import os
import sys
import time
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
root = Path(__file__).resolve().parents[8]
sys.path.insert(0, str(root))
out = Path(__file__).resolve().parents[1] / "🗑️generated" / "consistency-out" / "frames"
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(out)
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
profile = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🎚️profile.🐍️")
scene, script, language, resolution = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
extra = sys.argv[5:]
script_path = Path(script) if Path(script).is_absolute() else root / "🎓️teaching" / script
media = out / "m" / f"{scene}-{language}-{resolution.replace(',', 'x')}"
config = runtime.render_config(media)
chosen = profile.profile("preview")
command = [*runtime.manim_command(), "--config_file", str(config), "--media_dir", str(media), "--resolution", resolution, "--fps", f"{chosen.fps:g}",
           "--disable_caching", "-s", *extra, str(script_path), scene]
environment = {"SEMIO_TEACHING_OUTPUT_ROOT": str(out), "SEMIO_TEACHING_SPEECH_ROOT": str(runtime.source_speech_root(script_path, language=language).parent),
               "SEMIO_TEACHING_LANGUAGE": language, "SEMIO_TEACHING_AUDIENCE": "speaker",
               **profile.environment("preview", job_id=f"preview-{scene}-{time.time_ns():x}", fps=chosen.fps, resolution=resolution),
               "SEMIO_TEACHING_WORK": str(media / "work.json")}
started = time.time()
runtime.run_command(command, cwd=root / "🎓️teaching", env=environment, log=media / f"try-{time.time_ns()}.log")
print("seconds", time.time() - started)
print("\n".join(sorted(str(p) for p in media.rglob("*.png"))))
