cd /home/deck/semio
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️18/DEV-PUZZLE-5D-FULLY-WORKING-END-TO-END/🗑️generated"
for t in check graph-wire-check verify-import-acceptance test; do
  while [ "$(pgrep -fc 'repo run @semio-tech/puzzle-5d-rs')" -ge 3 ]; do sleep 15; done
  repo run @semio-tech/puzzle-5d-rs:$t --lang=en > "$G/rs-$t.log" 2>&1; echo "exit $?" >> "$G/rs-$t.log"
done
