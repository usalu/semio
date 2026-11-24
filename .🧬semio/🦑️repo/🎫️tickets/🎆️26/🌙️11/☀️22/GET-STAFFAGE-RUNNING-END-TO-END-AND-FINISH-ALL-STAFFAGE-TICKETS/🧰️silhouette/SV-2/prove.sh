#!/bin/zsh
# 🩺️ The proof of repaired plants, one process after the other: the real `🧊️catalog-silhouette` test of each plant, `validateAsset` against the state before the repair, the verdict of the staffage check CLI, and the plant's tests in `🧊️three-views`, `🌡️catalog-physics` and `🫁️breathing-catalog`; logs in 🗑️generated/silhouette/SV-2/{vt,proof}: prove.sh <slug>…
source "${0:A:h}/env.sh"
mkdir -p "$GEN/proof"
for a in "$@"; do
  sil="$("$W/vt.sh" "sil-$a" 🧊️catalog-silhouette "$a")"
  why="$(grep -m1 -E "AssertionError|Error:" "$GEN/vt/sil-$a.log" | cut -c1-170)"
  (cd "$S/📦️packages/🟦️typescript" && "$B" "$W/validate.ts" "$a" --against "$GEN/backup") > "$GEN/proof/$a-validate.log" 2>&1
  (cd "$S" && timeout 2400 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$GEN/proof/$a-check.log" 2>&1
  tv="$("$W/vt.sh" "tv-$a" 🧊️three-views "$a")"
  ph="$("$W/vt.sh" "ph-$a" 🌡️catalog-physics "$a")"
  br="$(SUFFIX="" "$W/vt.sh" "br-$a" 🫁️breathing-catalog "$a")"
  echo "$a | silhouette: $sil $why | validate: $(tail -1 "$GEN/proof/$a-validate.log" | cut -f2,3) | check: $(grep -m1 -E '^(✓|✗)' "$GEN/proof/$a-check.log" | cut -c1-60) | three-views: $tv | physics: $ph | breathing: $br" | tee -a "$GEN/proof/lines.txt"
done
