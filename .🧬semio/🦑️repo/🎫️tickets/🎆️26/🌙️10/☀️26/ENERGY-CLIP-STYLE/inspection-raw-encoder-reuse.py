"""🔬 Reuse actual finite UI sequences to isolate the long physical raw encoder input."""
import importlib
import importlib.util
import json
from pathlib import Path
import sys
from unittest.mock import patch

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
spec = importlib.util.spec_from_file_location("collection_verify", TICKET / "inspection-chapter-verify.py")
v = importlib.util.module_from_spec(spec)
spec.loader.exec_module(v)
p = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🐍️")
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
raw = v.BASE / "1/v/PhysicalFundamentals_FullSection.mp4"
images = TICKET / "🗑️generated/iv/real-ui-encoder-2"
work = TICKET / "🗑️generated/iv/raw-encoder-reuse"
work.mkdir(parents=True, exist_ok=True)
output = work / "chapter.mp4"
settings = {"language": "de", "audience": "spectator", "resolution": "480,270", "fps": 4, "quality": "-ql"}


def transition(scene, snapshot, directory, settings, session, *, phase):
    """🖼️ Reuse only the already rendered real UI raster sequences without changing their bytes."""
    return (images / "i", 26) if phase == "focus" else (images / "o", 15)


with patch.object(p, "_transition", side_effect=transition):
    p._render_chapter("PhysicalFundamentals_FullSection", raw, output, settings, runtime.ToolSession(), work)
measured = v.STREAMS.info(output)
result = {"expected_frames": v.STREAMS.info(raw)["frames"] + 41, "actual": measured}
value = json.dumps(result, ensure_ascii=False, indent=2)
(TICKET / "inspection-raw-encoder-reuse.md").write_text("# Long Raw Encoder With Reused Actual UI Sequences\n\n" +
    "\n".join("    " + line for line in value.splitlines()) + "\n", encoding="utf-8")
print(value, flush=True)
assert measured["frames"] == result["expected_frames"]
