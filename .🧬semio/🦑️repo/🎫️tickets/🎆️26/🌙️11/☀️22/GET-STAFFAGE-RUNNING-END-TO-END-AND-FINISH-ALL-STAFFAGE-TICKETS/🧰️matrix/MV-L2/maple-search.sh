#!/bin/zsh
# 🔬️ The search of `norway-maple` on copies, inside a grant of 2200 MB of the worker (the loft search peaks at 1668 MB): `../MV-L1/loft.ts --save loft/norway-maple.json` on the backup (every foliage mass as one loft of its ellipsoids, three views, every scale); `lofts.ts --scales 100,200` applies the two bands that pass the silhouette guard as `$GEN/base/norway-maple.json`; `trial.sh norway-maple --scales 50` replaces the depth plates of 1:50 by rounded bodies and writes `$GEN/try/norway-maple.json` with the gate; then the reach of the lofts against their ellipsoids (`short.ts`), the guard with validator and sweep (`../MR/measure.ts`) and print suitability. Nothing in the catalog is written. maple-search.sh
source "${0:A:h}/env.sh"
L="$U/🧰️matrix/MV-L1"; a=norway-maple; mkdir -p "$GEN/try" "$GEN/base" "$W/loft"; cd "$T"
echo "loft tool sha-256 $(shasum -a 256 < "$L/loft.ts" | cut -c1-16)"
timeout 5400 "$B" "$L/loft.ts" "$GEN/backup/$a.json" --save "$W/loft/$a.json" --out "$GEN/try/$a-loft.json" > "$GEN/log/$a-loft.log" 2>&1; echo "loft exit $?"
grep -E '^1:[0-9]+: |ISSUE|LOST' "$GEN/log/$a-loft.log" | cut -c1-300
timeout 600 "$B" "$W/lofts.ts" "$GEN/backup/$a.json" --ops "$W/loft/$a.json" ${=$(< "$W/flags/$a.lofts")} --out "$GEN/base/$a.json"
zsh "$W/trial.sh" "$a" --scales 50 --paper 0.25
cd "$T"; timeout 1200 "$B" "$W/short.ts" "$GEN/try/$a.json" --scales 100,200 | tail -7 | cut -c1-300
(cd "$S/📦️packages/🟦️typescript"; timeout 3000 "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$GEN/try/$a.json" | grep -E "bbox|silhouette:|validate:|sweep:" | cut -c1-260)
timeout 600 "$B" "$U/🧰️matrix/MC/prints.ts" "$GEN/try/$a.json" | tail -3 | cut -c1-300
