"""🎚️ Replay of today's real capacity waits (MEM-ADMIT): every `[capacity] waiting Ns for ...` line the measuring runs of 2026-11-27 recorded (need, spare = available - floor - reserved-but-not-yet-resident - admitted ahead, the unrealized memory R and the number of running grants) is re-decided with the elastic charge: each running grant is assumed to hold OBSERVED_SHARE of its reservation (measured on the live grants: 852/1900 = 45 %, 1557/3600 = 43 %, 2746/4599 = 60 %), its elastic charge is peak + max(256 MB, 3 sigma, 15 % of the slack); the warm-up of 60 s and the 120 s convergence are counted as done for CONVERGED of the grants. `python3 🎚️replay-waits.py [converged-share]` prints one JSON object: the waits as recorded, as the memory rule alone (spare >= need) predicts them without elastic charges and with them; only the difference of the two model columns is attributable to elastic reservations, because the recorded waits also contain queue-order waits the model does not know."""
import glob, json, math, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
LINE = re.compile(r"waiting (\d+)s for (\S+): .*?\(need (\d+) MB; (-?\d+) MB spare = (\d+) MB available - (\d+) MB floor - (\d+) MB reserved but not yet resident - (\d+) MB admitted ahead; load ([\d.]+); (\d+) running")
OBSERVED_SHARE = [0.43, 0.45, 0.60]
CONVERGED = float(sys.argv[1]) if len(sys.argv) > 1 else 1.0
MINIMUM, FACTOR, KEEP = 256, 3, 0.15


def elastic_rest(r_total, running, share):
    if running == 0 or r_total == 0:
        return 0
    each = r_total / running
    reserved = each / (1 - share)
    peak = reserved * share
    sigma = 0.05 * reserved
    margin = max(MINIMUM, FACTOR * sigma, math.ceil((reserved - peak) * KEEP))
    return running * min(each, margin)


def replay(share, converged):
    episodes, rows = [], 0
    for path in sorted(glob.glob(os.path.join(HERE, "🗑️generated", "*.log"))):
        current, previous = None, None
        for text in open(path, encoding="utf-8", errors="replace"):
            match = LINE.search(text)
            if not match:
                continue
            waited, operation, need, spare, available, floor, reserved, ahead, load, running = match.groups()
            waited, need, spare, reserved, ahead, running = int(waited), int(need), int(spare), int(reserved), int(ahead), int(running)
            if current is None or waited < previous or need != current["need"]:
                current = {"file": os.path.basename(path), "need": need, "operation": operation, "first": waited, "recorded": waited, "fits_at": None, "lines": 0}
                episodes.append(current)
            previous = waited
            rest = elastic_rest(reserved, running, share) * converged + reserved * (1 - converged)
            counter = spare + reserved - rest
            current["lines"] += 1
            current["recorded"] = waited
            rows += 1
            if current["fits_at"] is None and counter >= need:
                current["fits_at"] = waited
    return episodes, rows


def waited(episodes):
    rows = [e for e in episodes if e["recorded"] >= 10]
    return rows, sum(min(e["recorded"], e["fits_at"] if e["fits_at"] is not None else e["recorded"]) for e in rows)


report = {}
for share in OBSERVED_SHARE:
    episodes, rows = replay(share, CONVERGED)
    base, _ = replay(share, 0.0)
    waiting, counter = waited(episodes)
    _, rigid = waited(base)
    recorded = sum(e["recorded"] for e in waiting)
    report[f"observedShare={share}"] = {
        "lines": rows,
        "episodes": len(episodes),
        "recordedWaitSeconds": recorded,
        "modelWithoutElasticSeconds": rigid,
        "modelWithElasticSeconds": counter,
        "modelReductionPercent": round(100 * (rigid - counter) / rigid, 1) if rigid else None,
        "episodesThatFitAtOnceWithElastic": sum(1 for e in waiting if e["fits_at"] is not None and e["fits_at"] <= e["first"]),
        "episodesThatFitAtOnceWithoutElastic": sum(1 for e in replay(share, 0.0)[0] if e["fits_at"] is not None and e["fits_at"] <= e["first"]),
        "longestRecordedSeconds": max((e["recorded"] for e in waiting), default=0),
    }
print(json.dumps(report, indent=1))
