#!/bin/zsh
# ⏮️ The silhouette guard of the file before this lane's write (`$GEN/backup/<id>.json`), inside the grant of its caller, into `$GEN/before/<id>-measure.txt`: before.sh <id>
source "${0:A:h}/env.sh"
mkdir -p "$GEN/before"; cd "$S/📦️packages/🟦️typescript" && "$B" "$M/measure.ts" "$GEN/backup/$1.json" > "$GEN/before/$1-measure.txt" 2>&1; grep -E "bbox|silhouette:" "$GEN/before/$1-measure.txt" | cut -c1-300
