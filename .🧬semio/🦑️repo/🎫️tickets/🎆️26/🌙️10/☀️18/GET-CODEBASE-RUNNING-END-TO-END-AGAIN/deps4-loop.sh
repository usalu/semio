#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=50
while [ $n -lt 70 ]; do
  L="$C/🗑️generated/deps4-sync$n.log"; rm -f "$L"
  "$C/deps4-run.sh" "$L" 7000 bun repo dependencies sync --all
  echo "$n $(tail -1 "$L") $(date +%T)" >> "$C/🗑️generated/deps4-loop.log"
  if grep -aq "^rc=0" "$L"; then echo DONE >> "$C/🗑️generated/deps4-loop.log"; exit 0; fi
  if grep -aq "Cannot find module\|SyntaxError\|System image plist\|unique parent kernel\|manifest changed during verification\|Stale loaded repository\|workspace membership changed\|descriptor changed\|custody changed\|SIGALRM" "$L"; then sleep 60; n=$((n+1)); continue; fi
  echo STOP >> "$C/🗑️generated/deps4-loop.log"; exit 1
done
