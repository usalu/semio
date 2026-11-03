"""🔍 List the cast ids a scene's fingerprint names: probe_casts.py <video-id> <Scene>."""
import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent))
import lib
rt = lib.runtime()
job = next(j for j in rt.segment_plan() if j["video_id"] == sys.argv[1] and j["segment"]["scene"] == sys.argv[2])
state = rt.segment_state({k: job["segment"][k] for k in ("script", "scene")}, lib.settings("en", "speaker", lib.QUALITY, lib.FPS))
print(len(state["casts"]), sorted(state["casts"]))
