#!/bin/zsh
# ⏮️ The silhouette guard of the state BEFORE the repair, inside the grant of its caller: `../MR/measure.ts` on `$GEN/before/<id>.json` (the catalog file as it stood before this lane wrote it), the lines of the boxes and the three views into `$GEN/before/<id>.txt`. before.sh <id>…
source "${0:A:h}/env.sh"
for a in "$@"; do (cd "$S/📦️packages/🟦️typescript" && timeout 1800 "$B" "$M/measure.ts" "$GEN/before/$a.json") 2>&1 | grep -E "silhouette:|validate:|sweep:|bbox" | cut -c1-300 > "$GEN/before/$a.txt"; cat "$GEN/before/$a.txt"; done
