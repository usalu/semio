#!/bin/sh
# usage: verify.sh <mirror-root>   (applies this batch to the mirror, then runs the laws)
set -eu
ROOT="$1"; HERE="$(cd "$(dirname "$0")" && pwd)"
PY=python3
"$PY" -I "$HERE/stage.py" check --root "$ROOT" || true
"$PY" -I "$HERE/stage.py" apply --root "$ROOT"
cd "$ROOT"
P="🧰️framework/🔨️modules/🏃️process"
for t in "$P/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🟦️.ts" "$P/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🧪️tests/🟦️.ts" "$P/🎛️owned-execution/🧪️tests/🟦️.ts" "$P/🎛️owned-execution/🧪️tests/📤️progress/🟦️.ts"; do
  (cd "$(dirname "$t")" && bun test "./$(basename "$t")") || exit 1
done
echo "lang batch: GREEN"
