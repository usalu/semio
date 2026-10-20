#!/bin/sh
# 🔬 Runs every test list against the pristine mirror (base) and the patched mirror in parallel and prints the comparison. usage: compare-all.sh <list>...
C="$(cd "$(dirname "$0")/.." && pwd)"; G="$C/🗑️generated/tax-stage"
for L in "$@"; do
  "$C/tax-stage-tools/batch.sh" base "$L" "${SECS:-90}" > /dev/null 2>&1 &
  "$C/tax-stage-tools/batch.sh" mirror "$L" "${SECS:-90}" > /dev/null 2>&1 &
  wait
  echo "=== $L"; python3 "$C/tax-stage-tools/cmp-tests.py" "$G/base-$L.txt" "$G/mirror-$L.txt"
done
echo ALL-DONE
