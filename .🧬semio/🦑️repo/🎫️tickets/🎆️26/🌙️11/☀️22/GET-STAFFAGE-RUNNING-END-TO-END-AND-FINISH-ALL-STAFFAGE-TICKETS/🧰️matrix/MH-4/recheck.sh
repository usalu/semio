#!/bin/zsh
# 🔁️ Repeats the product check CLI of assets whose check the memory queue gave up on, and appends the line to their proof: recheck.sh <id>…
source "${0:A:h}/env.sh"
source "$W/lock.sh"
for a in "$@"; do
  "$W/cap.sh" 1000 1800 "$S" "$B" "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" > "$GEN/proof/$a-check.log" 2>&1
  echo "check (repeated through the queue): $(grep -m1 -E '^(✓|✗)' "$GEN/proof/$a-check.log" | cut -c1-160)" | tee -a "$GEN/proof/$a.out"
done
