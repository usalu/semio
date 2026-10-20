#!/bin/sh
# 🔬 Runs explicit test files (never a directory) sequentially with a minted caller envelope, from the live tree or from the mirror, and prints one summary row per file.
# usage: run-tests.sh live|mirror <mirror-dir> <seconds> <relative-test-file>...
set -u
MODE="$1"; MIRROR="$2"; SECS="$3"; shift 3
HERE="$(cd "$(dirname "$0")" && pwd)"
LIVE="$(cd "$HERE/../../../../../../../.." && pwd)"
if [ "$MODE" = mirror ]; then BASE="$MIRROR"; else BASE="$LIVE"; fi
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
ART="${TMPDIR:-/tmp}/tax-stage-artifacts-$MODE"; rm -rf "$ART"; mkdir -p "$ART"; export SEMIO_TEST_ARTIFACT_DIR="$ART"
MINT="$ART/mint.ts"
cat > "$MINT" <<TS
import {createScriptProcessEnvelope,scriptProcessEnvironment} from "$LIVE/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {createNativeBuildCallerCapabilitiesV1} from "$LIVE/$LIB/🏗️build/📥️caller/🟦️.ts";
process.stdout.write(scriptProcessEnvironment(createScriptProcessEnvelope({version:1,owner:"tax-stage-test",maximumElapsedMilliseconds:3000000},createNativeBuildCallerCapabilitiesV1(process.cwd(),["show","projects"],"nx",process.env),Date.now()),{}).SEMIO_SCRIPT_PROCESS_INVOCATION!);
TS
cd "$BASE" || exit 2
for T in "$@"; do
  [ -f "$BASE/$T" ] || { echo "MISSING $T"; continue; }
  START=$(date +%s)
  OUT="$ART/$(echo "$T" | cksum | cut -d' ' -f1).log"
  SEMIO_SCRIPT_PROCESS_INVOCATION="$(bun "$MINT")" perl -e "alarm $SECS; exec @ARGV" -- bun test "$BASE/$T" --timeout 120000 > "$OUT" 2>&1
  RC=$?
  PASS=$(command grep -aE '^ *[0-9]+ pass' "$OUT" | tail -1 | awk '{print $1}'); FAIL=$(command grep -aE '^ *[0-9]+ fail' "$OUT" | tail -1 | awk '{print $1}')
  echo "$MODE rc=$RC pass=${PASS:-?} fail=${FAIL:-?} secs=$(( $(date +%s) - START )) $T"
  command grep -aE '^\(fail\)' "$OUT" | sed 's/ \[[0-9.]*ms\]//' | sort | head -${MAXFAIL:-30} | sed 's/^/    /'
done
