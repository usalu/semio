#!/bin/zsh
# 🔋️ G12 hub-lane battery: the outcome-4 product gates against one live hub, in sequence (≤ 1 headless browser at a time),
# credentials from the private env files (never argv, never printed), flags per preamble rule 17.
# usage: zsh g12-battery.sh <hubOrigin> <hub state dir (admin-capability.json)> <tag> [serveOrigin]
#   participant (user1) → quartet (user1) → security (user2) → hub coverage (user1) → durability (own hub; only with
#   G12_HUB_BINARY + G12_HUB_ROOT = the hub's binary and a published root) → user path en + de (user1, needs a serve joined to the hub).
# Captures: .🧬semio/🌐hub/s14-g12-logs/battery-<tag>/ ; summary: summary.txt there. Gateway: SEMIO_OS_MCP_BIN or the staged one.
set -u
HUB="${1:?hubOrigin}"; STATE="${2:?hub state dir}"; TAG="${3:?tag}"; SERVE="${4:-}"
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
C="$H/s14-g12-credentials"
OUT="$H/s14-g12-logs/battery-$TAG"; mkdir -p "$OUT"
MCP_RS="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust"
summary() { echo "$(date +%H:%M:%S) $1" | tee -a "$OUT/summary.txt"; }
acceptance() { /usr/bin/grep -E '^\[acceptance\]' "$1" | head -1 | cut -c1-400; }
as_user() { ( set -a; source "$C/$1.env"; set +a; shift; "$@" ); }
summary "battery $TAG hub=$HUB serve=${SERVE:-none} gateway=${SEMIO_OS_MCP_BIN:-staged}"
curl -sf -o /dev/null "$HUB/readyz" || { summary "hub not ready at $HUB"; exit 1; }
cd "$MCP_RS" || exit 1
as_user user1 bun ./📜️script.ts hub-agent-participant-check --hub "$HUB" > "$OUT/participant.txt" 2>&1; summary "participant rc=$? $(acceptance "$OUT/participant.txt")"
S_OS_MCP_QUARTET_OUT="$OUT/quartet" as_user user1 bun ./📜️script.ts inference-quartet-check --hub "$HUB" > "$OUT/quartet.txt" 2>&1; summary "quartet rc=$? $(acceptance "$OUT/quartet.txt")"
touch "$STATE/admin-request"; sleep 8
S_OS_MCP_SECURITY_OUT="$OUT/security" as_user user2 bun ./📜️script.ts security-check --hub "$HUB" --hub-admin-capability "$STATE/admin-capability.json" > "$OUT/security.txt" 2>&1; summary "security rc=$? $(acceptance "$OUT/security.txt")"
S_OS_MCP_COVERAGE_OUT="$OUT/coverage-hub" as_user user1 bun ./📜️script.ts plugin-coverage-check --hub "$HUB" > "$OUT/coverage-hub.txt" 2>&1; summary "coverage-hub rc=$? $(acceptance "$OUT/coverage-hub.txt")"
if [ -n "${G12_HUB_BINARY:-}" ] && [ -n "${G12_HUB_ROOT:-}" ]; then
  mkdir -p "$H/s14-g12-durability"
  SEMIO_TEST_ARTIFACT_DIR="$H/s14-g12-durability" OS_MCP_HUB_BINARY="$G12_HUB_BINARY" OS_MCP_HUB_DATA_DIR="$G12_HUB_ROOT" bun ./📜️script.ts hub-edit-durability-check > "$OUT/durability.txt" 2>&1; summary "durability rc=$? $(acceptance "$OUT/durability.txt")"
  rm -rf "$H/s14-g12-durability"/hub-edit-durability-*(N)
fi
if [ -n "$SERVE" ]; then
  for LOCALE in en de; do
    S_OS_MCP_USER_PATH_OUT="$OUT/user-path" as_user user1 bun ./📜️script.ts user-path-check --hub "$HUB" --serve "$SERVE" --locale "$LOCALE" > "$OUT/user-path-$LOCALE.txt" 2>&1; summary "user-path $LOCALE rc=$? $(acceptance "$OUT/user-path-$LOCALE.txt")"
  done
fi
summary "battery $TAG done"
