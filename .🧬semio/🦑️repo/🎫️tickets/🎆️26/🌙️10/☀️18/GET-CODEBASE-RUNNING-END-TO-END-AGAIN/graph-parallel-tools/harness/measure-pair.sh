#!/bin/sh
# 🧪️ One ledger-faithful cold run per (overlay, workers) pair, niced; prints the CPU of the parent between the first and the last owner and the ledger counts. usage: measure-pair.sh OVERLAY WORKERS TAG [N]
HERE=$(cd "$(dirname "$0")" && pwd)
echo "== $3 load $(uptime | sed 's/.*averages: //')"
OFFSET=0 WIDTH=2500 GP_LEDGER=1 SEMIO_GRAPH_PHASES=1 SEMIO_GRAPH_WORKER_MB=1024 "$HERE/run2.sh" "$1" "$2" "${4:-60}" "$3" 2>&1 | python3 -c "
import sys,json,re
t=sys.stdin.read()
m=re.search(r'SUMMARY (\{.*\})',t)
d=json.loads(m.group(1)) if m else {}
print({k:d.get(k) for k in ('inferenceCpuS','firstOwnerS','lastOwnerS','owners','projectsSha','ledger','accounting')})
print(re.findall(r'ROLES.*',t))"
