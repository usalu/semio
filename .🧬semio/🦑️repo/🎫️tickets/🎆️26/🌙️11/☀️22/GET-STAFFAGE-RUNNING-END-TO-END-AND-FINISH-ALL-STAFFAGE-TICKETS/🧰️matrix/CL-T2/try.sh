#!/bin/zsh
# 🧫️ Makes the trial copies of assets of lane CL-T2 and measures them, one process after the other inside the grant of its caller: `fix.ts <id> --out $GEN/try/<id>.json` (the digest of the catalog file it was made from into `$GEN/try/<id>.base`), then the geometry findings of `🧊️model-catalog` on the copy (`geo.ts`, with `SOLIDS=1` every solid of the scenes). try.sh <id>…
source "${0:A:h}/env.sh"
mkdir -p "$GEN/try"
for a in "$@"; do
  f="$(ls "$C"/*/"$a"/🔣️.json)"; shasum -a 256 < "$f" | cut -c1-16 > "$GEN/try/$a.base"
  (cd "$W" && "$B" fix.ts "$a" --out "$GEN/try/$a.json") 2>&1 | cut -c1-400
  (cd "$S" && "$B" "$W/geo.ts" ${SOLIDS:+--solids} "$GEN/try/$a.json") 2>&1 | cut -c1-1200
done
