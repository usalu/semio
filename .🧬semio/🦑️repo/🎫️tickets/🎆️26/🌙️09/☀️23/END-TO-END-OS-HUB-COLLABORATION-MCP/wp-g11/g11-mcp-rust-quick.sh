#!/bin/zsh
# 🧪️ G11: os-mcp Rust quick suite in G11's private target. usage: g11-mcp-rust-quick.sh <capture>
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" || exit 1
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-g11/target
date; nice -n 10 bun ./📜️script.ts test quick; echo "RC=$?"; date
