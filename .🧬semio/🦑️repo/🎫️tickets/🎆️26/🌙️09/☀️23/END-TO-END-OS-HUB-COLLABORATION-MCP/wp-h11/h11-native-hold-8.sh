#!/bin/zsh
# 🔐️ H11: the hub directory live lanes through their verb, each under its own claim of the shared server (first runs after the
# move off private containers). usage: h11-native-hold-8.sh <label>
cd /Users/ueli/Documents/semio
LABEL=$1
TS="/Users/ueli/Documents/semio/🌎️hub/📦️packages/🟦️typescript"
HUB="/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts"
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-h11-logs"
ENVS=(CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h11/target "CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" RUST_MIN_STACK=268435456)
for lane in postgres neo4j; do
  echo "=== $lane $(date +%T)" >> "$L/$LABEL-directory-lanes.txt"
  (cd "$TS" && bun ./📜️script.ts backend run $lane -- env "${ENVS[@]}" bun "$HUB" directory-live-lanes $lane) >> "$L/$LABEL-directory-lanes.txt" 2>&1
  echo "=== $lane exit $? $(date +%T)" >> "$L/$LABEL-directory-lanes.txt"
done
