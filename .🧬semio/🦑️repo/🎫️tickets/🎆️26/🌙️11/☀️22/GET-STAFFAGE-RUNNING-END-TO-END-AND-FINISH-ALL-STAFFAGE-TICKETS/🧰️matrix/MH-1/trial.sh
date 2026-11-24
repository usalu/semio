#!/bin/zsh
# 🧪️ Measures the trial copies $GEN/try/<id>.json, one asset after the other: the full gate (hard findings printed, log in $GEN/gate/try-<id>.log) and the silhouette guard, validateAsset and three-views sweep of backup and trial side by side (only the lines that differ, and the verdict lines of the trial): trial.sh <id>…   (NOGATE=1 skips the gate, GATE_ARGS narrows it)
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"
mkdir -p "$GEN/measure"
for a in "$@"; do
  [[ -z "$NOGATE" ]] && "${0:A:h}/g.sh" "$GEN/try/$a.json" "try-$a"
  [[ -f "$GEN/measure/before-$a.log" ]] || (cd "$S/📦️packages/🟦️typescript" && timeout 900 "$B" "$M/measure.ts" "$GEN/backup/$a.json" > "$GEN/measure/before-$a.log" 2>&1)
  (cd "$S/📦️packages/🟦️typescript" && timeout 900 "$B" "$M/measure.ts" "$GEN/try/$a.json" > "$GEN/measure/try-$a.log" 2>&1)
  diff "$GEN/measure/before-$a.log" "$GEN/measure/try-$a.log" | grep '^[<>]' | cut -c1-250
  grep -E "silhouette:|validate:|sweep:" "$GEN/measure/try-$a.log"
done
