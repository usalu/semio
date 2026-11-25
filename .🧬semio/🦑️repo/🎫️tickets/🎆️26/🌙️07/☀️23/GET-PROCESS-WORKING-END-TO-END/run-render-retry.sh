#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
for i in $(seq 1 ${E2E_ATTEMPTS:-6}); do
  rm -f $T/🗑️generated/e2e-report.json
  $T/run-forest-render.sh
  grep -q "^fin 2" $T/🗑️generated/${TAG:-forest}-out.txt || break
  echo "attempt $i killed" >> $T/🗑️generated/render-attempts.txt
done
