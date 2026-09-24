#!/bin/bash
# g6: runs the over-MCP genesis solve probe and samples the semio-os-mcp process mid-solve; $1 = run name.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-g6
G6_RUN="$1" bun g6-solve-probe.ts > "generated/g6-$1-live.txt" 2>&1 &
PROBE=$!
until grep -q "running in the plugin" "generated/g6-$1-live.txt" 2>/dev/null; do sleep 1; done
sleep 4
MCP=$(pgrep -P "$PROBE" -f semio-os-mcp | head -1)
sample "$MCP" 8 -file "generated/g6-$1-stacks.txt" > /dev/null 2>&1
wait "$PROBE"
