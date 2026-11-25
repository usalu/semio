#!/usr/bin/env bash
# 🟦️ Run the official Norm JS suites through native-exec, with machine-wide admission and no project graph.
set -eu
T="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
cd "$ROOT"
mkdir -p "$T/🗑️generated"
export NATIVE_EXEC_TARGET=@semio-tech/norm-js:test
exec repo capacity run --memory=2048 --cpu=1 -- repo "$T/native-exec.ts" "✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript" ./📜️script.ts test > "$T/🗑️generated/norm-js-native.log" 2>&1
