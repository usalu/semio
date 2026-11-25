#!/bin/zsh
# 🚦️ The slow proofs of MV-2 through the machine-wide memory queue (coordinator rule of 2026-11-24: every bun proof run goes through `repo capacity run`, 7000 MB for the matrix, 5000 MB for the check CLI), one after the other, from the repo root: queue.sh check <id>… | matrix <id>…   (results in $GEN/final.log, logs in $GEN/proof)
BATCH=MV-2 source "${0:A:h}/../MR/env.sh"
kind="$1"; shift; mkdir -p "$GEN/proof"; cd "$R"
for a in "$@"; do
  s=$SECONDS
  if [[ "$kind" == check ]]; then
    repo capacity run --memory=5000 -- timeout 3000 "$B" "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" > "$GEN/proof/$a-check.log" 2>&1
    echo "check $a exit $? $((SECONDS-s))s $(grep -m1 -E '^(✓|✗)' "$GEN/proof/$a-check.log" | cut -c1-160)" >> "$GEN/final.log"
  else
    rm -rf "$GEN/matrix-$a"
    repo capacity run --memory=7000 -- timeout 2700 "$B" "$T/📊️matrix.ts" --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$GEN/proof/$a-matrix.log" 2>&1
    echo "matrix $a exit $? $((SECONDS-s))s $(grep -E '^\[matrix\]' "$GEN/proof/$a-matrix.log" | tail -1 | cut -c1-200)" >> "$GEN/final.log"
    python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | tail -2 | cut -c1-300 >> "$GEN/final.log"
  fi
done
echo "queue $kind done" >> "$GEN/final.log"
