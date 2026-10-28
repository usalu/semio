"""🔬 Verify the native virtual UI review film and its exact shared handoff frame."""

import hashlib
import importlib
import os
import sys
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
import av
import numpy as np

runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
ticket = Path(__file__).parent
output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])


def decode(path):
    """📽️ Decode every frame, retain endpoint identity and compare the fixed navigation bands."""
    frames = []
    with av.open(str(path)) as container:
        stream = container.streams.video[0]
        rate, codec, width, height = float(stream.average_rate), stream.codec_context.name, stream.width, stream.height
        duration = float(stream.duration * stream.time_base)
        for frame in container.decode(stream):
            frames.append(frame.to_ndarray(format="rgb24"))
    assert frames and codec == "h264" and rate == 6 and (width, height) == (960, 540)
    first, last = frames[0], frames[-1]
    np.testing.assert_array_equal(first, last)
    nav, footer = round(60 * height / 900) - 2, round(34 * height / 900) - 2
    for frame in frames:
        np.testing.assert_array_equal(first[:nav], frame[:nav])
        np.testing.assert_array_equal(first[-footer:], frame[-footer:])
    return frames, duration


parts = []
metadata = []
for index in (0, 1):
    path = output / str(index) / "window.mp4"
    frames, duration = decode(path)
    parts.append(frames)
    metadata.append((len(frames), duration))
np.testing.assert_array_equal(parts[0][-1], parts[1][0])
frames, duration = decode(ticket / "virtual-ui.mp4")
assert len(frames) == sum(count for count, _ in metadata)
np.testing.assert_array_equal(frames[len(parts[0]) - 1], frames[len(parts[0])])
from PIL import Image
Image.fromarray(frames[18]).save(ticket / "virtual-ui-focused.png")
log = output / "decode.log"
log.unlink(missing_ok=True)
runtime.run_command([*runtime.ffmpeg_command(), "-v", "error", "-xerror", "-i", str(ticket / "virtual-ui.mp4"), "-f", "null", "-"], log=log)
movie_hash = hashlib.sha256((ticket / "virtual-ui.mp4").read_bytes()).hexdigest()
boundary_hash = hashlib.sha256(frames[0].tobytes()).hexdigest()
(ticket / "virtual-ui-render.md").write_text(
    "# Virtual UI Handoff Review\n\n"
    "The review film demonstrates physical understanding followed by ventilation. "
    "Its bodies contain two current native authored clause snapshots per lesson; this is a focused UI demonstration, not a regenerated full curriculum. "
    "Physical content uses the power-first scene; ventilation uses the volume scene. All original source-frame content is contained inside its lesson window.\n\n"
    f"The joined film has {len(frames)} fully decoded H.264 frames at 960×540, six fps, {duration:.6f} seconds. "
    f"Component frame counts and durations: {metadata}. All fixed navbar/footer pixels match throughout. "
    "Both component first/last frames and the adjacent physical-last/ventilation-first frames are byte-identical in decoded RGB. "
    "The joined frame count equals the exact sum of both components, and FFmpeg independently decodes the entire film without errors.\n\n"
    f"Video SHA-256: {movie_hash}\n\nDecoded overview RGB SHA-256: {boundary_hash}\n\n"
    "The production envelope is tested separately against all five lesson IDs with real synthetic narrated sources, frame counts, waveform timing and native geometry. "
    "See virtual-ui-tests.md and virtual-ui-audit.md. Review media is retained; temporary render caches, logs and PNG sequences are removed at ticket close.\n",
    encoding="utf-8",
)
print(f"[DEBUG] Virtual UI film verified: {len(frames)} frames, {duration:.6f}s, exact shared handoff", flush=True)
if sys.platform == "win32":
    os.startfile(str(ticket / "virtual-ui.mp4"))
else:
    runtime.run_command(["open" if sys.platform == "darwin" else "xdg-open", str(ticket / "virtual-ui.mp4")])
