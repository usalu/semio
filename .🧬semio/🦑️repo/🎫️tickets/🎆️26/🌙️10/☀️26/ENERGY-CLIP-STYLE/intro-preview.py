"""🎬 Render the shared institute intro with semio chrome for visual review."""

import importlib
import importlib.util
import os
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
from manim import tempconfig

runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
segment = runtime.load_manifest()[1]["intro"]
spec = importlib.util.spec_from_file_location("qa_intro", runtime.teaching_root(Path(runtime.__file__)) / segment["script"])
module = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = module
spec.loader.exec_module(module)
out = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) / "intro"
runtime.render_config(out)
with tempconfig({"media_dir": str(out), "text_dir": str(out / "t"), "images_dir": str(out / "i"), "pixel_width": 1280, "pixel_height": 720, "frame_rate": 5, "skip_animations": True, "write_to_movie": False, "save_last_frame": True, "disable_caching": True, "verbosity": "ERROR", "progress_bar": "none"}):
    scene = getattr(module, segment["scene"])()
    scene.render()
print(f"[DEBUG] Intro proof: {out}", flush=True)
