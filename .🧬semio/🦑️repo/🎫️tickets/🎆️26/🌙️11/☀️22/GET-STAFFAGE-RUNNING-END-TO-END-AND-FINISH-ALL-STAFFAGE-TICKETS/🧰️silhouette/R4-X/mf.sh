#!/bin/zsh
# 🧮️ misfit.ts for several views in one go, grouped by id pattern: mf.sh <id|file> <scale> <views,…> <ops|-> <min cells> [lines] [x0,x1,y0,y1]
source "${0:A:h}/env.sh"
cd "$S/📦️packages/🟦️typescript" || exit 1
for v in ${(s:,:)3}; do timeout 1500 "$B" "$W/misfit.ts" "$1" "$2" "$v" "$4" "$5" ${7:+"$7"} | cut -c1-170 | python3 -c '
import re, sys
n = int(sys.argv[1]); groups = {}; order = []
for line in sys.stdin:
    if line.startswith("=="):
        print(line.strip()); continue
    m = re.match(r"(\S+)\s+(\d+) of\s+(\d+) at \(([^)]*)\) (.*)", line)
    if not m: continue
    key = m[1] + " " + re.sub(r"\d+", "#", m[5].split(" as ")[0]) + (" as " + m[5].split(" as ")[1] if " as " in m[5] else "")
    g = groups.setdefault(key, [0, 0, 0, m[4]]); g[0] += int(m[2]); g[1] += int(m[3]); g[2] += 1
for key, g in sorted(groups.items(), key=lambda kv: -kv[1][0])[:n]:
    print(f"{g[0]:7d} of {g[1]:7d} in {g[2]:3d} first at ({g[3]}) {key}")
' "${6:-40}"; done
