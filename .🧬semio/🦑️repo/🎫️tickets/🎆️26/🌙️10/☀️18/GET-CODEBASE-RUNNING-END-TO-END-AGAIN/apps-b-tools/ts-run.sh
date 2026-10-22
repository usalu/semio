#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/apps-b-1022"
P=$(cat "$C/apps-b-tools/ts-projects.csv")
n=0
while [ $n -lt 12 ]; do
  n=$((n+1))
  nice -n 10 bun repo run-many --targets=test --projects="$P" --parallel=1 --exclude-dependencies --output=stream > "$G/ts-runmany.log" 2>&1
  rc=$?
  if [ $rc -ne 0 ] && /usr/bin/grep -q "runtime registry\|Laufzeitregistratur\|Uncaptured implementation\|SEMIO_LEASE_ORDER\|Original capture invocation changed\|Stale loaded repository" "$G/ts-runmany.log" && [ $(wc -l < "$G/ts-runmany.log") -lt 40 ]; then sleep 600; continue; fi
  break
done
echo "rc=$rc tries=$n" > "$G/ts-runmany.done"
