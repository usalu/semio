#!/usr/bin/env bash
# ⛓️ After the native JS run, re-probe en1990 and rerun the official Rust plugin test until it reports test results.
set -u
T="$(cd "$(dirname "$0")" && pwd)"
G="$T/🗑️generated"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
while pgrep -f run-norm-js-native.sh > /dev/null; do sleep 60; done
cd "$ROOT"
SEMIO_TECH_PLAY_PORT=6036 VERIFY_SIGNED_MODULES=1 PLAY_TEST_PICKER_ROUNDTRIP=1 PLAY_TEST_OBSERVE_ARCHIVES=1 PLAY_TEST_REQUIRE_DRAW=1 PLAY_TEST_PICKER_WAIT_MS=120000 python3 "$T/probe-norm-base-companions.py" en1990 > "$G/routes-en1990-10.log" 2>&1
echo "[DEBUG] en1990 probe exit $?"
for try in 1 2 3 4 5 6; do
  SEMIO_TEST_LEVEL=quick SEMIO_TEST_BUDGET_MS=7200000 NATIVE_EXEC_TARGET=@semio-tech/norm-plugin:test repo "$T/native-exec.ts" "🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust" ./📜️script.ts test quick > "$G/normtest-rs-test-h10.log" 2>&1
  code=$?
  grep -q "tests run:" "$G/normtest-rs-test-h10.log" && break
  echo "[DEBUG] norm-plugin:test try $try produced no test results"
  sleep 600
done
echo "[DEBUG] norm-plugin:test exit $code"
