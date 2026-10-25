#!/bin/bash
# Ticket input script: five consecutive focused runs of the four "with 200% text" pets specs; stops at the first failing run.
#   five.sh <stamp-prefix> [count]
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
prefix="$1"; count="${2:-5}"
for n in $(seq 1 "$count"); do
  echo "[five] run $n/$count ($prefix$n) $(date +%H:%M:%S)"
  bash "$here/../course-d-tools/focus.sh" "$prefix$n" dev --project pets --no-deps --grep "with 200% text" || { echo "[five] run $prefix$n FAILED $(date +%H:%M:%S)"; exit 1; }
  echo "[five] run $prefix$n PASSED $(date +%H:%M:%S)"
done
echo "[five] all $count runs PASSED"
