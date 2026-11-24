#!/bin/zsh
# 🧷️ Writes the repair of assets into the catalog: runs the ops writer `make/<id>.py`, applies `ops/<id>.json` with apply.ts (which reads the document right before it writes it), then runs the writer and the apply again and expects the same ops and "file unchanged" (the chain is idempotent; a writer that finds its repair in place keeps its ops): app.sh <id>…
source "${0:A:h}/env.sh"
mkdir -p "$GEN/ops"
for a in "$@"; do
  python3 "$W/make/$a.py" > /dev/null
  cp "$W/ops/$a.json" "$GEN/ops/$a.json"
  pb apply.ts "$a" | cut -c1-160
  python3 "$W/make/$a.py" > /dev/null 2>&1
  cmp -s "$W/ops/$a.json" "$GEN/ops/$a.json" || echo "   THE SECOND RUN WROTE OTHER OPS: $a"
  pb apply.ts "$a" | grep -q "file unchanged" && echo "   second run: file unchanged" || echo "   SECOND RUN CHANGED THE FILE: $a"
done
