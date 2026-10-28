"""🎬 Render the first complete physical beat through the canonical clip pipeline."""

import importlib
import os
import shutil
import sys
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
settings = {"quality": "-ql", "resolution": "640,360", "fps": 6}
segment = runtime.load_manifest()[0]["chapters"][0]["segments"][0]
clip = runtime.render_segment(segment, output / "p", settings, output_root=output, log_output=True)
final = Path(__file__).with_name("primary-accent.mp4")
shutil.copy2(clip, final)
print(f"[DEBUG] Primary accent physical preview: {final}", flush=True)
