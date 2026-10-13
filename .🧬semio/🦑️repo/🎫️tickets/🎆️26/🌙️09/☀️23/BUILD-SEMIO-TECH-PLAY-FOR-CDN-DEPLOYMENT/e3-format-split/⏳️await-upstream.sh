#!/bin/bash
# ⏳️ Re-runs one E3 `cargo check` until no upstream framework crate fails to compile (concurrent lanes edit them),
# at most <rounds> times with <pause> seconds between; prints the final verdict line.
# usage: ⏳️await-upstream.sh <tag> <rounds> <pause> <cargo args...>
TAG=$1; ROUNDS=$2; PAUSE=$3; shift 3
HERE="$(cd "$(dirname "$0")" && pwd)"; LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/e3/logs/$TAG.log"
for i in $(seq 1 "$ROUNDS"); do
  "$HERE/🏗️cargo.sh" "$TAG" "$@" > /dev/null
  broken=$(grep -m1 'could not compile `semio-framework' "$LOG")
  if [ -z "$broken" ]; then echo "round $i: upstream ok, own errors=$(grep -c '^error' "$LOG")"; exit 0; fi
  echo "round $i: ${broken:0:100}"
  [ "$i" -lt "$ROUNDS" ] && sleep "$PAUSE"
done
exit 1
