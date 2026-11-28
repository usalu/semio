#!/bin/zsh
# 🧊️ Single-asset model-catalog run of six assets of different categories: cat6.sh <log>
source "${0:A:h}/../MR/env.sh"
cd "$R" && SEMIO_STAFFAGE_MODEL_ASSETS=used-chess-table,walking-toddler,city-car,tealight,shrub-rose,kitchen-extractor-hood repo capacity run --memory=2500 -- timeout 3600 "$B" node_modules/vitest/vitest.mjs run --configLoader native --maxWorkers=1 --config "$S/🧪️tests/🎚️config/🟦️.ts" /🧊️model-catalog/ --testTimeout=3600000 --hookTimeout=3600000 > "$1" 2>&1
echo "exit $?" >> "$1"
