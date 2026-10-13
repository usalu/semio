#!/bin/bash
# 🔄️ Re-runs `🔁️fix-loop.py <tag>-<n> --apply <args>` until the plugin checks clean; a broken dependency (a peer lane
# mid-edit) waits a minute and retries instead of counting as clean. usage: 🔄️until-clean.sh <tag> [fix-loop args...]
HERE="$(cd "$(dirname "$0")" && pwd)"; TAG=$1; shift
for n in $(seq 1 40); do
  out=$(python3 "$HERE/🔁️fix-loop.py" "$TAG-$n" --apply "$@" 2>&1)
  echo "$out" | grep -E "errors,|DEPENDENCY|widened [0-9]+ items|unresolved [1-9]"
  if echo "$out" | grep -q "DEPENDENCY BROKEN"; then sleep 60; continue; fi
  echo "$out" | grep -q ": 0 errors" && exit 0
done
exit 1
