#!/bin/sh
# usage: publish-component.sh <name> <manifest> [initial-MiB]
# Wraps the component driver in a retained outer grant and sizes it from the driver's own requirement: a refusal
# `Component build needs N MiB for the local pass` raises the next grant to N plus a margin; warden cancellations and
# admission timeouts retry unchanged. Never bypasses the capacity gate or the warden.
cd /Users/ueli/Documents/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
G="${SEMIO_PUBLISH_LOGS:-/private/tmp/claude-501/-Users-ueli-Documents-semio/9ea38fee-6468-41de-903a-f7fec4948a18/scratchpad}"; mkdir -p "$G"
mem="${3:-4400}"; margin=400; n=0
while [ $n -lt 40 ]; do
  n=$((n+1))
  SEMIO_COMPILER_PARALLELISM=1 repo capacity run --memory="$mem" --cpu=1 -- repo "$T/native-exec.ts" "$PWD" './🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts' component --manifest "$2" > "$G/pub-$1.log" 2>&1
  rc=$?
  echo "attempt $n mem=$mem rc=$rc" >> "$G/pub-$1.attempts"
  [ $rc -eq 0 ] && exit 0
  need=$(grep -a -o "Component build needs [0-9]* MiB for the local pass" "$G/pub-$1.log" | tail -1 | grep -o "[0-9]*")
  if [ -n "$need" ]; then mem=$((need+margin)); cp "$G/pub-$1.log" "$G/pub-$1.attempt$n.log"; continue; fi
  grep -qaE "SIGTERM|capacity deadline|timed out|waited 1,?8|Wächter|warden" "$G/pub-$1.log" || exit $rc
  cp "$G/pub-$1.log" "$G/pub-$1.attempt$n.log"
done
exit 1
