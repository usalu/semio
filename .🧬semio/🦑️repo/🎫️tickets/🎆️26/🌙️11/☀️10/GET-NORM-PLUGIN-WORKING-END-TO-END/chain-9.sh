#!/usr/bin/env bash
# ⛓️ Rebuild norm-eurocode, waiting out other writers' transient source-boundary breaches, probe its routes, then rerun the official Rust and JS tests.
set -u
T="$(cd "$(dirname "$0")" && pwd)"
G="$T/🗑️generated"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
while pgrep -f build-play-components > /dev/null; do sleep 60; done
REBUILD_ATTEMPTS=24 REBUILD_PAUSE_SECONDS=300 "$T/rebuild-and-probe.sh" eurocode9 norm-eurocode en1990 en1991 en1997 en1998
cd "$ROOT"
for try in 1 2 3; do
  SEMIO_TEST_LEVEL=quick SEMIO_TEST_BUDGET_MS=7200000 NATIVE_EXEC_TARGET=@semio-tech/norm-plugin:test repo "$T/native-exec.ts" "🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust" ./📜️script.ts test quick > "$G/normtest-rs-test-h9.log" 2>&1
  code=$?
  grep -q "tests run:" "$G/normtest-rs-test-h9.log" && break
  sleep 300
done
echo "[DEBUG] norm-plugin:test exit $code"
for try in 1 2 3; do
  repo run @semio-tech/norm-js:test > "$G/norm-js-test-h9.log" 2>&1
  code=$?
  [ "$code" -eq 0 ] && break
  grep -qE "pass|fail" "$G/norm-js-test-h9.log" && grep -qE "[0-9]+ (pass|fail)" "$G/norm-js-test-h9.log" && break
  sleep 300
done
echo "[DEBUG] norm-js:test exit $code"
