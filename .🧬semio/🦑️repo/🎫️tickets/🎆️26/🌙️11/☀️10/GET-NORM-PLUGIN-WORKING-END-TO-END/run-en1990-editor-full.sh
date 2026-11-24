#!/usr/bin/env bash
# 🦀️ en1990 editor and scene: lib and integration tests (latency_bench skipped, it asserts in release builds only).
T="$(cd "$(dirname "$0")" && pwd)"
G="$T/🗑️generated"
cd "$T/../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990" || exit 1
rm -f "$G/en1990-ed-b.exit"
repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-en1990-editor -p semio-s-artifact-norm-en1990-scene -p semio-s-artifact-norm-en1990-inferences --lib --test integration --no-fail-fast --offline -- --skip latency_bench > "$G/en1990-ed-b.log" 2>&1
echo "full exit $?" > "$G/en1990-ed-b.exit"
