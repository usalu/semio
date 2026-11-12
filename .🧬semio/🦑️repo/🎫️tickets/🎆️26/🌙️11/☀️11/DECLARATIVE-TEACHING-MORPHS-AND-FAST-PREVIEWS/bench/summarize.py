"""📊 Print a markdown table of every bench-*.json under 🗑️generated (or a given folder)."""
import json
import sys
from pathlib import Path

root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parents[1] / "🗑️generated"
rows = []
for path in sorted(root.glob("*/bench-*.json")):
    try:
        d = json.loads(path.read_text(encoding="utf-8"))
    except ValueError:
        continue
    m = d["movie"]
    s = d["seconds"]
    c = d["morph_constructor_counts"]
    plays = [p["wall_s"] for p in d["play_log"] if p["morph"]]
    full = {k: v for k, v in c.items() if not k.startswith("_plan_seconds")}
    rows.append(f"| {d['label']} | {d['scene']} | {m.get('width')}x{m.get('height')}@{round(m.get('fps', 0))} | {m.get('frames')} / {m.get('duration_s') and round(m['duration_s'], 1)} s | {s['to_first_play']} / {s['to_first_frame']} | {s['render_total']} | {d['process_cpu_seconds']['total']} | {d['plays']} ({d['morph_plays']} morph) | {round(sum(plays), 1)} | {d['peak_tree_rss_mb']} | {full} | {d['plan_seconds']} |")
print("| label | scene | output | frames / duration | first play / first frame s | render s | proc CPU s | plays | morph-play wall s | peak RSS MB | constructor counts | _plan s |")
print("|---|---|---|---|---|---|---|---|---|---|---|---|")
print("\n".join(rows))
