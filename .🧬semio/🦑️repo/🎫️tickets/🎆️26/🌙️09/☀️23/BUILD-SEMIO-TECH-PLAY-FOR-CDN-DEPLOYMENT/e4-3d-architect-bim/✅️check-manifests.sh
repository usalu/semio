#!/bin/bash
# ✅️ Checks each given package manifest (`--lib` plus extra args, private E4 dirs, slot-gated, bounded) and prints one
# verdict line per manifest with its first error. usage: ✅️check-manifests.sh <tag> <seconds-each> "<extra args>" <manifest>...
TAG=$1; SECS=$2; EXTRA=$3; shift 3
HERE="$(cd "$(dirname "$0")" && pwd)"; E4=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/e4; i=0
for m in "$@"; do
  i=$((i+1)); rm -f "$E4/logs/$TAG-$i.log"
  out=$(bash "$HERE/🏗️cargo.sh" "$TAG-$i" "$SECS" check --offline --manifest-path "$m" --lib $EXTRA --message-format short 2>&1 | grep -m1 "^rc=")
  err=$(grep -m1 -E ': error|^error' "$E4/logs/$TAG-$i.log" | cut -c1-220)
  echo "$out | $m | $err"
done
