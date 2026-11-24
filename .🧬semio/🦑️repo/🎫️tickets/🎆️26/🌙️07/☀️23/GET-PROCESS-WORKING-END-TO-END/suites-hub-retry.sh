#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
for i in 1 2 3 4 5 6; do
  SUITES_CWD="/Users/ueli/Documents/semio/🌎️hub" $T/suites-root.sh hub test -p semio-hub-process -p semio-hub-process-component --no-fail-fast
  grep -q "^error: could not compile" $T/🗑️generated/suites-hub.txt || break
  sleep 120
done
touch $T/🗑️generated/suites-hub.done
