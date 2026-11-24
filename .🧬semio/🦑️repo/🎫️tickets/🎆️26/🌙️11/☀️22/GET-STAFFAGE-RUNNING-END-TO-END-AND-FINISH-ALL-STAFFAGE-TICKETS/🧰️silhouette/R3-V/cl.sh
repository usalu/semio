#!/bin/zsh
# 🔎️ The worst places of views of a vehicle in short: per place the side, the point, the distance and the nearest drawn shapes and body solids: cl.sh <id> <scale> <view|all> [ops.json|-] [count] [limit m]
source "${0:A:h}/env.sh"
cd "$S/📦️packages/🟦️typescript" && LIMIT=$6 timeout 900 "$B" "$W/clusters.ts" "$1" "$2" "$3" "${4:--}" "${5:-6}" | python3 -c '
import re, sys
for line in sys.stdin:
    if line.startswith("=="):
        print(line.strip()[:60]); continue
    m = re.match(r"  (\S+) \(([^)]*)\) d=(\S+)  drawn: (.*?) \| vol: (.*?) \| covering: (.*)", line)
    if not m: continue
    near = lambda s: " ".join(sorted(s.split(), key=lambda t: float(t.rsplit("]", 1)[1]))[:4])
    print(f"  {m[1][:6]} ({m[2]}) {m[3]} D[{near(m[4])}] V[{near(m[5])}] C[{m[6].strip()[:60]}]")
'
