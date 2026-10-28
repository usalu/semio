"""🎞️ Decode the complete physical lesson and verify it includes every published source."""

import importlib
import json
import os
import sys
from pathlib import Path

import av

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
movie = output / "rendered/Full_Physical_Fundamentals_640x360p6.mp4"
settings = {"quality": "-ql", "resolution": "640,360", "fps": 6.0}
jobs = [job for job in runtime.segment_plan() if job["video_id"] == "physical_fundamentals"]
expected_frames = 0
intro_duration = 0
for job in jobs:
    clip = output / "s" / str(job["source_ordinal"]) / "v" / (job["segment"]["scene"] + ".mp4")
    assert runtime.reusable_clip(clip, runtime.segment_state(job["segment"], settings)), str(clip)
    with av.open(clip) as source:
        expected_frames += source.streams.video[0].frames
        if job["source_ordinal"] == 1:
            intro_duration = float(source.duration / av.time_base)

poster = output / "physical-poster.png"
poster_written = False
frames = 0
previous = -1
with av.open(movie) as container:
    stream = container.streams.video[0]
    stream.codec_context.thread_count = 1
    assert (stream.width, stream.height) == (640, 360)
    assert float(stream.base_rate) == 6.0 and abs(float(stream.average_rate) - 6.0) < 0.001
    metadata = {"path": str(movie), "bytes": movie.stat().st_size, "width": stream.width,
                "height": stream.height, "fps": float(stream.base_rate), "average_fps": float(stream.average_rate), "codec": stream.codec_context.name,
                "duration_seconds": float(container.duration / av.time_base), "audio_streams": len(container.streams.audio)}
    for frame in container.decode(video=0):
        timestamp = float(frame.time)
        assert timestamp >= previous
        previous = timestamp
        frames += 1
        if not poster_written and timestamp >= intro_duration + 10:
            frame.to_image().save(poster)
            poster_written = True
assert frames == expected_frames and frames > 0
assert poster_written
metadata.update({"decoded_frames": frames, "expected_source_frames": expected_frames, "poster": str(poster)})
runtime.run_command([*runtime.ffmpeg_command(), "-hide_banner", "-loglevel", "error", "-nostdin", "-xerror", "-i", str(movie), "-f", "null", "-"], log=output / "verification-ffmpeg.log")
(output / "verification.json").write_text(json.dumps(metadata, ensure_ascii=False, indent=2), encoding="utf-8")
print("[DEBUG] Complete video passes PyAV source-frame count, monotonic timestamps, resolution/rate, and FFmpeg decode")
print(json.dumps(metadata, ensure_ascii=False))
