#!/bin/zsh
# 🤖️ G10: hub-agent-participant gate against a given hub (default canonical 7800).
cd /Users/ueli/Documents/semio || exit 1
date
OS_MCP_HUB_ORIGIN="${1:-http://127.0.0.1:7800}" NX_DAEMON=false bun nx run @semio-tech/framework-os-mcp-rs:hub-agent-participant-check
echo "EXIT=$?"; date
