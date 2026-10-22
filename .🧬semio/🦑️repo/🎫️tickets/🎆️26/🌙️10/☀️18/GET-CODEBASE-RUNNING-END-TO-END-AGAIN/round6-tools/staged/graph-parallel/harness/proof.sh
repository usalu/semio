#!/bin/sh
# usage: proof.sh N TAGPREFIX WORKERS...     (WS=ws-farm by default)
# Runs the real project-nodes inference once per worker count, one after the other, and writes one JSON line per run to /private/tmp/graph-parallel/TAGPREFIX.results
HERE=$(cd "$(dirname "$0")" && pwd)
N=$1; PREFIX=$2; shift 2
: > "/private/tmp/graph-parallel/$PREFIX.results"
for W in "$@"; do
  LOAD=$(uptime | sed 's/.*averages: //')
  OUT=$(WIDTH=6000 "$HERE/run2.sh" "${WS:-ws-farm}" "$W" "$N" "$PREFIX-$W")
  python3 - "$W" "$LOAD" "$PREFIX-$W" <<PY >> "/private/tmp/graph-parallel/$PREFIX.results"
import json, re, sys
text = """$OUT"""
summary = next((json.loads(line[8:]) for line in text.splitlines() if line.startswith("SUMMARY ")), None)
roles = dict(item.split("=") for item in next((line for line in text.splitlines() if line.startswith("ROLES ")), "ROLES").split()[1:])
print(json.dumps({"workers": sys.argv[1], "load": sys.argv[2], "tag": sys.argv[3], "summary": summary and {k: summary[k] for k in ("owners", "wallS", "cpuS", "inferenceCpuS", "firstOwnerS", "lastOwnerS", "ownersPerMin", "projectsSha", "looseSha", "rowsSha", "rows", "unavailable", "unavailableSha", "records") if k in summary}, "roles": roles, "killed": "RSS LIMIT" in text}))
PY
done
cat "/private/tmp/graph-parallel/$PREFIX.results"
