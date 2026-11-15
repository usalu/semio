import json
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
root = Path(__file__).resolve().parents[7]
manifest = json.loads((root / "🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json").read_text(encoding="utf-8"))
total = partial = multi = 0
for video in manifest["videos"]:
    print(video["id"], video["script"], video["output"], len(video["chapters"]))
    for chapter in video["chapters"]:
        segments = chapter["segments"]
        total += 1
        partial += any("clauses" in s for s in segments)
        multi += len(segments) > 1
        print("   ", chapter["scene"], [(s["script"].split("/")[-3], s["scene"], "clauses" if "clauses" in s else "") for s in segments])
print("chapters", total, "with clauses", partial, "multi-segment", multi)
