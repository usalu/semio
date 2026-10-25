#!/bin/sh
# usage: runlist.sh <listfile> -> prints file tail summary per file
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
while read f; do
  sh "$C/repo-tests-c-tools/one.sh" "$f" > "$C/🗑️generated/repo-tests-c/one/rl.log" 2>&1
  printf '%s | ' "$(/usr/bin/grep '^ *[0-9]* \(pass\|fail\)' "$C/🗑️generated/repo-tests-c/one/rl.log" | tr -s ' \n' ' ')"; echo "$f" | sed 's#.*/library/##' | cut -c1-90
done < "$1"
