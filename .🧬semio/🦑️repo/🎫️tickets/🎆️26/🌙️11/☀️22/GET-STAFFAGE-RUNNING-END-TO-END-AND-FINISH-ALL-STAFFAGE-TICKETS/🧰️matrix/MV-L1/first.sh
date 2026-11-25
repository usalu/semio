#!/bin/zsh
# 🧪️ First look of the loft carrier for the three plants inside one reservation: `loft.ts` on every plant (copies only), then the panels of `zooms-first.tsv`. first.sh
source "${0:A:h}/env.sh"
mkdir -p "$GEN/try" "$GEN/log" "$GEN/sheets" "$W/ops"; export MEMLOG="$GEN/log/mem-first.txt"
cd "$R" && INSIDE=1 repo capacity run --memory=1500 -- zsh -c '
for a in hawthorn field-maple black-alder; do s=$(date +%s)
"$W/q.sh" 0 "$T" 3000 "$B" "$W/loft.ts" "$a" --save "$W/ops/$a.json" --out "$GEN/try/$a.json" > "$GEN/log/$a-loft.log" 2>&1; echo "$a loft exit $? $(( $(date +%s)-s )) s"; done
while read -r a tag pose s view height; do
  [[ "$tag" == before ]] && f="$GEN/backup/$a.json" || f="$GEN/try/$a.json"
  "$W/q.sh" 0 "$T14" 600 "$B" "$W/zoom.ts" "$f" "$s" "$pose" "$view" "$GEN/sheets/z.$a.s$s.$pose.$view.$tag.png" $height 2>&1 | tail -1
done < "$W/zooms-first.tsv"'
