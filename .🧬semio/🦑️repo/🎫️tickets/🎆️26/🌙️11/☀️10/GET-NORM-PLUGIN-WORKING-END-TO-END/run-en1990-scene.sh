#!/usr/bin/env bash
# 🦀️ en1990 scene lib tests.
T="$(cd "$(dirname "$0")" && pwd)"
G="$T/🗑️generated"
cd "$T/../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990" || exit 1
rm -f "$G/en1990-scene.exit"
repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-en1990-scene --lib --no-fail-fast --offline > "$G/en1990-scene.log" 2>&1
echo "scene exit $?" > "$G/en1990-scene.exit"
