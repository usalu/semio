#!/bin/zsh
# 🏛️ Tries the repair of `steel-column` on a trial copy through MC's chain, inside one queue grant: the ops writer `MC/make/steel-column.py` with PLATE=<metres>, `MC/apply.ts` into a trial copy, then the measure of the suite without thresholds (`MC/derived.ts --far`), the print check of every band and its range ends (`MC/prints.ts`), `validateAsset` and the silhouette guard (`MR/measure.ts`): column-try.sh <plate metres>…   (output in 🗑️generated/matrix/MX-2/column/)
export BATCH=MX-2; source "${0:A:h}/../MR/env.sh"
MC="$U/🧰️matrix/MC"; P="$S/📦️packages/🟦️typescript"; out="$GEN/column"; mkdir -p "$out"
for t in "$@"; do
  f="$out/steel-column-$t.json"
  PLATE="$t" python3 "$MC/make/steel-column.py" > "$out/make-$t.txt" 2>&1 || { echo "plate $t: the ops writer failed"; cat "$out/make-$t.txt"; continue; }
  (cd "$P" && timeout 300 "$B" "$MC/apply.ts" steel-column --out "$f") > "$out/apply-$t.txt" 2>&1 || { echo "plate $t: apply failed"; tail -3 "$out/apply-$t.txt"; continue; }
  (cd "$T" && timeout 600 "$B" "$MC/derived.ts" "$f" --far) > "$out/derived-$t.txt" 2>&1
  (cd "$T" && timeout 600 "$B" "$MC/prints.ts" "$f") > "$out/prints-$t.txt" 2>&1
  (cd "$P" && timeout 900 "$B" "$M/measure.ts" "$f") > "$out/measure-$t.txt" 2>&1
  echo "== plate $t: $(tail -1 "$out/apply-$t.txt" | cut -c1-160)"; cat "$out/derived-$t.txt" | cut -c1-260; cat "$out/prints-$t.txt" | cut -c1-400; grep -E "1:|silhouette:|validate:|sweep:" "$out/measure-$t.txt" | cut -c1-260
done
