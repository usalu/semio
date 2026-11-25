#!/usr/bin/env bash
# 🚦️ Runs `smoke.mjs` against a built site under a capacity grant. Usage: smoke.sh <release|prototype> <output-name> [smoke.mjs options]
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../../.." && pwd)"
MODE="$1" NAME="$2"; shift 2
cd "$ROOT"
repo capacity run --memory="${SMOKE_MEMORY_MB:-2500}" -- timeout "${SMOKE_TIMEOUT_S:-2400}" \
  node "$HERE/smoke.mjs" "$ROOT/🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/📦️packages/🟦️typescript/dist/$MODE" "$HERE/../../🗑️generated/linux-s1/$NAME" "$@"
