#!/usr/bin/env bash
# 📊️ Runs the coverage matrix of the suite for assets one after the other (one job) into 🗑️generated/matrix/MB-2/matrix/<id> and prints the cells by status and the failing checks of each: matrix.sh <id>…
source "$(dirname "$0")/env.sh"
for a in "$@"; do
  out="$G/matrix/$a"; mkdir -p "$out"
  (cd "$T" && timeout ${TIMEOUT:-3000} "$BUN" "📊️matrix.ts" --language en --out "$out" --jobs 1 --assets "$a") > "$out/run.log" 2>&1
  echo "$a | exit $? | $(python3 "$M/cells.py" "$out")"
done
