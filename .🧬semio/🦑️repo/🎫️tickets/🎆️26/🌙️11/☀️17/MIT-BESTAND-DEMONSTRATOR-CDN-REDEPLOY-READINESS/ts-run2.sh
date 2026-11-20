#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_TEST_ARTIFACT_DIR=/Users/ueli/Documents/semio/.cache/ts-art SEMIO_WORKSPACE_ROOT=/Users/ueli/Documents/semio
O="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/MIT-BESTAND-DEMONSTRATOR-CDN-REDEPLOY-READINESS/🗑️generated/ts"; : > "$O/summary3.txt"
for n in 9 10 11 12; do f=$(sed -n ${n}p "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/MIT-BESTAND-DEMONSTRATOR-CDN-REDEPLOY-READINESS/ts-suites.txt")
repo capacity run --memory=1024 -- repo tools exec bun -- run "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/MIT-BESTAND-DEMONSTRATOR-CDN-REDEPLOY-READINESS/native-launch.ts" . repo tools exec bun -- test "./$f" > "$O/q$n.log" 2>&1; echo "$n exit=$?" >> "$O/summary3.txt"; done
echo DONE >> "$O/summary3.txt"
