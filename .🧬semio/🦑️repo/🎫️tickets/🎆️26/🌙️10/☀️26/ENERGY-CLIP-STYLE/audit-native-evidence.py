"""🎞️ Audit encoded native evidence independently without repeating presentation renders."""

import hashlib
import importlib
import json
import os
import sys
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
OUTPUT = Path(__file__).with_name("🗑️generated") / "virtual-ui-tests" / "native-evidence"
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(OUTPUT)
sys.stdout.reconfigure(encoding="utf-8")
import av
import numpy as np
from PIL import Image, ImageChops

runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
source = OUTPUT / "source.mp4"
runtime.run_command([*runtime.ffmpeg_command(), "-y", "-v", "error", "-f", "lavfi", "-i", "testsrc2=s=320x180:r=4:d=2",
                     "-f", "lavfi", "-i", "sine=frequency=440:sample_rate=48000:duration=2",
                     "-c:v", "libx264", "-preset", "ultrafast", "-crf", "0", "-pix_fmt", "yuv420p",
                     "-c:a", "aac", "-b:a", "192k", str(source)])
with av.open(str(source)) as container:
    originals = [np.asarray(frame.to_image().convert("RGB").resize((248, 140), Image.Resampling.BICUBIC)).astype(float)[3:-3, 3:-3]
                 for frame in container.decode(video=0)]
tests = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🧪️tests.🐍️")
validator = tests.PresentationNativeTest()
validator.presentation = importlib.import_module(tests.MODULE + ".🪟️presentation.🐍️")
results = []
for video in ("physical_fundamentals", "ventilation", "heating", "cooling", "energy_balance"):
    with av.open(str(OUTPUT / (video + ".mp4"))) as container:
        frames = [frame.to_image().convert("RGB") for frame in container.decode(video=0)]
    with av.open(str(OUTPUT / (video + ".mp4"))) as container:
        rate = container.streams.audio[0].codec_context.sample_rate
        audio = np.concatenate([frame.to_ndarray().astype(float).mean(axis=0) for frame in container.decode(audio=0)])
    middle = audio[round(2.2 * rate):round(3.8 * rate)]
    rms = lambda samples: float(np.sqrt(np.mean(np.square(samples))))
    frequencies = np.fft.rfftfreq(len(middle), d=1 / rate)
    actual = np.asarray(frames[12].crop((36, 29, 284, 169))).astype(float)[3:-3, 3:-3]
    errors = [float(np.mean(np.abs(actual - expected))) for expected in originals]
    record = {"video": video, "frames": len(frames), "boundary_equal": ImageChops.difference(frames[0], frames[-1]).getbbox() is None,
              "first_rgb_sha256": hashlib.sha256(frames[0].tobytes()).hexdigest(),
              "last_rgb_sha256": hashlib.sha256(frames[-1].tobytes()).hexdigest(),
              "audio_seconds": len(audio) / rate, "prefix_rms": rms(audio[:round(1.9 * rate)]),
              "suffix_rms": rms(audio[round(4.1 * rate):]), "middle_rms": rms(middle),
              "tone_hz": float(frequencies[np.argmax(np.abs(np.fft.rfft(middle)))]),
              "focus_source_frame_mae": errors}
    matrix = validator.assert_contained_source_frames(source, frames, tests.CASES["smoke"], 2)
    record["body_source_frame_order"] = [int(np.argmin(row)) for row in matrix]
    record["body_source_frame_mae"] = [row[index] for index, row in enumerate(matrix)]
    results.append(record)
(OUTPUT / "audit.json").write_text(json.dumps(results, indent=2) + "\n", encoding="utf-8")
print(json.dumps(results, indent=2))
