#!/bin/zsh
# 🧫️ Tries the spec of an asset without touching the catalog: writes the repaired backup document (the asset before this batch, `$GEN/backup/<id>.json`) to 🗑️generated/matrix/MF/try/<id>.json, prints the gate findings of it, the outlines that differ in the default pose, and compares silhouette, validateAsset and the three-views sweep of the backup and the trial (lines that differ only): t.sh <id> [--quiet]
export BATCH=MF; source "${0:A:h}/../MR/env.sh"
a="$1"; shift; mkdir -p "$GEN/try"
(cd "$T" && timeout 1800 "$B" "$W/fix.ts" "$a" --in "$GEN/backup/$a.json" --out "$GEN/try/$a.json" "$@") || true
(cd "$T" && timeout 900 "$B" "$W/restdiff.ts" "$GEN/backup/$a.json" "$GEN/try/$a.json")
cd "$S/📦️packages/🟦️typescript"
[[ -s "$GEN/try/$a.before.txt" ]] || timeout 1800 "$B" "$M/measure.ts" "$GEN/backup/$a.json" > "$GEN/try/$a.before.txt" 2>&1
timeout 1800 "$B" "$M/measure.ts" "$GEN/try/$a.json" > "$GEN/try/$a.after.txt" 2>&1
diff "$GEN/try/$a.before.txt" "$GEN/try/$a.after.txt" > /dev/null && echo "measure: identical ($(grep -E 'silhouette:|validate:|sweep:' "$GEN/try/$a.after.txt" | cut -d' ' -f2- | tr '\n' ';'))" || diff "$GEN/try/$a.before.txt" "$GEN/try/$a.after.txt" | cut -c1-330
