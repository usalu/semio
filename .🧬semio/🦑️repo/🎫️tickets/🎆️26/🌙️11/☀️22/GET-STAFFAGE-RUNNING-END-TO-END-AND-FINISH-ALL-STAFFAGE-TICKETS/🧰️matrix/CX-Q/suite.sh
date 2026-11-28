#!/bin/zsh
# 🧪️ Runs the named suites in one vitest process through the memory queue: suite.sh <log> <memory MB> <suite folder…>
D="${0:A:h}"; source "$D/../MR/env.sh"
log="$1"; mb="$2"; shift 2
cd "$R" && repo capacity run --memory="$mb" -- timeout 3400 "$B" node_modules/vitest/vitest.mjs run --reporter=verbose --configLoader native --maxWorkers=1 --config "$S/🧪️tests/🎚️config/🟦️.ts" "${@/#//}" --testTimeout=3000000 --hookTimeout=3000000 > "$log" 2>&1
echo "exit $?" >> "$log"
