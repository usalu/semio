#!/bin/sh
# usage: chain7.sh <batches-file> <prefix> <first> <last>
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
n=0
while read -r batch; do
  n=$((n+1))
  [ $n -lt $3 ] && continue
  [ $n -gt $4 ] && break
  sh $C/ts-c-tools/many.sh $2$n test "$batch"
done < $C/ts-c-tools/$1
