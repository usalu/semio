"""📽️ Verify the primary-accent preview and capture a readable presentation frame."""

import hashlib
import importlib
import os
import shutil
import sys
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
import av
from PIL import Image

runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
ticket = Path(__file__).parent
video = ticket / "primary-accent.mp4"
if len(sys.argv) > 1:
    shutil.copy2(Path(sys.argv[1]), video)
frames = 0
last = -1.0
poster = None
with av.open(str(video)) as container:
    stream = container.streams.video[0]
    codec, width, height, rate = stream.codec_context.name, stream.width, stream.height, str(stream.average_rate)
    duration = float(stream.duration * stream.time_base)
    audio = len(container.streams.audio)
    for frame in container.decode(stream):
        moment = float(frame.time)
        if moment < last:
            raise AssertionError("Video timestamps are not monotonic")
        frames += 1
        last = moment
        if poster is None and moment >= 14:
            poster = frame.to_image()
assert codec == "h264" and (width, height) in {(640, 360), (960, 540)} and frames > 60
assert duration > 30
runtime.run_command([*runtime.ffmpeg_command(), "-v", "error", "-xerror", "-i", str(video), "-f", "null", "-"], log=Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) / "decode.log")
if poster is not None:
    poster.save(ticket / "primary-accent.png")
sha = hashlib.sha256(video.read_bytes()).hexdigest()
(ticket / "accent-render.md").write_text(
    "# Primary Accent Physical Review Draft\n\n"
    "The complete first physical fundamentals beat rendered after the primary accent fixes. "
    "This is a focused review draft; the previously delivered full physical.mp4 remains the earlier render.\n\n"
    f"File: primary-accent.mp4\n\nDuration: {duration:.3f} seconds; {width}×{height}; {rate} fps; {codec}; "
    f"{frames} fully decoded frames; {audio} audio streams. Size: {video.stat().st_size:,} bytes.\n\n"
    f"SHA-256: {sha}\n\n"
    "PyAV decoded every frame with monotonic timestamps. FFmpeg independently decoded the complete movie with error escalation enabled. "
    "The canonical runtime rejected cache certification because authored sources changed during rendering. "
    "A fresh render hit a separate Manim family interpolation error on the architectural house group. "
    "This earlier complete movie is retained solely for accent review; no canonical provenance sidecar was written or bypassed. "
    "Representative all-series frames and scientific color pairings passed independent review in accent-review.md.\n",
    encoding="utf-8",
)
print(f"[DEBUG] Verified primary-accent.mp4: {duration:.3f}s, {frames} frames, {width}×{height}", flush=True)
if sys.platform == "win32":
    os.startfile(str(video))
else:
    runtime.run_command(["open" if sys.platform == "darwin" else "xdg-open", str(video)])
