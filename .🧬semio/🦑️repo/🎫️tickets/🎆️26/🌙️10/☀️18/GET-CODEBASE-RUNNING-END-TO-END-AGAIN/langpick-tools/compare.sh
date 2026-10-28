#!/bin/sh
# ⚖️ compare.sh [OUTDIR]: for every non-base `<batch>-<id>.log.fails` of a verify run, the failing tests that are NOT in the base failure sets (`<batch>-base-<id>.log.fails`, any batch) of the same test file. Prints `NEW <file-id> <test>` per new failure, then `new failures: N`.
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
OUT=${1:-$HERE/../🗑️generated/${LP_OUT:-langpick}}
new=0
for f in "$OUT"/*.log.fails; do
  case "$f" in *-base-*) continue;; esac
  id=$(echo "$f" | sed -E 's/.*-([0-9]+)\.log\.fails$/\1/')
  tmp=$(mktemp)
  cat "$OUT"/*-base-"$id".log.fails 2>/dev/null | sort -u > "$tmp"
  sort -u "$f" > "$f.u"
  extra=$(comm -13 "$tmp" "$f.u")
  [ -n "$extra" ] && { echo "$extra" | sed "s/^/NEW $id /"; new=$((new + $(echo "$extra" | wc -l))); }
  rm -f "$tmp" "$f.u"
done
echo "new failures: $new"
