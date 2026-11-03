"""🧪 Smoke-test posters, thumbnails and provenance on an existing clip: selftest.py <mp4> <video-id> <Scene>."""
import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent))
import lib
mp4, video, scene = Path(sys.argv[1]), sys.argv[2], sys.argv[3]
out = lib.CACHE.parent / "../.." / "x"
out = Path(sys.argv[4]) if len(sys.argv) > 4 else Path(".")
made = lib.stills(mp4, out, "selftest")
print(made["poster"], made["thumbnail"], round(lib.duration(mp4), 1))
rt = lib.runtime()
job = next(j for j in rt.segment_plan() if j["video_id"] == video and j["segment"]["scene"] == scene)
state = rt.segment_state({k: job["segment"][k] for k in ("script", "scene")}, lib.settings("en", "speaker", lib.QUALITY, lib.FPS))
p = lib.provenance(dict(state, casts=dict(list(state["casts"].items())[:2])))
import json
print(json.dumps(p, indent=1, ensure_ascii=False)[:1500])
