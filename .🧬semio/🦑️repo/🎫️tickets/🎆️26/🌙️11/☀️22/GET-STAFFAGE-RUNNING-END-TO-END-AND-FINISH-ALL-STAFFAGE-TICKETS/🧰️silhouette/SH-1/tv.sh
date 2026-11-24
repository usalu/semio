#!/bin/zsh
# 🧊️ The pose-and-clip sweep of the three-views guard for one person and scale with the guard's own functions (`🧰️three-views/tv1-sweep.ts`), then who sets the differing edge (tv-culprits.py): tv.sh <slug> [scale]
source "${0:A:h}/env.sh"
(cd "$U/🧰️three-views" && timeout 900 "$B" tv1-sweep.ts "$1" --detail 1 ${2:+--scale $2}) > "$GEN/tv-$1.txt" 2>&1
python3 "$W/tv-culprits.py" "$GEN/tv-$1.txt" | cut -c1-230
