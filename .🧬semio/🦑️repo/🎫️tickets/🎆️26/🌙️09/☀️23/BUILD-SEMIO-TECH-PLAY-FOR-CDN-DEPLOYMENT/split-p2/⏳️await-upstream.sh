#!/bin/bash
# ⏳️ Re-runs one lane-P2 cargo call until no crate outside the remodeling family fails to compile (peer fleets edit the
# framework concurrently), at most <rounds> times with <pause> seconds between; prints the final verdict line.
# usage: ⏳️await-upstream.sh <tag> <rounds> <pause> <cargo args...>
TAG=$1; ROUNDS=$2; PAUSE=$3; shift 3
HERE="$(cd "$(dirname "$0")" && pwd)"; LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/p2/logs/$TAG.log"
for i in $(seq 1 "$ROUNDS"); do
  bash "$HERE/🏗️cargo.sh" "$TAG" "$@" > /dev/null
  rc=$?
  broken=$(grep -E 'could not compile `' "$LOG" | grep -v 'semio-s-artifact-remodel-remodeling' | grep -m1 -v 'semio-hub-remodel')
  if [ -z "$broken" ]; then echo "round $i: upstream ok, rc=$rc own errors=$(grep -cE '^(error|.*: error)' "$LOG")"; exit $rc; fi
  echo "round $i $(date +%H:%M): ${broken:0:120}"
  [ "$i" -lt "$ROUNDS" ] && sleep "$PAUSE"
done
exit 1
