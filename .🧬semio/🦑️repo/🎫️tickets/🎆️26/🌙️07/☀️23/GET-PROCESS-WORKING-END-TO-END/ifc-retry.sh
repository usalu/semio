#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
L=$1; shift
for i in {1..40}; do
  $T/ifc-one.sh $L "$@"
  if grep -q "test result" /private/tmp/claude-501/-Users-ueli-Documents-semio/8cbd0d6d-32d5-4da7-9277-09843fb54394/scratchpad/$L.txt; then break; fi
  sleep 90
done
echo done > /private/tmp/claude-501/-Users-ueli-Documents-semio/8cbd0d6d-32d5-4da7-9277-09843fb54394/scratchpad/$L.done
