#!/bin/sh
# usage: publish-component.sh <name> <manifest>; retries transient warden/admission terminations
cd /Users/ueli/Documents/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
n=0
while [ $n -lt 8 ]; do
  n=$((n+1))
  SEMIO_COMPILER_PARALLELISM=1 repo "$T/native-exec.ts" "$PWD" './🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts' component --manifest "$2" > "$T/🗑️generated/pub-$1.log" 2>&1
  rc=$?
  echo "attempt $n rc=$rc" >> "$T/🗑️generated/pub-$1.attempts"
  [ $rc -eq 0 ] && exit 0
  grep -qE "SIGTERM \(exit 143\)|capacity deadline|timed out|waited 1,?8" "$T/🗑️generated/pub-$1.log" || exit $rc
  cp "$T/🗑️generated/pub-$1.log" "$T/🗑️generated/pub-$1.attempt$n.log"
done
exit 1
