#!/bin/sh
# usage: retry.sh <name> <repo args...>  one run after the other while the fleet image fault 'Sole exact committed recomputable' is the first failure (max 12 attempts, 150 s apart); logs <name>.aN.log, final rc in <name>.rc
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=$1; shift
i=1
while [ $i -le 12 ]; do
  log="$C/🗑️generated/dtest2-d/$n.a$i.log"
  nice -n 10 bun repo "$@" > "$log" 2>&1
  rc=$?
  if [ $rc -eq 0 ] || ! grep -q 'Sole exact committed recomputable' "$log"; then echo $rc > "$C/🗑️generated/dtest2-d/$n.rc"; echo "attempt $i" >> "$C/🗑️generated/dtest2-d/$n.rc"; exit $rc; fi
  i=$((i+1)); sleep 150
done
echo 99 > "$C/🗑️generated/dtest2-d/$n.rc"
