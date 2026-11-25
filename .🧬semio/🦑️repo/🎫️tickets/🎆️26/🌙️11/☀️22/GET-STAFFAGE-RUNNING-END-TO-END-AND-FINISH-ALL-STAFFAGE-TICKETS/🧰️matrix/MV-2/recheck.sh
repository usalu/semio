#!/bin/zsh
# 🔁️ The light proofs of plants of MV-2 again on the catalog files as they are, inside ONE reservation of the memory queue (1000 MB), one after the other: silhouette guard, `validateAsset` and three-views sweep (`../MR/measure.ts`), then the product check CLI. Result lines in `$GEN/t4/recheck.txt`, logs beside it. For a change another lane made to every catalog file (2026-11-25 19:09: `asset.tags` became `{ id, title }`). recheck.sh <id>…
BATCH=MV-2 source "${0:A:h}/../MR/env.sh"
export D="$GEN/t4"; mkdir -p "$D"; cd "$R"
repo capacity run --memory=1000 -- zsh -c '
for a in "$@"; do
  f="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"
  echo "===== $a $(date "+%F %T") file $(stat -f "%Sm %z" "$f") sha256 $(shasum -a 256 "$f" | cut -c1-16)" >> "$D/recheck.txt"
  (cd "$S/📦️packages/🟦️typescript" && timeout 900 "$B" "$M/measure.ts" "$a") > "$D/$a-measure.log" 2>&1; echo "measure exit $?" >> "$D/recheck.txt"
  grep -E "silhouette:|validate:|sweep:" "$D/$a-measure.log" >> "$D/recheck.txt"
  timeout 3000 "$B" "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" > "$D/$a-check.log" 2>&1
  echo "check exit $? $(grep -m1 -E "^(✓|✗)" "$D/$a-check.log" | cut -c1-160); scales: $(grep -cE "authored, suitable" "$D/$a-check.log") suitable, $(grep -cE "authored, (not|un)" "$D/$a-check.log") not" >> "$D/recheck.txt"
done
echo "recheck done $(date "+%T")" >> "$D/recheck.txt"' _ "$@"
