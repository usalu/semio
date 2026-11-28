#!/bin/zsh
# ⏮️ The silhouette guard of the files as they were before this lane wrote them (`$GEN/backup/<id>.json`), inside the grant of its caller: before.sh <id>… writes `$GEN/before/<id>-measure.log` and prints the lines per scale.
source "${0:A:h}/env.sh"
mkdir -p "$GEN/before"
for a in "$@"; do (cd "$S/📦️packages/🟦️typescript" && timeout 2400 "$B" "$M/measure.ts" "$GEN/backup/$a.json") > "$GEN/before/$a-measure.log" 2>&1; grep -E "bbox|silhouette:" "$GEN/before/$a-measure.log" | cut -c1-300; done
