#!/bin/zsh
# ✍️ Writes the repair of assets of batch MF into the catalog, one after the other: runs `fix.ts <id> --write` (gate of the result on stdout), then once more to show that the second run changes nothing; the log of each goes to 🗑️generated/matrix/MF/apply/<id>.txt and a one-line verdict is printed: apply.sh <id>…
export BATCH=MF; source "${0:A:h}/../MR/env.sh"
mkdir -p "$GEN/apply"
for a in "$@"; do
  (cd "$T" && timeout 1800 "$B" "$W/fix.ts" "$a" --write --quiet > "$GEN/apply/$a.txt" 2>&1)
  (cd "$T" && timeout 1800 "$B" "$W/fix.ts" "$a" --write --quiet --no-scan > "$GEN/apply/$a.again.txt" 2>&1)
  echo "$a | $(grep -E "hard findings" "$GEN/apply/$a.txt") | first: $(tail -1 "$GEN/apply/$a.txt" | cut -c1-60) | second: $(tail -1 "$GEN/apply/$a.again.txt" | cut -c1-60)"
done
