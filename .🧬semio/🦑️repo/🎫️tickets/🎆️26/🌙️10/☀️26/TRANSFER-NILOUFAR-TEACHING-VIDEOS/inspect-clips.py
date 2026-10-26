"""📐 Inspect completed clip stream metadata for final publication validation."""

import json
from pathlib import Path
import av

generated = Path(__file__).resolve().parent / "🗑️generated"
results = []
for clip in sorted((generated / "videos").glob("*/v/*.mp4")):
    with av.open(clip) as container:
        stream = container.streams.video[0]
        item = {"path": str(clip), "width": stream.width, "height": stream.height,
                "average_rate": str(stream.average_rate), "base_rate": str(stream.base_rate),
                "time_base": str(stream.time_base), "frames": stream.frames, "duration": stream.duration}
        results.append(item)
        print("[DEBUG]", clip.parent.parent.name, clip.name, item["width"], item["height"], item["average_rate"], flush=True)
(generated / "clip-streams.json").write_text(json.dumps(results, indent=2) + "\n", encoding="utf-8")
