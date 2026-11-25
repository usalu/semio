#!/bin/zsh
# 🔁️ One asset from its settled ops to a measured trial without the solver (for an ops file corrected by hand): `fit.ts` replays ops/<id>.json on the backup into $GEN/try/<id>.json, then the print profile (`prints.ts`), the scan of the file (`../MH-4/scan-file.ts`), the gate with twins and the measures run on the trial; logs in $GEN/gate and $GEN/measure: retry.sh <id>…   (run it inside a grant: a job of lane.sh)
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"
for a in "$@"; do
  echo "=== $a $(date +%H:%M)"
  (cd "$T" && "$B" "$W/fit.ts" "$GEN/backup/$a.json" --ops "$W/ops/$a.json" --out "$GEN/try/$a.json" 2>&1 | grep -v unchanged | tail -6 | cut -c1-240)
  (cd "$S/📦️packages/🟦️typescript" && "$B" "$W/prints.ts" "$GEN/try/$a.json" 2>&1 | sed -E 's/\{"profile".*"suitable"/"suitable"/' | cut -c1-300)
  (cd "$T" && "$B" "$M/../MH-4/scan-file.ts" "$GEN/try/$a.json" > "$GEN/gate/scan-$a.log" 2>&1); cut -c1-220 "$GEN/gate/scan-$a.log" | head -12
  (cd "$T" && "$B" "$M/gate.ts" "$GEN/try/$a.json" --twins > "$GEN/gate/try-$a.log" 2>&1)
  awk '/ HARD /{p=1; print; next} /^    /{if(p) print; next} {p=0} /^hard findings/{print}' "$GEN/gate/try-$a.log" | cut -c1-240
  (cd "$S/📦️packages/🟦️typescript" && "$B" "$M/measure.ts" "$GEN/try/$a.json" > "$GEN/measure/try-$a.log" 2>&1); grep -E "silhouette:|validate:|sweep:" "$GEN/measure/try-$a.log"
done
