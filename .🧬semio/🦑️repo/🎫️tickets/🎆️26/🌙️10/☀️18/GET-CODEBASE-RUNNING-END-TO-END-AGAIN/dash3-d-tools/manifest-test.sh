#!/bin/sh
# usage: manifest-test.sh <tag> <manifest-path> [cargo test args...]  graph-free cargo test of a standalone manifest; summary in <tag>.sum
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/dash3-d"; tag=$1; m=$2; shift 2
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test --manifest-path "$m" --no-fail-fast --message-format=short "$@" > "$O/$tag.log" 2>&1
rc=$?
echo "$(date +%H:%M:%S) rc=$rc $(/usr/bin/grep -E '^test result' "$O/$tag.log" | tr '\n' ' ' | cut -c1-400)" >> "$O/$tag.sum"
