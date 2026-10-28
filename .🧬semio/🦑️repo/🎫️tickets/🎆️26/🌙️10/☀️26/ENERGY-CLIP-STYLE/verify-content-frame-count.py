"""🧮 Compare compressed packet and decoded source frame counts without editing the input."""
from pathlib import Path
import hashlib
import importlib
import os
import re
import sys

ROOT = Path(__file__).resolve().parents[7]
sys.path.insert(0, str(ROOT))
TICKET = Path(__file__).parent
WORK = TICKET / "🗑️generated/content-camera/frame-count"
WORK.mkdir(parents=True, exist_ok=True)
INPUT = TICKET / "🗑️generated/content-camera-preview/captured-body.mp4"
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
before = hashlib.sha256(INPUT.read_bytes()).hexdigest()
counts = {}
for kind in ("copy", "decode"):
    log = WORK / (kind + ".log")
    command = runtime.ffmpeg_command() + ["-hide_banner", "-y", "-i", str(INPUT), "-map", "0:v:0", "-an"]
    command += ["-c:v", "copy"] if kind == "copy" else []
    runtime.run_command(command + ["-f", "null", "-"], log=log)
    matches = re.findall(r"frame=\s*(\d+)", log.read_text(encoding="utf-8"))
    counts[kind] = int(matches[-1])
import av
with av.open(str(INPUT)) as container:
    counts["pyav"] = sum(1 for frame in container.decode(video=0))
if hashlib.sha256(INPUT.read_bytes()).hexdigest() != before:
    raise RuntimeError("The retained source input changed during read-only frame verification")
text = "# Decoded Source Frame Count\n\nThe retained real source was read without modification. Its SHA-256 is `" + before + "`.\n\n"
text += "FFmpeg compressed copy probe: " + str(counts["copy"]) + " frames. FFmpeg decode probe: " + str(counts["decode"]) + " frames. Independent PyAV decode: " + str(counts["pyav"]) + " frames.\n\n"
text += "The composition frame count must use decoded video frames. Compressed packets can include a frame excluded by the container edit list. The probe must decode through the private FFmpeg interface before computing body duration and audio padding.\n"
(TICKET / "content-frame-count.md").write_text(text, encoding="utf-8")
print("[DEBUG] Source frame counts:", counts)
