#!/usr/bin/env bash
# ⛓️ Run the official Norm plugin test, then rebuild the structural guest and probe its routes.
set -u
T="$(cd "$(dirname "$0")" && pwd)"
G="$T/🗑️generated"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
cd "$ROOT"
for attempt in 1 2; do
  repo cache gc --lang=en > "$G/cache-gc-test-$attempt.log" 2>&1
  SEMIO_TEST_LEVEL=quick SEMIO_TEST_BUDGET_MS=7200000 NATIVE_EXEC_TARGET=@semio-tech/norm-plugin:test repo "$T/native-exec.ts" "🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust" ./📜️script.ts test quick > "$G/normtest-rs-test-h3-$attempt.log" 2>&1
  code=$?
  echo "[DEBUG] norm-plugin:test attempt $attempt exit $code"
  [ "$code" -eq 0 ] && break
  grep -aq "No space left on device\|exit 75\|SIGTERM" "$G/normtest-rs-test-h3-$attempt.log" || break
done
"$T/rebuild-and-probe.sh" structural norm-eurocode-structural en1992 en1993 en1994
