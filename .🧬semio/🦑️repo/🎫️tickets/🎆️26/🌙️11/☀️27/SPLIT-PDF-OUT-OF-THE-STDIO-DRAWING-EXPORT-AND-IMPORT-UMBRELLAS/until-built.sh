#!/usr/bin/env bash
# 🔁️ Re-runs a build script until it ends in a compiler verdict (errors or test results) rather than a host or tooling
# interruption; each run reuses the units the last one cached: until-built.sh <script> <log>
for attempt in 1 2 3 4 5 6 7 8 9 10; do
  "$1" > "$2" 2>&1; rc=$?
  if [ "$rc" = 0 ] || grep -qE '^error\[E[0-9]|^test result|^[^ ]+:[0-9]+:[0-9]+: error' "$2"; then echo "attempt $attempt exit $rc"; exit $rc; fi
  echo "attempt $attempt interrupted after $(grep -c Compiling "$2") units: $(grep -m1 -E 'Internal error|needs a live' "$2" | cut -c1-120)"; sleep 90
done
exit 1
