"""🎞️ Independently decode and verify every frame of the calm cursor preview."""
from __future__ import annotations

import hashlib
import importlib
import json
import math
import sys
from pathlib import Path

sys.dont_write_bytecode = True
TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))

import av
import numpy as np
from PIL import Image

movie = Path(sys.argv[1]).resolve()
generated = (TICKET / "🗑️generated").resolve()
if not movie.is_relative_to(generated):
    raise ValueError("Verify the owned generated movie before publishing")
samples = generated / "calm-media-review"
samples.mkdir(exist_ok=True)
count = 0
first = last = None
with av.open(str(movie)) as container:
    stream = container.streams.video[0]
    assert stream.codec_context.name == "h264"
    assert (stream.width, stream.height) == (1600, 900)
    assert float(stream.average_rate) == 30
    assert abs(float(stream.duration * stream.time_base) - 11) < 1e-8
    assert not container.streams.audio
    for frame in container.decode(stream):
        assert abs(float(frame.time) - count / 30) < 1e-8
        pixels = frame.to_ndarray(format="rgb24")
        if count == 0:
            first = pixels.copy()
        if count in (0, 90, 120, 165, 209, 270, 329):
            frame.to_image().save(samples / f"{count:04d}.png")
        last = pixels
        count += 1
assert count == 330
source_frames = movie.parent / "f"
native_frames = sorted(source_frames.glob("*.png"))
assert len(native_frames) == 132
with Image.open(native_frames[0]) as source:
    source_first = np.asarray(source.convert("RGB"))
with Image.open(native_frames[-1]) as source:
    source_last = np.asarray(source.convert("RGB"))
source_mse = float(np.mean((source_first.astype(np.float64) - source_last.astype(np.float64)) ** 2))
source_psnr = math.inf if source_mse == 0 else 10 * math.log10(255 ** 2 / source_mse)
assert source_psnr >= 45, f"Source boundary similarity is too low: {source_psnr:.3f} dB"
mse = float(np.mean((first.astype(np.float64) - last.astype(np.float64)) ** 2))
psnr = math.inf if mse == 0 else 10 * math.log10(255 ** 2 / mse)
assert psnr >= 45, f"Encoded boundary similarity is too low: {psnr:.3f} dB"
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
runtime.run_command([*runtime.ffmpeg_command(), "-v", "error", "-xerror", "-i", str(movie),
                     "-f", "null", "-"], log=generated / "calm-media-decode.log")
sha = hashlib.sha256(movie.read_bytes()).hexdigest()
originals = json.loads((generated / "original-delivery-hashes.json").read_text(encoding="utf-8"))
for entry in originals:
    assert hashlib.sha256((ROOT / entry["path"]).read_bytes()).hexdigest() == entry["sha256"]
result = {"file": str(movie), "sha256": sha, "bytes": movie.stat().st_size,
          "frames": count, "fps": 30, "seconds": count / 30, "width": 1600, "height": 900,
          "nativeFrames": len(native_frames), "nativeFps": 12,
          "sourceFirstLastPixelsEqual": source_mse == 0, "sourceBoundaryPsnrDb": str(source_psnr), "encodedFirstLastPixelsEqual": mse == 0,
          "encodedBoundaryPsnrDb": str(psnr), "originalVideosUnchanged": len(originals)}
(generated / "calm-media-result.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
(TICKET / "cursor-media-verification.md").write_text(
    "# Calm Cursor Media Verification\n\n"
    "PyAV independently decoded all 330 H.264 frames at 1600×900 and 30fps. "
    "The exact timeline is 11 seconds with strictly increasing, evenly spaced timestamps. "
    f"The preview has no audio stream. It uses {len(native_frames)} native browser captures at 12fps. "
    f"NumPy measures source boundary similarity of {source_psnr:.3f} dB (45 dB minimum). "
    f"The encoded boundary similarity is {psnr:.3f} dB (45 dB minimum), allowing normal H.264 quantization. "
    "FFmpeg separately decoded the complete movie with error checking and exited successfully. "
    "Seven frames were extracted under the owned generated directory for independent visual review.\n\n"
    f"Movie SHA-256: {sha}\n\n"
    f"Encoded size: {movie.stat().st_size:,} bytes. All six completed optimization lesson files retain their pre-change SHA-256 hashes.\n\n"
    "The separate actual-DOM audit in cursor-calm-audit.md verifies one intended hover target, "
    "the three-second dwell across 36 native captures, empty native click/press/release traces, and unopened navigation layers. "
    "The focused preview does not regenerate or alter the six existing lesson deliveries.\n",
    encoding="utf-8")
print("[DEBUG] Calm media verified: 330 frames, 11 seconds, stable source boundaries, six original videos unchanged", flush=True)
