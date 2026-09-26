#!/bin/zsh
# 🤖️ G10: live-agent-loop gate against G10's `s` serve. usage: g10-live-agent-loop.sh <port> <en|de>
HUBS="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" || exit 1
export S_OS_MCP_LIVE_SHELL_URL="http://127.0.0.1:$1" S_OS_MCP_LIVE_PLUGIN=note S_OS_MCP_LIVE_SPAWN=note S_OS_MCP_LIVE_LOCALE="$2" S_AGENT_BRIDGE_DIR="$HUBS/s12-g10-bridge"
date; bun ./📜️script.ts live-agent-loop-check; echo "RC=$?"; date
