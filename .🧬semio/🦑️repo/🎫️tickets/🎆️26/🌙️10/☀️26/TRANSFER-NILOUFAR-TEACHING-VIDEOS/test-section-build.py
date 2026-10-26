"""🧪 Exercise the real section build, silent mux, and concat interface."""
from pathlib import Path
import sys
import av

root = Path(sys.argv[1]).resolve()
ticket = Path(__file__).resolve().parent
sys.path.insert(0, str(root))
from tts_pipeline import build_section_video

output = Path(sys.argv[2]) if len(sys.argv) > 2 else build_section_video(root / "energy/demand/1_physical_fundamentals", [(ticket / "section-smoke.py", "MinimalScene", ticket / "missing-narration.mp3")], "section_smoke.mp4")
with av.open(str(output)) as container:
    assert len(container.streams.video) == 1
    assert len(container.streams.audio) == 1
    stream = container.streams.video[0]
    assert (stream.width, stream.height) == (1920, 1080)
    assert stream.average_rate.numerator / stream.average_rate.denominator == 60
    frames = sum(1 for _ in container.decode(video=0))
    assert frames == 30, frames
print("[DEBUG] Real section build passed: 1920x1080 at60fps, all30 frames decoded, optional silent audio present")
