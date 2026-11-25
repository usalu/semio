#!/usr/bin/env bash
T="$(cd "$(dirname "$0")" && pwd)"; cd "$T"
for i in 1 2 3 4 5 6; do
  BUILDER_LANE=wasip2 BUILDER_MEM=3000 BUILDER_TIMEOUT=30000 ./builder.sh exec "bash $T/p5-wasm-run.sh" > "$T/🗑️generated/p5-wasm-build-r$i.log" 2>&1
  rc=$?; echo "EXIT $rc" >> "$T/🗑️generated/p5-wasm-build-r$i.log"
  [ $rc = 0 ] && break
  grep -q "signal: 15" "$T/🗑️generated/p5-wasm-build-r$i.log" || break
done
