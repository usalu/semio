#!/usr/bin/env bash
# 📏️ Reproducible layout end-to-end run: activate (build+stage guest), serve on 6079 detached, run the browser acceptance probe, stop serve (ticket 26/09/16/LAYOUT-PLUGIN-END-TO-END).
set -u
T="$(cd "$(dirname "$0")" && pwd)"; ROOT="$(git -C "$T" rev-parse --show-toplevel)"; cd "$ROOT" || exit 1
OUT="$T/🗑️generated/claude"; mkdir -p "$OUT"; STAMP="$(date -u +%H%M%S)"
export SEMIO_LANGUAGE=en S_OS_PORT="${S_OS_PORT:-6079}"
repo run @semio-tech/framework-os-dev:activate-layout-react-prototype --output=stream > "$OUT/e2e-activate-$STAMP.txt" 2>&1 || { echo "activate failed: $OUT/e2e-activate-$STAMP.txt"; exit 1; }
repo run @semio-tech/framework-os-dev:serve-layout-react-prototype --output=stream > "$OUT/e2e-serve-$STAMP.txt" 2>&1 &
SERVE=$!; trap 'kill -TERM $SERVE 2>/dev/null; wait $SERVE 2>/dev/null' EXIT
until curl -fsS "http://127.0.0.1:$S_OS_PORT/" >/dev/null 2>&1; do kill -0 $SERVE 2>/dev/null || { echo "serve exited: $OUT/e2e-serve-$STAMP.txt"; exit 1; }; sleep 2; done
grep -h '\[stale\]' "$OUT/e2e-serve-$STAMP.txt" && echo "staged module is stale"
SEMIO_PROBE_URL="http://127.0.0.1:$S_OS_PORT/?plugin=layout" SEMIO_PROBE_RUN="$STAMP" repo tools exec bun -- "$T/claude-layout-acceptance.mjs"
