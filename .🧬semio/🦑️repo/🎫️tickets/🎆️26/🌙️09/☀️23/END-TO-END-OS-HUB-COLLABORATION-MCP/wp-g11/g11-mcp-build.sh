#!/bin/zsh
# 🏗️ G11: restage the os-mcp binary (package dist), niced. usage: g11-mcp-build.sh
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" || exit 1
export CARGO_INCREMENTAL=0
date; S=$(date +%s); nice -n 10 bun ./📜️script.ts build; echo "RC=$? secs=$(( $(date +%s)-S ))"; date
