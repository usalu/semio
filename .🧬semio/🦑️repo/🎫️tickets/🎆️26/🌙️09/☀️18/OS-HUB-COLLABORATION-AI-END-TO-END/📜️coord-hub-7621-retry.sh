#!/bin/zsh
# 🔁 Coordinator: restart hub 7621 (jc1-boot, target-jc1 binary, credential sign-in) every 3 min until /readyz answers 200; the old binary's 30 s wall budget trips under load, so this waits the load out.
T="$(cd "$(dirname "$0")" && pwd)"
for attempt in {1..40}; do
  OS_HUB_CREDENTIAL_SIGN_IN=true nohup bun "$T/🐍️ds1-hub-hold.ts" 7621 "/Users/ueli/Documents/semio/.🧬semio/🌐hub/jc1-boot" "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-jc1/debug/os-hub" > "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/coordinator/coord-hub-7621.txt" 2>&1 < /dev/null &
  pid=$!; echo $pid > "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/coordinator/coord-hub-7621-pid.txt"
  SECONDS=0
  until [ "$(curl -s -o /dev/null -m 2 -w '%{http_code}' http://127.0.0.1:7621/readyz)" = "200" ] || ! kill -0 $pid 2>/dev/null || [ $SECONDS -gt 240 ]; do sleep 5; done
  code=$(curl -s -o /dev/null -m 2 -w '%{http_code}' http://127.0.0.1:7621/readyz)
  echo "attempt $attempt $(date '+%H:%M:%S') 7621=$code load $(uptime | sed 's/.*averages: //' | cut -d, -f1)" >> "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/coordinator/coord-hub-7621-retry.txt"
  [ "$code" = "200" ] && exit 0
  sleep 180
done
exit 1
