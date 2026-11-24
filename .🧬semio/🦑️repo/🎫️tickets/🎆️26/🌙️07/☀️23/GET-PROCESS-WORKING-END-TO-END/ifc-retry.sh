#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
L=$1; shift
for i in {1..40}; do
  $T/ifc-one.sh $L "$@"
  if grep -q "test result" $T/🗑️generated/$L.txt; then break; fi
  sleep 90
done
echo done > $T/🗑️generated/$L.done
