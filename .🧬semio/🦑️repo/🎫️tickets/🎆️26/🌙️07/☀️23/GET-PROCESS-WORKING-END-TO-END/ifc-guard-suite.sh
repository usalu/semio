#!/bin/zsh
# usage: ifc-guard-suite.sh <label> <idle-seconds>: full 4x3 suite; samples and kills the test binary when its log stays silent that long
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/8cbd0d6d-32d5-4da7-9277-09843fb54394/scratchpad
L=$1; N=$2
$T/ifc-retry.sh $L -- --test-threads=4 >/dev/null 2>&1 &
while [ ! -f $S/$L.done ]; do
  sleep 20
  P=$(pgrep -f "semio_s_artifact_stdio_ifc_4x3-" | head -1)
  [ -z "$P" ] && continue
  AGE=$(( $(date +%s) - $(stat -f %m $S/$L.txt) ))
  if [ $AGE -gt $N ]; then sample $P 5 -file $S/$L.sample >/dev/null 2>&1; tail -3 $S/$L.txt > $S/$L.guard; kill -9 $P; fi
done
