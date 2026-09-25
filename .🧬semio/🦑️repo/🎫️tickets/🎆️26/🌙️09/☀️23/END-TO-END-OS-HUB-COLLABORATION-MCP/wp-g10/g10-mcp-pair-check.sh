#!/bin/zsh
# 🤝️ G10: semio MCP quick laws after the pair-receipt join change (remote + pair + workspace + inference).
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-g10/target
date
cargo test -p semio-framework-os-mcp --lib --no-fail-fast -- workspace:: inference:: artifact::
echo "TEST_EXIT=$?"; date
