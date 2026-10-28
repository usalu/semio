"""🧬 Measure actual frame formats and decoder resets inside the physical raw assembly."""
import importlib
import json
from pathlib import Path
import re
import sys

import av

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
raw = TICKET / "🗑️generated/i/de/spectator/1/v/PhysicalFundamentals_FullSection.mp4"
work = TICKET / "🗑️generated/iv/decoder-formats"
work.mkdir(parents=True, exist_ok=True)
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
log = work / "debug.log"
if not log.is_file():
    runtime.run_command([*runtime.ffmpeg_command(), "-loglevel", "debug", "-i", str(raw),
        "-map", "0:v:0", "-vf", "showinfo", "-an", "-f", "null", "-"], log=log)
text = log.read_text(encoding="utf-8")
changes, previous = [], None
for line in text.splitlines():
    if "Parsed_showinfo" in line and "fmt:" in line:
        state = [re.search(pattern, line) for pattern in (r"fmt:(\S+)", r"sar:(\S+)", r"\bs:(\S+)")]
        ordinal = re.search(r"\bn:\s*(\d+)", line)
        values = tuple(part.group(1) for part in state) if all(state) else None
        if values and ordinal and values != previous:
            changes.append({"frame": int(ordinal.group(1)), "format_sar_size": values})
            previous = values
resets = [line for line in text.splitlines() if any(word in line.lower()
          for word in ("reinit", "changing video frame", "filtergraph reconfigured", "video frame properties"))]
native = []
previous = None
with av.open(str(raw)) as media:
    for index, frame in enumerate(media.decode(video=0)):
        state = [frame.width, frame.height, frame.format.name, frame.color_range, frame.colorspace]
        if state != previous:
            native.append({"frame": index, "state": state})
            previous = state
result = {"ffmpeg_showinfo_changes": changes, "decoder_resets": resets, "pyav_changes": native}
value = json.dumps(result, ensure_ascii=False, indent=2)
(TICKET / "inspection-decoder-formats.md").write_text("# Physical Raw Decoder Format Transitions\n\n" +
    "\n".join("    " + line for line in value.splitlines()) + "\n", encoding="utf-8")
print(value)
