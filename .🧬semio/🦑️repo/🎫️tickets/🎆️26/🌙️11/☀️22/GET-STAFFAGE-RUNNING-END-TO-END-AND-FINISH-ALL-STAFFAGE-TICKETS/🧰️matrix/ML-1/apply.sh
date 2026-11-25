#!/bin/zsh
# ✍️ Writes the repair of assets into the catalog under one grant of the memory queue, one asset after the other: `fix.ts <id> --write` (the chain: the step of MF or MO, then `spec/<id>.json`; the document is read right before it is written and the authors' gate of the result is printed), then once more to show that the second run changes nothing; the log of each goes to 🗑️generated/matrix/ML-1/apply/<id>.txt and one verdict line is printed: apply.sh <id>…
source "${0:A:h}/env.sh"
mkdir -p "$GEN/apply"; cmd="cd '$T'"
for a in "$@"; do cmd="$cmd; '$B' '$W/fix.ts' $a --write --quiet > '$GEN/apply/$a.txt' 2>&1; '$B' '$W/fix.ts' $a --write --quiet --no-scan > '$GEN/apply/$a.again.txt' 2>&1"; done
qs 1000 3600 "$cmd" > "$GEN/apply/queue.log" 2>&1
for a in "$@"; do echo "$a | $(grep -E "hard findings" "$GEN/apply/$a.txt" | cut -c1-90) | first: $(tail -1 "$GEN/apply/$a.txt" | cut -c1-60) | second: $(tail -1 "$GEN/apply/$a.again.txt" | cut -c1-60)"; done
