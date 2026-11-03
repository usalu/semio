"""🔬 Show which parts of a segment state differ from its sidecar: why_stale.py <en|de> <video-id> <Scene>."""
import json
import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent))
import lib
rt = lib.runtime()
job = next(j for j in rt.segment_plan() if j["video_id"] == sys.argv[2] and j["segment"]["scene"] == sys.argv[3])
config = lib.settings(sys.argv[1], "speaker", lib.QUALITY, lib.FPS)
state = rt.segment_state({k: job["segment"][k] for k in ("script", "scene")}, config)
old = json.loads(lib.segment_clip(lib.base(sys.argv[1], "speaker"), job).with_suffix(".render.json").read_text(encoding="utf-8"))
for key in state:
    if state[key] != old.get(key):
        if isinstance(state[key], dict):
            for sub in sorted(set(state[key]) | set(old.get(key, {}))):
                if state[key].get(sub) != old.get(key, {}).get(sub):
                    print("differs", key, sub)
        else:
            print("differs", key)
