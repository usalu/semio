#!/usr/bin/env bash
# Probes the semio MCP server from .mcp.json: initialize, initialized, tools/list; prints elapsed seconds per stdout frame.
cd "$(git rev-parse --show-toplevel)" || exit 1
OUT="${1:?output file}"; WAIT="${2:-240}"
CMD=$(jq -r '.mcpServers.semio.command' .mcp.json)
ARGS=()
while IFS= read -r a; do ARGS+=("$a"); done < <(jq -r '.mcpServers.semio.args[]' .mcp.json)
start=$(date +%s)
{
  printf '%s\n' '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"probe","version":"0"}}}'
  sleep 1
  printf '%s\n' '{"jsonrpc":"2.0","method":"notifications/initialized"}'
  printf '%s\n' '{"jsonrpc":"2.0","id":2,"method":"tools/list"}'
  sleep "$WAIT"
} | timeout $((WAIT+5)) "$CMD" "${ARGS[@]}" 2>"$OUT.stderr" | while IFS= read -r line; do echo "t+$(( $(date +%s) - start ))s ${line:0:400}"; done > "$OUT"
