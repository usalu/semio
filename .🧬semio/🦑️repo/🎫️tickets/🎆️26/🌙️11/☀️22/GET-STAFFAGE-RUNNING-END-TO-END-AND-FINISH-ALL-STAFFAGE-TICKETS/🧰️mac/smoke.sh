#!/usr/bin/env bash
# 🚦️ Smoke of a built site in desktop and phone, English and German, under a capacity grant. Usage: smoke.sh <release|prototype> <name> [smoke.mjs options]
set -euo pipefail
source "$(dirname "$0")/env.sh"
MODE="$1" NAME="$2"; shift 2
cd "$ROOT"
repo capacity run --memory="${SMOKE_MEMORY_MB:-2500}" -- timeout "${SMOKE_TIMEOUT_S:-2400}" \
  node "$HERE/smoke.mjs" "$S/🌐️site/📦️packages/🟦️typescript/dist/$MODE" "$OUT/$NAME" "$@"
