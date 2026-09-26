#!/bin/zsh
# 🤖️ G11: hub-agent-participant gate against a given hub origin (required; 7800 belongs to W2 and is never defaulted).
cd /Users/ueli/Documents/semio || exit 1
date
OS_MCP_HUB_ORIGIN="${1:?usage: g11-participant.sh <hubOrigin>}" NX_DAEMON=false bun nx run @semio-tech/framework-os-mcp-rs:hub-agent-participant-check
echo "EXIT=$?"; date
