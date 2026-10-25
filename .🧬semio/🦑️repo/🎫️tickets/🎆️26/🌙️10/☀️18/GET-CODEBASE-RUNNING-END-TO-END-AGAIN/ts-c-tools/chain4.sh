#!/bin/sh
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
mv $C/🗑️generated/ts-c/m3.log $C/🗑️generated/ts-c/m3-try2.log 2>/dev/null; rm -f $C/🗑️generated/ts-c/m3.rc
sh $C/ts-c-tools/many.sh m3 typecheck @semio-tech/framework-os
n=0
while read -r batch; do
  n=$((n+1))
  sh $C/ts-c-tools/many.sh b$n test "$batch"
done < $C/ts-c-tools/batches.txt
