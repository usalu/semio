"""🎬 Render the complete physical chapter with the current four-level UI."""
import importlib
import os
from pathlib import Path
import sys

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
OUTPUT = TICKET / "🗑️generated/i"
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
os.environ.update(PYTHONUTF8="1", PYTHONIOENCODING="utf-8", SEMIO_TEACHING_OUTPUT_ROOT=str(OUTPUT))
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
result = runtime.compose_videos(["physical_fundamentals"], language="de", audience="spectator", quality_flag="-ql", output_root=OUTPUT, resolution="480,270", fps=4.0, jobs=1, log_output=True, concat_only=True, play=False)
print("[DEBUG] Four-level complete physical result", result, flush=True)
