#!/bin/sh
# usage: group.sh <tag> <crate>...   ONE cargo test invocation for all crates (shared compile), stale-churn retry; summary in <tag>.sum
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/dtest-d"
tag=$1; shift
args=""; for c in "$@"; do args="$args -p $c"; done
try=1
while :; do
  nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test $args --no-fail-fast --message-format=short > "$O/$tag.log" 2>&1
  rc=$?
  if [ $rc -ne 0 ] && [ $try -lt 4 ] && /usr/bin/grep -q 'SEMIO_STALE_IMPLEMENTATION\|Stale loaded Source revision\|Current shared Source-owner generation changed' "$O/$tag.log"; then try=$((try+1)); continue; fi
  break
done
echo "$(date +%H:%M:%S) $tag rc=$rc tries=$try" >> "$O/$tag.sum"
/usr/bin/grep -E '^test result|Running|^error|FAILED|failed' "$O/$tag.log" | cut -c1-200 >> "$O/$tag.sum"
echo done >> "$O/$tag.sum"
