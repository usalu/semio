#!/usr/bin/env bash
# 🚦️ Runs s1's `smoke.mjs` against a built site under a capacity grant, with screenshots and downloads in `🗑️generated/linux-s6/<name>`. Usage: smoke.sh <release|prototype> <name> [smoke.mjs options]
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../../.." && pwd)"
MODE="$1" NAME="$2"; shift 2
cd "$ROOT"
repo capacity run --memory="${SMOKE_MEMORY_MB:-3500}" -- timeout "${SMOKE_TIMEOUT_S:-2400}" \
  node "$HERE/../s1/smoke.mjs" "$ROOT/🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/📦️packages/🟦️typescript/dist/$MODE" "$HERE/../../🗑️generated/linux-s6/$NAME" "$@"
