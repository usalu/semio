#!/bin/zsh
# 🧍️ The loft trial of the reference person inside a queue grant: copies the catalog document of `tour-guide` to 🗑️generated/matrix/MX-2/tour/base.json, applies `ops/tour-guide.lofts.json` with loft.ts into trial.json (twice: the second run must change nothing), then measures both: the silhouette guard (guard.ts), the fixer's gate over every subject with the twin tier (`MR/gate.ts --twins`), the print check of every band and its range ends (`MC/prints.ts`) and `validateAsset` (`MC/validate.ts`): tour.sh [scales, default 50]
export BATCH=MX-2; source "${0:A:h}/../MR/env.sh"; out="$GEN/tour"; mkdir -p "$out"
P="$S/📦️packages/🟦️typescript"; MC="$U/🧰️matrix/MC"; scales="${1:-50}"
cp "$(ls "$S"/🗂️catalog/*/tour-guide/🔣️.json)" "$out/base.json"
(cd "$P" && timeout 300 "$B" "$W/loft.ts" "$out/base.json" --ops "$W/ops/tour-guide.lofts.json" --out "$out/trial.json") > "$out/loft.txt" 2>&1; tail -4 "$out/loft.txt" | cut -c1-300
(cd "$P" && timeout 300 "$B" "$W/loft.ts" "$out/trial.json" --ops "$W/ops/tour-guide.lofts.json" --out "$out/trial2.json") > "$out/loft2.txt" 2>&1; cmp -s "$out/trial.json" "$out/trial2.json" && echo "second run: unchanged" || echo "SECOND RUN CHANGED THE FILE: $(tail -1 "$out/loft2.txt")"
for f in base trial; do
  (cd "$P" && timeout 1500 "$B" "$W/guard.ts" "$out/$f.json" --scales "$scales" --tag "$f ") 2>&1 | grep -v '^\[' | cut -c1-300
  (cd "$T" && timeout 3000 "$B" "$M/gate.ts" "$out/$f.json" --scales "$scales" --twins) > "$out/gate-$f.txt" 2>&1; echo "$f gate: $(tail -1 "$out/gate-$f.txt")"; grep -E ' HARD | twin | reported ' "$out/gate-$f.txt" | cut -c1-230
  (cd "$T" && timeout 900 "$B" "$MC/prints.ts" "$out/$f.json") 2>&1 | grep -v '^\[' | cut -c1-400
  (cd "$P" && timeout 900 "$B" "$MC/validate.ts" "$out/$f.json") 2>&1 | grep -v '^\[' | cut -c1-300
done
