"""🔭️ W13: prints the poll samples of argv[1] per worker as one line per sample (time, state, report age, report uptime, taken, verified, presence body/continuation, the CLI tail), argv[2] = worker filter (substring), argv[3] = every n-th sample."""
import json, sys, time, collections
rows = [json.loads(line) for line in open(sys.argv[1])]
want, every = (sys.argv[2] if len(sys.argv) > 2 else ""), int(sys.argv[3]) if len(sys.argv) > 3 else 1
by = collections.defaultdict(list)
for row in rows:
    if want in row["w"]: by[row["w"]].append(row)
for worker, samples in by.items():
    print("==", worker)
    for row in samples[::every]:
        tail = (row["tail"] or "").replace("Reading the project graph slice ", "")
        print(time.strftime("%H:%M:%S", time.localtime(row["t"])), row["state"], row["reason"], "age", row["reportAge"], "up", row["up"], "pid", row["pid"], "taken", row["taken"], "ver", (row.get("verified") or {}).get("remembered"), "body", (row.get("body") or {}).get("continuation"), (row.get("body") or {}).get("fileChecks"), "fleet", row.get("fleet"), "|", tail[:150])
