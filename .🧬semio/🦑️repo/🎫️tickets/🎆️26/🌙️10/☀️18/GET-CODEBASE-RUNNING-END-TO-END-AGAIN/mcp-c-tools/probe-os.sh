#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
BIN="${SEMIO_OS_MCP_BIN:-$PWD/$C/🗑️generated/mcp-b/semio-os-mcp}"
POL="$PWD/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🎚️config/🏛️host-driver-policy.json"
export PROBE_COMMAND="[\"$BIN\",\"stdio\",\"--folder\",\".🧬semio/🔗space/os-mcp\",\"--scopes\",\"workspace.read,artifact.write,inference.execute,ui.observe,ui.control,conversation.write\",\"--host-driver-policy\",\"$POL\"]"
export PROBE_TIMEOUT_MS="${PROBE_TIMEOUT_MS:-60000}"
exec bun "$C/mcp-b-tools/probe.ts" semio "$PWD/$C/🗑️generated/mcp-c/probe-semio-${1:-run}.json"
