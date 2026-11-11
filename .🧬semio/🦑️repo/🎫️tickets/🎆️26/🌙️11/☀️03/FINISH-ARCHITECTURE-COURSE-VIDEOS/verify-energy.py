"""🔎 Verify current energy masters and source certificates."""
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent.parent / "☀️01" / "HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS" / "🧰️r3-media"))
import lib

selected = {"physical_fundamentals", "ventilation", "heating", "cooling", "energy_balance"}
cache, result = {}, []
for video in lib.runtime().load_manifest():
    if video["id"] not in selected:
        continue
    status = lib.video_status(video["id"], "en", "speaker", video["output"], cache)
    fresh = lib.master_fresh(status)
    record = {"video": video["id"], "fresh": fresh, "staleSources": [s["ordinal"] for s in status["segments"] if not s["fresh"]],
              "sha256": lib.digest(status["master"]) if status["master"].is_file() else None}
    result.append(record)
    print(json.dumps(record), flush=True)
(HERE / "🗑️generated" / "energy-verification.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
if len(result) != len(selected) or not all(item["fresh"] for item in result):
    raise SystemExit(1)
