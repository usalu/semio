#!/bin/sh
# 🏗️ Flow hub once, then procedural; sequential, only while no other :component build runs; one retry on a resident-service failure. Logs in lazy-logs.
cd /Users/ueli/Documents/semio || exit 1
TF=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
L="$TF/lazy-logs"
for name in semio-hub-flow-component semio-hub-procedural-component; do
  for attempt in 1 2; do
    while ps -eo command | grep -E "^[^ ]*(bun|repo) .*run .*component" | grep -v -E "grep|lazy-activation" | grep -q "run [a-z-]*:component"; do sleep 30; done
    mkdir -p "$L"; echo "$(date +%T) start $name try $attempt" >> "$L/run.txt"
    repo run "$name:component" -c prototype --exclude-dependencies > "$L/$name-$attempt.txt" 2>&1
    code=$?
    mkdir -p "$L"; echo "$(date +%T) end $name try $attempt exit $code" >> "$L/run.txt"
    [ "$code" -eq 0 ] && break
    grep -q "resident service met an unexpected failure" "$L/$name-$attempt.txt" || break
  done
  [ "$code" -eq 0 ] || break
done
