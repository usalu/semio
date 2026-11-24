#!/bin/zsh
# 📊️ Runs the coverage matrix of the suite for assets one after the other (one job each) into 🗑️generated/matrix/MF/matrix/<id> and prints the cells by status and the failing checks of each: matrix.sh <id>…
export BATCH=MF; source "${0:A:h}/../MR/env.sh"
for a in "$@"; do
  out="$GEN/matrix/$a"; mkdir -p "$out"; start=$(date +%s)
  (cd "$T" && timeout ${MATRIX_TIMEOUT:-5400} "$B" 📊️matrix.ts --language en --out "$out" --jobs 1 --fresh --assets "$a") > "$out/run.log" 2>&1
  echo "$a | exit $? | $(( $(date +%s) - start )) s | $(python3 "$W/cells.py" "$out")"
done
