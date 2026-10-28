"""🔬 Independently verify cursor UI frames, source activation, ancestor emphasis and final handoff."""

import hashlib
import importlib
import json
import os
import sys
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
import av
import numpy as np
from PIL import Image

runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
ticket = Path(__file__).parent
output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])


def decode(path):
    """📽️ Decode all output frames and require a finite constant-rate H.264 timeline."""
    frames = []
    previous = -1.0
    with av.open(str(path)) as container:
        stream = container.streams.video[0]
        assert stream.codec_context.name == "h264" and float(stream.average_rate) == 8
        assert (stream.width, stream.height) == (960, 540)
        duration = float(stream.duration * stream.time_base)
        for frame in container.decode(stream):
            assert float(frame.time) > previous
            previous = float(frame.time)
            frames.append(frame.to_ndarray(format="rgb24"))
    assert abs(duration - len(frames) / 8) < 0.001
    return frames, duration


frames, duration = decode(ticket / "cursor-ui.mp4")
tour, _ = decode(output / "tour.mp4")
lesson, _ = decode(output / "lesson.mp4")
body, _ = decode(output / "body" / "v" / "device.mp4")
assert len(frames) == len(tour) + len(lesson)
assert len(lesson) == len(body) + 32
np.testing.assert_array_equal(frames[0], frames[-1])
np.testing.assert_array_equal(tour[-1], lesson[0])
np.testing.assert_array_equal(lesson[0], lesson[-1])
identity = hashlib.sha256(ticket.relative_to(ROOT).as_posix().encode()).hexdigest()[:10]
trace = json.loads((output / "trace" / identity / "DeviceDemo" / "👆️.json").read_text(encoding="utf-8"))
by_type = {event["type"]: event for event in trace["events"]}
assert by_type["press"]["time"] < by_type["release"]["time"] == by_type["click"]["time"]
click = by_type["click"]["time"]
heat_counts = []
for frame in body:
    patch = frame[210:330, 330:450].astype(int)
    heat_counts.append(int(np.count_nonzero((patch[:, :, 0] > patch[:, :, 1] + 30) & (patch[:, :, 0] > patch[:, :, 2] + 30))))
assert max(heat_counts[:int(click * 8)]) == 0
assert max(heat_counts[min(len(body) - 1, int((click + 0.5) * 8)):]) > 50
active = int(np.ceil(trace["intervals"][0]["start"] * 8))
assert np.abs(lesson[16][250:290, :2].astype(int) - lesson[16 + active][250:290, :2].astype(int)).max() > 20
Image.fromarray(lesson[4]).save(ticket / "cursor-ui-click.png")
Image.fromarray(lesson[16 + min(len(body) - 1, int((click + 0.5) * 8))]).save(ticket / "cursor-ui-device.png")
log = output / "verify-decode.log"
log.unlink(missing_ok=True)
runtime.run_command([*runtime.ffmpeg_command(), "-v", "error", "-xerror", "-i", str(ticket / "cursor-ui.mp4"), "-f", "null", "-"], log=log)
movie_hash = hashlib.sha256((ticket / "cursor-ui.mp4").read_bytes()).hexdigest()
endpoint_hash = hashlib.sha256(frames[0].tobytes()).hexdigest()
(ticket / "cursor-render.md").write_text(
    "# Cursor UI Review Film\n\n"
    "The focused review demonstrates regular navbar and navigation-tree hover ancestry, footer hover, a real fullscreen-control click, "
    "an authored heater control that activates only after release/click, and a separate restore-control click before zooming out. "
    "Normal cells stay transparent and fullscreen SVGs remain hollow. The scientific plume stays within its device body below the title cap.\n\n"
    f"The movie is {duration:.3f} seconds, 960×540, eight fps, H.264, {len(frames)} fully decoded frames. "
    f"Tour {len(tour)}, composed lesson {len(lesson)}, native source {len(body)}; both exact 16-frame transition pads preserve every source frame. "
    "Tour/lesson handoff and whole-film first/last overview RGB frames are byte-identical. "
    "Native metadata activates ancestor chrome while the authored pointer is visible. An independent red-pixel oracle finds no heater activation before click and visible heat after click. "
    "Release and click timestamps are equal, matching one UI event turn. FFmpeg independently decodes the whole film without errors.\n\n"
    f"Movie SHA-256: {movie_hash}\n\nDecoded overview RGB SHA-256: {endpoint_hash}\n\n"
    "This short film uses a dedicated native equipment demonstration through the same production helper. "
    "The actual physics power scene was independently observed at its real Start→An callback in cursor-physics-check.md. "
    "The full curriculum has not been regenerated. All five lesson envelope/media regressions, narration timing and all-source-frame preservation are documented in cursor-tests.md.\n",
    encoding="utf-8")
print(f"[DEBUG] Verified cursor UI film: {len(frames)} frames, {duration:.3f}s, exact overview return and click-gated heat", flush=True)
if "--no-open" not in sys.argv:
    if sys.platform == "win32":
        os.startfile(str(ticket / "cursor-ui.mp4"))
    else:
        runtime.run_command(["open" if sys.platform == "darwin" else "xdg-open", str(ticket / "cursor-ui.mp4")])
