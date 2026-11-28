#!/bin/zsh
# 🧊️ The single-asset run of `🧊️model-catalog` as the closing-2 brief states it, inside the grant of its caller: modelcat.sh <id>[,<id>] writes the JSON report `$GEN/modelcat-<first id>.json`.
source "${0:A:h}/env.sh"
cd "$R" && SEMIO_STAFFAGE_MODEL_ASSETS="$1" "$B" node_modules/vitest/vitest.mjs run --configLoader native --maxWorkers=1 --config "$S/🧪️tests/🎚️config/🟦️.ts" /🧊️model-catalog/ --testTimeout=3600000 --hookTimeout=3600000 --reporter=default --reporter=json --outputFile="$GEN/modelcat-${1%%,*}.json"
