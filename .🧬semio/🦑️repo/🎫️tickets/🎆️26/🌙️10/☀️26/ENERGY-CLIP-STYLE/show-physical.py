"""📽️ Verify, retain and open the requested physical-fundamentals draft."""

import hashlib
import importlib
import os
import shutil
import sys
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
import av

TICKET = Path(__file__).resolve().parent
source = TICKET / "🗑️generated/o/rendered/Full_Physical_Fundamentals_640x360p6.mp4"
if not source.is_file() or source.stat().st_size < 1000:
    raise RuntimeError("Completed physical video is missing")
with av.open(str(source)) as video:
    stream = video.streams.video[0]
    width, height = stream.width, stream.height
    duration = float(video.duration / av.time_base) if video.duration else float(stream.duration * stream.time_base)
    codec = stream.codec_context.name
    fps = float(stream.average_rate)
    if (width, height) != (640, 360) or codec != "h264" or duration < 60:
        raise RuntimeError(f"Unexpected video profile: {width}x{height}, {codec}, {duration}")
    first = next(video.decode(stream))
    video.seek(int(min(duration / 2, 120) * av.time_base))
    middle = next(video.decode(stream))
    middle.to_image().save(TICKET / "🗑️generated/physical-check.png")
    audio = bool(video.streams.audio)
staged = TICKET / "🗑️generated/physical.mp4"
shutil.copyfile(source, staged)
digest = hashlib.sha256(staged.read_bytes()).hexdigest()
if digest != hashlib.sha256(source.read_bytes()).hexdigest():
    raise RuntimeError("Video copy verification failed")
final = TICKET / "physical.mp4"
staged.replace(final)
minutes, seconds = divmod(round(duration), 60)
(TICKET / "physical-preview.md").write_text(f"""# Physical Video Preview

Complete physical fundamentals draft, including the institute intro and all eight authored beats.

- File: [physical.mp4](./physical.mp4)
- Duration: {minutes}:{seconds:02d}
- Profile: {width} × {height}, {fps:.3f} fps, H.264
- Bytes: {final.stat().st_size}
- SHA-256: {digest}
- Audio stream: {audio}; no new narration was generated.

The first frame and a later decoded sample were verified with PyAV. The requested video is retained as a deliverable; transient rendering caches and logs may be removed.
""", encoding="utf-8")

runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
if sys.platform == "win32":
    os.startfile(str(final))
else:
    runtime.run_command(["open" if sys.platform == "darwin" else "xdg-open", str(final)])
print(f"[DEBUG] Opened {final} ({minutes}:{seconds:02d}, {width}x{height})", flush=True)
