#!/bin/zsh
# 🧪️ t.sh <name> <filter>: staffage vitest filter through the queue, log in $GEN/t-<name>.log
source "${0:A:h}/env.sh"
cd "$R" && repo capacity run --memory=2500 -- timeout 3000 $B node_modules/vitest/vitest.mjs run --configLoader native --maxWorkers=1 --config "$S/🧪️tests/🎚️config/🟦️.ts" "$2" "$3" --testTimeout=3000000 --hookTimeout=3000000 > "$GEN/t-$1.log" 2>&1
echo "exit $?" >> "$GEN/t-$1.log"
