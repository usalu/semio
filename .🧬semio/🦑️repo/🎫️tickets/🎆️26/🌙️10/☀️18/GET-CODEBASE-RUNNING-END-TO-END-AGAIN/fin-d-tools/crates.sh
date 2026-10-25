#!/bin/sh
# usage: crates.sh <tag> <crate>...  graph-free sequential `bun repo cargo ... test -p <crate>`; summary in <tag>.sum
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/fin-d"
tag=$1; shift
for c in "$@"; do
  nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test -p "$c" --no-fail-fast --message-format=short > "$O/$tag-$c.log" 2>&1
  rc=$?
  echo "$(date +%H:%M:%S) $c rc=$rc $(/usr/bin/grep -E '^test result' "$O/$tag-$c.log" | tr '\n' ' ' | cut -c1-200)" >> "$O/$tag.sum"
done
echo done >> "$O/$tag.sum"
