#!/bin/zsh
# 🧪️ G10: os-mcp Rust quick suite in G10's private target. usage: g10-mcp-rust-quick.sh <capture>
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" || exit 1
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-g10/target
date; nice -n 15 bun ./📜️script.ts test quick; echo "RC=$?"; date
