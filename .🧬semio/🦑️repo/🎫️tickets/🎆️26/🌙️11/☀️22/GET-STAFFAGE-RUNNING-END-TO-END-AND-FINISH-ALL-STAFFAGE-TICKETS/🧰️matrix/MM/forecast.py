"""🔮️ Forecasts the wall seconds of the full matrix per category and the critical path of its lanes: `python3 forecast.py <harvest.tsv> <estimate.jsonl> [<measured.tsv>]`.

Inputs: the TSV of `harvest.py` (seconds of records finished on the landed engine, wall time on the loaded host), `f-estimate.jsonl` of `estimate.ts` (processor seconds of the engine before the landing, every asset) and optionally a TSV `<asset> <seconds>` of runs measured by hand. An asset with a measured time uses it; every other asset gets its old estimate times the ratio of its category (measured seconds over old estimate of the assets of that category that have both), capped by nothing. Prints per category: assets, measured, forecast hours, the five slowest, and the lanes needed so that no lane has more than the longest asset; the forecast is in single-process hours on the host as it was loaded when the records were made (load 20 to 30 on 10 cores)."""
import collections
import json
import sys

harvest = {line.split("\t")[0]: (line.split("\t")[1], float(line.split("\t")[3])) for line in open(sys.argv[1])}
estimate = {entry["id"]: entry for entry in map(json.loads, open(sys.argv[2]))}
measured = {}
if len(sys.argv) > 3:
    for line in open(sys.argv[3]):
        key, seconds = line.split()[:2]
        measured[key] = float(seconds)
for key, (category, seconds) in harvest.items():
    measured.setdefault(key, seconds)
ratio = {}
for category in {entry["category"] for entry in estimate.values()}:
    both = [(estimate[key]["seconds"], seconds) for key, seconds in measured.items() if key in estimate and estimate[key]["category"] == category]
    ratio[category] = sum(b for a, b in both) / sum(a for a, b in both) if both else None
fallback = sum(seconds for seconds in measured.values()) / max(1e-9, sum(estimate[key]["seconds"] for key in measured if key in estimate))
rows = collections.defaultdict(list)
for key, entry in estimate.items():
    category = entry["category"]
    if key in measured:
        rows[category].append((key, measured[key], True))
    else:
        rows[category].append((key, entry["seconds"] * (ratio[category] if ratio[category] is not None else fallback), False))
total = 0.0
every = []
for category, items in sorted(rows.items()):
    seconds = sum(s for _, s, _ in items)
    total += seconds
    every += [(s, key) for key, s, _ in items]
    slow = sorted(items, key=lambda item: -item[1])[:5]
    print(f"{category:14} {len(items):3} assets {sum(1 for _, _, m in items if m):3} measured ratio {ratio[category] if ratio[category] is None else round(ratio[category], 2)!s:>5} {seconds / 3600:6.2f} h   slowest: " + ", ".join(f"{k} {s / 60:.0f} min{'' if m else '*'}" for k, s, m in slow))
every.sort(reverse=True)
print(f"all            {len(every):3} assets {total / 3600:6.2f} h single process; longest {every[0][1]} {every[0][0] / 60:.0f} min")
for lanes in (3, 4, 6, 8):
    load = [0.0] * lanes
    for seconds, _ in every:
        load[load.index(min(load))] += seconds
    print(f"{lanes} processes at a time: {max(load) / 3600:.2f} h (greedy, longest first, wall hours at the measured load; the quiet host is faster by the load factor up to 2.5x)")
