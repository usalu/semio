#!/bin/zsh
# 🧪️ One trial of an ops file on a copy: try.sh <id> [ops.json] applies the ops to the catalog file into $GEN/try/<id>.json (nothing in the catalog is written), runs the fit a second time on the copy (must change nothing), the model gate, and measure.ts (silhouette guard, validateAsset, three-views sweep) of the backup and of the copy side by side. MEASURE=0 skips the slow measure.
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
a="$1"; ops="${2:-$W/ops/$a.json}"; out="$GEN/try/$a.json"; mkdir -p "$GEN/try"
cd "$T"
"$W/b.sh" "$W/fit.ts" "$GEN/backup/$a.json" --ops "$ops" --out "$out" | grep -v "^written" | cut -c1-${COLS:-330}
"$W/b.sh" "$W/fit.ts" "$out" --ops "$ops" | tail -1
"$W/b.sh" "$W/mgate.ts" "$out" | cut -c1-${COLS:-330}
[[ "$MEASURE" == 0 ]] && exit 0
cd "$S/📦️packages/🟦️typescript" && "$W/q.sh" timeout 1200 "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$out" 2>&1 | grep -v "^ *$" | cut -c1-300
