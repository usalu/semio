"""🎞️ Cross-check failed physical clocks with native FFmpeg and H.264 packet metadata."""
import importlib
import importlib.util
import json
from pathlib import Path
import sys

import av

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
spec = importlib.util.spec_from_file_location("collection_verify", TICKET / "inspection-chapter-verify.py")
v = importlib.util.module_from_spec(spec)
spec.loader.exec_module(v)
collection = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.📚️collection.🐍️")
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
work = TICKET / "🗑️generated/iv/clock-native"
work.mkdir(parents=True, exist_ok=True)
settings = {"language": "de", "audience": "spectator", "resolution": "480,270", "fps": 4, "quality": "-ql"}
result = []
for path in (v.chapter_path("PhysicalFundamentals_FullSection"), v.BASE / "rendered/Full_Physical_Fundamentals_spectator_de_480x270p4.mp4"):
    native = collection.body_frames(path, settings, work, runtime.ToolSession())
    with av.open(str(path)) as media:
        stream = media.streams.video[0]
        packets = [(packet.pts, packet.dts, packet.duration) for packet in media.demux(stream) if packet.size]
        result.append({"path": str(path), "native_ffmpeg_frames": native, "header_frames": stream.frames,
                       "duration": float(stream.duration * stream.time_base), "packets": len(packets),
                       "first_packets": packets[:4], "last_packets": packets[-4:]})
text = json.dumps(result, indent=2)
(TICKET / "inspection-chapter-native-clock.md").write_text("# Native Physical Clock Cross-check\n\n" + "\n".join("    " + line for line in text.splitlines()) + "\n", encoding="utf-8")
print(text)
