"""🕰️ W13: one compact line per worker from the newest sample of the poll file argv[1]: state, reason, report age, report uptime, taken/missing, fleet counters, CLI tail; plus the Ready count."""
import json, sys, time
last = {}
for line in open(sys.argv[1]):
    row = json.loads(line); last[row["w"]] = row
print(time.strftime("%H:%M:%S"), "ready", sum(1 for r in last.values() if r["ready"]), "of", len(last))
for worker, r in sorted(last.items()):
    print(worker[3:], r["state"], r["reason"], "age", r["reportAge"], "up", r["up"], "taken", r["taken"], "miss", (r["ledger"] or {}).get("missing"), r["fleet"] or "", "|", (r["tail"] or "").replace("Reading the project graph slice ", "")[:120])
