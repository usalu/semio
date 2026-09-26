#!/bin/zsh
# 🔋️ G11 hub-lane battery: every outcome-4 proof against one live hub, in sequence (one headless browser at a time).
# usage: zsh g11-battery.sh <hubOrigin> <hubStateDir> <gatewayBinary> <tag> [servePort=6531]
#   S4 user path en + de (official MCP SDK client, shell approval, withdrawal, connected-client refusal) → quartet (generic
#   inference: grid3d guest solve, gis hub offer, approve → commit) → hub-agent-participant → security probe (scopes,
#   audience, rate limit, revocation + socket close). The gateway binary is pinned for every client (SEMIO_OS_MCP_BIN).
# Captures: .🧬semio/🌐hub/s13-g11-logs/battery-<tag>/ ; summary: summary.txt there.
set -u
HUB="${1:?hubOrigin}"; STATE="${2:?hubStateDir}"; BIN="${3:?gatewayBinary}"; TAG="${4:?tag}"; PORT="${5:-6531}"
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-g11
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
OUT="$H/s13-g11-logs/battery-$TAG"; mkdir -p "$OUT"
export SEMIO_OS_MCP_BIN="$BIN" S_AGENT_BRIDGE_DIR="$H/s13-g11-bridge"
HUBPORT="${HUB##*:}"
summary() { echo "$(date +%H:%M:%S) $1" | tee -a "$OUT/summary.txt"; }
summary "battery $TAG hub=$HUB gateway=$BIN serve=:$PORT"
curl -sf -o /dev/null "$HUB/readyz" || { summary "hub not ready at $HUB"; exit 1; }
SERVE_PID=""
if ! curl -sf -o /dev/null "http://127.0.0.1:$PORT/"; then
  SERVE_PID=$(python3 "$W/g11-detach.py" "$OUT/serve-detach.txt" zsh "$W/g11-serve.sh" s "$PORT" "$HUB")
  summary "serve :$PORT started pid $SERVE_PID"
  N=0; until curl -sf -o /dev/null "http://127.0.0.1:$PORT/" || [ $N -ge 60 ]; do sleep 5; N=$((N+1)); done
fi
cd /Users/ueli/Documents/semio || exit 1
MCP_RS="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust"
for LOCALE in en de; do
  (cd "$MCP_RS" && OS_MCP_HUB_ORIGIN="$HUB" S_OS_MCP_LIVE_SHELL_URL="http://127.0.0.1:$PORT" S_OS_MCP_LIVE_LOCALE="$LOCALE" S_OS_MCP_USER_PATH_OUT="$OUT/user-path" bun ./📜️script.ts user-path-check) > "$OUT/user-path-$LOCALE.txt" 2>&1
  summary "S4 $LOCALE rc=$? $(/usr/bin/grep -hc '^PASS' "$OUT/user-path-$LOCALE.txt") PASS / $(/usr/bin/grep -hc '^FAIL' "$OUT/user-path-$LOCALE.txt") FAIL"
done
bun "$W/g11-quartet-live.ts" "$HUB" "$BIN" > "$OUT/quartet.txt" 2>&1
summary "quartet rc=$? $(/usr/bin/grep -hc '^PASS' "$OUT/quartet.txt") PASS / $(/usr/bin/grep -hc '^FAIL' "$OUT/quartet.txt") FAIL"
(cd "$MCP_RS" && OS_MCP_HUB_ORIGIN="$HUB" bun ./📜️script.ts hub-agent-participant-check) > "$OUT/hub-agent-participant.txt" 2>&1
summary "hub-agent-participant rc=$? $(/usr/bin/grep -hc 'PASS' "$OUT/hub-agent-participant.txt") PASS lines"
bun "$W/g11-security-probe.ts" "$HUB" "$OUT/security" "$STATE" > "$OUT/security.txt" 2>&1
summary "security rc=$? $(/usr/bin/grep -h '^g11-security-probe' "$OUT/security.txt")"
if [ -n "$SERVE_PID" ]; then
  for c in $(ps -axo pid,ppid | awk -v P=$SERVE_PID '$2==P{print $1}'); do for g in $(ps -axo pid,ppid | awk -v P=$c '$2==P{print $1}'); do kill $g; done; kill $c; done; kill $SERVE_PID
  summary "serve :$PORT stopped"
fi
summary "battery $TAG done"
