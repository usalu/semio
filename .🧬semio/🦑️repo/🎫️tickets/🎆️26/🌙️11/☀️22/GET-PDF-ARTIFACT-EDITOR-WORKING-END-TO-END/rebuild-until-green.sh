#!/bin/sh
# usage: rebuild-until-green.sh <attempts>  (reruns rebuild-pdf-wasm.sh while the shared tree fails to compile; each attempt waits 10 minutes of quiet first)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
G="$here/🗑️generated"
n=0
while [ "$n" -lt "${1:-6}" ]; do
  n=$((n + 1))
  sh "$here/rebuild-pdf-wasm.sh" > "$G/wasm-build-loop-$n.out" 2>&1
  status=$?
  echo "attempt $n exit $status" >> "$G/wasm-build-loop.log"
  [ "$status" -eq 0 ] && exit 0
  grep -a "^error\[\|could not compile\|SIGKILL" "$G/wasm-build-loop-$n.out" | head -5 >> "$G/wasm-build-loop.log"
  sleep 600
done
exit 1
