#!/bin/zsh
# ✍️ Writes the repair of assets into the catalog, one asset after the other: keeps the untouched document in $GEN/backup/<id>.json (once), `fix.ts <id> --write` with `ops/<id>.json` (the document is read right before it is written), then once more to show that the second run changes nothing: apply.sh <id>…   (run it through q.sh)
source "${0:h}/env.sh"
mkdir -p "$GEN/apply" "$GEN/backup"; cd "$T"
for a in "$@"; do
  [[ -f "$GEN/backup/$a.json" ]] || cp "$(asset "$a")" "$GEN/backup/$a.json"
  "$B" "$W/fix.ts" $a --write --quiet > "$GEN/apply/$a.txt" 2>&1
  "$B" "$W/fix.ts" $a --write --quiet --no-scan > "$GEN/apply/$a.again.txt" 2>&1
  echo "$a | $(grep -E "hard findings" "$GEN/apply/$a.txt" | cut -c1-90) | first: $(tail -1 "$GEN/apply/$a.txt" | cut -c1-60) | second: $(tail -1 "$GEN/apply/$a.again.txt" | cut -c1-60)"
done
