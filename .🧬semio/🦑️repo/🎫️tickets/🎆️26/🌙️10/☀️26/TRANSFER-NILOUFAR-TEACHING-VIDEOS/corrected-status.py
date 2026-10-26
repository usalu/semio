"""📊 Report success-only progress of the declared corrected segment plan."""

import json
from pathlib import Path
import sys
import time

sys.dont_write_bytecode = True
ticket = Path(__file__).resolve().parent
sys.path.insert(0, str(Path.cwd() / "🎓️teaching"))
from video_runtime import segment_plan

root = ticket / "🗑️generated/g"
counts = {}
failures = []
started = []
for job in segment_plan():
    media = root / "s" / str(job["ordinal"])
    clip = media / "v" / f"{job['segment']['scene']}.mp4"
    ready = clip.with_suffix(".render.json").is_file()
    current = counts.setdefault(job["video_id"], [0, 0])
    current[0] += int(ready)
    current[1] += 1
    for record in media.glob("attempt-*.json"):
        try:
            attempt = json.loads(record.read_text(encoding="utf-8"))
        except ValueError:
            continue
        if "started" in attempt:
            started.append(attempt["started"])
        if attempt.get("exit_code", 0):
            failures.append({"ordinal": job["ordinal"], "scene": job["segment"]["scene"], "exit_code": attempt["exit_code"], "record": str(record)})
for video, (ready, total) in counts.items():
    print(f"[DEBUG] {video}: {ready}/{total} complete", flush=True)
print("[DEBUG] failures", json.dumps(failures), flush=True)
complete = sum(value[0] for value in counts.values())
total = sum(value[1] for value in counts.values())
elapsed = time.time() - min(started) if started else 0
rate = complete / elapsed if elapsed else 0
movies = len(list((root / "rendered").glob("*.mp4")))
summary = {"videos": counts, "failures": failures, "raw_complete": complete, "raw_total": total,
           "movies": movies, "elapsed_minutes": elapsed / 60,
           "observed_segments_per_minute": rate * 60, "straight_line_remaining_minutes": (total - complete) / rate / 60 if rate else None}
print("[DEBUG]", json.dumps(summary), flush=True)
(root / "progress.json").write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
