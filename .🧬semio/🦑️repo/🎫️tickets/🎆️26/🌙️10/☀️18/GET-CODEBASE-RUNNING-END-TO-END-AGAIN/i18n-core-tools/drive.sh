#!/bin/sh
# usage: drive.sh STEP...   runs the increments in order on the chain overlay and stops at the first failure
cd "$(dirname "$0")" || exit 1
for s in "$@"; do
  echo "=== $s"
  python3 inc.py "$s" || { echo "FAILED $s"; exit 1; }
  python3 parsecheck.py "$s"
done
echo "ALL DONE"
