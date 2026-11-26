#!/bin/zsh
# 🥈️ Second stage of closing lane CL-A inside ONE grant: for each asset whose light proof passed, the guard of its backup (`../MR/measure.ts`, the numbers of before), `prove.sh` (write, tell MZ, pins, gate, scan, prints, guard, check CLI, matrix), then for `goat` the product sheets of the backup and of the written file at every authored scale (`sheets.sh`; the squirrel's drawings do not change). stage2.sh <id>…
source "${0:A:h}/env.sh"
for a in "$@"; do
  (cd "$S/📦️packages/🟦️typescript" && timeout 3000 "$B" "$M/measure.ts" "$GEN/backup/$a.json") > "$GEN/proof-before-$a-measure.log" 2>&1; echo "guard of the backup of $a:"; grep -E "silhouette:|validate:|sweep:|bbox" "$GEN/proof-before-$a-measure.log" | cut -c1-300
  zsh "$W/prove.sh" "$a" "closing — nine assets against the detail pins"; [[ -f "$GEN/proof/$a.txt" ]] && cat "$GEN/proof/$a.txt"
done
[[ " $* " == *" goat "* && -f "$GEN/proof/goat.proved" ]] && POSES="browsing-upright" zsh "$W/sheets.sh" "before:$GEN/backup/goat.json" "after:$C/🐕️animals/goat/🔣️.json"
