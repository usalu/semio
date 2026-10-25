#!/bin/bash
# Ticket input script: focused runs of the shortage reload spec; stops at the first failure.
#   five.sh <stamp-prefix> [count]
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
prefix="$1"; count="${2:-5}"
for n in $(seq 1 "$count"); do
  echo "[five] run $n/$count ($prefix$n) $(date +%H:%M:%S)"
  bash "$here/../course-d-tools/focus.sh" "$prefix$n" dev --project shortage --no-deps --grep "a reload in the middle of a run keeps the answers" || { echo "[five] run $prefix$n FAILED $(date +%H:%M:%S)"; exit 1; }
  echo "[five] run $prefix$n PASSED $(date +%H:%M:%S)"
done
echo "[five] all $count runs PASSED"
