#!/bin/zsh
# 🧾️ The final proof of batch SAV on the catalog files, one process after the other, into 🗑️generated/silhouette/SAV/final: every check of the guard (`measure.ts`), the product drawings before and after at 1:50 and at the coarsest authored scale of every repaired asset (`sheets.sh`, stacked for review by `stack.ts`), the real `🧊️catalog-silhouette` test, `🧊️three-views`, `🌡️catalog-physics`, `⚡️asset-power`, `🫁️breathing-catalog` and `✅️asset-validation` with a name filter over the 23 assets, and the product check CLI per asset: final.sh
source "${0:A:h}/env.sh"
F="$GEN/final"; mkdir -p "$F"
all=($=ANIMALS $=VEHICLES)
(cd "$S/📦️packages/🟦️typescript" && timeout 2400 "$B" "$W/measure.ts" $all) > "$F/measure.txt" 2>&1
"$W/review.sh"
for s in 🧊️catalog-silhouette 🧊️three-views 🌡️catalog-physics 🫁️breathing-catalog ✅️asset-validation; do echo "== $s $("$W/vt.sh" "final-$s" "$s" $all | tr '\n' ' ' | cut -c1-900)"; done > "$F/suites.txt" 2>&1
echo "== ⚡️asset-power $(SUFFIX='(:| |$)' "$W/vt.sh" "final-power" "⚡️asset-power" $all | tr '\n' ' ' | cut -c1-600)" >> "$F/suites.txt" 2>&1
for a in $all; do (cd "$S" && timeout 900 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" 2>&1 | grep -E "^(✓|✗)|UNSUITABLE|^  [a-z]" | cut -c1-220 | head -4); done > "$F/check.txt" 2>&1
echo FINAL-DONE >> "$F/check.txt"
