#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_TEST_ARTIFACT_DIR=/Users/ueli/Documents/semio/.cache/ts-art SEMIO_WORKSPACE_ROOT=/Users/ueli/Documents/semio
O="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/MIT-BESTAND-DEMONSTRATOR-CDN-REDEPLOY-READINESS/🗑️generated/readiness"; : > "$O/summary.txt"; n=0
for f in "🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/📥️sources/🧪️tests/🟦️.ts" "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧪️tests/🟦️.ts"; do n=$((n+1))
repo capacity run --memory=2048 -- repo tools exec bun -- run "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/MIT-BESTAND-DEMONSTRATOR-CDN-REDEPLOY-READINESS/native-launch.ts" . repo tools exec bun -- test "./$f" > "$O/$n.log" 2>&1; echo "$n exit=$? $f" >> "$O/summary.txt"; done
echo DONE >> "$O/summary.txt"
