"""🔬 Reproduce finite chapter composition with one tiny body and real browser transitions."""
import importlib
import importlib.util
import json
from pathlib import Path
import sys

import av
import numpy as np

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
p = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🐍️")
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
work = TICKET / "🗑️generated/iv/real-ui-encoder-2"
work.mkdir(parents=True, exist_ok=True)
source = TICKET / "🗑️generated/icb/chapter-encode-ayl6sqs3/body.mp4"
output = work / "chapter.mp4"
settings = {"language": "de", "audience": "spectator", "resolution": "480,270", "fps": 4.0, "quality": "-ql"}
p._render_chapter("PhysicalFundamentals_FullSection", source, output, settings, runtime.ToolSession(), work)
with av.open(str(source)) as media:
    expected = [frame.to_ndarray(format="rgb24") for frame in media.decode(video=0)]
with av.open(str(output)) as media:
    frames = [frame.to_ndarray(format="rgb24") for frame in media.decode(video=0)]
result = {"expected_frames": 46, "decoded_frames": len(frames),
          "png_counts": {phase: len(list((work / phase).glob("*.png"))) for phase in ("i", "o")}}
assert len(frames) == 46, result
for actual, authored in zip(frames[26:31], expected, strict=True):
    np.testing.assert_array_equal(actual, authored)
result["body_exact"] = True
text = json.dumps(result, indent=2)
(TICKET / "inspection-chapter-real-ui-encoder.md").write_text("# Tiny Body With Actual UI Transitions\n\n" + "\n".join("    " + line for line in text.splitlines()) + "\n", encoding="utf-8")
print(text)
