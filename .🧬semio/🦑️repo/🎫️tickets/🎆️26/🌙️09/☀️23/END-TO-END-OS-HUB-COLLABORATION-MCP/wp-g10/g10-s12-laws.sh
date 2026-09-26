#!/bin/zsh
# 🧪️ G10: the session-12 laws whose captures were lost to the 12:50 folder reset — os-mcp remote (settle, backoff) +
# rendezvous (offer sweep) + bridge + transport, then the wgpu twin (handshake replay, tag 12, presence en/de).
# Private target, niced. Logs: `.🧬semio/🌐hub/s12-g10-logs/s12-laws-*.txt`. usage: zsh g10-s12-laws.sh
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-g10-logs"
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-g10/target
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" || exit 1
date; nice -n 15 cargo test -p semio-framework-os-mcp --lib --no-fail-fast -- workspace::remote:: rendezvous:: bridge:: policy:: > "$L/s12-laws-mcp.txt" 2>&1; echo "MCP_RC=$?"
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust" || exit 1
nice -n 15 cargo test -p semio-framework-os-renderer-wgpu --lib --no-fail-fast -- agent_bridge agent_presence agent_chat_panel wgpu_agent_overlays > "$L/s12-laws-wgpu.txt" 2>&1; echo "WGPU_RC=$?"; date
