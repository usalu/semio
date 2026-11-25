#!/bin/zsh
# 🧷️ Writes the repair of assets into the catalog: runs the ops writer `make/<id>.py`, applies `ops/<id>.json` with apply.ts (which reads the document right before it writes it), then runs the writer and the apply again and expects the same ops and "file unchanged" (the chain is idempotent; a writer that finds its repair in place keeps its ops). Prints one line per asset and appends it to $GEN/log/written.txt: write.sh <id>…   (run it through q.sh with 1000 MB)
source "${0:h}/env.sh"
mkdir -p "$GEN/ops" "$GEN/log"
for a in "$@"; do
  [[ -f "$GEN/backup/$a.json" && ! -f "$GEN/ops/$a.json" ]] && ! cmp -s "$GEN/backup/$a.json" "$(asset "$a")" && echo "$a: the catalog document changed since the backup (someone else wrote it); the ops are applied to the document as it is now" | tee -a "$GEN/log/written.txt"
  python3 "$W/make/$a.py" > /dev/null || { echo "$a: the ops writer failed"; continue; }
  cp "$W/ops/$a.json" "$GEN/ops/$a.json"
  first="$(cd "$P" && timeout 300 "$B" "$W/apply.ts" "$a" 2>&1 | tail -1 | cut -c1-200)"
  python3 "$W/make/$a.py" > /dev/null 2>&1
  same="same ops"; cmp -s "$W/ops/$a.json" "$GEN/ops/$a.json" || same="THE SECOND RUN WROTE OTHER OPS"
  second="$(cd "$P" && timeout 300 "$B" "$W/apply.ts" "$a" 2>&1 | tail -1 | cut -c1-200)"
  [[ "$second" == *"file unchanged"* ]] && second="second run: file unchanged" || second="SECOND RUN CHANGED THE FILE: $second"
  echo "$(date +%H:%M) $first | $same | $second" | tee -a "$GEN/log/written.txt"
done
