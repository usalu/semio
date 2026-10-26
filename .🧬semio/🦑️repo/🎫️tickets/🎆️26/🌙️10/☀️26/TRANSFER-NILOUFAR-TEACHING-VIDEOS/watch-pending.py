"""⏱️ Observe two bounded snapshots of long corrected segment progress without signaling."""

import json
from pathlib import Path
import re
import sys
import time

root = Path(__file__).resolve().parent / "🗑️generated/g"
ordinals = [int(value) for value in sys.argv[1:]]


def snapshot(number):
    """📸 Capture file growth and the latest native Manim frame progress."""
    media = root / "s" / str(number)
    partials = list((media / "p").rglob("*.mp4"))
    svgs = [path for path in (media / "t").glob("*.svg") if not path.stem.endswith("_")]
    logs = sorted(media.glob("attempt-*.log"), key=lambda path: path.stat().st_mtime)
    log = logs[-1]
    size = log.stat().st_size
    with log.open("rb") as stream:
        stream.seek(max(0, size - 4096))
        text = stream.read().decode("utf-8", errors="replace")
    text = re.sub(r"\x1b\[[0-9;]*[A-Za-z]", "", text)
    partial_stats, svg_stats = [], []
    for paths, stats in ((partials, partial_stats), (svgs, svg_stats)):
        for path in paths:
            try:
                stats.append(path.stat())
            except FileNotFoundError:
                continue
    return {"ordinal": number, "time": time.time(), "raw_ready": any((media / "v").glob("*.render.json")),
            "partial_count": len(partial_stats), "partial_bytes": sum(stat.st_size for stat in partial_stats),
            "svg_last_modified": max((stat.st_mtime for stat in svg_stats), default=0),
            "log_bytes": size, "last_progress": text.splitlines()[-1][-240:] if text.splitlines() else ""}


before = {number: snapshot(number) for number in ordinals}
time.sleep(10)
after = {number: snapshot(number) for number in ordinals}
report = []
for number in ordinals:
    changed = any(before[number][key] != after[number][key] for key in ("raw_ready", "partial_count", "partial_bytes", "svg_last_modified", "log_bytes"))
    report.append({"ordinal": number, "advancing": changed, "before": before[number], "after": after[number]})
    print("[DEBUG]", json.dumps(report[-1]), flush=True)
(root / "pending-progress.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
