#!/usr/bin/env bash
# 🔎️ Idle probe of the overview on a built site, with Chrome memory, under a capacity grant. Usage: idle-probe.sh <name> [seconds] [route] [release|prototype]
set -euo pipefail
source "$(dirname "$0")/env.sh"
NAME="$1" SECONDS_IDLE="${2:-160}" ROUTE="${3:-}" MODE="${4:-release}"
cd "$ROOT"
repo capacity run --memory="${PROBE_MEMORY_MB:-2500}" -- timeout "${PROBE_TIMEOUT_S:-900}" node "$HERE/idle-probe.mjs" "$S/🌐️site/📦️packages/🟦️typescript/dist/$MODE" "$OUT/$NAME" "$ROUTE" "$SECONDS_IDLE"
