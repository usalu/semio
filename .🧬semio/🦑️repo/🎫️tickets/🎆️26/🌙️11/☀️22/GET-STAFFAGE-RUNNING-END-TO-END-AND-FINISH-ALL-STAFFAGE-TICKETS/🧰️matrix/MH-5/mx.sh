#!/bin/zsh
# 🧮️ The matrix confirmation of people of this batch under a measured reservation: mx.sh <id>… runs `T/📊️matrix.ts --assets <id>` one after the other, each under its own grant of ${MEM:-600} MB (measured on seated-guitarist: peak of the process tree 392 MB, plus 30 %; the queue rule of 2026-11-25 09:20 allows 1200) of the machine-wide queue and under `../MM/guarded.sh` (the process tree is stopped when it holds more resident memory than the grant, and the log ends with its peak: the measurement the takeover brief asks for before less than 7000 MB is reserved). Output in $GEN/matrix-<id>, log $GEN/proof/<id>-matrix.log, one result line and the cells by status on stdout.
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
mem="${MEM:-600}"; mkdir -p "$GEN/proof"
for a in "$@"; do
  rm -rf "$GEN/matrix-$a"
  (cd "$R" && lockf -k "${TMPDIR:-/tmp}/semio-matrix-MH-5.lock" repo capacity run --memory="$mem" -- /bin/zsh -c 'cd "$1" && shift && exec "$@"' mx "$T" "$U/🧰️matrix/MM/guarded.sh" "$mem" "$GEN/proof/$a-matrix.log" timeout 9000 "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a") > "$GEN/proof/$a-matrix.queue.log" 2>&1
  echo "matrix $a $(date +%H:%M): $(grep -E '^\[matrix\] [0-9]+ cells' "$GEN/proof/$a-matrix.log" | tail -1 | cut -c1-160) | $(tail -1 "$GEN/proof/$a-matrix.log" | cut -c1-140)"
  python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | cut -c1-260
done
