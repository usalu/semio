#!/bin/zsh
# 🧪️ fw.sh <name>: framework GLB tests through the queue, log in $GEN/fw-<name>.log
source "${0:A:h}/env.sh"
cd "$R" && repo capacity run --memory=2500 -- timeout 1800 $B node_modules/vitest/vitest.mjs run --configLoader native --maxWorkers=1 --config "$R/🧰️framework/🧪️tests/🎚️config/🟦️.ts" "🧊️glb/🧪️tests" > "$GEN/fw-$1.log" 2>&1
echo "exit $?" >> "$GEN/fw-$1.log"
