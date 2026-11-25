#!/bin/zsh
# 📊️ The matrix confirmation of plants of MV-2 through the machine-wide memory queue, one after the other, from the repo root: `MEM=<MB> matrix.sh <id>…` runs `T/📊️matrix.ts --language en --out $GEN/matrix-<id> --jobs 1 --fresh --assets <id>` under `timed.sh` (the peak of the process tree's largest process is printed, so the reservation can follow the measurement; default 1200 MB, TMO seconds of timeout, default 7000; the queue rule of 2026-11-25 09:50), writes the log to `$GEN/proof/<id>-matrix.log` and appends the result line and the cell summary of `../MR/cells.py` to `$GEN/final2.log`.
BATCH=MV-2 source "${0:A:h}/../MR/env.sh"
mkdir -p "$GEN/proof"; cd "$R"
for a in "$@"; do
  s=$SECONDS; rm -rf "$GEN/matrix-$a"
  repo capacity run --memory=${MEM:-1200} -- "$W/timed.sh" timeout ${TMO:-7000} "$B" "$T/📊️matrix.ts" --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$GEN/proof/$a-matrix.log" 2>&1
  echo "matrix $a exit $? $((SECONDS-s))s peak $(grep 'maximum resident' "$GEN/proof/$a-matrix.log" | awk '{printf "%d", $1/1048576}') MB $(grep -E '^\[matrix\]' "$GEN/proof/$a-matrix.log" | tail -1 | cut -c1-200)" >> "$GEN/final2.log"
  python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | cut -c1-300 >> "$GEN/final2.log"
done
