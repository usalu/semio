#!/bin/sh
# usage: c2.sh <cycle-id> ; one full guard cycle, one product at a time: dash, print, course, course:dev, dashboard :run smoke, projektetage dev smoke
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
id="$1"; G="$C/🗑️generated/integ-g"
for prod in dash print course; do
  while :; do sh "$C/integ-g-tools/cycle.sh" "$id" "$prod"; rc=$?; [ $rc -ne 3 ] && break; sleep 120; done
done
sh "$C/integ-g-tools/devcheck.sh" "$id" > "$G/$id.devcheck.out" 2>&1
sh "$C/integ-g-tools/dash-run-smoke.sh" "$id" > "$G/$id.dashrun.out" 2>&1
sh "$C/integ-g-tools/projektetage-smoke.sh" "$id" > "$G/$id.projektetage.out" 2>&1
echo done > "$G/$id.finished"
