#!/bin/zsh
# 🔦️ The light proof of a trial copy before its catalog write, inside the grant of the worker: `light.sh <id>` prints for `$GEN/try/<id>.json` the reported tier of the last gate run, the guard with validator and sweep (`../MR/measure.ts`), print suitability at keys and band ends and what the bodies show beside the outlines at rest.
source "${0:A:h}/env.sh"
a="$1"; grep -E "HARD|reported|hard findings" "$GEN/log/$a-try-gate.log" | cut -c1-220
(cd "$S/📦️packages/🟦️typescript" && timeout 3000 "$B" "$M/measure.ts" "$GEN/try/$a.json" | grep -E "bbox|silhouette:|validate:|sweep:" | cut -c1-260)
cd "$T"; timeout 900 "$B" "$U/🧰️matrix/MC/prints.ts" "$GEN/try/$a.json" | tail -2 | cut -c1-200; tail -1 "$GEN/log/$a-try-rest.log"
