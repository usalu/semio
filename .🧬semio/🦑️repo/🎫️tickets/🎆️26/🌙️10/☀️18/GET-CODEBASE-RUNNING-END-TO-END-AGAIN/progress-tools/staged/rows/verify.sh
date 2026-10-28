#!/bin/sh
# usage: verify.sh <mirror-root>   (applies this batch to the mirror, then runs the laws)
set -eu
ROOT="$1"; HERE="$(cd "$(dirname "$0")" && pwd)"
PY=python3
"$PY" -I "$HERE/stage.py" apply --root "$ROOT"
cd "$ROOT/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress"
bun test ./🟦️.ts
echo "rows batch: GREEN"
