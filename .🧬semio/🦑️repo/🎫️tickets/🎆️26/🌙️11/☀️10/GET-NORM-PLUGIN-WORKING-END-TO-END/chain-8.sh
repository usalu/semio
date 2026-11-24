#!/usr/bin/env bash
# ⛓️ Rebuild norm-eurocode with the current close-path framework, probe its four routes, then rerun the official Rust and JS tests.
set -u
T="$(cd "$(dirname "$0")" && pwd)"
G="$T/🗑️generated"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
while pgrep -f build-play-components > /dev/null; do sleep 60; done
"$T/rebuild-and-probe.sh" eurocode8 norm-eurocode en1990 en1991 en1997 en1998
cd "$ROOT"
SEMIO_TEST_LEVEL=quick SEMIO_TEST_BUDGET_MS=7200000 NATIVE_EXEC_TARGET=@semio-tech/norm-plugin:test repo "$T/native-exec.ts" "🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust" ./📜️script.ts test quick > "$G/normtest-rs-test-h8.log" 2>&1
echo "[DEBUG] norm-plugin:test exit $?"
repo run @semio-tech/norm-js:test > "$G/norm-js-test-h8.log" 2>&1
echo "[DEBUG] norm-js:test exit $?"
