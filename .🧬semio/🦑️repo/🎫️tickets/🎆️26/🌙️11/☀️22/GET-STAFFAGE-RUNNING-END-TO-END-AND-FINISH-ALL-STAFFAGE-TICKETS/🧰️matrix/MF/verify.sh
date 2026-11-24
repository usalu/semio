#!/bin/zsh
# 🔒️ Says for every asset of batch MF whether its catalog document still is the repaired one (`fix.ts` would change nothing): replication can put an older copy back. verify.sh [<id>…]
export BATCH=MF; source "${0:A:h}/../MR/env.sh"
for a in ${@:-$(cut -f1 "$U/🧰️matrix/batches/MF.tsv")}; do
  f="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"; t="$(mktemp)"
  (cd "$T" && timeout 600 "$B" "$W/fix.ts" "$a" --no-scan --quiet --out "$t" > /dev/null 2>&1)
  cmp -s "$f" "$t" && echo "$a: repaired" || echo "$a: NOT the repaired document"
  rm -f "$t"
done
