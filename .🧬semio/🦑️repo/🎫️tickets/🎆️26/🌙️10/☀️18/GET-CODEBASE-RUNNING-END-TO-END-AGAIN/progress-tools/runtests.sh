#!/bin/sh
# usage: runtests.sh <mirror-name> <out-file> <test-dir-rel>...   -- prints "(pass|fail) name" lines per file, no mirror noise
C="$(cd "$(dirname "$0")/.." && pwd)"; NAME="$1"; OUT="$2"; shift 2
: > "$OUT"
mkdir -p "$C/🗑️generated/progress/art"
for d in "$@"; do
  echo "### $d" >> "$OUT"
  SEMIO_TEST_ARTIFACT_DIR="$C/🗑️generated/progress/art" python3 -I "$C/mirror-tools/mirror.py" run "$NAME" --cwd "$d" -- bun test ./🟦️.ts 2>&1 | grep -E '^\((pass|fail)\)|^ *[0-9]+ (pass|fail)$|^error:' | sed 's/ \[[0-9.]*m\?s\]$//' >> "$OUT"
done
