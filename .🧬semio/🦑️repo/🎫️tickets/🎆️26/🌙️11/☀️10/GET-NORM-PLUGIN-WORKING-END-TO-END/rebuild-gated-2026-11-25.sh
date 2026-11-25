#!/usr/bin/env bash
# 🔁️ Rebuilds each Standards component behind the source gate with many attempts, then sweeps all 15 Play routes only when every core was republished.
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; ROOT="$(cd "$T/../../../../../../.." && pwd)"
MODULES="$ROOT/.cache/b/p/m/browser/prototype/modules"; cd "$ROOT"
core_mtime() { stat -f %m "$MODULES"/*/"semio_hub_${1//-/_}_component.core.wasm" 2>/dev/null || echo 0; }
failed=""
for component in norm-eurocode-structural norm-eurocode norm-eurocode-materials norm norm-energy; do
  before=$(core_mtime "$component"); after=$before
  for attempt in 1 2 3 4 5 6 7 8; do
    "$T/source-gate.sh"
    repo "$T/native-exec.ts" "$T" ./build-play-components.ts build "$component" > "$G/rebuild-$component-sweep13-$attempt.log" 2>&1
    code=$?; after=$(core_mtime "$component")
    echo "[DEBUG] build $component attempt $attempt exit $code published $([ "$after" -gt "$before" ] && echo yes || echo no)"
    [ "$after" -gt "$before" ] && break
    sleep 60
  done
  [ "$after" -gt "$before" ] || failed="$failed $component"
done
echo "[DEBUG] build failures:${failed:- none}"
[ -z "$failed" ] || exit 1
SEMIO_TECH_PLAY_PORT=6036 VERIFY_SIGNED_MODULES=1 PLAY_TEST_PICKER_ROUNDTRIP=1 PLAY_TEST_OBSERVE_ARCHIVES=1 \
PLAY_TEST_REQUIRE_DRAW=1 PLAY_TEST_PICKER_WAIT_MS=120000 repo "$T/probe-norm-base-companions.ts" din4108 din16798 iso16757 vdi3805 din18599 en1990 en1991 en1997 en1998 en1992 en1993 en1994 en1995 en1996 en1999 > "$G/routes-sweep13.log" 2>&1
echo "[DEBUG] routes exit $?"
