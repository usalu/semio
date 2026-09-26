#!/bin/zsh
# 🧭️ G11 S4 user path run against G11's hub + serve. usage: zsh g11-user-path.sh <shellPort> <hubPort> <en|de> <run tag>
# The gateway binary is pinned with SEMIO_OS_MCP_BIN (a signed copy under s13-g11-bin) so no client initialize restages it.
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
export S_AGENT_BRIDGE_DIR="$H/s13-g11-bridge" SEMIO_OS_MCP_BIN="${SEMIO_OS_MCP_BIN:-$H/s13-g11-bin/semio-os-mcp-1512}"
cd /Users/ueli/Documents/semio || exit 1
date; S=$(date +%s)
bun .tmp-ticket/wp-g11/g11-user-path.ts "http://127.0.0.1:$1" "http://127.0.0.1:$2" "$3" "$H/s13-g11-logs/user-path-$4" user1@semio.dev gm1-local-dev-pass-1
echo "RC=$? secs=$(( $(date +%s)-S ))"; date
