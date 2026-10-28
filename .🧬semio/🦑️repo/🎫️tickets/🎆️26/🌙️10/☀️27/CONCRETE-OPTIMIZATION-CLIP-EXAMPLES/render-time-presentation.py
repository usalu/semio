"""⏱️ Time the lesson-window presentation wrapper on a finished segment: python render-time-presentation.py <video_id> <content.mp4> <output.mp4> [copies]"""
import importlib
import sys
import time
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
video_id, content, output = sys.argv[1], Path(sys.argv[2]), Path(sys.argv[3])
copies = int(sys.argv[4]) if len(sys.argv) > 4 else 1
settings = {"language": "en", "quality": "-qh", "resolution": None, "fps": 30.0, "audience": "speaker"}


def run(index: int) -> float:
    started = time.monotonic()
    runtime.render_presentation(video_id, content, output.with_name(f"{output.stem}-{index}{output.suffix}"), settings)
    return time.monotonic() - started


started = time.monotonic()
with ThreadPoolExecutor(copies) as pool:
    times = list(pool.map(run, range(copies)))
print("[DEBUG] presentation seconds", [round(value, 1) for value in times], "wall", round(time.monotonic() - started, 1), flush=True)
