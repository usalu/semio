#!/usr/bin/env bash
# 🔁️ Rebuild the given Standards components one at a time, retrying until each core is republished, then probe routes on Play.
# Usage: rebuild-and-probe.sh <tag> "<components>" <routes...>
set -u
T="$(cd "$(dirname "$0")" && pwd)"
G="$T/🗑️generated"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
MODULES="$ROOT/.cache/b/p/m/browser/prototype/modules"
tag="$1"; components="$2"; shift 2
mkdir -p "$G"
cd "$ROOT"
core_mtime() { stat -f %m "$MODULES"/*/"semio_hub_${1//-/_}_component.core.wasm" 2>/dev/null || echo 0; }
for component in $components; do
  before=$(core_mtime "$component")
  for attempt in $(seq 1 "${REBUILD_ATTEMPTS:-3}"); do
    repo cache gc --lang=en > "$G/cache-gc-$tag-$component.log" 2>&1
    repo "$T/native-exec.ts" "$T" ./build-play-components.ts build "$component" > "$G/rebuild-$component-$tag-$attempt.log" 2>&1
    code=$?
    after=$(core_mtime "$component")
    echo "[DEBUG] build $component attempt $attempt exit $code published $([ "$after" -gt "$before" ] && echo yes || echo no)"
    [ "$after" -gt "$before" ] && break
    sleep "${REBUILD_PAUSE_SECONDS:-0}"
  done
done
SEMIO_TECH_PLAY_PORT=6036 VERIFY_SIGNED_MODULES=1 PLAY_TEST_PICKER_ROUNDTRIP=1 PLAY_TEST_OBSERVE_ARCHIVES=1 \
PLAY_TEST_REQUIRE_DRAW=1 PLAY_TEST_PICKER_WAIT_MS=120000 repo "$T/probe-norm-base-companions.ts" "$@" > "$G/routes-$tag.log" 2>&1
echo "[DEBUG] routes exit $?"
