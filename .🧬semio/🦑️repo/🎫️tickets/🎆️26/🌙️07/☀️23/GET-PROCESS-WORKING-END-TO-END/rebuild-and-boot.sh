#!/bin/zsh
# Rebuild the hub, republish it and boot-check the page; repeats while the shared tree is mid-edit (build or boot failure).
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
cd /Users/ueli/Documents/semio
for i in $(seq 1 ${ATTEMPTS:-6}); do
  rm -f $T/🗑️generated/hub-build.done; HUB_ATTEMPTS=1 $T/hub-build.sh
  grep -q "^exit 0" $T/🗑️generated/hub-build.done || { sleep 300; continue; }
  $B $T/publish-hub.ts > $T/🗑️generated/publish.txt 2>&1 || { sleep 300; continue; }
  pkill -f "remote-debugging-port=9333"; sleep 2
  rm -f $T/🗑️generated/e2e-report.json
  STEPS='[]' SETTLE_MS=${BOOT_MS:-90000} $B $T/verify-process-firefox-bidi.ts > $T/🗑️generated/boot-check.txt 2>&1
  [ $? -eq 0 ] && { echo "boot ok attempt $i" > $T/🗑️generated/rebuild.done; exit 0; }
  sleep 300
done
echo "boot failed" > $T/🗑️generated/rebuild.done
