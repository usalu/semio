#!/bin/sh
# usage: crates.sh <tag> <crate>...   sequential graph-free cargo test per crate; retries stale-implementation churn; summary in <tag>.sum
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/dash3-d"
tag=$1; shift
for c in "$@"; do
  try=1
  while :; do
    nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test -p "$c" --no-fail-fast --message-format=short > "$O/$tag-$c.log" 2>&1
    rc=$?
    if [ $rc -ne 0 ] && [ $try -lt 4 ] && /usr/bin/grep -q 'SEMIO_STALE_IMPLEMENTATION\|Stale loaded Source revision\|Current shared Source-owner generation changed' "$O/$tag-$c.log"; then try=$((try+1)); continue; fi
    break
  done
  echo "$(date +%H:%M:%S) $c rc=$rc tries=$try $(/usr/bin/grep -E '^test result' "$O/$tag-$c.log" | tr '\n' ' ' | cut -c1-300)" >> "$O/$tag.sum"
done
echo done >> "$O/$tag.sum"
