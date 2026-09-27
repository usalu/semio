#!/bin/zsh
# 🏗️ G11: restage the os-mcp binary (package dist) through the fleet `native` mutex in build-fleet-b (rule 30).
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
date; S=$(date +%s)
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native g11 -- nice -n 15 bun ./📜️script.ts build
echo "RC=$? secs=$(( $(date +%s)-S ))"; date
