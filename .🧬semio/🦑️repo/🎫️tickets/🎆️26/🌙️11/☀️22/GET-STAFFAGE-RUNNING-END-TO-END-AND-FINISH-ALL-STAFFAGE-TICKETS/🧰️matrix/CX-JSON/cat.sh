#!/bin/zsh
# 🧪️ cat.sh <name> <ids comma separated>: single-asset model-catalog run through the queue, log in $GEN/cat-<name>.log
source "${0:A:h}/env.sh"
cd "$R" && SEMIO_STAFFAGE_MODEL_ASSETS="$2" repo capacity run --memory=3600 -- timeout 7200 $B node_modules/vitest/vitest.mjs run --configLoader native --maxWorkers=1 --config "$S/🧪️tests/🎚️config/🟦️.ts" /🧊️model-catalog/ --testTimeout=3600000 --hookTimeout=3600000 > "$GEN/cat-$1.log" 2>&1
echo "$1 exit $?" > "$GEN/cat-$1.done"
