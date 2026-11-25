#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
W=$T/🗑️generated/native/target/wasm32-wasip2/wasm-prototype/semio_hub_process_component.wasm
rm -f $T/🗑️generated/hub-retry.done
for i in $(seq 1 ${HUB_RETRIES:-12}); do
  rm -f $T/🗑️generated/hub-build.done; HUB_ATTEMPTS=1 $T/hub-build.sh
  [ -f $W ] && [ $W -nt $T/🗑️generated/hub-retry.log ] && break
  echo "attempt $i failed: $(grep -am1 '^error' $T/🗑️generated/hub-build.txt | cut -c1-160)" >> $T/🗑️generated/hub-retry.log; sleep 120
done
echo done > $T/🗑️generated/hub-retry.done
