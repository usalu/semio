"""🔬 Inspect every raw EB07 decoded frame through the existing FFmpeg interface."""
from pathlib import Path
import importlib
import json
import re
import sys

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
ticket = Path(__file__).resolve().parent
work = ticket / "🗑️generated/ie"
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
raw = ticket / "🗑️generated/i/de/spectator/5/v/EnergyBalance_07_TheorieVsPraxis.mp4"
session = runtime.ToolSession()
runtime.run_command([*runtime.ffmpeg_command(), "-hide_banner", "-i", str(raw), "-map", "0:v:0", "-vf", "showinfo", "-an", "-f", "null", "-"], session=session, log=work / "raw-params.log")
frames, changes, previous = [], [], None
for line in (work / "raw-params.log").read_text(encoding="utf-8").splitlines():
    if "[Parsed_showinfo" not in line:
        continue
    if " n:" in line:
        match = re.search(r"fmt:(\S+).*?sar:(\S+).*?s:(\S+)", line)
        if match:
            frames.append(dict(fmt=match[1], sar=match[2], size=match[3]))
    elif "color_range:" in line and frames:
        frames[-1].update(dict(re.findall(r"(color_\w+):(\S+)", line)))
for index, frame in enumerate(frames):
    if frame != previous:
        changes.append(dict(frame=index, parameters=frame))
        previous = frame
result = dict(decoded_frames=len(frames), parameter_changes=changes)
(work / "raw-params.json").write_text(json.dumps(result, indent=2), encoding="utf-8")
print("[DEBUG] " + json.dumps(result))
