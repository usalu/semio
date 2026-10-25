#!/bin/sh
# usage: chain5.sh <first> <last>   batches of ts-c-tools/$BATCHES
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
n=0
while read -r batch; do
  n=$((n+1))
  [ $n -lt $1 ] && continue
  [ $n -gt $2 ] && break
  sh $C/ts-c-tools/many.sh b$n test "$batch"
done < $C/ts-c-tools/$BATCHES
