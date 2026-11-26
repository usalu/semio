#!/bin/zsh
# 🛡️ The silhouette guard of the six assets of round MV-L2 once more on the catalog files, inside a grant of the worker (after the pocket rule of `🧊️catalog-silhouette` went live, D70): `../MR/measure.ts` per asset (guard per scale and view, validator, sweep) and the real test file `🧊️catalog-silhouette` with the six names as filter; lines into `$GEN/proof/guard.txt`. guard.sh
source "${0:A:h}/env.sh"
o="$GEN/proof/guard.txt"; ids=(norway-maple multi-stem-birch pampas-grass miscanthus-grass rowan used-timpani-pair); mkdir -p "$GEN/vt"
echo "guard of $(date +%F\ %H:%M), silhouette module of $(date -r "$S/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts" +%F\ %H:%M)" > "$o"
for a in $ids; do (cd "$S/📦️packages/🟦️typescript" && timeout 3000 "$B" "$M/measure.ts" "$a") 2>&1 | grep -E "bbox|silhouette:|validate:|sweep:" | cut -c1-260 >> "$o"; echo "$a sha-256 $(shasum -a 256 < "$(ls "$C"/*/"$a"/🔣️.json)" | cut -c1-16)" >> "$o"; done
(cd "$S/📦️packages/🟦️typescript" && timeout 3000 "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/🧊️catalog-silhouette/🟦️.ts" -t "(${(j:|:)ids})( |$)") > "$GEN/vt/guard-silhouette.log" 2>&1
echo "🧊️catalog-silhouette exit $? $(grep -E '^ *(Test Files|Tests) ' "$GEN/vt/guard-silhouette.log" | tr -s ' ' | tr '\n' ';')" >> "$o"
echo "done $(date +%H:%M)" >> "$o"
