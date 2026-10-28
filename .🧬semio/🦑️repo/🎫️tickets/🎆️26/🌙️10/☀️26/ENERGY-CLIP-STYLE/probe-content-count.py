"""🧮 Compare decoded and copied packet counts for portable MP4 edit-list fixtures."""

import importlib
import json
import os
import re
import sys
from pathlib import Path

import av

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
WORK = Path(__file__).parent / "🗑️generated/content-camera-tests/count"
WORK.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(WORK)
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
ffmpeg = runtime.ffmpeg_command()
base = WORK / "base.mp4"
runtime.run_command([*ffmpeg, "-y", "-v", "error", "-f", "lavfi", "-i", "testsrc2=s=320x180:r=8:d=2",
                     "-c:v", "libx264", "-preset", "fast", "-crf", "0", "-pix_fmt", "yuv420p", str(base)])
results = []
for seek in (0.0625, 0.125, 0.1875, 0.25, 0.375):
    clip = WORK / f"clip-{seek}.mp4"
    runtime.run_command([*ffmpeg, "-y", "-v", "error", "-ss", str(seek), "-i", str(base), "-t", "1.5", "-c", "copy", str(clip)])
    counts = {}
    for name, codec in (("copied", ["-c:v", "copy"]), ("decoded", [])):
        log = WORK / f"{seek}-{name}.log"
        runtime.run_command([*ffmpeg, "-hide_banner", "-i", str(clip), "-map", "0:v:0", *codec, "-f", "null", "-"], log=log)
        counts[name] = int(re.findall(r"frame=\s*(\d+)", log.read_text(encoding="utf-8"))[-1])
    with av.open(str(clip)) as container:
        counts["pyav"] = sum(1 for frame in container.decode(video=0))
    results.append({"seek": seek, **counts})
print(json.dumps(results))
(Path(__file__).parent / "content-frame-count-fixture.md").write_text("# Content Frame Count Fixture\n\n```json\n" + json.dumps(results, indent=2) + "\n```\n", encoding="utf-8")
