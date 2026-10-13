#!/bin/bash
# ⏳️ Re-runs one E3b cargo call until no upstream framework crate fails to compile (concurrent lanes edit them),
# at most <rounds> times with <pause> seconds between; prints the final verdict line.
# usage: ⏳️await-upstream.sh <tag> <rounds> <pause> <cargo args...>
TAG=$1; ROUNDS=$2; PAUSE=$3; shift 3
HERE="$(cd "$(dirname "$0")" && pwd)"; LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/e3b/logs/$TAG.log"
for i in $(seq 1 "$ROUNDS"); do
  "$HERE/🏗️cargo.sh" "$TAG" "$@" > /dev/null
  rc=$?
  broken=$(grep -m1 -E 'could not compile `semio-framework|could not compile `semio-s-artifact-stdio-(contract|binary|semio)' "$LOG")
  if [ -z "$broken" ]; then echo "round $i: upstream ok, rc=$rc own errors=$(grep -c '^error' "$LOG")"; exit $rc; fi
  echo "round $i: ${broken:0:100}"
  [ "$i" -lt "$ROUNDS" ] && sleep "$PAUSE"
done
exit 1
