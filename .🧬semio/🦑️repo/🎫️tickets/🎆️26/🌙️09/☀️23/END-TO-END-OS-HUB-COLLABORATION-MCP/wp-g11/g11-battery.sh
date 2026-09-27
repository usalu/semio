#!/bin/zsh
# 🔋️ G11 hub-lane battery: every outcome-4 product gate against one live hub, in sequence (one headless browser at a time).
# usage: zsh g11-battery.sh <hubOrigin> <admin-capability.json> <tag> [servePort=6531]
#   S4 user path en + de (`user-path-check`) → inference quartet (`inference-quartet-check`) → hub-agent-participant →
#   security (`security-check`) → plugin coverage (`plugin-coverage-check`, folder lane). The gateway is the staged current-tree
#   binary (`requireMcpBinary`); a serve on <servePort> joined to the hub is started when none answers and stopped at the end.
# Captures: .🧬semio/🌐hub/s13-g11-logs/battery-<tag>/ ; summary: summary.txt there.
set -u
HUB="${1:?hubOrigin}"; ADMIN="${2:?admin-capability.json}"; TAG="${3:?tag}"; PORT="${4:-6531}"
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-g11
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
OUT="$H/s13-g11-logs/battery-$TAG"; mkdir -p "$OUT"
MCP_RS="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust"
export S_AGENT_BRIDGE_DIR="$H/s13-g11-bridge"
summary() { echo "$(date +%H:%M:%S) $1" | tee -a "$OUT/summary.txt"; }
counts() { echo "$(/usr/bin/grep -c '^PASS' "$1") PASS / $(/usr/bin/grep -c '^FAIL' "$1") FAIL"; }
summary "battery $TAG hub=$HUB serve=:$PORT"
curl -sf -o /dev/null "$HUB/readyz" || { summary "hub not ready at $HUB"; exit 1; }
SERVE_PID=""
if ! curl -sf -o /dev/null "http://127.0.0.1:$PORT/"; then
  SERVE_PID=$(python3 "$W/g11-detach.py" "$OUT/serve-detach.txt" zsh "$W/g11-serve.sh" s "$PORT" "$HUB")
  summary "serve :$PORT started pid $SERVE_PID"
  N=0; until curl -sf -o /dev/null "http://127.0.0.1:$PORT/" || [ $N -ge 60 ]; do sleep 5; N=$((N+1)); done
fi
cd "$MCP_RS" || exit 1
for LOCALE in en de; do
  OS_MCP_HUB_ORIGIN="$HUB" S_OS_MCP_LIVE_SHELL_URL="http://127.0.0.1:$PORT" S_OS_MCP_LIVE_LOCALE="$LOCALE" S_OS_MCP_USER_PATH_OUT="$OUT/user-path" bun ./📜️script.ts user-path-check > "$OUT/user-path-$LOCALE.txt" 2>&1
  summary "S4 $LOCALE rc=$? $(counts "$OUT/user-path-$LOCALE.txt")"
done
OS_MCP_HUB_ORIGIN="$HUB" S_OS_MCP_QUARTET_OUT="$OUT/quartet" bun ./📜️script.ts inference-quartet-check > "$OUT/quartet.txt" 2>&1
summary "quartet rc=$? $(counts "$OUT/quartet.txt")"
OS_MCP_HUB_ORIGIN="$HUB" bun ./📜️script.ts hub-agent-participant-check > "$OUT/hub-agent-participant.txt" 2>&1
summary "hub-agent-participant rc=$? $(/usr/bin/grep -c 'PASS' "$OUT/hub-agent-participant.txt") PASS lines"
OS_MCP_HUB_ORIGIN="$HUB" OS_HUB_ADMIN_CAPABILITY_FILE="$ADMIN" S_OS_MCP_SECURITY_OUT="$OUT/security" bun ./📜️script.ts security-check > "$OUT/security.txt" 2>&1
summary "security rc=$? $(counts "$OUT/security.txt")"
if [ -n "$SERVE_PID" ]; then
  for c in $(ps -axo pid,ppid | awk -v P=$SERVE_PID '$2==P{print $1}'); do for g in $(ps -axo pid,ppid | awk -v P=$c '$2==P{print $1}'); do kill $g; done; kill $c; done; kill $SERVE_PID
  summary "serve :$PORT stopped"
fi
S_OS_MCP_COVERAGE_OUT="$OUT/coverage" bun ./📜️script.ts plugin-coverage-check > "$OUT/coverage.txt" 2>&1
summary "plugin-coverage rc=$? (table: $OUT/coverage/coverage-table.md)"
summary "battery $TAG done"
