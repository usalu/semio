"""🖼️ Save evenly spaced frames of rendered masters into 🗑️generated/frames for a visual spot check.

Usage: python frame-probe.py <language> <audience> <file-prefix> [count]
"""
import sys
from pathlib import Path

import av

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
language, audience, prefix = sys.argv[1:4]
count = int(sys.argv[4]) if len(sys.argv) > 4 else 3
OUT = TICKET / "🗑️generated" / "frames"
OUT.mkdir(parents=True, exist_ok=True)
for master in sorted((ROOT / ".🧬semio/🎓️teaching/⚡️cache/videos" / language / audience / "rendered").glob(f"{prefix}*.mp4")):
    with av.open(str(master)) as container:
        stream = container.streams.video[0]
        total = stream.frames or int(stream.duration * stream.time_base * 30)
        wanted = {round(total * (i + 1) / (count + 1)) for i in range(count)}
        for index, frame in enumerate(container.decode(stream)):
            if index in wanted:
                target = OUT / f"{master.stem}-{index:05d}.png"
                frame.to_image().resize((960, 540)).save(target)
                print("[DEBUG] saved", target.name, flush=True)
            if index >= max(wanted):
                break
