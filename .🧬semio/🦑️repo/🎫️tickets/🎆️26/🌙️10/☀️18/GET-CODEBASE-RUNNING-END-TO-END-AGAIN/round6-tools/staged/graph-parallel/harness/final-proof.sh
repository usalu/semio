#!/bin/sh
# 🏊️ The final identity and speed check of the farm: three disjoint stride samples of real projects, each inferred serially and by two workers, all niced, one tree at a time.
# usage: final-proof.sh [N] [WORKERS]   writes /private/tmp/graph-parallel/final.results (one JSON line per run) and compares each pair with compare-runs.py
HERE=$(cd "$(dirname "$0")" && pwd)
N=${1:-110}; W=${2:-2}
: > /private/tmp/graph-parallel/final.results
for OFF in 0 1 2; do
  for WK in 0 $W; do
    LOAD=$(uptime | sed 's/.*averages: //')
    OUT=$(OFFSET=$OFF WIDTH=6000 SEMIO_GRAPH_PHASES=1 "$HERE/run2.sh" ws-farm3 "$WK" "$N" "final-o$OFF-w$WK")
    python3 - "$OFF" "$WK" "$LOAD" <<PY >> /private/tmp/graph-parallel/final.results
import json, sys
text = """$OUT"""
summary = next((json.loads(l[8:]) for l in text.splitlines() if l.startswith("SUMMARY ")), None)
roles = dict(i.split("=") for i in next((l for l in text.splitlines() if l.startswith("ROLES ")), "ROLES").split()[1:])
print(json.dumps({"offset": sys.argv[1], "workers": sys.argv[2], "load": sys.argv[3], "summary": summary and {k: summary[k] for k in ("owners", "wallS", "inferenceCpuS", "firstOwnerS", "lastOwnerS", "ownersPerMin", "projectsSha", "unavailableSha", "records") if k in summary}, "roles": roles, "killed": "RSS LIMIT" in text}))
PY
  done
  python3 "$HERE/compare-runs.py" "final-o$OFF-w0" "final-o$OFF-w$W" >> /private/tmp/graph-parallel/final.compare 2>&1
done
cat /private/tmp/graph-parallel/final.results
cat /private/tmp/graph-parallel/final.compare
