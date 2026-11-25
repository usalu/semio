#!/bin/zsh
# Runs run-e2e.sh on a private BiDi port and repeats while another agent kills the browser (exit 2).
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
export BIDI_PORT=${BIDI_PORT:-9433}
for i in $(seq 1 ${E2E_ATTEMPTS:-8}); do
  rm -f $T/🗑️generated/e2e-report.json
  $T/run-e2e.sh
  grep -q "^fin 2" $T/🗑️generated/run-e2e-out.txt || break
  echo "attempt $i: browser killed, retrying" >> $T/🗑️generated/run-e2e-attempts.txt
done
echo "retry-fin $(tail -1 $T/🗑️generated/run-e2e-out.txt)" >> $T/🗑️generated/run-e2e-attempts.txt
