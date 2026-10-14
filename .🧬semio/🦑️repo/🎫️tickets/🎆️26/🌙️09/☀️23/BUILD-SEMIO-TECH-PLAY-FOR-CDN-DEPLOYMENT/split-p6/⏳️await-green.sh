#!/bin/bash
# ⏳️ Re-runs one P6 cargo check until no upstream (non-family) crate fails, or the deadline passes. Prints the last result.
# usage: ⏳️await-green.sh <tag> <family-prefix> <seconds> <cargo args...>
TAG=$1; PREFIX=$2; LIMIT=$3; shift 3
HERE="$(cd "$(dirname "$0")" && pwd)"; LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/p6/logs/$TAG.log"
END=$(( $(date +%s) + LIMIT ))
while :; do
  "$HERE/🦀️cargo.sh" "$TAG" "$@"
  UP=$(grep -E '^error: could not compile' "$LOG" | grep -v "\`$PREFIX" | head -3)
  if [ -z "$UP" ]; then echo "upstream green"; exit 0; fi
  echo "upstream red: $UP"
  [ "$(date +%s)" -ge "$END" ] && exit 3
  sleep 120
done
