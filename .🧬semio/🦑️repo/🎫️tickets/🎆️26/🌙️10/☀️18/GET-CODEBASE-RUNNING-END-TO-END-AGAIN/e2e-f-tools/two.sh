#!/bin/bash
# Ticket input script: two consecutive focused runs (--repeat-each 8) of the four "with 200% text" pets specs on the dev topology; stops at the first failing run.
#   two.sh <stamp-prefix> [topology]
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
prefix="$1"; topology="${2:-dev}"
for n in 1 2; do
  echo "[two] run $n ($prefix$n) $(date +%H:%M:%S)"
  bash "$here/../course-d-tools/focus.sh" "$prefix$n" "$topology" --project pets --no-deps --repeat-each 8 --grep "with 200% text" || { echo "[two] run $prefix$n FAILED $(date +%H:%M:%S)"; exit 1; }
  echo "[two] run $prefix$n PASSED $(date +%H:%M:%S)"
done
echo "[two] both runs PASSED"
