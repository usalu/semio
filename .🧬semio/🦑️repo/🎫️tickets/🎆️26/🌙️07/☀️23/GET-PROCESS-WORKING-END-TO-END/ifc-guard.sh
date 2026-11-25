#!/bin/zsh
# usage: ifc-guard.sh <label> <filter> <seconds>: runs one filtered test, samples and kills the test binary when it exceeds <seconds>
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/8cbd0d6d-32d5-4da7-9277-09843fb54394/scratchpad
L=$1; F=$2; N=$3
$T/ifc-retry.sh $L "$F" -- >/dev/null 2>&1 &
RP=$!
for i in {1..2000}; do
  sleep 1
  [ -f $S/$L.done ] && exit 0
  P=$(pgrep -f "semio_s_artifact_stdio_ifc_4x3-.*$F" | head -1)
  if [ -n "$P" ]; then
    [ -z "$START" ] && START=$i
    if [ $((i-START)) -gt $N ]; then sample $P 5 -file $S/$L.sample >/dev/null 2>&1; kill -9 $P; echo "guard killed $P after $N s" > $S/$L.guard; exit 0; fi
  fi
done
