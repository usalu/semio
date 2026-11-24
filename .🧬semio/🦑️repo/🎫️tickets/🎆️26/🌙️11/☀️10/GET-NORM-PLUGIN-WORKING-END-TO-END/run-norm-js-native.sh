#!/usr/bin/env bash
# 🟦️ Run the official Norm JS suites through native-exec, which skips the project graph and its memory admission.
set -u
T="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
cd "$ROOT"
NATIVE_EXEC_TARGET=@semio-tech/norm-js:test repo "$T/native-exec.ts" "✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript" ./📜️script.ts test > "$T/🗑️generated/norm-js-native.log" 2>&1
echo "[DEBUG] norm-js native exit $?"
